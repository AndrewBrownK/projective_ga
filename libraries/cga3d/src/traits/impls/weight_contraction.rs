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
//   Median:         4       7       0     N/A
//  Average:         6      11       0     N/A
//  Maximum:       101     127       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       1       0       0
//   Median:         6      15       0       0
//  Average:        13      22       0       0
//  Maximum:       225     260       0       0
impl std::ops::Div<WeightContractionInfix> for AntiCircleRotor {
    type Output = WeightContractionInfixPartial<AntiCircleRotor>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       11        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       10       14        0      N/A
    //  no simd       10       21        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(other[scalar]) * self.group2().xyz()).with_w(
                (other[e41] * self[e15])
                    + (other[e42] * self[e25])
                    + (other[e43] * self[e35])
                    + (other[e23] * self[e23])
                    + (other[e31] * self[e31])
                    + (other[e12] * self[e12])
                    + (other[e15] * self[e41])
                    + (other[e25] * self[e42])
                    + (other[e35] * self[e43])
                    + (other[scalar] * self[scalar])
                    - (other[e45] * self[e45]),
            ),
        )
    }
}
impl WeightContraction<AntiDipoleInversion> for AntiCircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        0        3        0      N/A
    //    simd4        4        1        0      N/A
    // Totals...
    // yes simd        8       12        0      N/A
    //  no simd       20       21        0        0
    fn weight_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        let right_anti_dual_g3_w = other[e5] * -1.0;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(other[e4]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g3_xyz[1] * self[e42]) - (right_anti_dual_g3_xyz[2] * self[e43]))
                + (Simd32x3::from(right_anti_dual_g3_w) * self.group0()).with_w(0.0)
                + (right_anti_dual_g3_xyz.zxy() * self.group1().yzx()).with_w(0.0)
                - (right_anti_dual_g3_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g3_xyz[0] * self[e41]),
            // e5
            (right_anti_dual_g3_w * self[e45]) + (right_anti_dual_g3_xyz[0] * self[e15]) + (right_anti_dual_g3_xyz[1] * self[e25]) + (right_anti_dual_g3_xyz[2] * self[e35]),
        )
    }
}
impl WeightContraction<AntiDualNum> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       11        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(other[scalar]) * self.group2(),
        )
    }
}
impl WeightContraction<AntiFlector> for AntiCircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        0        3        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        7       11        0      N/A
    //  no simd       16       17        0        0
    fn weight_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e5] * -1.0;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g1_xyz[1] * self[e42]) - (right_anti_dual_g1_xyz[2] * self[e43]))
                + (Simd32x3::from(right_anti_dual_g1_w) * self.group0()).with_w(0.0)
                + (right_anti_dual_g1_xyz.zxy() * self.group1().yzx()).with_w(0.0)
                - (right_anti_dual_g1_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g1_xyz[0] * self[e41]),
            // e5
            (right_anti_dual_g1_w * self[e45]) + (right_anti_dual_g1_xyz[0] * self[e15]) + (right_anti_dual_g1_xyz[1] * self[e25]) + (right_anti_dual_g1_xyz[2] * self[e35]),
        )
    }
}
impl WeightContraction<AntiLine> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
    fn weight_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]) + (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]),
        )
    }
}
impl WeightContraction<AntiMotor> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        7        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6       10        0      N/A
    //  no simd        6       17        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(other[scalar]) * self.group2().xyz()).with_w(
                (self[e41] * other[e15])
                    + (self[e42] * other[e25])
                    + (self[e43] * other[e35])
                    + (self[e23] * other[e23])
                    + (self[e31] * other[e31])
                    + (self[e12] * other[e12])
                    + (self[scalar] * other[scalar]),
            ),
        )
    }
}
impl WeightContraction<AntiPlane> for AntiCircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        0        3        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        7       11        0      N/A
    //  no simd       16       17        0        0
    fn weight_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e5] * -1.0;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0_xyz[1] * self[e42]) - (right_anti_dual_g0_xyz[2] * self[e43]))
                + (Simd32x3::from(right_anti_dual_g0_w) * self.group0()).with_w(0.0)
                + (right_anti_dual_g0_xyz.zxy() * self.group1().yzx()).with_w(0.0)
                - (right_anti_dual_g0_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g0_xyz[0] * self[e41]),
            // e5
            (right_anti_dual_g0_w * self[e45]) + (right_anti_dual_g0_xyz[0] * self[e15]) + (right_anti_dual_g0_xyz[1] * self[e25]) + (right_anti_dual_g0_xyz[2] * self[e35]),
        )
    }
}
impl WeightContraction<Dipole> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
    fn weight_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e41] * other[e15])
                + (self[e42] * other[e25])
                + (self[e43] * other[e35])
                + (self[e23] * other[e23])
                + (self[e31] * other[e31])
                + (self[e12] * other[e12])
                + (self[e15] * other[e41])
                + (self[e25] * other[e42])
                + (self[e35] * other[e43])
                - (self[e45] * other[e45]),
        )
    }
}
impl WeightContraction<DipoleInversion> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
    fn weight_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e41] * other[e15])
                + (self[e42] * other[e25])
                + (self[e43] * other[e35])
                + (self[e23] * other[e23])
                + (self[e31] * other[e31])
                + (self[e12] * other[e12])
                + (self[e15] * other[e41])
                + (self[e25] * other[e42])
                + (self[e35] * other[e43])
                - (self[e45] * other[e45]),
        )
    }
}
impl WeightContraction<DualNum> for AntiCircleRotor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn weight_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e5] * -1.0) * self.group0().with_w(self[e45]))
    }
}
impl WeightContraction<FlatPoint> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]) - (self[e45] * other[e45]),
        )
    }
}
impl WeightContraction<Flector> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]) - (self[e45] * other[e45]),
        )
    }
}
impl WeightContraction<Motor> for AntiCircleRotor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn weight_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e5] * -1.0) * self.group0().with_w(self[e45]))
    }
}
impl WeightContraction<MultiVector> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       14       19        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        6        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd       18       28        0      N/A
    //  no simd       30       47        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g9_xyz = other.group1().xyz();
        let right_anti_dual_g9_w = other[e5] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_anti_dual_g0[1] * self[scalar])
                    + (self[e23] * other[e23])
                    + (self[e31] * other[e31])
                    + (self[e12] * other[e12])
                    + (self[e15] * other[e41])
                    + (self[e25] * other[e42])
                    + (self[e35] * other[e43])
                    - (right_anti_dual_g8[0] * self[e41])
                    - (right_anti_dual_g8[1] * self[e42])
                    - (right_anti_dual_g8[2] * self[e43])
                    - (self[e45] * other[e45]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(other[e4]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g9_xyz[1] * self[e42]) - (right_anti_dual_g9_xyz[2] * self[e43]))
                + (Simd32x3::from(right_anti_dual_g9_w) * self.group0()).with_w(0.0)
                + (right_anti_dual_g9_xyz.zxy() * self.group1().yzx()).with_w(0.0)
                - (right_anti_dual_g9_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g9_xyz[0] * self[e41]),
            // e5
            (right_anti_dual_g9_w * self[e45]) + (right_anti_dual_g9_xyz[0] * self[e15]) + (right_anti_dual_g9_xyz[1] * self[e25]) + (right_anti_dual_g9_xyz[2] * self[e35]),
            // e15, e25, e35, e45
            Simd32x4::from(right_anti_dual_g0[1]) * self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            Simd32x3::from(right_anti_dual_g0[1]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(right_anti_dual_g0[1]) * self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<RoundPoint> for AntiCircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        0        3        0      N/A
    //    simd4        4        1        0      N/A
    // Totals...
    // yes simd        8       12        0      N/A
    //  no simd       20       21        0        0
    fn weight_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e5] * -1.0;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(other[e4]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g0_xyz[1] * self[e42]) - (right_anti_dual_g0_xyz[2] * self[e43]))
                + (Simd32x3::from(right_anti_dual_g0_w) * self.group0()).with_w(0.0)
                + (right_anti_dual_g0_xyz.zxy() * self.group1().yzx()).with_w(0.0)
                - (right_anti_dual_g0_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g0_xyz[0] * self[e41]),
            // e5
            (right_anti_dual_g0_w * self[e45]) + (right_anti_dual_g0_xyz[0] * self[e15]) + (right_anti_dual_g0_xyz[1] * self[e25]) + (right_anti_dual_g0_xyz[2] * self[e35]),
        )
    }
}
impl WeightContraction<Scalar> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       11        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(other[scalar]) * self.group2(),
        )
    }
}
impl WeightContraction<VersorEven> for AntiCircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        0        3        0      N/A
    //    simd4        4        1        0      N/A
    // Totals...
    // yes simd        8       12        0      N/A
    //  no simd       20       21        0        0
    fn weight_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        let right_anti_dual_g3_w = other[e5] * -1.0;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(other[e4]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g3_xyz[1] * self[e42]) - (right_anti_dual_g3_xyz[2] * self[e43]))
                + (Simd32x3::from(right_anti_dual_g3_w) * self.group0()).with_w(0.0)
                + (right_anti_dual_g3_xyz.zxy() * self.group1().yzx()).with_w(0.0)
                - (right_anti_dual_g3_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g3_xyz[0] * self[e41]),
            // e5
            (right_anti_dual_g3_w * self[e45]) + (right_anti_dual_g3_xyz[0] * self[e15]) + (right_anti_dual_g3_xyz[1] * self[e25]) + (right_anti_dual_g3_xyz[2] * self[e35]),
        )
    }
}
impl WeightContraction<VersorOdd> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       11        0        0
    //    simd3        0        3        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       10       15        0      N/A
    //  no simd       10       24        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2_xyz = other.group2().xyz() * Simd32x3::from(-1.0);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(other[scalar]) * self.group2().xyz()).with_w(
                (self[e23] * other[e23])
                    + (self[e31] * other[e31])
                    + (self[e12] * other[e12])
                    + (self[e15] * other[e41])
                    + (self[e25] * other[e42])
                    + (self[e35] * other[e43])
                    + (self[scalar] * other[scalar])
                    - (right_anti_dual_g2_xyz[0] * self[e41])
                    - (right_anti_dual_g2_xyz[1] * self[e42])
                    - (right_anti_dual_g2_xyz[2] * self[e43])
                    - (self[e45] * other[e45]),
            ),
        )
    }
}
impl std::ops::Div<WeightContractionInfix> for AntiDipoleInversion {
    type Output = WeightContractionInfixPartial<AntiDipoleInversion>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       10        0        0
    //    simd3        0        5        0      N/A
    //    simd4        7        5        0      N/A
    // Totals...
    // yes simd       15       20        0      N/A
    //  no simd       36       45        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(other[scalar]) * self.group2().xyz()).with_w(
                (other[e41] * self[e415])
                    + (other[e42] * self[e425])
                    + (other[e43] * self[e435])
                    + (other[e23] * self[e423])
                    + (other[e31] * self[e431])
                    + (other[e12] * self[e412])
                    + (other[scalar] * self[e4]),
            ),
            // e1, e2, e3, e5
            (Simd32x4::from(other[scalar]) * self.group3())
                + (Simd32x4::from([other[e42], other[e43], other[e41], other[e23]]) * self.group2().zxyx())
                + (Simd32x4::from([self[e431], self[e412], self[e423], self[e415]]) * other.group2().zxyx())
                + (other.group1().wwwy() * self.group1().xyz().with_w(self[e315]))
                + Simd32x3::from(0.0).with_w((other[e12] * self[e125]) + (other[e25] * self[e425]) + (other[e35] * self[e435]))
                - (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                - (other.group0().zxy() * self.group2().yzx()).with_w(0.0)
                - (self.group0().zxy() * other.group2().yzx()).with_w(0.0),
        )
    }
}
impl WeightContraction<AntiDipoleInversion> for AntiDipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       16        0        0
    //    simd3        2        5        0      N/A
    //    simd4        6        4        0      N/A
    // Totals...
    // yes simd       20       25        0      N/A
    //  no simd       42       47        0        0
    fn weight_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        let right_anti_dual_g3_w = other[e5] * -1.0;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(right_anti_dual_g2_w) * self.group1().xyz()) + (right_anti_dual_g3_xyz.yzx() * self.group0().zxy())
                - (right_anti_dual_g3_xyz.zxy() * self.group0().yzx()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g3_xyz[1] * self[e425]) - (right_anti_dual_g3_xyz[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g2_w) * self.group2().xyz()).with_w(0.0)
                + (Simd32x3::from(right_anti_dual_g3_w) * self.group0()).with_w(0.0)
                - (self.group1().wwwx() * right_anti_dual_g3_xyz.with_w(right_anti_dual_g3_xyz[0])),
            // e15, e25, e35, scalar
            (self.group1() * Simd32x3::from(right_anti_dual_g3_w).with_w(other[e321]))
                + (self.group2().yzxw() * right_anti_dual_g3_xyz.zxy().with_w(right_anti_dual_g3_w))
                + Simd32x3::from(0.0).with_w(
                    (right_anti_dual_g2_w * self[e5]) + (right_anti_dual_g3_xyz[0] * self[e1]) + (right_anti_dual_g3_xyz[1] * self[e2]) + (right_anti_dual_g3_xyz[2] * self[e3])
                        - (right_anti_dual_g1_xyz[0] * self[e415])
                        - (right_anti_dual_g1_xyz[1] * self[e425])
                        - (right_anti_dual_g1_xyz[2] * self[e435])
                        - (right_anti_dual_g2_xyz[0] * self[e423])
                        - (right_anti_dual_g2_xyz[1] * self[e431])
                        - (right_anti_dual_g2_xyz[2] * self[e412])
                        - (other[e431] * self[e315])
                        - (other[e412] * self[e125]),
                )
                - (self.group2().zxyx() * right_anti_dual_g3_xyz.yzx().with_w(other[e423])),
        )
    }
}
impl WeightContraction<AntiDualNum> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       15        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(other[scalar]) * self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(other[scalar]) * self.group3(),
        )
    }
}
impl WeightContraction<AntiFlatPoint> for AntiDipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Scalar::from_groups(
            // scalar
            (self[e321] * other[e321]) - (right_anti_dual_g0_xyz[0] * self[e423]) - (right_anti_dual_g0_xyz[1] * self[e431]) - (right_anti_dual_g0_xyz[2] * self[e412]),
        )
    }
}
impl WeightContraction<AntiFlector> for AntiDipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        9        0        0
    //    simd3        1        4        0      N/A
    //    simd4        5        3        0      N/A
    // Totals...
    // yes simd       11       16        0      N/A
    //  no simd       28       33        0        0
    fn weight_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e5] * -1.0;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (right_anti_dual_g1_xyz.yzx() * self.group0().zxy()) - (right_anti_dual_g1_xyz.zxy() * self.group0().yzx()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g1_xyz[1] * self[e425]) - (right_anti_dual_g1_xyz[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g1_w) * self.group0()).with_w(0.0)
                - (self.group1().wwwx() * right_anti_dual_g1_xyz.with_w(right_anti_dual_g1_xyz[0])),
            // e15, e25, e35, scalar
            (self.group1() * Simd32x3::from(right_anti_dual_g1_w).with_w(other[e321]))
                + (self.group2().yzxw() * right_anti_dual_g1_xyz.zxy().with_w(right_anti_dual_g1_w))
                + Simd32x3::from(0.0).with_w(
                    (right_anti_dual_g1_xyz[0] * self[e1]) + (right_anti_dual_g1_xyz[1] * self[e2]) + (right_anti_dual_g1_xyz[2] * self[e3])
                        - (right_anti_dual_g0_xyz[1] * self[e431])
                        - (right_anti_dual_g0_xyz[2] * self[e412]),
                )
                - (right_anti_dual_g1_xyz.yzx() * self.group2().zxy()).with_w(right_anti_dual_g0_xyz[0] * self[e423]),
        )
    }
}
impl WeightContraction<AntiLine> for AntiDipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        0        4        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        9       13        0      N/A
    //  no simd       18       21        0        0
    fn weight_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e431]) - (right_anti_dual_g0[2] * self[e412]))
                + (right_anti_dual_g0 * Simd32x3::from(self[e321])).with_w(0.0)
                + (self.group0().yzx() * other.group1().zxy()).with_w(0.0)
                - (self.group0().zxy() * other.group1().yzx()).with_w(right_anti_dual_g0[0] * self[e423]),
            // e5
            (self[e415] * other[e15]) + (self[e425] * other[e25]) + (self[e435] * other[e35])
                - (right_anti_dual_g0[0] * self[e235])
                - (right_anti_dual_g0[1] * self[e315])
                - (right_anti_dual_g0[2] * self[e125]),
        )
    }
}
impl WeightContraction<AntiMotor> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        0        4        0      N/A
    //    simd4        4        4        0      N/A
    // Totals...
    // yes simd       10       17        0      N/A
    //  no simd       22       37        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_anti_dual_g0[3]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_anti_dual_g0[3]) * self.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(right_anti_dual_g0[3]) * self.group2().xyz())
                .with_w((right_anti_dual_g0[3] * self[e4]) - (right_anti_dual_g0[0] * self[e423]) - (right_anti_dual_g0[1] * self[e431]) - (right_anti_dual_g0[2] * self[e412])),
            // e1, e2, e3, e5
            (right_anti_dual_g0 * Simd32x3::from(self[e321]).with_w(self[e5]))
                + (Simd32x4::from([self[e431], self[e412], self[e423], self[e415]]) * other.group1().zxyx())
                + Simd32x3::from(0.0).with_w((self[e425] * other[e25]) + (self[e435] * other[e35]) - (right_anti_dual_g0[1] * self[e315]) - (right_anti_dual_g0[2] * self[e125]))
                + (Simd32x3::from(right_anti_dual_g0[3]) * self.group3().xyz()).with_w(0.0)
                - (self.group0().zxy() * other.group1().yzx()).with_w(right_anti_dual_g0[0] * self[e235]),
        )
    }
}
impl WeightContraction<AntiPlane> for AntiDipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        1        5        0      N/A
    //    simd4        5        2        0      N/A
    // Totals...
    // yes simd        8       13        0      N/A
    //  no simd       25       29        0        0
    fn weight_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e5] * -1.0;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (right_anti_dual_g0_xyz.yzx() * self.group0().zxy()) - (right_anti_dual_g0_xyz.zxy() * self.group0().yzx()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0_xyz[1] * self[e425]) - (right_anti_dual_g0_xyz[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g0_w) * self.group0()).with_w(0.0)
                - (self.group1().wwwx() * right_anti_dual_g0_xyz.with_w(right_anti_dual_g0_xyz[0])),
            // e15, e25, e35, scalar
            (Simd32x4::from(right_anti_dual_g0_w) * self.group1().xyz().with_w(self[e4]))
                + Simd32x3::from(0.0).with_w((right_anti_dual_g0_xyz[1] * self[e2]) + (right_anti_dual_g0_xyz[2] * self[e3]))
                + (right_anti_dual_g0_xyz.zxy() * self.group2().yzx()).with_w(right_anti_dual_g0_xyz[0] * self[e1])
                - (right_anti_dual_g0_xyz.yzx() * self.group2().zxy()).with_w(0.0),
        )
    }
}
impl WeightContraction<Circle> for AntiDipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
    fn weight_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        Scalar::from_groups(
            // scalar
            (self[e321] * other[e321])
                - (right_anti_dual_g1_xyz[0] * self[e415])
                - (right_anti_dual_g1_xyz[1] * self[e425])
                - (right_anti_dual_g1_xyz[2] * self[e435])
                - (self[e423] * other[e235])
                - (self[e431] * other[e315])
                - (self[e412] * other[e125])
                - (self[e235] * other[e423])
                - (self[e315] * other[e431])
                - (self[e125] * other[e412]),
        )
    }
}
impl WeightContraction<CircleRotor> for AntiDipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
    fn weight_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        Scalar::from_groups(
            // scalar
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
        )
    }
}
impl WeightContraction<Dipole> for AntiDipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       11        0        0
    //    simd3        0        5        0      N/A
    //    simd4        6        1        0      N/A
    // Totals...
    // yes simd       13       17        0      N/A
    //  no simd       31       30        0        0
    fn weight_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (other.group1().wwwx() * self.group1().xyz().with_w(self[e423]))
                + Simd32x3::from(0.0).with_w((self[e415] * other[e41]) + (self[e425] * other[e42]) + (self[e435] * other[e43]))
                + (self.group0().yzx() * other.group2().zxy()).with_w(self[e431] * other[e31])
                + (other.group0().yzx() * self.group2().zxy()).with_w(self[e412] * other[e12])
                - (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                - (self.group0().zxy() * other.group2().yzx()).with_w(0.0)
                - (other.group0().zxy() * self.group2().yzx()).with_w(0.0),
            // e5
            (self[e415] * other[e15]) + (self[e425] * other[e25]) + (self[e435] * other[e35]) + (self[e235] * other[e23]) + (self[e315] * other[e31]) + (self[e125] * other[e12]),
        )
    }
}
impl WeightContraction<DipoleInversion> for AntiDipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       11        0        0
    //    simd3        0        5        0      N/A
    //    simd4        6        1        0      N/A
    // Totals...
    // yes simd       13       17        0      N/A
    //  no simd       31       30        0        0
    fn weight_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (other.group1().wwwx() * self.group1().xyz().with_w(self[e423]))
                + Simd32x3::from(0.0).with_w((self[e415] * other[e41]) + (self[e425] * other[e42]) + (self[e435] * other[e43]))
                + (self.group0().yzx() * other.group2().zxy()).with_w(self[e431] * other[e31])
                + (other.group0().yzx() * self.group2().zxy()).with_w(self[e412] * other[e12])
                - (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                - (self.group0().zxy() * other.group2().yzx()).with_w(0.0)
                - (other.group0().zxy() * self.group2().yzx()).with_w(0.0),
            // e5
            (self[e415] * other[e15]) + (self[e425] * other[e25]) + (self[e435] * other[e35]) + (self[e235] * other[e23]) + (self[e315] * other[e31]) + (self[e125] * other[e12]),
        )
    }
}
impl WeightContraction<DualNum> for AntiDipoleInversion {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        9        0        0
    fn weight_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other[e5] * -1.0) * self.group0().with_w(self[e4]),
            // e15, e25, e35, e3215
            (Simd32x3::from(other[e5] * -1.0) * self.group1().xyz()).with_w(0.0),
        )
    }
}
impl WeightContraction<FlatPoint> for AntiDipoleInversion {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        3        4        0      N/A
    //  no simd       12       12        0        0
    fn weight_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x4::from([self[e431], self[e412], self[e423], self[e415]]) * other.group0().zxyx())
                + (self.group1().xyzy() * other.group0().wwwy())
                + Simd32x3::from(0.0).with_w(self[e435] * other[e35])
                - (self.group0().zxy() * other.group0().yzx()).with_w(0.0),
        )
    }
}
impl WeightContraction<Flector> for AntiDipoleInversion {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        3        4        0      N/A
    //  no simd       12       12        0        0
    fn weight_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x4::from([self[e431], self[e412], self[e423], self[e415]]) * other.group0().zxyx())
                + (self.group1().xyzy() * other.group0().wwwy())
                + Simd32x3::from(0.0).with_w(self[e435] * other[e35])
                - (self.group0().zxy() * other.group0().yzx()).with_w(0.0),
        )
    }
}
impl WeightContraction<Line> for AntiDipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
    fn weight_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(self[e423] * other[e235])
                - (self[e431] * other[e315])
                - (self[e412] * other[e125])
                - (self[e415] * other[e415])
                - (self[e425] * other[e425])
                - (self[e435] * other[e435]),
        )
    }
}
impl WeightContraction<Motor> for AntiDipoleInversion {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        6       10        0      N/A
    //  no simd        6       14        0        0
    fn weight_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e5] * -1.0;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(right_anti_dual_g1_w) * self.group0()).with_w(
                (right_anti_dual_g1_w * self[e4])
                    - (right_anti_dual_g0_xyz[0] * self[e415])
                    - (right_anti_dual_g0_xyz[1] * self[e425])
                    - (right_anti_dual_g0_xyz[2] * self[e435])
                    - (right_anti_dual_g1_xyz[0] * self[e423])
                    - (right_anti_dual_g1_xyz[1] * self[e431])
                    - (right_anti_dual_g1_xyz[2] * self[e412]),
            ),
            // e15, e25, e35, e3215
            (Simd32x3::from(right_anti_dual_g1_w) * self.group1().xyz()).with_w(0.0),
        )
    }
}
impl WeightContraction<MultiVector> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       23       32        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4       17        0      N/A
    //    simd4       10        3        0      N/A
    // Totals...
    // yes simd       37       53        0      N/A
    //  no simd       75       97        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g5 = other.group6().xyz();
        let right_anti_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g9_xyz = other.group1().xyz();
        let right_anti_dual_g9_w = other[e5] * -1.0;
        let right_anti_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_anti_dual_g10 * self[e5])
                    + (right_anti_dual_g9_w * self[e4])
                    + (right_anti_dual_g9_xyz[0] * self[e1])
                    + (right_anti_dual_g9_xyz[1] * self[e2])
                    + (right_anti_dual_g9_xyz[2] * self[e3])
                    + (self[e321] * other[e321])
                    - (right_anti_dual_g5[0] * self[e415])
                    - (right_anti_dual_g5[1] * self[e425])
                    - (right_anti_dual_g5[2] * self[e435])
                    - (self[e423] * other[e235])
                    - (self[e431] * other[e315])
                    - (self[e412] * other[e125])
                    - (self[e235] * other[e423])
                    - (self[e315] * other[e431])
                    - (self[e125] * other[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(right_anti_dual_g0[1]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]))
                + (Simd32x4::from([other[e45], other[e45], other[e45], other[e41]]) * self.group1().xyzx())
                + Simd32x3::from(0.0).with_w((self[e412] * other[e12]) + (self[e425] * other[e42]) + (self[e435] * other[e43]))
                + (right_anti_dual_g8.yzx() * self.group0().zxy()).with_w(self[e423] * other[e23])
                + (other.group4().yzx() * self.group2().zxy()).with_w(self[e431] * other[e31])
                - (Simd32x3::from(self[e321]) * other.group5()).with_w(0.0)
                - (right_anti_dual_g8.zxy() * self.group0().yzx()).with_w(0.0)
                - (other.group4().zxy() * self.group2().yzx()).with_w(0.0),
            // e5
            (right_anti_dual_g0[1] * self[e5]) + (self[e235] * other[e23]) + (self[e315] * other[e31]) + (self[e125] * other[e12])
                - (right_anti_dual_g8[0] * self[e415])
                - (right_anti_dual_g8[1] * self[e425])
                - (right_anti_dual_g8[2] * self[e435]),
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g9_xyz[1] * self[e425]) - (right_anti_dual_g9_xyz[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g9_w) * self.group1().xyz()).with_w(0.0)
                + (right_anti_dual_g9_xyz.zxy() * self.group2().yzx()).with_w(0.0)
                - (right_anti_dual_g9_xyz.yzx() * self.group2().zxy()).with_w(right_anti_dual_g9_xyz[0] * self[e415]),
            // e41, e42, e43
            (Simd32x3::from(right_anti_dual_g10) * self.group1().xyz()) + (right_anti_dual_g9_xyz.yzx() * self.group0().zxy())
                - (right_anti_dual_g9_xyz.zxy() * self.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(right_anti_dual_g10) * self.group2().xyz()) + (Simd32x3::from(right_anti_dual_g9_w) * self.group0())
                - (right_anti_dual_g9_xyz * Simd32x3::from(self[e321])),
            // e415, e425, e435, e321
            Simd32x4::from(right_anti_dual_g0[1]) * self.group1(),
            // e423, e431, e412
            Simd32x3::from(right_anti_dual_g0[1]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(right_anti_dual_g0[1]) * self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<RoundPoint> for AntiDipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd3        2        7        0      N/A
    //    simd4        6        2        0      N/A
    // Totals...
    // yes simd       11       17        0      N/A
    //  no simd       33       37        0        0
    fn weight_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e5] * -1.0;
        let right_anti_dual_g1 = other[e4] * -1.0;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(right_anti_dual_g1) * self.group1().xyz()) + (right_anti_dual_g0_xyz.yzx() * self.group0().zxy())
                - (right_anti_dual_g0_xyz.zxy() * self.group0().yzx()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0_xyz[1] * self[e425]) - (right_anti_dual_g0_xyz[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g0_w) * self.group0()).with_w(0.0)
                + (Simd32x3::from(right_anti_dual_g1) * self.group2().xyz()).with_w(0.0)
                - (self.group1().wwwx() * right_anti_dual_g0_xyz.with_w(right_anti_dual_g0_xyz[0])),
            // e15, e25, e35, scalar
            (Simd32x4::from(right_anti_dual_g0_w) * self.group1().xyz().with_w(self[e4]))
                + Simd32x3::from(0.0).with_w((right_anti_dual_g0_xyz[0] * self[e1]) + (right_anti_dual_g0_xyz[1] * self[e2]) + (right_anti_dual_g0_xyz[2] * self[e3]))
                + (right_anti_dual_g0_xyz.zxy() * self.group2().yzx()).with_w(right_anti_dual_g1 * self[e5])
                - (right_anti_dual_g0_xyz.yzx() * self.group2().zxy()).with_w(0.0),
        )
    }
}
impl WeightContraction<Scalar> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       15        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(other[scalar]) * self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(other[scalar]) * self.group3(),
        )
    }
}
impl WeightContraction<VersorEven> for AntiDipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       16        0        0
    //    simd3        2        5        0      N/A
    //    simd4        6        4        0      N/A
    // Totals...
    // yes simd       20       25        0      N/A
    //  no simd       42       47        0        0
    fn weight_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        let right_anti_dual_g3_w = other[e5] * -1.0;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(right_anti_dual_g2_w) * self.group1().xyz()) + (right_anti_dual_g3_xyz.yzx() * self.group0().zxy())
                - (right_anti_dual_g3_xyz.zxy() * self.group0().yzx()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g3_xyz[1] * self[e425]) - (right_anti_dual_g3_xyz[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g2_w) * self.group2().xyz()).with_w(0.0)
                + (Simd32x3::from(right_anti_dual_g3_w) * self.group0()).with_w(0.0)
                - (self.group1().wwwx() * right_anti_dual_g3_xyz.with_w(right_anti_dual_g3_xyz[0])),
            // e15, e25, e35, scalar
            (self.group1() * Simd32x3::from(right_anti_dual_g3_w).with_w(other[e321]))
                + (self.group2().yzxw() * right_anti_dual_g3_xyz.zxy().with_w(right_anti_dual_g3_w))
                + Simd32x3::from(0.0).with_w(
                    (right_anti_dual_g2_w * self[e5]) + (right_anti_dual_g3_xyz[0] * self[e1]) + (right_anti_dual_g3_xyz[1] * self[e2]) + (right_anti_dual_g3_xyz[2] * self[e3])
                        - (right_anti_dual_g0_xyz[1] * self[e315])
                        - (right_anti_dual_g0_xyz[2] * self[e125])
                        - (right_anti_dual_g1_xyz[0] * self[e415])
                        - (right_anti_dual_g1_xyz[1] * self[e425])
                        - (right_anti_dual_g1_xyz[2] * self[e435])
                        - (right_anti_dual_g2_xyz[0] * self[e423])
                        - (right_anti_dual_g2_xyz[1] * self[e431])
                        - (right_anti_dual_g2_xyz[2] * self[e412]),
                )
                - (self.group2().zxyx() * right_anti_dual_g3_xyz.yzx().with_w(right_anti_dual_g0_xyz[0])),
        )
    }
}
impl WeightContraction<VersorOdd> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       11        0        0
    //    simd3        0        6        0      N/A
    //    simd4        7        5        0      N/A
    // Totals...
    // yes simd       13       22        0      N/A
    //  no simd       34       49        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2_xyz = other.group2().xyz() * Simd32x3::from(-1.0);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(other[scalar]) * self.group2().xyz()).with_w(
                (self[e423] * other[e23])
                    + (self[e431] * other[e31])
                    + (self[e412] * other[e12])
                    + (self[e415] * other[e41])
                    + (self[e425] * other[e42])
                    + (self[e435] * other[e43])
                    + (self[e4] * other[scalar]),
            ),
            // e1, e2, e3, e5
            (Simd32x4::from(other[scalar]) * self.group3())
                + (self.group2().zxyy() * other.group0().yzx().with_w(other[e31]))
                + (other.group1().wwwx() * self.group1().xyz().with_w(self[e235]))
                + Simd32x3::from(0.0).with_w((right_anti_dual_g2_xyz[2] * self[e435]) * -1.0)
                + (right_anti_dual_g2_xyz.yzx() * self.group0().zxy()).with_w(self[e125] * other[e12])
                - (self.group1().wwwx() * other.group1().xyz().with_w(right_anti_dual_g2_xyz[0]))
                - (right_anti_dual_g2_xyz.zxy() * self.group0().yzx()).with_w(right_anti_dual_g2_xyz[1] * self[e425])
                - (self.group2().yzx() * other.group0().zxy()).with_w(0.0),
        )
    }
}
impl std::ops::Div<WeightContractionInfix> for AntiDualNum {
    type Output = WeightContractionInfixPartial<AntiDualNum>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       12        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e3215] * -1.0) * other.group0()).with_w(other[scalar] * self[scalar]),
            // e15, e25, e35, e3215
            Simd32x4::from(self[e3215]) * (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(other[scalar]),
        )
    }
}
impl WeightContraction<AntiDipoleInversion> for AntiDualNum {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       14        0        0
    fn weight_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215]) * (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e4]),
            // e1, e2, e3, e5
            Simd32x4::from(self[e3215]) * (other.group0() * Simd32x3::from(-1.0)).with_w(other[e321]),
        )
    }
}
impl WeightContraction<AntiDualNum> for AntiDualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        0        1        0      N/A
    // no simd        0        2        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(other[scalar]) * self.group0())
    }
}
impl WeightContraction<AntiFlatPoint> for AntiDualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e3215] * other[e321], 0.0]))
    }
}
impl WeightContraction<AntiFlector> for AntiDualNum {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        5        0        0
    fn weight_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (Simd32x3::from(self[e3215] * -1.0) * other.group1().xyz()).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e3215] * other[e321]),
        )
    }
}
impl WeightContraction<AntiLine> for AntiDualNum {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        4        0        0
    fn weight_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ (Simd32x3::from(self[e3215] * -1.0) * other.group0()).with_w(0.0))
    }
}
impl WeightContraction<AntiMotor> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        9        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(right_anti_dual_g0[3] * self[scalar]),
            // e15, e25, e35, e3215
            right_anti_dual_g0 * Simd32x4::from(self[e3215]),
        )
    }
}
impl WeightContraction<AntiPlane> for AntiDualNum {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        4        0        0
    fn weight_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ (Simd32x3::from(self[e3215] * -1.0) * other.group0().xyz()).with_w(0.0))
    }
}
impl WeightContraction<Circle> for AntiDualNum {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
    fn weight_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e3215] * -1.0) * other.group0().with_w(other[e321] * -1.0))
    }
}
impl WeightContraction<CircleRotor> for AntiDualNum {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
    fn weight_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e3215] * -1.0) * other.group0().with_w(other[e321] * -1.0))
    }
}
impl WeightContraction<Dipole> for AntiDualNum {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn weight_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(self[e3215] * -1.0) * other.group0(),
            // e15, e25, e35
            Simd32x3::from(self[e3215] * -1.0) * other.group1().xyz(),
        )
    }
}
impl WeightContraction<DipoleInversion> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       11        0        0
    fn weight_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(self[e3215]) * (other.group0() * Simd32x3::from(-1.0)).with_w(other[e1234]),
            // e15, e25, e35, e3215
            (Simd32x3::from(self[e3215] * -1.0) * other.group1().xyz()).with_w(0.0),
        )
    }
}
impl WeightContraction<MultiVector> for AntiDualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        9        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        4        0      N/A
    // Totals...
    // yes simd        1       14        0      N/A
    //  no simd        1       23        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(right_anti_dual_g0[1] * self[scalar]) + (self[e3215] * other[e1234]), 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(self[e3215] * -1.0) * other.group7()).with_w(0.0),
            // e5
            self[e3215] * other[e321],
            // e15, e25, e35, e45
            (Simd32x3::from(self[e3215] * -1.0) * other.group5()).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e3215] * -1.0) * other.group4(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e3215] * other[e4]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[e3215] * -1.0) * other.group1().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(right_anti_dual_g0[1] * self[e3215]),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<RoundPoint> for AntiDualNum {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
    fn weight_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(self[e3215] * -1.0) * other.group0().xyz().with_w(other[e4] * -1.0))
    }
}
impl WeightContraction<Scalar> for AntiDualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        0        1        0      N/A
    // no simd        0        2        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(other[scalar]) * self.group0())
    }
}
impl WeightContraction<Sphere> for AntiDualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e3215] * other[e1234])
    }
}
impl WeightContraction<VersorEven> for AntiDualNum {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       15        0        0
    fn weight_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215]) * other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            Simd32x4::from(self[e3215]) * (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(other[e321]),
        )
    }
}
impl WeightContraction<VersorOdd> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        6        0      N/A
    //  no simd        1       13        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e3215] * -1.0) * other.group0().xyz()).with_w((self[e3215] * other[e1234]) + (self[scalar] * other[scalar])),
            // e15, e25, e35, e3215
            Simd32x4::from(self[e3215]) * (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(other[scalar]),
        )
    }
}
impl std::ops::Div<WeightContractionInfix> for AntiFlatPoint {
    type Output = WeightContractionInfixPartial<AntiFlatPoint>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for AntiFlatPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        3        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd       13       19        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1, e2, e3, e5
            (self.group0().yzxx() * right_anti_dual_g0.zxy().with_w(other[e23])) + Simd32x3::from(0.0).with_w((other[e31] * self[e315]) + (other[e12] * self[e125]))
                - (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                - (right_anti_dual_g0.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl WeightContraction<AntiDipoleInversion> for AntiFlatPoint {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       12       16        0        0
    fn weight_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w((other[e321] * self[e321]) - (other[e412] * self[e125]))
                - (self.group0().xyzy() * Simd32x3::from(other[e4]).with_w(other[e431]))
                - (self.group0().wwwx() * right_anti_dual_g3_xyz.with_w(other[e423])),
            // e15, e25, e35, e3215
            ((right_anti_dual_g3_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g3_xyz.yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl WeightContraction<AntiDualNum> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl WeightContraction<AntiFlatPoint> for AntiFlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e321] * self[e321])
    }
}
impl WeightContraction<AntiFlector> for AntiFlatPoint {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        1        3        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        5        0      N/A
    //  no simd        3       14        0        0
    fn weight_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e5] * -1.0);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321]) * (right_anti_dual_g1.xyz() * Simd32x3::from(-1.0)).with_w(other[e321]),
            // e15, e25, e35, e3215
            ((right_anti_dual_g1.zxy() * self.group0().yzx()) - (right_anti_dual_g1.yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl WeightContraction<AntiLine> for AntiFlatPoint {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        9        0        0
    fn weight_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (right_anti_dual_g0 * Simd32x3::from(self[e321]))
                .with_w(-(right_anti_dual_g0[0] * self[e235]) - (right_anti_dual_g0[1] * self[e315]) - (right_anti_dual_g0[2] * self[e125])),
        )
    }
}
impl WeightContraction<AntiMotor> for AntiFlatPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        2       11        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1, e2, e3, e5
            (Simd32x3::from(self[e321] * -1.0) * other.group0().xyz()).with_w((self[e235] * other[e23]) + (self[e315] * other[e31]) + (self[e125] * other[e12])),
        )
    }
}
impl WeightContraction<AntiPlane> for AntiFlatPoint {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        3       10        0        0
    fn weight_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        AntiLine::from_groups(
            // e23, e31, e12
            right_anti_dual_g0_xyz * Simd32x3::from(self[e321] * -1.0),
            // e15, e25, e35
            (right_anti_dual_g0_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g0_xyz.yzx() * self.group0().zxy()),
        )
    }
}
impl WeightContraction<Circle> for AntiFlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e321] * other[e321]) - (self[e235] * other[e423]) - (self[e315] * other[e431]) - (self[e125] * other[e412]),
        )
    }
}
impl WeightContraction<CircleRotor> for AntiFlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e321] * other[e321]) - (self[e235] * other[e423]) - (self[e315] * other[e431]) - (self[e125] * other[e412]),
        )
    }
}
impl WeightContraction<Dipole> for AntiFlatPoint {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        3        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       13       15        0        0
    fn weight_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (self.group0().yzxx() * right_anti_dual_g0.zxy().with_w(other[e23])) + Simd32x3::from(0.0).with_w((self[e315] * other[e31]) + (self[e125] * other[e12]))
                - (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                - (right_anti_dual_g0.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl WeightContraction<DipoleInversion> for AntiFlatPoint {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        3        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       13       15        0        0
    fn weight_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (self.group0().yzxx() * right_anti_dual_g0.zxy().with_w(other[e23])) + Simd32x3::from(0.0).with_w((self[e315] * other[e31]) + (self[e125] * other[e12]))
                - (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                - (right_anti_dual_g0.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl WeightContraction<MultiVector> for AntiFlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        9        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2       10        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        9       20        0      N/A
    //  no simd       19       41        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_anti_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_anti_dual_g9 = other.group1().xyz().with_w(other[e5] * -1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self[e321] * other[e321]) - (self[e235] * other[e423]) - (self[e315] * other[e431]) - (self[e125] * other[e412]), 0.0]),
            // e1, e2, e3, e4
            (right_anti_dual_g6_xyz * Simd32x3::from(self[e321])).with_w(0.0) + (right_anti_dual_g7.zxy() * self.group0().yzx()).with_w(0.0)
                - (right_anti_dual_g7.yzx() * self.group0().zxy()).with_w(0.0),
            // e5
            -(right_anti_dual_g6_xyz[0] * self[e235]) - (right_anti_dual_g6_xyz[1] * self[e315]) - (right_anti_dual_g6_xyz[2] * self[e125]),
            // e15, e25, e35, e45
            ((right_anti_dual_g9.zxy() * self.group0().yzx()) - (right_anti_dual_g9.yzx() * self.group0().zxy())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            -(Simd32x3::from(self[e321]) * right_anti_dual_g9.xyz()) - (Simd32x3::from(other[e4]) * self.group0().xyz()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(right_anti_dual_g0[1] * self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(right_anti_dual_g0[1]) * self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<RoundPoint> for AntiFlatPoint {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        4        0      N/A
    // no simd        6       12        0        0
    fn weight_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        AntiLine::from_groups(
            // e23, e31, e12
            -(right_anti_dual_g0_xyz * Simd32x3::from(self[e321])) - (Simd32x3::from(other[e4]) * self.group0().xyz()),
            // e15, e25, e35
            (right_anti_dual_g0_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g0_xyz.yzx() * self.group0().zxy()),
        )
    }
}
impl WeightContraction<Scalar> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl WeightContraction<VersorEven> for AntiFlatPoint {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       12       16        0        0
    fn weight_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g3_xyz = other.group3().xyz();
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w((self[e321] * other[e321]) - (right_anti_dual_g0_xyz[2] * self[e125]))
                - (self.group0().xyzy() * Simd32x3::from(other[e4]).with_w(right_anti_dual_g0_xyz[1]))
                - (self.group0().wwwx() * right_anti_dual_g3_xyz.with_w(right_anti_dual_g0_xyz[0])),
            // e15, e25, e35, e3215
            ((right_anti_dual_g3_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g3_xyz.yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl WeightContraction<VersorOdd> for AntiFlatPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       13       16        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1, e2, e3, e5
            (self.group0().zxyx() * other.group0().yzx().with_w(other[e23])) + Simd32x3::from(0.0).with_w((self[e315] * other[e31]) + (self[e125] * other[e12]))
                - (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                - (self.group0().yzx() * other.group0().zxy()).with_w(0.0),
        )
    }
}
impl std::ops::Div<WeightContractionInfix> for AntiFlector {
    type Output = WeightContractionInfixPartial<AntiFlector>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        3        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd       17       23        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1, e2, e3, e5
            (Simd32x4::from(other[scalar]) * self.group1())
                + (self.group0().yzxx() * right_anti_dual_g0.zxy().with_w(other[e23]))
                + Simd32x3::from(0.0).with_w((other[e31] * self[e315]) + (other[e12] * self[e125]))
                - (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                - (right_anti_dual_g0.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl WeightContraction<AntiDipoleInversion> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       16       20        0        0
    fn weight_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(
                (right_anti_dual_g3_xyz[0] * self[e1]) + (right_anti_dual_g3_xyz[1] * self[e2]) + (right_anti_dual_g3_xyz[2] * self[e3]) + (other[e321] * self[e321])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
            ) - (Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[e5]))
                - (self.group0().wwwx() * right_anti_dual_g3_xyz.with_w(other[e423])),
            // e15, e25, e35, e3215
            ((right_anti_dual_g3_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g3_xyz.yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl WeightContraction<AntiDualNum> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl WeightContraction<AntiFlatPoint> for AntiFlector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e321] * self[e321])
    }
}
impl WeightContraction<AntiFlector> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        6       14        0        0
    fn weight_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (right_anti_dual_g1_xyz * Simd32x3::from(self[e321] * -1.0))
                .with_w((right_anti_dual_g1_xyz[0] * self[e1]) + (right_anti_dual_g1_xyz[1] * self[e2]) + (right_anti_dual_g1_xyz[2] * self[e3]) + (other[e321] * self[e321])),
            // e15, e25, e35, e3215
            ((right_anti_dual_g1_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g1_xyz.yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl WeightContraction<AntiLine> for AntiFlector {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        9        0        0
    fn weight_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (right_anti_dual_g0 * Simd32x3::from(self[e321]))
                .with_w(-(right_anti_dual_g0[0] * self[e235]) - (right_anti_dual_g0[1] * self[e315]) - (right_anti_dual_g0[2] * self[e125])),
        )
    }
}
impl WeightContraction<AntiMotor> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd       10       18        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_anti_dual_g0[3]) * self.group0(),
            // e1, e2, e3, e5
            (right_anti_dual_g0 * Simd32x3::from(self[e321]).with_w(self[e5]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[0] * self[e235]) - (right_anti_dual_g0[1] * self[e315]) - (right_anti_dual_g0[2] * self[e125]))
                + (Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz()).with_w(0.0),
        )
    }
}
impl WeightContraction<AntiPlane> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        5       13        0        0
    fn weight_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (right_anti_dual_g0_xyz * Simd32x3::from(self[e321] * -1.0))
                .with_w((right_anti_dual_g0_xyz[0] * self[e1]) + (right_anti_dual_g0_xyz[1] * self[e2]) + (right_anti_dual_g0_xyz[2] * self[e3])),
            // e15, e25, e35, e3215
            ((right_anti_dual_g0_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g0_xyz.yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl WeightContraction<Circle> for AntiFlector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e321] * other[e321]) - (self[e235] * other[e423]) - (self[e315] * other[e431]) - (self[e125] * other[e412]),
        )
    }
}
impl WeightContraction<CircleRotor> for AntiFlector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e321] * other[e321]) - (self[e235] * other[e423]) - (self[e315] * other[e431]) - (self[e125] * other[e412]),
        )
    }
}
impl WeightContraction<Dipole> for AntiFlector {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        3        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       13       15        0        0
    fn weight_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (self.group0().yzxx() * right_anti_dual_g0.zxy().with_w(other[e23])) + Simd32x3::from(0.0).with_w((self[e315] * other[e31]) + (self[e125] * other[e12]))
                - (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                - (right_anti_dual_g0.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl WeightContraction<DipoleInversion> for AntiFlector {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        3        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       13       15        0        0
    fn weight_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (self.group0().yzxx() * right_anti_dual_g0.zxy().with_w(other[e23])) + Simd32x3::from(0.0).with_w((self[e315] * other[e31]) + (self[e125] * other[e12]))
                - (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                - (right_anti_dual_g0.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl WeightContraction<MultiVector> for AntiFlector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       14        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2       11        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd       15       26        0      N/A
    //  no simd       28       49        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_anti_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_anti_dual_g9_xyz = other.group1().xyz();
        let right_anti_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_anti_dual_g10 * self[e5])
                    + (right_anti_dual_g9_xyz[0] * self[e1])
                    + (right_anti_dual_g9_xyz[1] * self[e2])
                    + (right_anti_dual_g9_xyz[2] * self[e3])
                    + (self[e321] * other[e321])
                    - (self[e235] * other[e423])
                    - (self[e315] * other[e431])
                    - (self[e125] * other[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (right_anti_dual_g6_xyz * Simd32x3::from(self[e321])).with_w(0.0)
                + (Simd32x3::from(right_anti_dual_g0[1]) * self.group1().xyz()).with_w(0.0)
                + (right_anti_dual_g7.zxy() * self.group0().yzx()).with_w(0.0)
                - (right_anti_dual_g7.yzx() * self.group0().zxy()).with_w(0.0),
            // e5
            (right_anti_dual_g0[1] * self[e5]) - (right_anti_dual_g6_xyz[0] * self[e235]) - (right_anti_dual_g6_xyz[1] * self[e315]) - (right_anti_dual_g6_xyz[2] * self[e125]),
            // e15, e25, e35, e45
            ((right_anti_dual_g9_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g9_xyz.yzx() * self.group0().zxy())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from(right_anti_dual_g10) * self.group0().xyz()) - (right_anti_dual_g9_xyz * Simd32x3::from(self[e321])),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(right_anti_dual_g0[1] * self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(right_anti_dual_g0[1]) * self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<RoundPoint> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       13       16        0        0
    fn weight_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w((right_anti_dual_g0_xyz[0] * self[e1]) + (right_anti_dual_g0_xyz[1] * self[e2]) + (right_anti_dual_g0_xyz[2] * self[e3]))
                - (Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[e5]))
                - (right_anti_dual_g0_xyz * Simd32x3::from(self[e321])).with_w(0.0),
            // e15, e25, e35, e3215
            ((right_anti_dual_g0_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g0_xyz.yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl WeightContraction<Scalar> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl WeightContraction<VersorEven> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       16       20        0        0
    fn weight_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g3_xyz = other.group3().xyz();
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(
                (right_anti_dual_g3_xyz[0] * self[e1]) + (right_anti_dual_g3_xyz[1] * self[e2]) + (right_anti_dual_g3_xyz[2] * self[e3]) + (self[e321] * other[e321])
                    - (right_anti_dual_g0_xyz[1] * self[e315])
                    - (right_anti_dual_g0_xyz[2] * self[e125]),
            ) - (Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[e5]))
                - (self.group0().wwwx() * right_anti_dual_g3_xyz.with_w(right_anti_dual_g0_xyz[0])),
            // e15, e25, e35, e3215
            ((right_anti_dual_g3_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g3_xyz.yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl WeightContraction<VersorOdd> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       17       20        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1, e2, e3, e5
            (Simd32x4::from(other[scalar]) * self.group1())
                + (self.group0().zxyx() * other.group0().yzx().with_w(other[e23]))
                + Simd32x3::from(0.0).with_w((self[e315] * other[e31]) + (self[e125] * other[e12]))
                - (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                - (self.group0().yzx() * other.group0().zxy()).with_w(0.0),
        )
    }
}
impl std::ops::Div<WeightContractionInfix> for AntiLine {
    type Output = WeightContractionInfixPartial<AntiLine>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       12        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(other[scalar]) * self.group0()).with_w(
                (other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) + (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]),
            ),
            // e15, e25, e35, e3215
            (Simd32x3::from(other[scalar]) * self.group1()).with_w(0.0),
        )
    }
}
impl WeightContraction<AntiDipoleInversion> for AntiLine {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        3        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd       12       12        0        0
    fn weight_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(right_anti_dual_g3_xyz[2] * self[e35])
                + (Simd32x3::from(other[e4]) * self.group1()).with_w(right_anti_dual_g3_xyz[1] * self[e25])
                + (right_anti_dual_g3_xyz.zxy() * self.group0().yzx()).with_w(right_anti_dual_g3_xyz[0] * self[e15])
                - (right_anti_dual_g3_xyz.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl WeightContraction<AntiDualNum> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(other[scalar]) * self.group1(),
        )
    }
}
impl WeightContraction<AntiFlector> for AntiLine {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        9        9        0        0
    fn weight_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w((right_anti_dual_g1_xyz[1] * self[e25]) + (right_anti_dual_g1_xyz[2] * self[e35]))
                + (right_anti_dual_g1_xyz.zxy() * self.group0().yzx()).with_w(right_anti_dual_g1_xyz[0] * self[e15])
                - (right_anti_dual_g1_xyz.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl WeightContraction<AntiLine> for AntiLine {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn weight_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]))
    }
}
impl WeightContraction<AntiMotor> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        9        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(other[scalar]) * self.group0()).with_w((self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12])),
            // e15, e25, e35, e3215
            (Simd32x3::from(other[scalar]) * self.group1()).with_w(0.0),
        )
    }
}
impl WeightContraction<AntiPlane> for AntiLine {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        9        9        0        0
    fn weight_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w((right_anti_dual_g0_xyz[1] * self[e25]) + (right_anti_dual_g0_xyz[2] * self[e35]))
                + (right_anti_dual_g0_xyz.zxy() * self.group0().yzx()).with_w(right_anti_dual_g0_xyz[0] * self[e15])
                - (right_anti_dual_g0_xyz.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl WeightContraction<Dipole> for AntiLine {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
    fn weight_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]) + (self[e15] * other[e41]) + (self[e25] * other[e42]) + (self[e35] * other[e43]),
        )
    }
}
impl WeightContraction<DipoleInversion> for AntiLine {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
    fn weight_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]) + (self[e15] * other[e41]) + (self[e25] * other[e42]) + (self[e35] * other[e43]),
        )
    }
}
impl WeightContraction<MultiVector> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7        9        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        5        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        9       15        0      N/A
    //  no simd       15       26        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g9_xyz = other.group1().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]) + (self[e15] * other[e41]) + (self[e25] * other[e42]) + (self[e35] * other[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(other[e4]) * self.group1()).with_w(0.0) + (right_anti_dual_g9_xyz.zxy() * self.group0().yzx()).with_w(0.0)
                - (right_anti_dual_g9_xyz.yzx() * self.group0().zxy()).with_w(0.0),
            // e5
            (right_anti_dual_g9_xyz[0] * self[e15]) + (right_anti_dual_g9_xyz[1] * self[e25]) + (right_anti_dual_g9_xyz[2] * self[e35]),
            // e15, e25, e35, e45
            (Simd32x3::from(right_anti_dual_g0[1]) * self.group1()).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(right_anti_dual_g0[1]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<RoundPoint> for AntiLine {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        3        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd       12       12        0        0
    fn weight_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(right_anti_dual_g0_xyz[2] * self[e35])
                + (Simd32x3::from(other[e4]) * self.group1()).with_w(right_anti_dual_g0_xyz[1] * self[e25])
                + (right_anti_dual_g0_xyz.zxy() * self.group0().yzx()).with_w(right_anti_dual_g0_xyz[0] * self[e15])
                - (right_anti_dual_g0_xyz.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl WeightContraction<Scalar> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(other[scalar]) * self.group1(),
        )
    }
}
impl WeightContraction<VersorEven> for AntiLine {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        3        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd       12       12        0        0
    fn weight_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(right_anti_dual_g3_xyz[2] * self[e35])
                + (Simd32x3::from(other[e4]) * self.group1()).with_w(right_anti_dual_g3_xyz[1] * self[e25])
                + (right_anti_dual_g3_xyz.zxy() * self.group0().yzx()).with_w(right_anti_dual_g3_xyz[0] * self[e15])
                - (right_anti_dual_g3_xyz.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl WeightContraction<VersorOdd> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       12        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(other[scalar]) * self.group0()).with_w(
                (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]) + (self[e15] * other[e41]) + (self[e25] * other[e42]) + (self[e35] * other[e43]),
            ),
            // e15, e25, e35, e3215
            (Simd32x3::from(other[scalar]) * self.group1()).with_w(0.0),
        )
    }
}
impl std::ops::Div<WeightContractionInfix> for AntiMotor {
    type Output = WeightContractionInfixPartial<AntiMotor>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        7        0        0
    //    simd3        1        4        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        7       12        0      N/A
    //  no simd       15       23        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x4::from(other[scalar]) * self.group0())
                + Simd32x3::from(0.0).with_w(
                    (other[e31] * self[e31]) + (other[e12] * self[e12])
                        - (right_anti_dual_g0[0] * self[e15])
                        - (right_anti_dual_g0[1] * self[e25])
                        - (right_anti_dual_g0[2] * self[e35]),
                )
                + (right_anti_dual_g0 * Simd32x3::from(self[e3215])).with_w(other[e23] * self[e23]),
            // e15, e25, e35, e3215
            ((Simd32x3::from(other[scalar]) * self.group1().xyz()) - (Simd32x3::from(self[e3215]) * other.group1().xyz())).with_w(other[scalar] * self[e3215]),
        )
    }
}
impl WeightContraction<AntiDipoleInversion> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        0        4        0      N/A
    //    simd4        4        1        0      N/A
    // Totals...
    // yes simd        6       11        0      N/A
    //  no simd       18       22        0        0
    fn weight_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215] * -1.0) * right_anti_dual_g3_xyz.with_w(right_anti_dual_g2_w),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w((right_anti_dual_g3_xyz[1] * self[e25]) + (right_anti_dual_g3_xyz[2] * self[e35]) + (other[e321] * self[e3215]))
                + (right_anti_dual_g3_xyz.zxy() * self.group0().yzx()).with_w(right_anti_dual_g3_xyz[0] * self[e15])
                - (Simd32x3::from(right_anti_dual_g2_w) * self.group1().xyz()).with_w(0.0)
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (right_anti_dual_g3_xyz.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl WeightContraction<AntiDualNum> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * self.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl WeightContraction<AntiFlatPoint> for AntiMotor {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([other[e321] * self[e3215], 0.0]))
    }
}
impl WeightContraction<AntiFlector> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd3        0        3        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd       10       14        0        0
    fn weight_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (right_anti_dual_g1_xyz * Simd32x3::from(self[e3215] * -1.0)).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w((right_anti_dual_g1_xyz[1] * self[e25]) + (right_anti_dual_g1_xyz[2] * self[e35]) + (other[e321] * self[e3215]))
                + (right_anti_dual_g1_xyz.zxy() * self.group0().yzx()).with_w(right_anti_dual_g1_xyz[0] * self[e15])
                - (right_anti_dual_g1_xyz.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl WeightContraction<AntiLine> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        9        0        0
    fn weight_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[0] * self[e23]) - (right_anti_dual_g0[1] * self[e31]) - (right_anti_dual_g0[2] * self[e12])),
            // e15, e25, e35, e3215
            (right_anti_dual_g0 * Simd32x3::from(self[e3215])).with_w(0.0),
        )
    }
}
impl WeightContraction<AntiMotor> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        6       14        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(other[scalar]) * self.group0().xyz())
                .with_w((other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[scalar] * self[scalar])),
            // e15, e25, e35, e3215
            ((Simd32x3::from(other[scalar]) * self.group1().xyz()) - (Simd32x3::from(self[e3215]) * other.group0().xyz())).with_w(other[scalar] * self[e3215]),
        )
    }
}
impl WeightContraction<AntiPlane> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd3        0        3        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        9       13        0        0
    fn weight_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (right_anti_dual_g0_xyz * Simd32x3::from(self[e3215] * -1.0)).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w((right_anti_dual_g0_xyz[1] * self[e25]) + (right_anti_dual_g0_xyz[2] * self[e35]))
                + (right_anti_dual_g0_xyz.zxy() * self.group0().yzx()).with_w(right_anti_dual_g0_xyz[0] * self[e15])
                - (right_anti_dual_g0_xyz.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl WeightContraction<Circle> for AntiMotor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
    fn weight_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e3215] * -1.0) * other.group0().with_w(other[e321] * -1.0))
    }
}
impl WeightContraction<CircleRotor> for AntiMotor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
    fn weight_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e3215] * -1.0) * other.group0().with_w(other[e321] * -1.0))
    }
}
impl WeightContraction<Dipole> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        0        3        0      N/A
    // Totals...
    // yes simd        5       10        0      N/A
    //  no simd        5       16        0        0
    fn weight_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (right_anti_dual_g0 * Simd32x3::from(self[e3215])).with_w(
                (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12])
                    - (right_anti_dual_g0[0] * self[e15])
                    - (right_anti_dual_g0[1] * self[e25])
                    - (right_anti_dual_g0[2] * self[e35]),
            ),
            // e15, e25, e35, e3215
            (Simd32x3::from(self[e3215] * -1.0) * other.group1().xyz()).with_w(0.0),
        )
    }
}
impl WeightContraction<DipoleInversion> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        0        3        0      N/A
    // Totals...
    // yes simd        6       11        0      N/A
    //  no simd        6       17        0        0
    fn weight_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (right_anti_dual_g0 * Simd32x3::from(self[e3215])).with_w(
                (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]) + (self[e3215] * other[e1234])
                    - (right_anti_dual_g0[0] * self[e15])
                    - (right_anti_dual_g0[1] * self[e25])
                    - (right_anti_dual_g0[2] * self[e35]),
            ),
            // e15, e25, e35, e3215
            (Simd32x3::from(self[e3215] * -1.0) * other.group1().xyz()).with_w(0.0),
        )
    }
}
impl WeightContraction<MultiVector> for AntiMotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       17        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2       11        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd       15       29        0      N/A
    //  no simd       28       52        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_anti_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_anti_dual_g9_xyz = other.group1().xyz();
        let right_anti_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_anti_dual_g0[1] * self[scalar]) + (self[e3215] * other[e1234])
                    - (right_anti_dual_g6_xyz[0] * self[e23])
                    - (right_anti_dual_g6_xyz[1] * self[e31])
                    - (right_anti_dual_g6_xyz[2] * self[e12])
                    - (right_anti_dual_g7[0] * self[e15])
                    - (right_anti_dual_g7[1] * self[e25])
                    - (right_anti_dual_g7[2] * self[e35]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (right_anti_dual_g9_xyz.zxy() * self.group0().yzx()).with_w(0.0)
                - (Simd32x3::from(right_anti_dual_g10) * self.group1().xyz()).with_w(0.0)
                - (Simd32x3::from(self[e3215]) * other.group7()).with_w(0.0)
                - (right_anti_dual_g9_xyz.yzx() * self.group0().zxy()).with_w(0.0),
            // e5
            (right_anti_dual_g9_xyz[0] * self[e15]) + (right_anti_dual_g9_xyz[1] * self[e25]) + (right_anti_dual_g9_xyz[2] * self[e35]) + (self[e3215] * other[e321]),
            // e15, e25, e35, e45
            ((right_anti_dual_g6_xyz * Simd32x3::from(self[e3215])) + (Simd32x3::from(right_anti_dual_g0[1]) * self.group1().xyz())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (right_anti_dual_g7 * Simd32x3::from(self[e3215])) + (Simd32x3::from(right_anti_dual_g0[1]) * self.group0().xyz()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(right_anti_dual_g10 * self[e3215] * -1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            right_anti_dual_g9_xyz * Simd32x3::from(self[e3215] * -1.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(right_anti_dual_g0[1] * self[e3215]),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<RoundPoint> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        5        0        0
    //    simd3        0        3        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        9        0      N/A
    //  no simd       13       18        0        0
    fn weight_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1 = other[e4] * -1.0;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215] * -1.0) * right_anti_dual_g0_xyz.with_w(right_anti_dual_g1),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w((right_anti_dual_g0_xyz[1] * self[e25]) + (right_anti_dual_g0_xyz[2] * self[e35]))
                + (right_anti_dual_g0_xyz.zxy() * self.group0().yzx()).with_w(right_anti_dual_g0_xyz[0] * self[e15])
                - (Simd32x3::from(right_anti_dual_g1) * self.group1().xyz()).with_w(0.0)
                - (right_anti_dual_g0_xyz.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl WeightContraction<Scalar> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * self.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl WeightContraction<Sphere> for AntiMotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e3215] * other[e1234])
    }
}
impl WeightContraction<VersorEven> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        0        4        0      N/A
    //    simd4        4        1        0      N/A
    // Totals...
    // yes simd        6       11        0      N/A
    //  no simd       18       22        0        0
    fn weight_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215] * -1.0) * right_anti_dual_g3_xyz.with_w(right_anti_dual_g2_w),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w((right_anti_dual_g3_xyz[1] * self[e25]) + (right_anti_dual_g3_xyz[2] * self[e35]) + (self[e3215] * other[e321]))
                + (right_anti_dual_g3_xyz.zxy() * self.group0().yzx()).with_w(right_anti_dual_g3_xyz[0] * self[e15])
                - (Simd32x3::from(right_anti_dual_g2_w) * self.group1().xyz()).with_w(0.0)
                - (Simd32x3::from(self[e3215]) * other.group0().xyz()).with_w(0.0)
                - (right_anti_dual_g3_xyz.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl WeightContraction<VersorOdd> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        8       12        0      N/A
    //  no simd       16       25        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (right_anti_dual_g0 * Simd32x3::from(self[e3215]).with_w(self[scalar]))
                + (self.group0().xyzx() * Simd32x3::from(right_anti_dual_g0[3]).with_w(other[e23]))
                + Simd32x3::from(0.0).with_w(
                    (self[e31] * other[e31]) + (self[e12] * other[e12]) + (self[e3215] * other[e1234])
                        - (right_anti_dual_g0[0] * self[e15])
                        - (right_anti_dual_g0[1] * self[e25])
                        - (right_anti_dual_g0[2] * self[e35]),
                ),
            // e15, e25, e35, e3215
            ((Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz()) - (Simd32x3::from(self[e3215]) * other.group1().xyz())).with_w(right_anti_dual_g0[3] * self[e3215]),
        )
    }
}
impl std::ops::Div<WeightContractionInfix> for AntiPlane {
    type Output = WeightContractionInfixPartial<AntiPlane>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl WeightContraction<AntiDipoleInversion> for AntiPlane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        Scalar::from_groups(
            // scalar
            (right_anti_dual_g3_xyz[0] * self[e1]) + (right_anti_dual_g3_xyz[1] * self[e2]) + (right_anti_dual_g3_xyz[2] * self[e3]) - (other[e4] * self[e5]),
        )
    }
}
impl WeightContraction<AntiDualNum> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl WeightContraction<AntiFlector> for AntiPlane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn weight_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        Scalar::from_groups(
            // scalar
            (right_anti_dual_g1_xyz[0] * self[e1]) + (right_anti_dual_g1_xyz[1] * self[e2]) + (right_anti_dual_g1_xyz[2] * self[e3]),
        )
    }
}
impl WeightContraction<AntiMotor> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl WeightContraction<AntiPlane> for AntiPlane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn weight_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Scalar::from_groups(
            // scalar
            (right_anti_dual_g0_xyz[0] * self[e1]) + (right_anti_dual_g0_xyz[1] * self[e2]) + (right_anti_dual_g0_xyz[2] * self[e3]),
        )
    }
}
impl WeightContraction<MultiVector> for AntiPlane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       10        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g9_xyz = other.group1().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_anti_dual_g9_xyz[0] * self[e1]) + (right_anti_dual_g9_xyz[1] * self[e2]) + (right_anti_dual_g9_xyz[2] * self[e3]) - (self[e5] * other[e4]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(right_anti_dual_g0[1]) * self.group0().xyz()).with_w(0.0),
            // e5
            right_anti_dual_g0[1] * self[e5],
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
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<RoundPoint> for AntiPlane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Scalar::from_groups(
            // scalar
            (right_anti_dual_g0_xyz[0] * self[e1]) + (right_anti_dual_g0_xyz[1] * self[e2]) + (right_anti_dual_g0_xyz[2] * self[e3]) - (self[e5] * other[e4]),
        )
    }
}
impl WeightContraction<Scalar> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl WeightContraction<VersorEven> for AntiPlane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        Scalar::from_groups(
            // scalar
            (right_anti_dual_g3_xyz[0] * self[e1]) + (right_anti_dual_g3_xyz[1] * self[e2]) + (right_anti_dual_g3_xyz[2] * self[e3]) - (self[e5] * other[e4]),
        )
    }
}
impl WeightContraction<VersorOdd> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl std::ops::Div<WeightContractionInfix> for AntiScalar {
    type Output = WeightContractionInfixPartial<AntiScalar>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for AntiScalar {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       20        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345] * -1.0) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            Simd32x4::from(self[e12345]) * other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl WeightContraction<AntiDipoleInversion> for AntiScalar {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        6        0      N/A
    // Totals...
    // yes simd        0        7        0      N/A
    //  no simd        0       27        0        0
    fn weight_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(self[e12345]) * other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightContraction<AntiDualNum> for AntiScalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        0        1        0      N/A
    // no simd        0        2        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(self[e12345]) * other.group0())
    }
}
impl WeightContraction<AntiFlatPoint> for AntiScalar {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl WeightContraction<AntiFlector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn weight_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightContraction<AntiLine> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn weight_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(self[e12345] * -1.0) * other.group0(),
            // e235, e315, e125
            Simd32x3::from(self[e12345] * -1.0) * other.group1(),
        )
    }
}
impl WeightContraction<AntiMotor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl WeightContraction<AntiPlane> for AntiScalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightContraction<AntiScalar> for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_contraction(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e12345] * self[e12345] * -1.0)
    }
}
impl WeightContraction<Circle> for AntiScalar {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       14        0        0
    fn weight_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(self[e12345]) * other.group2(),
        )
    }
}
impl WeightContraction<CircleRotor> for AntiScalar {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       19        0        0
    fn weight_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            Simd32x4::from(self[e12345]) * other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightContraction<Dipole> for AntiScalar {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       16        0        0
    fn weight_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345] * -1.0) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            Simd32x3::from(self[e12345] * -1.0) * other.group2(),
        )
    }
}
impl WeightContraction<DipoleInversion> for AntiScalar {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        6        0      N/A
    // Totals...
    // yes simd        0        8        0      N/A
    //  no simd        0       28        0        0
    fn weight_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345] * -1.0) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            Simd32x4::from(self[e12345]) * other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            Simd32x4::from(self[e12345]) * other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl WeightContraction<DualNum> for AntiScalar {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        3        0        0
    fn weight_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(self[e12345] * -1.0) * other.group0())
    }
}
impl WeightContraction<FlatPoint> for AntiScalar {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl WeightContraction<Flector> for AntiScalar {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn weight_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl WeightContraction<Line> for AntiScalar {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn weight_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(self[e12345]) * other.group0(),
            // e15, e25, e35
            Simd32x3::from(self[e12345]) * other.group1(),
        )
    }
}
impl WeightContraction<Motor> for AntiScalar {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn weight_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightContraction<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd2        0        2        0      N/A
    //    simd3        0        6        0      N/A
    //    simd4        0        6        0      N/A
    // Totals...
    // yes simd        0       19        0      N/A
    //  no simd        0       51        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(self[e12345]) * other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * (other.group9().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]),
            // e5
            self[e12345] * other[e3215],
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * other.group8().with_w(other[e321]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group7(),
            // e23, e31, e12
            Simd32x3::from(self[e12345]) * other.group6().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * (other.group5() * Simd32x3::from(-1.0)).with_w(other[e45]),
            // e423, e431, e412
            Simd32x3::from(self[e12345] * -1.0) * other.group4(),
            // e235, e315, e125
            Simd32x3::from(self[e12345] * -1.0) * other.group3().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            self[e12345] * other[e4] * -1.0,
        )
    }
}
impl WeightContraction<Plane> for AntiScalar {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl WeightContraction<RoundPoint> for AntiScalar {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn weight_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            self[e12345] * other[e4] * -1.0,
        )
    }
}
impl WeightContraction<Scalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e12345] * other[scalar])
    }
}
impl WeightContraction<Sphere> for AntiScalar {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        8        0        0
    fn weight_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]),
            // e5
            self[e12345] * other[e3215],
        )
    }
}
impl WeightContraction<VersorEven> for AntiScalar {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        8        0      N/A
    // no simd        0       32        0        0
    fn weight_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(self[e12345]) * other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightContraction<VersorOdd> for AntiScalar {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        6        0      N/A
    // Totals...
    // yes simd        0        8        0      N/A
    //  no simd        0       30        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            Simd32x4::from(self[e12345]) * (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(other[e3215]),
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]),
        )
    }
}
impl std::ops::Div<WeightContractionInfix> for Circle {
    type Output = WeightContractionInfixPartial<Circle>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       10        0        0
    //    simd3        0        6        0      N/A
    //    simd4        6        3        0      N/A
    // Totals...
    // yes simd       13       19        0      N/A
    //  no simd       31       40        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(other[scalar]) * self.group2()).with_w(
                (other[e41] * self[e415])
                    + (other[e42] * self[e425])
                    + (other[e43] * self[e435])
                    + (other[e23] * self[e423])
                    + (other[e31] * self[e431])
                    + (other[e12] * self[e412]),
            ),
            // e1, e2, e3, e5
            (Simd32x4::from([self[e431], self[e412], self[e423], self[e415]]) * other.group2().zxyx())
                + (other.group1().wwwx() * self.group1().xyz().with_w(self[e235]))
                + Simd32x3::from(0.0).with_w((other[e12] * self[e125]) + (other[e25] * self[e425]) + (other[e35] * self[e435]))
                + (other.group0().yzx() * self.group2().zxy()).with_w(other[e31] * self[e315])
                - (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                - (other.group0().zxy() * self.group2().yzx()).with_w(0.0)
                - (self.group0().zxy() * other.group2().yzx()).with_w(0.0),
        )
    }
}
impl WeightContraction<AntiDipoleInversion> for Circle {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       15        0        0
    //    simd3        2        8        0      N/A
    //    simd4        6        1        0      N/A
    // Totals...
    // yes simd       17       24        0      N/A
    //  no simd       39       43        0        0
    fn weight_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        let right_anti_dual_g3_w = other[e5] * -1.0;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(right_anti_dual_g2_w) * self.group1().xyz()) + (right_anti_dual_g3_xyz.yzx() * self.group0().zxy())
                - (right_anti_dual_g3_xyz.zxy() * self.group0().yzx()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g3_xyz[1] * self[e425]) - (right_anti_dual_g3_xyz[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g2_w) * self.group2()).with_w(0.0)
                + (Simd32x3::from(right_anti_dual_g3_w) * self.group0()).with_w(0.0)
                - (self.group1().wwwx() * right_anti_dual_g3_xyz.with_w(right_anti_dual_g3_xyz[0])),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(
                -(right_anti_dual_g1_xyz[0] * self[e415])
                    - (right_anti_dual_g1_xyz[1] * self[e425])
                    - (right_anti_dual_g1_xyz[2] * self[e435])
                    - (right_anti_dual_g2_xyz[0] * self[e423])
                    - (right_anti_dual_g2_xyz[1] * self[e431])
                    - (right_anti_dual_g2_xyz[2] * self[e412])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
            ) + (Simd32x3::from(right_anti_dual_g3_w) * self.group1().xyz()).with_w(0.0)
                + (right_anti_dual_g3_xyz.zxy() * self.group2().yzx()).with_w(0.0)
                - (right_anti_dual_g3_xyz.yzx() * self.group2().zxy()).with_w(other[e321] * self[e321] * -1.0),
        )
    }
}
impl WeightContraction<AntiDualNum> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       10        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * self.group2(),
        )
    }
}
impl WeightContraction<AntiFlatPoint> for Circle {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Scalar::from_groups(
            // scalar
            (other[e321] * self[e321]) - (right_anti_dual_g0_xyz[0] * self[e423]) - (right_anti_dual_g0_xyz[1] * self[e431]) - (right_anti_dual_g0_xyz[2] * self[e412]),
        )
    }
}
impl WeightContraction<AntiFlector> for Circle {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd3        1        6        0      N/A
    //    simd4        5        1        0      N/A
    // Totals...
    // yes simd        9       15        0      N/A
    //  no simd       26       30        0        0
    fn weight_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e5] * -1.0;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (right_anti_dual_g1_xyz.yzx() * self.group0().zxy()) - (right_anti_dual_g1_xyz.zxy() * self.group0().yzx()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g1_xyz[1] * self[e425]) - (right_anti_dual_g1_xyz[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g1_w) * self.group0()).with_w(0.0)
                - (self.group1().wwwx() * right_anti_dual_g1_xyz.with_w(right_anti_dual_g1_xyz[0])),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0_xyz[0] * self[e423]) - (right_anti_dual_g0_xyz[1] * self[e431]) - (right_anti_dual_g0_xyz[2] * self[e412]))
                + (Simd32x3::from(right_anti_dual_g1_w) * self.group1().xyz()).with_w(0.0)
                + (right_anti_dual_g1_xyz.zxy() * self.group2().yzx()).with_w(0.0)
                - (right_anti_dual_g1_xyz.yzx() * self.group2().zxy()).with_w(other[e321] * self[e321] * -1.0),
        )
    }
}
impl WeightContraction<AntiLine> for Circle {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        0        4        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        9       13        0      N/A
    //  no simd       18       21        0        0
    fn weight_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e431]) - (right_anti_dual_g0[2] * self[e412]))
                + (right_anti_dual_g0 * Simd32x3::from(self[e321])).with_w(0.0)
                + (other.group1().zxy() * self.group0().yzx()).with_w(0.0)
                - (other.group1().yzx() * self.group0().zxy()).with_w(right_anti_dual_g0[0] * self[e423]),
            // e5
            (other[e15] * self[e415]) + (other[e25] * self[e425]) + (other[e35] * self[e435])
                - (right_anti_dual_g0[0] * self[e235])
                - (right_anti_dual_g0[1] * self[e315])
                - (right_anti_dual_g0[2] * self[e125]),
        )
    }
}
impl WeightContraction<AntiMotor> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        0        4        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd       18       28        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(other[scalar]) * self.group2()).with_w((other[e23] * self[e423]) + (other[e31] * self[e431]) + (other[e12] * self[e412])),
            // e1, e2, e3, e5
            (Simd32x4::from([self[e431], self[e412], self[e423], self[e415]]) * other.group1().zxyx())
                + Simd32x3::from(0.0)
                    .with_w((other[e23] * self[e235]) + (other[e31] * self[e315]) + (other[e12] * self[e125]) + (other[e25] * self[e425]) + (other[e35] * self[e435]))
                - (Simd32x3::from(self[e321]) * other.group0().xyz()).with_w(0.0)
                - (self.group0().zxy() * other.group1().yzx()).with_w(0.0),
        )
    }
}
impl WeightContraction<AntiPlane> for Circle {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        3        6        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        6       10        0      N/A
    //  no simd       18       25        0        0
    fn weight_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e5] * -1.0;
        Dipole::from_groups(
            // e41, e42, e43
            (right_anti_dual_g0_xyz.yzx() * self.group0().zxy()) - (right_anti_dual_g0_xyz.zxy() * self.group0().yzx()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0_xyz[1] * self[e425]) - (right_anti_dual_g0_xyz[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g0_w) * self.group0()).with_w(0.0)
                - (self.group1().wwwx() * right_anti_dual_g0_xyz.with_w(right_anti_dual_g0_xyz[0])),
            // e15, e25, e35
            (Simd32x3::from(right_anti_dual_g0_w) * self.group1().xyz()) + (right_anti_dual_g0_xyz.zxy() * self.group2().yzx())
                - (right_anti_dual_g0_xyz.yzx() * self.group2().zxy()),
        )
    }
}
impl WeightContraction<Circle> for Circle {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
    fn weight_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        Scalar::from_groups(
            // scalar
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
impl WeightContraction<CircleRotor> for Circle {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
    fn weight_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        Scalar::from_groups(
            // scalar
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
        )
    }
}
impl WeightContraction<Dipole> for Circle {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       11        0        0
    //    simd3        0        5        0      N/A
    //    simd4        6        1        0      N/A
    // Totals...
    // yes simd       13       17        0      N/A
    //  no simd       31       30        0        0
    fn weight_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (other.group1().wwwx() * self.group1().xyz().with_w(self[e423]))
                + Simd32x3::from(0.0).with_w((self[e415] * other[e41]) + (self[e425] * other[e42]) + (self[e435] * other[e43]))
                + (self.group0().yzx() * other.group2().zxy()).with_w(self[e431] * other[e31])
                + (self.group2().zxy() * other.group0().yzx()).with_w(self[e412] * other[e12])
                - (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                - (self.group0().zxy() * other.group2().yzx()).with_w(0.0)
                - (self.group2().yzx() * other.group0().zxy()).with_w(0.0),
            // e5
            (self[e415] * other[e15]) + (self[e425] * other[e25]) + (self[e435] * other[e35]) + (self[e235] * other[e23]) + (self[e315] * other[e31]) + (self[e125] * other[e12]),
        )
    }
}
impl WeightContraction<DipoleInversion> for Circle {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       11        0        0
    //    simd3        0        5        0      N/A
    //    simd4        6        1        0      N/A
    // Totals...
    // yes simd       13       17        0      N/A
    //  no simd       31       30        0        0
    fn weight_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (other.group1().wwwx() * self.group1().xyz().with_w(self[e423]))
                + Simd32x3::from(0.0).with_w((self[e415] * other[e41]) + (self[e425] * other[e42]) + (self[e435] * other[e43]))
                + (self.group0().yzx() * other.group2().zxy()).with_w(self[e431] * other[e31])
                + (self.group2().zxy() * other.group0().yzx()).with_w(self[e412] * other[e12])
                - (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                - (self.group0().zxy() * other.group2().yzx()).with_w(0.0)
                - (self.group2().yzx() * other.group0().zxy()).with_w(0.0),
            // e5
            (self[e415] * other[e15]) + (self[e425] * other[e25]) + (self[e435] * other[e35]) + (self[e235] * other[e23]) + (self[e315] * other[e31]) + (self[e125] * other[e12]),
        )
    }
}
impl WeightContraction<DualNum> for Circle {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn weight_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(other[e5] * -1.0) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(other[e5] * -1.0) * self.group1().xyz(),
        )
    }
}
impl WeightContraction<FlatPoint> for Circle {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        3        4        0      N/A
    //  no simd       12       12        0        0
    fn weight_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x4::from([self[e431], self[e412], self[e423], self[e415]]) * other.group0().zxyx())
                + (self.group1().xyzy() * other.group0().wwwy())
                + Simd32x3::from(0.0).with_w(self[e435] * other[e35])
                - (self.group0().zxy() * other.group0().yzx()).with_w(0.0),
        )
    }
}
impl WeightContraction<Flector> for Circle {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        3        4        0      N/A
    //  no simd       12       12        0        0
    fn weight_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x4::from([self[e431], self[e412], self[e423], self[e415]]) * other.group0().zxyx())
                + (self.group1().xyzy() * other.group0().wwwy())
                + Simd32x3::from(0.0).with_w(self[e435] * other[e35])
                - (self.group0().zxy() * other.group0().yzx()).with_w(0.0),
        )
    }
}
impl WeightContraction<Line> for Circle {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
    fn weight_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(self[e423] * other[e235])
                - (self[e431] * other[e315])
                - (self[e412] * other[e125])
                - (self[e415] * other[e415])
                - (self[e425] * other[e425])
                - (self[e435] * other[e435]),
        )
    }
}
impl WeightContraction<Motor> for Circle {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        9        0      N/A
    //  no simd        5       13        0        0
    fn weight_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e5] * -1.0;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(right_anti_dual_g1_w) * self.group0()).with_w(
                -(right_anti_dual_g0_xyz[0] * self[e415])
                    - (right_anti_dual_g0_xyz[1] * self[e425])
                    - (right_anti_dual_g0_xyz[2] * self[e435])
                    - (right_anti_dual_g1_xyz[0] * self[e423])
                    - (right_anti_dual_g1_xyz[1] * self[e431])
                    - (right_anti_dual_g1_xyz[2] * self[e412]),
            ),
            // e15, e25, e35, e3215
            (Simd32x3::from(right_anti_dual_g1_w) * self.group1().xyz()).with_w(0.0),
        )
    }
}
impl WeightContraction<MultiVector> for Circle {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       17       26        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4       17        0      N/A
    //    simd4        9        2        0      N/A
    // Totals...
    // yes simd       30       46        0      N/A
    //  no simd       65       87        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g5 = other.group6().xyz();
        let right_anti_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g9_xyz = other.group1().xyz();
        let right_anti_dual_g9_w = other[e5] * -1.0;
        let right_anti_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
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
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from([other[e45], other[e45], other[e45], other[e41]]) * self.group1().xyzx())
                + Simd32x3::from(0.0).with_w((self[e412] * other[e12]) + (self[e425] * other[e42]) + (self[e435] * other[e43]))
                + (right_anti_dual_g8.yzx() * self.group0().zxy()).with_w(self[e423] * other[e23])
                + (self.group2().zxy() * other.group4().yzx()).with_w(self[e431] * other[e31])
                - (Simd32x3::from(self[e321]) * other.group5()).with_w(0.0)
                - (right_anti_dual_g8.zxy() * self.group0().yzx()).with_w(0.0)
                - (self.group2().yzx() * other.group4().zxy()).with_w(0.0),
            // e5
            (self[e235] * other[e23]) + (self[e315] * other[e31]) + (self[e125] * other[e12])
                - (right_anti_dual_g8[0] * self[e415])
                - (right_anti_dual_g8[1] * self[e425])
                - (right_anti_dual_g8[2] * self[e435]),
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g9_xyz[1] * self[e425]) - (right_anti_dual_g9_xyz[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g9_w) * self.group1().xyz()).with_w(0.0)
                + (right_anti_dual_g9_xyz.zxy() * self.group2().yzx()).with_w(0.0)
                - (right_anti_dual_g9_xyz.yzx() * self.group2().zxy()).with_w(right_anti_dual_g9_xyz[0] * self[e415]),
            // e41, e42, e43
            (Simd32x3::from(right_anti_dual_g10) * self.group1().xyz()) + (right_anti_dual_g9_xyz.yzx() * self.group0().zxy())
                - (right_anti_dual_g9_xyz.zxy() * self.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(right_anti_dual_g10) * self.group2()) + (Simd32x3::from(right_anti_dual_g9_w) * self.group0()) - (right_anti_dual_g9_xyz * Simd32x3::from(self[e321])),
            // e415, e425, e435, e321
            Simd32x4::from(right_anti_dual_g0[1]) * self.group1(),
            // e423, e431, e412
            Simd32x3::from(right_anti_dual_g0[1]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(right_anti_dual_g0[1]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<RoundPoint> for Circle {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd3        4        8        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        8       13        0      N/A
    //  no simd       25       32        0        0
    fn weight_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e5] * -1.0;
        let right_anti_dual_g1 = other[e4] * -1.0;
        Dipole::from_groups(
            // e41, e42, e43
            (Simd32x3::from(right_anti_dual_g1) * self.group1().xyz()) + (right_anti_dual_g0_xyz.yzx() * self.group0().zxy())
                - (right_anti_dual_g0_xyz.zxy() * self.group0().yzx()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0_xyz[1] * self[e425]) - (right_anti_dual_g0_xyz[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g0_w) * self.group0()).with_w(0.0)
                + (Simd32x3::from(right_anti_dual_g1) * self.group2()).with_w(0.0)
                - (self.group1().wwwx() * right_anti_dual_g0_xyz.with_w(right_anti_dual_g0_xyz[0])),
            // e15, e25, e35
            (Simd32x3::from(right_anti_dual_g0_w) * self.group1().xyz()) + (right_anti_dual_g0_xyz.zxy() * self.group2().yzx())
                - (right_anti_dual_g0_xyz.yzx() * self.group2().zxy()),
        )
    }
}
impl WeightContraction<Scalar> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       10        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * self.group2(),
        )
    }
}
impl WeightContraction<VersorEven> for Circle {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       13        0        0
    //    simd3        2        7        0      N/A
    //    simd4        6        2        0      N/A
    // Totals...
    // yes simd       16       22        0      N/A
    //  no simd       38       42        0        0
    fn weight_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        let right_anti_dual_g3_w = other[e5] * -1.0;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(right_anti_dual_g2_w) * self.group1().xyz()) + (right_anti_dual_g3_xyz.yzx() * self.group0().zxy())
                - (right_anti_dual_g3_xyz.zxy() * self.group0().yzx()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g3_xyz[1] * self[e425]) - (right_anti_dual_g3_xyz[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g2_w) * self.group2()).with_w(0.0)
                + (Simd32x3::from(right_anti_dual_g3_w) * self.group0()).with_w(0.0)
                - (self.group1().wwwx() * right_anti_dual_g3_xyz.with_w(right_anti_dual_g3_xyz[0])),
            // e15, e25, e35, scalar
            (self.group1() * Simd32x3::from(right_anti_dual_g3_w).with_w(other[e321]))
                + Simd32x3::from(0.0).with_w(
                    -(right_anti_dual_g0_xyz[1] * self[e315])
                        - (right_anti_dual_g0_xyz[2] * self[e125])
                        - (right_anti_dual_g1_xyz[0] * self[e415])
                        - (right_anti_dual_g1_xyz[1] * self[e425])
                        - (right_anti_dual_g1_xyz[2] * self[e435])
                        - (right_anti_dual_g2_xyz[0] * self[e423])
                        - (right_anti_dual_g2_xyz[1] * self[e431])
                        - (right_anti_dual_g2_xyz[2] * self[e412]),
                )
                + (right_anti_dual_g3_xyz.zxy() * self.group2().yzx()).with_w(0.0)
                - (right_anti_dual_g3_xyz.yzx() * self.group2().zxy()).with_w(right_anti_dual_g0_xyz[0] * self[e235]),
        )
    }
}
impl WeightContraction<VersorOdd> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       10        0        0
    //    simd3        0        7        0      N/A
    //    simd4        5        3        0      N/A
    // Totals...
    // yes simd       10       20        0      N/A
    //  no simd       25       43        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2_xyz = other.group2().xyz() * Simd32x3::from(-1.0);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(other[scalar]) * self.group2()).with_w(
                (self[e423] * other[e23])
                    + (self[e431] * other[e31])
                    + (self[e412] * other[e12])
                    + (self[e415] * other[e41])
                    + (self[e425] * other[e42])
                    + (self[e435] * other[e43]),
            ),
            // e1, e2, e3, e5
            (other.group1().wwwx() * self.group1().xyz().with_w(self[e235]))
                + (right_anti_dual_g2_xyz.yzx() * self.group0().zxy()).with_w(self[e315] * other[e31])
                + (self.group2().zxy() * other.group0().yzx()).with_w(self[e125] * other[e12])
                - (self.group1().wwwx() * other.group1().xyz().with_w(right_anti_dual_g2_xyz[0]))
                - (right_anti_dual_g2_xyz.zxy() * self.group0().yzx()).with_w(right_anti_dual_g2_xyz[1] * self[e425])
                - (self.group2().yzx() * other.group0().zxy()).with_w(right_anti_dual_g2_xyz[2] * self[e435]),
        )
    }
}
impl std::ops::Div<WeightContractionInfix> for CircleRotor {
    type Output = WeightContractionInfixPartial<CircleRotor>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       11        0        0
    //    simd3        1        9        0      N/A
    //    simd4        9        5        0      N/A
    // Totals...
    // yes simd       16       25        0      N/A
    //  no simd       45       58        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((right_anti_dual_g0 * Simd32x3::from(self[e12345])) + (Simd32x3::from(other[scalar]) * self.group0())).with_w(other[scalar] * self[e12345]),
            // e415, e425, e435, e321
            (right_anti_dual_g1 * Simd32x4::from(self[e12345])) + (Simd32x4::from(other[scalar]) * self.group1()),
            // e235, e315, e125, e5
            (other.group2().wwwx() * self.group2().xyz().with_w(self[e415]))
                + Simd32x3::from(0.0).with_w((other[e25] * self[e425]) + (other[e35] * self[e435]) - (right_anti_dual_g1[1] * self[e315]) - (right_anti_dual_g1[2] * self[e125]))
                - (self.group2().wwwx() * other.group2().xyz().with_w(right_anti_dual_g1[0])),
            // e1, e2, e3, e4
            Simd32x3::from(0.0)
                .with_w(-(right_anti_dual_g0[2] * self[e435]) - (right_anti_dual_g1[0] * self[e423]) - (right_anti_dual_g1[1] * self[e431]) - (right_anti_dual_g1[2] * self[e412]))
                + (Simd32x3::from(right_anti_dual_g1[3]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * right_anti_dual_g1.xyz()).with_w(0.0)
                + (right_anti_dual_g0.zxy() * self.group2().yzx()).with_w(0.0)
                + (self.group0().yzx() * other.group2().zxy()).with_w(0.0)
                - (right_anti_dual_g0.yzx() * self.group2().zxy()).with_w(right_anti_dual_g0[0] * self[e415])
                - (self.group0().zxy() * other.group2().yzx()).with_w(right_anti_dual_g0[1] * self[e425]),
        )
    }
}
impl WeightContraction<AntiDipoleInversion> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       18        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        7        0      N/A
    //    simd4        8        4        0      N/A
    // Totals...
    // yes simd       22       31        0      N/A
    //  no simd       54       59        0        0
    fn weight_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3 = other.group3().xyz().with_w(other[e5] * -1.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(
                -(right_anti_dual_g1_xyz[0] * self[e415])
                    - (right_anti_dual_g1_xyz[1] * self[e425])
                    - (right_anti_dual_g1_xyz[2] * self[e435])
                    - (right_anti_dual_g2_xyz[0] * self[e423])
                    - (right_anti_dual_g2_xyz[1] * self[e431])
                    - (right_anti_dual_g2_xyz[2] * self[e412])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
            ) + (Simd32x3::from(right_anti_dual_g2_w) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e12345]) * other.group0()).with_w(0.0)
                + (self.group0().zxy() * right_anti_dual_g3.yzx()).with_w(0.0)
                - (Simd32x4::from([right_anti_dual_g3[2], right_anti_dual_g3[0], right_anti_dual_g3[1], right_anti_dual_g1_w * self[e321]]) * self.group0().yzx().with_w(1.0)),
            // e23, e31, e12, e45
            (self.group2() * Simd32x3::from(right_anti_dual_g2_w).with_w(right_anti_dual_g1_w))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g3[1] * self[e425]) - (right_anti_dual_g3[2] * self[e435]))
                + (right_anti_dual_g1_xyz * Simd32x3::from(self[e12345])).with_w(0.0)
                + (Simd32x3::from(right_anti_dual_g3[3]) * self.group0()).with_w(0.0)
                - (right_anti_dual_g3.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            ((right_anti_dual_g2_xyz * Simd32x3::from(self[e12345]))
                + (Simd32x3::from(right_anti_dual_g3[3]) * self.group1().xyz())
                + Simd32x2::from(0.0).with_z((right_anti_dual_g3[1] * self[e235]) - (right_anti_dual_g3[0] * self[e315]))
                + (right_anti_dual_g3.zx() * self.group2().yz()).with_z(0.0)
                - (right_anti_dual_g3.yz() * self.group2().zx()).with_z(0.0))
            .with_w(right_anti_dual_g2_w * self[e12345]),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g3 * Simd32x4::from(self[e12345]),
        )
    }
}
impl WeightContraction<AntiDualNum> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        3        0      N/A
    // no simd        0       12        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other[scalar]) * self.group0().with_w(self[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125, e5
            self.group2() * other.group0().yyyx(),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}
impl WeightContraction<AntiFlatPoint> for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3        9        0        0
    fn weight_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e321] * -1.0;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(right_anti_dual_g0_w * self[e12345]),
            // e15, e25, e35, scalar
            (right_anti_dual_g0_xyz * Simd32x3::from(self[e12345])).with_w(
                -(right_anti_dual_g0_w * self[e321])
                    - (right_anti_dual_g0_xyz[0] * self[e423])
                    - (right_anti_dual_g0_xyz[1] * self[e431])
                    - (right_anti_dual_g0_xyz[2] * self[e412]),
            ),
        )
    }
}
impl WeightContraction<AntiFlector> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        9        0        0
    //    simd3        0        6        0      N/A
    //    simd4        7        3        0      N/A
    // Totals...
    // yes simd       10       18        0      N/A
    //  no simd       31       39        0        0
    fn weight_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e321] * -1.0;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e5] * -1.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0_xyz[0] * self[e423]) - (right_anti_dual_g0_xyz[1] * self[e431]) - (right_anti_dual_g0_xyz[2] * self[e412]))
                + (self.group0().zxy() * right_anti_dual_g1.yzx()).with_w(0.0)
                - (Simd32x4::from([right_anti_dual_g1[2], right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g0_w * self[e321]]) * self.group0().yzx().with_w(1.0)),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g1[1] * self[e425]) - (right_anti_dual_g1[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g1[3]) * self.group0()).with_w(right_anti_dual_g0_w * self[e12345])
                - (right_anti_dual_g1.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            (right_anti_dual_g0_xyz * Simd32x3::from(self[e12345])).with_w(0.0)
                + (Simd32x3::from(right_anti_dual_g1[3]) * self.group1().xyz()).with_w(0.0)
                + (right_anti_dual_g1.zxy() * self.group2().yzx()).with_w(0.0)
                - (right_anti_dual_g1.yzx() * self.group2().zxy()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g1 * Simd32x4::from(self[e12345]),
        )
    }
}
impl WeightContraction<AntiLine> for CircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        0        7        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        9       16        0      N/A
    //  no simd       18       30        0        0
    fn weight_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (right_anti_dual_g0 * Simd32x3::from(self[e12345])).with_w(0.0),
            // e235, e315, e125, e4
            (right_anti_dual_g1 * Simd32x3::from(self[e12345]))
                .with_w(-(right_anti_dual_g0[0] * self[e423]) - (right_anti_dual_g0[1] * self[e431]) - (right_anti_dual_g0[2] * self[e412])),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(
                -(right_anti_dual_g0[1] * self[e315])
                    - (right_anti_dual_g0[2] * self[e125])
                    - (right_anti_dual_g1[0] * self[e415])
                    - (right_anti_dual_g1[1] * self[e425])
                    - (right_anti_dual_g1[2] * self[e435]),
            ) + (right_anti_dual_g0 * Simd32x3::from(self[e321])).with_w(0.0)
                + (right_anti_dual_g1.yzx() * self.group0().zxy()).with_w(0.0)
                - (right_anti_dual_g1.zxy() * self.group0().yzx()).with_w(right_anti_dual_g0[0] * self[e235]),
        )
    }
}
impl WeightContraction<AntiMotor> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       10        0        0
    //    simd3        1        6        0      N/A
    //    simd4        5        2        0      N/A
    // Totals...
    // yes simd       12       18        0      N/A
    //  no simd       29       36        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other[scalar]) * self.group0().with_w(self[e12345]),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[scalar]) * self.group1().xyz()) - (Simd32x3::from(self[e12345]) * other.group0().xyz())).with_w(other[scalar] * self[e321]),
            // e235, e315, e125, e5
            (self.group2() * Simd32x3::from(other[scalar]).with_w(other[e3215]))
                + Simd32x3::from(0.0).with_w(
                    (other[e23] * self[e235])
                        + (other[e31] * self[e315])
                        + (other[e12] * self[e125])
                        + (other[e15] * self[e415])
                        + (other[e25] * self[e425])
                        + (other[e35] * self[e435]),
                )
                - (Simd32x3::from(self[e12345]) * other.group1().xyz()).with_w(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w((other[e31] * self[e431]) + (other[e12] * self[e412])) + (self.group0().yzx() * other.group1().zxy()).with_w(other[e23] * self[e423])
                - (Simd32x3::from(self[e321]) * other.group0().xyz()).with_w(0.0)
                - (self.group0().zxy() * other.group1().yzx()).with_w(0.0),
        )
    }
}
impl WeightContraction<AntiPlane> for CircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        1        6        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        6       11        0      N/A
    //  no simd       20       29        0        0
    fn weight_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e5] * -1.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            (self.group0().zxy() * right_anti_dual_g0.yzx()) - (self.group0().yzx() * right_anti_dual_g0.zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e425]) - (right_anti_dual_g0[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g0[3]) * self.group0()).with_w(0.0)
                - (right_anti_dual_g0.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz()).with_w(0.0) + (right_anti_dual_g0.zxy() * self.group2().yzx()).with_w(0.0)
                - (right_anti_dual_g0.yzx() * self.group2().zxy()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g0 * Simd32x4::from(self[e12345]),
        )
    }
}
impl WeightContraction<AntiScalar> for CircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_contraction(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e12345] * self[e12345] * -1.0)
    }
}
impl WeightContraction<Circle> for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       11        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd        9       21        0        0
    fn weight_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            right_anti_dual_g1 * Simd32x4::from(self[e12345]),
            // e15, e25, e35, scalar
            (Simd32x3::from(self[e12345]) * other.group2()).with_w(
                -(right_anti_dual_g1[0] * self[e415])
                    - (right_anti_dual_g1[1] * self[e425])
                    - (right_anti_dual_g1[2] * self[e435])
                    - (right_anti_dual_g1[3] * self[e321])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125])
                    - (other[e235] * self[e423])
                    - (other[e315] * self[e431])
                    - (other[e125] * self[e412]),
            ),
        )
    }
}
impl WeightContraction<CircleRotor> for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       12        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       10       15        0      N/A
    //  no simd       10       22        0        0
    fn weight_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        let right_anti_dual_g2_xyz = other.group2().xyz();
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            right_anti_dual_g1 * Simd32x4::from(self[e12345]),
            // e15, e25, e35, scalar
            (right_anti_dual_g2_xyz * Simd32x3::from(self[e12345])).with_w(
                -(right_anti_dual_g2_xyz[0] * self[e423])
                    - (right_anti_dual_g2_xyz[1] * self[e431])
                    - (right_anti_dual_g2_xyz[2] * self[e412])
                    - (right_anti_dual_g1[0] * self[e415])
                    - (right_anti_dual_g1[1] * self[e425])
                    - (right_anti_dual_g1[2] * self[e435])
                    - (right_anti_dual_g1[3] * self[e321])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125])
                    - (other[e12345] * self[e12345]),
            ),
        )
    }
}
impl WeightContraction<Dipole> for CircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       11        0        0
    //    simd3        0        9        0      N/A
    //    simd4        6        3        0      N/A
    // Totals...
    // yes simd       14       23        0      N/A
    //  no simd       32       50        0        0
    fn weight_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_anti_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            right_anti_dual_g0 * Simd32x3::from(self[e12345]),
            // e415, e425, e435, e321
            right_anti_dual_g1 * Simd32x4::from(self[e12345]),
            // e235, e315, e125, e4
            (right_anti_dual_g2 * Simd32x3::from(self[e12345])).with_w(
                -(right_anti_dual_g0[0] * self[e415])
                    - (right_anti_dual_g0[1] * self[e425])
                    - (right_anti_dual_g0[2] * self[e435])
                    - (right_anti_dual_g1[0] * self[e423])
                    - (right_anti_dual_g1[1] * self[e431])
                    - (right_anti_dual_g1[2] * self[e412]),
            ),
            // e1, e2, e3, e5
            Simd32x3::from(0.0)
                .with_w(-(right_anti_dual_g2[1] * self[e425]) - (right_anti_dual_g2[2] * self[e435]) - (right_anti_dual_g1[1] * self[e315]) - (right_anti_dual_g1[2] * self[e125]))
                + (Simd32x3::from(right_anti_dual_g1[3]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * right_anti_dual_g1.xyz()).with_w(0.0)
                + (right_anti_dual_g0.zxy() * self.group2().yzx()).with_w(0.0)
                + (right_anti_dual_g2.yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group2().zxyx() * right_anti_dual_g0.yzx().with_w(right_anti_dual_g1[0]))
                - (right_anti_dual_g2.zxy() * self.group0().yzx()).with_w(right_anti_dual_g2[0] * self[e415]),
        )
    }
}
impl WeightContraction<DipoleInversion> for CircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        0        4        0      N/A
    //    simd4        6        8        0      N/A
    // Totals...
    // yes simd       12       21        0      N/A
    //  no simd       30       53        0        0
    fn weight_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            right_anti_dual_g0 * Simd32x3::from(self[e12345]),
            // e415, e425, e435, e321
            right_anti_dual_g1 * Simd32x4::from(self[e12345]),
            // e235, e315, e125, e4
            (Simd32x3::from(self[e12345] * -1.0) * other.group2().xyz()).with_w(
                (self[e12345] * other[e1234])
                    - (right_anti_dual_g0[0] * self[e415])
                    - (right_anti_dual_g0[1] * self[e425])
                    - (right_anti_dual_g0[2] * self[e435])
                    - (right_anti_dual_g1[0] * self[e423])
                    - (right_anti_dual_g1[1] * self[e431])
                    - (right_anti_dual_g1[2] * self[e412]),
            ),
            // e1, e2, e3, e5
            (Simd32x4::from([right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g1[2], other[e15]]) * self.group1().wwwx())
                + (self.group1().xyzy() * Simd32x3::from(right_anti_dual_g1[3]).with_w(other[e25]))
                + (self.group2().yzxw() * right_anti_dual_g0.zxy().with_w(other[e3215]))
                + (other.group2().zxyz() * self.group0().yzx().with_w(self[e435]))
                - (self.group2().zxyx() * right_anti_dual_g0.yzx().with_w(right_anti_dual_g1[0]))
                - (self.group2().wwwy() * other.group3().xyz().with_w(right_anti_dual_g1[1]))
                - (self.group0().zxy() * other.group2().yzx()).with_w(right_anti_dual_g1[2] * self[e125]),
        )
    }
}
impl WeightContraction<DualNum> for CircleRotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       11        0        0
    fn weight_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(other[e5] * -1.0) * self.group0()).with_w(self[e12345] * other[e12345] * -1.0),
            // e15, e25, e35, e3215
            Simd32x4::from(other[e5] * -1.0) * self.group1().xyz().with_w(self[e12345]),
        )
    }
}
impl WeightContraction<FlatPoint> for CircleRotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd       13       20        0        0
    fn weight_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            right_anti_dual_g0 * Simd32x4::from(self[e12345]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e425]) - (right_anti_dual_g0[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz()).with_w(0.0)
                + (self.group0().zxy() * right_anti_dual_g0.yzx()).with_w(0.0)
                - (Simd32x4::from([self[e431], self[e412], self[e423], self[e415]]) * right_anti_dual_g0.zxyx()),
        )
    }
}
impl WeightContraction<Flector> for CircleRotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        5        0      N/A
    // Totals...
    // yes simd        5        9        0      N/A
    //  no simd       17       28        0        0
    fn weight_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            right_anti_dual_g0 * Simd32x4::from(self[e12345]),
            // e1, e2, e3, e5
            (Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e425]) - (right_anti_dual_g0[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz()).with_w(0.0)
                + (self.group0().zxy() * right_anti_dual_g0.yzx()).with_w(0.0)
                - (Simd32x4::from([self[e431], self[e412], self[e423], self[e415]]) * right_anti_dual_g0.zxyx()),
        )
    }
}
impl WeightContraction<Line> for CircleRotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       12        0        0
    fn weight_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e12345]) * other.group0()).with_w(
                -(self[e423] * other[e235])
                    - (self[e431] * other[e315])
                    - (self[e412] * other[e125])
                    - (self[e415] * other[e415])
                    - (self[e425] * other[e425])
                    - (self[e435] * other[e435]),
            ),
            // e15, e25, e35, e3215
            (Simd32x3::from(self[e12345]) * other.group1()).with_w(0.0),
        )
    }
}
impl WeightContraction<Motor> for CircleRotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        9        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        8       13        0      N/A
    //  no simd       16       22        0        0
    fn weight_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e12345] * -1.0);
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e5] * -1.0;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (right_anti_dual_g0 * Simd32x4::from(self[e12345]))
                + Simd32x3::from(0.0).with_w(
                    -(right_anti_dual_g1_xyz[0] * self[e423])
                        - (right_anti_dual_g1_xyz[1] * self[e431])
                        - (right_anti_dual_g1_xyz[2] * self[e412])
                        - (right_anti_dual_g0[0] * self[e415])
                        - (right_anti_dual_g0[1] * self[e425])
                        - (right_anti_dual_g0[2] * self[e435]),
                )
                + (Simd32x3::from(right_anti_dual_g1_w) * self.group0()).with_w(0.0),
            // e15, e25, e35, e3215
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e12345])) + (Simd32x3::from(right_anti_dual_g1_w) * self.group1().xyz())).with_w(right_anti_dual_g1_w * self[e12345]),
        )
    }
}
impl WeightContraction<MultiVector> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       20       31        0        0
    //    simd2        0        1        0      N/A
    //    simd3        8       23        0      N/A
    //    simd4       12        6        0      N/A
    // Totals...
    // yes simd       40       61        0      N/A
    //  no simd       92      126        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
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
                (right_anti_dual_g0[0] * self[e12345])
                    - (right_anti_dual_g5[0] * self[e415])
                    - (right_anti_dual_g5[1] * self[e425])
                    - (right_anti_dual_g5[2] * self[e435])
                    - (right_anti_dual_g3[0] * self[e423])
                    - (right_anti_dual_g3[1] * self[e431])
                    - (right_anti_dual_g3[2] * self[e412])
                    - (right_anti_dual_g3[3] * self[e321])
                    - (self[e235] * other[e423])
                    - (self[e315] * other[e431])
                    - (self[e125] * other[e412]),
                right_anti_dual_g0[1] * self[e12345],
            ]),
            // e1, e2, e3, e4
            (self.group2().yzxw() * right_anti_dual_g7.zxy().with_w(other[e1234]))
                + Simd32x3::from(0.0).with_w(
                    -(right_anti_dual_g7[2] * self[e435]) - (right_anti_dual_g6[0] * self[e423]) - (right_anti_dual_g6[1] * self[e431]) - (right_anti_dual_g6[2] * self[e412]),
                )
                + (Simd32x3::from(right_anti_dual_g6[3]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * right_anti_dual_g6.xyz()).with_w(0.0)
                + (right_anti_dual_g8.yzx() * self.group0().zxy()).with_w(0.0)
                - (Simd32x3::from(self[e12345]) * other.group9().xyz()).with_w(0.0)
                - (right_anti_dual_g7.yzx() * self.group2().zxy()).with_w(right_anti_dual_g7[0] * self[e415])
                - (right_anti_dual_g8.zxy() * self.group0().yzx()).with_w(right_anti_dual_g7[1] * self[e425]),
            // e5
            (self[e12345] * other[e3215])
                - (right_anti_dual_g8[0] * self[e415])
                - (right_anti_dual_g8[1] * self[e425])
                - (right_anti_dual_g8[2] * self[e435])
                - (right_anti_dual_g6[0] * self[e235])
                - (right_anti_dual_g6[1] * self[e315])
                - (right_anti_dual_g6[2] * self[e125]),
            // e15, e25, e35, e45
            (right_anti_dual_g3 * Simd32x4::from(self[e12345]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g9[1] * self[e425]) - (right_anti_dual_g9[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g9[3]) * self.group1().xyz()).with_w(0.0)
                + (right_anti_dual_g9.zxy() * self.group2().yzx()).with_w(0.0)
                - (right_anti_dual_g9.yzxx() * self.group2().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(right_anti_dual_g10) * self.group1().xyz()) + (Simd32x3::from(self[e12345]) * other.group7()) + (self.group0().zxy() * right_anti_dual_g9.yzx())
                - (self.group0().yzx() * right_anti_dual_g9.zxy()),
            // e23, e31, e12
            (right_anti_dual_g5 * Simd32x3::from(self[e12345]))
                + (Simd32x3::from(right_anti_dual_g10) * self.group2().xyz())
                + (Simd32x3::from(right_anti_dual_g9[3]) * self.group0())
                - (Simd32x3::from(self[e321]) * right_anti_dual_g9.xyz()),
            // e415, e425, e435, e321
            (right_anti_dual_g6 * Simd32x4::from(self[e12345])) + (Simd32x4::from(right_anti_dual_g0[1]) * self.group1()),
            // e423, e431, e412
            (right_anti_dual_g7 * Simd32x3::from(self[e12345])) + (Simd32x3::from(right_anti_dual_g0[1]) * self.group0()),
            // e235, e315, e125
            (right_anti_dual_g8 * Simd32x3::from(self[e12345])) + (Simd32x3::from(right_anti_dual_g0[1]) * self.group2().xyz()),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g9 * Simd32x4::from(self[e12345]),
            // e1234
            right_anti_dual_g10 * self[e12345],
        )
    }
}
impl WeightContraction<Plane> for CircleRotor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl WeightContraction<RoundPoint> for CircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        7        0        0
    //    simd2        0        1        0      N/A
    //    simd3        5        7        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        9       17        0      N/A
    //  no simd       28       38        0        0
    fn weight_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e5] * -1.0);
        let right_anti_dual_g1 = other[e4] * -1.0;
        DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(right_anti_dual_g1) * self.group1().xyz()) + (self.group0().zxy() * right_anti_dual_g0.yzx()) - (self.group0().yzx() * right_anti_dual_g0.zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e425]) - (right_anti_dual_g0[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g1) * self.group2().xyz()).with_w(0.0)
                + (Simd32x3::from(right_anti_dual_g0[3]) * self.group0()).with_w(0.0)
                - (right_anti_dual_g0.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz())
                + (right_anti_dual_g0.zxy() * self.group2().yzx())
                + Simd32x2::from(0.0).with_z(right_anti_dual_g0[0] * self[e315] * -1.0)
                - (right_anti_dual_g0.yz() * self.group2().zx()).with_z(0.0))
            .with_w(right_anti_dual_g1 * self[e12345]),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g0 * Simd32x4::from(self[e12345]),
        )
    }
}
impl WeightContraction<Scalar> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       11        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(other[scalar]) * self.group2(),
        )
    }
}
impl WeightContraction<Sphere> for CircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        8        0        0
    fn weight_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]),
            // e5
            self[e12345] * other[e3215],
        )
    }
}
impl WeightContraction<VersorEven> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       19        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        6        0      N/A
    //    simd4        8        5        0      N/A
    // Totals...
    // yes simd       22       32        0      N/A
    //  no simd       54       61        0        0
    fn weight_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e12345] * -1.0);
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3 = other.group3().xyz().with_w(other[e5] * -1.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (right_anti_dual_g0 * Simd32x4::from(self[e12345]))
                + Simd32x3::from(0.0).with_w(
                    -(right_anti_dual_g1_xyz[0] * self[e415])
                        - (right_anti_dual_g1_xyz[1] * self[e425])
                        - (right_anti_dual_g1_xyz[2] * self[e435])
                        - (right_anti_dual_g2_xyz[0] * self[e423])
                        - (right_anti_dual_g2_xyz[1] * self[e431])
                        - (right_anti_dual_g2_xyz[2] * self[e412])
                        - (right_anti_dual_g0[0] * self[e235])
                        - (right_anti_dual_g0[1] * self[e315])
                        - (right_anti_dual_g0[2] * self[e125]),
                )
                + (Simd32x3::from(right_anti_dual_g2_w) * self.group1().xyz()).with_w(0.0)
                + (self.group0().zxy() * right_anti_dual_g3.yzx()).with_w(0.0)
                - (Simd32x4::from([right_anti_dual_g3[2], right_anti_dual_g3[0], right_anti_dual_g3[1], right_anti_dual_g1_w * self[e321]]) * self.group0().yzx().with_w(1.0)),
            // e23, e31, e12, e45
            (self.group2() * Simd32x3::from(right_anti_dual_g2_w).with_w(right_anti_dual_g1_w))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g3[1] * self[e425]) - (right_anti_dual_g3[2] * self[e435]))
                + (right_anti_dual_g1_xyz * Simd32x3::from(self[e12345])).with_w(0.0)
                + (Simd32x3::from(right_anti_dual_g3[3]) * self.group0()).with_w(0.0)
                - (right_anti_dual_g3.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            ((right_anti_dual_g2_xyz * Simd32x3::from(self[e12345]))
                + (Simd32x3::from(right_anti_dual_g3[3]) * self.group1().xyz())
                + Simd32x2::from(0.0).with_z((right_anti_dual_g3[1] * self[e235]) - (right_anti_dual_g3[0] * self[e315]))
                + (right_anti_dual_g3.zx() * self.group2().yz()).with_z(0.0)
                - (right_anti_dual_g3.yz() * self.group2().zx()).with_z(0.0))
            .with_w(right_anti_dual_g2_w * self[e12345]),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g3 * Simd32x4::from(self[e12345]),
        )
    }
}
impl WeightContraction<VersorOdd> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       11        0        0
    //    simd3        1        8        0      N/A
    //    simd4       10        7        0      N/A
    // Totals...
    // yes simd       17       26        0      N/A
    //  no simd       49       63        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_anti_dual_g2_xyz = other.group2().xyz() * Simd32x3::from(-1.0);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x3::from(other[scalar]) * self.group0()) - (Simd32x3::from(self[e12345]) * other.group0().xyz())).with_w(self[e12345] * other[scalar]),
            // e415, e425, e435, e321
            (right_anti_dual_g1 * Simd32x4::from(self[e12345])) + (Simd32x4::from(other[scalar]) * self.group1()),
            // e235, e315, e125, e5
            (self.group2() * Simd32x3::from(other[scalar]).with_w(other[e3215]))
                + Simd32x3::from(0.0).with_w(
                    -(right_anti_dual_g2_xyz[0] * self[e415])
                        - (right_anti_dual_g2_xyz[1] * self[e425])
                        - (right_anti_dual_g2_xyz[2] * self[e435])
                        - (right_anti_dual_g1[0] * self[e235])
                        - (right_anti_dual_g1[1] * self[e315])
                        - (right_anti_dual_g1[2] * self[e125]),
                )
                + (right_anti_dual_g2_xyz * Simd32x3::from(self[e12345])).with_w(0.0),
            // e1, e2, e3, e4
            (Simd32x4::from([right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g1[2], other[e41]]) * self.group1().wwwx())
                + (self.group1().xyzy() * Simd32x3::from(right_anti_dual_g1[3]).with_w(other[e42]))
                + (other.group0().yzxz() * self.group2().zxy().with_w(self[e435]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g1[1] * self[e431]) - (right_anti_dual_g1[2] * self[e412]))
                + (right_anti_dual_g2_xyz.yzx() * self.group0().zxy()).with_w(self[e12345] * other[e1234])
                - (Simd32x3::from(self[e12345]) * other.group3().xyz()).with_w(0.0)
                - (right_anti_dual_g2_xyz.zxy() * self.group0().yzx()).with_w(right_anti_dual_g1[0] * self[e423])
                - (self.group2().yzx() * other.group0().zxy()).with_w(0.0),
        )
    }
}
impl std::ops::Div<WeightContractionInfix> for Dipole {
    type Output = WeightContractionInfixPartial<Dipole>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for Dipole {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       10        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        9       13        0      N/A
    //  no simd        9       20        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(other[scalar]) * self.group2()).with_w(
                (other[e41] * self[e15])
                    + (other[e42] * self[e25])
                    + (other[e43] * self[e35])
                    + (other[e23] * self[e23])
                    + (other[e31] * self[e31])
                    + (other[e12] * self[e12])
                    + (other[e15] * self[e41])
                    + (other[e25] * self[e42])
                    + (other[e35] * self[e43])
                    - (other[e45] * self[e45]),
            ),
        )
    }
}
impl WeightContraction<AntiDipoleInversion> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        0        3        0      N/A
    //    simd4        4        1        0      N/A
    // Totals...
    // yes simd        8       12        0      N/A
    //  no simd       20       21        0        0
    fn weight_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        let right_anti_dual_g3_w = other[e5] * -1.0;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(other[e4]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g3_xyz[1] * self[e42]) - (right_anti_dual_g3_xyz[2] * self[e43]))
                + (Simd32x3::from(right_anti_dual_g3_w) * self.group0()).with_w(0.0)
                + (right_anti_dual_g3_xyz.zxy() * self.group1().yzx()).with_w(0.0)
                - (right_anti_dual_g3_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g3_xyz[0] * self[e41]),
            // e5
            (right_anti_dual_g3_w * self[e45]) + (right_anti_dual_g3_xyz[0] * self[e15]) + (right_anti_dual_g3_xyz[1] * self[e25]) + (right_anti_dual_g3_xyz[2] * self[e35]),
        )
    }
}
impl WeightContraction<AntiDualNum> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       10        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35
            Simd32x3::from(other[scalar]) * self.group2(),
        )
    }
}
impl WeightContraction<AntiFlector> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        0        3        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        7       11        0      N/A
    //  no simd       16       17        0        0
    fn weight_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e5] * -1.0;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g1_xyz[1] * self[e42]) - (right_anti_dual_g1_xyz[2] * self[e43]))
                + (Simd32x3::from(right_anti_dual_g1_w) * self.group0()).with_w(0.0)
                + (right_anti_dual_g1_xyz.zxy() * self.group1().yzx()).with_w(0.0)
                - (right_anti_dual_g1_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g1_xyz[0] * self[e41]),
            // e5
            (right_anti_dual_g1_w * self[e45]) + (right_anti_dual_g1_xyz[0] * self[e15]) + (right_anti_dual_g1_xyz[1] * self[e25]) + (right_anti_dual_g1_xyz[2] * self[e35]),
        )
    }
}
impl WeightContraction<AntiLine> for Dipole {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
    fn weight_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43]),
        )
    }
}
impl WeightContraction<AntiMotor> for Dipole {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        5        9        0      N/A
    //  no simd        5       16        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(other[scalar]) * self.group2()).with_w(
                (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43]),
            ),
        )
    }
}
impl WeightContraction<AntiPlane> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        0        3        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        7       11        0      N/A
    //  no simd       16       17        0        0
    fn weight_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e5] * -1.0;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0_xyz[1] * self[e42]) - (right_anti_dual_g0_xyz[2] * self[e43]))
                + (Simd32x3::from(right_anti_dual_g0_w) * self.group0()).with_w(0.0)
                + (right_anti_dual_g0_xyz.zxy() * self.group1().yzx()).with_w(0.0)
                - (right_anti_dual_g0_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g0_xyz[0] * self[e41]),
            // e5
            (right_anti_dual_g0_w * self[e45]) + (right_anti_dual_g0_xyz[0] * self[e15]) + (right_anti_dual_g0_xyz[1] * self[e25]) + (right_anti_dual_g0_xyz[2] * self[e35]),
        )
    }
}
impl WeightContraction<Dipole> for Dipole {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
    fn weight_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (other[e41] * self[e15])
                + (other[e42] * self[e25])
                + (other[e43] * self[e35])
                + (other[e23] * self[e23])
                + (other[e31] * self[e31])
                + (other[e12] * self[e12])
                + (other[e15] * self[e41])
                + (other[e25] * self[e42])
                + (other[e35] * self[e43])
                - (other[e45] * self[e45]),
        )
    }
}
impl WeightContraction<DipoleInversion> for Dipole {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
    fn weight_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e41] * other[e15])
                + (self[e42] * other[e25])
                + (self[e43] * other[e35])
                + (self[e23] * other[e23])
                + (self[e31] * other[e31])
                + (self[e12] * other[e12])
                + (self[e15] * other[e41])
                + (self[e25] * other[e42])
                + (self[e35] * other[e43])
                - (self[e45] * other[e45]),
        )
    }
}
impl WeightContraction<DualNum> for Dipole {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn weight_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e5] * -1.0) * self.group0().with_w(self[e45]))
    }
}
impl WeightContraction<FlatPoint> for Dipole {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]) - (self[e45] * other[e45]),
        )
    }
}
impl WeightContraction<Flector> for Dipole {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]) - (self[e45] * other[e45]),
        )
    }
}
impl WeightContraction<Motor> for Dipole {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn weight_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e5] * -1.0) * self.group0().with_w(self[e45]))
    }
}
impl WeightContraction<MultiVector> for Dipole {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       18        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        6        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd       17       27        0      N/A
    //  no simd       29       46        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g9_xyz = other.group1().xyz();
        let right_anti_dual_g9_w = other[e5] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]) + (self[e15] * other[e41]) + (self[e25] * other[e42]) + (self[e35] * other[e43])
                    - (right_anti_dual_g8[0] * self[e41])
                    - (right_anti_dual_g8[1] * self[e42])
                    - (right_anti_dual_g8[2] * self[e43])
                    - (self[e45] * other[e45]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(other[e4]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g9_xyz[1] * self[e42]) - (right_anti_dual_g9_xyz[2] * self[e43]))
                + (Simd32x3::from(right_anti_dual_g9_w) * self.group0()).with_w(0.0)
                + (right_anti_dual_g9_xyz.zxy() * self.group1().yzx()).with_w(0.0)
                - (right_anti_dual_g9_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g9_xyz[0] * self[e41]),
            // e5
            (right_anti_dual_g9_w * self[e45]) + (right_anti_dual_g9_xyz[0] * self[e15]) + (right_anti_dual_g9_xyz[1] * self[e25]) + (right_anti_dual_g9_xyz[2] * self[e35]),
            // e15, e25, e35, e45
            Simd32x4::from(right_anti_dual_g0[1]) * self.group2().with_w(self[e45]),
            // e41, e42, e43
            Simd32x3::from(right_anti_dual_g0[1]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(right_anti_dual_g0[1]) * self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<RoundPoint> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        0        3        0      N/A
    //    simd4        4        1        0      N/A
    // Totals...
    // yes simd        8       12        0      N/A
    //  no simd       20       21        0        0
    fn weight_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e5] * -1.0;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(other[e4]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g0_xyz[1] * self[e42]) - (right_anti_dual_g0_xyz[2] * self[e43]))
                + (Simd32x3::from(right_anti_dual_g0_w) * self.group0()).with_w(0.0)
                + (right_anti_dual_g0_xyz.zxy() * self.group1().yzx()).with_w(0.0)
                - (right_anti_dual_g0_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g0_xyz[0] * self[e41]),
            // e5
            (right_anti_dual_g0_w * self[e45]) + (right_anti_dual_g0_xyz[0] * self[e15]) + (right_anti_dual_g0_xyz[1] * self[e25]) + (right_anti_dual_g0_xyz[2] * self[e35]),
        )
    }
}
impl WeightContraction<Scalar> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       10        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35
            Simd32x3::from(other[scalar]) * self.group2(),
        )
    }
}
impl WeightContraction<VersorEven> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        0        3        0      N/A
    //    simd4        4        1        0      N/A
    // Totals...
    // yes simd        8       12        0      N/A
    //  no simd       20       21        0        0
    fn weight_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        let right_anti_dual_g3_w = other[e5] * -1.0;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(other[e4]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g3_xyz[1] * self[e42]) - (right_anti_dual_g3_xyz[2] * self[e43]))
                + (Simd32x3::from(right_anti_dual_g3_w) * self.group0()).with_w(0.0)
                + (right_anti_dual_g3_xyz.zxy() * self.group1().yzx()).with_w(0.0)
                - (right_anti_dual_g3_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g3_xyz[0] * self[e41]),
            // e5
            (right_anti_dual_g3_w * self[e45]) + (right_anti_dual_g3_xyz[0] * self[e15]) + (right_anti_dual_g3_xyz[1] * self[e25]) + (right_anti_dual_g3_xyz[2] * self[e35]),
        )
    }
}
impl WeightContraction<VersorOdd> for Dipole {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       10        0        0
    //    simd3        0        3        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd        9       23        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2_xyz = other.group2().xyz() * Simd32x3::from(-1.0);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(other[scalar]) * self.group2()).with_w(
                (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]) + (self[e15] * other[e41]) + (self[e25] * other[e42]) + (self[e35] * other[e43])
                    - (right_anti_dual_g2_xyz[0] * self[e41])
                    - (right_anti_dual_g2_xyz[1] * self[e42])
                    - (right_anti_dual_g2_xyz[2] * self[e43])
                    - (self[e45] * other[e45]),
            ),
        )
    }
}
impl std::ops::Div<WeightContractionInfix> for DipoleInversion {
    type Output = WeightContractionInfixPartial<DipoleInversion>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       15        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4        9        0      N/A
    //    simd4        8        4        0      N/A
    // Totals...
    // yes simd       19       29        0      N/A
    //  no simd       51       60        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(
                (other[e43] * self[e35]) + (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12])
                    - (right_anti_dual_g2[1] * self[e42])
                    - (right_anti_dual_g2[2] * self[e43]),
            ) + (Simd32x3::from(right_anti_dual_g2[3]) * self.group0()).with_w(other[e41] * self[e15])
                + (other.group0().yzx() * self.group3().zxy()).with_w(other[e42] * self[e25])
                - (other.group1() * Simd32x3::from(self[e1234]).with_w(self[e45]))
                - (other.group0().zxy() * self.group3().yzx()).with_w(right_anti_dual_g2[0] * self[e41]),
            // e23, e31, e12, e45
            (right_anti_dual_g2 * Simd32x3::from(self[e1234]).with_w(self[e45]))
                + Simd32x3::from(0.0).with_w((other[e23] * self[e4235]) + (other[e31] * self[e4315]) + (other[e12] * self[e4125]))
                + (Simd32x3::from(right_anti_dual_g2[3]) * self.group1().xyz()).with_w(0.0)
                - (Simd32x3::from(other[e45]) * self.group3().xyz()).with_w(0.0)
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_anti_dual_g2[3]) * self.group2().xyz())
                + (right_anti_dual_g2.yzx() * self.group3().zxy())
                + Simd32x2::from(0.0).with_z(right_anti_dual_g2[1] * self[e4235] * -1.0)
                - (Simd32x3::from(self[e3215]) * other.group1().xyz())
                - (right_anti_dual_g2.zx() * self.group3().yz()).with_z(0.0))
            .with_w(right_anti_dual_g2[3] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_anti_dual_g2[3]) * self.group3(),
        )
    }
}
impl WeightContraction<AntiDipoleInversion> for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       16        0        0
    //    simd3        1        8        0      N/A
    //    simd4       11        6        0      N/A
    // Totals...
    // yes simd       17       30        0      N/A
    //  no simd       52       64        0        0
    fn weight_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        let right_anti_dual_g3_w = other[e5] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (right_anti_dual_g3_xyz * Simd32x3::from(self[e1234])) - (Simd32x3::from(right_anti_dual_g2_w) * self.group3().xyz()),
            // e415, e425, e435, e321
            (right_anti_dual_g3_xyz.yzx() * self.group3().zxy()).with_w(right_anti_dual_g3_w * self[e1234])
                - (self.group3().yzxw() * right_anti_dual_g3_xyz.zxy().with_w(right_anti_dual_g2_w)),
            // e235, e315, e125, e4
            (self.group3().xyzx() * Simd32x3::from(right_anti_dual_g3_w).with_w(other[e423]))
                + Simd32x3::from(0.0).with_w(
                    (right_anti_dual_g1_w * self[e1234]) + (other[e431] * self[e4315]) + (other[e412] * self[e4125])
                        - (right_anti_dual_g3_xyz[0] * self[e41])
                        - (right_anti_dual_g3_xyz[1] * self[e42])
                        - (right_anti_dual_g3_xyz[2] * self[e43]),
                )
                - (right_anti_dual_g3_xyz * Simd32x3::from(self[e3215])).with_w(right_anti_dual_g2_w * self[e45]),
            // e1, e2, e3, e5
            (Simd32x4::from(right_anti_dual_g3_w) * self.group0().with_w(self[e45]))
                + (self.group2().wwwx() * right_anti_dual_g2_xyz.with_w(right_anti_dual_g3_xyz[0]))
                + Simd32x3::from(0.0).with_w((right_anti_dual_g2_xyz[2] * self[e4125]) * -1.0)
                + (right_anti_dual_g1_xyz.zxy() * self.group3().yzx()).with_w(right_anti_dual_g3_xyz[1] * self[e25])
                + (right_anti_dual_g3_xyz.zxy() * self.group1().yzx()).with_w(right_anti_dual_g3_xyz[2] * self[e35])
                - (Simd32x4::from(self[e3215]) * other.group0().with_w(right_anti_dual_g1_w))
                - (self.group3().zxyx() * right_anti_dual_g1_xyz.yzx().with_w(right_anti_dual_g2_xyz[0]))
                - (Simd32x3::from(right_anti_dual_g2_w) * self.group2().xyz()).with_w(0.0)
                - (right_anti_dual_g3_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g2_xyz[1] * self[e4315]),
        )
    }
}
impl WeightContraction<AntiDualNum> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0().yyyx() * self.group0().with_w(self[e1234]),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[scalar]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group3(),
        )
    }
}
impl WeightContraction<AntiFlatPoint> for DipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3        9        0        0
    fn weight_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e321] * -1.0);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            right_anti_dual_g0 * Simd32x4::from(self[e1234]),
            // e5
            -(right_anti_dual_g0[0] * self[e4235]) - (right_anti_dual_g0[1] * self[e4315]) - (right_anti_dual_g0[2] * self[e4125]) - (right_anti_dual_g0[3] * self[e3215]),
        )
    }
}
impl WeightContraction<AntiFlector> for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       14        0        0
    //    simd2        0        1        0      N/A
    //    simd3        1        6        0      N/A
    //    simd4        6        2        0      N/A
    // Totals...
    // yes simd       11       23        0      N/A
    //  no simd       31       42        0        0
    fn weight_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e321] * -1.0;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e5] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            right_anti_dual_g1_xyz * Simd32x3::from(self[e1234]),
            // e415, e425, e435, e321
            ((right_anti_dual_g1_xyz.yzx() * self.group3().zxy()) - (right_anti_dual_g1_xyz.zxy() * self.group3().yzx())).with_w(right_anti_dual_g1_w * self[e1234]),
            // e235, e315, e125, e4
            (Simd32x2::from(right_anti_dual_g1_w) * self.group3().xy()).with_zw(right_anti_dual_g1_w * self[e4125], right_anti_dual_g0_w * self[e1234])
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g1_xyz[1] * self[e42]) - (right_anti_dual_g1_xyz[2] * self[e43]))
                - (right_anti_dual_g1_xyz * Simd32x3::from(self[e3215])).with_w(right_anti_dual_g1_xyz[0] * self[e41]),
            // e1, e2, e3, e5
            (Simd32x4::from(right_anti_dual_g1_w) * self.group0().with_w(self[e45]))
                + (self.group2().wwwx() * right_anti_dual_g0_xyz.with_w(right_anti_dual_g1_xyz[0]))
                + Simd32x3::from(0.0).with_w(
                    (right_anti_dual_g1_xyz[2] * self[e35])
                        - (right_anti_dual_g0_xyz[0] * self[e4235])
                        - (right_anti_dual_g0_xyz[1] * self[e4315])
                        - (right_anti_dual_g0_xyz[2] * self[e4125]),
                )
                + (right_anti_dual_g1_xyz.zxy() * self.group1().yzx()).with_w(right_anti_dual_g1_xyz[1] * self[e25])
                - (right_anti_dual_g1_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g0_w * self[e3215]),
        )
    }
}
impl WeightContraction<AntiLine> for DipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        0        7        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        9       16        0      N/A
    //  no simd       18       30        0        0
    fn weight_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            right_anti_dual_g0 * Simd32x3::from(self[e1234]),
            // e23, e31, e12, e45
            (right_anti_dual_g1 * Simd32x3::from(self[e1234]))
                .with_w(-(right_anti_dual_g0[0] * self[e4235]) - (right_anti_dual_g0[1] * self[e4315]) - (right_anti_dual_g0[2] * self[e4125])),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(
                -(right_anti_dual_g0[1] * self[e31])
                    - (right_anti_dual_g0[2] * self[e12])
                    - (right_anti_dual_g1[0] * self[e41])
                    - (right_anti_dual_g1[1] * self[e42])
                    - (right_anti_dual_g1[2] * self[e43]),
            ) + (right_anti_dual_g0 * Simd32x3::from(self[e3215])).with_w(0.0)
                + (right_anti_dual_g1.yzx() * self.group3().zxy()).with_w(0.0)
                - (right_anti_dual_g1.zxy() * self.group3().yzx()).with_w(right_anti_dual_g0[0] * self[e23]),
        )
    }
}
impl WeightContraction<AntiMotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       12        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        4        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       16       21        0      N/A
    //  no simd       36       40        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from([self[e41], self[e42], self[e43], self[e23]]) * other.group0().wwwx())
                + Simd32x3::from(0.0).with_w(
                    (other[e31] * self[e31])
                        + (other[e12] * self[e12])
                        + (other[e15] * self[e41])
                        + (other[e25] * self[e42])
                        + (other[e35] * self[e43])
                        + (other[e3215] * self[e1234]),
                )
                - (Simd32x3::from(self[e1234]) * other.group0().xyz()).with_w(0.0),
            // e23, e31, e12, e45
            (Simd32x4::from(other[scalar]) * self.group1()) + Simd32x3::from(0.0).with_w((other[e23] * self[e4235]) + (other[e31] * self[e4315]) + (other[e12] * self[e4125]))
                - (Simd32x3::from(self[e1234]) * other.group1().xyz()).with_w(0.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[scalar]) * self.group2().xyz())
                + Simd32x2::from(0.0).with_z((other[e25] * self[e4235]) - (other[e15] * self[e4315]))
                + (other.group1().zx() * self.group3().yz()).with_z(0.0)
                - (Simd32x3::from(self[e3215]) * other.group0().xyz())
                - (other.group1().yz() * self.group3().zx()).with_z(0.0))
            .with_w(other[scalar] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group3(),
        )
    }
}
impl WeightContraction<AntiPlane> for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        8        0        0
    //    simd3        1        7        0      N/A
    //    simd4        5        1        0      N/A
    // Totals...
    // yes simd        8       16        0      N/A
    //  no simd       25       33        0        0
    fn weight_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e5] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            right_anti_dual_g0_xyz * Simd32x3::from(self[e1234]),
            // e415, e425, e435, e321
            ((right_anti_dual_g0_xyz.yzx() * self.group3().zxy()) - (right_anti_dual_g0_xyz.zxy() * self.group3().yzx())).with_w(right_anti_dual_g0_w * self[e1234]),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0_xyz[1] * self[e42]) - (right_anti_dual_g0_xyz[2] * self[e43]))
                + (Simd32x3::from(right_anti_dual_g0_w) * self.group3().xyz()).with_w(0.0)
                - (right_anti_dual_g0_xyz * Simd32x3::from(self[e3215])).with_w(right_anti_dual_g0_xyz[0] * self[e41]),
            // e1, e2, e3, e5
            (Simd32x4::from(right_anti_dual_g0_w) * self.group0().with_w(self[e45]))
                + Simd32x3::from(0.0).with_w((right_anti_dual_g0_xyz[1] * self[e25]) + (right_anti_dual_g0_xyz[2] * self[e35]))
                + (right_anti_dual_g0_xyz.zxy() * self.group1().yzx()).with_w(right_anti_dual_g0_xyz[0] * self[e15])
                - (right_anti_dual_g0_xyz.yzx() * self.group1().zxy()).with_w(0.0),
        )
    }
}
impl WeightContraction<Circle> for DipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        7        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       20       21        0        0
    fn weight_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * other.group2().with_w(right_anti_dual_g1_w))
                + (self.group3().yzxx() * right_anti_dual_g1_xyz.zxy().with_w(other[e423]))
                + Simd32x3::from(0.0).with_w((other[e431] * self[e4315]) + (other[e412] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (right_anti_dual_g1_xyz.yzx() * self.group3().zxy()).with_w(0.0),
            // e5
            -(right_anti_dual_g1_w * self[e3215]) - (other[e235] * self[e4235]) - (other[e315] * self[e4315]) - (other[e125] * self[e4125]),
        )
    }
}
impl WeightContraction<CircleRotor> for DipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        7        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       20       21        0        0
    fn weight_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * right_anti_dual_g2_xyz.with_w(right_anti_dual_g1_w))
                + (self.group3().yzxx() * right_anti_dual_g1_xyz.zxy().with_w(other[e423]))
                + Simd32x3::from(0.0).with_w((other[e431] * self[e4315]) + (other[e412] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (right_anti_dual_g1_xyz.yzx() * self.group3().zxy()).with_w(0.0),
            // e5
            -(right_anti_dual_g1_w * self[e3215])
                - (right_anti_dual_g2_xyz[0] * self[e4235])
                - (right_anti_dual_g2_xyz[1] * self[e4315])
                - (right_anti_dual_g2_xyz[2] * self[e4125]),
        )
    }
}
impl WeightContraction<Dipole> for DipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       12        0        0
    //    simd3        2        9        0      N/A
    //    simd4        6        1        0      N/A
    // Totals...
    // yes simd       15       22        0      N/A
    //  no simd       37       43        0        0
    fn weight_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (other.group0().yzx() * self.group3().zxy()) - (Simd32x3::from(self[e1234]) * other.group1().xyz()) - (other.group0().zxy() * self.group3().yzx()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w((other[e31] * self[e4315]) + (other[e12] * self[e4125]))
                + (right_anti_dual_g2 * Simd32x3::from(self[e1234])).with_w(other[e23] * self[e4235])
                - (Simd32x3::from(other[e45]) * self.group3().xyz()).with_w(0.0)
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(
                (other[e42] * self[e25]) + (other[e43] * self[e35]) + (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12])
                    - (right_anti_dual_g2[1] * self[e42])
                    - (right_anti_dual_g2[2] * self[e43]),
            ) + (right_anti_dual_g2.yzx() * self.group3().zxy()).with_w(other[e41] * self[e15])
                - (other.group1() * Simd32x3::from(self[e3215]).with_w(self[e45]))
                - (right_anti_dual_g2.zxy() * self.group3().yzx()).with_w(right_anti_dual_g2[0] * self[e41]),
        )
    }
}
impl WeightContraction<DipoleInversion> for DipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       19       24        0        0
    //    simd3        2        3        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd       24       30        0      N/A
    //  no simd       37       45        0        0
    fn weight_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (other.group0().yzx() * self.group3().zxy()) - (Simd32x3::from(self[e1234]) * other.group1().xyz()) - (other.group0().zxy() * self.group3().yzx()),
            // e23, e31, e12, e45
            Simd32x4::from([
                -(other[e41] * self[e3215]) - (other[e45] * self[e4235]) - (other[e15] * self[e1234]),
                -(other[e42] * self[e3215]) - (other[e45] * self[e4315]) - (other[e25] * self[e1234]),
                -(other[e43] * self[e3215]) - (other[e45] * self[e4125]) - (other[e35] * self[e1234]),
                (other[e23] * self[e4235]) + (other[e31] * self[e4315]) + (other[e12] * self[e4125]),
            ]),
            // e15, e25, e35, scalar
            (other.group2().zxyx() * self.group3().yzx().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w(
                    (other[e41] * self[e15])
                        + (other[e42] * self[e25])
                        + (other[e43] * self[e35])
                        + (other[e23] * self[e23])
                        + (other[e31] * self[e31])
                        + (other[e12] * self[e12])
                        + (other[e25] * self[e42])
                        + (other[e35] * self[e43])
                        + (other[e1234] * self[e3215])
                        + (other[e3215] * self[e1234])
                        - (other[e4315] * self[e4315])
                        - (other[e4125] * self[e4125]),
                )
                - (other.group1() * Simd32x3::from(self[e3215]).with_w(self[e45]))
                - (self.group3().zxyx() * other.group2().yzx().with_w(other[e4235])),
        )
    }
}
impl WeightContraction<DualNum> for DipoleInversion {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn weight_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e5] * -1.0) * self.group3().xyz().with_w(self[e1234]),
            // e1, e2, e3, e5
            Simd32x4::from(other[e5] * -1.0) * self.group0().with_w(self[e45]),
        )
    }
}
impl WeightContraction<FlatPoint> for DipoleInversion {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       13       16        0        0
    fn weight_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w((self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]))
                - (other.group0() * Simd32x3::from(self[e1234]).with_w(self[e45]))
                - (Simd32x3::from(other[e45]) * self.group3().xyz()).with_w(0.0),
            // e15, e25, e35, e3215
            ((self.group3().yzx() * other.group0().zxy()) - (self.group3().zxy() * other.group0().yzx())).with_w(0.0),
        )
    }
}
impl WeightContraction<Flector> for DipoleInversion {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       16       20        0        0
    fn weight_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(
                (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]) + (self[e1234] * other[e3215])
                    - (self[e4315] * other[e4315])
                    - (self[e4125] * other[e4125]),
            ) - (Simd32x4::from([other[e45], other[e45], other[e45], other[e4235]]) * self.group3().xyzx())
                - (other.group0() * Simd32x3::from(self[e1234]).with_w(self[e45])),
            // e15, e25, e35, e3215
            ((self.group3().yzx() * other.group0().zxy()) - (self.group3().zxy() * other.group0().yzx())).with_w(0.0),
        )
    }
}
impl WeightContraction<Line> for DipoleInversion {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn weight_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(self[e4315] * other[e315]) - (self[e4125] * other[e125]))
                + (Simd32x3::from(self[e1234]) * other.group1()).with_w(0.0)
                + (other.group0().zxy() * self.group3().yzx()).with_w(0.0)
                - (Simd32x4::from([other[e425], other[e435], other[e415], other[e235]]) * self.group3().zxyx()),
        )
    }
}
impl WeightContraction<Motor> for DipoleInversion {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd       17       21        0        0
    fn weight_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e5] * -1.0);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_anti_dual_g1[3]) * self.group3().xyz().with_w(self[e1234]),
            // e1, e2, e3, e5
            (right_anti_dual_g1 * Simd32x3::from(self[e1234]).with_w(self[e45]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g1[1] * self[e4315]) - (right_anti_dual_g1[2] * self[e4125]))
                + (Simd32x3::from(right_anti_dual_g1[3]) * self.group0()).with_w(0.0)
                + (right_anti_dual_g0_xyz.zxy() * self.group3().yzx()).with_w(0.0)
                - (self.group3().zxyx() * right_anti_dual_g0_xyz.yzx().with_w(right_anti_dual_g1[0])),
        )
    }
}
impl WeightContraction<MultiVector> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       22       35        0        0
    //    simd2        0        1        0      N/A
    //    simd3        8       24        0      N/A
    //    simd4       12        7        0      N/A
    // Totals...
    // yes simd       42       67        0      N/A
    //  no simd       94      137        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g3 = other.group8().with_w(other[e321] * -1.0);
        let right_anti_dual_g5 = other.group6().xyz();
        let right_anti_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_anti_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_anti_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g9_xyz = other.group1().xyz();
        let right_anti_dual_g9_w = other[e5] * -1.0;
        let right_anti_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_anti_dual_g1_xyz[0] * self[e4235])
                    + (right_anti_dual_g1_xyz[1] * self[e4315])
                    + (right_anti_dual_g1_xyz[2] * self[e4125])
                    + (self[e1234] * other[e3215])
                    + (self[e3215] * other[e1234])
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
                0.0,
            ]),
            // e1, e2, e3, e4
            (right_anti_dual_g3 * Simd32x4::from(self[e1234]))
                + (self.group3().yzxx() * right_anti_dual_g5.zxy().with_w(other[e423]))
                + (Simd32x3::from(right_anti_dual_g9_w) * self.group0()).with_w(self[e4315] * other[e431])
                + (right_anti_dual_g9_xyz.zxy() * self.group1().yzx()).with_w(self[e4125] * other[e412])
                - (Simd32x4::from(right_anti_dual_g10) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                - (Simd32x3::from(self[e3215]) * other.group7()).with_w(right_anti_dual_g9_xyz[2] * self[e43])
                - (right_anti_dual_g5.yzx() * self.group3().zxy()).with_w(right_anti_dual_g9_xyz[0] * self[e41])
                - (right_anti_dual_g9_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g9_xyz[1] * self[e42]),
            // e5
            (right_anti_dual_g9_w * self[e45]) + (right_anti_dual_g9_xyz[0] * self[e15]) + (right_anti_dual_g9_xyz[1] * self[e25]) + (right_anti_dual_g9_xyz[2] * self[e35])
                - (right_anti_dual_g3[0] * self[e4235])
                - (right_anti_dual_g3[1] * self[e4315])
                - (right_anti_dual_g3[2] * self[e4125])
                - (right_anti_dual_g3[3] * self[e3215]),
            // e15, e25, e35, e45
            (Simd32x4::from(right_anti_dual_g0[1]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g6_xyz[1] * self[e4315]) - (right_anti_dual_g6_xyz[2] * self[e4125]))
                + (right_anti_dual_g6_xyz * Simd32x3::from(self[e3215])).with_w(0.0)
                + (right_anti_dual_g8.yzx() * self.group3().zxy()).with_w(0.0)
                - (self.group3().yzxx() * right_anti_dual_g8.zxy().with_w(right_anti_dual_g6_xyz[0])),
            // e41, e42, e43
            (right_anti_dual_g6_xyz * Simd32x3::from(self[e1234])) + (Simd32x3::from(right_anti_dual_g0[1]) * self.group0()) + (right_anti_dual_g7.zxy() * self.group3().yzx())
                - (right_anti_dual_g7.yzx() * self.group3().zxy()),
            // e23, e31, e12
            (right_anti_dual_g7 * Simd32x3::from(self[e3215])) + (right_anti_dual_g8 * Simd32x3::from(self[e1234])) + (Simd32x3::from(right_anti_dual_g0[1]) * self.group1().xyz())
                - (Simd32x3::from(other[e45]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (right_anti_dual_g9_xyz.yzx() * self.group3().zxy()).with_w(right_anti_dual_g9_w * self[e1234])
                - (self.group3().yzxw() * right_anti_dual_g9_xyz.zxy().with_w(right_anti_dual_g10)),
            // e423, e431, e412
            (right_anti_dual_g9_xyz * Simd32x3::from(self[e1234])) - (Simd32x3::from(right_anti_dual_g10) * self.group3().xyz()),
            // e235, e315, e125
            (Simd32x3::from(right_anti_dual_g9_w) * self.group3().xyz()) - (right_anti_dual_g9_xyz * Simd32x3::from(self[e3215])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_anti_dual_g0[1]) * self.group3(),
            // e1234
            right_anti_dual_g0[1] * self[e1234],
        )
    }
}
impl WeightContraction<Plane> for DipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e1234] * other[e3215]) - (self[e4235] * other[e4235]) - (self[e4315] * other[e4315]) - (self[e4125] * other[e4125]),
        )
    }
}
impl WeightContraction<RoundPoint> for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       10        0        0
    //    simd3        1        8        0      N/A
    //    simd4        7        2        0      N/A
    // Totals...
    // yes simd       11       20        0      N/A
    //  no simd       34       42        0        0
    fn weight_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e5] * -1.0;
        let right_anti_dual_g1 = other[e4] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (right_anti_dual_g0_xyz * Simd32x3::from(self[e1234])) - (Simd32x3::from(right_anti_dual_g1) * self.group3().xyz()),
            // e415, e425, e435, e321
            (right_anti_dual_g0_xyz.yzx() * self.group3().zxy()).with_w(right_anti_dual_g0_w * self[e1234])
                - (self.group3().yzxw() * right_anti_dual_g0_xyz.zxy().with_w(right_anti_dual_g1)),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0_xyz[0] * self[e41]) - (right_anti_dual_g0_xyz[1] * self[e42]) - (right_anti_dual_g0_xyz[2] * self[e43]))
                + (Simd32x3::from(right_anti_dual_g0_w) * self.group3().xyz()).with_w(0.0)
                - (right_anti_dual_g0_xyz * Simd32x3::from(self[e3215])).with_w(right_anti_dual_g1 * self[e45]),
            // e1, e2, e3, e5
            (Simd32x4::from(right_anti_dual_g0_w) * self.group0().with_w(self[e45]))
                + Simd32x3::from(0.0).with_w((right_anti_dual_g0_xyz[1] * self[e25]) + (right_anti_dual_g0_xyz[2] * self[e35]))
                + (right_anti_dual_g0_xyz.zxy() * self.group1().yzx()).with_w(right_anti_dual_g0_xyz[0] * self[e15])
                - (Simd32x3::from(right_anti_dual_g1) * self.group2().xyz()).with_w(0.0)
                - (right_anti_dual_g0_xyz.yzx() * self.group1().zxy()).with_w(0.0),
        )
    }
}
impl WeightContraction<Scalar> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       15        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[scalar]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group3(),
        )
    }
}
impl WeightContraction<Sphere> for DipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        5        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd        4        8        0        0
    fn weight_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        Scalar::from_groups(
            // scalar
            (right_anti_dual_g0_xyz[0] * self[e4235])
                + (right_anti_dual_g0_xyz[1] * self[e4315])
                + (right_anti_dual_g0_xyz[2] * self[e4125])
                + (self[e1234] * other[e3215])
                + (self[e3215] * other[e1234]),
        )
    }
}
impl WeightContraction<VersorEven> for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       16        0        0
    //    simd3        1        8        0      N/A
    //    simd4       11        6        0      N/A
    // Totals...
    // yes simd       17       30        0      N/A
    //  no simd       52       64        0        0
    fn weight_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        let right_anti_dual_g3_w = other[e5] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (right_anti_dual_g3_xyz * Simd32x3::from(self[e1234])) - (Simd32x3::from(right_anti_dual_g2_w) * self.group3().xyz()),
            // e415, e425, e435, e321
            (right_anti_dual_g3_xyz.yzx() * self.group3().zxy()).with_w(right_anti_dual_g3_w * self[e1234])
                - (self.group3().yzxw() * right_anti_dual_g3_xyz.zxy().with_w(right_anti_dual_g2_w)),
            // e235, e315, e125, e4
            (self.group3().xyzx() * Simd32x3::from(right_anti_dual_g3_w).with_w(right_anti_dual_g0_xyz[0]))
                + Simd32x3::from(0.0).with_w(
                    (right_anti_dual_g1_w * self[e1234]) + (right_anti_dual_g0_xyz[1] * self[e4315]) + (right_anti_dual_g0_xyz[2] * self[e4125])
                        - (right_anti_dual_g3_xyz[0] * self[e41])
                        - (right_anti_dual_g3_xyz[1] * self[e42])
                        - (right_anti_dual_g3_xyz[2] * self[e43]),
                )
                - (right_anti_dual_g3_xyz * Simd32x3::from(self[e3215])).with_w(right_anti_dual_g2_w * self[e45]),
            // e1, e2, e3, e5
            (Simd32x4::from(right_anti_dual_g3_w) * self.group0().with_w(self[e45]))
                + (self.group2().wwwx() * right_anti_dual_g2_xyz.with_w(right_anti_dual_g3_xyz[0]))
                + Simd32x3::from(0.0).with_w((right_anti_dual_g2_xyz[2] * self[e4125]) * -1.0)
                + (right_anti_dual_g1_xyz.zxy() * self.group3().yzx()).with_w(right_anti_dual_g3_xyz[1] * self[e25])
                + (right_anti_dual_g3_xyz.zxy() * self.group1().yzx()).with_w(right_anti_dual_g3_xyz[2] * self[e35])
                - (Simd32x4::from(self[e3215]) * right_anti_dual_g0_xyz.with_w(right_anti_dual_g1_w))
                - (self.group3().zxyx() * right_anti_dual_g1_xyz.yzx().with_w(right_anti_dual_g2_xyz[0]))
                - (Simd32x3::from(right_anti_dual_g2_w) * self.group2().xyz()).with_w(0.0)
                - (right_anti_dual_g3_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g2_xyz[1] * self[e4315]),
        )
    }
}
impl WeightContraction<VersorOdd> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       16        0        0
    //    simd3        3       10        0      N/A
    //    simd4        8        6        0      N/A
    // Totals...
    // yes simd       22       32        0      N/A
    //  no simd       52       70        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_anti_dual_g2_xyz = other.group2().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from([right_anti_dual_g0[2], right_anti_dual_g0[0], right_anti_dual_g0[1], right_anti_dual_g3_xyz[0]]) * self.group3().yzxx())
                + Simd32x3::from(0.0).with_w(
                    (right_anti_dual_g3_xyz[2] * self[e4125])
                        + (self[e23] * other[e23])
                        + (self[e31] * other[e31])
                        + (self[e12] * other[e12])
                        + (self[e1234] * other[e3215])
                        + (self[e3215] * other[e1234])
                        - (right_anti_dual_g2_xyz[0] * self[e41])
                        - (right_anti_dual_g2_xyz[1] * self[e42])
                        - (right_anti_dual_g2_xyz[2] * self[e43])
                        - (right_anti_dual_g0[1] * self[e25])
                        - (right_anti_dual_g0[2] * self[e35]),
                )
                + (Simd32x3::from(right_anti_dual_g0[3]) * self.group0()).with_w(right_anti_dual_g3_xyz[1] * self[e4315])
                - (other.group1() * Simd32x3::from(self[e1234]).with_w(self[e45]))
                - (right_anti_dual_g0.yzxx() * self.group3().zxy().with_w(self[e15])),
            // e23, e31, e12, e45
            (right_anti_dual_g0 * Simd32x3::from(self[e3215]).with_w(self[e45]))
                + Simd32x3::from(0.0).with_w((self[e4315] * other[e31]) + (self[e4125] * other[e12]))
                + (right_anti_dual_g2_xyz * Simd32x3::from(self[e1234])).with_w(self[e4235] * other[e23])
                + (Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz()).with_w(0.0)
                - (Simd32x3::from(other[e45]) * self.group3().xyz()).with_w(0.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_anti_dual_g0[3]) * self.group2().xyz()) + (right_anti_dual_g2_xyz.yzx() * self.group3().zxy())
                - (Simd32x3::from(self[e3215]) * other.group1().xyz())
                - (right_anti_dual_g2_xyz.zxy() * self.group3().yzx()))
            .with_w(right_anti_dual_g0[3] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_anti_dual_g0[3]) * self.group3(),
        )
    }
}
impl std::ops::Div<WeightContractionInfix> for DualNum {
    type Output = WeightContractionInfixPartial<DualNum>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for DualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        5        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       23        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self[e12345]) * (other.group0() * Simd32x3::from(-1.0)).with_w(right_anti_dual_g2[3]),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            right_anti_dual_g2 * Simd32x3::from(self[e12345]).with_w(self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}
impl WeightContraction<AntiDipoleInversion> for DualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        5        0      N/A
    // Totals...
    // yes simd        0        8        0      N/A
    //  no simd        0       25        0        0
    fn weight_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2 = other.group2().xyz().with_w(other[e4] * -1.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x3::from(self[e12345]) * other.group0()).with_w(right_anti_dual_g2[3] * self[e5]),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            right_anti_dual_g2 * Simd32x4::from(self[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightContraction<AntiDualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        3        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(
            // e5, e12345
            Simd32x2::from([(other[e3215] * self[e12345]) + (other[scalar] * self[e5]), other[scalar] * self[e12345]]),
        )
    }
}
impl WeightContraction<AntiFlatPoint> for DualNum {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl WeightContraction<AntiFlector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn weight_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightContraction<AntiLine> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn weight_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(self[e12345] * -1.0) * other.group0(),
            // e235, e315, e125
            Simd32x3::from(self[e12345] * -1.0) * other.group1(),
        )
    }
}
impl WeightContraction<AntiMotor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        5        0        0
    //    simd2        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        1        8        0      N/A
    //  no simd        1       15        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e415, e425, e435, e12345
            right_anti_dual_g0 * Simd32x4::from(self[e12345]),
            // e235, e315, e125, e5
            (Simd32x2::from(self[e12345] * -1.0) * other.group1().xy())
                .with_zw(other[e35] * self[e12345] * -1.0, (right_anti_dual_g0[3] * self[e5]) + (other[e3215] * self[e12345])),
        )
    }
}
impl WeightContraction<AntiPlane> for DualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightContraction<AntiScalar> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_contraction(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e12345] * self[e12345] * -1.0)
    }
}
impl WeightContraction<Circle> for DualNum {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       14        0        0
    fn weight_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(self[e12345]) * other.group2(),
        )
    }
}
impl WeightContraction<CircleRotor> for DualNum {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       19        0        0
    fn weight_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            Simd32x4::from(self[e12345]) * other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightContraction<Dipole> for DualNum {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       16        0        0
    fn weight_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345] * -1.0) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            Simd32x3::from(self[e12345] * -1.0) * other.group2(),
        )
    }
}
impl WeightContraction<DipoleInversion> for DualNum {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        6        0      N/A
    // Totals...
    // yes simd        0        8        0      N/A
    //  no simd        0       28        0        0
    fn weight_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345] * -1.0) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            Simd32x4::from(self[e12345]) * other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            Simd32x4::from(self[e12345]) * other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl WeightContraction<DualNum> for DualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        3        0        0
    fn weight_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(self[e12345] * -1.0) * other.group0())
    }
}
impl WeightContraction<FlatPoint> for DualNum {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl WeightContraction<Flector> for DualNum {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn weight_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl WeightContraction<Line> for DualNum {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn weight_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(self[e12345]) * other.group0(),
            // e15, e25, e35
            Simd32x3::from(self[e12345]) * other.group1(),
        )
    }
}
impl WeightContraction<Motor> for DualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn weight_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightContraction<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        9        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        6        0      N/A
    //    simd4        0        6        0      N/A
    // Totals...
    // yes simd        2       22        0      N/A
    //  no simd        2       53        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(right_anti_dual_g10 * self[e5]) + (right_anti_dual_g0[0] * self[e12345]), right_anti_dual_g0[1] * self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * (other.group9().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]),
            // e5
            (right_anti_dual_g0[1] * self[e5]) + (self[e12345] * other[e3215]),
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * other.group8().with_w(other[e321]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group7(),
            // e23, e31, e12
            Simd32x3::from(self[e12345]) * other.group6().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * (other.group5() * Simd32x3::from(-1.0)).with_w(other[e45]),
            // e423, e431, e412
            Simd32x3::from(self[e12345] * -1.0) * other.group4(),
            // e235, e315, e125
            Simd32x3::from(self[e12345] * -1.0) * other.group3().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            right_anti_dual_g10 * self[e12345],
        )
    }
}
impl WeightContraction<Plane> for DualNum {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl WeightContraction<RoundPoint> for DualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0        8        0        0
    fn weight_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = other[e4] * -1.0;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(right_anti_dual_g1 * self[e5]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(right_anti_dual_g1 * self[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group0().xyz().with_w(other[e5] * -1.0),
        )
    }
}
impl WeightContraction<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        0        1        0      N/A
    // no simd        0        2        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(other[scalar]) * self.group0())
    }
}
impl WeightContraction<Sphere> for DualNum {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        8        0        0
    fn weight_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]),
            // e5
            self[e12345] * other[e3215],
        )
    }
}
impl WeightContraction<VersorEven> for DualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        5        0      N/A
    // Totals...
    // yes simd        1        9        0      N/A
    //  no simd        1       26        0        0
    fn weight_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2 = other.group2().xyz().with_w(other[e4] * -1.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x3::from(self[e12345]) * other.group0().xyz()).with_w((right_anti_dual_g2[3] * self[e5]) - (self[e12345] * other[e12345])),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            right_anti_dual_g2 * Simd32x4::from(self[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightContraction<VersorOdd> for DualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        5        0      N/A
    // Totals...
    // yes simd        1       10        0      N/A
    //  no simd        1       29        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            right_anti_dual_g0 * Simd32x4::from(self[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            (Simd32x3::from(self[e12345] * -1.0) * other.group2().xyz()).with_w((right_anti_dual_g0[3] * self[e5]) + (self[e12345] * other[e3215])),
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]),
        )
    }
}
impl std::ops::Div<WeightContractionInfix> for FlatPoint {
    type Output = WeightContractionInfixPartial<FlatPoint>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for FlatPoint {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3        8        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(other[scalar] * self[e45]),
            // e15, e25, e35, scalar
            (Simd32x3::from(other[scalar]) * self.group0().xyz()).with_w((other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) - (other[e45] * self[e45])),
        )
    }
}
impl WeightContraction<AntiDipoleInversion> for FlatPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
    fn weight_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e4]) * self.group0(),
            // e5
            (right_anti_dual_g3_xyz[0] * self[e15]) + (right_anti_dual_g3_xyz[1] * self[e25]) + (right_anti_dual_g3_xyz[2] * self[e35]) - (other[e5] * self[e45]),
        )
    }
}
impl WeightContraction<AntiDualNum> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl WeightContraction<AntiFlector> for FlatPoint {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (right_anti_dual_g1_xyz[0] * self[e15]) + (right_anti_dual_g1_xyz[1] * self[e25]) + (right_anti_dual_g1_xyz[2] * self[e35]) - (other[e5] * self[e45]),
            0.0,
        ]))
    }
}
impl WeightContraction<AntiMotor> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl WeightContraction<AntiPlane> for FlatPoint {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (right_anti_dual_g0_xyz[0] * self[e15]) + (right_anti_dual_g0_xyz[1] * self[e25]) + (right_anti_dual_g0_xyz[2] * self[e35]) - (other[e5] * self[e45]),
            0.0,
        ]))
    }
}
impl WeightContraction<Dipole> for FlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) - (other[e45] * self[e45]),
        )
    }
}
impl WeightContraction<DipoleInversion> for FlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) - (other[e45] * self[e45]),
        )
    }
}
impl WeightContraction<DualNum> for FlatPoint {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([other[e5] * self[e45] * -1.0, 0.0]))
    }
}
impl WeightContraction<FlatPoint> for FlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e45] * self[e45] * -1.0)
    }
}
impl WeightContraction<Flector> for FlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e45] * other[e45] * -1.0)
    }
}
impl WeightContraction<Motor> for FlatPoint {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e45] * other[e5] * -1.0, 0.0]))
    }
}
impl WeightContraction<MultiVector> for FlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        6       10        0      N/A
    //  no simd        6       16        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g9_xyz = other.group1().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self[e15] * other[e41]) + (self[e25] * other[e42]) + (self[e35] * other[e43]) - (self[e45] * other[e45]), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(other[e4]) * self.group0(),
            // e5
            (right_anti_dual_g9_xyz[0] * self[e15]) + (right_anti_dual_g9_xyz[1] * self[e25]) + (right_anti_dual_g9_xyz[2] * self[e35]) - (self[e45] * other[e5]),
            // e15, e25, e35, e45
            Simd32x4::from(other[scalar]) * self.group0(),
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
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<RoundPoint> for FlatPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
    fn weight_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e4]) * self.group0(),
            // e5
            (right_anti_dual_g0_xyz[0] * self[e15]) + (right_anti_dual_g0_xyz[1] * self[e25]) + (right_anti_dual_g0_xyz[2] * self[e35]) - (self[e45] * other[e5]),
        )
    }
}
impl WeightContraction<Scalar> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl WeightContraction<VersorEven> for FlatPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
    fn weight_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e4]) * self.group0(),
            // e5
            (right_anti_dual_g3_xyz[0] * self[e15]) + (right_anti_dual_g3_xyz[1] * self[e25]) + (right_anti_dual_g3_xyz[2] * self[e35]) - (self[e45] * other[e5]),
        )
    }
}
impl WeightContraction<VersorOdd> for FlatPoint {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3        8        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(self[e45] * other[scalar]),
            // e15, e25, e35, scalar
            (Simd32x3::from(other[scalar]) * self.group0().xyz()).with_w((self[e15] * other[e41]) + (self[e25] * other[e42]) + (self[e35] * other[e43]) - (self[e45] * other[e45])),
        )
    }
}
impl std::ops::Div<WeightContractionInfix> for Flector {
    type Output = WeightContractionInfixPartial<Flector>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       14        0        0
    //    simd3        0        6        0      N/A
    //    simd4        5        1        0      N/A
    // Totals...
    // yes simd       12       21        0      N/A
    //  no simd       27       36        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w((other[e42] * self[e25]) + (other[e43] * self[e35])) + (other.group0().yzx() * self.group1().zxy()).with_w(other[e41] * self[e15])
                - (other.group0().zxy() * self.group1().yzx()).with_w(other[e45] * self[e45]),
            // e23, e31, e12, e45
            Simd32x4::from([
                -(other[e41] * self[e3215]) - (other[e45] * self[e4235]),
                -(other[e42] * self[e3215]) - (other[e45] * self[e4315]),
                -(other[e43] * self[e3215]) - (other[e45] * self[e4125]),
                (other[e23] * self[e4235]) + (other[e31] * self[e4315]) + (other[e12] * self[e4125]) + (other[scalar] * self[e45]),
            ]),
            // e15, e25, e35, e1234
            (Simd32x3::from(other[scalar]) * self.group0().xyz()).with_w(0.0) + (other.group2().zxy() * self.group1().yzx()).with_w(0.0)
                - (Simd32x3::from(self[e3215]) * other.group1().xyz()).with_w(0.0)
                - (other.group2().yzx() * self.group1().zxy()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl WeightContraction<AntiDipoleInversion> for Flector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       13        0        0
    //    simd3        1        5        0      N/A
    //    simd4        6        4        0      N/A
    // Totals...
    // yes simd       12       22        0      N/A
    //  no simd       32       44        0        0
    fn weight_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        let right_anti_dual_g3_w = other[e5] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_anti_dual_g2_w * -1.0) * self.group1().xyz(),
            // e415, e425, e435, e321
            ((right_anti_dual_g3_xyz.yzx() * self.group1().zxy()) - (right_anti_dual_g3_xyz.zxy() * self.group1().yzx())).with_w(right_anti_dual_g2_w * self[e3215] * -1.0),
            // e235, e315, e125, e4
            (self.group1().xyzx() * Simd32x3::from(right_anti_dual_g3_w).with_w(other[e423]))
                + Simd32x3::from(0.0).with_w((other[e431] * self[e4315]) + (other[e412] * self[e4125]))
                - (right_anti_dual_g3_xyz * Simd32x3::from(self[e3215])).with_w(right_anti_dual_g2_w * self[e45]),
            // e1, e2, e3, e5
            (self.group1().yzxw() * right_anti_dual_g1_xyz.zxy().with_w(other[e321]))
                + Simd32x3::from(0.0).with_w(
                    (right_anti_dual_g3_w * self[e45])
                        + (right_anti_dual_g3_xyz[0] * self[e15])
                        + (right_anti_dual_g3_xyz[1] * self[e25])
                        + (right_anti_dual_g3_xyz[2] * self[e35])
                        - (right_anti_dual_g2_xyz[2] * self[e4125]),
                )
                - (self.group1().zxyx() * right_anti_dual_g1_xyz.yzx().with_w(right_anti_dual_g2_xyz[0]))
                - (self.group1().wwwy() * other.group0().with_w(right_anti_dual_g2_xyz[1]))
                - (Simd32x3::from(right_anti_dual_g2_w) * self.group0().xyz()).with_w(0.0),
        )
    }
}
impl WeightContraction<AntiDualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[scalar]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl WeightContraction<AntiFlatPoint> for Flector {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (other[e321] * self[e3215]) - (right_anti_dual_g0_xyz[0] * self[e4235]) - (right_anti_dual_g0_xyz[1] * self[e4315]) - (right_anti_dual_g0_xyz[2] * self[e4125]),
            0.0,
        ]))
    }
}
impl WeightContraction<AntiFlector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       16       21        0        0
    fn weight_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e5] * -1.0;
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((right_anti_dual_g1_xyz.yzx() * self.group1().zxy()) - (right_anti_dual_g1_xyz.zxy() * self.group1().yzx())).with_w(0.0),
            // e235, e315, e125, e5
            (self.group1() * Simd32x3::from(right_anti_dual_g1_w).with_w(other[e321]))
                + Simd32x3::from(0.0).with_w(
                    (right_anti_dual_g1_w * self[e45])
                        + (right_anti_dual_g1_xyz[0] * self[e15])
                        + (right_anti_dual_g1_xyz[1] * self[e25])
                        + (right_anti_dual_g1_xyz[2] * self[e35])
                        - (right_anti_dual_g0_xyz[1] * self[e4315])
                        - (right_anti_dual_g0_xyz[2] * self[e4125]),
                )
                - (self.group1().wwwx() * right_anti_dual_g1_xyz.with_w(right_anti_dual_g0_xyz[0])),
        )
    }
}
impl WeightContraction<AntiLine> for Flector {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        4        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd       13       18        0        0
    fn weight_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e4315]) - (right_anti_dual_g0[2] * self[e4125]))
                + (right_anti_dual_g0 * Simd32x3::from(self[e3215])).with_w(0.0)
                + (right_anti_dual_g1.yzx() * self.group1().zxy()).with_w(0.0)
                - (self.group1().yzxx() * right_anti_dual_g1.zxy().with_w(right_anti_dual_g0[0])),
        )
    }
}
impl WeightContraction<AntiMotor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        4        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd       17       24        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e15, e25, e35, e45
            (right_anti_dual_g0 * Simd32x3::from(self[e3215]).with_w(self[e45]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e4315]) - (right_anti_dual_g0[2] * self[e4125]))
                + (Simd32x3::from(right_anti_dual_g0[3]) * self.group0().xyz()).with_w(0.0)
                + (other.group1().zxy() * self.group1().yzx()).with_w(0.0)
                - (self.group1().zxyx() * other.group1().yzx().with_w(right_anti_dual_g0[0])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_anti_dual_g0[3]) * self.group1(),
        )
    }
}
impl WeightContraction<AntiPlane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd       13       17        0        0
    fn weight_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((right_anti_dual_g0_xyz.yzx() * self.group1().zxy()) - (right_anti_dual_g0_xyz.zxy() * self.group1().yzx())).with_w(0.0),
            // e235, e315, e125, e5
            (Simd32x4::from(other[e5] * -1.0) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e45]]))
                + Simd32x3::from(0.0).with_w((right_anti_dual_g0_xyz[0] * self[e15]) + (right_anti_dual_g0_xyz[1] * self[e25]) + (right_anti_dual_g0_xyz[2] * self[e35]))
                - (right_anti_dual_g0_xyz * Simd32x3::from(self[e3215])).with_w(0.0),
        )
    }
}
impl WeightContraction<Circle> for Flector {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        7        9        0      N/A
    //  no simd       16       16        0        0
    fn weight_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (self.group1().yzxx() * right_anti_dual_g1_xyz.zxy().with_w(other[e423])) + Simd32x3::from(0.0).with_w((other[e431] * self[e4315]) + (other[e412] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (right_anti_dual_g1_xyz.yzx() * self.group1().zxy()).with_w(0.0),
            // e5
            (other[e321] * self[e3215]) - (other[e235] * self[e4235]) - (other[e315] * self[e4315]) - (other[e125] * self[e4125]),
        )
    }
}
impl WeightContraction<CircleRotor> for Flector {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        7        9        0      N/A
    //  no simd       16       16        0        0
    fn weight_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (self.group1().yzxx() * right_anti_dual_g1_xyz.zxy().with_w(other[e423])) + Simd32x3::from(0.0).with_w((other[e431] * self[e4315]) + (other[e412] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (right_anti_dual_g1_xyz.yzx() * self.group1().zxy()).with_w(0.0),
            // e5
            (other[e321] * self[e3215]) - (right_anti_dual_g2_xyz[0] * self[e4235]) - (right_anti_dual_g2_xyz[1] * self[e4315]) - (right_anti_dual_g2_xyz[2] * self[e4125]),
        )
    }
}
impl WeightContraction<Dipole> for Flector {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       12        0        0
    //    simd3        1        5        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd       10       18        0      N/A
    //  no simd       21       31        0        0
    fn weight_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (other.group0().yzx() * self.group1().zxy()) - (other.group0().zxy() * self.group1().yzx()),
            // e23, e31, e12, e45
            Simd32x4::from([
                -(other[e41] * self[e3215]) - (other[e45] * self[e4235]),
                -(other[e42] * self[e3215]) - (other[e45] * self[e4315]),
                -(other[e43] * self[e3215]) - (other[e45] * self[e4125]),
                (other[e23] * self[e4235]) + (other[e31] * self[e4315]) + (other[e12] * self[e4125]),
            ]),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w((other[e42] * self[e25]) + (other[e43] * self[e35])) + (right_anti_dual_g2.yzx() * self.group1().zxy()).with_w(other[e41] * self[e15])
                - (other.group1() * Simd32x3::from(self[e3215]).with_w(self[e45]))
                - (right_anti_dual_g2.zxy() * self.group1().yzx()).with_w(0.0),
        )
    }
}
impl WeightContraction<DipoleInversion> for Flector {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       14        0        0
    //    simd3        1        2        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd       13       19        0      N/A
    //  no simd       24       32        0        0
    fn weight_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (other.group0().yzx() * self.group1().zxy()) - (other.group0().zxy() * self.group1().yzx()),
            // e23, e31, e12, e45
            Simd32x4::from([
                -(other[e41] * self[e3215]) - (other[e45] * self[e4235]),
                -(other[e42] * self[e3215]) - (other[e45] * self[e4315]),
                -(other[e43] * self[e3215]) - (other[e45] * self[e4125]),
                (other[e23] * self[e4235]) + (other[e31] * self[e4315]) + (other[e12] * self[e4125]),
            ]),
            // e15, e25, e35, scalar
            (other.group2().zxyw() * self.group1().yzxw())
                + Simd32x3::from(0.0)
                    .with_w((other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]))
                - (other.group1() * Simd32x3::from(self[e3215]).with_w(self[e45]))
                - (self.group1().zxyx() * other.group2().yzx().with_w(other[e4235])),
        )
    }
}
impl WeightContraction<DualNum> for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        6        0        0
    fn weight_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (Simd32x3::from(other[e5] * -1.0) * self.group1().xyz()).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(other[e5] * self[e45] * -1.0),
        )
    }
}
impl WeightContraction<FlatPoint> for Flector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        1        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        3       11        0        0
    fn weight_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other[e45] * -1.0) * self.group1().xyz().with_w(self[e45]),
            // e15, e25, e35, e3215
            ((other.group0().zxy() * self.group1().yzx()) - (other.group0().yzx() * self.group1().zxy())).with_w(0.0),
        )
    }
}
impl WeightContraction<Flector> for Flector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        6       14        0        0
    fn weight_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(other[e45] * -1.0) * self.group1().xyz())
                .with_w(-(other[e45] * self[e45]) - (other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125])),
            // e15, e25, e35, e3215
            ((other.group0().zxy() * self.group1().yzx()) - (other.group0().yzx() * self.group1().zxy())).with_w(0.0),
        )
    }
}
impl WeightContraction<Line> for Flector {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        3        4        0      N/A
    //  no simd        9        9        0        0
    fn weight_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(self[e4315] * other[e315]) - (self[e4125] * other[e125])) + (other.group0().zxy() * self.group1().yzx()).with_w(0.0)
                - (Simd32x4::from([other[e425], other[e435], other[e415], other[e235]]) * self.group1().zxyx()),
        )
    }
}
impl WeightContraction<Motor> for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd3        0        2        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        9       14        0        0
    fn weight_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e5] * -1.0;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (Simd32x3::from(right_anti_dual_g1_w) * self.group1().xyz()).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g1_xyz[1] * self[e4315]) - (right_anti_dual_g1_xyz[2] * self[e4125]))
                + (right_anti_dual_g0_xyz.zxy() * self.group1().yzx()).with_w(right_anti_dual_g1_w * self[e45])
                - (self.group1().zxyx() * right_anti_dual_g0_xyz.yzx().with_w(right_anti_dual_g1_xyz[0])),
        )
    }
}
impl WeightContraction<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       16       25        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4       17        0      N/A
    //    simd4        8        5        0      N/A
    // Totals...
    // yes simd       28       48        0      N/A
    //  no simd       60       98        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g5 = other.group6().xyz();
        let right_anti_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_anti_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_anti_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g9_xyz = other.group1().xyz();
        let right_anti_dual_g9_w = other[e5] * -1.0;
        let right_anti_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_anti_dual_g1_xyz[0] * self[e4235]) + (right_anti_dual_g1_xyz[1] * self[e4315]) + (right_anti_dual_g1_xyz[2] * self[e4125]) + (self[e3215] * other[e1234])
                    - (right_anti_dual_g7[0] * self[e15])
                    - (right_anti_dual_g7[1] * self[e25])
                    - (right_anti_dual_g7[2] * self[e35])
                    - (self[e45] * other[e45]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (self.group1().yzxx() * right_anti_dual_g5.zxy().with_w(other[e423])) + Simd32x3::from(0.0).with_w((self[e4315] * other[e431]) + (self[e4125] * other[e412]))
                - (Simd32x4::from(right_anti_dual_g10) * self.group0())
                - (Simd32x3::from(self[e3215]) * other.group7()).with_w(0.0)
                - (right_anti_dual_g5.yzx() * self.group1().zxy()).with_w(0.0),
            // e5
            (right_anti_dual_g9_w * self[e45])
                + (right_anti_dual_g9_xyz[0] * self[e15])
                + (right_anti_dual_g9_xyz[1] * self[e25])
                + (right_anti_dual_g9_xyz[2] * self[e35])
                + (self[e3215] * other[e321])
                - (self[e4235] * other[e235])
                - (self[e4315] * other[e315])
                - (self[e4125] * other[e125]),
            // e15, e25, e35, e45
            (Simd32x4::from(right_anti_dual_g0[1]) * self.group0())
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g6_xyz[1] * self[e4315]) - (right_anti_dual_g6_xyz[2] * self[e4125]))
                + (right_anti_dual_g6_xyz * Simd32x3::from(self[e3215])).with_w(0.0)
                + (right_anti_dual_g8.yzx() * self.group1().zxy()).with_w(0.0)
                - (self.group1().yzxx() * right_anti_dual_g8.zxy().with_w(right_anti_dual_g6_xyz[0])),
            // e41, e42, e43
            (right_anti_dual_g7.zxy() * self.group1().yzx()) - (right_anti_dual_g7.yzx() * self.group1().zxy()),
            // e23, e31, e12
            (right_anti_dual_g7 * Simd32x3::from(self[e3215])) - (Simd32x3::from(other[e45]) * self.group1().xyz()),
            // e415, e425, e435, e321
            ((right_anti_dual_g9_xyz.yzx() * self.group1().zxy()) - (right_anti_dual_g9_xyz.zxy() * self.group1().yzx())).with_w(right_anti_dual_g10 * self[e3215] * -1.0),
            // e423, e431, e412
            Simd32x3::from(right_anti_dual_g10 * -1.0) * self.group1().xyz(),
            // e235, e315, e125
            (Simd32x3::from(right_anti_dual_g9_w) * self.group1().xyz()) - (right_anti_dual_g9_xyz * Simd32x3::from(self[e3215])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_anti_dual_g0[1]) * self.group1(),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<Plane> for Flector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn weight_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -(self[e4235] * other[e4235]) - (self[e4315] * other[e4315]) - (self[e4125] * other[e4125]))
    }
}
impl WeightContraction<RoundPoint> for Flector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       14        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        5       20        0      N/A
    //  no simd        9       31        0        0
    fn weight_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e5] * -1.0;
        let right_anti_dual_g1 = other[e4] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_anti_dual_g1 * -1.0) * self.group1().xyz(),
            // e415, e425, e435, e321
            ((right_anti_dual_g0_xyz.yzx() * self.group1().zxy()) - (right_anti_dual_g0_xyz.zxy() * self.group1().yzx())).with_w(right_anti_dual_g1 * self[e3215] * -1.0),
            // e235, e315, e125, e4
            ((Simd32x3::from(right_anti_dual_g0_w) * self.group1().xyz()) - (right_anti_dual_g0_xyz * Simd32x3::from(self[e3215]))).with_w(right_anti_dual_g1 * self[e45] * -1.0),
            // e1, e2, e3, e5
            (Simd32x2::from(right_anti_dual_g1 * -1.0) * self.group0().xy()).with_zw(
                right_anti_dual_g1 * self[e35] * -1.0,
                (right_anti_dual_g0_w * self[e45]) + (right_anti_dual_g0_xyz[0] * self[e15]) + (right_anti_dual_g0_xyz[1] * self[e25]) + (right_anti_dual_g0_xyz[2] * self[e35]),
            ),
        )
    }
}
impl WeightContraction<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[scalar]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl WeightContraction<Sphere> for Flector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        7        0        0
    fn weight_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        Scalar::from_groups(
            // scalar
            (right_anti_dual_g0_xyz[0] * self[e4235]) + (right_anti_dual_g0_xyz[1] * self[e4315]) + (right_anti_dual_g0_xyz[2] * self[e4125]) + (self[e3215] * other[e1234]),
        )
    }
}
impl WeightContraction<VersorEven> for Flector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       13        0        0
    //    simd3        1        5        0      N/A
    //    simd4        6        4        0      N/A
    // Totals...
    // yes simd       12       22        0      N/A
    //  no simd       32       44        0        0
    fn weight_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        let right_anti_dual_g3_w = other[e5] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_anti_dual_g2_w * -1.0) * self.group1().xyz(),
            // e415, e425, e435, e321
            ((right_anti_dual_g3_xyz.yzx() * self.group1().zxy()) - (right_anti_dual_g3_xyz.zxy() * self.group1().yzx())).with_w(right_anti_dual_g2_w * self[e3215] * -1.0),
            // e235, e315, e125, e4
            (self.group1().xyzx() * Simd32x3::from(right_anti_dual_g3_w).with_w(right_anti_dual_g0_xyz[0]))
                + Simd32x3::from(0.0).with_w((right_anti_dual_g0_xyz[1] * self[e4315]) + (right_anti_dual_g0_xyz[2] * self[e4125]))
                - (right_anti_dual_g3_xyz * Simd32x3::from(self[e3215])).with_w(right_anti_dual_g2_w * self[e45]),
            // e1, e2, e3, e5
            (self.group1().yzxw() * right_anti_dual_g1_xyz.zxy().with_w(other[e321]))
                + Simd32x3::from(0.0).with_w(
                    (right_anti_dual_g3_w * self[e45])
                        + (right_anti_dual_g3_xyz[0] * self[e15])
                        + (right_anti_dual_g3_xyz[1] * self[e25])
                        + (right_anti_dual_g3_xyz[2] * self[e35])
                        - (right_anti_dual_g2_xyz[2] * self[e4125]),
                )
                - (self.group1().zxyy() * right_anti_dual_g1_xyz.yzx().with_w(right_anti_dual_g2_xyz[1]))
                - (self.group1().wwwx() * right_anti_dual_g0_xyz.with_w(right_anti_dual_g2_xyz[0]))
                - (Simd32x3::from(right_anti_dual_g2_w) * self.group0().xyz()).with_w(0.0),
        )
    }
}
impl WeightContraction<VersorOdd> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7        9        0        0
    //    simd3        0        7        0      N/A
    //    simd4        7        5        0      N/A
    // Totals...
    // yes simd       14       21        0      N/A
    //  no simd       35       50        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_anti_dual_g2 = (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(other[e3215]);
        let right_anti_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from([right_anti_dual_g0[2], right_anti_dual_g0[0], right_anti_dual_g0[1], right_anti_dual_g3_xyz[0]]) * self.group1().yzxx())
                + Simd32x3::from(0.0).with_w(
                    (right_anti_dual_g3_xyz[1] * self[e4315]) + (right_anti_dual_g3_xyz[2] * self[e4125]) + (self[e3215] * other[e1234])
                        - (right_anti_dual_g0[1] * self[e25])
                        - (right_anti_dual_g0[2] * self[e35])
                        - (self[e45] * other[e45]),
                )
                - (right_anti_dual_g0.yzxx() * self.group1().zxy().with_w(self[e15])),
            // e23, e31, e12, e45
            (right_anti_dual_g0 * Simd32x3::from(self[e3215]).with_w(self[e45]))
                + Simd32x3::from(0.0).with_w((self[e4235] * other[e23]) + (self[e4315] * other[e31]) + (self[e4125] * other[e12]))
                - (Simd32x3::from(other[e45]) * self.group1().xyz()).with_w(0.0),
            // e15, e25, e35, e1234
            (Simd32x3::from(right_anti_dual_g0[3]) * self.group0().xyz()).with_w(0.0) + (right_anti_dual_g2.yzx() * self.group1().zxy()).with_w(0.0)
                - (Simd32x3::from(self[e3215]) * other.group1().xyz()).with_w(0.0)
                - (right_anti_dual_g2.zxy() * self.group1().yzx()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_anti_dual_g0[3]) * self.group1(),
        )
    }
}
impl std::ops::Div<WeightContractionInfix> for Line {
    type Output = WeightContractionInfixPartial<Line>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for Line {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        8        0        0
    //    simd3        0        4        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        8       13        0      N/A
    //  no simd       17       24        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (Simd32x3::from(other[scalar]) * self.group0()).with_w(0.0),
            // e235, e315, e125, e4
            (Simd32x3::from(other[scalar]) * self.group1()).with_w((other[e41] * self[e415]) + (other[e42] * self[e425]) + (other[e43] * self[e435])),
            // e1, e2, e3, e5
            (Simd32x4::from([self[e415], self[e425], self[e435], self[e235]]) * other.group1().wwwx())
                + Simd32x3::from(0.0).with_w((other[e12] * self[e125]) + (other[e15] * self[e415]) + (other[e25] * self[e425]) + (other[e35] * self[e435]))
                + (other.group0().yzx() * self.group1().zxy()).with_w(other[e31] * self[e315])
                - (other.group0().zxy() * self.group1().yzx()).with_w(0.0),
        )
    }
}
impl WeightContraction<AntiDipoleInversion> for Line {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       11        0        0
    //    simd3        0        5        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        9       16        0      N/A
    //  no simd       18       26        0        0
    fn weight_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_anti_dual_g2_w) * self.group0(),
            // e23, e31, e12, e45
            (Simd32x3::from(right_anti_dual_g2_w) * self.group1())
                .with_w(-(right_anti_dual_g3_xyz[0] * self[e415]) - (right_anti_dual_g3_xyz[1] * self[e425]) - (right_anti_dual_g3_xyz[2] * self[e435])),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(
                -(right_anti_dual_g1_xyz[1] * self[e425])
                    - (right_anti_dual_g1_xyz[2] * self[e435])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
            ) + (Simd32x3::from(other[e5] * -1.0) * self.group0()).with_w(0.0)
                + (right_anti_dual_g3_xyz.zxy() * self.group1().yzx()).with_w(0.0)
                - (right_anti_dual_g3_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g1_xyz[0] * self[e415]),
        )
    }
}
impl WeightContraction<AntiDualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(other[scalar]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * self.group1(),
        )
    }
}
impl WeightContraction<AntiFlector> for Line {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd3        0        3        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd       13       13        0        0
    fn weight_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g1_xyz[1] * self[e425]) - (right_anti_dual_g1_xyz[2] * self[e435]))
                + (Simd32x3::from(other[e5] * -1.0) * self.group0()).with_w(0.0)
                + (right_anti_dual_g1_xyz.zxy() * self.group1().yzx()).with_w(0.0)
                - (right_anti_dual_g1_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g1_xyz[0] * self[e415]),
        )
    }
}
impl WeightContraction<AntiLine> for Line {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
    fn weight_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (other[e23] * self[e235]) + (other[e31] * self[e315]) + (other[e12] * self[e125]) + (other[e15] * self[e415]) + (other[e25] * self[e425]) + (other[e35] * self[e435]),
            0.0,
        ]))
    }
}
impl WeightContraction<AntiMotor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       12        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x3::from(other[scalar]) * self.group0()).with_w(0.0),
            // e235, e315, e125, e5
            (Simd32x3::from(other[scalar]) * self.group1()).with_w(
                (other[e23] * self[e235])
                    + (other[e31] * self[e315])
                    + (other[e12] * self[e125])
                    + (other[e15] * self[e415])
                    + (other[e25] * self[e425])
                    + (other[e35] * self[e435]),
            ),
        )
    }
}
impl WeightContraction<AntiPlane> for Line {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd3        0        3        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd       13       13        0        0
    fn weight_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0_xyz[1] * self[e425]) - (right_anti_dual_g0_xyz[2] * self[e435]))
                + (Simd32x3::from(other[e5] * -1.0) * self.group0()).with_w(0.0)
                + (right_anti_dual_g0_xyz.zxy() * self.group1().yzx()).with_w(0.0)
                - (right_anti_dual_g0_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g0_xyz[0] * self[e415]),
        )
    }
}
impl WeightContraction<Circle> for Line {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
    fn weight_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        Scalar::from_groups(
            // scalar
            -(right_anti_dual_g1_xyz[0] * self[e415])
                - (right_anti_dual_g1_xyz[1] * self[e425])
                - (right_anti_dual_g1_xyz[2] * self[e435])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125]),
        )
    }
}
impl WeightContraction<CircleRotor> for Line {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
    fn weight_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        Scalar::from_groups(
            // scalar
            -(right_anti_dual_g1_xyz[0] * self[e415])
                - (right_anti_dual_g1_xyz[1] * self[e425])
                - (right_anti_dual_g1_xyz[2] * self[e435])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125]),
        )
    }
}
impl WeightContraction<Dipole> for Line {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        9        0        0
    //    simd3        0        3        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        8       12        0      N/A
    //  no simd       17       18        0        0
    fn weight_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e43] * self[e435])
                + (Simd32x3::from(other[e45]) * self.group0()).with_w(other[e42] * self[e425])
                + (other.group0().yzx() * self.group1().zxy()).with_w(other[e41] * self[e415])
                - (other.group0().zxy() * self.group1().yzx()).with_w(0.0),
            // e5
            (other[e23] * self[e235]) + (other[e31] * self[e315]) + (other[e12] * self[e125]) + (other[e15] * self[e415]) + (other[e25] * self[e425]) + (other[e35] * self[e435]),
        )
    }
}
impl WeightContraction<DipoleInversion> for Line {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        9        0        0
    //    simd3        0        3        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        8       12        0      N/A
    //  no simd       17       18        0        0
    fn weight_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e43] * self[e435])
                + (Simd32x3::from(other[e45]) * self.group0()).with_w(other[e42] * self[e425])
                + (other.group0().yzx() * self.group1().zxy()).with_w(other[e41] * self[e415])
                - (other.group0().zxy() * self.group1().yzx()).with_w(0.0),
            // e5
            (other[e23] * self[e235]) + (other[e31] * self[e315]) + (other[e12] * self[e125]) + (other[e15] * self[e415]) + (other[e25] * self[e425]) + (other[e35] * self[e435]),
        )
    }
}
impl WeightContraction<DualNum> for Line {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        4        0        0
    fn weight_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ (Simd32x3::from(other[e5] * -1.0) * self.group0()).with_w(0.0))
    }
}
impl WeightContraction<FlatPoint> for Line {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn weight_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x3::from(other[e45]) * self.group0()).with_w((other[e15] * self[e415]) + (other[e25] * self[e425]) + (other[e35] * self[e435])),
        )
    }
}
impl WeightContraction<Flector> for Line {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn weight_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x3::from(other[e45]) * self.group0()).with_w((other[e15] * self[e415]) + (other[e25] * self[e425]) + (other[e35] * self[e435])),
        )
    }
}
impl WeightContraction<Line> for Line {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn weight_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -(other[e415] * self[e415]) - (other[e425] * self[e425]) - (other[e435] * self[e435]))
    }
}
impl WeightContraction<Motor> for Line {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        7        0        0
    fn weight_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0_xyz[0] * self[e415]) - (right_anti_dual_g0_xyz[1] * self[e425]) - (right_anti_dual_g0_xyz[2] * self[e435])),
            // e15, e25, e35, e3215
            (Simd32x3::from(other[e5] * -1.0) * self.group0()).with_w(0.0),
        )
    }
}
impl WeightContraction<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       20        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0       10        0      N/A
    //    simd4        6        1        0      N/A
    // Totals...
    // yes simd       16       32        0      N/A
    //  no simd       34       56        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g5 = other.group6().xyz();
        let right_anti_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g9_xyz = other.group1().xyz();
        let right_anti_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(right_anti_dual_g5[0] * self[e415])
                    - (right_anti_dual_g5[1] * self[e425])
                    - (right_anti_dual_g5[2] * self[e435])
                    - (self[e235] * other[e423])
                    - (self[e315] * other[e431])
                    - (self[e125] * other[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e435] * other[e43])
                + (Simd32x3::from(other[e45]) * self.group0()).with_w(self[e415] * other[e41])
                + (self.group1().zxy() * other.group4().yzx()).with_w(self[e425] * other[e42])
                - (self.group1().yzx() * other.group4().zxy()).with_w(0.0),
            // e5
            (self[e235] * other[e23]) + (self[e315] * other[e31]) + (self[e125] * other[e12])
                - (right_anti_dual_g8[0] * self[e415])
                - (right_anti_dual_g8[1] * self[e425])
                - (right_anti_dual_g8[2] * self[e435]),
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w((right_anti_dual_g9_xyz[2] * self[e435]) * -1.0) + (right_anti_dual_g9_xyz.zxy() * self.group1().yzx()).with_w(0.0)
                - (Simd32x4::from([other[e5], other[e5], other[e5], right_anti_dual_g9_xyz[1] * self[e425]]) * self.group0().with_w(1.0))
                - (right_anti_dual_g9_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g9_xyz[0] * self[e415]),
            // e41, e42, e43
            Simd32x3::from(right_anti_dual_g10) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(right_anti_dual_g10) * self.group1(),
            // e415, e425, e435, e321
            (Simd32x3::from(right_anti_dual_g0[1]) * self.group0()).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(right_anti_dual_g0[1]) * self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<RoundPoint> for Line {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd        8       20        0        0
    fn weight_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1 = other[e4] * -1.0;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_anti_dual_g1) * self.group0(),
            // e23, e31, e12, e45
            (Simd32x3::from(right_anti_dual_g1) * self.group1())
                .with_w(-(right_anti_dual_g0_xyz[0] * self[e415]) - (right_anti_dual_g0_xyz[1] * self[e425]) - (right_anti_dual_g0_xyz[2] * self[e435])),
            // e15, e25, e35
            (Simd32x3::from(other[e5] * -1.0) * self.group0()) + (right_anti_dual_g0_xyz.zxy() * self.group1().yzx()) - (right_anti_dual_g0_xyz.yzx() * self.group1().zxy()),
        )
    }
}
impl WeightContraction<Scalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(other[scalar]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * self.group1(),
        )
    }
}
impl WeightContraction<VersorEven> for Line {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       11        0        0
    //    simd3        0        5        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        9       16        0      N/A
    //  no simd       18       26        0        0
    fn weight_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_anti_dual_g2_w) * self.group0(),
            // e23, e31, e12, e45
            (Simd32x3::from(right_anti_dual_g2_w) * self.group1())
                .with_w(-(right_anti_dual_g3_xyz[0] * self[e415]) - (right_anti_dual_g3_xyz[1] * self[e425]) - (right_anti_dual_g3_xyz[2] * self[e435])),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(
                -(right_anti_dual_g0_xyz[1] * self[e315])
                    - (right_anti_dual_g0_xyz[2] * self[e125])
                    - (right_anti_dual_g1_xyz[0] * self[e415])
                    - (right_anti_dual_g1_xyz[1] * self[e425])
                    - (right_anti_dual_g1_xyz[2] * self[e435]),
            ) + (Simd32x3::from(other[e5] * -1.0) * self.group0()).with_w(0.0)
                + (right_anti_dual_g3_xyz.zxy() * self.group1().yzx()).with_w(0.0)
                - (right_anti_dual_g3_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g0_xyz[0] * self[e235]),
        )
    }
}
impl WeightContraction<VersorOdd> for Line {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        0        5        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        7       14        0      N/A
    //  no simd       16       27        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2_xyz = other.group2().xyz() * Simd32x3::from(-1.0);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (Simd32x3::from(other[scalar]) * self.group0()).with_w(0.0),
            // e235, e315, e125, e4
            (Simd32x3::from(other[scalar]) * self.group1()).with_w((self[e415] * other[e41]) + (self[e425] * other[e42]) + (self[e435] * other[e43])),
            // e1, e2, e3, e5
            (Simd32x4::from([self[e415], self[e425], self[e435], self[e235]]) * other.group1().wwwx())
                + Simd32x3::from(0.0).with_w((self[e125] * other[e12]) - (right_anti_dual_g2_xyz[1] * self[e425]) - (right_anti_dual_g2_xyz[2] * self[e435]))
                + (self.group1().zxy() * other.group0().yzx()).with_w(self[e315] * other[e31])
                - (self.group1().yzx() * other.group0().zxy()).with_w(right_anti_dual_g2_xyz[0] * self[e415]),
        )
    }
}
impl std::ops::Div<WeightContractionInfix> for Motor {
    type Output = WeightContractionInfixPartial<Motor>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        1        5        0      N/A
    //    simd4        5        5        0      N/A
    // Totals...
    // yes simd       10       18        0      N/A
    //  no simd       27       43        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self[e12345]) * (other.group0() * Simd32x3::from(-1.0)).with_w(right_anti_dual_g2[3]),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_anti_dual_g2[3]) * self.group0().xyz()) - (Simd32x3::from(self[e12345]) * other.group1().xyz())).with_w(other[e45] * self[e12345]),
            // e235, e315, e125, e5
            (right_anti_dual_g2 * Simd32x3::from(self[e12345]).with_w(self[e5]))
                + (self.group1().xyzx() * Simd32x3::from(right_anti_dual_g2[3]).with_w(other[e23]))
                + Simd32x3::from(0.0).with_w(
                    (other[e31] * self[e315]) + (other[e12] * self[e125])
                        - (right_anti_dual_g2[0] * self[e415])
                        - (right_anti_dual_g2[1] * self[e425])
                        - (right_anti_dual_g2[2] * self[e435]),
                ),
            // e1, e2, e3, e4
            (self.group0().xyzx() * Simd32x3::from(other[e45]).with_w(other[e41]))
                + Simd32x3::from(0.0).with_w(other[e43] * self[e435])
                + (other.group0().yzx() * self.group1().zxy()).with_w(other[e42] * self[e425])
                - (other.group0().zxy() * self.group1().yzx()).with_w(0.0),
        )
    }
}
impl WeightContraction<AntiDipoleInversion> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       15        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4        5        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       15       24        0      N/A
    //  no simd       35       44        0        0
    fn weight_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3 = other.group3().xyz().with_w(other[e5] * -1.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from(right_anti_dual_g2_w) * self.group0().xyz().with_w(self[e5]))
                + Simd32x3::from(0.0).with_w(
                    -(right_anti_dual_g1[0] * self[e415])
                        - (right_anti_dual_g1[1] * self[e425])
                        - (right_anti_dual_g1[2] * self[e435])
                        - (other[e423] * self[e235])
                        - (other[e431] * self[e315])
                        - (other[e412] * self[e125]),
                )
                + (Simd32x3::from(self[e12345]) * other.group0()).with_w(0.0),
            // e23, e31, e12, e45
            (right_anti_dual_g1 * Simd32x4::from(self[e12345]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g3[0] * self[e415]) - (right_anti_dual_g3[1] * self[e425]) - (right_anti_dual_g3[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g2_w) * self.group1().xyz()).with_w(0.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_anti_dual_g3[3]) * self.group0().xyz())
                + (Simd32x3::from(self[e12345]) * other.group2().xyz())
                + (right_anti_dual_g3.zxy() * self.group1().yzx())
                + Simd32x2::from(0.0).with_z(right_anti_dual_g3[0] * self[e315] * -1.0)
                - (right_anti_dual_g3.yz() * self.group1().zx()).with_z(0.0))
            .with_w(right_anti_dual_g2_w * self[e12345]),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g3 * Simd32x4::from(self[e12345]),
        )
    }
}
impl WeightContraction<AntiDualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        1        9        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(other[scalar]) * self.group0(),
            // e235, e315, e125, e5
            (Simd32x3::from(other[scalar]) * self.group1().xyz()).with_w((other[e3215] * self[e12345]) + (other[scalar] * self[e5])),
        )
    }
}
impl WeightContraction<AntiFlatPoint> for Motor {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl WeightContraction<AntiFlector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        3        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        6        9        0      N/A
    //  no simd       18       21        0        0
    fn weight_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e5] * -1.0);
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g1[1] * self[e425]) - (right_anti_dual_g1[2] * self[e435]) - (other[e321] * self[e12345]))
                + (Simd32x3::from(right_anti_dual_g1[3]) * self.group0().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e12345]) * other.group0().xyz()).with_w(0.0)
                + (right_anti_dual_g1.zxy() * self.group1().yzx()).with_w(0.0)
                - (right_anti_dual_g1.yzxx() * self.group1().zxy().with_w(self[e415])),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g1 * Simd32x4::from(self[e12345]),
        )
    }
}
impl WeightContraction<AntiLine> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        4        0      N/A
    // Totals...
    // yes simd        5       10        0      N/A
    //  no simd        5       18        0        0
    fn weight_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        Motor::from_groups(
            // e415, e425, e435, e12345
            (right_anti_dual_g0 * Simd32x3::from(self[e12345])).with_w(0.0),
            // e235, e315, e125, e5
            (right_anti_dual_g1 * Simd32x3::from(self[e12345])).with_w(
                -(right_anti_dual_g0[0] * self[e235])
                    - (right_anti_dual_g0[1] * self[e315])
                    - (right_anti_dual_g0[2] * self[e125])
                    - (right_anti_dual_g1[0] * self[e415])
                    - (right_anti_dual_g1[1] * self[e425])
                    - (right_anti_dual_g1[2] * self[e435]),
            ),
        )
    }
}
impl WeightContraction<AntiMotor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        8       12        0      N/A
    //  no simd       16       25        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((Simd32x3::from(other[scalar]) * self.group0().xyz()) - (Simd32x3::from(self[e12345]) * other.group0().xyz())).with_w(other[scalar] * self[e12345]),
            // e235, e315, e125, e5
            (right_anti_dual_g1 * Simd32x4::from(self[e12345]))
                + (Simd32x4::from(other[scalar]) * self.group1())
                + Simd32x3::from(0.0).with_w(
                    (other[e23] * self[e235]) + (other[e31] * self[e315]) + (other[e12] * self[e125])
                        - (right_anti_dual_g1[0] * self[e415])
                        - (right_anti_dual_g1[1] * self[e425])
                        - (right_anti_dual_g1[2] * self[e435]),
                ),
        )
    }
}
impl WeightContraction<AntiPlane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd       13       17        0        0
    fn weight_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e5] * -1.0);
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e425]) - (right_anti_dual_g0[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g0[3]) * self.group0().xyz()).with_w(0.0)
                + (right_anti_dual_g0.zxy() * self.group1().yzx()).with_w(0.0)
                - (right_anti_dual_g0.yzxx() * self.group1().zxy().with_w(self[e415])),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g0 * Simd32x4::from(self[e12345]),
        )
    }
}
impl WeightContraction<AntiScalar> for Motor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_contraction(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e12345] * self[e12345] * -1.0)
    }
}
impl WeightContraction<Circle> for Motor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        5       10        0      N/A
    //  no simd        5       17        0        0
    fn weight_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            right_anti_dual_g1 * Simd32x4::from(self[e12345]),
            // e15, e25, e35, scalar
            (Simd32x3::from(self[e12345]) * other.group2()).with_w(
                -(right_anti_dual_g1[0] * self[e415])
                    - (right_anti_dual_g1[1] * self[e425])
                    - (right_anti_dual_g1[2] * self[e435])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
            ),
        )
    }
}
impl WeightContraction<CircleRotor> for Motor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6       11        0      N/A
    //  no simd        6       18        0        0
    fn weight_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            right_anti_dual_g1 * Simd32x4::from(self[e12345]),
            // e15, e25, e35, scalar
            (Simd32x3::from(self[e12345]) * other.group2().xyz()).with_w(
                -(right_anti_dual_g1[0] * self[e415])
                    - (right_anti_dual_g1[1] * self[e425])
                    - (right_anti_dual_g1[2] * self[e435])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125])
                    - (other[e12345] * self[e12345]),
            ),
        )
    }
}
impl WeightContraction<Dipole> for Motor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        0        6        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd        9       17        0      N/A
    //  no simd       18       38        0        0
    fn weight_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_anti_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            right_anti_dual_g0 * Simd32x3::from(self[e12345]),
            // e415, e425, e435, e321
            right_anti_dual_g1 * Simd32x4::from(self[e12345]),
            // e235, e315, e125, e4
            (right_anti_dual_g2 * Simd32x3::from(self[e12345]))
                .with_w(-(right_anti_dual_g0[0] * self[e415]) - (right_anti_dual_g0[1] * self[e425]) - (right_anti_dual_g0[2] * self[e435])),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(
                -(right_anti_dual_g2[0] * self[e415])
                    - (right_anti_dual_g2[1] * self[e425])
                    - (right_anti_dual_g2[2] * self[e435])
                    - (right_anti_dual_g1[1] * self[e315])
                    - (right_anti_dual_g1[2] * self[e125]),
            ) + (Simd32x3::from(right_anti_dual_g1[3]) * self.group0().xyz()).with_w(0.0)
                + (right_anti_dual_g0.zxy() * self.group1().yzx()).with_w(0.0)
                - (self.group1().zxyx() * right_anti_dual_g0.yzx().with_w(right_anti_dual_g1[0])),
        )
    }
}
impl WeightContraction<DipoleInversion> for Motor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       10        0        0
    //    simd3        0        5        0      N/A
    //    simd4        4        4        0      N/A
    // Totals...
    // yes simd       10       19        0      N/A
    //  no simd       22       41        0        0
    fn weight_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            right_anti_dual_g0 * Simd32x3::from(self[e12345]),
            // e415, e425, e435, e321
            right_anti_dual_g1 * Simd32x4::from(self[e12345]),
            // e235, e315, e125, e4
            (Simd32x3::from(self[e12345] * -1.0) * other.group2().xyz())
                .with_w((other[e1234] * self[e12345]) - (right_anti_dual_g0[0] * self[e415]) - (right_anti_dual_g0[1] * self[e425]) - (right_anti_dual_g0[2] * self[e435])),
            // e1, e2, e3, e5
            (self.group0() * Simd32x3::from(right_anti_dual_g1[3]).with_w(other[e3215]))
                + Simd32x3::from(0.0).with_w((other[e25] * self[e425]) + (other[e35] * self[e435]) - (right_anti_dual_g1[1] * self[e315]) - (right_anti_dual_g1[2] * self[e125]))
                + (right_anti_dual_g0.zxy() * self.group1().yzx()).with_w(other[e15] * self[e415])
                - (self.group1().zxyx() * right_anti_dual_g0.yzx().with_w(right_anti_dual_g1[0]))
                - (Simd32x3::from(self[e12345]) * other.group3().xyz()).with_w(0.0),
        )
    }
}
impl WeightContraction<DualNum> for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        7        0        0
    fn weight_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(other[e12345] * self[e12345] * -1.0),
            // e15, e25, e35, e3215
            Simd32x4::from(other[e5] * -1.0) * self.group0(),
        )
    }
}
impl WeightContraction<FlatPoint> for Motor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        2       14        0        0
    fn weight_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            right_anti_dual_g0 * Simd32x4::from(self[e12345]),
            // e1, e2, e3, e5
            (Simd32x3::from(right_anti_dual_g0[3]) * self.group0().xyz())
                .with_w(-(right_anti_dual_g0[0] * self[e415]) - (right_anti_dual_g0[1] * self[e425]) - (right_anti_dual_g0[2] * self[e435])),
        )
    }
}
impl WeightContraction<Flector> for Motor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        9       18        0        0
    fn weight_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            right_anti_dual_g0 * Simd32x4::from(self[e12345]),
            // e1, e2, e3, e5
            (self.group0() * Simd32x3::from(right_anti_dual_g0[3]).with_w(other[e3215]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e425]) - (right_anti_dual_g0[2] * self[e435]))
                - (self.group0().wwwx() * other.group1().xyz().with_w(right_anti_dual_g0[0])),
        )
    }
}
impl WeightContraction<Line> for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        9        0        0
    fn weight_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e12345]) * other.group0()).with_w(-(other[e415] * self[e415]) - (other[e425] * self[e425]) - (other[e435] * self[e435])),
            // e15, e25, e35, e3215
            (Simd32x3::from(self[e12345]) * other.group1()).with_w(0.0),
        )
    }
}
impl WeightContraction<Motor> for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        4        9        0      N/A
    //  no simd        6       15        0        0
    fn weight_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1_w = other[e5] * -1.0;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (right_anti_dual_g0_xyz * Simd32x3::from(self[e12345])).with_w(
                -(right_anti_dual_g0_xyz[0] * self[e415]) - (right_anti_dual_g0_xyz[1] * self[e425]) - (right_anti_dual_g0_xyz[2] * self[e435]) - (other[e12345] * self[e12345]),
            ),
            // e15, e25, e35, e3215
            ((Simd32x3::from(right_anti_dual_g1_w) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * other.group1().xyz())).with_w(right_anti_dual_g1_w * self[e12345]),
        )
    }
}
impl WeightContraction<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       16       27        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4       17        0      N/A
    //    simd4        8        4        0      N/A
    // Totals...
    // yes simd       28       49        0      N/A
    //  no simd       60       96        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g5 = other.group6().xyz();
        let right_anti_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_anti_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_anti_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g9 = other.group1().xyz().with_w(other[e5] * -1.0);
        let right_anti_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_anti_dual_g10 * self[e5]) + (right_anti_dual_g0[0] * self[e12345])
                    - (right_anti_dual_g5[0] * self[e415])
                    - (right_anti_dual_g5[1] * self[e425])
                    - (right_anti_dual_g5[2] * self[e435])
                    - (self[e235] * other[e423])
                    - (self[e315] * other[e431])
                    - (self[e125] * other[e412]),
                right_anti_dual_g0[1] * self[e12345],
            ]),
            // e1, e2, e3, e4
            (self.group0() * Simd32x3::from(other[e45]).with_w(other[e1234]))
                + Simd32x3::from(0.0).with_w((right_anti_dual_g7[2] * self[e435]) * -1.0)
                + (right_anti_dual_g7.zxy() * self.group1().yzx()).with_w(0.0)
                - (self.group0().wwwx() * other.group9().xyz().with_w(right_anti_dual_g7[0]))
                - (right_anti_dual_g7.yzx() * self.group1().zxy()).with_w(right_anti_dual_g7[1] * self[e425]),
            // e5
            (right_anti_dual_g0[1] * self[e5]) + (self[e12345] * other[e3215])
                - (right_anti_dual_g6_xyz[0] * self[e235])
                - (right_anti_dual_g6_xyz[1] * self[e315])
                - (right_anti_dual_g6_xyz[2] * self[e125])
                - (right_anti_dual_g8[0] * self[e415])
                - (right_anti_dual_g8[1] * self[e425])
                - (right_anti_dual_g8[2] * self[e435]),
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g9[1] * self[e425]) - (right_anti_dual_g9[2] * self[e435]) - (self[e12345] * other[e321]))
                + (Simd32x3::from(right_anti_dual_g9[3]) * self.group0().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e12345]) * other.group8()).with_w(0.0)
                + (right_anti_dual_g9.zxy() * self.group1().yzx()).with_w(0.0)
                - (right_anti_dual_g9.yzxx() * self.group1().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(right_anti_dual_g10) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * other.group7()),
            // e23, e31, e12
            (right_anti_dual_g5 * Simd32x3::from(self[e12345])) + (Simd32x3::from(right_anti_dual_g10) * self.group1().xyz()),
            // e415, e425, e435, e321
            ((right_anti_dual_g6_xyz * Simd32x3::from(self[e12345])) + (Simd32x3::from(right_anti_dual_g0[1]) * self.group0().xyz())).with_w(self[e12345] * other[e45]),
            // e423, e431, e412
            right_anti_dual_g7 * Simd32x3::from(self[e12345]),
            // e235, e315, e125
            (right_anti_dual_g8 * Simd32x3::from(self[e12345])) + (Simd32x3::from(right_anti_dual_g0[1]) * self.group1().xyz()),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g9 * Simd32x4::from(self[e12345]),
            // e1234
            right_anti_dual_g10 * self[e12345],
        )
    }
}
impl WeightContraction<Plane> for Motor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl WeightContraction<RoundPoint> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        9        0        0
    //    simd2        0        2        0      N/A
    //    simd3        3        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        5       15        0      N/A
    //  no simd       11       27        0        0
    fn weight_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e5] * -1.0);
        let right_anti_dual_g1 = other[e4] * -1.0;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(right_anti_dual_g1) * self.group0().xyz().with_w(self[e5]),
            // e23, e31, e12, e45
            (Simd32x2::from(right_anti_dual_g1) * self.group1().xy()).with_zw(
                right_anti_dual_g1 * self[e125],
                -(right_anti_dual_g0[0] * self[e415]) - (right_anti_dual_g0[1] * self[e425]) - (right_anti_dual_g0[2] * self[e435]),
            ),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_anti_dual_g0[3]) * self.group0().xyz())
                + (right_anti_dual_g0.zxy() * self.group1().yzx())
                + Simd32x2::from(0.0).with_z(right_anti_dual_g0[0] * self[e315] * -1.0)
                - (right_anti_dual_g0.yz() * self.group1().zx()).with_z(0.0))
            .with_w(right_anti_dual_g1 * self[e12345]),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g0 * Simd32x4::from(self[e12345]),
        )
    }
}
impl WeightContraction<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(other[scalar]) * self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl WeightContraction<Sphere> for Motor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        8        0        0
    fn weight_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]),
            // e5
            self[e12345] * other[e3215],
        )
    }
}
impl WeightContraction<VersorEven> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       17        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4        5        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       15       26        0      N/A
    //  no simd       35       46        0        0
    fn weight_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3 = other.group3().xyz().with_w(other[e5] * -1.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() * Simd32x3::from(right_anti_dual_g2_w).with_w(other[e12345] * -1.0))
                + Simd32x3::from(0.0).with_w(
                    -(right_anti_dual_g0_xyz[0] * self[e235])
                        - (right_anti_dual_g0_xyz[1] * self[e315])
                        - (right_anti_dual_g0_xyz[2] * self[e125])
                        - (right_anti_dual_g1[0] * self[e415])
                        - (right_anti_dual_g1[1] * self[e425])
                        - (right_anti_dual_g1[2] * self[e435]),
                )
                + (right_anti_dual_g0_xyz * Simd32x3::from(self[e12345])).with_w(right_anti_dual_g2_w * self[e5]),
            // e23, e31, e12, e45
            (right_anti_dual_g1 * Simd32x4::from(self[e12345]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g3[0] * self[e415]) - (right_anti_dual_g3[1] * self[e425]) - (right_anti_dual_g3[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g2_w) * self.group1().xyz()).with_w(0.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_anti_dual_g3[3]) * self.group0().xyz())
                + (Simd32x3::from(self[e12345]) * other.group2().xyz())
                + (right_anti_dual_g3.zxy() * self.group1().yzx())
                + Simd32x2::from(0.0).with_z(right_anti_dual_g3[0] * self[e315] * -1.0)
                - (right_anti_dual_g3.yz() * self.group1().zx()).with_z(0.0))
            .with_w(right_anti_dual_g2_w * self[e12345]),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g3 * Simd32x4::from(self[e12345]),
        )
    }
}
impl WeightContraction<VersorOdd> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       10        0        0
    //    simd3        1        5        0      N/A
    //    simd4        6        6        0      N/A
    // Totals...
    // yes simd       13       21        0      N/A
    //  no simd       33       49        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_anti_dual_g2 = (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(other[e3215]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            right_anti_dual_g0 * Simd32x4::from(self[e12345]),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_anti_dual_g0[3]) * self.group0().xyz()) - (Simd32x3::from(self[e12345]) * other.group1().xyz())).with_w(self[e12345] * other[e45]),
            // e235, e315, e125, e5
            (right_anti_dual_g2 * Simd32x4::from(self[e12345]))
                + (Simd32x4::from(right_anti_dual_g0[3]) * self.group1())
                + Simd32x3::from(0.0).with_w(
                    (self[e235] * other[e23]) + (self[e315] * other[e31]) + (self[e125] * other[e12])
                        - (right_anti_dual_g2[0] * self[e415])
                        - (right_anti_dual_g2[1] * self[e425])
                        - (right_anti_dual_g2[2] * self[e435]),
                ),
            // e1, e2, e3, e4
            (self.group0() * Simd32x3::from(other[e45]).with_w(other[e1234]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e425]) - (right_anti_dual_g0[2] * self[e435]))
                + (Simd32x3::from(self[e12345] * -1.0) * other.group3().xyz()).with_w(0.0)
                + (right_anti_dual_g0.zxy() * self.group1().yzx()).with_w(0.0)
                - (right_anti_dual_g0.yzxx() * self.group1().zxy().with_w(self[e415])),
        )
    }
}
impl std::ops::Div<WeightContractionInfix> for MultiVector {
    type Output = WeightContractionInfixPartial<MultiVector>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       20       28        0        0
    //    simd3        8       21        0      N/A
    //    simd4       12        7        0      N/A
    // Totals...
    // yes simd       40       56        0      N/A
    //  no simd       92      119        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43]) + (other[scalar] * self[scalar])
                    - (right_anti_dual_g0[0] * self[e15])
                    - (right_anti_dual_g0[1] * self[e25])
                    - (right_anti_dual_g0[2] * self[e35])
                    - (right_anti_dual_g1[0] * self[e23])
                    - (right_anti_dual_g1[1] * self[e31])
                    - (right_anti_dual_g1[2] * self[e12])
                    - (right_anti_dual_g1[3] * self[e45]),
                other[scalar] * self[e12345],
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(other[scalar]) * self.group1())
                + Simd32x3::from(0.0).with_w(
                    -(right_anti_dual_g0[2] * self[e435]) - (right_anti_dual_g1[0] * self[e423]) - (right_anti_dual_g1[1] * self[e431]) - (right_anti_dual_g1[2] * self[e412]),
                )
                + (Simd32x3::from(right_anti_dual_g1[3]) * self.group6().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * right_anti_dual_g1.xyz()).with_w(0.0)
                + (right_anti_dual_g0.zxy() * self.group8().yzx()).with_w(0.0)
                + (self.group7().yzx() * other.group2().zxy()).with_w(0.0)
                - (right_anti_dual_g0.yzx() * self.group8().zxy()).with_w(right_anti_dual_g0[0] * self[e415])
                - (self.group7().zxy() * other.group2().yzx()).with_w(right_anti_dual_g0[1] * self[e425]),
            // e5
            (other[e15] * self[e415]) + (other[e25] * self[e425]) + (other[e35] * self[e435]) + (other[scalar] * self[e5])
                - (right_anti_dual_g1[0] * self[e235])
                - (right_anti_dual_g1[1] * self[e315])
                - (right_anti_dual_g1[2] * self[e125]),
            // e15, e25, e35, e45
            (Simd32x4::from(other[scalar]) * self.group3())
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g1[1] * self[e4315]) - (right_anti_dual_g1[2] * self[e4125]))
                + (Simd32x3::from(self[e3215]) * right_anti_dual_g1.xyz()).with_w(0.0)
                + (other.group2().zxy() * self.group9().yzx()).with_w(0.0)
                - (self.group9().zxyx() * other.group2().yzx().with_w(right_anti_dual_g1[0])),
            // e41, e42, e43
            (Simd32x3::from(other[scalar]) * self.group4()) + (Simd32x3::from(self[e1234]) * right_anti_dual_g1.xyz()) + (right_anti_dual_g0.zxy() * self.group9().yzx())
                - (right_anti_dual_g0.yzx() * self.group9().zxy()),
            // e23, e31, e12
            (right_anti_dual_g0 * Simd32x3::from(self[e3215])) + (Simd32x3::from(other[scalar]) * self.group5())
                - (Simd32x3::from(right_anti_dual_g1[3]) * self.group9().xyz())
                - (Simd32x3::from(self[e1234]) * other.group2().xyz()),
            // e415, e425, e435, e321
            (right_anti_dual_g1 * Simd32x4::from(self[e12345])) + (Simd32x4::from(other[scalar]) * self.group6()),
            // e423, e431, e412
            (right_anti_dual_g0 * Simd32x3::from(self[e12345])) + (Simd32x3::from(other[scalar]) * self.group7()),
            // e235, e315, e125
            (Simd32x3::from(other[scalar]) * self.group8()) - (Simd32x3::from(self[e12345]) * other.group2().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group9(),
            // e1234
            other[scalar] * self[e1234],
        )
    }
}
impl WeightContraction<AntiDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       22       33        0        0
    //    simd3        8       17        0      N/A
    //    simd4       12       10        0      N/A
    // Totals...
    // yes simd       42       60        0      N/A
    //  no simd       94      124        0        0
    fn weight_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3 = other.group3().xyz().with_w(other[e5] * -1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_anti_dual_g2_w * self[e5])
                    + (right_anti_dual_g3[0] * self[e1])
                    + (right_anti_dual_g3[1] * self[e2])
                    + (right_anti_dual_g3[2] * self[e3])
                    + (right_anti_dual_g3[3] * self[e4])
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
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * right_anti_dual_g2_xyz.with_w(right_anti_dual_g1_w))
                + (Simd32x4::from([right_anti_dual_g3[2], right_anti_dual_g3[0], right_anti_dual_g3[1], other[e431] * self[e4315]]) * self.group5().yzx().with_w(1.0))
                + (self.group9().yzxx() * right_anti_dual_g1_xyz.zxy().with_w(other[e423]))
                + (Simd32x3::from(right_anti_dual_g3[3]) * self.group4()).with_w(other[e412] * self[e4125])
                - (Simd32x4::from(right_anti_dual_g2_w) * self.group3())
                - (right_anti_dual_g3.yzxx() * self.group5().zxy().with_w(self[e41]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(right_anti_dual_g3[2] * self[e43])
                - (right_anti_dual_g1_xyz.yzx() * self.group9().zxy()).with_w(right_anti_dual_g3[1] * self[e42]),
            // e5
            (right_anti_dual_g3[0] * self[e15]) + (right_anti_dual_g3[1] * self[e25]) + (right_anti_dual_g3[2] * self[e35]) + (right_anti_dual_g3[3] * self[e45])
                - (right_anti_dual_g1_w * self[e3215])
                - (right_anti_dual_g2_xyz[0] * self[e4235])
                - (right_anti_dual_g2_xyz[1] * self[e4315])
                - (right_anti_dual_g2_xyz[2] * self[e4125]),
            // e15, e25, e35, e45
            (Simd32x4::from(self[e12345]) * right_anti_dual_g2_xyz.with_w(right_anti_dual_g1_w))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g3[1] * self[e425]) - (right_anti_dual_g3[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g3[3]) * self.group6().xyz()).with_w(0.0)
                + (self.group8().yzx() * right_anti_dual_g3.zxy()).with_w(0.0)
                - (right_anti_dual_g3.yzxx() * self.group8().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(right_anti_dual_g2_w) * self.group6().xyz()) + (Simd32x3::from(self[e12345]) * other.group0()) + (self.group7().zxy() * right_anti_dual_g3.yzx())
                - (self.group7().yzx() * right_anti_dual_g3.zxy()),
            // e23, e31, e12
            (right_anti_dual_g1_xyz * Simd32x3::from(self[e12345]))
                + (Simd32x3::from(right_anti_dual_g2_w) * self.group8())
                + (Simd32x3::from(right_anti_dual_g3[3]) * self.group7())
                - (Simd32x3::from(self[e321]) * right_anti_dual_g3.xyz()),
            // e415, e425, e435, e321
            (right_anti_dual_g3.yzxw() * self.group9().zxy().with_w(self[e1234]))
                - (Simd32x4::from([right_anti_dual_g3[2], right_anti_dual_g3[0], right_anti_dual_g3[1], right_anti_dual_g2_w]) * self.group9().yzxw()),
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_anti_dual_g3.xyz()) - (Simd32x3::from(right_anti_dual_g2_w) * self.group9().xyz()),
            // e235, e315, e125
            (Simd32x3::from(right_anti_dual_g3[3]) * self.group9().xyz()) - (Simd32x3::from(self[e3215]) * right_anti_dual_g3.xyz()),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g3 * Simd32x4::from(self[e12345]),
            // e1234
            right_anti_dual_g2_w * self[e12345],
        )
    }
}
impl WeightContraction<AntiDualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        0        4        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        2       14        0      N/A
    //  no simd        2       34        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other[e3215] * self[e1234]) + (other[scalar] * self[scalar]), other[scalar] * self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group1(),
            // e5
            (other[e3215] * self[e12345]) + (other[scalar] * self[e5]),
            // e15, e25, e35, e45
            Simd32x4::from(other[scalar]) * self.group3(),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group4(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group6(),
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * self.group7(),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * self.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group9(),
            // e1234
            other[scalar] * self[e1234],
        )
    }
}
impl WeightContraction<AntiFlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        6       11        0      N/A
    //  no simd        6       17        0        0
    fn weight_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e321] * -1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(right_anti_dual_g0[0] * self[e423]) - (right_anti_dual_g0[1] * self[e431]) - (right_anti_dual_g0[2] * self[e412]) - (right_anti_dual_g0[3] * self[e321]),
                0.0,
            ]),
            // e1, e2, e3, e4
            right_anti_dual_g0 * Simd32x4::from(self[e1234]),
            // e5
            -(right_anti_dual_g0[0] * self[e4235]) - (right_anti_dual_g0[1] * self[e4315]) - (right_anti_dual_g0[2] * self[e4125]) - (right_anti_dual_g0[3] * self[e3215]),
            // e15, e25, e35, e45
            right_anti_dual_g0 * Simd32x4::from(self[e12345]),
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
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<AntiFlector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       16       25        0        0
    //    simd2        0        1        0      N/A
    //    simd3        5       12        0      N/A
    //    simd4        8        5        0      N/A
    // Totals...
    // yes simd       29       43        0      N/A
    //  no simd       63       83        0        0
    fn weight_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e321] * -1.0);
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e5] * -1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_anti_dual_g1[0] * self[e1]) + (right_anti_dual_g1[1] * self[e2]) + (right_anti_dual_g1[2] * self[e3]) + (right_anti_dual_g1[3] * self[e4])
                    - (right_anti_dual_g0[0] * self[e423])
                    - (right_anti_dual_g0[1] * self[e431])
                    - (right_anti_dual_g0[2] * self[e412])
                    - (right_anti_dual_g0[3] * self[e321]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (right_anti_dual_g0 * Simd32x4::from(self[e1234]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g1[1] * self[e42]) - (right_anti_dual_g1[2] * self[e43]))
                + (Simd32x3::from(right_anti_dual_g1[3]) * self.group4()).with_w(0.0)
                + (self.group5().yzx() * right_anti_dual_g1.zxy()).with_w(0.0)
                - (right_anti_dual_g1.yzxx() * self.group5().zxy().with_w(self[e41])),
            // e5
            (right_anti_dual_g1[0] * self[e15]) + (right_anti_dual_g1[1] * self[e25]) + (right_anti_dual_g1[2] * self[e35]) + (right_anti_dual_g1[3] * self[e45])
                - (right_anti_dual_g0[0] * self[e4235])
                - (right_anti_dual_g0[1] * self[e4315])
                - (right_anti_dual_g0[2] * self[e4125])
                - (right_anti_dual_g0[3] * self[e3215]),
            // e15, e25, e35, e45
            (right_anti_dual_g0 * Simd32x4::from(self[e12345]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g1[1] * self[e425]) - (right_anti_dual_g1[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g1[3]) * self.group6().xyz()).with_w(0.0)
                + (self.group8().yzx() * right_anti_dual_g1.zxy()).with_w(0.0)
                - (right_anti_dual_g1.yzxx() * self.group8().zxy().with_w(self[e415])),
            // e41, e42, e43
            (self.group7().zxy() * right_anti_dual_g1.yzx()) - (self.group7().yzx() * right_anti_dual_g1.zxy()),
            // e23, e31, e12
            (Simd32x3::from(right_anti_dual_g1[3]) * self.group7()) - (Simd32x3::from(self[e321]) * right_anti_dual_g1.xyz()),
            // e415, e425, e435, e321
            ((right_anti_dual_g1.yzx() * self.group9().zxy()) + Simd32x2::from(0.0).with_z(right_anti_dual_g1[1] * self[e4235] * -1.0)
                - (right_anti_dual_g1.zx() * self.group9().yz()).with_z(0.0))
            .with_w(right_anti_dual_g1[3] * self[e1234]),
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * right_anti_dual_g1.xyz(),
            // e235, e315, e125
            (Simd32x3::from(right_anti_dual_g1[3]) * self.group9().xyz()) - (Simd32x3::from(self[e3215]) * right_anti_dual_g1.xyz()),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g1 * Simd32x4::from(self[e12345]),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<AntiLine> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       17        0        0
    //    simd3        0       11        0      N/A
    //    simd4        6        1        0      N/A
    // Totals...
    // yes simd       18       29        0      N/A
    //  no simd       36       54        0        0
    fn weight_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(right_anti_dual_g0[0] * self[e23])
                    - (right_anti_dual_g0[1] * self[e31])
                    - (right_anti_dual_g0[2] * self[e12])
                    - (right_anti_dual_g1[0] * self[e41])
                    - (right_anti_dual_g1[1] * self[e42])
                    - (right_anti_dual_g1[2] * self[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e431]) - (right_anti_dual_g0[2] * self[e412]))
                + (right_anti_dual_g0 * Simd32x3::from(self[e321])).with_w(0.0)
                + (right_anti_dual_g1.yzx() * self.group7().zxy()).with_w(0.0)
                - (right_anti_dual_g1.zxy() * self.group7().yzx()).with_w(right_anti_dual_g0[0] * self[e423]),
            // e5
            -(right_anti_dual_g0[0] * self[e235])
                - (right_anti_dual_g0[1] * self[e315])
                - (right_anti_dual_g0[2] * self[e125])
                - (right_anti_dual_g1[0] * self[e415])
                - (right_anti_dual_g1[1] * self[e425])
                - (right_anti_dual_g1[2] * self[e435]),
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e4315]) - (right_anti_dual_g0[2] * self[e4125]))
                + (right_anti_dual_g0 * Simd32x3::from(self[e3215])).with_w(0.0)
                + (right_anti_dual_g1.yzx() * self.group9().zxy()).with_w(0.0)
                - (self.group9().yzxx() * right_anti_dual_g1.zxy().with_w(right_anti_dual_g0[0])),
            // e41, e42, e43
            right_anti_dual_g0 * Simd32x3::from(self[e1234]),
            // e23, e31, e12
            right_anti_dual_g1 * Simd32x3::from(self[e1234]),
            // e415, e425, e435, e321
            (right_anti_dual_g0 * Simd32x3::from(self[e12345])).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            right_anti_dual_g1 * Simd32x3::from(self[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       16       24        0        0
    //    simd3        4       14        0      N/A
    //    simd4        8        5        0      N/A
    // Totals...
    // yes simd       28       43        0      N/A
    //  no simd       60       86        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_anti_dual_g0[3] * self[scalar]) + (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43]) + (other[e3215] * self[e1234])
                    - (right_anti_dual_g0[0] * self[e23])
                    - (right_anti_dual_g0[1] * self[e31])
                    - (right_anti_dual_g0[2] * self[e12]),
                right_anti_dual_g0[3] * self[e12345],
            ]),
            // e1, e2, e3, e4
            (right_anti_dual_g0 * Simd32x3::from(self[e321]).with_w(self[e4]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e431]) - (right_anti_dual_g0[2] * self[e412]))
                + (Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz()).with_w(0.0)
                + (self.group7().yzx() * other.group1().zxy()).with_w(0.0)
                - (self.group7().zxy() * other.group1().yzx()).with_w(right_anti_dual_g0[0] * self[e423]),
            // e5
            (right_anti_dual_g0[3] * self[e5]) + (other[e15] * self[e415]) + (other[e25] * self[e425]) + (other[e35] * self[e435]) + (other[e3215] * self[e12345])
                - (right_anti_dual_g0[0] * self[e235])
                - (right_anti_dual_g0[1] * self[e315])
                - (right_anti_dual_g0[2] * self[e125]),
            // e15, e25, e35, e45
            (right_anti_dual_g0 * Simd32x3::from(self[e3215]).with_w(self[e45]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e4315]) - (right_anti_dual_g0[2] * self[e4125]))
                + (Simd32x3::from(right_anti_dual_g0[3]) * self.group3().xyz()).with_w(0.0)
                + (other.group1().zxy() * self.group9().yzx()).with_w(0.0)
                - (self.group9().zxyx() * other.group1().yzx().with_w(right_anti_dual_g0[0])),
            // e41, e42, e43
            (Simd32x3::from(right_anti_dual_g0[3]) * self.group4()) + (Simd32x3::from(self[e1234]) * right_anti_dual_g0.xyz()),
            // e23, e31, e12
            (Simd32x3::from(right_anti_dual_g0[3]) * self.group5()) - (Simd32x3::from(self[e1234]) * other.group1().xyz()),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_anti_dual_g0[3]) * self.group6().xyz()) + (Simd32x3::from(self[e12345]) * right_anti_dual_g0.xyz())).with_w(right_anti_dual_g0[3] * self[e321]),
            // e423, e431, e412
            Simd32x3::from(right_anti_dual_g0[3]) * self.group7(),
            // e235, e315, e125
            (Simd32x3::from(right_anti_dual_g0[3]) * self.group8()) - (Simd32x3::from(self[e12345]) * other.group1().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_anti_dual_g0[3]) * self.group9(),
            // e1234
            right_anti_dual_g0[3] * self[e1234],
        )
    }
}
impl WeightContraction<AntiPlane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       16        0        0
    //    simd2        0        1        0      N/A
    //    simd3        5       12        0      N/A
    //    simd4        6        3        0      N/A
    // Totals...
    // yes simd       19       32        0      N/A
    //  no simd       47       66        0        0
    fn weight_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e5] * -1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_anti_dual_g0[0] * self[e1]) + (right_anti_dual_g0[1] * self[e2]) + (right_anti_dual_g0[2] * self[e3]) + (right_anti_dual_g0[3] * self[e4]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e42]) - (right_anti_dual_g0[2] * self[e43]))
                + (Simd32x3::from(right_anti_dual_g0[3]) * self.group4()).with_w(0.0)
                + (self.group5().yzx() * right_anti_dual_g0.zxy()).with_w(0.0)
                - (right_anti_dual_g0.yzxx() * self.group5().zxy().with_w(self[e41])),
            // e5
            (right_anti_dual_g0[0] * self[e15]) + (right_anti_dual_g0[1] * self[e25]) + (right_anti_dual_g0[2] * self[e35]) + (right_anti_dual_g0[3] * self[e45]),
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e425]) - (right_anti_dual_g0[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g0[3]) * self.group6().xyz()).with_w(0.0)
                + (self.group8().yzx() * right_anti_dual_g0.zxy()).with_w(0.0)
                - (right_anti_dual_g0.yzxx() * self.group8().zxy().with_w(self[e415])),
            // e41, e42, e43
            (self.group7().zxy() * right_anti_dual_g0.yzx()) - (self.group7().yzx() * right_anti_dual_g0.zxy()),
            // e23, e31, e12
            (Simd32x3::from(right_anti_dual_g0[3]) * self.group7()) - (Simd32x3::from(self[e321]) * right_anti_dual_g0.xyz()),
            // e415, e425, e435, e321
            ((right_anti_dual_g0.yzx() * self.group9().zxy()) + Simd32x2::from(0.0).with_z(right_anti_dual_g0[1] * self[e4235] * -1.0)
                - (right_anti_dual_g0.zx() * self.group9().yz()).with_z(0.0))
            .with_w(right_anti_dual_g0[3] * self[e1234]),
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * right_anti_dual_g0.xyz(),
            // e235, e315, e125
            (Simd32x3::from(right_anti_dual_g0[3]) * self.group9().xyz()) - (Simd32x3::from(self[e3215]) * right_anti_dual_g0.xyz()),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g0 * Simd32x4::from(self[e12345]),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<AntiScalar> for MultiVector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_contraction(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e12345] * self[e12345] * -1.0)
    }
}
impl WeightContraction<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       17        0        0
    //    simd3        0        4        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       17       24        0      N/A
    //  no simd       29       41        0        0
    fn weight_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
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
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * other.group2().with_w(right_anti_dual_g1_w))
                + (self.group9().yzxx() * right_anti_dual_g1_xyz.zxy().with_w(other[e423]))
                + Simd32x3::from(0.0).with_w((other[e431] * self[e4315]) + (other[e412] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (right_anti_dual_g1_xyz.yzx() * self.group9().zxy()).with_w(0.0),
            // e5
            -(right_anti_dual_g1_w * self[e3215]) - (other[e235] * self[e4235]) - (other[e315] * self[e4315]) - (other[e125] * self[e4125]),
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * other.group2().with_w(right_anti_dual_g1_w),
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12
            right_anti_dual_g1_xyz * Simd32x3::from(self[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       14       18        0        0
    //    simd3        0        4        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       18       25        0      N/A
    //  no simd       30       42        0        0
    fn weight_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(right_anti_dual_g1_w * self[e321])
                    - (right_anti_dual_g1_xyz[0] * self[e415])
                    - (right_anti_dual_g1_xyz[1] * self[e425])
                    - (right_anti_dual_g1_xyz[2] * self[e435])
                    - (right_anti_dual_g2_xyz[0] * self[e423])
                    - (right_anti_dual_g2_xyz[1] * self[e431])
                    - (right_anti_dual_g2_xyz[2] * self[e412])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125])
                    - (other[e12345] * self[e12345]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * right_anti_dual_g2_xyz.with_w(right_anti_dual_g1_w))
                + (self.group9().yzxx() * right_anti_dual_g1_xyz.zxy().with_w(other[e423]))
                + Simd32x3::from(0.0).with_w((other[e431] * self[e4315]) + (other[e412] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (right_anti_dual_g1_xyz.yzx() * self.group9().zxy()).with_w(0.0),
            // e5
            -(right_anti_dual_g1_w * self[e3215])
                - (right_anti_dual_g2_xyz[0] * self[e4235])
                - (right_anti_dual_g2_xyz[1] * self[e4315])
                - (right_anti_dual_g2_xyz[2] * self[e4125]),
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * right_anti_dual_g2_xyz.with_w(right_anti_dual_g1_w),
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12
            right_anti_dual_g1_xyz * Simd32x3::from(self[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       18       24        0        0
    //    simd3        4       18        0      N/A
    //    simd4        9        3        0      N/A
    // Totals...
    // yes simd       31       45        0      N/A
    //  no simd       66       90        0        0
    fn weight_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_anti_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
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
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0)
                .with_w(-(right_anti_dual_g0[2] * self[e435]) - (right_anti_dual_g1[0] * self[e423]) - (right_anti_dual_g1[1] * self[e431]) - (right_anti_dual_g1[2] * self[e412]))
                + (Simd32x3::from(right_anti_dual_g1[3]) * self.group6().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * right_anti_dual_g1.xyz()).with_w(0.0)
                + (right_anti_dual_g0.zxy() * self.group8().yzx()).with_w(0.0)
                + (right_anti_dual_g2.yzx() * self.group7().zxy()).with_w(0.0)
                - (right_anti_dual_g0.yzx() * self.group8().zxy()).with_w(right_anti_dual_g0[0] * self[e415])
                - (right_anti_dual_g2.zxy() * self.group7().yzx()).with_w(right_anti_dual_g0[1] * self[e425]),
            // e5
            -(right_anti_dual_g2[0] * self[e415])
                - (right_anti_dual_g2[1] * self[e425])
                - (right_anti_dual_g2[2] * self[e435])
                - (right_anti_dual_g1[0] * self[e235])
                - (right_anti_dual_g1[1] * self[e315])
                - (right_anti_dual_g1[2] * self[e125]),
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g1[1] * self[e4315]) - (right_anti_dual_g1[2] * self[e4125]))
                + (Simd32x3::from(self[e3215]) * right_anti_dual_g1.xyz()).with_w(0.0)
                + (right_anti_dual_g2.yzx() * self.group9().zxy()).with_w(0.0)
                - (self.group9().yzxx() * right_anti_dual_g2.zxy().with_w(right_anti_dual_g1[0])),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_anti_dual_g1.xyz()) + (right_anti_dual_g0.zxy() * self.group9().yzx()) - (right_anti_dual_g0.yzx() * self.group9().zxy()),
            // e23, e31, e12
            (right_anti_dual_g0 * Simd32x3::from(self[e3215])) + (right_anti_dual_g2 * Simd32x3::from(self[e1234])) - (Simd32x3::from(right_anti_dual_g1[3]) * self.group9().xyz()),
            // e415, e425, e435, e321
            right_anti_dual_g1 * Simd32x4::from(self[e12345]),
            // e423, e431, e412
            right_anti_dual_g0 * Simd32x3::from(self[e12345]),
            // e235, e315, e125
            right_anti_dual_g2 * Simd32x3::from(self[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       23       32        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4       16        0      N/A
    //    simd4       10        4        0      N/A
    // Totals...
    // yes simd       37       53        0      N/A
    //  no simd       75       98        0        0
    fn weight_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43]) + (other[e1234] * self[e3215]) + (other[e3215] * self[e1234])
                    - (right_anti_dual_g0[0] * self[e15])
                    - (right_anti_dual_g0[1] * self[e25])
                    - (right_anti_dual_g0[2] * self[e35])
                    - (right_anti_dual_g1[0] * self[e23])
                    - (right_anti_dual_g1[1] * self[e31])
                    - (right_anti_dual_g1[2] * self[e12])
                    - (right_anti_dual_g1[3] * self[e45])
                    - (other[e4235] * self[e4235])
                    - (other[e4315] * self[e4315])
                    - (other[e4125] * self[e4125]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (other.group2().zxyw() * self.group7().yzx().with_w(self[e12345]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g1[0] * self[e423]) - (right_anti_dual_g1[1] * self[e431]) - (right_anti_dual_g1[2] * self[e412]))
                + (Simd32x3::from(right_anti_dual_g1[3]) * self.group6().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * right_anti_dual_g1.xyz()).with_w(0.0)
                + (right_anti_dual_g0.zxy() * self.group8().yzx()).with_w(0.0)
                - (Simd32x2::from(self[e12345]) * other.group3().xy()).with_zw(other[e4125] * self[e12345], right_anti_dual_g0[2] * self[e435])
                - (right_anti_dual_g0.yzx() * self.group8().zxy()).with_w(right_anti_dual_g0[0] * self[e415])
                - (self.group7().zxy() * other.group2().yzx()).with_w(right_anti_dual_g0[1] * self[e425]),
            // e5
            (other[e15] * self[e415]) + (other[e25] * self[e425]) + (other[e35] * self[e435]) + (other[e3215] * self[e12345])
                - (right_anti_dual_g1[0] * self[e235])
                - (right_anti_dual_g1[1] * self[e315])
                - (right_anti_dual_g1[2] * self[e125]),
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g1[1] * self[e4315]) - (right_anti_dual_g1[2] * self[e4125]))
                + (Simd32x3::from(self[e3215]) * right_anti_dual_g1.xyz()).with_w(0.0)
                + (other.group2().zxy() * self.group9().yzx()).with_w(0.0)
                - (self.group9().zxyx() * other.group2().yzx().with_w(right_anti_dual_g1[0])),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_anti_dual_g1.xyz()) + (right_anti_dual_g0.zxy() * self.group9().yzx()) - (right_anti_dual_g0.yzx() * self.group9().zxy()),
            // e23, e31, e12
            (right_anti_dual_g0 * Simd32x3::from(self[e3215]))
                - (Simd32x3::from(right_anti_dual_g1[3]) * self.group9().xyz())
                - (Simd32x3::from(self[e1234]) * other.group2().xyz()),
            // e415, e425, e435, e321
            right_anti_dual_g1 * Simd32x4::from(self[e12345]),
            // e423, e431, e412
            right_anti_dual_g0 * Simd32x3::from(self[e12345]),
            // e235, e315, e125
            Simd32x3::from(self[e12345] * -1.0) * other.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1       12        0        0
    //    simd3        0        4        0      N/A
    // Totals...
    // yes simd        1       16        0      N/A
    //  no simd        1       24        0        0
    fn weight_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([-(other[e5] * self[e4]) - (other[e12345] * self[e12345]), 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(other[e5] * -1.0) * self.group4()).with_w(0.0),
            // e5
            other[e5] * self[e45] * -1.0,
            // e15, e25, e35, e45
            (Simd32x3::from(other[e5] * -1.0) * self.group6().xyz()).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other[e5] * -1.0) * self.group7(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e5] * self[e1234] * -1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(other[e5] * -1.0) * self.group9().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(other[e5] * self[e12345] * -1.0),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        9        0        0
    //    simd3        2        8        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        9       17        0      N/A
    //  no simd       19       33        0        0
    fn weight_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43]) - (other[e45] * self[e45]), 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(other[e45]) * self.group6().xyz()).with_w(0.0) + (self.group7().yzx() * other.group0().zxy()).with_w(0.0)
                - (self.group7().zxy() * other.group0().yzx()).with_w(0.0),
            // e5
            (other[e15] * self[e415]) + (other[e25] * self[e425]) + (other[e35] * self[e435]),
            // e15, e25, e35, e45
            ((other.group0().zxy() * self.group9().yzx()) - (other.group0().yzx() * self.group9().zxy())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            -(Simd32x3::from(other[e45]) * self.group9().xyz()) - (Simd32x3::from(self[e1234]) * other.group0().xyz()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e45] * self[e12345]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[e12345] * -1.0) * other.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       14        0        0
    //    simd3        2        9        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd       15       23        0      N/A
    //  no simd       28       41        0        0
    fn weight_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43]) + (other[e3215] * self[e1234])
                    - (other[e45] * self[e45])
                    - (other[e4235] * self[e4235])
                    - (other[e4315] * self[e4315])
                    - (other[e4125] * self[e4125]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(other[e45]) * self.group6().xyz()).with_w(0.0) + (self.group7().yzx() * other.group0().zxy()).with_w(0.0)
                - (Simd32x3::from(self[e12345]) * other.group1().xyz()).with_w(0.0)
                - (self.group7().zxy() * other.group0().yzx()).with_w(0.0),
            // e5
            (other[e15] * self[e415]) + (other[e25] * self[e425]) + (other[e35] * self[e435]) + (other[e3215] * self[e12345]),
            // e15, e25, e35, e45
            ((other.group0().zxy() * self.group9().yzx()) - (other.group0().yzx() * self.group9().zxy())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            -(Simd32x3::from(other[e45]) * self.group9().xyz()) - (Simd32x3::from(self[e1234]) * other.group0().xyz()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e45] * self[e12345]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[e12345] * -1.0) * other.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7        9        0        0
    //    simd3        0        5        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd       15       24        0        0
    fn weight_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(other[e415] * self[e415])
                    - (other[e425] * self[e425])
                    - (other[e435] * self[e435])
                    - (other[e235] * self[e423])
                    - (other[e315] * self[e431])
                    - (other[e125] * self[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(self[e1234]) * other.group1()).with_w(0.0) + (other.group0().zxy() * self.group9().yzx()).with_w(0.0)
                - (other.group0().yzx() * self.group9().zxy()).with_w(0.0),
            // e5
            -(other[e235] * self[e4235]) - (other[e315] * self[e4315]) - (other[e125] * self[e4125]),
            // e15, e25, e35, e45
            (Simd32x3::from(self[e12345]) * other.group1()).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e12345]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       15        0        0
    //    simd3        2        9        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd       15       24        0      N/A
    //  no simd       28       42        0        0
    fn weight_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e5] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_anti_dual_g1_w * self[e4])
                    - (right_anti_dual_g0_xyz[0] * self[e415])
                    - (right_anti_dual_g0_xyz[1] * self[e425])
                    - (right_anti_dual_g0_xyz[2] * self[e435])
                    - (right_anti_dual_g1_xyz[0] * self[e423])
                    - (right_anti_dual_g1_xyz[1] * self[e431])
                    - (right_anti_dual_g1_xyz[2] * self[e412])
                    - (other[e12345] * self[e12345]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (right_anti_dual_g1_xyz * Simd32x3::from(self[e1234])).with_w(0.0)
                + (Simd32x3::from(right_anti_dual_g1_w) * self.group4()).with_w(0.0)
                + (right_anti_dual_g0_xyz.zxy() * self.group9().yzx()).with_w(0.0)
                - (right_anti_dual_g0_xyz.yzx() * self.group9().zxy()).with_w(0.0),
            // e5
            (right_anti_dual_g1_w * self[e45]) - (right_anti_dual_g1_xyz[0] * self[e4235]) - (right_anti_dual_g1_xyz[1] * self[e4315]) - (right_anti_dual_g1_xyz[2] * self[e4125]),
            // e15, e25, e35, e45
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e12345])) + (Simd32x3::from(right_anti_dual_g1_w) * self.group6().xyz())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (right_anti_dual_g0_xyz * Simd32x3::from(self[e12345])) + (Simd32x3::from(right_anti_dual_g1_w) * self.group7()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(right_anti_dual_g1_w * self[e1234]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(right_anti_dual_g1_w) * self.group9().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(right_anti_dual_g1_w * self[e12345]),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       53       68        0        0
    //    simd2        0        1        0      N/A
    //    simd3       20       42        0      N/A
    //    simd4       28       16        0      N/A
    // Totals...
    // yes simd      101      127        0      N/A
    //  no simd      225      260        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
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
                    - (right_anti_dual_g3[0] * self[e423])
                    - (right_anti_dual_g3[1] * self[e431])
                    - (right_anti_dual_g3[2] * self[e412])
                    - (right_anti_dual_g3[3] * self[e321])
                    - (right_anti_dual_g6[0] * self[e23])
                    - (right_anti_dual_g6[1] * self[e31])
                    - (right_anti_dual_g6[2] * self[e12])
                    - (right_anti_dual_g6[3] * self[e45])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
                right_anti_dual_g0[1] * self[e12345],
            ]),
            // e1, e2, e3, e4
            (right_anti_dual_g1 * Simd32x4::from(self[e12345]))
                + (right_anti_dual_g3 * Simd32x4::from(self[e1234]))
                + (Simd32x4::from(right_anti_dual_g0[1]) * self.group1())
                + (self.group9().yzxx() * right_anti_dual_g5.zxy().with_w(other[e423]))
                + Simd32x3::from(0.0).with_w(
                    -(right_anti_dual_g6[1] * self[e431]) - (right_anti_dual_g6[2] * self[e412]) - (right_anti_dual_g9[1] * self[e42]) - (right_anti_dual_g9[2] * self[e43]),
                )
                + (Simd32x3::from(right_anti_dual_g6[3]) * self.group6().xyz()).with_w(0.0)
                + (Simd32x3::from(right_anti_dual_g9[3]) * self.group4()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * right_anti_dual_g6.xyz()).with_w(0.0)
                + (right_anti_dual_g7.zxy() * self.group8().yzx()).with_w(other[e431] * self[e4315])
                + (right_anti_dual_g8.yzx() * self.group7().zxy()).with_w(other[e412] * self[e4125])
                + (self.group5().yzx() * right_anti_dual_g9.zxy()).with_w(0.0)
                - (Simd32x4::from(right_anti_dual_g10) * self.group3())
                - (right_anti_dual_g9.yzxx() * self.group5().zxy().with_w(self[e41]))
                - (Simd32x3::from(self[e3215]) * other.group7()).with_w(right_anti_dual_g6[0] * self[e423])
                - (right_anti_dual_g5.yzx() * self.group9().zxy()).with_w(right_anti_dual_g7[0] * self[e415])
                - (right_anti_dual_g7.yzx() * self.group8().zxy()).with_w(right_anti_dual_g7[1] * self[e425])
                - (right_anti_dual_g8.zxy() * self.group7().yzx()).with_w(right_anti_dual_g7[2] * self[e435]),
            // e5
            (right_anti_dual_g0[1] * self[e5])
                + (right_anti_dual_g9[0] * self[e15])
                + (right_anti_dual_g9[1] * self[e25])
                + (right_anti_dual_g9[2] * self[e35])
                + (right_anti_dual_g9[3] * self[e45])
                + (other[e3215] * self[e12345])
                - (right_anti_dual_g8[0] * self[e415])
                - (right_anti_dual_g8[1] * self[e425])
                - (right_anti_dual_g8[2] * self[e435])
                - (right_anti_dual_g3[0] * self[e4235])
                - (right_anti_dual_g3[1] * self[e4315])
                - (right_anti_dual_g3[2] * self[e4125])
                - (right_anti_dual_g3[3] * self[e3215])
                - (right_anti_dual_g6[0] * self[e235])
                - (right_anti_dual_g6[1] * self[e315])
                - (right_anti_dual_g6[2] * self[e125]),
            // e15, e25, e35, e45
            (right_anti_dual_g3 * Simd32x4::from(self[e12345]))
                + (Simd32x4::from(right_anti_dual_g0[1]) * self.group3())
                + Simd32x3::from(0.0).with_w(
                    -(right_anti_dual_g6[1] * self[e4315]) - (right_anti_dual_g6[2] * self[e4125]) - (right_anti_dual_g9[1] * self[e425]) - (right_anti_dual_g9[2] * self[e435]),
                )
                + (Simd32x3::from(right_anti_dual_g9[3]) * self.group6().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e3215]) * right_anti_dual_g6.xyz()).with_w(0.0)
                + (right_anti_dual_g8.yzx() * self.group9().zxy()).with_w(0.0)
                + (self.group8().yzx() * right_anti_dual_g9.zxy()).with_w(0.0)
                - (right_anti_dual_g9.yzxx() * self.group8().zxy().with_w(self[e415]))
                - (self.group9().yzxx() * right_anti_dual_g8.zxy().with_w(right_anti_dual_g6[0])),
            // e41, e42, e43
            (Simd32x3::from(right_anti_dual_g10) * self.group6().xyz())
                + (Simd32x3::from(right_anti_dual_g0[1]) * self.group4())
                + (Simd32x3::from(self[e12345]) * other.group7())
                + (Simd32x3::from(self[e1234]) * right_anti_dual_g6.xyz())
                + (right_anti_dual_g7.zxy() * self.group9().yzx())
                + (self.group7().zxy() * right_anti_dual_g9.yzx())
                - (right_anti_dual_g7.yzx() * self.group9().zxy())
                - (self.group7().yzx() * right_anti_dual_g9.zxy()),
            // e23, e31, e12
            (right_anti_dual_g5 * Simd32x3::from(self[e12345]))
                + (right_anti_dual_g7 * Simd32x3::from(self[e3215]))
                + (right_anti_dual_g8 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(right_anti_dual_g10) * self.group8())
                + (Simd32x3::from(right_anti_dual_g0[1]) * self.group5())
                + (Simd32x3::from(right_anti_dual_g9[3]) * self.group7())
                - (Simd32x3::from(right_anti_dual_g6[3]) * self.group9().xyz())
                - (Simd32x3::from(self[e321]) * right_anti_dual_g9.xyz()),
            // e415, e425, e435, e321
            (right_anti_dual_g6 * Simd32x4::from(self[e12345]))
                + (Simd32x4::from(right_anti_dual_g0[1]) * self.group6())
                + (right_anti_dual_g9.yzxw() * self.group9().zxy().with_w(self[e1234]))
                - (Simd32x4::from([right_anti_dual_g9[2], right_anti_dual_g9[0], right_anti_dual_g9[1], right_anti_dual_g10]) * self.group9().yzxw()),
            // e423, e431, e412
            (right_anti_dual_g7 * Simd32x3::from(self[e12345]))
                + (Simd32x3::from(right_anti_dual_g0[1]) * self.group7())
                + (Simd32x3::from(self[e1234]) * right_anti_dual_g9.xyz())
                - (Simd32x3::from(right_anti_dual_g10) * self.group9().xyz()),
            // e235, e315, e125
            (right_anti_dual_g8 * Simd32x3::from(self[e12345]))
                + (Simd32x3::from(right_anti_dual_g0[1]) * self.group8())
                + (Simd32x3::from(right_anti_dual_g9[3]) * self.group9().xyz())
                - (Simd32x3::from(self[e3215]) * right_anti_dual_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (right_anti_dual_g9 * Simd32x4::from(self[e12345])) + (Simd32x4::from(right_anti_dual_g0[1]) * self.group9()),
            // e1234
            (right_anti_dual_g10 * self[e12345]) + (right_anti_dual_g0[1] * self[e1234]),
        )
    }
}
impl WeightContraction<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3        9        0        0
    fn weight_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e1234] * other[e3215]) - (self[e4235] * other[e4235]) - (self[e4315] * other[e4315]) - (self[e4125] * other[e4125]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(self[e12345] * -1.0) * other.group0().xyz()).with_w(0.0),
            // e5
            self[e12345] * other[e3215],
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
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       16        0        0
    //    simd3        6       14        0      N/A
    //    simd4        8        6        0      N/A
    // Totals...
    // yes simd       23       36        0      N/A
    //  no simd       59       82        0        0
    fn weight_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e5] * -1.0);
        let right_anti_dual_g1 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_anti_dual_g1 * self[e5])
                    + (right_anti_dual_g0[0] * self[e1])
                    + (right_anti_dual_g0[1] * self[e2])
                    + (right_anti_dual_g0[2] * self[e3])
                    + (right_anti_dual_g0[3] * self[e4]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e42]) - (right_anti_dual_g0[2] * self[e43]))
                + (Simd32x3::from(right_anti_dual_g0[3]) * self.group4()).with_w(0.0)
                + (self.group5().yzx() * right_anti_dual_g0.zxy()).with_w(0.0)
                - (Simd32x4::from(right_anti_dual_g1) * self.group3())
                - (right_anti_dual_g0.yzxx() * self.group5().zxy().with_w(self[e41])),
            // e5
            (right_anti_dual_g0[0] * self[e15]) + (right_anti_dual_g0[1] * self[e25]) + (right_anti_dual_g0[2] * self[e35]) + (right_anti_dual_g0[3] * self[e45]),
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e425]) - (right_anti_dual_g0[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g0[3]) * self.group6().xyz()).with_w(0.0)
                + (self.group8().yzx() * right_anti_dual_g0.zxy()).with_w(0.0)
                - (right_anti_dual_g0.yzxx() * self.group8().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(right_anti_dual_g1) * self.group6().xyz()) + (self.group7().zxy() * right_anti_dual_g0.yzx()) - (self.group7().yzx() * right_anti_dual_g0.zxy()),
            // e23, e31, e12
            (Simd32x3::from(right_anti_dual_g1) * self.group8()) + (Simd32x3::from(right_anti_dual_g0[3]) * self.group7())
                - (Simd32x3::from(self[e321]) * right_anti_dual_g0.xyz()),
            // e415, e425, e435, e321
            (right_anti_dual_g0.yzxw() * self.group9().zxy().with_w(self[e1234]))
                - (Simd32x4::from([right_anti_dual_g0[2], right_anti_dual_g0[0], right_anti_dual_g0[1], right_anti_dual_g1]) * self.group9().yzxw()),
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_anti_dual_g0.xyz()) - (Simd32x3::from(right_anti_dual_g1) * self.group9().xyz()),
            // e235, e315, e125
            (Simd32x3::from(right_anti_dual_g0[3]) * self.group9().xyz()) - (Simd32x3::from(self[e3215]) * right_anti_dual_g0.xyz()),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g0 * Simd32x4::from(self[e12345]),
            // e1234
            right_anti_dual_g1 * self[e12345],
        )
    }
}
impl WeightContraction<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        4        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0       11        0      N/A
    //  no simd        0       32        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(other[scalar]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group1(),
            // e5
            self[e5] * other[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(other[scalar]) * self.group3(),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group4(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group6(),
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * self.group7(),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * self.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group9(),
            // e1234
            self[e1234] * other[scalar],
        )
    }
}
impl WeightContraction<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        4       13        0        0
    fn weight_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_anti_dual_g0[0] * self[e4235])
                    + (right_anti_dual_g0[1] * self[e4315])
                    + (right_anti_dual_g0[2] * self[e4125])
                    + (right_anti_dual_g0[3] * self[e3215])
                    + (self[e1234] * other[e3215]),
                0.0,
            ]),
            // e1, e2, e3, e4
            right_anti_dual_g0 * Simd32x4::from(self[e12345]),
            // e5
            self[e12345] * other[e3215],
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
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       23       34        0        0
    //    simd3        8       17        0      N/A
    //    simd4       12       10        0      N/A
    // Totals...
    // yes simd       43       61        0      N/A
    //  no simd       95      125        0        0
    fn weight_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3 = other.group3().xyz().with_w(other[e5] * -1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_anti_dual_g2_w * self[e5])
                    + (right_anti_dual_g3[0] * self[e1])
                    + (right_anti_dual_g3[1] * self[e2])
                    + (right_anti_dual_g3[2] * self[e3])
                    + (right_anti_dual_g3[3] * self[e4])
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
                    - (self[e12345] * other[e12345]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * right_anti_dual_g2_xyz.with_w(right_anti_dual_g1_w))
                + (Simd32x4::from([right_anti_dual_g3[2], right_anti_dual_g3[0], right_anti_dual_g3[1], right_anti_dual_g0_xyz[1] * self[e4315]])
                    * self.group5().yzx().with_w(1.0))
                + (self.group9().yzxx() * right_anti_dual_g1_xyz.zxy().with_w(right_anti_dual_g0_xyz[0]))
                + (Simd32x3::from(right_anti_dual_g3[3]) * self.group4()).with_w(right_anti_dual_g0_xyz[2] * self[e4125])
                - (Simd32x4::from(right_anti_dual_g2_w) * self.group3())
                - (right_anti_dual_g3.yzxx() * self.group5().zxy().with_w(self[e41]))
                - (right_anti_dual_g0_xyz * Simd32x3::from(self[e3215])).with_w(right_anti_dual_g3[1] * self[e42])
                - (right_anti_dual_g1_xyz.yzx() * self.group9().zxy()).with_w(right_anti_dual_g3[2] * self[e43]),
            // e5
            (right_anti_dual_g3[0] * self[e15]) + (right_anti_dual_g3[1] * self[e25]) + (right_anti_dual_g3[2] * self[e35]) + (right_anti_dual_g3[3] * self[e45])
                - (right_anti_dual_g1_w * self[e3215])
                - (right_anti_dual_g2_xyz[0] * self[e4235])
                - (right_anti_dual_g2_xyz[1] * self[e4315])
                - (right_anti_dual_g2_xyz[2] * self[e4125]),
            // e15, e25, e35, e45
            (Simd32x4::from(self[e12345]) * right_anti_dual_g2_xyz.with_w(right_anti_dual_g1_w))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g3[1] * self[e425]) - (right_anti_dual_g3[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g3[3]) * self.group6().xyz()).with_w(0.0)
                + (self.group8().yzx() * right_anti_dual_g3.zxy()).with_w(0.0)
                - (right_anti_dual_g3.yzxx() * self.group8().zxy().with_w(self[e415])),
            // e41, e42, e43
            (right_anti_dual_g0_xyz * Simd32x3::from(self[e12345]))
                + (Simd32x3::from(right_anti_dual_g2_w) * self.group6().xyz())
                + (self.group7().zxy() * right_anti_dual_g3.yzx())
                - (self.group7().yzx() * right_anti_dual_g3.zxy()),
            // e23, e31, e12
            (right_anti_dual_g1_xyz * Simd32x3::from(self[e12345]))
                + (Simd32x3::from(right_anti_dual_g2_w) * self.group8())
                + (Simd32x3::from(right_anti_dual_g3[3]) * self.group7())
                - (Simd32x3::from(self[e321]) * right_anti_dual_g3.xyz()),
            // e415, e425, e435, e321
            (right_anti_dual_g3.yzxw() * self.group9().zxy().with_w(self[e1234]))
                - (Simd32x4::from([right_anti_dual_g3[2], right_anti_dual_g3[0], right_anti_dual_g3[1], right_anti_dual_g2_w]) * self.group9().yzxw()),
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_anti_dual_g3.xyz()) - (Simd32x3::from(right_anti_dual_g2_w) * self.group9().xyz()),
            // e235, e315, e125
            (Simd32x3::from(right_anti_dual_g3[3]) * self.group9().xyz()) - (Simd32x3::from(self[e3215]) * right_anti_dual_g3.xyz()),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g3 * Simd32x4::from(self[e12345]),
            // e1234
            right_anti_dual_g2_w * self[e12345],
        )
    }
}
impl WeightContraction<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       24       34        0        0
    //    simd2        0        2        0      N/A
    //    simd3        9       17        0      N/A
    //    simd4       13       11        0      N/A
    // Totals...
    // yes simd       46       64        0      N/A
    //  no simd      103      133        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_anti_dual_g2_xyz = other.group2().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g3 = (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_anti_dual_g3[0] * self[e4235])
                    + (right_anti_dual_g3[1] * self[e4315])
                    + (right_anti_dual_g3[2] * self[e4125])
                    + (right_anti_dual_g3[3] * self[e3215])
                    + (self[scalar] * other[scalar])
                    + (self[e15] * other[e41])
                    + (self[e25] * other[e42])
                    + (self[e35] * other[e43])
                    + (self[e1234] * other[e3215])
                    - (right_anti_dual_g2_xyz[0] * self[e41])
                    - (right_anti_dual_g2_xyz[1] * self[e42])
                    - (right_anti_dual_g2_xyz[2] * self[e43])
                    - (right_anti_dual_g1[0] * self[e23])
                    - (right_anti_dual_g1[1] * self[e31])
                    - (right_anti_dual_g1[2] * self[e12])
                    - (right_anti_dual_g1[3] * self[e45]),
                self[e12345] * other[scalar],
            ]),
            // e1, e2, e3, e4
            (right_anti_dual_g3 * Simd32x4::from(self[e12345]))
                + (Simd32x4::from(other[scalar]) * self.group1())
                + (Simd32x4::from([right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g1[2], other[e41]]) * self.group6().wwwx())
                + (self.group6().xyzy() * Simd32x3::from(right_anti_dual_g1[3]).with_w(other[e42]))
                + (other.group0().yzxz() * self.group8().zxy().with_w(self[e435]))
                + Simd32x3::from(0.0).with_w((right_anti_dual_g1[2] * self[e412]) * -1.0)
                + (right_anti_dual_g2_xyz.yzx() * self.group7().zxy()).with_w(0.0)
                - (right_anti_dual_g2_xyz.zxy() * self.group7().yzx()).with_w(right_anti_dual_g1[0] * self[e423])
                - (self.group8().yzx() * other.group0().zxy()).with_w(right_anti_dual_g1[1] * self[e431]),
            // e5
            (self[e12345] * other[e3215]) + (self[e5] * other[scalar])
                - (right_anti_dual_g2_xyz[0] * self[e415])
                - (right_anti_dual_g2_xyz[1] * self[e425])
                - (right_anti_dual_g2_xyz[2] * self[e435])
                - (right_anti_dual_g1[0] * self[e235])
                - (right_anti_dual_g1[1] * self[e315])
                - (right_anti_dual_g1[2] * self[e125]),
            // e15, e25, e35, e45
            (Simd32x4::from(other[scalar]) * self.group3())
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g1[1] * self[e4315]) - (right_anti_dual_g1[2] * self[e4125]))
                + (Simd32x3::from(self[e3215]) * right_anti_dual_g1.xyz()).with_w(0.0)
                + (right_anti_dual_g2_xyz.yzx() * self.group9().zxy()).with_w(0.0)
                - (self.group9().yzxx() * right_anti_dual_g2_xyz.zxy().with_w(right_anti_dual_g1[0])),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_anti_dual_g1.xyz())
                + (Simd32x3::from(other[scalar]) * self.group4())
                + Simd32x2::from(0.0).with_z((self[e4315] * other[e41]) - (self[e4235] * other[e42]))
                + (self.group9().zx() * other.group0().yz()).with_z(0.0)
                - (self.group9().yz() * other.group0().zx()).with_z(0.0),
            // e23, e31, e12
            (right_anti_dual_g2_xyz * Simd32x3::from(self[e1234])) + (Simd32x3::from(other[scalar]) * self.group5())
                - (Simd32x3::from(right_anti_dual_g1[3]) * self.group9().xyz())
                - (Simd32x3::from(self[e3215]) * other.group0().xyz()),
            // e415, e425, e435, e321
            (right_anti_dual_g1 * Simd32x4::from(self[e12345])) + (Simd32x4::from(other[scalar]) * self.group6()),
            // e423, e431, e412
            (Simd32x3::from(other[scalar]) * self.group7()) - (Simd32x3::from(self[e12345]) * other.group0().xyz()),
            // e235, e315, e125
            (right_anti_dual_g2_xyz * Simd32x3::from(self[e12345])) + (Simd32x3::from(other[scalar]) * self.group8()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group9(),
            // e1234
            self[e1234] * other[scalar],
        )
    }
}
impl std::ops::Div<WeightContractionInfix> for Plane {
    type Output = WeightContractionInfixPartial<Plane>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for Plane {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        1        7        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        6       11        0      N/A
    //  no simd       20       31        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            (right_anti_dual_g0.zxy() * self.group0().yzx()) - (right_anti_dual_g0.yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            (self.group0().wwwx() * right_anti_dual_g0.with_w(other[e23])) + Simd32x3::from(0.0).with_w((other[e31] * self[e4315]) + (other[e12] * self[e4125]))
                - (Simd32x3::from(other[e45]) * self.group0().xyz()).with_w(0.0),
            // e15, e25, e35, e1234
            (other.group2().zxy() * self.group0().yzx()).with_w(0.0)
                - (Simd32x3::from(self[e3215]) * other.group1().xyz()).with_w(0.0)
                - (other.group2().yzx() * self.group0().zxy()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group0(),
        )
    }
}
impl WeightContraction<AntiDipoleInversion> for Plane {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        9        0        0
    //    simd3        1        4        0      N/A
    //    simd4        5        4        0      N/A
    // Totals...
    // yes simd        7       17        0      N/A
    //  no simd       24       37        0        0
    fn weight_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_anti_dual_g2_w * -1.0) * self.group0().xyz(),
            // e415, e425, e435, e321
            ((right_anti_dual_g3_xyz.yzx() * self.group0().zxy()) - (right_anti_dual_g3_xyz.zxy() * self.group0().yzx())).with_w(right_anti_dual_g2_w * self[e3215] * -1.0),
            // e235, e315, e125, e4
            (self.group0().xyzx() * Simd32x3::from(other[e5] * -1.0).with_w(other[e423])) + Simd32x3::from(0.0).with_w((other[e431] * self[e4315]) + (other[e412] * self[e4125]))
                - (right_anti_dual_g3_xyz * Simd32x3::from(self[e3215])).with_w(0.0),
            // e1, e2, e3, e5
            (self.group0().yzxw() * right_anti_dual_g1_xyz.zxy().with_w(other[e321])) + Simd32x3::from(0.0).with_w((right_anti_dual_g2_xyz[2] * self[e4125]) * -1.0)
                - (self.group0().zxyx() * right_anti_dual_g1_xyz.yzx().with_w(right_anti_dual_g2_xyz[0]))
                - (self.group0().wwwy() * other.group0().with_w(right_anti_dual_g2_xyz[1])),
        )
    }
}
impl WeightContraction<AntiDualNum> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl WeightContraction<AntiFlatPoint> for Plane {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (other[e321] * self[e3215]) - (right_anti_dual_g0_xyz[0] * self[e4235]) - (right_anti_dual_g0_xyz[1] * self[e4315]) - (right_anti_dual_g0_xyz[2] * self[e4125]),
            0.0,
        ]))
    }
}
impl WeightContraction<AntiFlector> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd       12       17        0        0
    fn weight_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1_xyz = other.group1().xyz();
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((right_anti_dual_g1_xyz.yzx() * self.group0().zxy()) - (right_anti_dual_g1_xyz.zxy() * self.group0().yzx())).with_w(0.0),
            // e235, e315, e125, e5
            (self.group0() * Simd32x3::from(other[e5] * -1.0).with_w(other[e321]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g0_xyz[1] * self[e4315]) - (right_anti_dual_g0_xyz[2] * self[e4125]))
                - (self.group0().wwwx() * right_anti_dual_g1_xyz.with_w(right_anti_dual_g0_xyz[0])),
        )
    }
}
impl WeightContraction<AntiLine> for Plane {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        4        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd       13       18        0        0
    fn weight_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e4315]) - (right_anti_dual_g0[2] * self[e4125]))
                + (right_anti_dual_g0 * Simd32x3::from(self[e3215])).with_w(0.0)
                + (right_anti_dual_g1.yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * right_anti_dual_g1.zxy().with_w(right_anti_dual_g0[0])),
        )
    }
}
impl WeightContraction<AntiMotor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       13       16        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            (self.group0().yzxx() * other.group1().zxy().with_w(other[e23])) + Simd32x3::from(0.0).with_w((other[e31] * self[e4315]) + (other[e12] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0().xyz()).with_w(0.0)
                - (other.group1().yzx() * self.group0().zxy()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group0(),
        )
    }
}
impl WeightContraction<AntiPlane> for Plane {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        2        4        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        6       13        0        0
    fn weight_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Line::from_groups(
            // e415, e425, e435
            (right_anti_dual_g0_xyz.yzx() * self.group0().zxy()) - (right_anti_dual_g0_xyz.zxy() * self.group0().yzx()),
            // e235, e315, e125
            (Simd32x3::from(other[e5] * -1.0) * self.group0().xyz()) - (right_anti_dual_g0_xyz * Simd32x3::from(self[e3215])),
        )
    }
}
impl WeightContraction<Circle> for Plane {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        7        9        0      N/A
    //  no simd       16       16        0        0
    fn weight_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (self.group0().yzxx() * right_anti_dual_g1_xyz.zxy().with_w(other[e423])) + Simd32x3::from(0.0).with_w((other[e431] * self[e4315]) + (other[e412] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (right_anti_dual_g1_xyz.yzx() * self.group0().zxy()).with_w(0.0),
            // e5
            (other[e321] * self[e3215]) - (other[e235] * self[e4235]) - (other[e315] * self[e4315]) - (other[e125] * self[e4125]),
        )
    }
}
impl WeightContraction<CircleRotor> for Plane {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        7        9        0      N/A
    //  no simd       16       16        0        0
    fn weight_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (self.group0().yzxx() * right_anti_dual_g1_xyz.zxy().with_w(other[e423])) + Simd32x3::from(0.0).with_w((other[e431] * self[e4315]) + (other[e412] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (right_anti_dual_g1_xyz.yzx() * self.group0().zxy()).with_w(0.0),
            // e5
            (other[e321] * self[e3215]) - (right_anti_dual_g2_xyz[0] * self[e4235]) - (right_anti_dual_g2_xyz[1] * self[e4315]) - (right_anti_dual_g2_xyz[2] * self[e4125]),
        )
    }
}
impl WeightContraction<Dipole> for Plane {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        3        8        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        6       11        0      N/A
    //  no simd       18       30        0        0
    fn weight_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        Dipole::from_groups(
            // e41, e42, e43
            (right_anti_dual_g0.zxy() * self.group0().yzx()) - (right_anti_dual_g0.yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            (self.group0().wwwx() * right_anti_dual_g0.with_w(other[e23])) + Simd32x3::from(0.0).with_w((other[e31] * self[e4315]) + (other[e12] * self[e4125]))
                - (Simd32x3::from(other[e45]) * self.group0().xyz()).with_w(0.0),
            // e15, e25, e35
            (right_anti_dual_g2.yzx() * self.group0().zxy()) - (Simd32x3::from(self[e3215]) * other.group1().xyz()) - (right_anti_dual_g2.zxy() * self.group0().yzx()),
        )
    }
}
impl WeightContraction<DipoleInversion> for Plane {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd3        1        4        0      N/A
    //    simd4        5        4        0      N/A
    // Totals...
    // yes simd        7       12        0      N/A
    //  no simd       24       32        0        0
    fn weight_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (right_anti_dual_g0.zxy() * self.group0().yzx()) - (right_anti_dual_g0.yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            (self.group0().wwwx() * right_anti_dual_g0.with_w(other[e23])) + Simd32x3::from(0.0).with_w((other[e31] * self[e4315]) + (other[e12] * self[e4125]))
                - (Simd32x3::from(other[e45]) * self.group0().xyz()).with_w(0.0),
            // e15, e25, e35, scalar
            (other.group2().zxyw() * self.group0().yzxw()) + Simd32x3::from(0.0).with_w((other[e4125] * self[e4125]) * -1.0)
                - (self.group0().zxyy() * other.group2().yzx().with_w(other[e4315]))
                - (self.group0().wwwx() * other.group1().xyz().with_w(other[e4235])),
        )
    }
}
impl WeightContraction<DualNum> for Plane {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        4        0        0
    fn weight_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ (Simd32x3::from(other[e5] * -1.0) * self.group0().xyz()).with_w(0.0))
    }
}
impl WeightContraction<FlatPoint> for Plane {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        2        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        6       11        0        0
    fn weight_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(other[e45] * -1.0) * self.group0().xyz(),
            // e15, e25, e35
            (other.group0().zxy() * self.group0().yzx()) + Simd32x2::from(0.0).with_z((other[e15] * self[e4315]) * -1.0) - (other.group0().yz() * self.group0().zx()).with_z(0.0),
        )
    }
}
impl WeightContraction<Flector> for Plane {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        5       13        0        0
    fn weight_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(other[e45] * -1.0) * self.group0().xyz()).with_w(-(other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125])),
            // e15, e25, e35, e3215
            ((other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl WeightContraction<Line> for Plane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        3        4        0      N/A
    //  no simd        9        9        0        0
    fn weight_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(other[e315] * self[e4315]) - (other[e125] * self[e4125])) + (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
                - (Simd32x4::from([other[e425], other[e435], other[e415], other[e235]]) * self.group0().zxyx()),
        )
    }
}
impl WeightContraction<Motor> for Plane {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        9       13        0        0
    fn weight_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1_xyz = other.group1().xyz();
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (Simd32x3::from(other[e5] * -1.0) * self.group0().xyz()).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g1_xyz[1] * self[e4315]) - (right_anti_dual_g1_xyz[2] * self[e4125]))
                + (right_anti_dual_g0_xyz.zxy() * self.group0().yzx()).with_w(0.0)
                - (self.group0().zxyx() * right_anti_dual_g0_xyz.yzx().with_w(right_anti_dual_g1_xyz[0])),
        )
    }
}
impl WeightContraction<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       17        0        0
    //    simd3        4       17        0      N/A
    //    simd4        6        3        0      N/A
    // Totals...
    // yes simd       18       37        0      N/A
    //  no simd       44       80        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g5 = other.group6().xyz();
        let right_anti_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_anti_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_anti_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g9_xyz = other.group1().xyz();
        let right_anti_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_anti_dual_g1_xyz[0] * self[e4235]) + (right_anti_dual_g1_xyz[1] * self[e4315]) + (right_anti_dual_g1_xyz[2] * self[e4125]) + (other[e1234] * self[e3215]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (self.group0().yzxx() * right_anti_dual_g5.zxy().with_w(other[e423])) + Simd32x3::from(0.0).with_w((other[e431] * self[e4315]) + (other[e412] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group7()).with_w(0.0)
                - (right_anti_dual_g5.yzx() * self.group0().zxy()).with_w(0.0),
            // e5
            (other[e321] * self[e3215]) - (other[e235] * self[e4235]) - (other[e315] * self[e4315]) - (other[e125] * self[e4125]),
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g6_xyz[1] * self[e4315]) - (right_anti_dual_g6_xyz[2] * self[e4125]))
                + (right_anti_dual_g6_xyz * Simd32x3::from(self[e3215])).with_w(0.0)
                + (right_anti_dual_g8.yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * right_anti_dual_g8.zxy().with_w(right_anti_dual_g6_xyz[0])),
            // e41, e42, e43
            (right_anti_dual_g7.zxy() * self.group0().yzx()) - (right_anti_dual_g7.yzx() * self.group0().zxy()),
            // e23, e31, e12
            (right_anti_dual_g7 * Simd32x3::from(self[e3215])) - (Simd32x3::from(other[e45]) * self.group0().xyz()),
            // e415, e425, e435, e321
            ((right_anti_dual_g9_xyz.yzx() * self.group0().zxy()) - (right_anti_dual_g9_xyz.zxy() * self.group0().yzx())).with_w(right_anti_dual_g10 * self[e3215] * -1.0),
            // e423, e431, e412
            Simd32x3::from(right_anti_dual_g10 * -1.0) * self.group0().xyz(),
            // e235, e315, e125
            (Simd32x3::from(other[e5] * -1.0) * self.group0().xyz()) - (right_anti_dual_g9_xyz * Simd32x3::from(self[e3215])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<Plane> for Plane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn weight_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -(other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]))
    }
}
impl WeightContraction<RoundPoint> for Plane {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        2       10        0      N/A
    //  no simd        6       20        0        0
    fn weight_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1 = other[e4] * -1.0;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_anti_dual_g1 * -1.0) * self.group0().xyz(),
            // e415, e425, e435, e321
            ((right_anti_dual_g0_xyz.yzx() * self.group0().zxy()) - (right_anti_dual_g0_xyz.zxy() * self.group0().yzx())).with_w(right_anti_dual_g1 * self[e3215] * -1.0),
            // e235, e315, e125
            (Simd32x3::from(other[e5] * -1.0) * self.group0().xyz()) - (right_anti_dual_g0_xyz * Simd32x3::from(self[e3215])),
        )
    }
}
impl WeightContraction<Scalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl WeightContraction<Sphere> for Plane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        7        0        0
    fn weight_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        Scalar::from_groups(
            // scalar
            (right_anti_dual_g0_xyz[0] * self[e4235]) + (right_anti_dual_g0_xyz[1] * self[e4315]) + (right_anti_dual_g0_xyz[2] * self[e4125]) + (self[e3215] * other[e1234]),
        )
    }
}
impl WeightContraction<VersorEven> for Plane {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        9        0        0
    //    simd3        1        4        0      N/A
    //    simd4        5        4        0      N/A
    // Totals...
    // yes simd        7       17        0      N/A
    //  no simd       24       37        0        0
    fn weight_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_anti_dual_g2_w * -1.0) * self.group0().xyz(),
            // e415, e425, e435, e321
            ((right_anti_dual_g3_xyz.yzx() * self.group0().zxy()) - (right_anti_dual_g3_xyz.zxy() * self.group0().yzx())).with_w(right_anti_dual_g2_w * self[e3215] * -1.0),
            // e235, e315, e125, e4
            (self.group0().xyzx() * Simd32x3::from(other[e5] * -1.0).with_w(right_anti_dual_g0_xyz[0]))
                + Simd32x3::from(0.0).with_w((right_anti_dual_g0_xyz[1] * self[e4315]) + (right_anti_dual_g0_xyz[2] * self[e4125]))
                - (right_anti_dual_g3_xyz * Simd32x3::from(self[e3215])).with_w(0.0),
            // e1, e2, e3, e5
            (self.group0().yzxw() * right_anti_dual_g1_xyz.zxy().with_w(other[e321])) + Simd32x3::from(0.0).with_w((right_anti_dual_g2_xyz[2] * self[e4125]) * -1.0)
                - (self.group0().zxyy() * right_anti_dual_g1_xyz.yzx().with_w(right_anti_dual_g2_xyz[1]))
                - (self.group0().wwwx() * right_anti_dual_g0_xyz.with_w(right_anti_dual_g2_xyz[0])),
        )
    }
}
impl WeightContraction<VersorOdd> for Plane {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       12        0        0
    //    simd3        0        6        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd       11       20        0      N/A
    //  no simd       23       38        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2 = (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(other[e3215]);
        let right_anti_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0().zxyx() * other.group0().yzx().with_w(right_anti_dual_g3_xyz[0]))
                + Simd32x3::from(0.0).with_w((right_anti_dual_g3_xyz[1] * self[e4315]) + (right_anti_dual_g3_xyz[2] * self[e4125]) + (self[e3215] * other[e1234]))
                - (self.group0().yzx() * other.group0().zxy()).with_w(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([
                -(self[e4235] * other[e45]) - (self[e3215] * other[e41]),
                -(self[e4315] * other[e45]) - (self[e3215] * other[e42]),
                -(self[e4125] * other[e45]) - (self[e3215] * other[e43]),
                (self[e4235] * other[e23]) + (self[e4315] * other[e31]) + (self[e4125] * other[e12]),
            ]),
            // e15, e25, e35, e1234
            (right_anti_dual_g2.yzx() * self.group0().zxy()).with_w(0.0)
                - (Simd32x3::from(self[e3215]) * other.group1().xyz()).with_w(0.0)
                - (right_anti_dual_g2.zxy() * self.group0().yzx()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group0(),
        )
    }
}
impl std::ops::Div<WeightContractionInfix> for RoundPoint {
    type Output = WeightContractionInfixPartial<RoundPoint>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar]) * self.group0(), /* e5 */ other[scalar] * self[e5])
    }
}
impl WeightContraction<AntiDipoleInversion> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        5        0        0
    fn weight_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        Scalar::from_groups(
            // scalar
            (right_anti_dual_g3_xyz[0] * self[e1]) + (right_anti_dual_g3_xyz[1] * self[e2]) + (right_anti_dual_g3_xyz[2] * self[e3])
                - (other[e4] * self[e5])
                - (other[e5] * self[e4]),
        )
    }
}
impl WeightContraction<AntiDualNum> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar]) * self.group0(), /* e5 */ other[scalar] * self[e5])
    }
}
impl WeightContraction<AntiFlector> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        Scalar::from_groups(
            // scalar
            (right_anti_dual_g1_xyz[0] * self[e1]) + (right_anti_dual_g1_xyz[1] * self[e2]) + (right_anti_dual_g1_xyz[2] * self[e3]) - (other[e5] * self[e4]),
        )
    }
}
impl WeightContraction<AntiMotor> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar]) * self.group0(), /* e5 */ other[scalar] * self[e5])
    }
}
impl WeightContraction<AntiPlane> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Scalar::from_groups(
            // scalar
            (right_anti_dual_g0_xyz[0] * self[e1]) + (right_anti_dual_g0_xyz[1] * self[e2]) + (right_anti_dual_g0_xyz[2] * self[e3]) - (other[e5] * self[e4]),
        )
    }
}
impl WeightContraction<DualNum> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e5] * self[e4] * -1.0)
    }
}
impl WeightContraction<Motor> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e5] * self[e4] * -1.0)
    }
}
impl WeightContraction<MultiVector> for RoundPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd2        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        4       12        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g9_xyz = other.group1().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_anti_dual_g9_xyz[0] * self[e1]) + (right_anti_dual_g9_xyz[1] * self[e2]) + (right_anti_dual_g9_xyz[2] * self[e3])
                    - (other[e4] * self[e5])
                    - (other[e5] * self[e4]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(right_anti_dual_g0[1]) * self.group0(),
            // e5
            right_anti_dual_g0[1] * self[e5],
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
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightContraction<RoundPoint> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        5        0        0
    fn weight_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Scalar::from_groups(
            // scalar
            (right_anti_dual_g0_xyz[0] * self[e1]) + (right_anti_dual_g0_xyz[1] * self[e2]) + (right_anti_dual_g0_xyz[2] * self[e3])
                - (other[e4] * self[e5])
                - (other[e5] * self[e4]),
        )
    }
}
impl WeightContraction<Scalar> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar]) * self.group0(), /* e5 */ self[e5] * other[scalar])
    }
}
impl WeightContraction<VersorEven> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        5        0        0
    fn weight_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        Scalar::from_groups(
            // scalar
            (right_anti_dual_g3_xyz[0] * self[e1]) + (right_anti_dual_g3_xyz[1] * self[e2]) + (right_anti_dual_g3_xyz[2] * self[e3])
                - (self[e4] * other[e5])
                - (self[e5] * other[e4]),
        )
    }
}
impl WeightContraction<VersorOdd> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar]) * self.group0(), /* e5 */ self[e5] * other[scalar])
    }
}
impl std::ops::Div<WeightContractionInfix> for Scalar {
    type Output = WeightContractionInfixPartial<Scalar>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[scalar] * self[scalar])
    }
}
impl WeightContraction<AntiDualNum> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[scalar] * self[scalar])
    }
}
impl WeightContraction<AntiMotor> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[scalar] * self[scalar])
    }
}
impl WeightContraction<MultiVector> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[scalar] * self[scalar])
    }
}
impl WeightContraction<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[scalar] * self[scalar])
    }
}
impl WeightContraction<VersorOdd> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * other[scalar])
    }
}
impl std::ops::Div<WeightContractionInfix> for Sphere {
    type Output = WeightContractionInfixPartial<Sphere>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for Sphere {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        5        0        0
    //    simd2        0        1        0      N/A
    //    simd3        5        8        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        9       16        0      N/A
    //  no simd       28       39        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            (right_anti_dual_g0.zxy() * self.group0().yzx()) - (Simd32x3::from(self[e1234]) * other.group1().xyz()) - (right_anti_dual_g0.yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            (self.group0().wwwx() * right_anti_dual_g0.with_w(other[e23])) + Simd32x3::from(0.0).with_w((other[e31] * self[e4315]) + (other[e12] * self[e4125]))
                - (Simd32x3::from(other[e45]) * self.group0().xyz()).with_w(0.0)
                - (Simd32x3::from(self[e1234]) * other.group2().xyz()).with_w(0.0),
            // e15, e25, e35, e1234
            ((other.group2().zxy() * self.group0().yzx()) + Simd32x2::from(0.0).with_z(other[e15] * self[e4315] * -1.0)
                - (Simd32x3::from(self[e3215]) * other.group1().xyz())
                - (other.group2().yz() * self.group0().zx()).with_z(0.0))
            .with_w(other[scalar] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group0(),
        )
    }
}
impl WeightContraction<AntiDipoleInversion> for Sphere {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        9        0        0
    //    simd3        1        6        0      N/A
    //    simd4        7        4        0      N/A
    // Totals...
    // yes simd       11       19        0      N/A
    //  no simd       34       43        0        0
    fn weight_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        let right_anti_dual_g3_w = other[e5] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (right_anti_dual_g3_xyz * Simd32x3::from(self[e1234])) - (Simd32x3::from(right_anti_dual_g2_w) * self.group0().xyz()),
            // e415, e425, e435, e321
            (right_anti_dual_g3_xyz.yzx() * self.group0().zxy()).with_w(right_anti_dual_g3_w * self[e1234])
                - (self.group0().yzxw() * right_anti_dual_g3_xyz.zxy().with_w(right_anti_dual_g2_w)),
            // e235, e315, e125, e4
            (self.group0().xyzx() * Simd32x3::from(right_anti_dual_g3_w).with_w(other[e423]))
                + Simd32x3::from(0.0).with_w((right_anti_dual_g1_w * self[e1234]) + (other[e431] * self[e4315]) + (other[e412] * self[e4125]))
                - (right_anti_dual_g3_xyz * Simd32x3::from(self[e3215])).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g2_xyz[1] * self[e4315]) - (right_anti_dual_g2_xyz[2] * self[e4125]))
                + (right_anti_dual_g2_xyz * Simd32x3::from(self[e1234])).with_w(0.0)
                + (right_anti_dual_g1_xyz.zxy() * self.group0().yzx()).with_w(0.0)
                - (Simd32x4::from(self[e3215]) * other.group0().with_w(right_anti_dual_g1_w))
                - (self.group0().zxyx() * right_anti_dual_g1_xyz.yzx().with_w(right_anti_dual_g2_xyz[0])),
        )
    }
}
impl WeightContraction<AntiDualNum> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(other[e3215] * self[e1234]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(other[scalar] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group0(),
        )
    }
}
impl WeightContraction<AntiFlatPoint> for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3        9        0        0
    fn weight_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e321] * -1.0);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            right_anti_dual_g0 * Simd32x4::from(self[e1234]),
            // e5
            -(right_anti_dual_g0[0] * self[e4235]) - (right_anti_dual_g0[1] * self[e4315]) - (right_anti_dual_g0[2] * self[e4125]) - (right_anti_dual_g0[3] * self[e3215]),
        )
    }
}
impl WeightContraction<AntiFlector> for Sphere {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd3        2        5        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        5       14        0      N/A
    //  no simd        9       27        0        0
    fn weight_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e321] * -1.0;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e5] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            right_anti_dual_g1_xyz * Simd32x3::from(self[e1234]),
            // e415, e425, e435, e321
            ((right_anti_dual_g1_xyz.yzx() * self.group0().zxy()) - (right_anti_dual_g1_xyz.zxy() * self.group0().yzx())).with_w(right_anti_dual_g1_w * self[e1234]),
            // e235, e315, e125, e4
            ((Simd32x3::from(right_anti_dual_g1_w) * self.group0().xyz()) - (right_anti_dual_g1_xyz * Simd32x3::from(self[e3215]))).with_w(right_anti_dual_g0_w * self[e1234]),
            // e1, e2, e3, e5
            Simd32x4::from([
                self[e1234],
                self[e1234],
                self[e1234],
                -(right_anti_dual_g0_w * self[e3215])
                    - (right_anti_dual_g0_xyz[0] * self[e4235])
                    - (right_anti_dual_g0_xyz[1] * self[e4315])
                    - (right_anti_dual_g0_xyz[2] * self[e4125]),
            ]) * right_anti_dual_g0_xyz.with_w(1.0),
        )
    }
}
impl WeightContraction<AntiLine> for Sphere {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        2        6        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd        8       25        0        0
    fn weight_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        Dipole::from_groups(
            // e41, e42, e43
            right_anti_dual_g0 * Simd32x3::from(self[e1234]),
            // e23, e31, e12, e45
            Simd32x4::from([
                self[e1234],
                self[e1234],
                self[e1234],
                -(right_anti_dual_g0[0] * self[e4235]) - (right_anti_dual_g0[1] * self[e4315]) - (right_anti_dual_g0[2] * self[e4125]),
            ]) * right_anti_dual_g1.with_w(1.0),
            // e15, e25, e35
            (right_anti_dual_g0 * Simd32x3::from(self[e3215])) + (right_anti_dual_g1.yzx() * self.group0().zxy()) - (right_anti_dual_g1.zxy() * self.group0().yzx()),
        )
    }
}
impl WeightContraction<AntiMotor> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        4        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        5       14        0      N/A
    //  no simd       11       32        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(self[e1234]) * (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(other[e3215]),
            // e23, e31, e12, e45
            Simd32x4::from([
                self[e1234],
                self[e1234],
                self[e1234],
                (other[e23] * self[e4235]) + (other[e31] * self[e4315]) + (other[e12] * self[e4125]),
            ]) * (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(1.0),
            // e15, e25, e35, e1234
            ((other.group1().zxy() * self.group0().yzx()) + Simd32x2::from(0.0).with_z(other[e15] * self[e4315] * -1.0)
                - (Simd32x3::from(self[e3215]) * other.group0().xyz())
                - (other.group1().yz() * self.group0().zx()).with_z(0.0))
            .with_w(other[scalar] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group0(),
        )
    }
}
impl WeightContraction<AntiPlane> for Sphere {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        6       17        0        0
    fn weight_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e5] * -1.0;
        Circle::from_groups(
            // e423, e431, e412
            right_anti_dual_g0_xyz * Simd32x3::from(self[e1234]),
            // e415, e425, e435, e321
            ((right_anti_dual_g0_xyz.yzx() * self.group0().zxy()) - (right_anti_dual_g0_xyz.zxy() * self.group0().yzx())).with_w(right_anti_dual_g0_w * self[e1234]),
            // e235, e315, e125
            (Simd32x3::from(right_anti_dual_g0_w) * self.group0().xyz()) - (right_anti_dual_g0_xyz * Simd32x3::from(self[e3215])),
        )
    }
}
impl WeightContraction<Circle> for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        7        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       20       21        0        0
    fn weight_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * other.group2().with_w(right_anti_dual_g1_w))
                + (self.group0().yzxx() * right_anti_dual_g1_xyz.zxy().with_w(other[e423]))
                + Simd32x3::from(0.0).with_w((other[e431] * self[e4315]) + (other[e412] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (right_anti_dual_g1_xyz.yzx() * self.group0().zxy()).with_w(0.0),
            // e5
            -(right_anti_dual_g1_w * self[e3215]) - (other[e235] * self[e4235]) - (other[e315] * self[e4315]) - (other[e125] * self[e4125]),
        )
    }
}
impl WeightContraction<CircleRotor> for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        7        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       20       21        0        0
    fn weight_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * right_anti_dual_g2_xyz.with_w(right_anti_dual_g1_w))
                + (self.group0().yzxx() * right_anti_dual_g1_xyz.zxy().with_w(other[e423]))
                + Simd32x3::from(0.0).with_w((other[e431] * self[e4315]) + (other[e412] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (right_anti_dual_g1_xyz.yzx() * self.group0().zxy()).with_w(0.0),
            // e5
            -(right_anti_dual_g1_w * self[e3215])
                - (right_anti_dual_g2_xyz[0] * self[e4235])
                - (right_anti_dual_g2_xyz[1] * self[e4315])
                - (right_anti_dual_g2_xyz[2] * self[e4125]),
        )
    }
}
impl WeightContraction<Dipole> for Sphere {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        4        9        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        7       13        0      N/A
    //  no simd       24       37        0        0
    fn weight_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        Dipole::from_groups(
            // e41, e42, e43
            (right_anti_dual_g0.zxy() * self.group0().yzx()) - (Simd32x3::from(self[e1234]) * other.group1().xyz()) - (right_anti_dual_g0.yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            (Simd32x4::from([self[e1234], self[e1234], self[e1234], other[e31] * self[e4315]]) * right_anti_dual_g2.with_w(1.0))
                + (self.group0().wwwx() * right_anti_dual_g0.with_w(other[e23]))
                + Simd32x3::from(0.0).with_w(other[e12] * self[e4125])
                - (Simd32x3::from(other[e45]) * self.group0().xyz()).with_w(0.0),
            // e15, e25, e35
            (right_anti_dual_g2.yzx() * self.group0().zxy()) - (Simd32x3::from(self[e3215]) * other.group1().xyz()) - (right_anti_dual_g2.zxy() * self.group0().yzx()),
        )
    }
}
impl WeightContraction<DipoleInversion> for Sphere {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        2        6        0      N/A
    //    simd4        6        4        0      N/A
    // Totals...
    // yes simd       10       14        0      N/A
    //  no simd       32       38        0        0
    fn weight_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (right_anti_dual_g0.zxy() * self.group0().yzx()) - (Simd32x3::from(self[e1234]) * other.group1().xyz()) - (right_anti_dual_g0.yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            (self.group0().wwwx() * right_anti_dual_g0.with_w(other[e23])) + Simd32x3::from(0.0).with_w((other[e31] * self[e4315]) + (other[e12] * self[e4125]))
                - (Simd32x3::from(other[e45]) * self.group0().xyz()).with_w(0.0)
                - (Simd32x3::from(self[e1234]) * other.group2().xyz()).with_w(0.0),
            // e15, e25, e35, scalar
            (other.group2().zxyw() * self.group0().yzxw()) + Simd32x3::from(0.0).with_w((other[e3215] * self[e1234]) - (other[e4125] * self[e4125]))
                - (self.group0().zxyy() * other.group2().yzx().with_w(other[e4315]))
                - (self.group0().wwwx() * other.group1().xyz().with_w(other[e4235])),
        )
    }
}
impl WeightContraction<DualNum> for Sphere {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn weight_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other[e5] * -1.0) * self.group0().xyz().with_w(self[e1234]))
    }
}
impl WeightContraction<FlatPoint> for Sphere {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        3        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        9       13        0        0
    fn weight_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            -(Simd32x3::from(other[e45]) * self.group0().xyz()) - (Simd32x3::from(self[e1234]) * other.group0().xyz()),
            // e15, e25, e35
            (other.group0().zxy() * self.group0().yzx()) + Simd32x2::from(0.0).with_z((other[e15] * self[e4315]) * -1.0) - (other.group0().yz() * self.group0().zx()).with_z(0.0),
        )
    }
}
impl WeightContraction<Flector> for Sphere {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       13       16        0        0
    fn weight_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w((other[e3215] * self[e1234]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]))
                - (Simd32x4::from([other[e45], other[e45], other[e45], other[e4235]]) * self.group0().xyzx())
                - (Simd32x3::from(self[e1234]) * other.group0().xyz()).with_w(0.0),
            // e15, e25, e35, e3215
            ((other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl WeightContraction<Line> for Sphere {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn weight_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(other[e315] * self[e4315]) - (other[e125] * self[e4125]))
                + (Simd32x3::from(self[e1234]) * other.group1()).with_w(0.0)
                + (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
                - (Simd32x4::from([other[e425], other[e435], other[e415], other[e235]]) * self.group0().zxyx()),
        )
    }
}
impl WeightContraction<Motor> for Sphere {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd       13       17        0        0
    fn weight_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1_xyz = other.group1().xyz();
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e5] * -1.0) * self.group0().xyz().with_w(self[e1234]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g1_xyz[1] * self[e4315]) - (right_anti_dual_g1_xyz[2] * self[e4125]))
                + (right_anti_dual_g1_xyz * Simd32x3::from(self[e1234])).with_w(0.0)
                + (right_anti_dual_g0_xyz.zxy() * self.group0().yzx()).with_w(0.0)
                - (self.group0().zxyx() * right_anti_dual_g0_xyz.yzx().with_w(right_anti_dual_g1_xyz[0])),
        )
    }
}
impl WeightContraction<MultiVector> for Sphere {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       18        0        0
    //    simd2        0        1        0      N/A
    //    simd3        6       19        0      N/A
    //    simd4        8        5        0      N/A
    // Totals...
    // yes simd       23       43        0      N/A
    //  no simd       59       97        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g3 = other.group8().with_w(other[e321] * -1.0);
        let right_anti_dual_g5 = other.group6().xyz();
        let right_anti_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_anti_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_anti_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g9_xyz = other.group1().xyz();
        let right_anti_dual_g9_w = other[e5] * -1.0;
        let right_anti_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_anti_dual_g1_xyz[0] * self[e4235])
                    + (right_anti_dual_g1_xyz[1] * self[e4315])
                    + (right_anti_dual_g1_xyz[2] * self[e4125])
                    + (other[e3215] * self[e1234])
                    + (other[e1234] * self[e3215]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (right_anti_dual_g3 * Simd32x4::from(self[e1234]))
                + (self.group0().yzxx() * right_anti_dual_g5.zxy().with_w(other[e423]))
                + Simd32x3::from(0.0).with_w((other[e431] * self[e4315]) + (other[e412] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group7()).with_w(0.0)
                - (right_anti_dual_g5.yzx() * self.group0().zxy()).with_w(0.0),
            // e5
            -(right_anti_dual_g3[0] * self[e4235]) - (right_anti_dual_g3[1] * self[e4315]) - (right_anti_dual_g3[2] * self[e4125]) - (right_anti_dual_g3[3] * self[e3215]),
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g6_xyz[1] * self[e4315]) - (right_anti_dual_g6_xyz[2] * self[e4125]))
                + (right_anti_dual_g6_xyz * Simd32x3::from(self[e3215])).with_w(0.0)
                + (right_anti_dual_g8.yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * right_anti_dual_g8.zxy().with_w(right_anti_dual_g6_xyz[0])),
            // e41, e42, e43
            (right_anti_dual_g6_xyz * Simd32x3::from(self[e1234])) + (right_anti_dual_g7.zxy() * self.group0().yzx()) - (right_anti_dual_g7.yzx() * self.group0().zxy()),
            // e23, e31, e12
            (right_anti_dual_g7 * Simd32x3::from(self[e3215])) + (right_anti_dual_g8 * Simd32x3::from(self[e1234])) - (Simd32x3::from(other[e45]) * self.group0().xyz()),
            // e415, e425, e435, e321
            (right_anti_dual_g9_xyz.yzx() * self.group0().zxy()).with_w(right_anti_dual_g9_w * self[e1234])
                - (self.group0().yzxw() * right_anti_dual_g9_xyz.zxy().with_w(right_anti_dual_g10)),
            // e423, e431, e412
            (right_anti_dual_g9_xyz * Simd32x3::from(self[e1234])) - (Simd32x3::from(right_anti_dual_g10) * self.group0().xyz()),
            // e235, e315, e125
            (Simd32x3::from(right_anti_dual_g9_w) * self.group0().xyz()) - (right_anti_dual_g9_xyz * Simd32x3::from(self[e3215])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_anti_dual_g0[1]) * self.group0(),
            // e1234
            right_anti_dual_g0[1] * self[e1234],
        )
    }
}
impl WeightContraction<Plane> for Sphere {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (other[e3215] * self[e1234]) - (other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]),
        )
    }
}
impl WeightContraction<RoundPoint> for Sphere {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        2        5        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        3        9        0      N/A
    //  no simd       10       22        0        0
    fn weight_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e5] * -1.0;
        let right_anti_dual_g1 = other[e4] * -1.0;
        Circle::from_groups(
            // e423, e431, e412
            (right_anti_dual_g0_xyz * Simd32x3::from(self[e1234])) - (Simd32x3::from(right_anti_dual_g1) * self.group0().xyz()),
            // e415, e425, e435, e321
            (right_anti_dual_g0_xyz.yzx() * self.group0().zxy()).with_w(right_anti_dual_g0_w * self[e1234])
                - (self.group0().yzxw() * right_anti_dual_g0_xyz.zxy().with_w(right_anti_dual_g1)),
            // e235, e315, e125
            (Simd32x3::from(right_anti_dual_g0_w) * self.group0().xyz()) - (right_anti_dual_g0_xyz * Simd32x3::from(self[e3215])),
        )
    }
}
impl WeightContraction<Scalar> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1234
            other[scalar] * self[e1234],
        )
    }
}
impl WeightContraction<Sphere> for Sphere {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        5        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd        4        8        0        0
    fn weight_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        Scalar::from_groups(
            // scalar
            (right_anti_dual_g0_xyz[0] * self[e4235])
                + (right_anti_dual_g0_xyz[1] * self[e4315])
                + (right_anti_dual_g0_xyz[2] * self[e4125])
                + (other[e3215] * self[e1234])
                + (other[e1234] * self[e3215]),
        )
    }
}
impl WeightContraction<VersorEven> for Sphere {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        9        0        0
    //    simd3        1        6        0      N/A
    //    simd4        7        4        0      N/A
    // Totals...
    // yes simd       11       19        0      N/A
    //  no simd       34       43        0        0
    fn weight_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        let right_anti_dual_g3_w = other[e5] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (right_anti_dual_g3_xyz * Simd32x3::from(self[e1234])) - (Simd32x3::from(right_anti_dual_g2_w) * self.group0().xyz()),
            // e415, e425, e435, e321
            (right_anti_dual_g3_xyz.yzx() * self.group0().zxy()).with_w(right_anti_dual_g3_w * self[e1234])
                - (self.group0().yzxw() * right_anti_dual_g3_xyz.zxy().with_w(right_anti_dual_g2_w)),
            // e235, e315, e125, e4
            (self.group0().xyzx() * Simd32x3::from(right_anti_dual_g3_w).with_w(right_anti_dual_g0_xyz[0]))
                + Simd32x3::from(0.0).with_w((right_anti_dual_g1_w * self[e1234]) + (right_anti_dual_g0_xyz[1] * self[e4315]) + (right_anti_dual_g0_xyz[2] * self[e4125]))
                - (right_anti_dual_g3_xyz * Simd32x3::from(self[e3215])).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g2_xyz[1] * self[e4315]) - (right_anti_dual_g2_xyz[2] * self[e4125]))
                + (right_anti_dual_g2_xyz * Simd32x3::from(self[e1234])).with_w(0.0)
                + (right_anti_dual_g1_xyz.zxy() * self.group0().yzx()).with_w(0.0)
                - (Simd32x4::from(self[e3215]) * right_anti_dual_g0_xyz.with_w(right_anti_dual_g1_w))
                - (self.group0().zxyx() * right_anti_dual_g1_xyz.yzx().with_w(right_anti_dual_g2_xyz[0])),
        )
    }
}
impl WeightContraction<VersorOdd> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        2        9        0      N/A
    //    simd4        6        3        0      N/A
    // Totals...
    // yes simd       12       20        0      N/A
    //  no simd       34       47        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2_xyz = other.group2().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0().zxyx() * other.group0().yzx().with_w(right_anti_dual_g3_xyz[0]))
                + Simd32x3::from(0.0)
                    .with_w((right_anti_dual_g3_xyz[1] * self[e4315]) + (right_anti_dual_g3_xyz[2] * self[e4125]) + (self[e3215] * other[e1234]) + (self[e1234] * other[e3215]))
                - (Simd32x3::from(self[e1234]) * other.group1().xyz()).with_w(0.0)
                - (self.group0().yzx() * other.group0().zxy()).with_w(0.0),
            // e23, e31, e12, e45
            (Simd32x4::from([self[e1234], self[e1234], self[e1234], self[e4235] * other[e23]]) * right_anti_dual_g2_xyz.with_w(1.0))
                + Simd32x3::from(0.0).with_w((self[e4315] * other[e31]) + (self[e4125] * other[e12]))
                - (Simd32x3::from(self[e3215]) * other.group0().xyz()).with_w(0.0)
                - (Simd32x3::from(other[e45]) * self.group0().xyz()).with_w(0.0),
            // e15, e25, e35, e1234
            ((right_anti_dual_g2_xyz.yzx() * self.group0().zxy()) - (Simd32x3::from(self[e3215]) * other.group1().xyz()) - (right_anti_dual_g2_xyz.zxy() * self.group0().yzx()))
                .with_w(self[e1234] * other[scalar]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group0(),
        )
    }
}
impl std::ops::Div<WeightContractionInfix> for VersorEven {
    type Output = WeightContractionInfixPartial<VersorEven>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       12        0        0
    //    simd3        1        9        0      N/A
    //    simd4       10        7        0      N/A
    // Totals...
    // yes simd       19       28        0      N/A
    //  no simd       51       67        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_anti_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((right_anti_dual_g0 * Simd32x3::from(self[e12345])) + (Simd32x3::from(right_anti_dual_g2[3]) * self.group0().xyz())).with_w(right_anti_dual_g2[3] * self[e12345]),
            // e415, e425, e435, e321
            (right_anti_dual_g1 * Simd32x4::from(self[e12345])) + (Simd32x4::from(right_anti_dual_g2[3]) * self.group1()),
            // e235, e315, e125, e5
            (right_anti_dual_g2 * Simd32x3::from(self[e12345]).with_w(self[e5]))
                + Simd32x3::from(0.0).with_w(
                    -(right_anti_dual_g1[0] * self[e235])
                        - (right_anti_dual_g1[1] * self[e315])
                        - (right_anti_dual_g1[2] * self[e125])
                        - (right_anti_dual_g2[0] * self[e415])
                        - (right_anti_dual_g2[1] * self[e425])
                        - (right_anti_dual_g2[2] * self[e435]),
                )
                + (Simd32x3::from(right_anti_dual_g2[3]) * self.group2().xyz()).with_w(0.0),
            // e1, e2, e3, e4
            (Simd32x4::from(right_anti_dual_g2[3]) * self.group3())
                + Simd32x3::from(0.0).with_w(
                    -(right_anti_dual_g0[1] * self[e425]) - (right_anti_dual_g0[2] * self[e435]) - (right_anti_dual_g1[1] * self[e431]) - (right_anti_dual_g1[2] * self[e412]),
                )
                + (Simd32x3::from(right_anti_dual_g1[3]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * right_anti_dual_g1.xyz()).with_w(0.0)
                + (right_anti_dual_g0.zxy() * self.group2().yzx()).with_w(0.0)
                + (right_anti_dual_g2.yzx() * self.group0().zxy()).with_w(0.0)
                - (Simd32x4::from([right_anti_dual_g2[2], right_anti_dual_g2[0], right_anti_dual_g2[1], right_anti_dual_g1[0]]) * self.group0().yzxx())
                - (right_anti_dual_g0.yzx() * self.group2().zxy()).with_w(right_anti_dual_g0[0] * self[e415]),
        )
    }
}
impl WeightContraction<AntiDipoleInversion> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       20        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        5        0      N/A
    //    simd4        8        6        0      N/A
    // Totals...
    // yes simd       24       33        0      N/A
    //  no simd       56       63        0        0
    fn weight_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3 = other.group3().xyz().with_w(other[e5] * -1.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from(right_anti_dual_g2_w) * self.group1().xyz().with_w(self[e5]))
                + (right_anti_dual_g3.yzxx() * self.group0().zxy().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w(
                    (right_anti_dual_g3[2] * self[e3]) + (right_anti_dual_g3[3] * self[e4])
                        - (right_anti_dual_g2_xyz[1] * self[e431])
                        - (right_anti_dual_g2_xyz[2] * self[e412])
                        - (right_anti_dual_g1[0] * self[e415])
                        - (right_anti_dual_g1[1] * self[e425])
                        - (right_anti_dual_g1[2] * self[e435])
                        - (right_anti_dual_g1[3] * self[e321])
                        - (other[e423] * self[e235])
                        - (other[e431] * self[e315])
                        - (other[e412] * self[e125]),
                )
                + (Simd32x3::from(self[e12345]) * other.group0()).with_w(right_anti_dual_g3[1] * self[e2])
                - (Simd32x4::from([right_anti_dual_g3[2], right_anti_dual_g3[0], right_anti_dual_g3[1], right_anti_dual_g2_xyz[0]]) * self.group0().yzxx()),
            // e23, e31, e12, e45
            (right_anti_dual_g1 * Simd32x4::from(self[e12345]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g3[1] * self[e425]) - (right_anti_dual_g3[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g2_w) * self.group2().xyz()).with_w(0.0)
                + (Simd32x3::from(right_anti_dual_g3[3]) * self.group0().xyz()).with_w(0.0)
                - (right_anti_dual_g3.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            ((right_anti_dual_g2_xyz * Simd32x3::from(self[e12345]))
                + (Simd32x3::from(right_anti_dual_g3[3]) * self.group1().xyz())
                + Simd32x2::from(0.0).with_z((right_anti_dual_g3[1] * self[e235]) - (right_anti_dual_g3[0] * self[e315]))
                + (right_anti_dual_g3.zx() * self.group2().yz()).with_z(0.0)
                - (right_anti_dual_g3.yz() * self.group2().zx()).with_z(0.0))
            .with_w(right_anti_dual_g2_w * self[e12345]),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g3 * Simd32x4::from(self[e12345]),
        )
    }
}
impl WeightContraction<AntiDualNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        1        6        0      N/A
    //  no simd        1       17        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125, e5
            (Simd32x3::from(other[scalar]) * self.group2().xyz()).with_w((other[e3215] * self[e12345]) + (other[scalar] * self[e5])),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group3(),
        )
    }
}
impl WeightContraction<AntiFlatPoint> for VersorEven {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3        9        0        0
    fn weight_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e321] * -1.0;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(right_anti_dual_g0_w * self[e12345]),
            // e15, e25, e35, scalar
            (right_anti_dual_g0_xyz * Simd32x3::from(self[e12345])).with_w(
                -(right_anti_dual_g0_w * self[e321])
                    - (right_anti_dual_g0_xyz[0] * self[e423])
                    - (right_anti_dual_g0_xyz[1] * self[e431])
                    - (right_anti_dual_g0_xyz[2] * self[e412]),
            ),
        )
    }
}
impl WeightContraction<AntiFlector> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       10        0        0
    //    simd3        0        4        0      N/A
    //    simd4        7        5        0      N/A
    // Totals...
    // yes simd       13       19        0      N/A
    //  no simd       34       42        0        0
    fn weight_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e321] * -1.0;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e5] * -1.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (right_anti_dual_g1.yzxx() * self.group0().zxy().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w(
                    (right_anti_dual_g1[1] * self[e2]) + (right_anti_dual_g1[2] * self[e3]) + (right_anti_dual_g1[3] * self[e4])
                        - (right_anti_dual_g0_w * self[e321])
                        - (right_anti_dual_g0_xyz[1] * self[e431])
                        - (right_anti_dual_g0_xyz[2] * self[e412]),
                )
                - (Simd32x4::from([right_anti_dual_g1[2], right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g0_xyz[0]]) * self.group0().yzxx()),
            // e23, e31, e12, e45
            (self.group0() * Simd32x3::from(right_anti_dual_g1[3]).with_w(right_anti_dual_g0_w))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g1[1] * self[e425]) - (right_anti_dual_g1[2] * self[e435]))
                - (right_anti_dual_g1.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            (right_anti_dual_g0_xyz * Simd32x3::from(self[e12345])).with_w(0.0)
                + (Simd32x3::from(right_anti_dual_g1[3]) * self.group1().xyz()).with_w(0.0)
                + (right_anti_dual_g1.zxy() * self.group2().yzx()).with_w(0.0)
                - (right_anti_dual_g1.yzx() * self.group2().zxy()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g1 * Simd32x4::from(self[e12345]),
        )
    }
}
impl WeightContraction<AntiLine> for VersorEven {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        0        7        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        9       16        0      N/A
    //  no simd       18       30        0        0
    fn weight_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (right_anti_dual_g0 * Simd32x3::from(self[e12345])).with_w(0.0),
            // e235, e315, e125, e4
            (right_anti_dual_g1 * Simd32x3::from(self[e12345]))
                .with_w(-(right_anti_dual_g0[0] * self[e423]) - (right_anti_dual_g0[1] * self[e431]) - (right_anti_dual_g0[2] * self[e412])),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(
                -(right_anti_dual_g0[1] * self[e315])
                    - (right_anti_dual_g0[2] * self[e125])
                    - (right_anti_dual_g1[0] * self[e415])
                    - (right_anti_dual_g1[1] * self[e425])
                    - (right_anti_dual_g1[2] * self[e435]),
            ) + (right_anti_dual_g0 * Simd32x3::from(self[e321])).with_w(0.0)
                + (right_anti_dual_g1.yzx() * self.group0().zxy()).with_w(0.0)
                - (right_anti_dual_g1.zxy() * self.group0().yzx()).with_w(right_anti_dual_g0[0] * self[e235]),
        )
    }
}
impl WeightContraction<AntiMotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        1        4        0      N/A
    //    simd4        6        7        0      N/A
    // Totals...
    // yes simd       13       20        0      N/A
    //  no simd       33       49        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(right_anti_dual_g0[3]) * self.group0(),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz()) + (Simd32x3::from(self[e12345]) * right_anti_dual_g0.xyz())).with_w(right_anti_dual_g0[3] * self[e321]),
            // e235, e315, e125, e5
            (right_anti_dual_g1 * Simd32x4::from(self[e12345]))
                + (Simd32x4::from(right_anti_dual_g0[3]) * self.group2())
                + Simd32x3::from(0.0).with_w(
                    -(right_anti_dual_g0[0] * self[e235])
                        - (right_anti_dual_g0[1] * self[e315])
                        - (right_anti_dual_g0[2] * self[e125])
                        - (right_anti_dual_g1[0] * self[e415])
                        - (right_anti_dual_g1[1] * self[e425])
                        - (right_anti_dual_g1[2] * self[e435]),
                ),
            // e1, e2, e3, e4
            (right_anti_dual_g0 * Simd32x3::from(self[e321]).with_w(self[e4]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e431]) - (right_anti_dual_g0[2] * self[e412]))
                + (Simd32x3::from(right_anti_dual_g0[3]) * self.group3().xyz()).with_w(0.0)
                + (right_anti_dual_g1.yzx() * self.group0().zxy()).with_w(0.0)
                - (Simd32x4::from([right_anti_dual_g1[2], right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g0[0]]) * self.group0().yzxx()),
        )
    }
}
impl WeightContraction<AntiPlane> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        0        5        0      N/A
    //    simd4        6        3        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd       27       33        0        0
    fn weight_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e5] * -1.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (right_anti_dual_g0.yzxx() * self.group0().zxy().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w((right_anti_dual_g0[1] * self[e2]) + (right_anti_dual_g0[2] * self[e3]) + (right_anti_dual_g0[3] * self[e4]))
                - (right_anti_dual_g0.zxy() * self.group0().yzx()).with_w(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e425]) - (right_anti_dual_g0[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g0[3]) * self.group0().xyz()).with_w(0.0)
                - (right_anti_dual_g0.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz()).with_w(0.0) + (right_anti_dual_g0.zxy() * self.group2().yzx()).with_w(0.0)
                - (right_anti_dual_g0.yzx() * self.group2().zxy()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g0 * Simd32x4::from(self[e12345]),
        )
    }
}
impl WeightContraction<AntiScalar> for VersorEven {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_contraction(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e12345] * self[e12345] * -1.0)
    }
}
impl WeightContraction<Circle> for VersorEven {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       11        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd        9       21        0        0
    fn weight_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            right_anti_dual_g1 * Simd32x4::from(self[e12345]),
            // e15, e25, e35, scalar
            (Simd32x3::from(self[e12345]) * other.group2()).with_w(
                -(right_anti_dual_g1[0] * self[e415])
                    - (right_anti_dual_g1[1] * self[e425])
                    - (right_anti_dual_g1[2] * self[e435])
                    - (right_anti_dual_g1[3] * self[e321])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125])
                    - (other[e235] * self[e423])
                    - (other[e315] * self[e431])
                    - (other[e125] * self[e412]),
            ),
        )
    }
}
impl WeightContraction<CircleRotor> for VersorEven {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       12        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       10       15        0      N/A
    //  no simd       10       22        0        0
    fn weight_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        let right_anti_dual_g2_xyz = other.group2().xyz();
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            right_anti_dual_g1 * Simd32x4::from(self[e12345]),
            // e15, e25, e35, scalar
            (right_anti_dual_g2_xyz * Simd32x3::from(self[e12345])).with_w(
                -(right_anti_dual_g2_xyz[0] * self[e423])
                    - (right_anti_dual_g2_xyz[1] * self[e431])
                    - (right_anti_dual_g2_xyz[2] * self[e412])
                    - (right_anti_dual_g1[0] * self[e415])
                    - (right_anti_dual_g1[1] * self[e425])
                    - (right_anti_dual_g1[2] * self[e435])
                    - (right_anti_dual_g1[3] * self[e321])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125])
                    - (other[e12345] * self[e12345]),
            ),
        )
    }
}
impl WeightContraction<Dipole> for VersorEven {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       11        0        0
    //    simd3        0        9        0      N/A
    //    simd4        6        3        0      N/A
    // Totals...
    // yes simd       14       23        0      N/A
    //  no simd       32       50        0        0
    fn weight_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_anti_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            right_anti_dual_g0 * Simd32x3::from(self[e12345]),
            // e415, e425, e435, e321
            right_anti_dual_g1 * Simd32x4::from(self[e12345]),
            // e235, e315, e125, e4
            (right_anti_dual_g2 * Simd32x3::from(self[e12345])).with_w(
                -(right_anti_dual_g0[0] * self[e415])
                    - (right_anti_dual_g0[1] * self[e425])
                    - (right_anti_dual_g0[2] * self[e435])
                    - (right_anti_dual_g1[0] * self[e423])
                    - (right_anti_dual_g1[1] * self[e431])
                    - (right_anti_dual_g1[2] * self[e412]),
            ),
            // e1, e2, e3, e5
            Simd32x3::from(0.0)
                .with_w(-(right_anti_dual_g2[1] * self[e425]) - (right_anti_dual_g2[2] * self[e435]) - (right_anti_dual_g1[1] * self[e315]) - (right_anti_dual_g1[2] * self[e125]))
                + (Simd32x3::from(right_anti_dual_g1[3]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * right_anti_dual_g1.xyz()).with_w(0.0)
                + (right_anti_dual_g0.zxy() * self.group2().yzx()).with_w(0.0)
                + (right_anti_dual_g2.yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group2().zxyx() * right_anti_dual_g0.yzx().with_w(right_anti_dual_g1[0]))
                - (right_anti_dual_g2.zxy() * self.group0().yzx()).with_w(right_anti_dual_g2[0] * self[e415]),
        )
    }
}
impl WeightContraction<DipoleInversion> for VersorEven {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       11        0        0
    //    simd3        0        6        0      N/A
    //    simd4        7        6        0      N/A
    // Totals...
    // yes simd       14       23        0      N/A
    //  no simd       35       53        0        0
    fn weight_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            right_anti_dual_g0 * Simd32x3::from(self[e12345]),
            // e415, e425, e435, e321
            right_anti_dual_g1 * Simd32x4::from(self[e12345]),
            // e235, e315, e125, e4
            (Simd32x3::from(self[e12345] * -1.0) * other.group2().xyz()).with_w(
                (other[e1234] * self[e12345])
                    - (right_anti_dual_g0[0] * self[e415])
                    - (right_anti_dual_g0[1] * self[e425])
                    - (right_anti_dual_g0[2] * self[e435])
                    - (right_anti_dual_g1[0] * self[e423])
                    - (right_anti_dual_g1[1] * self[e431])
                    - (right_anti_dual_g1[2] * self[e412]),
            ),
            // e1, e2, e3, e5
            (Simd32x4::from([right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g1[2], other[e15]]) * self.group1().wwwx())
                + (other.group2().zxyz() * self.group0().yzx().with_w(self[e435]))
                + (self.group1().xyzy() * Simd32x3::from(right_anti_dual_g1[3]).with_w(other[e25]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g1[1] * self[e315]) - (right_anti_dual_g1[2] * self[e125]))
                + (right_anti_dual_g0.zxy() * self.group2().yzx()).with_w(other[e3215] * self[e12345])
                - (self.group2().zxyx() * right_anti_dual_g0.yzx().with_w(right_anti_dual_g1[0]))
                - (Simd32x3::from(self[e12345]) * other.group3().xyz()).with_w(0.0)
                - (other.group2().yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl WeightContraction<DualNum> for VersorEven {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        6        0      N/A
    //  no simd        1       11        0        0
    fn weight_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(other[e5] * -1.0) * self.group0().xyz()).with_w(-(other[e5] * self[e4]) - (other[e12345] * self[e12345])),
            // e15, e25, e35, e3215
            Simd32x4::from(other[e5] * -1.0) * self.group1().xyz().with_w(self[e12345]),
        )
    }
}
impl WeightContraction<FlatPoint> for VersorEven {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd       13       20        0        0
    fn weight_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            right_anti_dual_g0 * Simd32x4::from(self[e12345]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e425]) - (right_anti_dual_g0[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz()).with_w(0.0)
                + (right_anti_dual_g0.yzx() * self.group0().zxy()).with_w(0.0)
                - (right_anti_dual_g0.zxyx() * self.group0().yzx().with_w(self[e415])),
        )
    }
}
impl WeightContraction<Flector> for VersorEven {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        5        0      N/A
    // Totals...
    // yes simd        5        9        0      N/A
    //  no simd       17       28        0        0
    fn weight_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            right_anti_dual_g0 * Simd32x4::from(self[e12345]),
            // e1, e2, e3, e5
            (Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e425]) - (right_anti_dual_g0[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz()).with_w(0.0)
                + (right_anti_dual_g0.yzx() * self.group0().zxy()).with_w(0.0)
                - (right_anti_dual_g0.zxyx() * self.group0().yzx().with_w(self[e415])),
        )
    }
}
impl WeightContraction<Line> for VersorEven {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       12        0        0
    fn weight_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e12345]) * other.group0()).with_w(
                -(other[e415] * self[e415])
                    - (other[e425] * self[e425])
                    - (other[e435] * self[e435])
                    - (other[e235] * self[e423])
                    - (other[e315] * self[e431])
                    - (other[e125] * self[e412]),
            ),
            // e15, e25, e35, e3215
            (Simd32x3::from(self[e12345]) * other.group1()).with_w(0.0),
        )
    }
}
impl WeightContraction<Motor> for VersorEven {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       10        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        8       14        0      N/A
    //  no simd       16       23        0        0
    fn weight_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e5] * -1.0;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (self.group0() * Simd32x3::from(right_anti_dual_g1_w).with_w(other[e12345] * -1.0))
                + Simd32x3::from(0.0).with_w(
                    -(right_anti_dual_g0_xyz[0] * self[e415])
                        - (right_anti_dual_g0_xyz[1] * self[e425])
                        - (right_anti_dual_g0_xyz[2] * self[e435])
                        - (right_anti_dual_g1_xyz[0] * self[e423])
                        - (right_anti_dual_g1_xyz[1] * self[e431])
                        - (right_anti_dual_g1_xyz[2] * self[e412]),
                )
                + (right_anti_dual_g0_xyz * Simd32x3::from(self[e12345])).with_w(right_anti_dual_g1_w * self[e4]),
            // e15, e25, e35, e3215
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e12345])) + (Simd32x3::from(right_anti_dual_g1_w) * self.group1().xyz())).with_w(right_anti_dual_g1_w * self[e12345]),
        )
    }
}
impl WeightContraction<MultiVector> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       26       37        0        0
    //    simd2        0        3        0      N/A
    //    simd3        9       19        0      N/A
    //    simd4       13        9        0      N/A
    // Totals...
    // yes simd       48       68        0      N/A
    //  no simd      105      136        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
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
                (right_anti_dual_g10 * self[e5])
                    + (right_anti_dual_g0[0] * self[e12345])
                    + (right_anti_dual_g9[0] * self[e1])
                    + (right_anti_dual_g9[1] * self[e2])
                    + (right_anti_dual_g9[2] * self[e3])
                    + (right_anti_dual_g9[3] * self[e4])
                    - (right_anti_dual_g5[0] * self[e415])
                    - (right_anti_dual_g5[1] * self[e425])
                    - (right_anti_dual_g5[2] * self[e435])
                    - (right_anti_dual_g3[0] * self[e423])
                    - (right_anti_dual_g3[1] * self[e431])
                    - (right_anti_dual_g3[2] * self[e412])
                    - (right_anti_dual_g3[3] * self[e321])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
                right_anti_dual_g0[1] * self[e12345],
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(right_anti_dual_g0[1]) * self.group3())
                + (self.group0().zxyw() * right_anti_dual_g8.yzx().with_w(other[e1234]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g7[1] * self[e425]) - (right_anti_dual_g7[2] * self[e435]) - (right_anti_dual_g6[2] * self[e412]))
                + (Simd32x3::from(right_anti_dual_g6[3]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * right_anti_dual_g6.xyz()).with_w(0.0)
                + (right_anti_dual_g7.zxy() * self.group2().yzx()).with_w(0.0)
                - (self.group0().yzxx() * right_anti_dual_g8.zxy().with_w(right_anti_dual_g6[0]))
                - (self.group0().wwwy() * other.group9().xyz().with_w(right_anti_dual_g6[1]))
                - (right_anti_dual_g7.yzx() * self.group2().zxy()).with_w(right_anti_dual_g7[0] * self[e415]),
            // e5
            (right_anti_dual_g0[1] * self[e5]) + (other[e3215] * self[e12345])
                - (right_anti_dual_g8[0] * self[e415])
                - (right_anti_dual_g8[1] * self[e425])
                - (right_anti_dual_g8[2] * self[e435])
                - (right_anti_dual_g6[0] * self[e235])
                - (right_anti_dual_g6[1] * self[e315])
                - (right_anti_dual_g6[2] * self[e125]),
            // e15, e25, e35, e45
            (right_anti_dual_g3 * Simd32x4::from(self[e12345]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g9[1] * self[e425]) - (right_anti_dual_g9[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g9[3]) * self.group1().xyz()).with_w(0.0)
                + (right_anti_dual_g9.zxy() * self.group2().yzx()).with_w(0.0)
                - (right_anti_dual_g9.yzxx() * self.group2().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(right_anti_dual_g10) * self.group1().xyz())
                + (Simd32x3::from(self[e12345]) * other.group7())
                + Simd32x2::from(0.0).with_z((right_anti_dual_g9[0] * self[e431]) - (right_anti_dual_g9[1] * self[e423]))
                + (right_anti_dual_g9.yz() * self.group0().zx()).with_z(0.0)
                - (right_anti_dual_g9.zx() * self.group0().yz()).with_z(0.0),
            // e23, e31, e12
            (right_anti_dual_g5 * Simd32x3::from(self[e12345]))
                + (Simd32x3::from(right_anti_dual_g10) * self.group2().xyz())
                + (Simd32x3::from(right_anti_dual_g9[3]) * self.group0().xyz())
                - (Simd32x3::from(self[e321]) * right_anti_dual_g9.xyz()),
            // e415, e425, e435, e321
            (right_anti_dual_g6 * Simd32x4::from(self[e12345])) + (Simd32x4::from(right_anti_dual_g0[1]) * self.group1()),
            // e423, e431, e412
            (right_anti_dual_g7 * Simd32x3::from(self[e12345])) + (Simd32x3::from(right_anti_dual_g0[1]) * self.group0().xyz()),
            // e235, e315, e125
            (right_anti_dual_g8 * Simd32x3::from(self[e12345])) + (Simd32x3::from(right_anti_dual_g0[1]) * self.group2().xyz()),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g9 * Simd32x4::from(self[e12345]),
            // e1234
            right_anti_dual_g10 * self[e12345],
        )
    }
}
impl WeightContraction<Plane> for VersorEven {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl WeightContraction<RoundPoint> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       10        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        5        0      N/A
    //    simd4        6        4        0      N/A
    // Totals...
    // yes simd       12       20        0      N/A
    //  no simd       36       43        0        0
    fn weight_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e5] * -1.0);
        let right_anti_dual_g1 = other[e4] * -1.0;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from(right_anti_dual_g1) * self.group1().xyz().with_w(self[e5]))
                + (right_anti_dual_g0.yzxx() * self.group0().zxy().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w((right_anti_dual_g0[1] * self[e2]) + (right_anti_dual_g0[2] * self[e3]) + (right_anti_dual_g0[3] * self[e4]))
                - (right_anti_dual_g0.zxy() * self.group0().yzx()).with_w(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e425]) - (right_anti_dual_g0[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g1) * self.group2().xyz()).with_w(0.0)
                + (Simd32x3::from(right_anti_dual_g0[3]) * self.group0().xyz()).with_w(0.0)
                - (right_anti_dual_g0.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz())
                + (right_anti_dual_g0.zxy() * self.group2().yzx())
                + Simd32x2::from(0.0).with_z(right_anti_dual_g0[0] * self[e315] * -1.0)
                - (right_anti_dual_g0.yz() * self.group2().zx()).with_z(0.0))
            .with_w(right_anti_dual_g1 * self[e12345]),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g0 * Simd32x4::from(self[e12345]),
        )
    }
}
impl WeightContraction<Scalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(other[scalar]) * self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group3(),
        )
    }
}
impl WeightContraction<Sphere> for VersorEven {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        8        0        0
    fn weight_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]),
            // e5
            other[e3215] * self[e12345],
        )
    }
}
impl WeightContraction<VersorEven> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       21        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        4        0      N/A
    //    simd4        8        7        0      N/A
    // Totals...
    // yes simd       25       34        0      N/A
    //  no simd       57       65        0        0
    fn weight_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e12345] * -1.0);
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3 = other.group3().xyz().with_w(other[e5] * -1.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (right_anti_dual_g0 * Simd32x4::from(self[e12345]))
                + (Simd32x4::from(right_anti_dual_g2_w) * self.group1().xyz().with_w(self[e5]))
                + (right_anti_dual_g3.yzxx() * self.group0().zxy().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w(
                    (right_anti_dual_g3[1] * self[e2]) + (right_anti_dual_g3[2] * self[e3]) + (right_anti_dual_g3[3] * self[e4])
                        - (right_anti_dual_g2_xyz[1] * self[e431])
                        - (right_anti_dual_g2_xyz[2] * self[e412])
                        - (right_anti_dual_g0[0] * self[e235])
                        - (right_anti_dual_g0[1] * self[e315])
                        - (right_anti_dual_g0[2] * self[e125])
                        - (right_anti_dual_g1[0] * self[e415])
                        - (right_anti_dual_g1[1] * self[e425])
                        - (right_anti_dual_g1[2] * self[e435])
                        - (right_anti_dual_g1[3] * self[e321]),
                )
                - (Simd32x4::from([right_anti_dual_g3[2], right_anti_dual_g3[0], right_anti_dual_g3[1], right_anti_dual_g2_xyz[0]]) * self.group0().yzxx()),
            // e23, e31, e12, e45
            (right_anti_dual_g1 * Simd32x4::from(self[e12345]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g3[1] * self[e425]) - (right_anti_dual_g3[2] * self[e435]))
                + (Simd32x3::from(right_anti_dual_g2_w) * self.group2().xyz()).with_w(0.0)
                + (Simd32x3::from(right_anti_dual_g3[3]) * self.group0().xyz()).with_w(0.0)
                - (right_anti_dual_g3.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            ((right_anti_dual_g2_xyz * Simd32x3::from(self[e12345]))
                + (Simd32x3::from(right_anti_dual_g3[3]) * self.group1().xyz())
                + Simd32x2::from(0.0).with_z((right_anti_dual_g3[1] * self[e235]) - (right_anti_dual_g3[0] * self[e315]))
                + (right_anti_dual_g3.zx() * self.group2().yz()).with_z(0.0)
                - (right_anti_dual_g3.yz() * self.group2().zx()).with_z(0.0))
            .with_w(right_anti_dual_g2_w * self[e12345]),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g3 * Simd32x4::from(self[e12345]),
        )
    }
}
impl WeightContraction<VersorOdd> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        9        0        0
    //    simd3        1        4        0      N/A
    //    simd4       11       12        0      N/A
    // Totals...
    // yes simd       17       25        0      N/A
    //  no simd       52       69        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_anti_dual_g2 = (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(other[e3215]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x3::from(other[scalar]) * self.group0().xyz()) - (Simd32x3::from(self[e12345]) * other.group0().xyz())).with_w(self[e12345] * other[scalar]),
            // e415, e425, e435, e321
            (right_anti_dual_g1 * Simd32x4::from(self[e12345])) + (Simd32x4::from(other[scalar]) * self.group1()),
            // e235, e315, e125, e5
            (right_anti_dual_g2 * Simd32x4::from(self[e12345]))
                + (Simd32x4::from(other[scalar]) * self.group2())
                + Simd32x3::from(0.0).with_w(
                    -(right_anti_dual_g1[0] * self[e235])
                        - (right_anti_dual_g1[1] * self[e315])
                        - (right_anti_dual_g1[2] * self[e125])
                        - (right_anti_dual_g2[0] * self[e415])
                        - (right_anti_dual_g2[1] * self[e425])
                        - (right_anti_dual_g2[2] * self[e435]),
                ),
            // e1, e2, e3, e4
            (Simd32x4::from(other[scalar]) * self.group3())
                + (Simd32x4::from([right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g1[2], other[e41]]) * self.group1().wwwx())
                + (Simd32x4::from([right_anti_dual_g2[1], right_anti_dual_g2[2], right_anti_dual_g2[0], other[e1234]]) * self.group0().zxyw())
                + (self.group1().xyzy() * Simd32x3::from(right_anti_dual_g1[3]).with_w(other[e42]))
                + (other.group0().yzxz() * self.group2().zxy().with_w(self[e435]))
                + Simd32x3::from(0.0).with_w((right_anti_dual_g1[2] * self[e412]) * -1.0)
                - (Simd32x4::from([right_anti_dual_g2[2], right_anti_dual_g2[0], right_anti_dual_g2[1], right_anti_dual_g1[0]]) * self.group0().yzxx())
                - (self.group0().wwwy() * other.group3().xyz().with_w(right_anti_dual_g1[1]))
                - (self.group2().yzx() * other.group0().zxy()).with_w(0.0),
        )
    }
}
impl std::ops::Div<WeightContractionInfix> for VersorOdd {
    type Output = WeightContractionInfixPartial<VersorOdd>;
    fn div(self, _rhs: WeightContractionInfix) -> Self::Output {
        WeightContractionInfixPartial(self)
    }
}
impl WeightContraction<AntiCircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       15        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4        8        0      N/A
    //    simd4        8        5        0      N/A
    // Totals...
    // yes simd       20       29        0      N/A
    //  no simd       52       61        0        0
    fn weight_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from(right_anti_dual_g2[3]) * self.group0())
                + Simd32x3::from(0.0).with_w(
                    (other[e42] * self[e25]) + (other[e43] * self[e35]) + (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12])
                        - (right_anti_dual_g2[1] * self[e42])
                        - (right_anti_dual_g2[2] * self[e43]),
                )
                + (other.group0().yzx() * self.group3().zxy()).with_w(other[e41] * self[e15])
                - (other.group1() * Simd32x3::from(self[e1234]).with_w(self[e45]))
                - (other.group0().zxy() * self.group3().yzx()).with_w(right_anti_dual_g2[0] * self[e41]),
            // e23, e31, e12, e45
            (right_anti_dual_g2 * Simd32x3::from(self[e1234]).with_w(self[e45]))
                + Simd32x3::from(0.0).with_w((other[e23] * self[e4235]) + (other[e31] * self[e4315]) + (other[e12] * self[e4125]))
                + (Simd32x3::from(right_anti_dual_g2[3]) * self.group1().xyz()).with_w(0.0)
                - (Simd32x3::from(other[e45]) * self.group3().xyz()).with_w(0.0)
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_anti_dual_g2[3]) * self.group2().xyz())
                + (right_anti_dual_g2.yzx() * self.group3().zxy())
                + Simd32x2::from(0.0).with_z(right_anti_dual_g2[1] * self[e4235] * -1.0)
                - (Simd32x3::from(self[e3215]) * other.group1().xyz())
                - (right_anti_dual_g2.zx() * self.group3().yz()).with_z(0.0))
            .with_w(right_anti_dual_g2[3] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_anti_dual_g2[3]) * self.group3(),
        )
    }
}
impl WeightContraction<AntiDipoleInversion> for VersorOdd {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       16        0        0
    //    simd3        1        8        0      N/A
    //    simd4       11        6        0      N/A
    // Totals...
    // yes simd       17       30        0      N/A
    //  no simd       52       64        0        0
    fn weight_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        let right_anti_dual_g3_w = other[e5] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (right_anti_dual_g3_xyz * Simd32x3::from(self[e1234])) - (Simd32x3::from(right_anti_dual_g2_w) * self.group3().xyz()),
            // e415, e425, e435, e321
            (right_anti_dual_g3_xyz.yzx() * self.group3().zxy()).with_w(right_anti_dual_g3_w * self[e1234])
                - (self.group3().yzxw() * right_anti_dual_g3_xyz.zxy().with_w(right_anti_dual_g2_w)),
            // e235, e315, e125, e4
            (self.group3().xyzx() * Simd32x3::from(right_anti_dual_g3_w).with_w(other[e423]))
                + Simd32x3::from(0.0).with_w(
                    (right_anti_dual_g1_w * self[e1234]) + (other[e431] * self[e4315]) + (other[e412] * self[e4125])
                        - (right_anti_dual_g3_xyz[0] * self[e41])
                        - (right_anti_dual_g3_xyz[1] * self[e42])
                        - (right_anti_dual_g3_xyz[2] * self[e43]),
                )
                - (right_anti_dual_g3_xyz * Simd32x3::from(self[e3215])).with_w(right_anti_dual_g2_w * self[e45]),
            // e1, e2, e3, e5
            (Simd32x4::from(right_anti_dual_g3_w) * self.group0().xyz().with_w(self[e45]))
                + (self.group2().wwwx() * right_anti_dual_g2_xyz.with_w(right_anti_dual_g3_xyz[0]))
                + Simd32x3::from(0.0).with_w((right_anti_dual_g2_xyz[2] * self[e4125]) * -1.0)
                + (right_anti_dual_g1_xyz.zxy() * self.group3().yzx()).with_w(right_anti_dual_g3_xyz[1] * self[e25])
                + (right_anti_dual_g3_xyz.zxy() * self.group1().yzx()).with_w(right_anti_dual_g3_xyz[2] * self[e35])
                - (Simd32x4::from(self[e3215]) * other.group0().with_w(right_anti_dual_g1_w))
                - (self.group3().zxyx() * right_anti_dual_g1_xyz.yzx().with_w(right_anti_dual_g2_xyz[0]))
                - (Simd32x3::from(right_anti_dual_g2_w) * self.group2().xyz()).with_w(0.0)
                - (right_anti_dual_g3_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g2_xyz[1] * self[e4315]),
        )
    }
}
impl WeightContraction<AntiDualNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        1        6        0      N/A
    //  no simd        1       17        0        0
    fn weight_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x3::from(other[scalar]) * self.group0().xyz()).with_w((other[e3215] * self[e1234]) + (other[scalar] * self[scalar])),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[scalar]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group3(),
        )
    }
}
impl WeightContraction<AntiFlatPoint> for VersorOdd {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3        9        0        0
    fn weight_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e321] * -1.0);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            right_anti_dual_g0 * Simd32x4::from(self[e1234]),
            // e5
            -(right_anti_dual_g0[0] * self[e4235]) - (right_anti_dual_g0[1] * self[e4315]) - (right_anti_dual_g0[2] * self[e4125]) - (right_anti_dual_g0[3] * self[e3215]),
        )
    }
}
impl WeightContraction<AntiFlector> for VersorOdd {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       14        0        0
    //    simd2        0        1        0      N/A
    //    simd3        1        6        0      N/A
    //    simd4        6        2        0      N/A
    // Totals...
    // yes simd       11       23        0      N/A
    //  no simd       31       42        0        0
    fn weight_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e321] * -1.0;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e5] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            right_anti_dual_g1_xyz * Simd32x3::from(self[e1234]),
            // e415, e425, e435, e321
            ((right_anti_dual_g1_xyz.yzx() * self.group3().zxy()) - (right_anti_dual_g1_xyz.zxy() * self.group3().yzx())).with_w(right_anti_dual_g1_w * self[e1234]),
            // e235, e315, e125, e4
            (Simd32x2::from(right_anti_dual_g1_w) * self.group3().xy()).with_zw(right_anti_dual_g1_w * self[e4125], right_anti_dual_g0_w * self[e1234])
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g1_xyz[1] * self[e42]) - (right_anti_dual_g1_xyz[2] * self[e43]))
                - (right_anti_dual_g1_xyz * Simd32x3::from(self[e3215])).with_w(right_anti_dual_g1_xyz[0] * self[e41]),
            // e1, e2, e3, e5
            (Simd32x4::from(right_anti_dual_g1_w) * self.group0().xyz().with_w(self[e45]))
                + (self.group2().wwwx() * right_anti_dual_g0_xyz.with_w(right_anti_dual_g1_xyz[0]))
                + Simd32x3::from(0.0).with_w(
                    (right_anti_dual_g1_xyz[2] * self[e35])
                        - (right_anti_dual_g0_xyz[0] * self[e4235])
                        - (right_anti_dual_g0_xyz[1] * self[e4315])
                        - (right_anti_dual_g0_xyz[2] * self[e4125]),
                )
                + (right_anti_dual_g1_xyz.zxy() * self.group1().yzx()).with_w(right_anti_dual_g1_xyz[1] * self[e25])
                - (right_anti_dual_g1_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g0_w * self[e3215]),
        )
    }
}
impl WeightContraction<AntiLine> for VersorOdd {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        0        7        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        9       16        0      N/A
    //  no simd       18       30        0        0
    fn weight_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            right_anti_dual_g0 * Simd32x3::from(self[e1234]),
            // e23, e31, e12, e45
            (right_anti_dual_g1 * Simd32x3::from(self[e1234]))
                .with_w(-(right_anti_dual_g0[0] * self[e4235]) - (right_anti_dual_g0[1] * self[e4315]) - (right_anti_dual_g0[2] * self[e4125])),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(
                -(right_anti_dual_g0[1] * self[e31])
                    - (right_anti_dual_g0[2] * self[e12])
                    - (right_anti_dual_g1[0] * self[e41])
                    - (right_anti_dual_g1[1] * self[e42])
                    - (right_anti_dual_g1[2] * self[e43]),
            ) + (right_anti_dual_g0 * Simd32x3::from(self[e3215])).with_w(0.0)
                + (right_anti_dual_g1.yzx() * self.group3().zxy()).with_w(0.0)
                - (right_anti_dual_g1.zxy() * self.group3().yzx()).with_w(right_anti_dual_g0[0] * self[e23]),
        )
    }
}
impl WeightContraction<AntiMotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       12        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        3        0      N/A
    //    simd4        4        5        0      N/A
    // Totals...
    // yes simd       16       22        0      N/A
    //  no simd       36       45        0        0
    fn weight_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (right_anti_dual_g0 * Simd32x3::from(self[e1234]).with_w(self[scalar]))
                + (self.group0().xyzx() * Simd32x3::from(right_anti_dual_g0[3]).with_w(other[e15]))
                + Simd32x3::from(0.0).with_w(
                    (other[e25] * self[e42]) + (other[e35] * self[e43]) + (other[e3215] * self[e1234])
                        - (right_anti_dual_g0[0] * self[e23])
                        - (right_anti_dual_g0[1] * self[e31])
                        - (right_anti_dual_g0[2] * self[e12]),
                ),
            // e23, e31, e12, e45
            (Simd32x4::from(right_anti_dual_g0[3]) * self.group1())
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[0] * self[e4235]) - (right_anti_dual_g0[1] * self[e4315]) - (right_anti_dual_g0[2] * self[e4125]))
                - (Simd32x3::from(self[e1234]) * other.group1().xyz()).with_w(0.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_anti_dual_g0[3]) * self.group2().xyz())
                + (Simd32x3::from(self[e3215]) * right_anti_dual_g0.xyz())
                + Simd32x2::from(0.0).with_z((other[e25] * self[e4235]) - (other[e15] * self[e4315]))
                + (other.group1().zx() * self.group3().yz()).with_z(0.0)
                - (other.group1().yz() * self.group3().zx()).with_z(0.0))
            .with_w(right_anti_dual_g0[3] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_anti_dual_g0[3]) * self.group3(),
        )
    }
}
impl WeightContraction<AntiPlane> for VersorOdd {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        8        0        0
    //    simd3        1        7        0      N/A
    //    simd4        5        1        0      N/A
    // Totals...
    // yes simd        8       16        0      N/A
    //  no simd       25       33        0        0
    fn weight_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e5] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            right_anti_dual_g0_xyz * Simd32x3::from(self[e1234]),
            // e415, e425, e435, e321
            ((right_anti_dual_g0_xyz.yzx() * self.group3().zxy()) - (right_anti_dual_g0_xyz.zxy() * self.group3().yzx())).with_w(right_anti_dual_g0_w * self[e1234]),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0_xyz[1] * self[e42]) - (right_anti_dual_g0_xyz[2] * self[e43]))
                + (Simd32x3::from(right_anti_dual_g0_w) * self.group3().xyz()).with_w(0.0)
                - (right_anti_dual_g0_xyz * Simd32x3::from(self[e3215])).with_w(right_anti_dual_g0_xyz[0] * self[e41]),
            // e1, e2, e3, e5
            (Simd32x4::from(right_anti_dual_g0_w) * self.group0().xyz().with_w(self[e45]))
                + Simd32x3::from(0.0).with_w((right_anti_dual_g0_xyz[1] * self[e25]) + (right_anti_dual_g0_xyz[2] * self[e35]))
                + (right_anti_dual_g0_xyz.zxy() * self.group1().yzx()).with_w(right_anti_dual_g0_xyz[0] * self[e15])
                - (right_anti_dual_g0_xyz.yzx() * self.group1().zxy()).with_w(0.0),
        )
    }
}
impl WeightContraction<Circle> for VersorOdd {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        7        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       20       21        0        0
    fn weight_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * other.group2().with_w(right_anti_dual_g1_w))
                + (self.group3().yzxx() * right_anti_dual_g1_xyz.zxy().with_w(other[e423]))
                + Simd32x3::from(0.0).with_w((other[e431] * self[e4315]) + (other[e412] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (right_anti_dual_g1_xyz.yzx() * self.group3().zxy()).with_w(0.0),
            // e5
            -(right_anti_dual_g1_w * self[e3215]) - (other[e235] * self[e4235]) - (other[e315] * self[e4315]) - (other[e125] * self[e4125]),
        )
    }
}
impl WeightContraction<CircleRotor> for VersorOdd {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        7        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       20       21        0        0
    fn weight_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * right_anti_dual_g2_xyz.with_w(right_anti_dual_g1_w))
                + (self.group3().yzxx() * right_anti_dual_g1_xyz.zxy().with_w(other[e423]))
                + Simd32x3::from(0.0).with_w((other[e431] * self[e4315]) + (other[e412] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (right_anti_dual_g1_xyz.yzx() * self.group3().zxy()).with_w(0.0),
            // e5
            -(right_anti_dual_g1_w * self[e3215])
                - (right_anti_dual_g2_xyz[0] * self[e4235])
                - (right_anti_dual_g2_xyz[1] * self[e4315])
                - (right_anti_dual_g2_xyz[2] * self[e4125]),
        )
    }
}
impl WeightContraction<Dipole> for VersorOdd {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       12        0        0
    //    simd3        2        9        0      N/A
    //    simd4        6        1        0      N/A
    // Totals...
    // yes simd       15       22        0      N/A
    //  no simd       37       43        0        0
    fn weight_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (other.group0().yzx() * self.group3().zxy()) - (Simd32x3::from(self[e1234]) * other.group1().xyz()) - (other.group0().zxy() * self.group3().yzx()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w((other[e31] * self[e4315]) + (other[e12] * self[e4125]))
                + (right_anti_dual_g2 * Simd32x3::from(self[e1234])).with_w(other[e23] * self[e4235])
                - (Simd32x3::from(other[e45]) * self.group3().xyz()).with_w(0.0)
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(
                (other[e42] * self[e25]) + (other[e43] * self[e35]) + (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12])
                    - (right_anti_dual_g2[1] * self[e42])
                    - (right_anti_dual_g2[2] * self[e43]),
            ) + (right_anti_dual_g2.yzx() * self.group3().zxy()).with_w(other[e41] * self[e15])
                - (other.group1() * Simd32x3::from(self[e3215]).with_w(self[e45]))
                - (right_anti_dual_g2.zxy() * self.group3().yzx()).with_w(right_anti_dual_g2[0] * self[e41]),
        )
    }
}
impl WeightContraction<DipoleInversion> for VersorOdd {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       19       24        0        0
    //    simd3        2        3        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd       24       30        0      N/A
    //  no simd       37       45        0        0
    fn weight_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (other.group0().yzx() * self.group3().zxy()) - (Simd32x3::from(self[e1234]) * other.group1().xyz()) - (other.group0().zxy() * self.group3().yzx()),
            // e23, e31, e12, e45
            Simd32x4::from([
                -(other[e41] * self[e3215]) - (other[e45] * self[e4235]) - (other[e15] * self[e1234]),
                -(other[e42] * self[e3215]) - (other[e45] * self[e4315]) - (other[e25] * self[e1234]),
                -(other[e43] * self[e3215]) - (other[e45] * self[e4125]) - (other[e35] * self[e1234]),
                (other[e23] * self[e4235]) + (other[e31] * self[e4315]) + (other[e12] * self[e4125]),
            ]),
            // e15, e25, e35, scalar
            (other.group2().zxyx() * self.group3().yzx().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w(
                    (other[e41] * self[e15])
                        + (other[e42] * self[e25])
                        + (other[e43] * self[e35])
                        + (other[e23] * self[e23])
                        + (other[e31] * self[e31])
                        + (other[e12] * self[e12])
                        + (other[e25] * self[e42])
                        + (other[e35] * self[e43])
                        + (other[e1234] * self[e3215])
                        + (other[e3215] * self[e1234])
                        - (other[e4315] * self[e4315])
                        - (other[e4125] * self[e4125]),
                )
                - (other.group1() * Simd32x3::from(self[e3215]).with_w(self[e45]))
                - (self.group3().zxyx() * other.group2().yzx().with_w(other[e4235])),
        )
    }
}
impl WeightContraction<DualNum> for VersorOdd {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn weight_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e5] * -1.0) * self.group3().xyz().with_w(self[e1234]),
            // e1, e2, e3, e5
            Simd32x4::from(other[e5] * -1.0) * self.group0().xyz().with_w(self[e45]),
        )
    }
}
impl WeightContraction<FlatPoint> for VersorOdd {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       13       16        0        0
    fn weight_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w((other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43]))
                - (other.group0() * Simd32x3::from(self[e1234]).with_w(self[e45]))
                - (Simd32x3::from(other[e45]) * self.group3().xyz()).with_w(0.0),
            // e15, e25, e35, e3215
            ((other.group0().zxy() * self.group3().yzx()) - (other.group0().yzx() * self.group3().zxy())).with_w(0.0),
        )
    }
}
impl WeightContraction<Flector> for VersorOdd {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       16       20        0        0
    fn weight_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(
                (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43]) + (other[e3215] * self[e1234])
                    - (other[e4315] * self[e4315])
                    - (other[e4125] * self[e4125]),
            ) - (Simd32x4::from([other[e45], other[e45], other[e45], other[e4235]]) * self.group3().xyzx())
                - (other.group0() * Simd32x3::from(self[e1234]).with_w(self[e45])),
            // e15, e25, e35, e3215
            ((other.group0().zxy() * self.group3().yzx()) - (other.group0().yzx() * self.group3().zxy())).with_w(0.0),
        )
    }
}
impl WeightContraction<Line> for VersorOdd {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn weight_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(other[e315] * self[e4315]) - (other[e125] * self[e4125]))
                + (Simd32x3::from(self[e1234]) * other.group1()).with_w(0.0)
                + (other.group0().zxy() * self.group3().yzx()).with_w(0.0)
                - (Simd32x4::from([other[e425], other[e435], other[e415], other[e235]]) * self.group3().zxyx()),
        )
    }
}
impl WeightContraction<Motor> for VersorOdd {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd       17       21        0        0
    fn weight_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e5] * -1.0);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_anti_dual_g1[3]) * self.group3().xyz().with_w(self[e1234]),
            // e1, e2, e3, e5
            (right_anti_dual_g1 * Simd32x3::from(self[e1234]).with_w(self[e45]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g1[1] * self[e4315]) - (right_anti_dual_g1[2] * self[e4125]))
                + (Simd32x3::from(right_anti_dual_g1[3]) * self.group0().xyz()).with_w(0.0)
                + (right_anti_dual_g0_xyz.zxy() * self.group3().yzx()).with_w(0.0)
                - (self.group3().zxyx() * right_anti_dual_g0_xyz.yzx().with_w(right_anti_dual_g1[0])),
        )
    }
}
impl WeightContraction<MultiVector> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       23       36        0        0
    //    simd2        0        1        0      N/A
    //    simd3        8       24        0      N/A
    //    simd4       13        7        0      N/A
    // Totals...
    // yes simd       44       68        0      N/A
    //  no simd       99      138        0        0
    fn weight_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g3 = other.group8().with_w(other[e321] * -1.0);
        let right_anti_dual_g5 = other.group6().xyz();
        let right_anti_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_anti_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_anti_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g9_xyz = other.group1().xyz();
        let right_anti_dual_g9_w = other[e5] * -1.0;
        let right_anti_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_anti_dual_g0[1] * self[scalar])
                    + (right_anti_dual_g1_xyz[0] * self[e4235])
                    + (right_anti_dual_g1_xyz[1] * self[e4315])
                    + (right_anti_dual_g1_xyz[2] * self[e4125])
                    + (other[e3215] * self[e1234])
                    + (other[e1234] * self[e3215])
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
                0.0,
            ]),
            // e1, e2, e3, e4
            (right_anti_dual_g3 * Simd32x4::from(self[e1234]))
                + (self.group3().yzxx() * right_anti_dual_g5.zxy().with_w(other[e423]))
                + Simd32x3::from(0.0).with_w(other[e412] * self[e4125])
                + (Simd32x3::from(right_anti_dual_g9_w) * self.group0().xyz()).with_w(0.0)
                + (right_anti_dual_g9_xyz.zxy() * self.group1().yzx()).with_w(other[e431] * self[e4315])
                - (Simd32x4::from(right_anti_dual_g10) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                - (Simd32x3::from(self[e3215]) * other.group7()).with_w(right_anti_dual_g9_xyz[2] * self[e43])
                - (right_anti_dual_g5.yzx() * self.group3().zxy()).with_w(right_anti_dual_g9_xyz[0] * self[e41])
                - (right_anti_dual_g9_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g9_xyz[1] * self[e42]),
            // e5
            (right_anti_dual_g9_w * self[e45]) + (right_anti_dual_g9_xyz[0] * self[e15]) + (right_anti_dual_g9_xyz[1] * self[e25]) + (right_anti_dual_g9_xyz[2] * self[e35])
                - (right_anti_dual_g3[0] * self[e4235])
                - (right_anti_dual_g3[1] * self[e4315])
                - (right_anti_dual_g3[2] * self[e4125])
                - (right_anti_dual_g3[3] * self[e3215]),
            // e15, e25, e35, e45
            (Simd32x4::from(right_anti_dual_g0[1]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g6_xyz[1] * self[e4315]) - (right_anti_dual_g6_xyz[2] * self[e4125]))
                + (right_anti_dual_g6_xyz * Simd32x3::from(self[e3215])).with_w(0.0)
                + (right_anti_dual_g8.yzx() * self.group3().zxy()).with_w(0.0)
                - (self.group3().yzxx() * right_anti_dual_g8.zxy().with_w(right_anti_dual_g6_xyz[0])),
            // e41, e42, e43
            (right_anti_dual_g6_xyz * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(right_anti_dual_g0[1]) * self.group0().xyz())
                + (right_anti_dual_g7.zxy() * self.group3().yzx())
                - (right_anti_dual_g7.yzx() * self.group3().zxy()),
            // e23, e31, e12
            (right_anti_dual_g7 * Simd32x3::from(self[e3215])) + (right_anti_dual_g8 * Simd32x3::from(self[e1234])) + (Simd32x3::from(right_anti_dual_g0[1]) * self.group1().xyz())
                - (Simd32x3::from(other[e45]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (right_anti_dual_g9_xyz.yzx() * self.group3().zxy()).with_w(right_anti_dual_g9_w * self[e1234])
                - (self.group3().yzxw() * right_anti_dual_g9_xyz.zxy().with_w(right_anti_dual_g10)),
            // e423, e431, e412
            (right_anti_dual_g9_xyz * Simd32x3::from(self[e1234])) - (Simd32x3::from(right_anti_dual_g10) * self.group3().xyz()),
            // e235, e315, e125
            (Simd32x3::from(right_anti_dual_g9_w) * self.group3().xyz()) - (right_anti_dual_g9_xyz * Simd32x3::from(self[e3215])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_anti_dual_g0[1]) * self.group3(),
            // e1234
            right_anti_dual_g0[1] * self[e1234],
        )
    }
}
impl WeightContraction<Plane> for VersorOdd {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (other[e3215] * self[e1234]) - (other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]),
        )
    }
}
impl WeightContraction<RoundPoint> for VersorOdd {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       10        0        0
    //    simd3        1        8        0      N/A
    //    simd4        7        2        0      N/A
    // Totals...
    // yes simd       11       20        0      N/A
    //  no simd       34       42        0        0
    fn weight_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e5] * -1.0;
        let right_anti_dual_g1 = other[e4] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (right_anti_dual_g0_xyz * Simd32x3::from(self[e1234])) - (Simd32x3::from(right_anti_dual_g1) * self.group3().xyz()),
            // e415, e425, e435, e321
            (right_anti_dual_g0_xyz.yzx() * self.group3().zxy()).with_w(right_anti_dual_g0_w * self[e1234])
                - (self.group3().yzxw() * right_anti_dual_g0_xyz.zxy().with_w(right_anti_dual_g1)),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0_xyz[0] * self[e41]) - (right_anti_dual_g0_xyz[1] * self[e42]) - (right_anti_dual_g0_xyz[2] * self[e43]))
                + (Simd32x3::from(right_anti_dual_g0_w) * self.group3().xyz()).with_w(0.0)
                - (right_anti_dual_g0_xyz * Simd32x3::from(self[e3215])).with_w(right_anti_dual_g1 * self[e45]),
            // e1, e2, e3, e5
            (Simd32x4::from(right_anti_dual_g0_w) * self.group0().xyz().with_w(self[e45]))
                + Simd32x3::from(0.0).with_w((right_anti_dual_g0_xyz[1] * self[e25]) + (right_anti_dual_g0_xyz[2] * self[e35]))
                + (right_anti_dual_g0_xyz.zxy() * self.group1().yzx()).with_w(right_anti_dual_g0_xyz[0] * self[e15])
                - (Simd32x3::from(right_anti_dual_g1) * self.group2().xyz()).with_w(0.0)
                - (right_anti_dual_g0_xyz.yzx() * self.group1().zxy()).with_w(0.0),
        )
    }
}
impl WeightContraction<Scalar> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn weight_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[scalar]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group3(),
        )
    }
}
impl WeightContraction<Sphere> for VersorOdd {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        5        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd        4        8        0        0
    fn weight_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        Scalar::from_groups(
            // scalar
            (right_anti_dual_g0_xyz[0] * self[e4235])
                + (right_anti_dual_g0_xyz[1] * self[e4315])
                + (right_anti_dual_g0_xyz[2] * self[e4125])
                + (other[e3215] * self[e1234])
                + (other[e1234] * self[e3215]),
        )
    }
}
impl WeightContraction<VersorEven> for VersorOdd {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       16        0        0
    //    simd3        1        8        0      N/A
    //    simd4       11        6        0      N/A
    // Totals...
    // yes simd       17       30        0      N/A
    //  no simd       52       64        0        0
    fn weight_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        let right_anti_dual_g3_w = other[e5] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (right_anti_dual_g3_xyz * Simd32x3::from(self[e1234])) - (Simd32x3::from(right_anti_dual_g2_w) * self.group3().xyz()),
            // e415, e425, e435, e321
            (right_anti_dual_g3_xyz.yzx() * self.group3().zxy()).with_w(right_anti_dual_g3_w * self[e1234])
                - (self.group3().yzxw() * right_anti_dual_g3_xyz.zxy().with_w(right_anti_dual_g2_w)),
            // e235, e315, e125, e4
            (self.group3().xyzx() * Simd32x3::from(right_anti_dual_g3_w).with_w(right_anti_dual_g0_xyz[0]))
                + Simd32x3::from(0.0).with_w(
                    (right_anti_dual_g1_w * self[e1234]) + (right_anti_dual_g0_xyz[1] * self[e4315]) + (right_anti_dual_g0_xyz[2] * self[e4125])
                        - (right_anti_dual_g3_xyz[0] * self[e41])
                        - (right_anti_dual_g3_xyz[1] * self[e42])
                        - (right_anti_dual_g3_xyz[2] * self[e43]),
                )
                - (right_anti_dual_g3_xyz * Simd32x3::from(self[e3215])).with_w(right_anti_dual_g2_w * self[e45]),
            // e1, e2, e3, e5
            (Simd32x4::from(right_anti_dual_g3_w) * self.group0().xyz().with_w(self[e45]))
                + (self.group2().wwwx() * right_anti_dual_g2_xyz.with_w(right_anti_dual_g3_xyz[0]))
                + Simd32x3::from(0.0).with_w((right_anti_dual_g2_xyz[2] * self[e4125]) * -1.0)
                + (right_anti_dual_g1_xyz.zxy() * self.group3().yzx()).with_w(right_anti_dual_g3_xyz[1] * self[e25])
                + (right_anti_dual_g3_xyz.zxy() * self.group1().yzx()).with_w(right_anti_dual_g3_xyz[2] * self[e35])
                - (Simd32x4::from(self[e3215]) * right_anti_dual_g0_xyz.with_w(right_anti_dual_g1_w))
                - (self.group3().zxyx() * right_anti_dual_g1_xyz.yzx().with_w(right_anti_dual_g2_xyz[0]))
                - (Simd32x3::from(right_anti_dual_g2_w) * self.group2().xyz()).with_w(0.0)
                - (right_anti_dual_g3_xyz.yzx() * self.group1().zxy()).with_w(right_anti_dual_g2_xyz[1] * self[e4315]),
        )
    }
}
impl WeightContraction<VersorOdd> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       16        0        0
    //    simd3        3        9        0      N/A
    //    simd4        8        7        0      N/A
    // Totals...
    // yes simd       23       32        0      N/A
    //  no simd       53       71        0        0
    fn weight_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_anti_dual_g2_xyz = other.group2().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from(right_anti_dual_g0[3]) * self.group0())
                + (Simd32x4::from([right_anti_dual_g0[2], right_anti_dual_g0[0], right_anti_dual_g0[1], right_anti_dual_g3_xyz[0]]) * self.group3().yzxx())
                + Simd32x3::from(0.0).with_w(
                    (right_anti_dual_g3_xyz[1] * self[e4315])
                        + (right_anti_dual_g3_xyz[2] * self[e4125])
                        + (other[e23] * self[e23])
                        + (other[e31] * self[e31])
                        + (other[e12] * self[e12])
                        + (other[e1234] * self[e3215])
                        + (other[e3215] * self[e1234])
                        - (right_anti_dual_g2_xyz[0] * self[e41])
                        - (right_anti_dual_g2_xyz[1] * self[e42])
                        - (right_anti_dual_g2_xyz[2] * self[e43])
                        - (right_anti_dual_g0[1] * self[e25])
                        - (right_anti_dual_g0[2] * self[e35]),
                )
                - (other.group1() * Simd32x3::from(self[e1234]).with_w(self[e45]))
                - (right_anti_dual_g0.yzxx() * self.group3().zxy().with_w(self[e15])),
            // e23, e31, e12, e45
            (right_anti_dual_g0 * Simd32x3::from(self[e3215]).with_w(self[e45]))
                + Simd32x3::from(0.0).with_w((other[e31] * self[e4315]) + (other[e12] * self[e4125]))
                + (right_anti_dual_g2_xyz * Simd32x3::from(self[e1234])).with_w(other[e23] * self[e4235])
                + (Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz()).with_w(0.0)
                - (Simd32x3::from(other[e45]) * self.group3().xyz()).with_w(0.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_anti_dual_g0[3]) * self.group2().xyz()) + (right_anti_dual_g2_xyz.yzx() * self.group3().zxy())
                - (Simd32x3::from(self[e3215]) * other.group1().xyz())
                - (right_anti_dual_g2_xyz.zxy() * self.group3().yzx()))
            .with_w(right_anti_dual_g0[3] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_anti_dual_g0[3]) * self.group3(),
        )
    }
}
