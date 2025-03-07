// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 502
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         4       9       0
//  Average:         6      13       0
//  Maximum:       111     147       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         5      19       0
//  Average:        11      26       0
//  Maximum:       211     265       0
impl std::ops::Div<BulkContractionInfix> for AntiCircleRotor {
    type Output = BulkContractionInfixPartial<AntiCircleRotor>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       11        0
    //    simd3        0        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       10       17        0
    //  no simd       10       32        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual_g2[3]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual_g2[3]) * self.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(right_dual_g2[3]) * self.group2().xyz()).with_w(
                (right_dual_g2[3] * self[scalar])
                    - (right_dual_g0[0] * self[e15])
                    - (right_dual_g0[1] * self[e25])
                    - (right_dual_g0[2] * self[e35])
                    - (right_dual_g1[0] * self[e23])
                    - (right_dual_g1[1] * self[e31])
                    - (right_dual_g1[2] * self[e12])
                    - (right_dual_g1[3] * self[e45])
                    - (right_dual_g2[0] * self[e41])
                    - (right_dual_g2[1] * self[e42])
                    - (right_dual_g2[2] * self[e43]),
            ),
        )
    }
}
impl BulkContraction<AntiDipoleInversion> for AntiCircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       15       28        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g3[2] * self[e31]) + (right_dual_g3[3] * self[e41]),
                (right_dual_g3[0] * self[e12]) + (right_dual_g3[3] * self[e42]),
                (right_dual_g3[1] * self[e23]) + (right_dual_g3[3] * self[e43]),
                -(right_dual_g2[3] * self[e45]) - (right_dual_g3[2] * self[e43]),
            ]) - (right_dual_g3.yzxx() * self.group1().zxy().with_w(self[e41]))
                - (self.group2().xyz() * right_dual_g2.www()).with_w(right_dual_g3[1] * self[e42]),
            // e5
            (right_dual_g3[0] * self[e15]) + (right_dual_g3[1] * self[e25]) + (right_dual_g3[2] * self[e35]) + (right_dual_g3[3] * self[e45]),
        )
    }
}
impl BulkContraction<AntiDualNum> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
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
impl BulkContraction<AntiFlector> for AntiCircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        8       14        0
    //  no simd       11       20        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g1[2] * self[e31]) + (right_dual_g1[3] * self[e41]),
                (right_dual_g1[0] * self[e12]) + (right_dual_g1[3] * self[e42]),
                (right_dual_g1[1] * self[e23]) + (right_dual_g1[3] * self[e43]),
                -(right_dual_g1[1] * self[e42]) - (right_dual_g1[2] * self[e43]),
            ]) - (right_dual_g1.yzxx() * self.group1().zxy().with_w(self[e41])),
            // e5
            (right_dual_g1[0] * self[e15]) + (right_dual_g1[1] * self[e25]) + (right_dual_g1[2] * self[e35]) + (right_dual_g1[3] * self[e45]),
        )
    }
}
impl BulkContraction<AntiLine> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        Scalar::from_groups(
            // scalar
            -(right_dual_g0[0] * self[e23])
                - (right_dual_g0[1] * self[e31])
                - (right_dual_g0[2] * self[e12])
                - (right_dual_g1[0] * self[e41])
                - (right_dual_g1[1] * self[e42])
                - (right_dual_g1[2] * self[e43]),
        )
    }
}
impl BulkContraction<AntiMotor> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       25        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual_g0[3]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual_g0[3]) * self.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(right_dual_g0[3]) * self.group2().xyz()).with_w(
                (right_dual_g0[3] * self[scalar])
                    - (right_dual_g0[0] * self[e23])
                    - (right_dual_g0[1] * self[e31])
                    - (right_dual_g0[2] * self[e12])
                    - (right_dual_g1[0] * self[e41])
                    - (right_dual_g1[1] * self[e42])
                    - (right_dual_g1[2] * self[e43]),
            ),
        )
    }
}
impl BulkContraction<AntiPlane> for AntiCircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        8       14        0
    //  no simd       11       20        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[2] * self[e31]) + (right_dual_g0[3] * self[e41]),
                (right_dual_g0[0] * self[e12]) + (right_dual_g0[3] * self[e42]),
                (right_dual_g0[1] * self[e23]) + (right_dual_g0[3] * self[e43]),
                -(right_dual_g0[1] * self[e42]) - (right_dual_g0[2] * self[e43]),
            ]) - (right_dual_g0.yzxx() * self.group1().zxy().with_w(self[e41])),
            // e5
            (right_dual_g0[0] * self[e15]) + (right_dual_g0[1] * self[e25]) + (right_dual_g0[2] * self[e35]) + (right_dual_g0[3] * self[e45]),
        )
    }
}
impl BulkContraction<Dipole> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd        9       20        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        Scalar::from_groups(
            // scalar
            -(right_dual_g0[0] * self[e15])
                - (right_dual_g0[1] * self[e25])
                - (right_dual_g0[2] * self[e35])
                - (right_dual_g2[0] * self[e41])
                - (right_dual_g2[1] * self[e42])
                - (right_dual_g2[2] * self[e43])
                - (right_dual_g1[0] * self[e23])
                - (right_dual_g1[1] * self[e31])
                - (right_dual_g1[2] * self[e12])
                - (right_dual_g1[3] * self[e45]),
        )
    }
}
impl BulkContraction<DipoleInversion> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        9       13        0
    //  no simd        9       21        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Scalar::from_groups(
            // scalar
            -(right_dual_g0[0] * self[e15])
                - (right_dual_g0[1] * self[e25])
                - (right_dual_g0[2] * self[e35])
                - (right_dual_g1[0] * self[e23])
                - (right_dual_g1[1] * self[e31])
                - (right_dual_g1[2] * self[e12])
                - (right_dual_g1[3] * self[e45])
                - (right_dual_g2[0] * self[e41])
                - (right_dual_g2[1] * self[e42])
                - (right_dual_g2[2] * self[e43]),
        )
    }
}
impl BulkContraction<DualNum> for AntiCircleRotor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e5] * -1.0) * self.group0().with_w(self[e45]))
    }
}
impl BulkContraction<FlatPoint> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Scalar::from_groups(
            // scalar
            -(right_dual_g0[0] * self[e41]) - (right_dual_g0[1] * self[e42]) - (right_dual_g0[2] * self[e43]) - (right_dual_g0[3] * self[e45]),
        )
    }
}
impl BulkContraction<Flector> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Scalar::from_groups(
            // scalar
            -(right_dual_g0[0] * self[e41]) - (right_dual_g0[1] * self[e42]) - (right_dual_g0[2] * self[e43]) - (right_dual_g0[3] * self[e45]),
        )
    }
}
impl BulkContraction<Motor> for AntiCircleRotor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e5] * -1.0) * self.group0().with_w(self[e45]))
    }
}
impl BulkContraction<MultiVector> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       25        0
    //    simd2        0        1        0
    //    simd3        0        5        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       19       35        0
    //  no simd       25       58        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g6 = other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_dual_g9 = other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g0[1] * self[scalar])
                    - (right_dual_g7[0] * self[e15])
                    - (right_dual_g7[1] * self[e25])
                    - (right_dual_g7[2] * self[e35])
                    - (right_dual_g8[0] * self[e41])
                    - (right_dual_g8[1] * self[e42])
                    - (right_dual_g8[2] * self[e43])
                    - (right_dual_g6[0] * self[e23])
                    - (right_dual_g6[1] * self[e31])
                    - (right_dual_g6[2] * self[e12])
                    - (right_dual_g6[3] * self[e45]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g9[2] * self[e31]) + (right_dual_g9[3] * self[e41]),
                (right_dual_g9[0] * self[e12]) + (right_dual_g9[3] * self[e42]),
                (right_dual_g9[1] * self[e23]) + (right_dual_g9[3] * self[e43]),
                -(right_dual_g10 * self[e45]) - (right_dual_g9[2] * self[e43]),
            ]) - (right_dual_g9.yzxx() * self.group1().zxy().with_w(self[e41]))
                - (Simd32x3::from(right_dual_g10) * self.group2().xyz()).with_w(right_dual_g9[1] * self[e42]),
            // e5
            (right_dual_g9[0] * self[e15]) + (right_dual_g9[1] * self[e25]) + (right_dual_g9[2] * self[e35]) + (right_dual_g9[3] * self[e45]),
            // e15, e25, e35, e45
            Simd32x4::from(right_dual_g0[1]) * self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            Simd32x3::from(right_dual_g0[1]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(right_dual_g0[1]) * self.group1().xyz(),
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
impl BulkContraction<RoundPoint> for AntiCircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       15       25        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other[e4] * -1.0;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[2] * self[e31]) + (right_dual_g0[3] * self[e41]),
                (right_dual_g0[0] * self[e12]) + (right_dual_g0[3] * self[e42]),
                (right_dual_g0[1] * self[e23]) + (right_dual_g0[3] * self[e43]),
                -(right_dual_g1 * self[e45]) - (right_dual_g0[2] * self[e43]),
            ]) - (right_dual_g0.yzxx() * self.group1().zxy().with_w(self[e41]))
                - (Simd32x3::from(right_dual_g1) * self.group2().xyz()).with_w(right_dual_g0[1] * self[e42]),
            // e5
            (right_dual_g0[0] * self[e15]) + (right_dual_g0[1] * self[e25]) + (right_dual_g0[2] * self[e35]) + (right_dual_g0[3] * self[e45]),
        )
    }
}
impl BulkContraction<Scalar> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
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
impl BulkContraction<VersorEven> for AntiCircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       15       28        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2 = other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g3[2] * self[e31]) + (right_dual_g3[3] * self[e41]),
                (right_dual_g3[0] * self[e12]) + (right_dual_g3[3] * self[e42]),
                (right_dual_g3[1] * self[e23]) + (right_dual_g3[3] * self[e43]),
                -(right_dual_g2[3] * self[e45]) - (right_dual_g3[2] * self[e43]),
            ]) - (right_dual_g3.yzxx() * self.group1().zxy().with_w(self[e41]))
                - (self.group2().xyz() * right_dual_g2.www()).with_w(right_dual_g3[1] * self[e42]),
            // e5
            (right_dual_g3[0] * self[e15]) + (right_dual_g3[1] * self[e25]) + (right_dual_g3[2] * self[e35]) + (right_dual_g3[3] * self[e45]),
        )
    }
}
impl BulkContraction<VersorOdd> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       11        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       10       17        0
    //  no simd       10       33        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual_g0[3]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual_g0[3]) * self.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(right_dual_g0[3]) * self.group2().xyz()).with_w(
                (right_dual_g0[3] * self[scalar])
                    - (right_dual_g0[0] * self[e15])
                    - (right_dual_g0[1] * self[e25])
                    - (right_dual_g0[2] * self[e35])
                    - (right_dual_g1[0] * self[e23])
                    - (right_dual_g1[1] * self[e31])
                    - (right_dual_g1[2] * self[e12])
                    - (right_dual_g1[3] * self[e45])
                    - (right_dual_g2[0] * self[e41])
                    - (right_dual_g2[1] * self[e42])
                    - (right_dual_g2[2] * self[e43]),
            ),
        )
    }
}
impl std::ops::Div<BulkContractionInfix> for AntiDipoleInversion {
    type Output = BulkContractionInfixPartial<AntiDipoleInversion>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       24        0
    //    simd3        0        4        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       21       33        0
    //  no simd       30       56        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual_g2[3]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual_g2[3]) * self.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(right_dual_g2[3]) * self.group2().xyz()).with_w(
                (right_dual_g2[3] * self[e4])
                    - (right_dual_g0[0] * self[e415])
                    - (right_dual_g0[1] * self[e425])
                    - (right_dual_g0[2] * self[e435])
                    - (right_dual_g1[0] * self[e423])
                    - (right_dual_g1[1] * self[e431])
                    - (right_dual_g1[2] * self[e412]),
            ),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g1[0] * self[e321]) + (right_dual_g1[3] * self[e415]) + (right_dual_g2[3] * self[e1]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g1[1] * self[e321]) + (right_dual_g1[3] * self[e425]) + (right_dual_g2[3] * self[e2]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g1[2] * self[e321]) + (right_dual_g1[3] * self[e435]) + (right_dual_g2[3] * self[e3]),
                -(right_dual_g1[0] * self[e235]) - (right_dual_g1[1] * self[e315]) - (right_dual_g1[2] * self[e125]) - (right_dual_g2[2] * self[e435]),
            ]) + (right_dual_g2.yzxw() * self.group0().zxy().with_w(self[e5]))
                - (right_dual_g2.zxyx() * self.group0().yzx().with_w(self[e415]))
                - (right_dual_g0.yzx() * self.group2().zxy()).with_w(right_dual_g2[1] * self[e425]),
        )
    }
}
impl BulkContraction<AntiDipoleInversion> for AntiDipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        2        4        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       21       31        0
    //  no simd       37       57        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(right_dual_g2[3]) * self.group1().xyz()) + (self.group0().zxy() * right_dual_g3.yzx()) - (self.group0().yzx() * right_dual_g3.zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual_g2[3] * self[e235]) + (right_dual_g3[3] * self[e423]),
                (right_dual_g2[3] * self[e315]) + (right_dual_g3[3] * self[e431]),
                (right_dual_g2[3] * self[e125]) + (right_dual_g3[3] * self[e412]),
                -(right_dual_g3[1] * self[e425]) - (right_dual_g3[2] * self[e435]),
            ]) - (right_dual_g3.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(right_dual_g3[3]) * self.group1().xyz().with_w(self[e4]))
                + (right_dual_g3.zxyx() * self.group2().yzx().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual_g2[3] * self[e5]) + (right_dual_g3[1] * self[e2]) + (right_dual_g3[2] * self[e3])
                        - (right_dual_g1[0] * self[e415])
                        - (right_dual_g1[1] * self[e425])
                        - (right_dual_g1[2] * self[e435])
                        - (right_dual_g1[3] * self[e321])
                        - (right_dual_g2[1] * self[e431])
                        - (right_dual_g2[2] * self[e412])
                        - (other[e423] * self[e235])
                        - (other[e431] * self[e315])
                        - (other[e412] * self[e125]),
                )
                - (right_dual_g3.yzx() * self.group2().zxy()).with_w(right_dual_g2[0] * self[e423]),
        )
    }
}
impl BulkContraction<AntiDualNum> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
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
impl BulkContraction<AntiFlatPoint> for AntiDipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(
            // scalar
            -(right_dual_g0[0] * self[e423]) - (right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412]) - (right_dual_g0[3] * self[e321]),
        )
    }
}
impl BulkContraction<AntiFlector> for AntiDipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        8        0
    //    simd3        1        4        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       10       17        0
    //  no simd       24       40        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (self.group0().zxy() * right_dual_g1.yzx()) - (self.group0().yzx() * right_dual_g1.zxy()),
            // e23, e31, e12, e45
            (self.group0() * right_dual_g1.www()).with_w(-(right_dual_g1[1] * self[e425]) - (right_dual_g1[2] * self[e435])) - (right_dual_g1.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(right_dual_g1[3]) * self.group1().xyz().with_w(self[e4]))
                + (right_dual_g1.zxyx() * self.group2().yzx().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual_g1[1] * self[e2]) + (right_dual_g1[2] * self[e3])
                        - (right_dual_g0[1] * self[e431])
                        - (right_dual_g0[2] * self[e412])
                        - (right_dual_g0[3] * self[e321]),
                )
                - (right_dual_g1.yzx() * self.group2().zxy()).with_w(right_dual_g0[0] * self[e423]),
        )
    }
}
impl BulkContraction<AntiLine> for AntiDipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       24        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[0] * self[e321]) + (right_dual_g1[1] * self[e412]),
                (right_dual_g0[1] * self[e321]) + (right_dual_g1[2] * self[e423]),
                (right_dual_g0[2] * self[e321]) + (right_dual_g1[0] * self[e431]),
                -(right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412]),
            ]) - (right_dual_g1.zxy() * self.group0().yzx()).with_w(right_dual_g0[0] * self[e423]),
            // e5
            -(right_dual_g0[0] * self[e235])
                - (right_dual_g0[1] * self[e315])
                - (right_dual_g0[2] * self[e125])
                - (right_dual_g1[0] * self[e415])
                - (right_dual_g1[1] * self[e425])
                - (right_dual_g1[2] * self[e435]),
        )
    }
}
impl BulkContraction<AntiMotor> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       16        0
    //    simd3        0        3        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       12       23        0
    //  no simd       18       41        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual_g0[3]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual_g0[3]) * self.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(right_dual_g0[3]) * self.group2().xyz())
                .with_w((right_dual_g0[3] * self[e4]) - (right_dual_g0[0] * self[e423]) - (right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[0] * self[e321]) + (right_dual_g0[3] * self[e1]),
                (right_dual_g0[1] * self[e321]) + (right_dual_g0[3] * self[e2]),
                (right_dual_g0[2] * self[e321]) + (right_dual_g0[3] * self[e3]),
                -(right_dual_g0[0] * self[e235])
                    - (right_dual_g0[1] * self[e315])
                    - (right_dual_g0[2] * self[e125])
                    - (right_dual_g1[1] * self[e425])
                    - (right_dual_g1[2] * self[e435]),
            ]) + (self.group0().zxy() * right_dual_g1.yzx()).with_w(right_dual_g0[3] * self[e5])
                - (right_dual_g1.zxyx() * self.group0().yzx().with_w(self[e415])),
        )
    }
}
impl BulkContraction<AntiPlane> for AntiDipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        1        5        0
    //    simd4        3        4        0
    // Totals...
    // yes simd        6       13        0
    //  no simd       17       35        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (self.group0().zxy() * right_dual_g0.yzx()) - (self.group0().yzx() * right_dual_g0.zxy()),
            // e23, e31, e12, e45
            (self.group0() * right_dual_g0.www()).with_w(-(right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435])) - (right_dual_g0.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(right_dual_g0[3]) * self.group1().xyz().with_w(self[e4]))
                + (right_dual_g0.zxyx() * self.group2().yzx().with_w(self[e1]))
                + (right_dual_g0.yzx() * self.group2().zxy() * Simd32x3::from(-1.0)).with_w((right_dual_g0[1] * self[e2]) + (right_dual_g0[2] * self[e3])),
        )
    }
}
impl BulkContraction<Circle> for AntiDipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       11        0
    //  no simd        9       14        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(
            // scalar
            -(right_dual_g1[0] * self[e415])
                - (right_dual_g1[1] * self[e425])
                - (right_dual_g1[2] * self[e435])
                - (right_dual_g1[3] * self[e321])
                - (self[e423] * other[e235])
                - (self[e431] * other[e315])
                - (self[e412] * other[e125])
                - (self[e235] * other[e423])
                - (self[e315] * other[e431])
                - (self[e125] * other[e412]),
        )
    }
}
impl BulkContraction<CircleRotor> for AntiDipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        9       12        0
    //  no simd        9       18        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(
            // scalar
            -(right_dual_g1[0] * self[e415])
                - (right_dual_g1[1] * self[e425])
                - (right_dual_g1[2] * self[e435])
                - (right_dual_g1[3] * self[e321])
                - (right_dual_g2[0] * self[e423])
                - (right_dual_g2[1] * self[e431])
                - (right_dual_g2[2] * self[e412])
                - (self[e235] * other[e423])
                - (self[e315] * other[e431])
                - (self[e125] * other[e412]),
        )
    }
}
impl BulkContraction<Dipole> for AntiDipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       25       40        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g2[1] * self[e412]) + (right_dual_g1[0] * self[e321]) + (right_dual_g1[3] * self[e415]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g2[2] * self[e423]) + (right_dual_g1[1] * self[e321]) + (right_dual_g1[3] * self[e425]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g2[0] * self[e431]) + (right_dual_g1[2] * self[e321]) + (right_dual_g1[3] * self[e435]),
                -(right_dual_g0[0] * self[e415]) - (right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]) - (right_dual_g1[2] * self[e412]),
            ]) - (right_dual_g0.yzx() * self.group2().zxy()).with_w(right_dual_g1[1] * self[e431])
                - (right_dual_g2.zxy() * self.group0().yzx()).with_w(right_dual_g1[0] * self[e423]),
            // e5
            -(right_dual_g2[0] * self[e415])
                - (right_dual_g2[1] * self[e425])
                - (right_dual_g2[2] * self[e435])
                - (right_dual_g1[0] * self[e235])
                - (right_dual_g1[1] * self[e315])
                - (right_dual_g1[2] * self[e125]),
        )
    }
}
impl BulkContraction<DipoleInversion> for AntiDipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       25       41        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g1[0] * self[e321]) + (right_dual_g1[3] * self[e415]) + (right_dual_g2[1] * self[e412]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g1[1] * self[e321]) + (right_dual_g1[3] * self[e425]) + (right_dual_g2[2] * self[e423]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g1[2] * self[e321]) + (right_dual_g1[3] * self[e435]) + (right_dual_g2[0] * self[e431]),
                -(right_dual_g0[2] * self[e435]) - (right_dual_g1[0] * self[e423]) - (right_dual_g1[1] * self[e431]) - (right_dual_g1[2] * self[e412]),
            ]) - (right_dual_g0.yzx() * self.group2().zxy()).with_w(right_dual_g0[0] * self[e415])
                - (self.group0().yzx() * right_dual_g2.zxy()).with_w(right_dual_g0[1] * self[e425]),
            // e5
            -(right_dual_g1[0] * self[e235])
                - (right_dual_g1[1] * self[e315])
                - (right_dual_g1[2] * self[e125])
                - (right_dual_g2[0] * self[e415])
                - (right_dual_g2[1] * self[e425])
                - (right_dual_g2[2] * self[e435]),
        )
    }
}
impl BulkContraction<DualNum> for AntiDipoleInversion {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x2::from(-1.0);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(right_dual_g0[0]) * self.group0().with_w(self[e4]),
            // e15, e25, e35, e3215
            (self.group1().xyz() * right_dual_g0.xx().with_z(right_dual_g0[0])).with_w(0.0),
        )
    }
}
impl BulkContraction<FlatPoint> for AntiDipoleInversion {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[1] * self[e412]) + (right_dual_g0[3] * self[e415]),
                (right_dual_g0[2] * self[e423]) + (right_dual_g0[3] * self[e425]),
                (right_dual_g0[0] * self[e431]) + (right_dual_g0[3] * self[e435]),
                -(right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]),
            ]) - (right_dual_g0.zxyx() * self.group0().yzx().with_w(self[e415])),
        )
    }
}
impl BulkContraction<Flector> for AntiDipoleInversion {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[1] * self[e412]) + (right_dual_g0[3] * self[e415]),
                (right_dual_g0[2] * self[e423]) + (right_dual_g0[3] * self[e425]),
                (right_dual_g0[0] * self[e431]) + (right_dual_g0[3] * self[e435]),
                -(right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]),
            ]) - (right_dual_g0.zxyx() * self.group0().yzx().with_w(self[e415])),
        )
    }
}
impl BulkContraction<Line> for AntiDipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
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
impl BulkContraction<Motor> for AntiDipoleInversion {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       21        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(right_dual_g1[3]) * self.group0()).with_w(
                (right_dual_g1[3] * self[e4])
                    - (right_dual_g0[0] * self[e415])
                    - (right_dual_g0[1] * self[e425])
                    - (right_dual_g0[2] * self[e435])
                    - (right_dual_g1[0] * self[e423])
                    - (right_dual_g1[1] * self[e431])
                    - (right_dual_g1[2] * self[e412]),
            ),
            // e15, e25, e35, e3215
            (self.group1().xyz() * right_dual_g1.www()).with_w(0.0),
        )
    }
}
impl BulkContraction<MultiVector> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       49        0
    //    simd2        0        1        0
    //    simd3        4       12        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       44       67        0
    //  no simd       64      107        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g6 = other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_dual_g9 = other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g10 * self[e5])
                    + (right_dual_g9[0] * self[e1])
                    + (right_dual_g9[1] * self[e2])
                    + (right_dual_g9[2] * self[e3])
                    + (right_dual_g9[3] * self[e4])
                    + (self[e321] * other[e321])
                    - (self[e423] * other[e235])
                    - (self[e431] * other[e315])
                    - (self[e412] * other[e125])
                    - (self[e415] * other[e415])
                    - (self[e425] * other[e425])
                    - (self[e435] * other[e435])
                    - (self[e235] * other[e423])
                    - (self[e315] * other[e431])
                    - (self[e125] * other[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g7[2] * self[e315]) + (right_dual_g8[1] * self[e412]) + (right_dual_g6[0] * self[e321]) + (right_dual_g6[3] * self[e415]),
                (right_dual_g7[0] * self[e125]) + (right_dual_g8[2] * self[e423]) + (right_dual_g6[1] * self[e321]) + (right_dual_g6[3] * self[e425]),
                (right_dual_g7[1] * self[e235]) + (right_dual_g8[0] * self[e431]) + (right_dual_g6[2] * self[e321]) + (right_dual_g6[3] * self[e435]),
                -(right_dual_g7[0] * self[e415]) - (right_dual_g7[1] * self[e425]) - (right_dual_g7[2] * self[e435]) - (right_dual_g6[2] * self[e412]),
            ]) + (Simd32x4::from(right_dual_g0[1]) * self.group3().xyz().with_w(self[e4]))
                - (right_dual_g7.yzx() * self.group2().zxy()).with_w(right_dual_g6[1] * self[e431])
                - (right_dual_g8.zxy() * self.group0().yzx()).with_w(right_dual_g6[0] * self[e423]),
            // e5
            (right_dual_g0[1] * self[e5])
                - (right_dual_g8[0] * self[e415])
                - (right_dual_g8[1] * self[e425])
                - (right_dual_g8[2] * self[e435])
                - (right_dual_g6[0] * self[e235])
                - (right_dual_g6[1] * self[e315])
                - (right_dual_g6[2] * self[e125]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g9[2] * self[e315]) + (right_dual_g9[3] * self[e415]),
                (right_dual_g9[0] * self[e125]) + (right_dual_g9[3] * self[e425]),
                (right_dual_g9[1] * self[e235]) + (right_dual_g9[3] * self[e435]),
                -(right_dual_g9[1] * self[e425]) - (right_dual_g9[2] * self[e435]),
            ]) - (right_dual_g9.yzxx() * self.group2().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(right_dual_g10) * self.group1().xyz()) + (self.group0().zxy() * right_dual_g9.yzx()) - (self.group0().yzx() * right_dual_g9.zxy()),
            // e23, e31, e12
            (Simd32x3::from(right_dual_g10) * self.group2().xyz()) + (Simd32x3::from(right_dual_g9[3]) * self.group0()) - (Simd32x3::from(self[e321]) * right_dual_g9.xyz()),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual_g0[1]) * self.group1(),
            // e423, e431, e412
            Simd32x3::from(right_dual_g0[1]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(right_dual_g0[1]) * self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl BulkContraction<RoundPoint> for AntiDipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       12        0
    //    simd3        2        5        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       11       21        0
    //  no simd       24       43        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other[e4] * -1.0;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(right_dual_g1) * self.group1().xyz()) + (self.group0().zxy() * right_dual_g0.yzx()) - (self.group0().yzx() * right_dual_g0.zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual_g1 * self[e235]) + (right_dual_g0[3] * self[e423]),
                (right_dual_g1 * self[e315]) + (right_dual_g0[3] * self[e431]),
                (right_dual_g1 * self[e125]) + (right_dual_g0[3] * self[e412]),
                -(right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]),
            ]) - (right_dual_g0.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(right_dual_g0[3]) * self.group1().xyz().with_w(self[e4]))
                + (right_dual_g0.zxyx() * self.group2().yzx().with_w(self[e1]))
                + (right_dual_g0.yzx() * self.group2().zxy() * Simd32x3::from(-1.0))
                    .with_w((right_dual_g1 * self[e5]) + (right_dual_g0[1] * self[e2]) + (right_dual_g0[2] * self[e3])),
        )
    }
}
impl BulkContraction<Scalar> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
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
impl BulkContraction<VersorEven> for AntiDipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        2        4        0
    //    simd4        4        7        0
    // Totals...
    // yes simd       21       32        0
    //  no simd       37       61        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(right_dual_g2[3]) * self.group1().xyz()) + (self.group0().zxy() * right_dual_g3.yzx()) - (self.group0().yzx() * right_dual_g3.zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual_g2[3] * self[e235]) + (right_dual_g3[3] * self[e423]),
                (right_dual_g2[3] * self[e315]) + (right_dual_g3[3] * self[e431]),
                (right_dual_g2[3] * self[e125]) + (right_dual_g3[3] * self[e412]),
                -(right_dual_g3[1] * self[e425]) - (right_dual_g3[2] * self[e435]),
            ]) - (right_dual_g3.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(right_dual_g3[3]) * self.group1().xyz().with_w(self[e4]))
                + (right_dual_g3.zxyx() * self.group2().yzx().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual_g2[3] * self[e5]) + (right_dual_g3[1] * self[e2]) + (right_dual_g3[2] * self[e3])
                        - (right_dual_g0[0] * self[e235])
                        - (right_dual_g0[1] * self[e315])
                        - (right_dual_g0[2] * self[e125])
                        - (right_dual_g1[0] * self[e415])
                        - (right_dual_g1[1] * self[e425])
                        - (right_dual_g1[2] * self[e435])
                        - (right_dual_g1[3] * self[e321])
                        - (right_dual_g2[1] * self[e431])
                        - (right_dual_g2[2] * self[e412]),
                )
                - (right_dual_g3.yzx() * self.group2().zxy()).with_w(right_dual_g2[0] * self[e423]),
        )
    }
}
impl BulkContraction<VersorOdd> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       25        0
    //    simd3        0        4        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       21       34        0
    //  no simd       30       57        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual_g0[3]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual_g0[3]) * self.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(right_dual_g0[3]) * self.group2().xyz()).with_w(
                (right_dual_g0[3] * self[e4])
                    - (right_dual_g0[0] * self[e415])
                    - (right_dual_g0[1] * self[e425])
                    - (right_dual_g0[2] * self[e435])
                    - (right_dual_g1[0] * self[e423])
                    - (right_dual_g1[1] * self[e431])
                    - (right_dual_g1[2] * self[e412]),
            ),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g0[3] * self[e1]) + (right_dual_g1[0] * self[e321]) + (right_dual_g1[3] * self[e415]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g0[3] * self[e2]) + (right_dual_g1[1] * self[e321]) + (right_dual_g1[3] * self[e425]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g0[3] * self[e3]) + (right_dual_g1[2] * self[e321]) + (right_dual_g1[3] * self[e435]),
                -(right_dual_g1[0] * self[e235]) - (right_dual_g1[1] * self[e315]) - (right_dual_g1[2] * self[e125]) - (right_dual_g2[2] * self[e435]),
            ]) + (self.group0().zxy() * right_dual_g2.yzx()).with_w(right_dual_g0[3] * self[e5])
                - (right_dual_g2.zxyx() * self.group0().yzx().with_w(self[e415]))
                - (right_dual_g0.yzx() * self.group2().zxy()).with_w(right_dual_g2[1] * self[e425]),
        )
    }
}
impl std::ops::Div<BulkContractionInfix> for AntiDualNum {
    type Output = BulkContractionInfixPartial<AntiDualNum>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       19        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0().xx().with_zw(self[e3215], self[scalar]) * (other.group0() * Simd32x3::from(-1.0)).with_w(right_dual_g2[3]),
            // e15, e25, e35, e3215
            Simd32x4::from(self[e3215]) * (other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])).xyz().with_w(right_dual_g2[3]),
        )
    }
}
impl BulkContraction<AntiDipoleInversion> for AntiDualNum {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       22        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215]) * (other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).xyz().with_w(other[e4] * -1.0) * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            Simd32x4::from(self[e3215]) * other.group0().with_w(other[e321] * -1.0) * Simd32x4::from(-1.0),
        )
    }
}
impl BulkContraction<AntiDualNum> for AntiDualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(other[scalar]) * self.group0())
    }
}
impl BulkContraction<AntiFlatPoint> for AntiDualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        4        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e3215] * other[e321] * -1.0, 1.0]) * Simd32x2::from([-1.0, 0.0]))
    }
}
impl BulkContraction<AntiFlector> for AntiDualNum {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       16        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            ((other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).xyz() * self.group0().xx().with_z(self[e3215]) * Simd32x3::from(-1.0)).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e3215] * other[e321] * -1.0) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        )
    }
}
impl BulkContraction<AntiLine> for AntiDualNum {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            (Simd32x3::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0]) * self.group0().xx().with_z(self[e3215])).with_w(0.0),
        )
    }
}
impl BulkContraction<AntiMotor> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(right_dual_g0[3] * self[scalar]),
            // e15, e25, e35, e3215
            right_dual_g0 * Simd32x4::from(self[e3215]),
        )
    }
}
impl BulkContraction<AntiPlane> for AntiDualNum {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            (other.group0().xyz() * self.group0().xx().with_z(self[e3215]) * Simd32x3::from(-1.0)).with_w(0.0),
        )
    }
}
impl BulkContraction<Circle> for AntiDualNum {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e3215]) * other.group0().with_w(other[e321] * -1.0) * Simd32x4::from(-1.0))
    }
}
impl BulkContraction<CircleRotor> for AntiDualNum {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e3215]) * other.group0().with_w(other[e321] * -1.0) * Simd32x4::from(-1.0))
    }
}
impl BulkContraction<Dipole> for AntiDualNum {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(self[e3215]) * other.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            Simd32x3::from(self[e3215]) * (other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])).xyz(),
        )
    }
}
impl BulkContraction<DipoleInversion> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       14        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(self[e3215]) * (other.group0() * Simd32x3::from(-1.0)).with_w(other[e1234]),
            // e15, e25, e35, e3215
            ((other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])).xyz() * self.group0().xx().with_z(self[e3215])).with_w(0.0),
        )
    }
}
impl BulkContraction<MultiVector> for AntiDualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd2        0        1        0
    //    simd3        0        7        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1       17        0
    //  no simd        1       41        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(right_dual_g0[1] * self[scalar]) + (self[e3215] * other[e1234]), 0.0]),
            // e1, e2, e3, e4
            (other.group7() * self.group0().xx().with_z(self[e3215]) * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            self[e3215] * other[e321],
            // e15, e25, e35, e45
            ((other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])).xyz() * self.group0().xx().with_z(self[e3215])).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e3215]) * other.group4() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e3215] * other[e4] * -1.0) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[e3215]) * (other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(right_dual_g0[1] * self[e3215]),
            // e1234
            0.0,
        )
    }
}
impl BulkContraction<RoundPoint> for AntiDualNum {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215]) * (other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).xyz().with_w(other[e4] * -1.0) * Simd32x4::from(-1.0),
        )
    }
}
impl BulkContraction<Scalar> for AntiDualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(other[scalar]) * self.group0())
    }
}
impl BulkContraction<Sphere> for AntiDualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e3215] * other[e1234])
    }
}
impl BulkContraction<VersorEven> for AntiDualNum {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       26        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215]) * (other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).xyz().with_w(other[e4] * -1.0) * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            Simd32x4::from(self[e3215]) * (other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).xyz().with_w(other[e321] * -1.0) * Simd32x4::from(-1.0),
        )
    }
}
impl BulkContraction<VersorOdd> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1       18        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0().xx().with_zw(self[e3215], (right_dual_g0[3] * self[scalar]) + (self[e3215] * other[e1234])) * right_dual_g0.xyz().with_w(1.0),
            // e15, e25, e35, e3215
            Simd32x4::from(self[e3215]) * (other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])).xyz().with_w(right_dual_g0[3]),
        )
    }
}
impl std::ops::Div<BulkContractionInfix> for AntiFlatPoint {
    type Output = BulkContractionInfixPartial<AntiFlatPoint>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for AntiFlatPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        8       23        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g1[0] * self[e321]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g1[1] * self[e321]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g1[2] * self[e321]),
                -(right_dual_g1[1] * self[e315]) - (right_dual_g1[2] * self[e125]),
            ]) - (right_dual_g0.yzx() * self.group0().zxy()).with_w(right_dual_g1[0] * self[e235]),
        )
    }
}
impl BulkContraction<AntiDipoleInversion> for AntiFlatPoint {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        2        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        9       24        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g3 = other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            ((self.group0().xyz() * (other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).www()) - (right_dual_g3.xyz() * self.group0().www()))
                .with_w((other[e321] * self[e321]) - (other[e423] * self[e235]) - (other[e431] * self[e315]) - (other[e412] * self[e125])),
            // e15, e25, e35, e3215
            ((right_dual_g3.zxy() * self.group0().yzx()) - (right_dual_g3.yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl BulkContraction<AntiDualNum> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl BulkContraction<AntiFlatPoint> for AntiFlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e321] * self[e321])
    }
}
impl BulkContraction<AntiFlector> for AntiFlatPoint {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        3       19        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321]) * right_dual_g1.xyz().with_w(other[e321] * -1.0) * Simd32x4::from(-1.0),
            // e15, e25, e35, e3215
            ((right_dual_g1.zxy() * self.group0().yzx()) - (right_dual_g1.yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl BulkContraction<AntiLine> for AntiFlatPoint {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (right_dual_g0 * Simd32x3::from(self[e321])).with_w(-(right_dual_g0[0] * self[e235]) - (right_dual_g0[1] * self[e315]) - (right_dual_g0[2] * self[e125])),
        )
    }
}
impl BulkContraction<AntiMotor> for AntiFlatPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       14        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual_g0[3]) * self.group0(),
            // e1, e2, e3, e5
            (right_dual_g0.xyz() * self.group0().www()).with_w(-(right_dual_g0[0] * self[e235]) - (right_dual_g0[1] * self[e315]) - (right_dual_g0[2] * self[e125])),
        )
    }
}
impl BulkContraction<AntiPlane> for AntiFlatPoint {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        5        0
    //  no simd        3       16        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(self[e321]) * right_dual_g0.xyz() * Simd32x3::from(-1.0),
            // e15, e25, e35
            (right_dual_g0.zxy() * self.group0().yzx()) - (right_dual_g0.yzx() * self.group0().zxy()),
        )
    }
}
impl BulkContraction<Circle> for AntiFlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e321] * other[e321]) - (self[e235] * other[e423]) - (self[e315] * other[e431]) - (self[e125] * other[e412]),
        )
    }
}
impl BulkContraction<CircleRotor> for AntiFlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e321] * other[e321]) - (self[e235] * other[e423]) - (self[e315] * other[e431]) - (self[e125] * other[e412]),
        )
    }
}
impl BulkContraction<Dipole> for AntiFlatPoint {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        8       19        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g1[0] * self[e321]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g1[1] * self[e321]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g1[2] * self[e321]),
                -(right_dual_g1[1] * self[e315]) - (right_dual_g1[2] * self[e125]),
            ]) - (right_dual_g0.yzx() * self.group0().zxy()).with_w(right_dual_g1[0] * self[e235]),
        )
    }
}
impl BulkContraction<DipoleInversion> for AntiFlatPoint {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       19        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g1[0] * self[e321]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g1[1] * self[e321]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g1[2] * self[e321]),
                -(right_dual_g1[1] * self[e315]) - (right_dual_g1[2] * self[e125]),
            ]) - (self.group0().zxyx() * right_dual_g0.yzx().with_w(right_dual_g1[0])),
        )
    }
}
impl BulkContraction<MultiVector> for AntiFlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd2        0        1        0
    //    simd3        2        9        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       21        0
    //  no simd       19       46        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g6 = other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g9 = other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self[e321] * other[e321]) - (self[e235] * other[e423]) - (self[e315] * other[e431]) - (self[e125] * other[e412]), 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(self[e321]) * right_dual_g6.xyz()).with_w(0.0) + (right_dual_g7.zxy() * self.group0().yzx()).with_w(0.0)
                - (right_dual_g7.yzx() * self.group0().zxy()).with_w(0.0),
            // e5
            -(right_dual_g6[0] * self[e235]) - (right_dual_g6[1] * self[e315]) - (right_dual_g6[2] * self[e125]),
            // e15, e25, e35, e45
            ((right_dual_g9.zxy() * self.group0().yzx()) - (right_dual_g9.yzx() * self.group0().zxy())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from(other[e4] * -1.0) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * right_dual_g9.xyz()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(right_dual_g0[1] * self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(right_dual_g0[1]) * self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl BulkContraction<RoundPoint> for AntiFlatPoint {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        2        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        6       17        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiLine::from_groups(
            // e23, e31, e12
            (Simd32x3::from(other[e4] * -1.0) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * right_dual_g0.xyz()),
            // e15, e25, e35
            (right_dual_g0.zxy() * self.group0().yzx()) - (right_dual_g0.yzx() * self.group0().zxy()),
        )
    }
}
impl BulkContraction<Scalar> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl BulkContraction<VersorEven> for AntiFlatPoint {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        1        3        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        9       28        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (self.group0().xyz() * (other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).www())
                .with_w((self[e321] * other[e321]) - (right_dual_g0[1] * self[e315]) - (right_dual_g0[2] * self[e125]))
                - (self.group0().wwwx() * right_dual_g3.xyz().with_w(right_dual_g0[0])),
            // e15, e25, e35, e3215
            ((right_dual_g3.zxy() * self.group0().yzx()) - (right_dual_g3.yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl BulkContraction<VersorOdd> for AntiFlatPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        8       24        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual_g0[3]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g1[0] * self[e321]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g1[1] * self[e321]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g1[2] * self[e321]),
                -(right_dual_g1[1] * self[e315]) - (right_dual_g1[2] * self[e125]),
            ]) - (self.group0().zxyx() * right_dual_g0.yzx().with_w(right_dual_g1[0])),
        )
    }
}
impl std::ops::Div<BulkContractionInfix> for AntiFlector {
    type Output = BulkContractionInfixPartial<AntiFlector>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd3        0        3        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       16        0
    //  no simd       12       31        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual_g2[3]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g1[0] * self[e321]) + (right_dual_g2[3] * self[e1]),
                (right_dual_g1[1] * self[e321]) + (right_dual_g2[3] * self[e2]),
                (right_dual_g1[2] * self[e321]) + (right_dual_g2[3] * self[e3]),
                -(right_dual_g1[1] * self[e315]) - (right_dual_g1[2] * self[e125]),
            ]) + (right_dual_g0.zxy() * self.group0().yzx()).with_w(right_dual_g2[3] * self[e5])
                - (right_dual_g0.yzx() * self.group0().zxy()).with_w(right_dual_g1[0] * self[e235]),
        )
    }
}
impl BulkContraction<AntiDipoleInversion> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        8        0
    //    simd3        1        4        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        8       14        0
    //  no simd       16       28        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(
                (right_dual_g2[3] * self[e5]) + (right_dual_g3[1] * self[e2]) + (right_dual_g3[2] * self[e3]) + (other[e321] * self[e321])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
            ) + (self.group0().xyz() * right_dual_g2.www()).with_w(right_dual_g3[0] * self[e1])
                - (right_dual_g3.xyz() * self.group0().www()).with_w(other[e423] * self[e235]),
            // e15, e25, e35, e3215
            ((right_dual_g3.zxy() * self.group0().yzx()) - (right_dual_g3.yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl BulkContraction<AntiDualNum> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl BulkContraction<AntiFlatPoint> for AntiFlector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e321] * self[e321])
    }
}
impl BulkContraction<AntiFlector> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        1        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        6       20        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (right_dual_g1.xyz() * self.group0().www() * Simd32x3::from(-1.0))
                .with_w((right_dual_g1[0] * self[e1]) + (right_dual_g1[1] * self[e2]) + (right_dual_g1[2] * self[e3]) + (other[e321] * self[e321])),
            // e15, e25, e35, e3215
            ((right_dual_g1.zxy() * self.group0().yzx()) - (right_dual_g1.yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl BulkContraction<AntiLine> for AntiFlector {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (right_dual_g0 * Simd32x3::from(self[e321])).with_w(-(right_dual_g0[0] * self[e235]) - (right_dual_g0[1] * self[e315]) - (right_dual_g0[2] * self[e125])),
        )
    }
}
impl BulkContraction<AntiMotor> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        6       18        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual_g0[3]) * self.group0(),
            // e1, e2, e3, e5
            (right_dual_g0 * self.group0().www().with_w(self[e5]))
                + (self.group1().xyz() * right_dual_g0.www()).with_w(-(right_dual_g0[0] * self[e235]) - (right_dual_g0[1] * self[e315]) - (right_dual_g0[2] * self[e125])),
        )
    }
}
impl BulkContraction<AntiPlane> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        1        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        5       19        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (right_dual_g0.xyz() * self.group0().www() * Simd32x3::from(-1.0))
                .with_w((right_dual_g0[0] * self[e1]) + (right_dual_g0[1] * self[e2]) + (right_dual_g0[2] * self[e3])),
            // e15, e25, e35, e3215
            ((right_dual_g0.zxy() * self.group0().yzx()) - (right_dual_g0.yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl BulkContraction<Circle> for AntiFlector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e321] * other[e321]) - (self[e235] * other[e423]) - (self[e315] * other[e431]) - (self[e125] * other[e412]),
        )
    }
}
impl BulkContraction<CircleRotor> for AntiFlector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e321] * other[e321]) - (self[e235] * other[e423]) - (self[e315] * other[e431]) - (self[e125] * other[e412]),
        )
    }
}
impl BulkContraction<Dipole> for AntiFlector {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        8       19        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g1[0] * self[e321]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g1[1] * self[e321]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g1[2] * self[e321]),
                -(right_dual_g1[1] * self[e315]) - (right_dual_g1[2] * self[e125]),
            ]) - (right_dual_g0.yzx() * self.group0().zxy()).with_w(right_dual_g1[0] * self[e235]),
        )
    }
}
impl BulkContraction<DipoleInversion> for AntiFlector {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       19        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g1[0] * self[e321]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g1[1] * self[e321]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g1[2] * self[e321]),
                -(right_dual_g1[1] * self[e315]) - (right_dual_g1[2] * self[e125]),
            ]) - (self.group0().zxyx() * right_dual_g0.yzx().with_w(right_dual_g1[0])),
        )
    }
}
impl BulkContraction<MultiVector> for AntiFlector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       14        0
    //    simd2        0        1        0
    //    simd3        2       10        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       15       27        0
    //  no simd       28       54        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g6 = other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g9 = other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g10 * self[e5]) + (right_dual_g9[0] * self[e1]) + (right_dual_g9[1] * self[e2]) + (right_dual_g9[2] * self[e3]) + (self[e321] * other[e321])
                    - (self[e235] * other[e423])
                    - (self[e315] * other[e431])
                    - (self[e125] * other[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(right_dual_g0[1]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * right_dual_g6.xyz()).with_w(0.0)
                + (right_dual_g7.zxy() * self.group0().yzx()).with_w(0.0)
                - (right_dual_g7.yzx() * self.group0().zxy()).with_w(0.0),
            // e5
            (right_dual_g0[1] * self[e5]) - (right_dual_g6[0] * self[e235]) - (right_dual_g6[1] * self[e315]) - (right_dual_g6[2] * self[e125]),
            // e15, e25, e35, e45
            ((right_dual_g9.zxy() * self.group0().yzx()) - (right_dual_g9.yzx() * self.group0().zxy())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from(right_dual_g10) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * right_dual_g9.xyz()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(right_dual_g0[1] * self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(right_dual_g0[1]) * self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl BulkContraction<RoundPoint> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        5        0
    //    simd3        1        3        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        4       11        0
    //  no simd        9       26        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other[e4] * -1.0;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x4::from([self[e321], self[e321], self[e321], 1.0])
                * right_dual_g0.xyz().with_w((right_dual_g1 * self[e5]) + (right_dual_g0[1] * self[e2]) + (right_dual_g0[2] * self[e3]))
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
                + (Simd32x3::from(right_dual_g1) * self.group0().xyz()).with_w(right_dual_g0[0] * self[e1]),
            // e15, e25, e35, e3215
            ((right_dual_g0.zxy() * self.group0().yzx()) - (right_dual_g0.yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl BulkContraction<Scalar> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl BulkContraction<VersorEven> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        3        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        8       14        0
    //  no simd       16       32        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(
                (right_dual_g2[3] * self[e5]) + (right_dual_g3[1] * self[e2]) + (right_dual_g3[2] * self[e3]) + (self[e321] * other[e321])
                    - (right_dual_g0[1] * self[e315])
                    - (right_dual_g0[2] * self[e125]),
            ) + (self.group0().xyz() * right_dual_g2.www()).with_w(right_dual_g3[0] * self[e1])
                - (self.group0().wwwx() * right_dual_g3.xyz().with_w(right_dual_g0[0])),
            // e15, e25, e35, e3215
            ((right_dual_g3.zxy() * self.group0().yzx()) - (right_dual_g3.yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl BulkContraction<VersorOdd> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        6       13        0
    //  no simd       12       28        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual_g0[3]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[3] * self[e1]) + (right_dual_g1[0] * self[e321]),
                (right_dual_g0[3] * self[e2]) + (right_dual_g1[1] * self[e321]),
                (right_dual_g0[3] * self[e3]) + (right_dual_g1[2] * self[e321]),
                -(right_dual_g1[1] * self[e315]) - (right_dual_g1[2] * self[e125]),
            ]) + (right_dual_g0.zxyw() * self.group0().yzx().with_w(self[e5]))
                - (self.group0().zxyx() * right_dual_g0.yzx().with_w(right_dual_g1[0])),
        )
    }
}
impl std::ops::Div<BulkContractionInfix> for AntiLine {
    type Output = BulkContractionInfixPartial<AntiLine>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        5       23        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(right_dual_g2[3]) * self.group0()).with_w(
                -(right_dual_g0[0] * self[e15])
                    - (right_dual_g0[1] * self[e25])
                    - (right_dual_g0[2] * self[e35])
                    - (right_dual_g1[0] * self[e23])
                    - (right_dual_g1[1] * self[e31])
                    - (right_dual_g1[2] * self[e12]),
            ),
            // e15, e25, e35, e3215
            (self.group1() * right_dual_g2.www()).with_w(0.0),
        )
    }
}
impl BulkContraction<AntiDipoleInversion> for AntiLine {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       20        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                -(right_dual_g2[3] * self[e15]) - (right_dual_g3[1] * self[e12]),
                -(right_dual_g2[3] * self[e25]) - (right_dual_g3[2] * self[e23]),
                -(right_dual_g2[3] * self[e35]) - (right_dual_g3[0] * self[e31]),
                (right_dual_g3[1] * self[e25]) + (right_dual_g3[2] * self[e35]),
            ]) + (right_dual_g3.zxyx() * self.group0().yzx().with_w(self[e15])),
        )
    }
}
impl BulkContraction<AntiDualNum> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(other[scalar]) * self.group1(),
        )
    }
}
impl BulkContraction<AntiFlector> for AntiLine {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd3        0        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        5       16        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (right_dual_g1.zxyx() * self.group0().yzx().with_w(self[e15]))
                + (self.group0().zxy() * right_dual_g1.yzx() * Simd32x3::from(-1.0)).with_w((right_dual_g1[1] * self[e25]) + (right_dual_g1[2] * self[e35])),
        )
    }
}
impl BulkContraction<AntiLine> for AntiLine {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        Scalar::from_groups(/* scalar */ -(right_dual_g0[0] * self[e23]) - (right_dual_g0[1] * self[e31]) - (right_dual_g0[2] * self[e12]))
    }
}
impl BulkContraction<AntiMotor> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       13        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(right_dual_g0[3]) * self.group0()).with_w(-(right_dual_g0[0] * self[e23]) - (right_dual_g0[1] * self[e31]) - (right_dual_g0[2] * self[e12])),
            // e15, e25, e35, e3215
            (self.group1() * right_dual_g0.www()).with_w(0.0),
        )
    }
}
impl BulkContraction<AntiPlane> for AntiLine {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd3        0        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        5       16        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (right_dual_g0.zxyx() * self.group0().yzx().with_w(self[e15]))
                + (self.group0().zxy() * right_dual_g0.yzx() * Simd32x3::from(-1.0)).with_w((right_dual_g0[1] * self[e25]) + (right_dual_g0[2] * self[e35])),
        )
    }
}
impl BulkContraction<Dipole> for AntiLine {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       13        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Scalar::from_groups(
            // scalar
            -(right_dual_g0[0] * self[e15])
                - (right_dual_g0[1] * self[e25])
                - (right_dual_g0[2] * self[e35])
                - (right_dual_g1[0] * self[e23])
                - (right_dual_g1[1] * self[e31])
                - (right_dual_g1[2] * self[e12]),
        )
    }
}
impl BulkContraction<DipoleInversion> for AntiLine {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       13        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Scalar::from_groups(
            // scalar
            -(right_dual_g0[0] * self[e15])
                - (right_dual_g0[1] * self[e25])
                - (right_dual_g0[2] * self[e35])
                - (right_dual_g1[0] * self[e23])
                - (right_dual_g1[1] * self[e31])
                - (right_dual_g1[2] * self[e12]),
        )
    }
}
impl BulkContraction<MultiVector> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       10        0
    //    simd2        0        1        0
    //    simd3        0        6        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       15       38        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g6 = other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g9 = other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(right_dual_g7[0] * self[e15])
                    - (right_dual_g7[1] * self[e25])
                    - (right_dual_g7[2] * self[e35])
                    - (right_dual_g6[0] * self[e23])
                    - (right_dual_g6[1] * self[e31])
                    - (right_dual_g6[2] * self[e12]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (self.group0().yzx() * right_dual_g9.zxy()).with_w(0.0)
                - (Simd32x3::from(other[e4] * -1.0) * self.group1()).with_w(0.0)
                - (self.group0().zxy() * right_dual_g9.yzx()).with_w(0.0),
            // e5
            (right_dual_g9[0] * self[e15]) + (right_dual_g9[1] * self[e25]) + (right_dual_g9[2] * self[e35]),
            // e15, e25, e35, e45
            (self.group1() * right_dual_g0.yy().with_z(right_dual_g0[1])).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(right_dual_g0[1]) * self.group0(),
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
impl BulkContraction<RoundPoint> for AntiLine {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       17        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other[e4] * -1.0;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                -(right_dual_g1 * self[e15]) - (right_dual_g0[1] * self[e12]),
                -(right_dual_g1 * self[e25]) - (right_dual_g0[2] * self[e23]),
                -(right_dual_g1 * self[e35]) - (right_dual_g0[0] * self[e31]),
                (right_dual_g0[1] * self[e25]) + (right_dual_g0[2] * self[e35]),
            ]) + (right_dual_g0.zxyx() * self.group0().yzx().with_w(self[e15])),
        )
    }
}
impl BulkContraction<Scalar> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(other[scalar]) * self.group1(),
        )
    }
}
impl BulkContraction<VersorEven> for AntiLine {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       20        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2 = other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                -(right_dual_g2[3] * self[e15]) - (right_dual_g3[1] * self[e12]),
                -(right_dual_g2[3] * self[e25]) - (right_dual_g3[2] * self[e23]),
                -(right_dual_g2[3] * self[e35]) - (right_dual_g3[0] * self[e31]),
                (right_dual_g3[1] * self[e25]) + (right_dual_g3[2] * self[e35]),
            ]) + (right_dual_g3.zxyx() * self.group0().yzx().with_w(self[e15])),
        )
    }
}
impl BulkContraction<VersorOdd> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       20        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(right_dual_g0[3]) * self.group0()).with_w(
                -(right_dual_g0[0] * self[e15])
                    - (right_dual_g0[1] * self[e25])
                    - (right_dual_g0[2] * self[e35])
                    - (right_dual_g1[0] * self[e23])
                    - (right_dual_g1[1] * self[e31])
                    - (right_dual_g1[2] * self[e12]),
            ),
            // e15, e25, e35, e3215
            (self.group1() * right_dual_g0.www()).with_w(0.0),
        )
    }
}
impl std::ops::Div<BulkContractionInfix> for AntiMotor {
    type Output = BulkContractionInfixPartial<AntiMotor>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd3        2        5        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        8       15        0
    //  no simd       12       31        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            ((right_dual_g0 * self.group1().www()) + (self.group0().xyz() * right_dual_g2.www())).with_w(
                (right_dual_g2[3] * self[scalar])
                    - (right_dual_g0[0] * self[e15])
                    - (right_dual_g0[1] * self[e25])
                    - (right_dual_g0[2] * self[e35])
                    - (right_dual_g1[0] * self[e23])
                    - (right_dual_g1[1] * self[e31])
                    - (right_dual_g1[2] * self[e12]),
            ),
            // e15, e25, e35, e3215
            ((Simd32x3::from(right_dual_g2[3]) * self.group1().xyz()) + (Simd32x3::from(self[e3215]) * right_dual_g1.xyz())).with_w(right_dual_g2[3] * self[e3215]),
        )
    }
}
impl BulkContraction<AntiDipoleInversion> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd3        0        1        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        6       16        0
    //  no simd       12       33        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215]) * right_dual_g3.xyz().with_w(right_dual_g2[3]) * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            Simd32x4::from([
                -(right_dual_g2[3] * self[e15]) - (right_dual_g3[1] * self[e12]),
                -(right_dual_g2[3] * self[e25]) - (right_dual_g3[2] * self[e23]),
                -(right_dual_g2[3] * self[e35]) - (right_dual_g3[0] * self[e31]),
                (right_dual_g3[1] * self[e25]) + (right_dual_g3[2] * self[e35]),
            ]) + (right_dual_g3.zxyx() * self.group0().yzx().with_w(self[e15]))
                - (other.group0() * self.group1().www()).with_w(other[e321] * self[e3215] * -1.0),
        )
    }
}
impl BulkContraction<AntiDualNum> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * self.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl BulkContraction<AntiFlatPoint> for AntiMotor {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        4        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([other[e321] * self[e3215] * -1.0, 1.0]) * Simd32x2::from([-1.0, 0.0]))
    }
}
impl BulkContraction<AntiFlector> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd3        0        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        9       21        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (right_dual_g1.xyz() * self.group1().www() * Simd32x3::from(-1.0)).with_w(0.0),
            // e1, e2, e3, e5
            (right_dual_g1.zxyx() * self.group0().yzx().with_w(self[e15])) + Simd32x3::from(0.0).with_w((right_dual_g1[1] * self[e25]) + (right_dual_g1[2] * self[e35]))
                - (right_dual_g1.yzx() * self.group0().zxy()).with_w(other[e321] * self[e3215] * -1.0),
        )
    }
}
impl BulkContraction<AntiLine> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(right_dual_g0[0] * self[e23]) - (right_dual_g0[1] * self[e31]) - (right_dual_g0[2] * self[e12])),
            // e15, e25, e35, e3215
            (right_dual_g0 * self.group1().www()).with_w(0.0),
        )
    }
}
impl BulkContraction<AntiMotor> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        1        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        6       18        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(right_dual_g0[3]) * self.group0().xyz())
                .with_w((right_dual_g0[3] * self[scalar]) - (right_dual_g0[0] * self[e23]) - (right_dual_g0[1] * self[e31]) - (right_dual_g0[2] * self[e12])),
            // e15, e25, e35, e3215
            ((Simd32x3::from(right_dual_g0[3]) * self.group1().xyz()) + (Simd32x3::from(self[e3215]) * right_dual_g0.xyz())).with_w(right_dual_g0[3] * self[e3215]),
        )
    }
}
impl BulkContraction<AntiPlane> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd3        0        4        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        5       22        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (right_dual_g0.xyz() * self.group1().www() * Simd32x3::from(-1.0)).with_w(0.0),
            // e1, e2, e3, e5
            (right_dual_g0.zxyx() * self.group0().yzx().with_w(self[e15]))
                + (right_dual_g0.yzx() * self.group0().zxy() * Simd32x3::from(-1.0)).with_w((right_dual_g0[1] * self[e25]) + (right_dual_g0[2] * self[e35])),
        )
    }
}
impl BulkContraction<Circle> for AntiMotor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (other.group0() * self.group1().www() * Simd32x3::from(-1.0)).with_w(self[e3215] * other[e321]),
        )
    }
}
impl BulkContraction<CircleRotor> for AntiMotor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e3215]) * other.group0().with_w(other[e321] * -1.0) * Simd32x4::from(-1.0))
    }
}
impl BulkContraction<Dipole> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       19        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (right_dual_g0 * Simd32x3::from(self[e3215])).with_w(
                -(right_dual_g0[0] * self[e15])
                    - (right_dual_g0[1] * self[e25])
                    - (right_dual_g0[2] * self[e35])
                    - (right_dual_g1[0] * self[e23])
                    - (right_dual_g1[1] * self[e31])
                    - (right_dual_g1[2] * self[e12]),
            ),
            // e15, e25, e35, e3215
            (right_dual_g1.xyz() * self.group1().www()).with_w(0.0),
        )
    }
}
impl BulkContraction<DipoleInversion> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        0
    //    simd3        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       20        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (right_dual_g0 * Simd32x3::from(self[e3215])).with_w(
                (self[e3215] * other[e1234])
                    - (right_dual_g0[0] * self[e15])
                    - (right_dual_g0[1] * self[e25])
                    - (right_dual_g0[2] * self[e35])
                    - (right_dual_g1[0] * self[e23])
                    - (right_dual_g1[1] * self[e31])
                    - (right_dual_g1[2] * self[e12]),
            ),
            // e15, e25, e35, e3215
            (right_dual_g1.xyz() * self.group1().www()).with_w(0.0),
        )
    }
}
impl BulkContraction<MultiVector> for AntiMotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       15        0
    //    simd2        0        1        0
    //    simd3        2       11        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       15       30        0
    //  no simd       28       62        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g6 = other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g9 = other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g0[1] * self[scalar]) + (self[e3215] * other[e1234])
                    - (right_dual_g7[0] * self[e15])
                    - (right_dual_g7[1] * self[e25])
                    - (right_dual_g7[2] * self[e35])
                    - (right_dual_g6[0] * self[e23])
                    - (right_dual_g6[1] * self[e31])
                    - (right_dual_g6[2] * self[e12]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (right_dual_g9.zxy() * self.group0().yzx()).with_w(0.0)
                - (Simd32x3::from(right_dual_g10) * self.group1().xyz()).with_w(0.0)
                - (Simd32x3::from(self[e3215]) * other.group7()).with_w(0.0)
                - (right_dual_g9.yzx() * self.group0().zxy()).with_w(0.0),
            // e5
            (right_dual_g9[0] * self[e15]) + (right_dual_g9[1] * self[e25]) + (right_dual_g9[2] * self[e35]) + (self[e3215] * other[e321]),
            // e15, e25, e35, e45
            ((Simd32x3::from(right_dual_g0[1]) * self.group1().xyz()) + (Simd32x3::from(self[e3215]) * right_dual_g6.xyz())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (right_dual_g7 * Simd32x3::from(self[e3215])) + (Simd32x3::from(right_dual_g0[1]) * self.group0().xyz()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(right_dual_g10 * self[e3215]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[e3215]) * right_dual_g9.xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(right_dual_g0[1] * self[e3215]),
            // e1234
            0.0,
        )
    }
}
impl BulkContraction<RoundPoint> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        8       25        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other[e4] * -1.0;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215]) * right_dual_g0.xyz().with_w(right_dual_g1) * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            Simd32x4::from([
                -(right_dual_g1 * self[e15]) - (right_dual_g0[1] * self[e12]),
                -(right_dual_g1 * self[e25]) - (right_dual_g0[2] * self[e23]),
                -(right_dual_g1 * self[e35]) - (right_dual_g0[0] * self[e31]),
                (right_dual_g0[1] * self[e25]) + (right_dual_g0[2] * self[e35]),
            ]) + (right_dual_g0.zxyx() * self.group0().yzx().with_w(self[e15])),
        )
    }
}
impl BulkContraction<Scalar> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * self.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl BulkContraction<Sphere> for AntiMotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e3215] * other[e1234])
    }
}
impl BulkContraction<VersorEven> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd3        0        1        0
    //    simd4        2        6        0
    // Totals...
    // yes simd        6       17        0
    //  no simd       12       37        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215]) * right_dual_g3.xyz().with_w(right_dual_g2[3]) * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            Simd32x4::from([
                -(right_dual_g0[0] * self[e3215]) - (right_dual_g2[3] * self[e15]),
                -(right_dual_g0[1] * self[e3215]) - (right_dual_g2[3] * self[e25]),
                -(right_dual_g0[2] * self[e3215]) - (right_dual_g2[3] * self[e35]),
                (right_dual_g3[1] * self[e25]) + (right_dual_g3[2] * self[e35]),
            ]) + (right_dual_g3.zxyx() * self.group0().yzx().with_w(self[e15]))
                - (right_dual_g3.yzx() * self.group0().zxy()).with_w(self[e3215] * other[e321] * -1.0),
        )
    }
}
impl BulkContraction<VersorOdd> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        2        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       16       29        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x4::from(right_dual_g0[3]) * self.group0())
                + (Simd32x4::from(self[e3215]) * right_dual_g0.xyz().with_w(other[e1234]))
                + Simd32x3::from(0.0).with_w(
                    -(right_dual_g0[0] * self[e15])
                        - (right_dual_g0[1] * self[e25])
                        - (right_dual_g0[2] * self[e35])
                        - (right_dual_g1[0] * self[e23])
                        - (right_dual_g1[1] * self[e31])
                        - (right_dual_g1[2] * self[e12]),
                ),
            // e15, e25, e35, e3215
            ((Simd32x3::from(right_dual_g0[3]) * self.group1().xyz()) + (Simd32x3::from(self[e3215]) * right_dual_g1.xyz())).with_w(right_dual_g0[3] * self[e3215]),
        )
    }
}
impl std::ops::Div<BulkContractionInfix> for AntiPlane {
    type Output = BulkContractionInfixPartial<AntiPlane>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl BulkContraction<AntiDipoleInversion> for AntiPlane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g3 = other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(
            // scalar
            (right_dual_g3[0] * self[e1]) + (right_dual_g3[1] * self[e2]) + (right_dual_g3[2] * self[e3]) - (other[e4] * self[e5]),
        )
    }
}
impl BulkContraction<AntiDualNum> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl BulkContraction<AntiFlector> for AntiPlane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(/* scalar */ (right_dual_g1[0] * self[e1]) + (right_dual_g1[1] * self[e2]) + (right_dual_g1[2] * self[e3]))
    }
}
impl BulkContraction<AntiMotor> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl BulkContraction<AntiPlane> for AntiPlane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(/* scalar */ (right_dual_g0[0] * self[e1]) + (right_dual_g0[1] * self[e2]) + (right_dual_g0[2] * self[e3]))
    }
}
impl BulkContraction<MultiVector> for AntiPlane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       14        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g9 = other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g9[0] * self[e1]) + (right_dual_g9[1] * self[e2]) + (right_dual_g9[2] * self[e3]) - (self[e5] * other[e4]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (self.group0().xyz() * right_dual_g0.yy().with_z(right_dual_g0[1])).with_w(0.0),
            // e5
            right_dual_g0[1] * self[e5],
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
impl BulkContraction<RoundPoint> for AntiPlane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(
            // scalar
            (right_dual_g0[0] * self[e1]) + (right_dual_g0[1] * self[e2]) + (right_dual_g0[2] * self[e3]) - (self[e5] * other[e4]),
        )
    }
}
impl BulkContraction<Scalar> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl BulkContraction<VersorEven> for AntiPlane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g3 = other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(
            // scalar
            (right_dual_g3[0] * self[e1]) + (right_dual_g3[1] * self[e2]) + (right_dual_g3[2] * self[e3]) - (self[e5] * other[e4]),
        )
    }
}
impl BulkContraction<VersorOdd> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl std::ops::Div<BulkContractionInfix> for AntiScalar {
    type Output = BulkContractionInfixPartial<AntiScalar>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for AntiScalar {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       22        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            Simd32x4::from(self[e12345]) * other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl BulkContraction<AntiDipoleInversion> for AntiScalar {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       27        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
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
impl BulkContraction<AntiDualNum> for AntiScalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(self[e12345]) * other.group0())
    }
}
impl BulkContraction<AntiFlatPoint> for AntiScalar {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self[e12345]) * other.group0().xyz().with_w(other[e321] * -1.0))
    }
}
impl BulkContraction<AntiFlector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl BulkContraction<AntiLine> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        4        0
    // no simd        0       12        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(self[e12345]) * other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * other.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl BulkContraction<AntiMotor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl BulkContraction<AntiPlane> for AntiScalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self[e12345]) * other.group0().xyz().with_w(other[e5] * -1.0))
    }
}
impl BulkContraction<AntiScalar> for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bulk_contraction(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e12345] * self[e12345] * -1.0)
    }
}
impl BulkContraction<Circle> for AntiScalar {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       14        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
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
impl BulkContraction<CircleRotor> for AntiScalar {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       19        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
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
impl BulkContraction<Dipole> for AntiScalar {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       20        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * other.group2() * Simd32x3::from(-1.0),
        )
    }
}
impl BulkContraction<DipoleInversion> for AntiScalar {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       30        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            Simd32x4::from(self[e12345]) * other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            Simd32x4::from(self[e12345]) * other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl BulkContraction<DualNum> for AntiScalar {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        4        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(self[e12345]) * Simd32x2::from([other[e5] * -1.0, other[e12345] * -1.0]))
    }
}
impl BulkContraction<FlatPoint> for AntiScalar {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, other[e45]]),
        )
    }
}
impl BulkContraction<Flector> for AntiScalar {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl BulkContraction<Line> for AntiScalar {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(self[e12345]) * other.group0(),
            // e15, e25, e35
            Simd32x3::from(self[e12345]) * other.group1(),
        )
    }
}
impl BulkContraction<Motor> for AntiScalar {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl BulkContraction<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd2        0        2        0
    //    simd3        0        6        0
    //    simd4        0        7        0
    // Totals...
    // yes simd        0       19        0
    //  no simd        0       54        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(self[e12345]) * other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            self[e12345] * other[e3215],
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group7(),
            // e23, e31, e12
            Simd32x3::from(self[e12345]) * other.group6().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            self[e12345] * other[e4] * -1.0,
        )
    }
}
impl BulkContraction<Plane> for AntiScalar {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn bulk_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(self[e12345]) * Simd32x4::from([other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0, other[e3215]]),
        )
    }
}
impl BulkContraction<RoundPoint> for AntiScalar {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            self[e12345] * other[e4] * -1.0,
        )
    }
}
impl BulkContraction<Scalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e12345] * other[scalar])
    }
}
impl BulkContraction<Sphere> for AntiScalar {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            self[e12345] * other[e3215],
        )
    }
}
impl BulkContraction<VersorEven> for AntiScalar {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        8        0
    // no simd        0       32        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
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
impl BulkContraction<VersorOdd> for AntiScalar {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        8        0
    // no simd        0       32        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            Simd32x4::from(self[e12345]) * other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<BulkContractionInfix> for Circle {
    type Output = BulkContractionInfixPartial<Circle>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        5        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       32        0
    //  no simd       25       51        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual_g2[3]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual_g2[3]) * self.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(right_dual_g2[3]) * self.group2()).with_w(
                -(right_dual_g0[0] * self[e415])
                    - (right_dual_g0[1] * self[e425])
                    - (right_dual_g0[2] * self[e435])
                    - (right_dual_g1[0] * self[e423])
                    - (right_dual_g1[1] * self[e431])
                    - (right_dual_g1[2] * self[e412]),
            ),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g1[0] * self[e321]) + (right_dual_g1[3] * self[e415]) + (right_dual_g2[1] * self[e412]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g1[1] * self[e321]) + (right_dual_g1[3] * self[e425]) + (right_dual_g2[2] * self[e423]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g1[2] * self[e321]) + (right_dual_g1[3] * self[e435]) + (right_dual_g2[0] * self[e431]),
                -(right_dual_g1[2] * self[e125]) - (right_dual_g2[0] * self[e415]) - (right_dual_g2[1] * self[e425]) - (right_dual_g2[2] * self[e435]),
            ]) - (right_dual_g0.yzx() * self.group2().zxy()).with_w(right_dual_g1[1] * self[e315])
                - (self.group0().yzx() * right_dual_g2.zxy()).with_w(right_dual_g1[0] * self[e235]),
        )
    }
}
impl BulkContraction<AntiDipoleInversion> for Circle {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       24        0
    //    simd3        2        4        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       19       32        0
    //  no simd       29       52        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(right_dual_g2[3]) * self.group1().xyz()) + (self.group0().zxy() * right_dual_g3.yzx()) - (self.group0().yzx() * right_dual_g3.zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual_g2[3] * self[e235]) + (right_dual_g3[3] * self[e423]),
                (right_dual_g2[3] * self[e315]) + (right_dual_g3[3] * self[e431]),
                (right_dual_g2[3] * self[e125]) + (right_dual_g3[3] * self[e412]),
                -(right_dual_g3[1] * self[e425]) - (right_dual_g3[2] * self[e435]),
            ]) - (right_dual_g3.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (right_dual_g3[2] * self[e315]) + (right_dual_g3[3] * self[e415]),
                (right_dual_g3[0] * self[e125]) + (right_dual_g3[3] * self[e425]),
                (right_dual_g3[1] * self[e235]) + (right_dual_g3[3] * self[e435]),
                -(right_dual_g1[0] * self[e415])
                    - (right_dual_g1[1] * self[e425])
                    - (right_dual_g1[2] * self[e435])
                    - (right_dual_g1[3] * self[e321])
                    - (right_dual_g2[1] * self[e431])
                    - (right_dual_g2[2] * self[e412])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
            ]) - (self.group2().zxy() * right_dual_g3.yzx()).with_w(right_dual_g2[0] * self[e423]),
        )
    }
}
impl BulkContraction<AntiDualNum> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
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
impl BulkContraction<AntiFlatPoint> for Circle {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(
            // scalar
            -(right_dual_g0[0] * self[e423]) - (right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412]) - (right_dual_g0[3] * self[e321]),
        )
    }
}
impl BulkContraction<AntiFlector> for Circle {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       12        0
    //    simd3        1        4        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       17       36        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (self.group0().zxy() * right_dual_g1.yzx()) - (self.group0().yzx() * right_dual_g1.zxy()),
            // e23, e31, e12, e45
            (self.group0() * right_dual_g1.www()).with_w(-(right_dual_g1[1] * self[e425]) - (right_dual_g1[2] * self[e435])) - (right_dual_g1.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (right_dual_g1[2] * self[e315]) + (right_dual_g1[3] * self[e415]),
                (right_dual_g1[0] * self[e125]) + (right_dual_g1[3] * self[e425]),
                (right_dual_g1[1] * self[e235]) + (right_dual_g1[3] * self[e435]),
                -(right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412]) - (right_dual_g0[3] * self[e321]),
            ]) - (self.group2().zxy() * right_dual_g1.yzx()).with_w(right_dual_g0[0] * self[e423]),
        )
    }
}
impl BulkContraction<AntiLine> for Circle {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       24        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[0] * self[e321]) + (right_dual_g1[1] * self[e412]),
                (right_dual_g0[1] * self[e321]) + (right_dual_g1[2] * self[e423]),
                (right_dual_g0[2] * self[e321]) + (right_dual_g1[0] * self[e431]),
                -(right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412]),
            ]) - (right_dual_g1.zxy() * self.group0().yzx()).with_w(right_dual_g0[0] * self[e423]),
            // e5
            -(right_dual_g0[0] * self[e235])
                - (right_dual_g0[1] * self[e315])
                - (right_dual_g0[2] * self[e125])
                - (right_dual_g1[0] * self[e415])
                - (right_dual_g1[1] * self[e425])
                - (right_dual_g1[2] * self[e435]),
        )
    }
}
impl BulkContraction<AntiMotor> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       10       21        0
    //  no simd       13       36        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual_g0[3]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual_g0[3]) * self.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(right_dual_g0[3]) * self.group2()).with_w(-(right_dual_g0[0] * self[e423]) - (right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[0] * self[e321]) + (right_dual_g1[1] * self[e412]),
                (right_dual_g0[1] * self[e321]) + (right_dual_g1[2] * self[e423]),
                (right_dual_g0[2] * self[e321]) + (right_dual_g1[0] * self[e431]),
                -(right_dual_g0[1] * self[e315])
                    - (right_dual_g0[2] * self[e125])
                    - (right_dual_g1[0] * self[e415])
                    - (right_dual_g1[1] * self[e425])
                    - (right_dual_g1[2] * self[e435]),
            ]) - (self.group0().yzx() * right_dual_g1.zxy()).with_w(right_dual_g0[0] * self[e235]),
        )
    }
}
impl BulkContraction<AntiPlane> for Circle {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd3        3        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd       14       28        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Dipole::from_groups(
            // e41, e42, e43
            (self.group0().zxy() * right_dual_g0.yzx()) - (self.group0().yzx() * right_dual_g0.zxy()),
            // e23, e31, e12, e45
            (self.group0() * right_dual_g0.www()).with_w(-(right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435])) - (right_dual_g0.xyzx() * self.group1().wwwx()),
            // e15, e25, e35
            (Simd32x3::from(right_dual_g0[3]) * self.group1().xyz()) + (self.group2().yzx() * right_dual_g0.zxy()) - (self.group2().zxy() * right_dual_g0.yzx()),
        )
    }
}
impl BulkContraction<Circle> for Circle {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       11        0
    //  no simd        9       14        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(
            // scalar
            -(right_dual_g1[0] * self[e415])
                - (right_dual_g1[1] * self[e425])
                - (right_dual_g1[2] * self[e435])
                - (right_dual_g1[3] * self[e321])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e235] * self[e423])
                - (other[e315] * self[e431])
                - (other[e125] * self[e412]),
        )
    }
}
impl BulkContraction<CircleRotor> for Circle {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        9       12        0
    //  no simd        9       18        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(
            // scalar
            -(right_dual_g1[0] * self[e415])
                - (right_dual_g1[1] * self[e425])
                - (right_dual_g1[2] * self[e435])
                - (right_dual_g1[3] * self[e321])
                - (right_dual_g2[0] * self[e423])
                - (right_dual_g2[1] * self[e431])
                - (right_dual_g2[2] * self[e412])
                - (self[e235] * other[e423])
                - (self[e315] * other[e431])
                - (self[e125] * other[e412]),
        )
    }
}
impl BulkContraction<Dipole> for Circle {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       25       40        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g2[1] * self[e412]) + (right_dual_g1[0] * self[e321]) + (right_dual_g1[3] * self[e415]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g2[2] * self[e423]) + (right_dual_g1[1] * self[e321]) + (right_dual_g1[3] * self[e425]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g2[0] * self[e431]) + (right_dual_g1[2] * self[e321]) + (right_dual_g1[3] * self[e435]),
                -(right_dual_g0[2] * self[e435]) - (right_dual_g1[0] * self[e423]) - (right_dual_g1[1] * self[e431]) - (right_dual_g1[2] * self[e412]),
            ]) - (right_dual_g0.yzx() * self.group2().zxy()).with_w(right_dual_g0[0] * self[e415])
                - (right_dual_g2.zxy() * self.group0().yzx()).with_w(right_dual_g0[1] * self[e425]),
            // e5
            -(right_dual_g2[0] * self[e415])
                - (right_dual_g2[1] * self[e425])
                - (right_dual_g2[2] * self[e435])
                - (right_dual_g1[0] * self[e235])
                - (right_dual_g1[1] * self[e315])
                - (right_dual_g1[2] * self[e125]),
        )
    }
}
impl BulkContraction<DipoleInversion> for Circle {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       25       41        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g1[0] * self[e321]) + (right_dual_g1[3] * self[e415]) + (right_dual_g2[1] * self[e412]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g1[1] * self[e321]) + (right_dual_g1[3] * self[e425]) + (right_dual_g2[2] * self[e423]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g1[2] * self[e321]) + (right_dual_g1[3] * self[e435]) + (right_dual_g2[0] * self[e431]),
                -(right_dual_g0[2] * self[e435]) - (right_dual_g1[0] * self[e423]) - (right_dual_g1[1] * self[e431]) - (right_dual_g1[2] * self[e412]),
            ]) - (right_dual_g0.yzx() * self.group2().zxy()).with_w(right_dual_g0[0] * self[e415])
                - (self.group0().yzx() * right_dual_g2.zxy()).with_w(right_dual_g0[1] * self[e425]),
            // e5
            -(right_dual_g1[0] * self[e235])
                - (right_dual_g1[1] * self[e315])
                - (right_dual_g1[2] * self[e125])
                - (right_dual_g2[0] * self[e415])
                - (right_dual_g2[1] * self[e425])
                - (right_dual_g2[2] * self[e435]),
        )
    }
}
impl BulkContraction<DualNum> for Circle {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        let right_dual_g0 = other.group0() * Simd32x2::from(-1.0);
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(right_dual_g0[0]) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(right_dual_g0[0]) * self.group1().xyz(),
        )
    }
}
impl BulkContraction<FlatPoint> for Circle {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       16        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[1] * self[e412]) + (right_dual_g0[3] * self[e415]),
                (right_dual_g0[2] * self[e423]) + (right_dual_g0[3] * self[e425]),
                (right_dual_g0[0] * self[e431]) + (right_dual_g0[3] * self[e435]),
                -(right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]),
            ]) - (self.group0().yzx() * right_dual_g0.zxy()).with_w(right_dual_g0[0] * self[e415]),
        )
    }
}
impl BulkContraction<Flector> for Circle {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       16        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[1] * self[e412]) + (right_dual_g0[3] * self[e415]),
                (right_dual_g0[2] * self[e423]) + (right_dual_g0[3] * self[e425]),
                (right_dual_g0[0] * self[e431]) + (right_dual_g0[3] * self[e435]),
                -(right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]),
            ]) - (self.group0().yzx() * right_dual_g0.zxy()).with_w(right_dual_g0[0] * self[e415]),
        )
    }
}
impl BulkContraction<Line> for Circle {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
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
impl BulkContraction<Motor> for Circle {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       20        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(right_dual_g1[3]) * self.group0()).with_w(
                -(right_dual_g0[0] * self[e415])
                    - (right_dual_g0[1] * self[e425])
                    - (right_dual_g0[2] * self[e435])
                    - (right_dual_g1[0] * self[e423])
                    - (right_dual_g1[1] * self[e431])
                    - (right_dual_g1[2] * self[e412]),
            ),
            // e15, e25, e35, e3215
            (self.group1().xyz() * right_dual_g1.www()).with_w(0.0),
        )
    }
}
impl BulkContraction<MultiVector> for Circle {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       43        0
    //    simd2        0        1        0
    //    simd3        4       12        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       37       60        0
    //  no simd       54       97        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g6 = other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_dual_g9 = other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e321] * other[e321])
                    - (self[e423] * other[e235])
                    - (self[e431] * other[e315])
                    - (self[e412] * other[e125])
                    - (self[e415] * other[e415])
                    - (self[e425] * other[e425])
                    - (self[e435] * other[e435])
                    - (self[e235] * other[e423])
                    - (self[e315] * other[e431])
                    - (self[e125] * other[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g7[2] * self[e315]) + (right_dual_g8[1] * self[e412]) + (right_dual_g6[0] * self[e321]) + (right_dual_g6[3] * self[e415]),
                (right_dual_g7[0] * self[e125]) + (right_dual_g8[2] * self[e423]) + (right_dual_g6[1] * self[e321]) + (right_dual_g6[3] * self[e425]),
                (right_dual_g7[1] * self[e235]) + (right_dual_g8[0] * self[e431]) + (right_dual_g6[2] * self[e321]) + (right_dual_g6[3] * self[e435]),
                -(right_dual_g7[0] * self[e415]) - (right_dual_g7[1] * self[e425]) - (right_dual_g7[2] * self[e435]) - (right_dual_g6[2] * self[e412]),
            ]) - (right_dual_g7.yzx() * self.group2().zxy()).with_w(right_dual_g6[1] * self[e431])
                - (right_dual_g8.zxy() * self.group0().yzx()).with_w(right_dual_g6[0] * self[e423]),
            // e5
            -(right_dual_g8[0] * self[e415])
                - (right_dual_g8[1] * self[e425])
                - (right_dual_g8[2] * self[e435])
                - (right_dual_g6[0] * self[e235])
                - (right_dual_g6[1] * self[e315])
                - (right_dual_g6[2] * self[e125]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g9[2] * self[e315]) + (right_dual_g9[3] * self[e415]),
                (right_dual_g9[0] * self[e125]) + (right_dual_g9[3] * self[e425]),
                (right_dual_g9[1] * self[e235]) + (right_dual_g9[3] * self[e435]),
                -(right_dual_g9[1] * self[e425]) - (right_dual_g9[2] * self[e435]),
            ]) - (right_dual_g9.yzxx() * self.group2().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(right_dual_g10) * self.group1().xyz()) + (self.group0().zxy() * right_dual_g9.yzx()) - (self.group0().yzx() * right_dual_g9.zxy()),
            // e23, e31, e12
            (Simd32x3::from(right_dual_g10) * self.group2()) + (Simd32x3::from(right_dual_g9[3]) * self.group0()) - (Simd32x3::from(self[e321]) * right_dual_g9.xyz()),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual_g0[1]) * self.group1(),
            // e423, e431, e412
            Simd32x3::from(right_dual_g0[1]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(right_dual_g0[1]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl BulkContraction<RoundPoint> for Circle {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        4        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       20       35        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other[e4] * -1.0;
        Dipole::from_groups(
            // e41, e42, e43
            (Simd32x3::from(right_dual_g1) * self.group1().xyz()) + (self.group0().zxy() * right_dual_g0.yzx()) - (self.group0().yzx() * right_dual_g0.zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual_g1 * self[e235]) + (right_dual_g0[3] * self[e423]),
                (right_dual_g1 * self[e315]) + (right_dual_g0[3] * self[e431]),
                (right_dual_g1 * self[e125]) + (right_dual_g0[3] * self[e412]),
                -(right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]),
            ]) - (right_dual_g0.xyzx() * self.group1().wwwx()),
            // e15, e25, e35
            (Simd32x3::from(right_dual_g0[3]) * self.group1().xyz()) + (self.group2().yzx() * right_dual_g0.zxy()) - (self.group2().zxy() * right_dual_g0.yzx()),
        )
    }
}
impl BulkContraction<Scalar> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
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
impl BulkContraction<VersorEven> for Circle {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       24        0
    //    simd3        2        4        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       19       33        0
    //  no simd       29       56        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(right_dual_g2[3]) * self.group1().xyz()) + (self.group0().zxy() * right_dual_g3.yzx()) - (self.group0().yzx() * right_dual_g3.zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual_g2[3] * self[e235]) + (right_dual_g3[3] * self[e423]),
                (right_dual_g2[3] * self[e315]) + (right_dual_g3[3] * self[e431]),
                (right_dual_g2[3] * self[e125]) + (right_dual_g3[3] * self[e412]),
                -(right_dual_g3[1] * self[e425]) - (right_dual_g3[2] * self[e435]),
            ]) - (right_dual_g3.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (right_dual_g3[2] * self[e315]) + (right_dual_g3[3] * self[e415]),
                (right_dual_g3[0] * self[e125]) + (right_dual_g3[3] * self[e425]),
                (right_dual_g3[1] * self[e235]) + (right_dual_g3[3] * self[e435]),
                -(right_dual_g0[0] * self[e235])
                    - (right_dual_g0[1] * self[e315])
                    - (right_dual_g0[2] * self[e125])
                    - (right_dual_g1[0] * self[e415])
                    - (right_dual_g1[1] * self[e425])
                    - (right_dual_g1[2] * self[e435])
                    - (right_dual_g1[3] * self[e321])
                    - (right_dual_g2[1] * self[e431])
                    - (right_dual_g2[2] * self[e412]),
            ]) - (self.group2().zxy() * right_dual_g3.yzx()).with_w(right_dual_g2[0] * self[e423]),
        )
    }
}
impl BulkContraction<VersorOdd> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        4        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       19       32        0
    //  no simd       25       52        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual_g0[3]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual_g0[3]) * self.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(right_dual_g0[3]) * self.group2()).with_w(
                -(right_dual_g0[0] * self[e415])
                    - (right_dual_g0[1] * self[e425])
                    - (right_dual_g0[2] * self[e435])
                    - (right_dual_g1[0] * self[e423])
                    - (right_dual_g1[1] * self[e431])
                    - (right_dual_g1[2] * self[e412]),
            ),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g1[0] * self[e321]) + (right_dual_g1[3] * self[e415]) + (right_dual_g2[1] * self[e412]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g1[1] * self[e321]) + (right_dual_g1[3] * self[e425]) + (right_dual_g2[2] * self[e423]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g1[2] * self[e321]) + (right_dual_g1[3] * self[e435]) + (right_dual_g2[0] * self[e431]),
                -(right_dual_g1[2] * self[e125]) - (right_dual_g2[0] * self[e415]) - (right_dual_g2[1] * self[e425]) - (right_dual_g2[2] * self[e435]),
            ]) - (self.group0().yzx() * right_dual_g2.zxy()).with_w(right_dual_g1[0] * self[e235])
                - (self.group2().zxy() * right_dual_g0.yzx()).with_w(right_dual_g1[1] * self[e315]),
        )
    }
}
impl std::ops::Div<BulkContractionInfix> for CircleRotor {
    type Output = BulkContractionInfixPartial<CircleRotor>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       31        0
    //    simd3        1        5        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       24       40        0
    //  no simd       35       62        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((right_dual_g0 * Simd32x3::from(self[e12345])) + (Simd32x3::from(right_dual_g2[3]) * self.group0())).with_w(right_dual_g2[3] * self[e12345]),
            // e415, e425, e435, e321
            (right_dual_g1 * Simd32x4::from(self[e12345])) + (Simd32x4::from(right_dual_g2[3]) * self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (right_dual_g2[0] * self[e12345]) + (right_dual_g2[3] * self[e235]),
                (right_dual_g2[1] * self[e12345]) + (right_dual_g2[3] * self[e315]),
                (right_dual_g2[2] * self[e12345]) + (right_dual_g2[3] * self[e125]),
                -(right_dual_g1[0] * self[e235])
                    - (right_dual_g1[1] * self[e315])
                    - (right_dual_g1[2] * self[e125])
                    - (right_dual_g2[0] * self[e415])
                    - (right_dual_g2[1] * self[e425])
                    - (right_dual_g2[2] * self[e435]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g1[0] * self[e321]) + (right_dual_g1[3] * self[e415]) + (right_dual_g2[1] * self[e412]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g1[1] * self[e321]) + (right_dual_g1[3] * self[e425]) + (right_dual_g2[2] * self[e423]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g1[2] * self[e321]) + (right_dual_g1[3] * self[e435]) + (right_dual_g2[0] * self[e431]),
                -(right_dual_g0[2] * self[e435]) - (right_dual_g1[0] * self[e423]) - (right_dual_g1[1] * self[e431]) - (right_dual_g1[2] * self[e412]),
            ]) - (right_dual_g0.yzx() * self.group2().zxy()).with_w(right_dual_g0[0] * self[e415])
                - (self.group0().yzx() * right_dual_g2.zxy()).with_w(right_dual_g0[1] * self[e425]),
        )
    }
}
impl BulkContraction<AntiDipoleInversion> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       29        0
    //    simd3        3        6        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       24       40        0
    //  no simd       39       67        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (right_dual_g2[3] * self[e415]) + (right_dual_g3[1] * self[e412]) + (other[e423] * self[e12345]),
                (right_dual_g2[3] * self[e425]) + (right_dual_g3[2] * self[e423]) + (other[e431] * self[e12345]),
                (right_dual_g2[3] * self[e435]) + (right_dual_g3[0] * self[e431]) + (other[e412] * self[e12345]),
                -(right_dual_g1[0] * self[e415])
                    - (right_dual_g1[1] * self[e425])
                    - (right_dual_g1[2] * self[e435])
                    - (right_dual_g1[3] * self[e321])
                    - (right_dual_g2[1] * self[e431])
                    - (right_dual_g2[2] * self[e412])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
            ]) - (self.group0().yzx() * right_dual_g3.zxy()).with_w(right_dual_g2[0] * self[e423]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual_g1[0] * self[e12345]) + (right_dual_g2[3] * self[e235]),
                (right_dual_g1[1] * self[e12345]) + (right_dual_g2[3] * self[e315]),
                (right_dual_g1[2] * self[e12345]) + (right_dual_g2[3] * self[e125]),
                -(right_dual_g3[1] * self[e425]) - (right_dual_g3[2] * self[e435]),
            ]) + (self.group0() * right_dual_g3.www()).with_w(right_dual_g1[3] * self[e12345])
                - (right_dual_g3.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual_g3[3]) * self.group1().xyz()) + (Simd32x3::from(self[e12345]) * right_dual_g2.xyz()) + (right_dual_g3.zxy() * self.group2().yzx())
                - (right_dual_g3.yzx() * self.group2().zxy()))
            .with_w(right_dual_g2[3] * self[e12345]),
            // e4235, e4315, e4125, e3215
            right_dual_g3 * Simd32x4::from(self[e12345]),
        )
    }
}
impl BulkContraction<AntiDualNum> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other[scalar]) * self.group0().with_w(self[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125, e5
            self.group2() * other.group0().yy().with_zw(other[scalar], other[e3215]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}
impl BulkContraction<AntiFlatPoint> for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       12        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(right_dual_g0[3] * self[e12345]),
            // e15, e25, e35, scalar
            (right_dual_g0.xyz() * self.group2().www())
                .with_w(-(right_dual_g0[0] * self[e423]) - (right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412]) - (right_dual_g0[3] * self[e321])),
        )
    }
}
impl BulkContraction<AntiFlector> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        7        0
    //    simd3        1        7        0
    //    simd4        5        4        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       27       44        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((self.group0().zxy() * right_dual_g1.yzx()) - (self.group0().yzx() * right_dual_g1.zxy()))
                .with_w(-(right_dual_g0[0] * self[e423]) - (right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412]) - (right_dual_g0[3] * self[e321])),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(right_dual_g1[1] * self[e425]) - (right_dual_g1[2] * self[e435]))
                + (self.group0() * right_dual_g1.www()).with_w(right_dual_g0[3] * self[e12345])
                - (right_dual_g1.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(right_dual_g1[3]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e12345]) * right_dual_g0.xyz()).with_w(0.0)
                + (right_dual_g1.zxy() * self.group2().yzx()).with_w(0.0)
                - (right_dual_g1.yzx() * self.group2().zxy()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            right_dual_g1 * Simd32x4::from(self[e12345]),
        )
    }
}
impl BulkContraction<AntiLine> for CircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        5        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       13       30        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (right_dual_g0 * self.group2().www()).with_w(0.0),
            // e235, e315, e125, e4
            (right_dual_g1 * Simd32x3::from(self[e12345])).with_w(-(right_dual_g0[0] * self[e423]) - (right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[0] * self[e321]) + (right_dual_g1[1] * self[e412]),
                (right_dual_g0[1] * self[e321]) + (right_dual_g1[2] * self[e423]),
                (right_dual_g0[2] * self[e321]) + (right_dual_g1[0] * self[e431]),
                -(right_dual_g0[1] * self[e315])
                    - (right_dual_g0[2] * self[e125])
                    - (right_dual_g1[0] * self[e415])
                    - (right_dual_g1[1] * self[e425])
                    - (right_dual_g1[2] * self[e435]),
            ]) - (right_dual_g1.zxy() * self.group0().yzx()).with_w(right_dual_g0[0] * self[e235]),
        )
    }
}
impl BulkContraction<AntiMotor> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       16        0
    //    simd3        1        3        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       20       45        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(right_dual_g0[3]) * self.group0().with_w(self[e12345]),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_dual_g0[3]) * self.group1().xyz()) + (Simd32x3::from(self[e12345]) * right_dual_g0.xyz())).with_w(right_dual_g0[3] * self[e321]),
            // e235, e315, e125, e5
            (Simd32x4::from([self[e12345], self[e12345], self[e12345], 1.0])
                * right_dual_g1.xyz().with_w(
                    -(right_dual_g0[0] * self[e235])
                        - (right_dual_g0[1] * self[e315])
                        - (right_dual_g0[2] * self[e125])
                        - (right_dual_g1[0] * self[e415])
                        - (right_dual_g1[1] * self[e425])
                        - (right_dual_g1[2] * self[e435]),
                ))
                + (self.group2() * right_dual_g0.www().with_w(right_dual_g1[3])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[0] * self[e321]) + (right_dual_g1[1] * self[e412]),
                (right_dual_g0[1] * self[e321]) + (right_dual_g1[2] * self[e423]),
                (right_dual_g0[2] * self[e321]) + (right_dual_g1[0] * self[e431]),
                -(right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412]),
            ]) - (self.group0().yzx() * right_dual_g1.zxy()).with_w(right_dual_g0[0] * self[e423]),
        )
    }
}
impl BulkContraction<AntiPlane> for CircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd3        1        6        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        5       11        0
    //  no simd       16       32        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        DipoleInversion::from_groups(
            // e41, e42, e43
            (self.group0().zxy() * right_dual_g0.yzx()) - (self.group0().yzx() * right_dual_g0.zxy()),
            // e23, e31, e12, e45
            (self.group0() * right_dual_g0.www()).with_w(-(right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435])) - (right_dual_g0.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(right_dual_g0[3]) * self.group1().xyz()).with_w(0.0) + (right_dual_g0.zxy() * self.group2().yzx()).with_w(0.0)
                - (right_dual_g0.yzx() * self.group2().zxy()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            right_dual_g0 * Simd32x4::from(self[e12345]),
        )
    }
}
impl BulkContraction<AntiScalar> for CircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bulk_contraction(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e12345] * self[e12345] * -1.0)
    }
}
impl BulkContraction<Circle> for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd        9       24        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            right_dual_g1 * Simd32x4::from(self[e12345]),
            // e15, e25, e35, scalar
            (Simd32x3::from(self[e12345]) * other.group2()).with_w(
                -(right_dual_g1[0] * self[e415])
                    - (right_dual_g1[1] * self[e425])
                    - (right_dual_g1[2] * self[e435])
                    - (right_dual_g1[3] * self[e321])
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
impl BulkContraction<CircleRotor> for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       11        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       10       29        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            right_dual_g1 * Simd32x4::from(self[e12345]),
            // e15, e25, e35, scalar
            (Simd32x3::from(self[e12345]) * right_dual_g2.xyz()).with_w(
                (right_dual_g2[3] * self[e12345])
                    - (right_dual_g1[0] * self[e415])
                    - (right_dual_g1[1] * self[e425])
                    - (right_dual_g1[2] * self[e435])
                    - (right_dual_g1[3] * self[e321])
                    - (right_dual_g2[0] * self[e423])
                    - (right_dual_g2[1] * self[e431])
                    - (right_dual_g2[2] * self[e412])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
            ),
        )
    }
}
impl BulkContraction<Dipole> for CircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        6        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       19       32        0
    //  no simd       25       50        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            right_dual_g0 * Simd32x3::from(self[e12345]),
            // e415, e425, e435, e321
            right_dual_g1 * Simd32x4::from(self[e12345]),
            // e235, e315, e125, e4
            (right_dual_g2 * Simd32x3::from(self[e12345])).with_w(
                -(right_dual_g0[0] * self[e415])
                    - (right_dual_g0[1] * self[e425])
                    - (right_dual_g0[2] * self[e435])
                    - (right_dual_g1[0] * self[e423])
                    - (right_dual_g1[1] * self[e431])
                    - (right_dual_g1[2] * self[e412]),
            ),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g2[1] * self[e412]) + (right_dual_g1[0] * self[e321]) + (right_dual_g1[3] * self[e415]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g2[2] * self[e423]) + (right_dual_g1[1] * self[e321]) + (right_dual_g1[3] * self[e425]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g2[0] * self[e431]) + (right_dual_g1[2] * self[e321]) + (right_dual_g1[3] * self[e435]),
                -(right_dual_g2[2] * self[e435]) - (right_dual_g1[0] * self[e235]) - (right_dual_g1[1] * self[e315]) - (right_dual_g1[2] * self[e125]),
            ]) - (right_dual_g0.yzx() * self.group2().zxy()).with_w(right_dual_g2[0] * self[e415])
                - (right_dual_g2.zxy() * self.group0().yzx()).with_w(right_dual_g2[1] * self[e425]),
        )
    }
}
impl BulkContraction<DipoleInversion> for CircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       24        0
    //    simd3        0        4        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       21       34        0
    //  no simd       30       60        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            right_dual_g0 * Simd32x3::from(self[e12345]),
            // e415, e425, e435, e321
            right_dual_g1 * Simd32x4::from(self[e12345]),
            // e235, e315, e125, e4
            (Simd32x3::from(self[e12345]) * right_dual_g2.xyz()).with_w(
                (right_dual_g2[3] * self[e12345])
                    - (right_dual_g0[0] * self[e415])
                    - (right_dual_g0[1] * self[e425])
                    - (right_dual_g0[2] * self[e435])
                    - (right_dual_g1[0] * self[e423])
                    - (right_dual_g1[1] * self[e431])
                    - (right_dual_g1[2] * self[e412]),
            ),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g1[0] * self[e321]) + (right_dual_g1[3] * self[e415]) + (right_dual_g2[1] * self[e412]) + (right_dual_g3[0] * self[e12345]),
                (right_dual_g1[1] * self[e321]) + (right_dual_g1[3] * self[e425]) + (right_dual_g2[2] * self[e423]) + (right_dual_g3[1] * self[e12345]),
                (right_dual_g1[2] * self[e321]) + (right_dual_g1[3] * self[e435]) + (right_dual_g2[0] * self[e431]) + (right_dual_g3[2] * self[e12345]),
                -(right_dual_g1[2] * self[e125]) - (right_dual_g2[0] * self[e415]) - (right_dual_g2[1] * self[e425]) - (right_dual_g2[2] * self[e435]),
            ]) + (self.group2().yzxw() * right_dual_g0.zxy().with_w(right_dual_g3[3]))
                - (self.group2().zxyx() * right_dual_g0.yzx().with_w(right_dual_g1[0]))
                - (self.group0().yzx() * right_dual_g2.zxy()).with_w(right_dual_g1[1] * self[e315]),
        )
    }
}
impl BulkContraction<DualNum> for CircleRotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x2::from(-1.0);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            right_dual_g0.xx().with_zw(right_dual_g0[0], right_dual_g0[1]) * self.group0().with_w(self[e12345]),
            // e15, e25, e35, e3215
            Simd32x4::from(right_dual_g0[0]) * self.group1().xyz().with_w(self[e12345]),
        )
    }
}
impl BulkContraction<FlatPoint> for CircleRotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        8       20        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            right_dual_g0 * Simd32x4::from(self[e12345]),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[1] * self[e412]) + (right_dual_g0[3] * self[e415]),
                (right_dual_g0[2] * self[e423]) + (right_dual_g0[3] * self[e425]),
                (right_dual_g0[0] * self[e431]) + (right_dual_g0[3] * self[e435]),
                -(right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]),
            ]) - (self.group0().yzx() * right_dual_g0.zxy()).with_w(right_dual_g0[0] * self[e415]),
        )
    }
}
impl BulkContraction<Flector> for CircleRotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       15        0
    //  no simd       12       28        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            right_dual_g0 * Simd32x4::from(self[e12345]),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[3] * self[e415]) + (right_dual_g1[0] * self[e12345]),
                (right_dual_g0[3] * self[e425]) + (right_dual_g1[1] * self[e12345]),
                (right_dual_g0[3] * self[e435]) + (right_dual_g1[2] * self[e12345]),
                -(right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]),
            ]) + (self.group0().zxy() * right_dual_g0.yzx()).with_w(right_dual_g1[3] * self[e12345])
                - (self.group0().yzx() * right_dual_g0.zxy()).with_w(right_dual_g0[0] * self[e415]),
        )
    }
}
impl BulkContraction<Line> for CircleRotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
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
            (other.group1() * self.group2().www()).with_w(0.0),
        )
    }
}
impl BulkContraction<Motor> for CircleRotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        8        0
    //    simd3        1        3        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        7       14        0
    //  no simd       12       29        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x4::from([self[e12345], self[e12345], self[e12345], 1.0])
                * right_dual_g0.xyz().with_w(
                    -(right_dual_g0[0] * self[e415])
                        - (right_dual_g0[1] * self[e425])
                        - (right_dual_g0[2] * self[e435])
                        - (right_dual_g1[0] * self[e423])
                        - (right_dual_g1[1] * self[e431])
                        - (right_dual_g1[2] * self[e412]),
                ))
                + (self.group0() * right_dual_g1.www()).with_w(right_dual_g0[3] * self[e12345]),
            // e15, e25, e35, e3215
            ((Simd32x3::from(right_dual_g1[3]) * self.group1().xyz()) + (Simd32x3::from(self[e12345]) * right_dual_g1.xyz())).with_w(right_dual_g1[3] * self[e12345]),
        )
    }
}
impl BulkContraction<MultiVector> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       50        0
    //    simd2        0        1        0
    //    simd3        8       18        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       46       76        0
    //  no simd       80      134        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1 = other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3_w = other[e321] * -1.0;
        let right_dual_g6 = other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_dual_g9 = other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g0[0] * self[e12345])
                    - (right_dual_g3_w * self[e321])
                    - (self[e423] * other[e235])
                    - (self[e431] * other[e315])
                    - (self[e412] * other[e125])
                    - (self[e415] * other[e415])
                    - (self[e425] * other[e425])
                    - (self[e435] * other[e435])
                    - (self[e235] * other[e423])
                    - (self[e315] * other[e431])
                    - (self[e125] * other[e412]),
                right_dual_g0[1] * self[e12345],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g7[2] * self[e315]) + (right_dual_g1[0] * self[e12345]) + (right_dual_g6[0] * self[e321]) + (right_dual_g6[3] * self[e415]),
                (right_dual_g7[0] * self[e125]) + (right_dual_g1[1] * self[e12345]) + (right_dual_g6[1] * self[e321]) + (right_dual_g6[3] * self[e425]),
                (right_dual_g7[1] * self[e235]) + (right_dual_g1[2] * self[e12345]) + (right_dual_g6[2] * self[e321]) + (right_dual_g6[3] * self[e435]),
                -(right_dual_g7[0] * self[e415]) - (right_dual_g7[1] * self[e425]) - (right_dual_g7[2] * self[e435]) - (right_dual_g6[2] * self[e412]),
            ]) + (right_dual_g8.yzx() * self.group0().zxy()).with_w(right_dual_g1[3] * self[e12345])
                - (right_dual_g7.yzx() * self.group2().zxy()).with_w(right_dual_g6[1] * self[e431])
                - (right_dual_g8.zxy() * self.group0().yzx()).with_w(right_dual_g6[0] * self[e423]),
            // e5
            (self[e12345] * other[e3215])
                - (right_dual_g8[0] * self[e415])
                - (right_dual_g8[1] * self[e425])
                - (right_dual_g8[2] * self[e435])
                - (right_dual_g6[0] * self[e235])
                - (right_dual_g6[1] * self[e315])
                - (right_dual_g6[2] * self[e125]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g9[2] * self[e315]) + (self[e12345] * other[e235]),
                (right_dual_g9[0] * self[e125]) + (self[e12345] * other[e315]),
                (right_dual_g9[1] * self[e235]) + (self[e12345] * other[e125]),
                -(right_dual_g9[1] * self[e425]) - (right_dual_g9[2] * self[e435]),
            ]) + (self.group1().xyz() * right_dual_g9.www()).with_w(right_dual_g3_w * self[e12345])
                - (right_dual_g9.yzxx() * self.group2().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(right_dual_g10) * self.group1().xyz()) + (Simd32x3::from(self[e12345]) * other.group7()) + (self.group0().zxy() * right_dual_g9.yzx())
                - (self.group0().yzx() * right_dual_g9.zxy()),
            // e23, e31, e12
            (Simd32x3::from(right_dual_g10) * self.group2().xyz()) + (Simd32x3::from(right_dual_g9[3]) * self.group0()) + (Simd32x3::from(self[e12345]) * other.group6().xyz())
                - (Simd32x3::from(self[e321]) * right_dual_g9.xyz()),
            // e415, e425, e435, e321
            (right_dual_g6 * Simd32x4::from(self[e12345])) + (Simd32x4::from(right_dual_g0[1]) * self.group1()),
            // e423, e431, e412
            (right_dual_g7 * Simd32x3::from(self[e12345])) + (Simd32x3::from(right_dual_g0[1]) * self.group0()),
            // e235, e315, e125
            (right_dual_g8 * Simd32x3::from(self[e12345])) + (Simd32x3::from(right_dual_g0[1]) * self.group2().xyz()),
            // e4235, e4315, e4125, e3215
            right_dual_g9 * Simd32x4::from(self[e12345]),
            // e1234
            right_dual_g10 * self[e12345],
        )
    }
}
impl BulkContraction<Plane> for CircleRotor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn bulk_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(self[e12345]) * Simd32x4::from([other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0, other[e3215]]),
        )
    }
}
impl BulkContraction<RoundPoint> for CircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd3        4        6        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       20       40        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other[e4] * -1.0;
        DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(right_dual_g1) * self.group1().xyz()) + (self.group0().zxy() * right_dual_g0.yzx()) - (self.group0().yzx() * right_dual_g0.zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual_g1 * self[e235]) + (right_dual_g0[3] * self[e423]),
                (right_dual_g1 * self[e315]) + (right_dual_g0[3] * self[e431]),
                (right_dual_g1 * self[e125]) + (right_dual_g0[3] * self[e412]),
                -(right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]),
            ]) - (right_dual_g0.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual_g0[3]) * self.group1().xyz()) + (right_dual_g0.zxy() * self.group2().yzx()) - (right_dual_g0.yzx() * self.group2().zxy()))
                .with_w(right_dual_g1 * self[e12345]),
            // e4235, e4315, e4125, e3215
            right_dual_g0 * Simd32x4::from(self[e12345]),
        )
    }
}
impl BulkContraction<Scalar> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
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
impl BulkContraction<Sphere> for CircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            self[e12345] * other[e3215],
        )
    }
}
impl BulkContraction<VersorEven> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       27        0
    //    simd3        3        7        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       22       40        0
    //  no simd       40       72        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (right_dual_g0[0] * self[e12345]) + (right_dual_g2[3] * self[e415]),
                (right_dual_g0[1] * self[e12345]) + (right_dual_g2[3] * self[e425]),
                (right_dual_g0[2] * self[e12345]) + (right_dual_g2[3] * self[e435]),
                -(right_dual_g0[0] * self[e235])
                    - (right_dual_g0[1] * self[e315])
                    - (right_dual_g0[2] * self[e125])
                    - (right_dual_g1[0] * self[e415])
                    - (right_dual_g1[1] * self[e425])
                    - (right_dual_g1[2] * self[e435])
                    - (right_dual_g1[3] * self[e321])
                    - (right_dual_g2[1] * self[e431])
                    - (right_dual_g2[2] * self[e412]),
            ]) + (self.group0().zxy() * right_dual_g3.yzx()).with_w(right_dual_g0[3] * self[e12345])
                - (self.group0().yzx() * right_dual_g3.zxy()).with_w(right_dual_g2[0] * self[e423]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual_g1[0] * self[e12345]) + (right_dual_g2[3] * self[e235]),
                (right_dual_g1[1] * self[e12345]) + (right_dual_g2[3] * self[e315]),
                (right_dual_g1[2] * self[e12345]) + (right_dual_g2[3] * self[e125]),
                -(right_dual_g3[1] * self[e425]) - (right_dual_g3[2] * self[e435]),
            ]) + (self.group0() * right_dual_g3.www()).with_w(right_dual_g1[3] * self[e12345])
                - (right_dual_g3.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual_g3[3]) * self.group1().xyz()) + (Simd32x3::from(self[e12345]) * right_dual_g2.xyz()) + (right_dual_g3.zxy() * self.group2().yzx())
                - (right_dual_g3.yzx() * self.group2().zxy()))
            .with_w(right_dual_g2[3] * self[e12345]),
            // e4235, e4315, e4125, e3215
            right_dual_g3 * Simd32x4::from(self[e12345]),
        )
    }
}
impl BulkContraction<VersorOdd> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       26        0
    //    simd3        1        5        0
    //    simd4        5        8        0
    // Totals...
    // yes simd       23       39        0
    //  no simd       40       73        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x3::from(right_dual_g0[3]) * self.group0()) + (Simd32x3::from(self[e12345]) * right_dual_g0.xyz())).with_w(right_dual_g0[3] * self[e12345]),
            // e415, e425, e435, e321
            (right_dual_g1 * Simd32x4::from(self[e12345])) + (Simd32x4::from(right_dual_g0[3]) * self.group1()),
            // e235, e315, e125, e5
            (Simd32x4::from([self[e12345], self[e12345], self[e12345], 1.0])
                * right_dual_g2.xyz().with_w(
                    -(right_dual_g1[0] * self[e235])
                        - (right_dual_g1[1] * self[e315])
                        - (right_dual_g1[2] * self[e125])
                        - (right_dual_g2[0] * self[e415])
                        - (right_dual_g2[1] * self[e425])
                        - (right_dual_g2[2] * self[e435]),
                ))
                + (self.group2() * right_dual_g0.www().with_w(right_dual_g2[3])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g1[0] * self[e321]) + (right_dual_g1[3] * self[e415]) + (right_dual_g3[0] * self[e12345]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g1[1] * self[e321]) + (right_dual_g1[3] * self[e425]) + (right_dual_g3[1] * self[e12345]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g1[2] * self[e321]) + (right_dual_g1[3] * self[e435]) + (right_dual_g3[2] * self[e12345]),
                -(right_dual_g0[0] * self[e415]) - (right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]) - (right_dual_g1[2] * self[e412]),
            ]) + (self.group0().zxy() * right_dual_g2.yzx()).with_w(right_dual_g3[3] * self[e12345])
                - (self.group0().yzx() * right_dual_g2.zxy()).with_w(right_dual_g1[0] * self[e423])
                - (right_dual_g0.yzx() * self.group2().zxy()).with_w(right_dual_g1[1] * self[e431]),
        )
    }
}
impl std::ops::Div<BulkContractionInfix> for Dipole {
    type Output = BulkContractionInfixPartial<Dipole>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for Dipole {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        9       16        0
    //  no simd        9       31        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual_g2[3]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual_g2[3]) * self.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(right_dual_g2[3]) * self.group2()).with_w(
                -(right_dual_g0[0] * self[e15])
                    - (right_dual_g0[1] * self[e25])
                    - (right_dual_g0[2] * self[e35])
                    - (right_dual_g1[0] * self[e23])
                    - (right_dual_g1[1] * self[e31])
                    - (right_dual_g1[2] * self[e12])
                    - (right_dual_g1[3] * self[e45])
                    - (right_dual_g2[0] * self[e41])
                    - (right_dual_g2[1] * self[e42])
                    - (right_dual_g2[2] * self[e43]),
            ),
        )
    }
}
impl BulkContraction<AntiDipoleInversion> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       15       28        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g3[2] * self[e31]) + (right_dual_g3[3] * self[e41]),
                (right_dual_g3[0] * self[e12]) + (right_dual_g3[3] * self[e42]),
                (right_dual_g3[1] * self[e23]) + (right_dual_g3[3] * self[e43]),
                -(right_dual_g2[3] * self[e45]) - (right_dual_g3[2] * self[e43]),
            ]) - (right_dual_g3.yzxy() * self.group1().zxy().with_w(self[e42]))
                - (self.group2() * right_dual_g2.www()).with_w(right_dual_g3[0] * self[e41]),
            // e5
            (right_dual_g3[0] * self[e15]) + (right_dual_g3[1] * self[e25]) + (right_dual_g3[2] * self[e35]) + (right_dual_g3[3] * self[e45]),
        )
    }
}
impl BulkContraction<AntiDualNum> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
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
impl BulkContraction<AntiFlector> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        8       14        0
    //  no simd       11       20        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g1[2] * self[e31]) + (right_dual_g1[3] * self[e41]),
                (right_dual_g1[0] * self[e12]) + (right_dual_g1[3] * self[e42]),
                (right_dual_g1[1] * self[e23]) + (right_dual_g1[3] * self[e43]),
                -(right_dual_g1[1] * self[e42]) - (right_dual_g1[2] * self[e43]),
            ]) - (right_dual_g1.yzxx() * self.group1().zxy().with_w(self[e41])),
            // e5
            (right_dual_g1[0] * self[e15]) + (right_dual_g1[1] * self[e25]) + (right_dual_g1[2] * self[e35]) + (right_dual_g1[3] * self[e45]),
        )
    }
}
impl BulkContraction<AntiLine> for Dipole {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        Scalar::from_groups(
            // scalar
            -(right_dual_g0[0] * self[e23])
                - (right_dual_g0[1] * self[e31])
                - (right_dual_g0[2] * self[e12])
                - (right_dual_g1[0] * self[e41])
                - (right_dual_g1[1] * self[e42])
                - (right_dual_g1[2] * self[e43]),
        )
    }
}
impl BulkContraction<AntiMotor> for Dipole {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        5       24        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual_g0[3]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual_g0[3]) * self.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(right_dual_g0[3]) * self.group2()).with_w(
                -(right_dual_g0[0] * self[e23])
                    - (right_dual_g0[1] * self[e31])
                    - (right_dual_g0[2] * self[e12])
                    - (right_dual_g1[0] * self[e41])
                    - (right_dual_g1[1] * self[e42])
                    - (right_dual_g1[2] * self[e43]),
            ),
        )
    }
}
impl BulkContraction<AntiPlane> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        8       14        0
    //  no simd       11       20        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[2] * self[e31]) + (right_dual_g0[3] * self[e41]),
                (right_dual_g0[0] * self[e12]) + (right_dual_g0[3] * self[e42]),
                (right_dual_g0[1] * self[e23]) + (right_dual_g0[3] * self[e43]),
                -(right_dual_g0[1] * self[e42]) - (right_dual_g0[2] * self[e43]),
            ]) - (right_dual_g0.yzxx() * self.group1().zxy().with_w(self[e41])),
            // e5
            (right_dual_g0[0] * self[e15]) + (right_dual_g0[1] * self[e25]) + (right_dual_g0[2] * self[e35]) + (right_dual_g0[3] * self[e45]),
        )
    }
}
impl BulkContraction<Dipole> for Dipole {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd        9       20        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        Scalar::from_groups(
            // scalar
            -(right_dual_g0[0] * self[e15])
                - (right_dual_g0[1] * self[e25])
                - (right_dual_g0[2] * self[e35])
                - (right_dual_g2[0] * self[e41])
                - (right_dual_g2[1] * self[e42])
                - (right_dual_g2[2] * self[e43])
                - (right_dual_g1[0] * self[e23])
                - (right_dual_g1[1] * self[e31])
                - (right_dual_g1[2] * self[e12])
                - (right_dual_g1[3] * self[e45]),
        )
    }
}
impl BulkContraction<DipoleInversion> for Dipole {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        9       13        0
    //  no simd        9       21        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Scalar::from_groups(
            // scalar
            -(right_dual_g0[0] * self[e15])
                - (right_dual_g0[1] * self[e25])
                - (right_dual_g0[2] * self[e35])
                - (right_dual_g1[0] * self[e23])
                - (right_dual_g1[1] * self[e31])
                - (right_dual_g1[2] * self[e12])
                - (right_dual_g1[3] * self[e45])
                - (right_dual_g2[0] * self[e41])
                - (right_dual_g2[1] * self[e42])
                - (right_dual_g2[2] * self[e43]),
        )
    }
}
impl BulkContraction<DualNum> for Dipole {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e5] * -1.0) * self.group0().with_w(self[e45]))
    }
}
impl BulkContraction<FlatPoint> for Dipole {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Scalar::from_groups(
            // scalar
            -(right_dual_g0[0] * self[e41]) - (right_dual_g0[1] * self[e42]) - (right_dual_g0[2] * self[e43]) - (right_dual_g0[3] * self[e45]),
        )
    }
}
impl BulkContraction<Flector> for Dipole {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Scalar::from_groups(
            // scalar
            -(right_dual_g0[0] * self[e41]) - (right_dual_g0[1] * self[e42]) - (right_dual_g0[2] * self[e43]) - (right_dual_g0[3] * self[e45]),
        )
    }
}
impl BulkContraction<Motor> for Dipole {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ (self.group0() * right_dual_g1.www()).with_w(right_dual_g1[3] * self[e45]))
    }
}
impl BulkContraction<MultiVector> for Dipole {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd2        0        1        0
    //    simd3        0        5        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       18       34        0
    //  no simd       24       57        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g6 = other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_dual_g9 = other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(right_dual_g7[0] * self[e15])
                    - (right_dual_g7[1] * self[e25])
                    - (right_dual_g7[2] * self[e35])
                    - (right_dual_g8[0] * self[e41])
                    - (right_dual_g8[1] * self[e42])
                    - (right_dual_g8[2] * self[e43])
                    - (right_dual_g6[0] * self[e23])
                    - (right_dual_g6[1] * self[e31])
                    - (right_dual_g6[2] * self[e12])
                    - (right_dual_g6[3] * self[e45]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g9[2] * self[e31]) + (right_dual_g9[3] * self[e41]),
                (right_dual_g9[0] * self[e12]) + (right_dual_g9[3] * self[e42]),
                (right_dual_g9[1] * self[e23]) + (right_dual_g9[3] * self[e43]),
                -(right_dual_g10 * self[e45]) - (right_dual_g9[2] * self[e43]),
            ]) - (right_dual_g9.yzxy() * self.group1().zxy().with_w(self[e42]))
                - (Simd32x3::from(right_dual_g10) * self.group2()).with_w(right_dual_g9[0] * self[e41]),
            // e5
            (right_dual_g9[0] * self[e15]) + (right_dual_g9[1] * self[e25]) + (right_dual_g9[2] * self[e35]) + (right_dual_g9[3] * self[e45]),
            // e15, e25, e35, e45
            Simd32x4::from(right_dual_g0[1]) * self.group2().with_w(self[e45]),
            // e41, e42, e43
            Simd32x3::from(right_dual_g0[1]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(right_dual_g0[1]) * self.group1().xyz(),
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
impl BulkContraction<RoundPoint> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       15       25        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other[e4] * -1.0;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[2] * self[e31]) + (right_dual_g0[3] * self[e41]),
                (right_dual_g0[0] * self[e12]) + (right_dual_g0[3] * self[e42]),
                (right_dual_g0[1] * self[e23]) + (right_dual_g0[3] * self[e43]),
                -(right_dual_g1 * self[e45]) - (right_dual_g0[2] * self[e43]),
            ]) - (right_dual_g0.yzxy() * self.group1().zxy().with_w(self[e42]))
                - (Simd32x3::from(right_dual_g1) * self.group2()).with_w(right_dual_g0[0] * self[e41]),
            // e5
            (right_dual_g0[0] * self[e15]) + (right_dual_g0[1] * self[e25]) + (right_dual_g0[2] * self[e35]) + (right_dual_g0[3] * self[e45]),
        )
    }
}
impl BulkContraction<Scalar> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
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
impl BulkContraction<VersorEven> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       15       28        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2 = other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g3[2] * self[e31]) + (right_dual_g3[3] * self[e41]),
                (right_dual_g3[0] * self[e12]) + (right_dual_g3[3] * self[e42]),
                (right_dual_g3[1] * self[e23]) + (right_dual_g3[3] * self[e43]),
                -(right_dual_g2[3] * self[e45]) - (right_dual_g3[2] * self[e43]),
            ]) - (right_dual_g3.yzxy() * self.group1().zxy().with_w(self[e42]))
                - (self.group2() * right_dual_g2.www()).with_w(right_dual_g3[0] * self[e41]),
            // e5
            (right_dual_g3[0] * self[e15]) + (right_dual_g3[1] * self[e25]) + (right_dual_g3[2] * self[e35]) + (right_dual_g3[3] * self[e45]),
        )
    }
}
impl BulkContraction<VersorOdd> for Dipole {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        9       16        0
    //  no simd        9       32        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual_g0[3]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual_g0[3]) * self.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(right_dual_g0[3]) * self.group2()).with_w(
                -(right_dual_g0[0] * self[e15])
                    - (right_dual_g0[1] * self[e25])
                    - (right_dual_g0[2] * self[e35])
                    - (right_dual_g1[0] * self[e23])
                    - (right_dual_g1[1] * self[e31])
                    - (right_dual_g1[2] * self[e12])
                    - (right_dual_g1[3] * self[e45])
                    - (right_dual_g2[0] * self[e41])
                    - (right_dual_g2[1] * self[e42])
                    - (right_dual_g2[2] * self[e43]),
            ),
        )
    }
}
impl std::ops::Div<BulkContractionInfix> for DipoleInversion {
    type Output = BulkContractionInfixPartial<DipoleInversion>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       29        0
    //    simd3        3        7        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       24       40        0
    //  no simd       39       66        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (right_dual_g0[2] * self[e4315]) + (right_dual_g1[0] * self[e1234]) + (right_dual_g2[3] * self[e41]),
                (right_dual_g0[0] * self[e4125]) + (right_dual_g1[1] * self[e1234]) + (right_dual_g2[3] * self[e42]),
                (right_dual_g0[1] * self[e4235]) + (right_dual_g1[2] * self[e1234]) + (right_dual_g2[3] * self[e43]),
                -(right_dual_g0[1] * self[e25])
                    - (right_dual_g0[2] * self[e35])
                    - (right_dual_g1[0] * self[e23])
                    - (right_dual_g1[1] * self[e31])
                    - (right_dual_g1[2] * self[e12])
                    - (right_dual_g1[3] * self[e45])
                    - (right_dual_g2[0] * self[e41])
                    - (right_dual_g2[1] * self[e42])
                    - (right_dual_g2[2] * self[e43]),
            ]) - (right_dual_g0.yzx() * self.group3().zxy()).with_w(right_dual_g0[0] * self[e15]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual_g2[0] * self[e1234]) + (right_dual_g2[3] * self[e23]),
                (right_dual_g2[1] * self[e1234]) + (right_dual_g2[3] * self[e31]),
                (right_dual_g2[2] * self[e1234]) + (right_dual_g2[3] * self[e12]),
                -(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125]),
            ]) + (right_dual_g0 * self.group3().www()).with_w(right_dual_g2[3] * self[e45])
                - (right_dual_g1.wwwx() * self.group3().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual_g2[3]) * self.group2().xyz()) + (Simd32x3::from(self[e3215]) * right_dual_g1.xyz()) + (right_dual_g2.yzx() * self.group3().zxy())
                - (right_dual_g2.zxy() * self.group3().yzx()))
            .with_w(right_dual_g2[3] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g2[3]) * self.group3(),
        )
    }
}
impl BulkContraction<AntiDipoleInversion> for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       10        0
    //    simd3        1        6        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       16       27        0
    //  no simd       48       72        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual_g3.xyz()) - (Simd32x3::from(right_dual_g2[3]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (right_dual_g3.yzxw() * self.group3().zxy().with_w(self[e1234])) - (self.group3().yzxw() * right_dual_g3.zxy().with_w(right_dual_g2[3])),
            // e235, e315, e125, e4
            (self.group3().xyzx() * right_dual_g3.www().with_w(other[e423]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual_g1[3] * self[e1234]) + (other[e431] * self[e4315]) + (other[e412] * self[e4125])
                        - (right_dual_g2[3] * self[e45])
                        - (right_dual_g3[1] * self[e42])
                        - (right_dual_g3[2] * self[e43]),
                )
                - (right_dual_g3.xyz() * self.group3().www()).with_w(right_dual_g3[0] * self[e41]),
            // e1, e2, e3, e5
            (right_dual_g3.zxyw() * self.group1().yzxw())
                + (self.group2().wwwz() * right_dual_g2.xyz().with_w(right_dual_g3[2]))
                + (self.group0() * right_dual_g3.www()).with_w(right_dual_g3[0] * self[e15])
                + (right_dual_g1.zxy() * self.group3().yzx()).with_w(right_dual_g3[1] * self[e25])
                - (Simd32x4::from(self[e3215]) * other.group0().with_w(right_dual_g1[3]))
                - (right_dual_g2.wwwy() * self.group2().xyz().with_w(self[e4315]))
                - (self.group3().zxyx() * right_dual_g1.yzx().with_w(right_dual_g2[0]))
                - (right_dual_g3.yzx() * self.group1().zxy()).with_w(right_dual_g2[2] * self[e4125]),
        )
    }
}
impl BulkContraction<AntiDualNum> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0().yy().with_zw(other[scalar], other[e3215]) * self.group0().with_w(self[e1234]),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[scalar]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group3(),
        )
    }
}
impl BulkContraction<AntiFlatPoint> for DipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            right_dual_g0 * Simd32x4::from(self[e1234]),
            // e5
            -(right_dual_g0[0] * self[e4235]) - (right_dual_g0[1] * self[e4315]) - (right_dual_g0[2] * self[e4125]) - (right_dual_g0[3] * self[e3215]),
        )
    }
}
impl BulkContraction<AntiFlector> for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        1        5        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       11       20        0
    //  no simd       31       48        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * right_dual_g1.xyz(),
            // e415, e425, e435, e321
            ((right_dual_g1.yzx() * self.group3().zxy()) - (right_dual_g1.zxy() * self.group3().yzx())).with_w(right_dual_g1[3] * self[e1234]),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(-(right_dual_g1[1] * self[e42]) - (right_dual_g1[2] * self[e43]))
                + (self.group3().xyz() * right_dual_g1.www()).with_w(right_dual_g0[3] * self[e1234])
                - (right_dual_g1.xyzx() * self.group3().www().with_w(self[e41])),
            // e1, e2, e3, e5
            (Simd32x4::from(right_dual_g1[3]) * self.group0().with_w(self[e45]))
                + (right_dual_g1.zxyx() * self.group1().yzx().with_w(self[e15]))
                + (self.group2().wwwy() * right_dual_g0.xyz().with_w(right_dual_g1[1]))
                + Simd32x3::from(0.0)
                    .with_w((right_dual_g1[2] * self[e35]) - (right_dual_g0[1] * self[e4315]) - (right_dual_g0[2] * self[e4125]) - (right_dual_g0[3] * self[e3215]))
                - (right_dual_g1.yzx() * self.group1().zxy()).with_w(right_dual_g0[0] * self[e4235]),
        )
    }
}
impl BulkContraction<AntiLine> for DipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        5        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       13       30        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            right_dual_g0 * Simd32x3::from(self[e1234]),
            // e23, e31, e12, e45
            (right_dual_g1 * Simd32x3::from(self[e1234])).with_w(-(right_dual_g0[0] * self[e4235]) - (right_dual_g0[1] * self[e4315]) - (right_dual_g0[2] * self[e4125])),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (right_dual_g0[0] * self[e3215]) + (right_dual_g1[1] * self[e4125]),
                (right_dual_g0[1] * self[e3215]) + (right_dual_g1[2] * self[e4235]),
                (right_dual_g0[2] * self[e3215]) + (right_dual_g1[0] * self[e4315]),
                -(right_dual_g0[0] * self[e23]) - (right_dual_g0[1] * self[e31]) - (right_dual_g0[2] * self[e12]) - (right_dual_g1[1] * self[e42]) - (right_dual_g1[2] * self[e43]),
            ]) - (right_dual_g1.zxy() * self.group3().yzx()).with_w(right_dual_g1[0] * self[e41]),
        )
    }
}
impl BulkContraction<AntiMotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       11        0
    //    simd3        3        5        0
    //    simd4        2        6        0
    // Totals...
    // yes simd       12       22        0
    //  no simd       24       50        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from([self[e1234], self[e1234], self[e1234], 1.0])
                * right_dual_g0.xyz().with_w(
                    -(right_dual_g0[0] * self[e23])
                        - (right_dual_g0[1] * self[e31])
                        - (right_dual_g0[2] * self[e12])
                        - (right_dual_g1[0] * self[e41])
                        - (right_dual_g1[1] * self[e42])
                        - (right_dual_g1[2] * self[e43]),
                ))
                + (self.group0() * right_dual_g0.www()).with_w(right_dual_g1[3] * self[e1234]),
            // e23, e31, e12, e45
            (Simd32x4::from(right_dual_g0[3]) * self.group1())
                + (Simd32x4::from([self[e1234], self[e1234], self[e1234], 1.0])
                    * right_dual_g1
                        .xyz()
                        .with_w(-(right_dual_g0[0] * self[e4235]) - (right_dual_g0[1] * self[e4315]) - (right_dual_g0[2] * self[e4125]))),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual_g0[3]) * self.group2().xyz()) + (Simd32x3::from(self[e3215]) * right_dual_g0.xyz()) + (right_dual_g1.yzx() * self.group3().zxy())
                - (right_dual_g1.zxy() * self.group3().yzx()))
            .with_w(right_dual_g0[3] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g0[3]) * self.group3(),
        )
    }
}
impl BulkContraction<AntiPlane> for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        5        0
    //    simd3        1        6        0
    //    simd4        3        4        0
    // Totals...
    // yes simd        6       15        0
    //  no simd       17       39        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * right_dual_g0.xyz(),
            // e415, e425, e435, e321
            ((right_dual_g0.yzx() * self.group3().zxy()) - (right_dual_g0.zxy() * self.group3().yzx())).with_w(right_dual_g0[3] * self[e1234]),
            // e235, e315, e125, e4
            (self.group3().xyz() * right_dual_g0.www()).with_w(-(right_dual_g0[1] * self[e42]) - (right_dual_g0[2] * self[e43]))
                - (right_dual_g0.xyzx() * self.group3().www().with_w(self[e41])),
            // e1, e2, e3, e5
            (Simd32x4::from(right_dual_g0[3]) * self.group0().with_w(self[e45]))
                + (right_dual_g0.zxyx() * self.group1().yzx().with_w(self[e15]))
                + (right_dual_g0.yzx() * self.group1().zxy() * Simd32x3::from(-1.0)).with_w((right_dual_g0[1] * self[e25]) + (right_dual_g0[2] * self[e35])),
        )
    }
}
impl BulkContraction<Circle> for DipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       16        0
    //  no simd       15       24        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual_g1[1] * self[e4125]) - (other[e423] * self[e3215]),
                -(right_dual_g1[2] * self[e4235]) - (other[e431] * self[e3215]),
                -(right_dual_g1[0] * self[e4315]) - (other[e412] * self[e3215]),
                (right_dual_g1[3] * self[e1234]) + (other[e412] * self[e4125]),
            ]) + (self.group3().yzxy() * right_dual_g1.zxy().with_w(other[e431]))
                + (other.group2() * self.group2().www()).with_w(other[e423] * self[e4235]),
            // e5
            -(right_dual_g1[3] * self[e3215]) - (other[e235] * self[e4235]) - (other[e315] * self[e4315]) - (other[e125] * self[e4125]),
        )
    }
}
impl BulkContraction<CircleRotor> for DipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       15       28        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual_g1[1] * self[e4125]) - (other[e423] * self[e3215]),
                -(right_dual_g1[2] * self[e4235]) - (other[e431] * self[e3215]),
                -(right_dual_g1[0] * self[e4315]) - (other[e412] * self[e3215]),
                (right_dual_g1[3] * self[e1234]) + (other[e412] * self[e4125]),
            ]) + (self.group3().yzxx() * right_dual_g1.zxy().with_w(other[e423]))
                + (right_dual_g2.xyz() * self.group2().www()).with_w(other[e431] * self[e4315]),
            // e5
            -(right_dual_g1[3] * self[e3215]) - (right_dual_g2[0] * self[e4235]) - (right_dual_g2[1] * self[e4315]) - (right_dual_g2[2] * self[e4125]),
        )
    }
}
impl BulkContraction<Dipole> for DipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       24        0
    //    simd3        2        6        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       19       32        0
    //  no simd       29       50        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_dual_g1.xyz()) + (right_dual_g0.zxy() * self.group3().yzx()) - (right_dual_g0.yzx() * self.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual_g0[0] * self[e3215]) + (right_dual_g2[0] * self[e1234]),
                (right_dual_g0[1] * self[e3215]) + (right_dual_g2[1] * self[e1234]),
                (right_dual_g0[2] * self[e3215]) + (right_dual_g2[2] * self[e1234]),
                -(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125]),
            ]) - (right_dual_g1.wwwx() * self.group3().xyzx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (right_dual_g2[1] * self[e4125]) + (right_dual_g1[0] * self[e3215]),
                (right_dual_g2[2] * self[e4235]) + (right_dual_g1[1] * self[e3215]),
                (right_dual_g2[0] * self[e4315]) + (right_dual_g1[2] * self[e3215]),
                -(right_dual_g0[1] * self[e25])
                    - (right_dual_g0[2] * self[e35])
                    - (right_dual_g2[0] * self[e41])
                    - (right_dual_g2[1] * self[e42])
                    - (right_dual_g2[2] * self[e43])
                    - (right_dual_g1[0] * self[e23])
                    - (right_dual_g1[1] * self[e31])
                    - (right_dual_g1[2] * self[e12])
                    - (right_dual_g1[3] * self[e45]),
            ]) - (right_dual_g2.zxy() * self.group3().yzx()).with_w(right_dual_g0[0] * self[e15]),
        )
    }
}
impl BulkContraction<DipoleInversion> for DipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        2        5        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       21       32        0
    //  no simd       37       60        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_dual_g1.xyz()) + (right_dual_g0.zxy() * self.group3().yzx()) - (right_dual_g0.yzx() * self.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual_g0[0] * self[e3215]) + (right_dual_g2[0] * self[e1234]),
                (right_dual_g0[1] * self[e3215]) + (right_dual_g2[1] * self[e1234]),
                (right_dual_g0[2] * self[e3215]) + (right_dual_g2[2] * self[e1234]),
                -(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125]),
            ]) - (right_dual_g1.wwwx() * self.group3().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(self[e3215]) * right_dual_g1.xyz().with_w(right_dual_g2[3]))
                + (self.group3().zxyx() * right_dual_g2.yzx().with_w(right_dual_g3[0]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual_g3[1] * self[e4315]) + (right_dual_g3[2] * self[e4125]) + (right_dual_g3[3] * self[e1234])
                        - (right_dual_g0[1] * self[e25])
                        - (right_dual_g0[2] * self[e35])
                        - (right_dual_g1[0] * self[e23])
                        - (right_dual_g1[1] * self[e31])
                        - (right_dual_g1[2] * self[e12])
                        - (right_dual_g1[3] * self[e45])
                        - (right_dual_g2[0] * self[e41])
                        - (right_dual_g2[1] * self[e42])
                        - (right_dual_g2[2] * self[e43]),
                )
                - (right_dual_g2.zxy() * self.group3().yzx()).with_w(right_dual_g0[0] * self[e15]),
        )
    }
}
impl BulkContraction<DualNum> for DipoleInversion {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x2::from(-1.0);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual_g0[0]) * self.group3().xyz().with_w(self[e1234]),
            // e1, e2, e3, e5
            Simd32x4::from(right_dual_g0[0]) * self.group0().with_w(self[e45]),
        )
    }
}
impl BulkContraction<FlatPoint> for DipoleInversion {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        1        3        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        9       21        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x4::from([self[e1234], self[e1234], self[e1234], 1.0])
                * right_dual_g0
                    .xyz()
                    .with_w(-(right_dual_g0[1] * self[e42]) - (right_dual_g0[2] * self[e43]) - (right_dual_g0[3] * self[e45])))
                - (self.group3().xyz() * right_dual_g0.www()).with_w(right_dual_g0[0] * self[e41]),
            // e15, e25, e35, e3215
            ((right_dual_g0.yzx() * self.group3().zxy()) - (right_dual_g0.zxy() * self.group3().yzx())).with_w(0.0),
        )
    }
}
impl BulkContraction<Flector> for DipoleInversion {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        8        0
    //    simd3        1        4        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        8       14        0
    //  no simd       16       28        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(
                (right_dual_g1[1] * self[e4315]) + (right_dual_g1[2] * self[e4125]) + (right_dual_g1[3] * self[e1234])
                    - (right_dual_g0[1] * self[e42])
                    - (right_dual_g0[2] * self[e43])
                    - (right_dual_g0[3] * self[e45]),
            ) + (right_dual_g0.xyz() * self.group2().www()).with_w(right_dual_g1[0] * self[e4235])
                - (self.group3().xyz() * right_dual_g0.www()).with_w(right_dual_g0[0] * self[e41]),
            // e15, e25, e35, e3215
            ((right_dual_g0.yzx() * self.group3().zxy()) - (right_dual_g0.zxy() * self.group3().yzx())).with_w(0.0),
        )
    }
}
impl BulkContraction<Line> for DipoleInversion {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e1234] * other[e235]) + (self[e4315] * other[e435]),
                (self[e1234] * other[e315]) + (self[e4125] * other[e415]),
                (self[e1234] * other[e125]) + (self[e4235] * other[e425]),
                -(self[e4315] * other[e315]) - (self[e4125] * other[e125]),
            ]) - (self.group3().zxyx() * other.group0().yzx().with_w(other[e235])),
        )
    }
}
impl BulkContraction<Motor> for DipoleInversion {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        6       14        0
    //  no simd       12       28        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual_g1[3]) * self.group3().xyz().with_w(self[e1234]),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[2] * self[e4315]) + (right_dual_g1[0] * self[e1234]),
                (right_dual_g0[0] * self[e4125]) + (right_dual_g1[1] * self[e1234]),
                (right_dual_g0[1] * self[e4235]) + (right_dual_g1[2] * self[e1234]),
                -(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125]),
            ]) + (self.group0() * right_dual_g1.www()).with_w(right_dual_g1[3] * self[e45])
                - (self.group3().zxyx() * right_dual_g0.yzx().with_w(right_dual_g1[0])),
        )
    }
}
impl BulkContraction<MultiVector> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       39        0
    //    simd2        0        1        0
    //    simd3        8       19        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       43       70        0
    //  no simd       89      142        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1 = other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3 = other.group8().with_w(other[e321] * -1.0);
        let right_dual_g6 = other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_dual_g9 = other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g1[0] * self[e4235])
                    + (right_dual_g1[1] * self[e4315])
                    + (right_dual_g1[2] * self[e4125])
                    + (right_dual_g1[3] * self[e3215])
                    + (self[e1234] * other[e3215])
                    - (right_dual_g7[0] * self[e15])
                    - (right_dual_g7[1] * self[e25])
                    - (right_dual_g7[2] * self[e35])
                    - (right_dual_g8[0] * self[e41])
                    - (right_dual_g8[1] * self[e42])
                    - (right_dual_g8[2] * self[e43])
                    - (right_dual_g6[0] * self[e23])
                    - (right_dual_g6[1] * self[e31])
                    - (right_dual_g6[2] * self[e12])
                    - (right_dual_g6[3] * self[e45]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (right_dual_g3 * Simd32x4::from(self[e1234]))
                + (self.group3().yzxy() * other.group6().zxy().with_w(other[e431]))
                + (self.group0() * right_dual_g9.www()).with_w(self[e4235] * other[e423])
                + (right_dual_g9.zxy() * self.group1().yzx()).with_w(self[e4125] * other[e412])
                - (Simd32x4::from(right_dual_g10) * self.group2().xyz().with_w(self[e45]))
                - (right_dual_g9.yzxz() * self.group1().zxy().with_w(self[e43]))
                - (other.group7() * self.group3().www()).with_w(right_dual_g9[0] * self[e41])
                - (self.group3().zxy() * other.group6().yzx()).with_w(right_dual_g9[1] * self[e42]),
            // e5
            (right_dual_g9[0] * self[e15]) + (right_dual_g9[1] * self[e25]) + (right_dual_g9[2] * self[e35]) + (right_dual_g9[3] * self[e45])
                - (right_dual_g3[0] * self[e4235])
                - (right_dual_g3[1] * self[e4315])
                - (right_dual_g3[2] * self[e4125])
                - (right_dual_g3[3] * self[e3215]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g8[1] * self[e4125]) + (right_dual_g6[0] * self[e3215]),
                (right_dual_g8[2] * self[e4235]) + (right_dual_g6[1] * self[e3215]),
                (right_dual_g8[0] * self[e4315]) + (right_dual_g6[2] * self[e3215]),
                -(right_dual_g6[1] * self[e4315]) - (right_dual_g6[2] * self[e4125]),
            ]) + (Simd32x4::from(right_dual_g0[1]) * self.group2().xyz().with_w(self[e45]))
                - (right_dual_g8.zxy() * self.group3().yzx()).with_w(right_dual_g6[0] * self[e4235]),
            // e41, e42, e43
            (Simd32x3::from(right_dual_g0[1]) * self.group0()) + (Simd32x3::from(self[e1234]) * right_dual_g6.xyz()) + (right_dual_g7.zxy() * self.group3().yzx())
                - (right_dual_g7.yzx() * self.group3().zxy()),
            // e23, e31, e12
            (right_dual_g7 * Simd32x3::from(self[e3215])) + (right_dual_g8 * Simd32x3::from(self[e1234])) + (Simd32x3::from(right_dual_g0[1]) * self.group1().xyz())
                - (Simd32x3::from(right_dual_g6[3]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (right_dual_g9.yzxw() * self.group3().zxy().with_w(self[e1234])) - (self.group3().yzxw() * right_dual_g9.zxy().with_w(right_dual_g10)),
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual_g9.xyz()) - (Simd32x3::from(right_dual_g10) * self.group3().xyz()),
            // e235, e315, e125
            (Simd32x3::from(right_dual_g9[3]) * self.group3().xyz()) - (Simd32x3::from(self[e3215]) * right_dual_g9.xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g0[1]) * self.group3(),
            // e1234
            right_dual_g0[1] * self[e1234],
        )
    }
}
impl BulkContraction<Plane> for DipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Scalar::from_groups(
            // scalar
            (right_dual_g0[0] * self[e4235]) + (right_dual_g0[1] * self[e4315]) + (right_dual_g0[2] * self[e4125]) + (right_dual_g0[3] * self[e1234]),
        )
    }
}
impl BulkContraction<RoundPoint> for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       12        0
    //    simd3        1        3        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       11       21        0
    //  no simd       25       45        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other[e4] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual_g0.xyz()) - (Simd32x3::from(right_dual_g1) * self.group3().xyz()),
            // e415, e425, e435, e321
            (right_dual_g0.yzxw() * self.group3().zxy().with_w(self[e1234])) - (self.group3().yzxw() * right_dual_g0.zxy().with_w(right_dual_g1)),
            // e235, e315, e125, e4
            (self.group3().xyz() * right_dual_g0.www()).with_w(-(right_dual_g1 * self[e45]) - (right_dual_g0[1] * self[e42]) - (right_dual_g0[2] * self[e43]))
                - (right_dual_g0.xyzx() * self.group3().www().with_w(self[e41])),
            // e1, e2, e3, e5
            Simd32x4::from([
                -(right_dual_g1 * self[e15]) - (right_dual_g0[1] * self[e12]),
                -(right_dual_g1 * self[e25]) - (right_dual_g0[2] * self[e23]),
                -(right_dual_g1 * self[e35]) - (right_dual_g0[0] * self[e31]),
                (right_dual_g0[1] * self[e25]) + (right_dual_g0[2] * self[e35]),
            ]) + (Simd32x4::from(right_dual_g0[3]) * self.group0().with_w(self[e45]))
                + (right_dual_g0.zxyx() * self.group1().yzx().with_w(self[e15])),
        )
    }
}
impl BulkContraction<Scalar> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
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
impl BulkContraction<Sphere> for DipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        6        0
    //  no simd        4        9        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Scalar::from_groups(
            // scalar
            (right_dual_g0[0] * self[e4235])
                + (right_dual_g0[1] * self[e4315])
                + (right_dual_g0[2] * self[e4125])
                + (right_dual_g0[3] * self[e3215])
                + (self[e1234] * other[e3215]),
        )
    }
}
impl BulkContraction<VersorEven> for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        1        5        0
    //    simd4       10       13        0
    // Totals...
    // yes simd       16       27        0
    //  no simd       48       76        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual_g3.xyz()) - (Simd32x3::from(right_dual_g2[3]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (right_dual_g3.yzxw() * self.group3().zxy().with_w(self[e1234])) - (self.group3().yzxw() * right_dual_g3.zxy().with_w(right_dual_g2[3])),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(
                (right_dual_g0[0] * self[e4235]) + (right_dual_g0[1] * self[e4315]) + (right_dual_g0[2] * self[e4125])
                    - (right_dual_g2[3] * self[e45])
                    - (right_dual_g3[1] * self[e42])
                    - (right_dual_g3[2] * self[e43]),
            ) + (self.group3().xyz() * right_dual_g3.www()).with_w(right_dual_g1[3] * self[e1234])
                - (right_dual_g3.xyzx() * self.group3().www().with_w(self[e41])),
            // e1, e2, e3, e5
            (Simd32x4::from(right_dual_g3[3]) * self.group0().with_w(self[e45]))
                + (right_dual_g3.zxyx() * self.group1().yzx().with_w(self[e15]))
                + (self.group2().wwwy() * right_dual_g2.xyz().with_w(right_dual_g3[1]))
                + (right_dual_g1.zxy() * self.group3().yzx()).with_w(right_dual_g3[2] * self[e35])
                - (Simd32x4::from(self[e3215]) * right_dual_g0.xyz().with_w(right_dual_g1[3]))
                - (right_dual_g2.wwwy() * self.group2().xyz().with_w(self[e4315]))
                - (self.group3().zxyz() * right_dual_g1.yzx().with_w(right_dual_g2[2]))
                - (right_dual_g3.yzx() * self.group1().zxy()).with_w(right_dual_g2[0] * self[e4235]),
        )
    }
}
impl BulkContraction<VersorOdd> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        0
    //    simd3        3        7        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       23       38        0
    //  no simd       47       76        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group3().yzxy() * right_dual_g0.zxy().with_w(right_dual_g3[1]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual_g3[2] * self[e4125]) + (right_dual_g3[3] * self[e3215])
                        - (right_dual_g0[0] * self[e15])
                        - (right_dual_g0[1] * self[e25])
                        - (right_dual_g0[2] * self[e35])
                        - (right_dual_g1[0] * self[e23])
                        - (right_dual_g1[1] * self[e31])
                        - (right_dual_g1[2] * self[e12])
                        - (right_dual_g1[3] * self[e45])
                        - (right_dual_g2[1] * self[e42])
                        - (right_dual_g2[2] * self[e43]),
                )
                + (self.group0() * right_dual_g0.www()).with_w(right_dual_g2[3] * self[e1234])
                + (right_dual_g1.xyz() * self.group2().www()).with_w(right_dual_g3[0] * self[e4235])
                - (right_dual_g0.yzx() * self.group3().zxy()).with_w(right_dual_g2[0] * self[e41]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual_g0[0] * self[e3215]) + (right_dual_g2[0] * self[e1234]),
                (right_dual_g0[1] * self[e3215]) + (right_dual_g2[1] * self[e1234]),
                (right_dual_g0[2] * self[e3215]) + (right_dual_g2[2] * self[e1234]),
                -(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125]),
            ]) + (Simd32x4::from(right_dual_g0[3]) * self.group1())
                - (right_dual_g1.wwwx() * self.group3().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual_g0[3]) * self.group2().xyz()) + (Simd32x3::from(self[e3215]) * right_dual_g1.xyz()) + (right_dual_g2.yzx() * self.group3().zxy())
                - (right_dual_g2.zxy() * self.group3().yzx()))
            .with_w(right_dual_g0[3] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g0[3]) * self.group3(),
        )
    }
}
impl std::ops::Div<BulkContractionInfix> for DualNum {
    type Output = BulkContractionInfixPartial<DualNum>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for DualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       23        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self[e12345]) * (other.group0() * Simd32x3::from(-1.0)).with_w(right_dual_g2[3]),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            right_dual_g2 * self.group0().yy().with_zw(self[e12345], self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}
impl BulkContraction<AntiDipoleInversion> for DualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        7        0
    // no simd        0       28        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().yy().with_zw(self[e12345], self[e5]) * other.group0().with_w(right_dual_g2[3]),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            right_dual_g2 * Simd32x4::from(self[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl BulkContraction<AntiDualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(
            // e5, e12345
            Simd32x2::from([(other[e3215] * self[e12345]) + (other[scalar] * self[e5]), other[scalar] * self[e12345]]),
        )
    }
}
impl BulkContraction<AntiFlatPoint> for DualNum {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self[e12345]) * other.group0().xyz().with_w(other[e321] * -1.0))
    }
}
impl BulkContraction<AntiFlector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl BulkContraction<AntiLine> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        4        0
    // no simd        0       12        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(self[e12345]) * other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * other.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl BulkContraction<AntiMotor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1       18        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e415, e425, e435, e12345
            right_dual_g0 * Simd32x4::from(self[e12345]),
            // e235, e315, e125, e5
            self.group0().yy().with_zw(self[e12345], (right_dual_g0[3] * self[e5]) + (right_dual_g1[3] * self[e12345])) * right_dual_g1.xyz().with_w(1.0),
        )
    }
}
impl BulkContraction<AntiPlane> for DualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self[e12345]) * other.group0().xyz().with_w(other[e5] * -1.0))
    }
}
impl BulkContraction<AntiScalar> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bulk_contraction(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e12345] * self[e12345] * -1.0)
    }
}
impl BulkContraction<Circle> for DualNum {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       14        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
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
impl BulkContraction<CircleRotor> for DualNum {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       19        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
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
impl BulkContraction<Dipole> for DualNum {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       20        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * other.group2() * Simd32x3::from(-1.0),
        )
    }
}
impl BulkContraction<DipoleInversion> for DualNum {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       30        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            Simd32x4::from(self[e12345]) * other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            Simd32x4::from(self[e12345]) * other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl BulkContraction<DualNum> for DualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        4        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(self[e12345]) * Simd32x2::from([other[e5] * -1.0, other[e12345] * -1.0]))
    }
}
impl BulkContraction<FlatPoint> for DualNum {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * Simd32x4::from([other[e15] * -1.0, other[e25] * -1.0, other[e35] * -1.0, other[e45]]),
        )
    }
}
impl BulkContraction<Flector> for DualNum {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl BulkContraction<Line> for DualNum {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(self[e12345]) * other.group0(),
            // e15, e25, e35
            Simd32x3::from(self[e12345]) * other.group1(),
        )
    }
}
impl BulkContraction<Motor> for DualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl BulkContraction<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        8        0
    //    simd2        0        1        0
    //    simd3        0        6        0
    //    simd4        0        7        0
    // Totals...
    // yes simd        2       22        0
    //  no simd        2       56        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(right_dual_g10 * self[e5]) + (right_dual_g0[0] * self[e12345]), right_dual_g0[1] * self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            (right_dual_g0[1] * self[e5]) + (self[e12345] * other[e3215]),
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group7(),
            // e23, e31, e12
            Simd32x3::from(self[e12345]) * other.group6().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            right_dual_g10 * self[e12345],
        )
    }
}
impl BulkContraction<Plane> for DualNum {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn bulk_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(self[e12345]) * Simd32x4::from([other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0, other[e3215]]),
        )
    }
}
impl BulkContraction<RoundPoint> for DualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       11        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other[e4] * -1.0;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(right_dual_g1 * self[e5]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(right_dual_g1 * self[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl BulkContraction<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(other[scalar]) * self.group0())
    }
}
impl BulkContraction<Sphere> for DualNum {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            self[e12345] * other[e3215],
        )
    }
}
impl BulkContraction<VersorEven> for DualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        8        0
    // Totals...
    // yes simd        1       10        0
    //  no simd        1       34        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().yy().with_zw(self[e12345], (right_dual_g0[3] * self[e12345]) + (right_dual_g2[3] * self[e5])) * right_dual_g0.xyz().with_w(1.0),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            right_dual_g2 * Simd32x4::from(self[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl BulkContraction<VersorOdd> for DualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        8        0
    // Totals...
    // yes simd        1       10        0
    //  no simd        1       34        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            right_dual_g0 * Simd32x4::from(self[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group0().yy().with_zw(self[e12345], (right_dual_g0[3] * self[e5]) + (right_dual_g2[3] * self[e12345])) * right_dual_g2.xyz().with_w(1.0),
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<BulkContractionInfix> for FlatPoint {
    type Output = BulkContractionInfixPartial<FlatPoint>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for FlatPoint {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       15        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(right_dual_g2[3] * self[e45]),
            // e15, e25, e35, scalar
            (self.group0().xyz() * right_dual_g2.www())
                .with_w(-(right_dual_g0[0] * self[e15]) - (right_dual_g0[1] * self[e25]) - (right_dual_g0[2] * self[e35]) - (other[e45] * self[e45])),
        )
    }
}
impl BulkContraction<AntiDipoleInversion> for FlatPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       17        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g3 = other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e4] * -1.0) * self.group0() * Simd32x4::from(-1.0),
            // e5
            (right_dual_g3[0] * self[e15]) + (right_dual_g3[1] * self[e25]) + (right_dual_g3[2] * self[e35]) + (right_dual_g3[3] * self[e45]),
        )
    }
}
impl BulkContraction<AntiDualNum> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl BulkContraction<AntiFlector> for FlatPoint {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (right_dual_g1[0] * self[e15]) + (right_dual_g1[1] * self[e25]) + (right_dual_g1[2] * self[e35]) + (right_dual_g1[3] * self[e45]),
            0.0,
        ]))
    }
}
impl BulkContraction<AntiMotor> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl BulkContraction<AntiPlane> for FlatPoint {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (right_dual_g0[0] * self[e15]) + (right_dual_g0[1] * self[e25]) + (right_dual_g0[2] * self[e35]) + (right_dual_g0[3] * self[e45]),
            0.0,
        ]))
    }
}
impl BulkContraction<Dipole> for FlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        7        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        Scalar::from_groups(
            // scalar
            -(right_dual_g0[0] * self[e15]) - (right_dual_g0[1] * self[e25]) - (right_dual_g0[2] * self[e35]) - (other[e45] * self[e45]),
        )
    }
}
impl BulkContraction<DipoleInversion> for FlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        7        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        Scalar::from_groups(
            // scalar
            -(right_dual_g0[0] * self[e15]) - (right_dual_g0[1] * self[e25]) - (right_dual_g0[2] * self[e35]) - (other[e45] * self[e45]),
        )
    }
}
impl BulkContraction<DualNum> for FlatPoint {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        4        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([other[e5] * self[e45] * -1.0, 1.0]) * Simd32x2::from([1.0, 0.0]))
    }
}
impl BulkContraction<FlatPoint> for FlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e45] * self[e45] * -1.0)
    }
}
impl BulkContraction<Flector> for FlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e45] * other[e45] * -1.0)
    }
}
impl BulkContraction<Motor> for FlatPoint {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        4        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e45] * other[e5] * -1.0, 1.0]) * Simd32x2::from([1.0, 0.0]))
    }
}
impl BulkContraction<MultiVector> for FlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        0
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       14        0
    //  no simd        6       28        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g9 = other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(right_dual_g7[0] * self[e15]) - (right_dual_g7[1] * self[e25]) - (right_dual_g7[2] * self[e35]) - (self[e45] * other[e45]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other[e4] * -1.0) * self.group0() * Simd32x4::from(-1.0),
            // e5
            (right_dual_g9[0] * self[e15]) + (right_dual_g9[1] * self[e25]) + (right_dual_g9[2] * self[e35]) + (right_dual_g9[3] * self[e45]),
            // e15, e25, e35, e45
            Simd32x4::from(other.group0().yx()[1]) * self.group0(),
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
impl BulkContraction<RoundPoint> for FlatPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       17        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e4] * -1.0) * self.group0() * Simd32x4::from(-1.0),
            // e5
            (right_dual_g0[0] * self[e15]) + (right_dual_g0[1] * self[e25]) + (right_dual_g0[2] * self[e35]) + (right_dual_g0[3] * self[e45]),
        )
    }
}
impl BulkContraction<Scalar> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl BulkContraction<VersorEven> for FlatPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       17        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g3 = other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e4] * -1.0) * self.group0() * Simd32x4::from(-1.0),
            // e5
            (right_dual_g3[0] * self[e15]) + (right_dual_g3[1] * self[e25]) + (right_dual_g3[2] * self[e35]) + (right_dual_g3[3] * self[e45]),
        )
    }
}
impl BulkContraction<VersorOdd> for FlatPoint {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       12        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(right_dual_g0[3] * self[e45]),
            // e15, e25, e35, scalar
            (Simd32x3::from(right_dual_g0[3]) * self.group0().xyz())
                .with_w(-(right_dual_g0[0] * self[e15]) - (right_dual_g0[1] * self[e25]) - (right_dual_g0[2] * self[e35]) - (self[e45] * other[e45])),
        )
    }
}
impl std::ops::Div<BulkContractionInfix> for Flector {
    type Output = BulkContractionInfixPartial<Flector>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        7        0
    //    simd3        1        8        0
    //    simd4        5        4        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       27       47        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((right_dual_g0.zxy() * self.group1().yzx()) - (right_dual_g0.yzx() * self.group1().zxy()))
                .with_w(-(right_dual_g0[0] * self[e15]) - (right_dual_g0[1] * self[e25]) - (right_dual_g0[2] * self[e35]) - (right_dual_g1[3] * self[e45])),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125]))
                + (right_dual_g0 * self.group1().www()).with_w(right_dual_g2[3] * self[e45])
                - (right_dual_g1.wwwx() * self.group1().xyzx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(right_dual_g2[3]) * self.group0().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e3215]) * right_dual_g1.xyz()).with_w(0.0)
                + (right_dual_g2.yzx() * self.group1().zxy()).with_w(0.0)
                - (right_dual_g2.zxy() * self.group1().yzx()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g2[3]) * self.group1(),
        )
    }
}
impl BulkContraction<AntiDipoleInversion> for Flector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd3        1        6        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       31       56        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual_g2[3]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            ((right_dual_g3.yzx() * self.group1().zxy()) - (right_dual_g3.zxy() * self.group1().yzx())).with_w(right_dual_g2[3] * self[e3215] * -1.0),
            // e235, e315, e125, e4
            (self.group1().xyzx() * right_dual_g3.www().with_w(other[e423])) + Simd32x3::from(0.0).with_w((other[e431] * self[e4315]) + (other[e412] * self[e4125]))
                - (right_dual_g3.xyz() * self.group1().www()).with_w(right_dual_g2[3] * self[e45]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w((right_dual_g3[1] * self[e25]) + (right_dual_g3[2] * self[e35]) + (right_dual_g3[3] * self[e45]) - (right_dual_g2[2] * self[e4125]))
                + (right_dual_g1.zxy() * self.group1().yzx()).with_w(right_dual_g3[0] * self[e15])
                - (Simd32x4::from(self[e3215]) * other.group0().with_w(right_dual_g1[3]))
                - (right_dual_g2.wwwy() * self.group0().xyz().with_w(self[e4315]))
                - (self.group1().zxyx() * right_dual_g1.yzx().with_w(right_dual_g2[0])),
        )
    }
}
impl BulkContraction<AntiDualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[scalar]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl BulkContraction<AntiFlatPoint> for Flector {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            -(right_dual_g0[0] * self[e4235]) - (right_dual_g0[1] * self[e4315]) - (right_dual_g0[2] * self[e4125]) - (right_dual_g0[3] * self[e3215]),
            0.0,
        ]))
    }
}
impl BulkContraction<AntiFlector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        1        2        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        8       12        0
    //  no simd       16       28        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((right_dual_g1.yzx() * self.group1().zxy()) - (right_dual_g1.zxy() * self.group1().yzx())).with_w(0.0),
            // e235, e315, e125, e5
            (right_dual_g1.wwwx() * self.group1().xyz().with_w(self[e15]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual_g1[1] * self[e25]) + (right_dual_g1[2] * self[e35]) + (right_dual_g1[3] * self[e45])
                        - (right_dual_g0[1] * self[e4315])
                        - (right_dual_g0[2] * self[e4125])
                        - (right_dual_g0[3] * self[e3215]),
                )
                - (self.group1().wwwx() * right_dual_g1.xyz().with_w(right_dual_g0[0])),
        )
    }
}
impl BulkContraction<AntiLine> for Flector {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       18        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g0[0] * self[e3215]) + (right_dual_g1[1] * self[e4125]),
                (right_dual_g0[1] * self[e3215]) + (right_dual_g1[2] * self[e4235]),
                (right_dual_g0[2] * self[e3215]) + (right_dual_g1[0] * self[e4315]),
                -(right_dual_g0[1] * self[e4315]) - (right_dual_g0[2] * self[e4125]),
            ]) - (self.group1().yzxx() * right_dual_g1.zxy().with_w(right_dual_g0[0])),
        )
    }
}
impl BulkContraction<AntiMotor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        6       13        0
    //  no simd       12       28        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g0[0] * self[e3215]) + (right_dual_g1[1] * self[e4125]),
                (right_dual_g0[1] * self[e3215]) + (right_dual_g1[2] * self[e4235]),
                (right_dual_g0[2] * self[e3215]) + (right_dual_g1[0] * self[e4315]),
                -(right_dual_g0[1] * self[e4315]) - (right_dual_g0[2] * self[e4125]),
            ]) + (Simd32x4::from(right_dual_g0[3]) * self.group0())
                - (self.group1().yzxx() * right_dual_g1.zxy().with_w(right_dual_g0[0])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g0[3]) * self.group1(),
        )
    }
}
impl BulkContraction<AntiPlane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        1        2        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        9       25        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((right_dual_g0.yzx() * self.group1().zxy()) - (right_dual_g0.zxy() * self.group1().yzx())).with_w(0.0),
            // e235, e315, e125, e5
            (Simd32x4::from([self[e3215], self[e3215], self[e3215], 1.0])
                * right_dual_g0
                    .xyz()
                    .with_w((right_dual_g0[1] * self[e25]) + (right_dual_g0[2] * self[e35]) + (right_dual_g0[3] * self[e45]))
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
                + (right_dual_g0.wwwx() * self.group1().xyz().with_w(self[e15])),
        )
    }
}
impl BulkContraction<Circle> for Flector {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        8       14        0
    //  no simd       11       20        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual_g1[1] * self[e4125]) - (other[e423] * self[e3215]),
                -(right_dual_g1[2] * self[e4235]) - (other[e431] * self[e3215]),
                -(right_dual_g1[0] * self[e4315]) - (other[e412] * self[e3215]),
                (other[e431] * self[e4315]) + (other[e412] * self[e4125]),
            ]) + (self.group1().yzxx() * right_dual_g1.zxy().with_w(other[e423])),
            // e5
            -(right_dual_g1[3] * self[e3215]) - (other[e235] * self[e4235]) - (other[e315] * self[e4315]) - (other[e125] * self[e4125]),
        )
    }
}
impl BulkContraction<CircleRotor> for Flector {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        8       15        0
    //  no simd       11       24        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual_g1[1] * self[e4125]) - (other[e423] * self[e3215]),
                -(right_dual_g1[2] * self[e4235]) - (other[e431] * self[e3215]),
                -(right_dual_g1[0] * self[e4315]) - (other[e412] * self[e3215]),
                (other[e431] * self[e4315]) + (other[e412] * self[e4125]),
            ]) + (self.group1().yzxx() * right_dual_g1.zxy().with_w(other[e423])),
            // e5
            -(right_dual_g1[3] * self[e3215]) - (right_dual_g2[0] * self[e4235]) - (right_dual_g2[1] * self[e4315]) - (right_dual_g2[2] * self[e4125]),
        )
    }
}
impl BulkContraction<Dipole> for Flector {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       12        0
    //    simd3        1        5        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       20        0
    //  no simd       17       39        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (right_dual_g0.zxy() * self.group1().yzx()) - (right_dual_g0.yzx() * self.group1().zxy()),
            // e23, e31, e12, e45
            (Simd32x4::from([self[e3215], self[e3215], self[e3215], 1.0]) * right_dual_g0.with_w(-(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125])))
                - (right_dual_g1.wwwx() * self.group1().xyzx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (right_dual_g2[1] * self[e4125]) + (right_dual_g1[0] * self[e3215]),
                (right_dual_g2[2] * self[e4235]) + (right_dual_g1[1] * self[e3215]),
                (right_dual_g2[0] * self[e4315]) + (right_dual_g1[2] * self[e3215]),
                -(right_dual_g0[1] * self[e25]) - (right_dual_g0[2] * self[e35]) - (right_dual_g1[3] * self[e45]),
            ]) - (right_dual_g2.zxy() * self.group1().yzx()).with_w(right_dual_g0[0] * self[e15]),
        )
    }
}
impl BulkContraction<DipoleInversion> for Flector {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        8        0
    //    simd3        1        4        0
    //    simd4        4        7        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       24       48        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (right_dual_g0.zxy() * self.group1().yzx()) - (right_dual_g0.yzx() * self.group1().zxy()),
            // e23, e31, e12, e45
            (Simd32x4::from([self[e3215], self[e3215], self[e3215], 1.0]) * right_dual_g0.with_w(-(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125])))
                - (right_dual_g1.wwwx() * self.group1().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(self[e3215]) * right_dual_g1.xyz().with_w(right_dual_g2[3]))
                + (self.group1().zxyx() * right_dual_g2.yzx().with_w(right_dual_g3[0]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual_g3[1] * self[e4315]) + (right_dual_g3[2] * self[e4125])
                        - (right_dual_g0[1] * self[e25])
                        - (right_dual_g0[2] * self[e35])
                        - (right_dual_g1[3] * self[e45]),
                )
                - (right_dual_g2.zxy() * self.group1().yzx()).with_w(right_dual_g0[0] * self[e15]),
        )
    }
}
impl BulkContraction<DualNum> for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x2::from(-1.0);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (self.group1().xyz() * right_dual_g0.xx().with_z(right_dual_g0[0])).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(right_dual_g0[0] * self[e45]),
        )
    }
}
impl BulkContraction<FlatPoint> for Flector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        5        0
    //  no simd        3       18        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(right_dual_g0[3]) * self.group1().xyz().with_w(self[e45]) * Simd32x4::from(-1.0),
            // e15, e25, e35, e3215
            ((right_dual_g0.yzx() * self.group1().zxy()) - (right_dual_g0.zxy() * self.group1().yzx())).with_w(0.0),
        )
    }
}
impl BulkContraction<Flector> for Flector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        1        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        6       24        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (self.group1().xyz() * right_dual_g0.www() * Simd32x3::from(-1.0))
                .with_w((right_dual_g1[0] * self[e4235]) + (right_dual_g1[1] * self[e4315]) + (right_dual_g1[2] * self[e4125]) - (right_dual_g0[3] * self[e45])),
            // e15, e25, e35, e3215
            ((right_dual_g0.yzx() * self.group1().zxy()) - (right_dual_g0.zxy() * self.group1().yzx())).with_w(0.0),
        )
    }
}
impl BulkContraction<Line> for Flector {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        5        9        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                self[e4315] * other[e435],
                self[e4125] * other[e415],
                self[e4235] * other[e425],
                -(self[e4315] * other[e315]) - (self[e4125] * other[e125]),
            ]) - (self.group1().zxyx() * other.group0().yzx().with_w(other[e235])),
        )
    }
}
impl BulkContraction<Motor> for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        9       21        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (self.group1().xyz() * right_dual_g1.www()).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125]))
                + (right_dual_g0.zxy() * self.group1().yzx()).with_w(right_dual_g1[3] * self[e45])
                - (self.group1().zxyx() * right_dual_g0.yzx().with_w(right_dual_g1[0])),
        )
    }
}
impl BulkContraction<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       37        0
    //    simd2        0        1        0
    //    simd3        4       14        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       30       58        0
    //  no simd       50      105        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1 = other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g6 = other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_dual_g9 = other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g1[0] * self[e4235]) + (right_dual_g1[1] * self[e4315]) + (right_dual_g1[2] * self[e4125]) + (right_dual_g1[3] * self[e3215])
                    - (right_dual_g7[0] * self[e15])
                    - (right_dual_g7[1] * self[e25])
                    - (right_dual_g7[2] * self[e35])
                    - (right_dual_g6[3] * self[e45]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual_g10 * self[e15]) - (self[e4125] * other[e425]),
                -(right_dual_g10 * self[e25]) - (self[e4235] * other[e435]),
                -(right_dual_g10 * self[e35]) - (self[e4315] * other[e415]),
                (self[e4315] * other[e431]) + (self[e4125] * other[e412]),
            ]) + (self.group1().yzxx() * other.group6().zxy().with_w(other[e423]))
                - (other.group7() * self.group1().www()).with_w(right_dual_g10 * self[e45]),
            // e5
            (right_dual_g9[0] * self[e15]) + (right_dual_g9[1] * self[e25]) + (right_dual_g9[2] * self[e35]) + (right_dual_g9[3] * self[e45]) + (self[e3215] * other[e321])
                - (self[e4235] * other[e235])
                - (self[e4315] * other[e315])
                - (self[e4125] * other[e125]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g8[1] * self[e4125]) + (right_dual_g6[0] * self[e3215]),
                (right_dual_g8[2] * self[e4235]) + (right_dual_g6[1] * self[e3215]),
                (right_dual_g8[0] * self[e4315]) + (right_dual_g6[2] * self[e3215]),
                -(right_dual_g6[1] * self[e4315]) - (right_dual_g6[2] * self[e4125]),
            ]) + (Simd32x4::from(right_dual_g0[1]) * self.group0())
                - (right_dual_g8.zxy() * self.group1().yzx()).with_w(right_dual_g6[0] * self[e4235]),
            // e41, e42, e43
            (right_dual_g7.zxy() * self.group1().yzx()) - (right_dual_g7.yzx() * self.group1().zxy()),
            // e23, e31, e12
            (right_dual_g7 * Simd32x3::from(self[e3215])) - (Simd32x3::from(right_dual_g6[3]) * self.group1().xyz()),
            // e415, e425, e435, e321
            ((right_dual_g9.yzx() * self.group1().zxy()) - (right_dual_g9.zxy() * self.group1().yzx())).with_w(right_dual_g10 * self[e3215] * -1.0),
            // e423, e431, e412
            Simd32x3::from(right_dual_g10) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            (Simd32x3::from(right_dual_g9[3]) * self.group1().xyz()) - (Simd32x3::from(self[e3215]) * right_dual_g9.xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g0[1]) * self.group1(),
            // e1234
            0.0,
        )
    }
}
impl BulkContraction<Plane> for Flector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn bulk_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Scalar::from_groups(/* scalar */ (right_dual_g0[0] * self[e4235]) + (right_dual_g0[1] * self[e4315]) + (right_dual_g0[2] * self[e4125]))
    }
}
impl BulkContraction<RoundPoint> for Flector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        2        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       18        0
    //  no simd        9       37        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other[e4] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual_g1) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            ((right_dual_g0.yzx() * self.group1().zxy()) - (right_dual_g0.zxy() * self.group1().yzx())).with_w(right_dual_g1 * self[e3215] * -1.0),
            // e235, e315, e125, e4
            ((Simd32x3::from(right_dual_g0[3]) * self.group1().xyz()) - (Simd32x3::from(self[e3215]) * right_dual_g0.xyz())).with_w(right_dual_g1 * self[e45] * -1.0),
            // e1, e2, e3, e5
            (Simd32x3::from(right_dual_g1) * self.group0().xyz() * Simd32x3::from(-1.0))
                .with_w((right_dual_g0[0] * self[e15]) + (right_dual_g0[1] * self[e25]) + (right_dual_g0[2] * self[e35]) + (right_dual_g0[3] * self[e45])),
        )
    }
}
impl BulkContraction<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[scalar]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl BulkContraction<Sphere> for Flector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Scalar::from_groups(
            // scalar
            (right_dual_g0[0] * self[e4235]) + (right_dual_g0[1] * self[e4315]) + (right_dual_g0[2] * self[e4125]) + (right_dual_g0[3] * self[e3215]),
        )
    }
}
impl BulkContraction<VersorEven> for Flector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd3        1        6        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       11       24        0
    //  no simd       31       60        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual_g2[3]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            ((right_dual_g3.yzx() * self.group1().zxy()) - (right_dual_g3.zxy() * self.group1().yzx())).with_w(right_dual_g2[3] * self[e3215] * -1.0),
            // e235, e315, e125, e4
            (self.group1().xyzx() * right_dual_g3.www().with_w(right_dual_g0[0])) + Simd32x3::from(0.0).with_w((right_dual_g0[1] * self[e4315]) + (right_dual_g0[2] * self[e4125]))
                - (right_dual_g3.xyz() * self.group1().www()).with_w(right_dual_g2[3] * self[e45]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w((right_dual_g3[1] * self[e25]) + (right_dual_g3[2] * self[e35]) + (right_dual_g3[3] * self[e45]) - (right_dual_g1[3] * self[e3215]))
                + (right_dual_g1.zxy() * self.group1().yzx()).with_w(right_dual_g3[0] * self[e15])
                - (right_dual_g2.wwwx() * self.group0().xyz().with_w(self[e4235]))
                - (self.group1().zxyy() * right_dual_g1.yzx().with_w(right_dual_g2[1]))
                - (self.group1().wwwz() * right_dual_g0.xyz().with_w(right_dual_g2[2])),
        )
    }
}
impl BulkContraction<VersorOdd> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd3        0        4        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       13       21        0
    //  no simd       34       56        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group1().yzxx() * right_dual_g0.zxy().with_w(right_dual_g3[0]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual_g3[1] * self[e4315]) + (right_dual_g3[2] * self[e4125]) + (right_dual_g3[3] * self[e3215])
                        - (right_dual_g0[1] * self[e25])
                        - (right_dual_g0[2] * self[e35])
                        - (right_dual_g1[3] * self[e45]),
                )
                - (right_dual_g0.yzxx() * self.group1().zxy().with_w(self[e15])),
            // e23, e31, e12, e45
            (right_dual_g0 * self.group1().www().with_w(self[e45])) + Simd32x3::from(0.0).with_w(-(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125]))
                - (right_dual_g1.wwwx() * self.group1().xyzx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(right_dual_g0[3]) * self.group0().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e3215]) * right_dual_g1.xyz()).with_w(0.0)
                + (right_dual_g2.yzx() * self.group1().zxy()).with_w(0.0)
                - (right_dual_g2.zxy() * self.group1().yzx()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g0[3]) * self.group1(),
        )
    }
}
impl std::ops::Div<BulkContractionInfix> for Line {
    type Output = BulkContractionInfixPartial<Line>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for Line {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        4        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       10       21        0
    //  no simd       13       35        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (self.group0() * right_dual_g2.www()).with_w(0.0),
            // e235, e315, e125, e4
            (Simd32x3::from(right_dual_g2[3]) * self.group1()).with_w(-(right_dual_g0[0] * self[e415]) - (right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g1[3] * self[e415]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g1[3] * self[e425]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g1[3] * self[e435]),
                -(right_dual_g1[0] * self[e235])
                    - (right_dual_g1[1] * self[e315])
                    - (right_dual_g1[2] * self[e125])
                    - (right_dual_g2[1] * self[e425])
                    - (right_dual_g2[2] * self[e435]),
            ]) - (right_dual_g0.yzx() * self.group1().zxy()).with_w(right_dual_g2[0] * self[e415]),
        )
    }
}
impl BulkContraction<AntiDipoleInversion> for Line {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       10       21        0
    //  no simd       13       36        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual_g2[3]) * self.group0(),
            // e23, e31, e12, e45
            (Simd32x3::from(right_dual_g2[3]) * self.group1()).with_w(-(right_dual_g3[0] * self[e415]) - (right_dual_g3[1] * self[e425]) - (right_dual_g3[2] * self[e435])),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (right_dual_g3[2] * self[e315]) + (right_dual_g3[3] * self[e415]),
                (right_dual_g3[0] * self[e125]) + (right_dual_g3[3] * self[e425]),
                (right_dual_g3[1] * self[e235]) + (right_dual_g3[3] * self[e435]),
                -(right_dual_g1[0] * self[e415]) - (right_dual_g1[1] * self[e425]) - (right_dual_g1[2] * self[e435]) - (other[e431] * self[e315]) - (other[e412] * self[e125]),
            ]) - (self.group1().zxy() * right_dual_g3.yzx()).with_w(other[e423] * self[e235]),
        )
    }
}
impl BulkContraction<AntiDualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(other[scalar]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * self.group1(),
        )
    }
}
impl BulkContraction<AntiFlector> for Line {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g1[2] * self[e315]) + (right_dual_g1[3] * self[e415]),
                (right_dual_g1[0] * self[e125]) + (right_dual_g1[3] * self[e425]),
                (right_dual_g1[1] * self[e235]) + (right_dual_g1[3] * self[e435]),
                -(right_dual_g1[1] * self[e425]) - (right_dual_g1[2] * self[e435]),
            ]) - (right_dual_g1.yzxx() * self.group1().zxy().with_w(self[e415])),
        )
    }
}
impl BulkContraction<AntiLine> for Line {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            -(right_dual_g0[0] * self[e235])
                - (right_dual_g0[1] * self[e315])
                - (right_dual_g0[2] * self[e125])
                - (right_dual_g1[0] * self[e415])
                - (right_dual_g1[1] * self[e425])
                - (right_dual_g1[2] * self[e435]),
            0.0,
        ]))
    }
}
impl BulkContraction<AntiMotor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       20        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e415, e425, e435, e12345
            (self.group0() * right_dual_g0.www()).with_w(0.0),
            // e235, e315, e125, e5
            (Simd32x3::from(right_dual_g0[3]) * self.group1()).with_w(
                -(right_dual_g0[0] * self[e235])
                    - (right_dual_g0[1] * self[e315])
                    - (right_dual_g0[2] * self[e125])
                    - (right_dual_g1[0] * self[e415])
                    - (right_dual_g1[1] * self[e425])
                    - (right_dual_g1[2] * self[e435]),
            ),
        )
    }
}
impl BulkContraction<AntiPlane> for Line {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g0[3] * self[e415]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g0[3] * self[e425]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g0[3] * self[e435]),
                -(right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]),
            ]) - (right_dual_g0.yzxx() * self.group1().zxy().with_w(self[e415])),
        )
    }
}
impl BulkContraction<Circle> for Line {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        7        0
    //  no simd        5       10        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(
            // scalar
            -(right_dual_g1[0] * self[e415])
                - (right_dual_g1[1] * self[e425])
                - (right_dual_g1[2] * self[e435])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125]),
        )
    }
}
impl BulkContraction<CircleRotor> for Line {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        7        0
    //  no simd        5       10        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(
            // scalar
            -(right_dual_g1[0] * self[e415])
                - (right_dual_g1[1] * self[e425])
                - (right_dual_g1[2] * self[e435])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125]),
        )
    }
}
impl BulkContraction<Dipole> for Line {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       13       28        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g1[3] * self[e415]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g1[3] * self[e425]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g1[3] * self[e435]),
                -(right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]),
            ]) - (right_dual_g0.yzx() * self.group1().zxy()).with_w(right_dual_g0[0] * self[e415]),
            // e5
            -(right_dual_g2[0] * self[e415])
                - (right_dual_g2[1] * self[e425])
                - (right_dual_g2[2] * self[e435])
                - (right_dual_g1[0] * self[e235])
                - (right_dual_g1[1] * self[e315])
                - (right_dual_g1[2] * self[e125]),
        )
    }
}
impl BulkContraction<DipoleInversion> for Line {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       13       29        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g1[3] * self[e415]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g1[3] * self[e425]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g1[3] * self[e435]),
                -(right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]),
            ]) - (right_dual_g0.yzx() * self.group1().zxy()).with_w(right_dual_g0[0] * self[e415]),
            // e5
            -(right_dual_g1[0] * self[e235])
                - (right_dual_g1[1] * self[e315])
                - (right_dual_g1[2] * self[e125])
                - (right_dual_g2[0] * self[e415])
                - (right_dual_g2[1] * self[e425])
                - (right_dual_g2[2] * self[e435]),
        )
    }
}
impl BulkContraction<DualNum> for Line {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        let right_dual_g0 = other.group0() * Simd32x2::from(-1.0);
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ (self.group0() * right_dual_g0.xx().with_z(right_dual_g0[0])).with_w(0.0))
    }
}
impl BulkContraction<FlatPoint> for Line {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2       10        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x3::from(right_dual_g0[3]) * self.group0()).with_w(-(right_dual_g0[0] * self[e415]) - (right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435])),
        )
    }
}
impl BulkContraction<Flector> for Line {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2       10        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x3::from(right_dual_g0[3]) * self.group0()).with_w(-(right_dual_g0[0] * self[e415]) - (right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435])),
        )
    }
}
impl BulkContraction<Line> for Line {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -(other[e415] * self[e415]) - (other[e425] * self[e425]) - (other[e435] * self[e435]))
    }
}
impl BulkContraction<Motor> for Line {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       14        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(right_dual_g0[0] * self[e415]) - (right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435])),
            // e15, e25, e35, e3215
            (self.group0() * (other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0])).www()).with_w(0.0),
        )
    }
}
impl BulkContraction<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       30        0
    //    simd2        0        1        0
    //    simd3        0        7        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       20       41        0
    //  no simd       26       65        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g6 = other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_dual_g9 = other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(self[e415] * other[e415])
                    - (self[e425] * other[e425])
                    - (self[e435] * other[e435])
                    - (self[e235] * other[e423])
                    - (self[e315] * other[e431])
                    - (self[e125] * other[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g7[2] * self[e315]) + (right_dual_g6[3] * self[e415]),
                (right_dual_g7[0] * self[e125]) + (right_dual_g6[3] * self[e425]),
                (right_dual_g7[1] * self[e235]) + (right_dual_g6[3] * self[e435]),
                -(right_dual_g7[1] * self[e425]) - (right_dual_g7[2] * self[e435]),
            ]) - (right_dual_g7.yzx() * self.group1().zxy()).with_w(right_dual_g7[0] * self[e415]),
            // e5
            -(right_dual_g8[0] * self[e415])
                - (right_dual_g8[1] * self[e425])
                - (right_dual_g8[2] * self[e435])
                - (right_dual_g6[0] * self[e235])
                - (right_dual_g6[1] * self[e315])
                - (right_dual_g6[2] * self[e125]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g9[2] * self[e315]) + (right_dual_g9[3] * self[e415]),
                (right_dual_g9[0] * self[e125]) + (right_dual_g9[3] * self[e425]),
                (right_dual_g9[1] * self[e235]) + (right_dual_g9[3] * self[e435]),
                -(right_dual_g9[1] * self[e425]) - (right_dual_g9[2] * self[e435]),
            ]) - (right_dual_g9.yzxx() * self.group1().zxy().with_w(self[e415])),
            // e41, e42, e43
            Simd32x3::from(right_dual_g10) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(right_dual_g10) * self.group1(),
            // e415, e425, e435, e321
            (self.group0() * right_dual_g0.yy().with_z(right_dual_g0[1])).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(right_dual_g0[1]) * self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl BulkContraction<RoundPoint> for Line {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        2        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        8       23        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other[e4] * -1.0;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual_g1) * self.group0(),
            // e23, e31, e12, e45
            (Simd32x3::from(right_dual_g1) * self.group1()).with_w(-(right_dual_g0[0] * self[e415]) - (right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435])),
            // e15, e25, e35
            (Simd32x3::from(right_dual_g0[3]) * self.group0()) + (self.group1().yzx() * right_dual_g0.zxy()) - (self.group1().zxy() * right_dual_g0.yzx()),
        )
    }
}
impl BulkContraction<Scalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(other[scalar]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * self.group1(),
        )
    }
}
impl BulkContraction<VersorEven> for Line {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       10       22        0
    //  no simd       13       40        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual_g2[3]) * self.group0(),
            // e23, e31, e12, e45
            (Simd32x3::from(right_dual_g2[3]) * self.group1()).with_w(-(right_dual_g3[0] * self[e415]) - (right_dual_g3[1] * self[e425]) - (right_dual_g3[2] * self[e435])),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (right_dual_g3[2] * self[e315]) + (right_dual_g3[3] * self[e415]),
                (right_dual_g3[0] * self[e125]) + (right_dual_g3[3] * self[e425]),
                (right_dual_g3[1] * self[e235]) + (right_dual_g3[3] * self[e435]),
                -(right_dual_g0[0] * self[e235])
                    - (right_dual_g0[1] * self[e315])
                    - (right_dual_g0[2] * self[e125])
                    - (right_dual_g1[1] * self[e425])
                    - (right_dual_g1[2] * self[e435]),
            ]) - (self.group1().zxy() * right_dual_g3.yzx()).with_w(right_dual_g1[0] * self[e415]),
        )
    }
}
impl BulkContraction<VersorOdd> for Line {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       10       21        0
    //  no simd       13       36        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (self.group0() * right_dual_g0.www()).with_w(0.0),
            // e235, e315, e125, e4
            (Simd32x3::from(right_dual_g0[3]) * self.group1()).with_w(-(right_dual_g0[0] * self[e415]) - (right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g1[3] * self[e415]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g1[3] * self[e425]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g1[3] * self[e435]),
                -(right_dual_g1[0] * self[e235])
                    - (right_dual_g1[1] * self[e315])
                    - (right_dual_g1[2] * self[e125])
                    - (right_dual_g2[1] * self[e425])
                    - (right_dual_g2[2] * self[e435]),
            ]) - (self.group1().zxy() * right_dual_g0.yzx()).with_w(right_dual_g2[0] * self[e415]),
        )
    }
}
impl std::ops::Div<BulkContractionInfix> for Motor {
    type Output = BulkContractionInfixPartial<Motor>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       16        0
    //    simd3        1        5        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       12       25        0
    //  no simd       20       47        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self[e12345]) * right_dual_g0.with_w(right_dual_g2[3]),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_dual_g2[3]) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * right_dual_g1.xyz())).with_w(right_dual_g1[3] * self[e12345]),
            // e235, e315, e125, e5
            (right_dual_g2 * self.group0().www().with_w(self[e5]))
                + (self.group1().xyz() * right_dual_g2.www()).with_w(
                    -(right_dual_g1[0] * self[e235])
                        - (right_dual_g1[1] * self[e315])
                        - (right_dual_g1[2] * self[e125])
                        - (right_dual_g2[0] * self[e415])
                        - (right_dual_g2[1] * self[e425])
                        - (right_dual_g2[2] * self[e435]),
                ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g1[3] * self[e415]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g1[3] * self[e425]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g1[3] * self[e435]),
                -(right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]),
            ]) - (right_dual_g0.yzx() * self.group1().zxy()).with_w(right_dual_g0[0] * self[e415]),
        )
    }
}
impl BulkContraction<AntiDipoleInversion> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       11        0
    //    simd3        4        7        0
    //    simd4        1        5        0
    // Totals...
    // yes simd       13       23        0
    //  no simd       24       52        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((other.group0() * self.group0().www()) + (self.group0().xyz() * right_dual_g2.www())).with_w(
                (right_dual_g2[3] * self[e5])
                    - (right_dual_g1[0] * self[e415])
                    - (right_dual_g1[1] * self[e425])
                    - (right_dual_g1[2] * self[e435])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
            ),
            // e23, e31, e12, e45
            (right_dual_g1 * Simd32x4::from(self[e12345]))
                + (self.group1().xyz() * right_dual_g2.www()).with_w(-(right_dual_g3[0] * self[e415]) - (right_dual_g3[1] * self[e425]) - (right_dual_g3[2] * self[e435])),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual_g3[3]) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * right_dual_g2.xyz()) + (right_dual_g3.zxy() * self.group1().yzx())
                - (right_dual_g3.yzx() * self.group1().zxy()))
            .with_w(right_dual_g2[3] * self[e12345]),
            // e4235, e4315, e4125, e3215
            right_dual_g3 * Simd32x4::from(self[e12345]),
        )
    }
}
impl BulkContraction<AntiDualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1       10        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(other[scalar]) * self.group0(),
            // e235, e315, e125, e5
            other.group0().yy().with_zw(other[scalar], (other[e3215] * self[e12345]) + (other[scalar] * self[e5])) * self.group1().xyz().with_w(1.0),
        )
    }
}
impl BulkContraction<AntiFlatPoint> for Motor {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self[e12345]) * other.group0().xyz().with_w(other[e321] * -1.0))
    }
}
impl BulkContraction<AntiFlector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        6       13        0
    //  no simd       12       28        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g1[2] * self[e315]) + (right_dual_g1[3] * self[e415]),
                (right_dual_g1[0] * self[e125]) + (right_dual_g1[3] * self[e425]),
                (right_dual_g1[1] * self[e235]) + (right_dual_g1[3] * self[e435]),
                -(right_dual_g1[1] * self[e425]) - (right_dual_g1[2] * self[e435]),
            ]) + (Simd32x4::from(self[e12345]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
                - (right_dual_g1.yzxx() * self.group1().zxy().with_w(self[e415])),
            // e4235, e4315, e4125, e3215
            right_dual_g1 * Simd32x4::from(self[e12345]),
        )
    }
}
impl BulkContraction<AntiLine> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       18        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        Motor::from_groups(
            // e415, e425, e435, e12345
            (right_dual_g0 * self.group0().www()).with_w(0.0),
            // e235, e315, e125, e5
            (right_dual_g1 * Simd32x3::from(self[e12345])).with_w(
                -(right_dual_g0[0] * self[e235])
                    - (right_dual_g0[1] * self[e315])
                    - (right_dual_g0[2] * self[e125])
                    - (right_dual_g1[0] * self[e415])
                    - (right_dual_g1[1] * self[e425])
                    - (right_dual_g1[2] * self[e435]),
            ),
        )
    }
}
impl BulkContraction<AntiMotor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        2        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       16       29        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((Simd32x3::from(right_dual_g0[3]) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * right_dual_g0.xyz())).with_w(right_dual_g0[3] * self[e12345]),
            // e235, e315, e125, e5
            (right_dual_g1 * Simd32x4::from(self[e12345]))
                + (Simd32x4::from(right_dual_g0[3]) * self.group1())
                + Simd32x3::from(0.0).with_w(
                    -(right_dual_g0[0] * self[e235])
                        - (right_dual_g0[1] * self[e315])
                        - (right_dual_g0[2] * self[e125])
                        - (right_dual_g1[0] * self[e415])
                        - (right_dual_g1[1] * self[e425])
                        - (right_dual_g1[2] * self[e435]),
                ),
        )
    }
}
impl BulkContraction<AntiPlane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       20        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g0[3] * self[e415]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g0[3] * self[e425]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g0[3] * self[e435]),
                -(right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]),
            ]) - (right_dual_g0.yzxx() * self.group1().zxy().with_w(self[e415])),
            // e4235, e4315, e4125, e3215
            right_dual_g0 * Simd32x4::from(self[e12345]),
        )
    }
}
impl BulkContraction<AntiScalar> for Motor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bulk_contraction(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e12345] * self[e12345] * -1.0)
    }
}
impl BulkContraction<Circle> for Motor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       20        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            right_dual_g1 * Simd32x4::from(self[e12345]),
            // e15, e25, e35, scalar
            (Simd32x3::from(self[e12345]) * other.group2()).with_w(
                -(right_dual_g1[0] * self[e415])
                    - (right_dual_g1[1] * self[e425])
                    - (right_dual_g1[2] * self[e435])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
            ),
        )
    }
}
impl BulkContraction<CircleRotor> for Motor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       25        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            right_dual_g1 * Simd32x4::from(self[e12345]),
            // e15, e25, e35, scalar
            (Simd32x3::from(self[e12345]) * right_dual_g2.xyz()).with_w(
                (right_dual_g2[3] * self[e12345])
                    - (right_dual_g1[0] * self[e415])
                    - (right_dual_g1[1] * self[e425])
                    - (right_dual_g1[2] * self[e435])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
            ),
        )
    }
}
impl BulkContraction<Dipole> for Motor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        5        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       10       22        0
    //  no simd       13       38        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            right_dual_g0 * Simd32x3::from(self[e12345]),
            // e415, e425, e435, e321
            right_dual_g1 * Simd32x4::from(self[e12345]),
            // e235, e315, e125, e4
            (right_dual_g2 * Simd32x3::from(self[e12345])).with_w(-(right_dual_g0[0] * self[e415]) - (right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g1[3] * self[e415]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g1[3] * self[e425]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g1[3] * self[e435]),
                -(right_dual_g2[1] * self[e425])
                    - (right_dual_g2[2] * self[e435])
                    - (right_dual_g1[0] * self[e235])
                    - (right_dual_g1[1] * self[e315])
                    - (right_dual_g1[2] * self[e125]),
            ]) - (right_dual_g0.yzx() * self.group1().zxy()).with_w(right_dual_g2[0] * self[e415]),
        )
    }
}
impl BulkContraction<DipoleInversion> for Motor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       16        0
    //    simd3        0        4        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       12       25        0
    //  no simd       18       48        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            right_dual_g0 * Simd32x3::from(self[e12345]),
            // e415, e425, e435, e321
            right_dual_g1 * Simd32x4::from(self[e12345]),
            // e235, e315, e125, e4
            (Simd32x3::from(self[e12345]) * right_dual_g2.xyz())
                .with_w((right_dual_g2[3] * self[e12345]) - (right_dual_g0[0] * self[e415]) - (right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g1[3] * self[e415]) + (right_dual_g3[0] * self[e12345]),
                (right_dual_g1[3] * self[e425]) + (right_dual_g3[1] * self[e12345]),
                (right_dual_g1[3] * self[e435]) + (right_dual_g3[2] * self[e12345]),
                -(right_dual_g1[1] * self[e315])
                    - (right_dual_g1[2] * self[e125])
                    - (right_dual_g2[0] * self[e415])
                    - (right_dual_g2[1] * self[e425])
                    - (right_dual_g2[2] * self[e435]),
            ]) + (right_dual_g0.zxy() * self.group1().yzx()).with_w(right_dual_g3[3] * self[e12345])
                - (self.group1().zxyx() * right_dual_g0.yzx().with_w(right_dual_g1[0])),
        )
    }
}
impl BulkContraction<DualNum> for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x2::from(-1.0);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(right_dual_g0[1] * self[e12345]),
            // e15, e25, e35, e3215
            Simd32x4::from(right_dual_g0[0]) * self.group0(),
        )
    }
}
impl BulkContraction<FlatPoint> for Motor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       14        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            right_dual_g0 * Simd32x4::from(self[e12345]),
            // e1, e2, e3, e5
            (self.group0().xyz() * right_dual_g0.www()).with_w(-(right_dual_g0[0] * self[e415]) - (right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435])),
        )
    }
}
impl BulkContraction<Flector> for Motor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        1        5        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        6       23        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            right_dual_g0 * Simd32x4::from(self[e12345]),
            // e1, e2, e3, e5
            (Simd32x4::from([self[e12345], self[e12345], self[e12345], 1.0])
                * right_dual_g1
                    .xyz()
                    .with_w(-(right_dual_g0[0] * self[e415]) - (right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435])))
                + (self.group0() * right_dual_g0.www().with_w(right_dual_g1[3])),
        )
    }
}
impl BulkContraction<Line> for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e12345]) * other.group0()).with_w(-(other[e415] * self[e415]) - (other[e425] * self[e425]) - (other[e435] * self[e435])),
            // e15, e25, e35, e3215
            (other.group1() * self.group0().www()).with_w(0.0),
        )
    }
}
impl BulkContraction<Motor> for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        1        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        6       22        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e12345]) * right_dual_g0.xyz())
                .with_w((right_dual_g0[3] * self[e12345]) - (right_dual_g0[0] * self[e415]) - (right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435])),
            // e15, e25, e35, e3215
            ((Simd32x3::from(right_dual_g1[3]) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * right_dual_g1.xyz())).with_w(right_dual_g1[3] * self[e12345]),
        )
    }
}
impl BulkContraction<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       39        0
    //    simd2        0        1        0
    //    simd3        4       13        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       30       59        0
    //  no simd       50      104        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1 = other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g6 = other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_dual_g9 = other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g10 * self[e5]) + (right_dual_g0[0] * self[e12345])
                    - (self[e415] * other[e415])
                    - (self[e425] * other[e425])
                    - (self[e435] * other[e435])
                    - (self[e235] * other[e423])
                    - (self[e315] * other[e431])
                    - (self[e125] * other[e412]),
                right_dual_g0[1] * self[e12345],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g1[0] * self[e12345]) + (right_dual_g6[3] * self[e415]),
                (right_dual_g1[1] * self[e12345]) + (right_dual_g6[3] * self[e425]),
                (right_dual_g1[2] * self[e12345]) + (right_dual_g6[3] * self[e435]),
                -(right_dual_g7[1] * self[e425]) - (right_dual_g7[2] * self[e435]),
            ]) + (right_dual_g7.zxy() * self.group1().yzx()).with_w(right_dual_g1[3] * self[e12345])
                - (right_dual_g7.yzx() * self.group1().zxy()).with_w(right_dual_g7[0] * self[e415]),
            // e5
            (right_dual_g0[1] * self[e5]) + (self[e12345] * other[e3215])
                - (right_dual_g8[0] * self[e415])
                - (right_dual_g8[1] * self[e425])
                - (right_dual_g8[2] * self[e435])
                - (right_dual_g6[0] * self[e235])
                - (right_dual_g6[1] * self[e315])
                - (right_dual_g6[2] * self[e125]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g9[2] * self[e315]) + (self[e12345] * other[e235]),
                (right_dual_g9[0] * self[e125]) + (self[e12345] * other[e315]),
                (right_dual_g9[1] * self[e235]) + (self[e12345] * other[e125]),
                -(right_dual_g9[1] * self[e425]) - (right_dual_g9[2] * self[e435]),
            ]) + (self.group0() * right_dual_g9.www().with_w(other[e321] * -1.0))
                - (right_dual_g9.yzxx() * self.group1().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(right_dual_g10) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * other.group7()),
            // e23, e31, e12
            (Simd32x3::from(right_dual_g10) * self.group1().xyz()) + (Simd32x3::from(self[e12345]) * other.group6().xyz()),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_dual_g0[1]) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * right_dual_g6.xyz())).with_w(right_dual_g6[3] * self[e12345]),
            // e423, e431, e412
            right_dual_g7 * Simd32x3::from(self[e12345]),
            // e235, e315, e125
            (right_dual_g8 * Simd32x3::from(self[e12345])) + (Simd32x3::from(right_dual_g0[1]) * self.group1().xyz()),
            // e4235, e4315, e4125, e3215
            right_dual_g9 * Simd32x4::from(self[e12345]),
            // e1234
            right_dual_g10 * self[e12345],
        )
    }
}
impl BulkContraction<Plane> for Motor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn bulk_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(self[e12345]) * Simd32x4::from([other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0, other[e3215]]),
        )
    }
}
impl BulkContraction<RoundPoint> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        5        0
    //    simd3        2        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        8       29        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other[e4] * -1.0;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(right_dual_g1) * self.group0().xyz().with_w(self[e5]),
            // e23, e31, e12, e45
            (Simd32x3::from(right_dual_g1) * self.group1().xyz()).with_w(-(right_dual_g0[0] * self[e415]) - (right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435])),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual_g0[3]) * self.group0().xyz()) + (right_dual_g0.zxy() * self.group1().yzx()) - (right_dual_g0.yzx() * self.group1().zxy()))
                .with_w(right_dual_g1 * self[e12345]),
            // e4235, e4315, e4125, e3215
            right_dual_g0 * Simd32x4::from(self[e12345]),
        )
    }
}
impl BulkContraction<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(other[scalar]) * self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl BulkContraction<Sphere> for Motor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            self[e12345] * other[e3215],
        )
    }
}
impl BulkContraction<VersorEven> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       11        0
    //    simd3        3        6        0
    //    simd4        3        7        0
    // Totals...
    // yes simd       13       24        0
    //  no simd       28       57        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() * right_dual_g2.www().with_w(right_dual_g0[3]))
                + Simd32x3::from(0.0).with_w(
                    -(right_dual_g0[0] * self[e235])
                        - (right_dual_g0[1] * self[e315])
                        - (right_dual_g0[2] * self[e125])
                        - (right_dual_g1[0] * self[e415])
                        - (right_dual_g1[1] * self[e425])
                        - (right_dual_g1[2] * self[e435]),
                )
                + (right_dual_g0.xyz() * self.group0().www()).with_w(right_dual_g2[3] * self[e5]),
            // e23, e31, e12, e45
            (right_dual_g1 * Simd32x4::from(self[e12345]))
                + (self.group1().xyz() * right_dual_g2.www()).with_w(-(right_dual_g3[0] * self[e415]) - (right_dual_g3[1] * self[e425]) - (right_dual_g3[2] * self[e435])),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual_g3[3]) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * right_dual_g2.xyz()) + (right_dual_g3.zxy() * self.group1().yzx())
                - (right_dual_g3.yzx() * self.group1().zxy()))
            .with_w(right_dual_g2[3] * self[e12345]),
            // e4235, e4315, e4125, e3215
            right_dual_g3 * Simd32x4::from(self[e12345]),
        )
    }
}
impl BulkContraction<VersorOdd> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        1        2        0
    //    simd4        4        9        0
    // Totals...
    // yes simd       14       26        0
    //  no simd       28       57        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            right_dual_g0 * Simd32x4::from(self[e12345]),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_dual_g0[3]) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * right_dual_g1.xyz())).with_w(right_dual_g1[3] * self[e12345]),
            // e235, e315, e125, e5
            (right_dual_g2 * Simd32x4::from(self[e12345]))
                + (Simd32x4::from(right_dual_g0[3]) * self.group1())
                + Simd32x3::from(0.0).with_w(
                    -(right_dual_g1[0] * self[e235])
                        - (right_dual_g1[1] * self[e315])
                        - (right_dual_g1[2] * self[e125])
                        - (right_dual_g2[0] * self[e415])
                        - (right_dual_g2[1] * self[e425])
                        - (right_dual_g2[2] * self[e435]),
                ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g3[0] * self[e12345]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g3[1] * self[e12345]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g3[2] * self[e12345]),
                -(right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]),
            ]) + (self.group0() * right_dual_g1.www().with_w(right_dual_g3[3]))
                - (right_dual_g0.yzxx() * self.group1().zxy().with_w(self[e415])),
        )
    }
}
impl std::ops::Div<BulkContractionInfix> for MultiVector {
    type Output = BulkContractionInfixPartial<MultiVector>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd3        8       17        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       46       71        0
    //  no simd       80      123        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g2[3] * self[scalar])
                    - (right_dual_g0[0] * self[e15])
                    - (right_dual_g0[1] * self[e25])
                    - (right_dual_g0[2] * self[e35])
                    - (right_dual_g1[0] * self[e23])
                    - (right_dual_g1[1] * self[e31])
                    - (right_dual_g1[2] * self[e12])
                    - (right_dual_g1[3] * self[e45])
                    - (right_dual_g2[0] * self[e41])
                    - (right_dual_g2[1] * self[e42])
                    - (right_dual_g2[2] * self[e43]),
                right_dual_g2[3] * self[e12345],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g1[0] * self[e321]) + (right_dual_g1[3] * self[e415]) + (right_dual_g2[1] * self[e412]) + (right_dual_g2[3] * self[e1]),
                (right_dual_g1[1] * self[e321]) + (right_dual_g1[3] * self[e425]) + (right_dual_g2[2] * self[e423]) + (right_dual_g2[3] * self[e2]),
                (right_dual_g1[2] * self[e321]) + (right_dual_g1[3] * self[e435]) + (right_dual_g2[0] * self[e431]) + (right_dual_g2[3] * self[e3]),
                -(right_dual_g0[2] * self[e435]) - (right_dual_g1[0] * self[e423]) - (right_dual_g1[1] * self[e431]) - (right_dual_g1[2] * self[e412]),
            ]) + (right_dual_g0.zxy() * self.group8().yzx()).with_w(right_dual_g2[3] * self[e4])
                - (right_dual_g0.yzx() * self.group8().zxy()).with_w(right_dual_g0[0] * self[e415])
                - (self.group7().yzx() * right_dual_g2.zxy()).with_w(right_dual_g0[1] * self[e425]),
            // e5
            (right_dual_g2[3] * self[e5])
                - (right_dual_g1[0] * self[e235])
                - (right_dual_g1[1] * self[e315])
                - (right_dual_g1[2] * self[e125])
                - (right_dual_g2[0] * self[e415])
                - (right_dual_g2[1] * self[e425])
                - (right_dual_g2[2] * self[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g2[1] * self[e4125]) + (right_dual_g2[3] * self[e15]),
                (right_dual_g2[2] * self[e4235]) + (right_dual_g2[3] * self[e25]),
                (right_dual_g2[0] * self[e4315]) + (right_dual_g2[3] * self[e35]),
                -(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125]),
            ]) + (right_dual_g1.xyz() * self.group9().www()).with_w(right_dual_g2[3] * self[e45])
                - (self.group9().yzxx() * right_dual_g2.zxy().with_w(right_dual_g1[0])),
            // e41, e42, e43
            (Simd32x3::from(right_dual_g2[3]) * self.group4()) + (Simd32x3::from(self[e1234]) * right_dual_g1.xyz()) + (right_dual_g0.zxy() * self.group9().yzx())
                - (right_dual_g0.yzx() * self.group9().zxy()),
            // e23, e31, e12
            (right_dual_g0 * Simd32x3::from(self[e3215])) + (Simd32x3::from(right_dual_g2[3]) * self.group5()) + (Simd32x3::from(self[e1234]) * right_dual_g2.xyz())
                - (Simd32x3::from(right_dual_g1[3]) * self.group9().xyz()),
            // e415, e425, e435, e321
            (right_dual_g1 * Simd32x4::from(self[e12345])) + (Simd32x4::from(right_dual_g2[3]) * self.group6()),
            // e423, e431, e412
            (right_dual_g0 * Simd32x3::from(self[e12345])) + (Simd32x3::from(right_dual_g2[3]) * self.group7()),
            // e235, e315, e125
            (Simd32x3::from(right_dual_g2[3]) * self.group8()) + (Simd32x3::from(self[e12345]) * right_dual_g2.xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g2[3]) * self.group9(),
            // e1234
            right_dual_g2[3] * self[e1234],
        )
    }
}
impl BulkContraction<AntiDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       37        0
    //    simd3        8       17        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       43       65        0
    //  no simd       89      132        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g2[3] * self[e5]) + (right_dual_g3[0] * self[e1]) + (right_dual_g3[1] * self[e2]) + (right_dual_g3[2] * self[e3]) + (right_dual_g3[3] * self[e4])
                    - (right_dual_g1[0] * self[e415])
                    - (right_dual_g1[1] * self[e425])
                    - (right_dual_g1[2] * self[e435])
                    - (right_dual_g1[3] * self[e321])
                    - (right_dual_g2[0] * self[e423])
                    - (right_dual_g2[1] * self[e431])
                    - (right_dual_g2[2] * self[e412])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * right_dual_g2.xyz().with_w(right_dual_g1[3]))
                + (self.group9().yzxz() * right_dual_g1.zxy().with_w(other[e412]))
                + (self.group4() * right_dual_g3.www()).with_w(other[e423] * self[e4235])
                + (self.group5().yzx() * right_dual_g3.zxy()).with_w(other[e431] * self[e4315])
                - (Simd32x4::from(right_dual_g2[3]) * self.group3())
                - (right_dual_g3.yzxy() * self.group5().zxy().with_w(self[e42]))
                - (other.group0() * self.group9().www()).with_w(right_dual_g3[0] * self[e41])
                - (right_dual_g1.yzx() * self.group9().zxy()).with_w(right_dual_g3[2] * self[e43]),
            // e5
            (right_dual_g3[0] * self[e15]) + (right_dual_g3[1] * self[e25]) + (right_dual_g3[2] * self[e35]) + (right_dual_g3[3] * self[e45])
                - (right_dual_g1[3] * self[e3215])
                - (right_dual_g2[0] * self[e4235])
                - (right_dual_g2[1] * self[e4315])
                - (right_dual_g2[2] * self[e4125]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g3[2] * self[e315]) + (right_dual_g3[3] * self[e415]),
                (right_dual_g3[0] * self[e125]) + (right_dual_g3[3] * self[e425]),
                (right_dual_g3[1] * self[e235]) + (right_dual_g3[3] * self[e435]),
                -(right_dual_g3[1] * self[e425]) - (right_dual_g3[2] * self[e435]),
            ]) + (Simd32x4::from(self[e12345]) * right_dual_g2.xyz().with_w(right_dual_g1[3]))
                - (self.group8().zxy() * right_dual_g3.yzx()).with_w(right_dual_g3[0] * self[e415]),
            // e41, e42, e43
            (Simd32x3::from(right_dual_g2[3]) * self.group6().xyz()) + (Simd32x3::from(self[e12345]) * other.group0()) + (self.group7().zxy() * right_dual_g3.yzx())
                - (self.group7().yzx() * right_dual_g3.zxy()),
            // e23, e31, e12
            (Simd32x3::from(right_dual_g2[3]) * self.group8()) + (Simd32x3::from(right_dual_g3[3]) * self.group7()) + (Simd32x3::from(self[e12345]) * right_dual_g1.xyz())
                - (Simd32x3::from(self[e321]) * right_dual_g3.xyz()),
            // e415, e425, e435, e321
            (right_dual_g3.yzxw() * self.group9().zxy().with_w(self[e1234])) - (self.group9().yzxw() * right_dual_g3.zxy().with_w(right_dual_g2[3])),
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual_g3.xyz()) - (Simd32x3::from(right_dual_g2[3]) * self.group9().xyz()),
            // e235, e315, e125
            (Simd32x3::from(right_dual_g3[3]) * self.group9().xyz()) - (Simd32x3::from(self[e3215]) * right_dual_g3.xyz()),
            // e4235, e4315, e4125, e3215
            right_dual_g3 * Simd32x4::from(self[e12345]),
            // e1234
            right_dual_g2[3] * self[e12345],
        )
    }
}
impl BulkContraction<AntiDualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       14        0
    //  no simd        2       34        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
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
impl BulkContraction<AntiFlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       20        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(right_dual_g0[0] * self[e423]) - (right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412]) - (right_dual_g0[3] * self[e321]),
                0.0,
            ]),
            // e1, e2, e3, e4
            right_dual_g0 * Simd32x4::from(self[e1234]),
            // e5
            -(right_dual_g0[0] * self[e4235]) - (right_dual_g0[1] * self[e4315]) - (right_dual_g0[2] * self[e4125]) - (right_dual_g0[3] * self[e3215]),
            // e15, e25, e35, e45
            right_dual_g0 * Simd32x4::from(self[e12345]),
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
impl BulkContraction<AntiFlector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       35        0
    //    simd3        4       11        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       30       51        0
    //  no simd       50       88        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g1[0] * self[e1]) + (right_dual_g1[1] * self[e2]) + (right_dual_g1[2] * self[e3]) + (right_dual_g1[3] * self[e4])
                    - (right_dual_g0[0] * self[e423])
                    - (right_dual_g0[1] * self[e431])
                    - (right_dual_g0[2] * self[e412])
                    - (right_dual_g0[3] * self[e321]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[0] * self[e1234]) + (right_dual_g1[2] * self[e31]),
                (right_dual_g0[1] * self[e1234]) + (right_dual_g1[0] * self[e12]),
                (right_dual_g0[2] * self[e1234]) + (right_dual_g1[1] * self[e23]),
                -(right_dual_g1[1] * self[e42]) - (right_dual_g1[2] * self[e43]),
            ]) + (self.group4() * right_dual_g1.www()).with_w(right_dual_g0[3] * self[e1234])
                - (right_dual_g1.yzxx() * self.group5().zxy().with_w(self[e41])),
            // e5
            (right_dual_g1[0] * self[e15]) + (right_dual_g1[1] * self[e25]) + (right_dual_g1[2] * self[e35]) + (right_dual_g1[3] * self[e45])
                - (right_dual_g0[0] * self[e4235])
                - (right_dual_g0[1] * self[e4315])
                - (right_dual_g0[2] * self[e4125])
                - (right_dual_g0[3] * self[e3215]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g1[2] * self[e315]) + (right_dual_g1[3] * self[e415]),
                (right_dual_g1[0] * self[e125]) + (right_dual_g1[3] * self[e425]),
                (right_dual_g1[1] * self[e235]) + (right_dual_g1[3] * self[e435]),
                -(right_dual_g1[1] * self[e425]) - (right_dual_g1[2] * self[e435]),
            ]) + (right_dual_g0 * Simd32x4::from(self[e12345]))
                - (self.group8().zxy() * right_dual_g1.yzx()).with_w(right_dual_g1[0] * self[e415]),
            // e41, e42, e43
            (self.group7().zxy() * right_dual_g1.yzx()) - (self.group7().yzx() * right_dual_g1.zxy()),
            // e23, e31, e12
            (Simd32x3::from(right_dual_g1[3]) * self.group7()) - (Simd32x3::from(self[e321]) * right_dual_g1.xyz()),
            // e415, e425, e435, e321
            ((right_dual_g1.yzx() * self.group9().zxy()) - (right_dual_g1.zxy() * self.group9().yzx())).with_w(right_dual_g1[3] * self[e1234]),
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * right_dual_g1.xyz(),
            // e235, e315, e125
            (Simd32x3::from(right_dual_g1[3]) * self.group9().xyz()) - (Simd32x3::from(self[e3215]) * right_dual_g1.xyz()),
            // e4235, e4315, e4125, e3215
            right_dual_g1 * Simd32x4::from(self[e12345]),
            // e1234
            0.0,
        )
    }
}
impl BulkContraction<AntiLine> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       29        0
    //    simd3        0        7        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       20       37        0
    //  no simd       26       54        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(right_dual_g0[0] * self[e23])
                    - (right_dual_g0[1] * self[e31])
                    - (right_dual_g0[2] * self[e12])
                    - (right_dual_g1[0] * self[e41])
                    - (right_dual_g1[1] * self[e42])
                    - (right_dual_g1[2] * self[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[0] * self[e321]) + (right_dual_g1[1] * self[e412]),
                (right_dual_g0[1] * self[e321]) + (right_dual_g1[2] * self[e423]),
                (right_dual_g0[2] * self[e321]) + (right_dual_g1[0] * self[e431]),
                -(right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412]),
            ]) - (right_dual_g1.zxy() * self.group7().yzx()).with_w(right_dual_g0[0] * self[e423]),
            // e5
            -(right_dual_g0[0] * self[e235])
                - (right_dual_g0[1] * self[e315])
                - (right_dual_g0[2] * self[e125])
                - (right_dual_g1[0] * self[e415])
                - (right_dual_g1[1] * self[e425])
                - (right_dual_g1[2] * self[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g0[0] * self[e3215]) + (right_dual_g1[1] * self[e4125]),
                (right_dual_g0[1] * self[e3215]) + (right_dual_g1[2] * self[e4235]),
                (right_dual_g0[2] * self[e3215]) + (right_dual_g1[0] * self[e4315]),
                -(right_dual_g0[1] * self[e4315]) - (right_dual_g0[2] * self[e4125]),
            ]) - (self.group9().yzxx() * right_dual_g1.zxy().with_w(right_dual_g0[0])),
            // e41, e42, e43
            right_dual_g0 * Simd32x3::from(self[e1234]),
            // e23, e31, e12
            right_dual_g1 * Simd32x3::from(self[e1234]),
            // e415, e425, e435, e321
            (right_dual_g0 * self.group0().yy().with_z(self[e12345])).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            right_dual_g1 * Simd32x3::from(self[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl BulkContraction<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       37        0
    //    simd3        4       11        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       30       53        0
    //  no simd       50       90        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g0[3] * self[scalar]) + (right_dual_g1[3] * self[e1234])
                    - (right_dual_g0[0] * self[e23])
                    - (right_dual_g0[1] * self[e31])
                    - (right_dual_g0[2] * self[e12])
                    - (right_dual_g1[0] * self[e41])
                    - (right_dual_g1[1] * self[e42])
                    - (right_dual_g1[2] * self[e43]),
                right_dual_g0[3] * self[e12345],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[0] * self[e321]) + (right_dual_g0[3] * self[e1]),
                (right_dual_g0[1] * self[e321]) + (right_dual_g0[3] * self[e2]),
                (right_dual_g0[2] * self[e321]) + (right_dual_g0[3] * self[e3]),
                -(right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412]),
            ]) + (self.group7().zxy() * right_dual_g1.yzx()).with_w(right_dual_g0[3] * self[e4])
                - (self.group7().yzx() * right_dual_g1.zxy()).with_w(right_dual_g0[0] * self[e423]),
            // e5
            (right_dual_g0[3] * self[e5]) + (right_dual_g1[3] * self[e12345])
                - (right_dual_g0[0] * self[e235])
                - (right_dual_g0[1] * self[e315])
                - (right_dual_g0[2] * self[e125])
                - (right_dual_g1[0] * self[e415])
                - (right_dual_g1[1] * self[e425])
                - (right_dual_g1[2] * self[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g0[3] * self[e15]) + (right_dual_g1[1] * self[e4125]),
                (right_dual_g0[3] * self[e25]) + (right_dual_g1[2] * self[e4235]),
                (right_dual_g0[3] * self[e35]) + (right_dual_g1[0] * self[e4315]),
                -(right_dual_g0[1] * self[e4315]) - (right_dual_g0[2] * self[e4125]),
            ]) + (right_dual_g0 * self.group9().www().with_w(self[e45]))
                - (self.group9().yzxx() * right_dual_g1.zxy().with_w(right_dual_g0[0])),
            // e41, e42, e43
            (Simd32x3::from(right_dual_g0[3]) * self.group4()) + (Simd32x3::from(self[e1234]) * right_dual_g0.xyz()),
            // e23, e31, e12
            (Simd32x3::from(right_dual_g0[3]) * self.group5()) + (Simd32x3::from(self[e1234]) * right_dual_g1.xyz()),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_dual_g0[3]) * self.group6().xyz()) + (Simd32x3::from(self[e12345]) * right_dual_g0.xyz())).with_w(right_dual_g0[3] * self[e321]),
            // e423, e431, e412
            Simd32x3::from(right_dual_g0[3]) * self.group7(),
            // e235, e315, e125
            (Simd32x3::from(right_dual_g0[3]) * self.group8()) + (Simd32x3::from(self[e12345]) * right_dual_g1.xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g0[3]) * self.group9(),
            // e1234
            right_dual_g0[3] * self[e1234],
        )
    }
}
impl BulkContraction<AntiPlane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       25        0
    //    simd3        4        9        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       20       38        0
    //  no simd       34       68        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g0[0] * self[e1]) + (right_dual_g0[1] * self[e2]) + (right_dual_g0[2] * self[e3]) + (right_dual_g0[3] * self[e4]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[2] * self[e31]) + (right_dual_g0[3] * self[e41]),
                (right_dual_g0[0] * self[e12]) + (right_dual_g0[3] * self[e42]),
                (right_dual_g0[1] * self[e23]) + (right_dual_g0[3] * self[e43]),
                -(right_dual_g0[1] * self[e42]) - (right_dual_g0[2] * self[e43]),
            ]) - (right_dual_g0.yzxx() * self.group5().zxy().with_w(self[e41])),
            // e5
            (right_dual_g0[0] * self[e15]) + (right_dual_g0[1] * self[e25]) + (right_dual_g0[2] * self[e35]) + (right_dual_g0[3] * self[e45]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g0[3] * self[e415]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g0[3] * self[e425]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g0[3] * self[e435]),
                -(right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]),
            ]) - (right_dual_g0.yzxx() * self.group8().zxy().with_w(self[e415])),
            // e41, e42, e43
            (self.group7().zxy() * right_dual_g0.yzx()) - (self.group7().yzx() * right_dual_g0.zxy()),
            // e23, e31, e12
            (Simd32x3::from(right_dual_g0[3]) * self.group7()) - (Simd32x3::from(self[e321]) * right_dual_g0.xyz()),
            // e415, e425, e435, e321
            ((right_dual_g0.yzx() * self.group9().zxy()) - (right_dual_g0.zxy() * self.group9().yzx())).with_w(right_dual_g0[3] * self[e1234]),
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * right_dual_g0.xyz(),
            // e235, e315, e125
            (Simd32x3::from(right_dual_g0[3]) * self.group9().xyz()) - (Simd32x3::from(self[e3215]) * right_dual_g0.xyz()),
            // e4235, e4315, e4125, e3215
            right_dual_g0 * Simd32x4::from(self[e12345]),
            // e1234
            0.0,
        )
    }
}
impl BulkContraction<AntiScalar> for MultiVector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bulk_contraction(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e12345] * self[e12345] * -1.0)
    }
}
impl BulkContraction<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       23        0
    //    simd3        0        3        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       18       29        0
    //  no simd       24       44        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(right_dual_g1[0] * self[e415])
                    - (right_dual_g1[1] * self[e425])
                    - (right_dual_g1[2] * self[e435])
                    - (right_dual_g1[3] * self[e321])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125])
                    - (other[e235] * self[e423])
                    - (other[e315] * self[e431])
                    - (other[e125] * self[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual_g1[1] * self[e4125]) - (other[e423] * self[e3215]),
                -(right_dual_g1[2] * self[e4235]) - (other[e431] * self[e3215]),
                -(right_dual_g1[0] * self[e4315]) - (other[e412] * self[e3215]),
                (right_dual_g1[3] * self[e1234]) + (other[e412] * self[e4125]),
            ]) + (self.group9().yzxy() * right_dual_g1.zxy().with_w(other[e431]))
                + (Simd32x3::from(self[e1234]) * other.group2()).with_w(other[e423] * self[e4235]),
            // e5
            -(right_dual_g1[3] * self[e3215]) - (other[e235] * self[e4235]) - (other[e315] * self[e4315]) - (other[e125] * self[e4125]),
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * other.group2().with_w(right_dual_g1[3]),
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[e12345]) * right_dual_g1.xyz(),
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
impl BulkContraction<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        3        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       19       31        0
    //  no simd       25       49        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g2[3] * self[e12345])
                    - (right_dual_g1[0] * self[e415])
                    - (right_dual_g1[1] * self[e425])
                    - (right_dual_g1[2] * self[e435])
                    - (right_dual_g1[3] * self[e321])
                    - (right_dual_g2[0] * self[e423])
                    - (right_dual_g2[1] * self[e431])
                    - (right_dual_g2[2] * self[e412])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual_g1[1] * self[e4125]) - (other[e423] * self[e3215]),
                -(right_dual_g1[2] * self[e4235]) - (other[e431] * self[e3215]),
                -(right_dual_g1[0] * self[e4315]) - (other[e412] * self[e3215]),
                (right_dual_g1[3] * self[e1234]) + (other[e412] * self[e4125]),
            ]) + (self.group9().yzxx() * right_dual_g1.zxy().with_w(other[e423]))
                + (Simd32x3::from(self[e1234]) * right_dual_g2.xyz()).with_w(other[e431] * self[e4315]),
            // e5
            -(right_dual_g1[3] * self[e3215]) - (right_dual_g2[0] * self[e4235]) - (right_dual_g2[1] * self[e4315]) - (right_dual_g2[2] * self[e4125]),
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * right_dual_g2.xyz().with_w(right_dual_g1[3]),
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[e12345]) * right_dual_g1.xyz(),
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
impl BulkContraction<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       42        0
    //    simd3        4       12        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       37       57        0
    //  no simd       54       90        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(right_dual_g0[0] * self[e15])
                    - (right_dual_g0[1] * self[e25])
                    - (right_dual_g0[2] * self[e35])
                    - (right_dual_g2[0] * self[e41])
                    - (right_dual_g2[1] * self[e42])
                    - (right_dual_g2[2] * self[e43])
                    - (right_dual_g1[0] * self[e23])
                    - (right_dual_g1[1] * self[e31])
                    - (right_dual_g1[2] * self[e12])
                    - (right_dual_g1[3] * self[e45]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g2[1] * self[e412]) + (right_dual_g1[0] * self[e321]) + (right_dual_g1[3] * self[e415]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g2[2] * self[e423]) + (right_dual_g1[1] * self[e321]) + (right_dual_g1[3] * self[e425]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g2[0] * self[e431]) + (right_dual_g1[2] * self[e321]) + (right_dual_g1[3] * self[e435]),
                -(right_dual_g0[2] * self[e435]) - (right_dual_g1[0] * self[e423]) - (right_dual_g1[1] * self[e431]) - (right_dual_g1[2] * self[e412]),
            ]) - (right_dual_g0.yzx() * self.group8().zxy()).with_w(right_dual_g0[0] * self[e415])
                - (right_dual_g2.zxy() * self.group7().yzx()).with_w(right_dual_g0[1] * self[e425]),
            // e5
            -(right_dual_g2[0] * self[e415])
                - (right_dual_g2[1] * self[e425])
                - (right_dual_g2[2] * self[e435])
                - (right_dual_g1[0] * self[e235])
                - (right_dual_g1[1] * self[e315])
                - (right_dual_g1[2] * self[e125]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g2[1] * self[e4125]) + (right_dual_g1[0] * self[e3215]),
                (right_dual_g2[2] * self[e4235]) + (right_dual_g1[1] * self[e3215]),
                (right_dual_g2[0] * self[e4315]) + (right_dual_g1[2] * self[e3215]),
                -(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125]),
            ]) - (self.group9().yzxx() * right_dual_g2.zxy().with_w(right_dual_g1[0])),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_dual_g1.xyz()) + (right_dual_g0.zxy() * self.group9().yzx()) - (right_dual_g0.yzx() * self.group9().zxy()),
            // e23, e31, e12
            (right_dual_g0 * Simd32x3::from(self[e3215])) + (right_dual_g2 * Simd32x3::from(self[e1234])) - (Simd32x3::from(right_dual_g1[3]) * self.group9().xyz()),
            // e415, e425, e435, e321
            right_dual_g1 * Simd32x4::from(self[e12345]),
            // e423, e431, e412
            right_dual_g0 * Simd32x3::from(self[e12345]),
            // e235, e315, e125
            right_dual_g2 * Simd32x3::from(self[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl BulkContraction<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       48        0
    //    simd3        4       11        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       44       65        0
    //  no simd       64      105        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g2[3] * self[e3215])
                    + (right_dual_g3[0] * self[e4235])
                    + (right_dual_g3[1] * self[e4315])
                    + (right_dual_g3[2] * self[e4125])
                    + (right_dual_g3[3] * self[e1234])
                    - (right_dual_g0[0] * self[e15])
                    - (right_dual_g0[1] * self[e25])
                    - (right_dual_g0[2] * self[e35])
                    - (right_dual_g1[0] * self[e23])
                    - (right_dual_g1[1] * self[e31])
                    - (right_dual_g1[2] * self[e12])
                    - (right_dual_g1[3] * self[e45])
                    - (right_dual_g2[0] * self[e41])
                    - (right_dual_g2[1] * self[e42])
                    - (right_dual_g2[2] * self[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g1[0] * self[e321]) + (right_dual_g1[3] * self[e415]) + (right_dual_g2[1] * self[e412]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g1[1] * self[e321]) + (right_dual_g1[3] * self[e425]) + (right_dual_g2[2] * self[e423]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g1[2] * self[e321]) + (right_dual_g1[3] * self[e435]) + (right_dual_g2[0] * self[e431]),
                -(right_dual_g0[2] * self[e435]) - (right_dual_g1[0] * self[e423]) - (right_dual_g1[1] * self[e431]) - (right_dual_g1[2] * self[e412]),
            ]) + (Simd32x4::from(self[e12345]) * right_dual_g3.xyz().with_w(right_dual_g2[3]))
                - (right_dual_g0.yzx() * self.group8().zxy()).with_w(right_dual_g0[0] * self[e415])
                - (self.group7().yzx() * right_dual_g2.zxy()).with_w(right_dual_g0[1] * self[e425]),
            // e5
            (right_dual_g3[3] * self[e12345])
                - (right_dual_g1[0] * self[e235])
                - (right_dual_g1[1] * self[e315])
                - (right_dual_g1[2] * self[e125])
                - (right_dual_g2[0] * self[e415])
                - (right_dual_g2[1] * self[e425])
                - (right_dual_g2[2] * self[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g1[0] * self[e3215]) + (right_dual_g2[1] * self[e4125]),
                (right_dual_g1[1] * self[e3215]) + (right_dual_g2[2] * self[e4235]),
                (right_dual_g1[2] * self[e3215]) + (right_dual_g2[0] * self[e4315]),
                -(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125]),
            ]) - (self.group9().yzxx() * right_dual_g2.zxy().with_w(right_dual_g1[0])),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_dual_g1.xyz()) + (right_dual_g0.zxy() * self.group9().yzx()) - (right_dual_g0.yzx() * self.group9().zxy()),
            // e23, e31, e12
            (right_dual_g0 * Simd32x3::from(self[e3215])) + (Simd32x3::from(self[e1234]) * right_dual_g2.xyz()) - (Simd32x3::from(right_dual_g1[3]) * self.group9().xyz()),
            // e415, e425, e435, e321
            right_dual_g1 * Simd32x4::from(self[e12345]),
            // e423, e431, e412
            right_dual_g0 * Simd32x3::from(self[e12345]),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * right_dual_g2.xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl BulkContraction<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        1       10        0
    //  no simd        1       19        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x2::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(right_dual_g0[0] * self[e4]) + (right_dual_g0[1] * self[e12345]), 0.0]),
            // e1, e2, e3, e4
            (self.group4() * right_dual_g0.xx().with_z(right_dual_g0[0])).with_w(0.0),
            // e5
            right_dual_g0[0] * self[e45],
            // e15, e25, e35, e45
            (self.group6().xyz() * right_dual_g0.xx().with_z(right_dual_g0[0])).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(right_dual_g0[0]) * self.group7(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(right_dual_g0[0] * self[e1234]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(right_dual_g0[0]) * self.group9().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(right_dual_g0[0] * self[e12345]),
            // e1234
            0.0,
        )
    }
}
impl BulkContraction<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        8        0
    //    simd3        2        8        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       19       36        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(right_dual_g0[0] * self[e41]) - (right_dual_g0[1] * self[e42]) - (right_dual_g0[2] * self[e43]) - (right_dual_g0[3] * self[e45]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(right_dual_g0[3]) * self.group6().xyz()).with_w(0.0) + (self.group7().zxy() * right_dual_g0.yzx()).with_w(0.0)
                - (self.group7().yzx() * right_dual_g0.zxy()).with_w(0.0),
            // e5
            -(right_dual_g0[0] * self[e415]) - (right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]),
            // e15, e25, e35, e45
            ((right_dual_g0.yzx() * self.group9().zxy()) - (right_dual_g0.zxy() * self.group9().yzx())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from(self[e1234]) * right_dual_g0.xyz()) - (Simd32x3::from(right_dual_g0[3]) * self.group9().xyz()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(right_dual_g0[3] * self[e12345]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * right_dual_g0.xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl BulkContraction<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       13        0
    //    simd3        2        9        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       15       24        0
    //  no simd       28       48        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g1[0] * self[e4235]) + (right_dual_g1[1] * self[e4315]) + (right_dual_g1[2] * self[e4125]) + (right_dual_g1[3] * self[e1234])
                    - (right_dual_g0[0] * self[e41])
                    - (right_dual_g0[1] * self[e42])
                    - (right_dual_g0[2] * self[e43])
                    - (right_dual_g0[3] * self[e45]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(right_dual_g0[3]) * self.group6().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e12345]) * right_dual_g1.xyz()).with_w(0.0)
                + (self.group7().zxy() * right_dual_g0.yzx()).with_w(0.0)
                - (self.group7().yzx() * right_dual_g0.zxy()).with_w(0.0),
            // e5
            (right_dual_g1[3] * self[e12345]) - (right_dual_g0[0] * self[e415]) - (right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]),
            // e15, e25, e35, e45
            ((right_dual_g0.yzx() * self.group9().zxy()) - (right_dual_g0.zxy() * self.group9().yzx())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from(self[e1234]) * right_dual_g0.xyz()) - (Simd32x3::from(right_dual_g0[3]) * self.group9().xyz()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(right_dual_g0[3] * self[e12345]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * right_dual_g0.xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl BulkContraction<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        9        0
    //    simd3        0        5        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       24        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
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
            (other.group1() * self.group0().yy().with_z(self[e12345])).with_w(0.0),
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
impl BulkContraction<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       14        0
    //    simd3        2        9        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       15       25        0
    //  no simd       28       49        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g0[3] * self[e12345]) + (right_dual_g1[3] * self[e4])
                    - (right_dual_g0[0] * self[e415])
                    - (right_dual_g0[1] * self[e425])
                    - (right_dual_g0[2] * self[e435])
                    - (right_dual_g1[0] * self[e423])
                    - (right_dual_g1[1] * self[e431])
                    - (right_dual_g1[2] * self[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(right_dual_g1[3]) * self.group4()).with_w(0.0)
                + (Simd32x3::from(self[e1234]) * right_dual_g1.xyz()).with_w(0.0)
                + (right_dual_g0.zxy() * self.group9().yzx()).with_w(0.0)
                - (right_dual_g0.yzx() * self.group9().zxy()).with_w(0.0),
            // e5
            (right_dual_g1[3] * self[e45]) - (right_dual_g1[0] * self[e4235]) - (right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125]),
            // e15, e25, e35, e45
            ((Simd32x3::from(right_dual_g1[3]) * self.group6().xyz()) + (Simd32x3::from(self[e12345]) * right_dual_g1.xyz())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from(right_dual_g1[3]) * self.group7()) + (Simd32x3::from(self[e12345]) * right_dual_g0.xyz()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(right_dual_g1[3] * self[e1234]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(right_dual_g1[3]) * self.group9().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(right_dual_g1[3] * self[e12345]),
            // e1234
            0.0,
        )
    }
}
impl BulkContraction<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       71       95        0
    //    simd2        0        1        0
    //    simd3       20       36        0
    //    simd4       20       15        0
    // Totals...
    // yes simd      111      147        0
    //  no simd      211      265        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1 = other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3 = other.group8().with_w(other[e321] * -1.0);
        let right_dual_g6 = other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_dual_g9 = other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g10 * self[e5])
                    + (right_dual_g0[0] * self[e12345])
                    + (right_dual_g0[1] * self[scalar])
                    + (right_dual_g1[0] * self[e4235])
                    + (right_dual_g1[1] * self[e4315])
                    + (right_dual_g1[2] * self[e4125])
                    + (right_dual_g1[3] * self[e3215])
                    + (right_dual_g9[0] * self[e1])
                    + (right_dual_g9[1] * self[e2])
                    + (right_dual_g9[2] * self[e3])
                    + (right_dual_g9[3] * self[e4])
                    + (other[e3215] * self[e1234])
                    - (right_dual_g7[0] * self[e15])
                    - (right_dual_g7[1] * self[e25])
                    - (right_dual_g7[2] * self[e35])
                    - (right_dual_g8[0] * self[e41])
                    - (right_dual_g8[1] * self[e42])
                    - (right_dual_g8[2] * self[e43])
                    - (right_dual_g3[0] * self[e423])
                    - (right_dual_g3[1] * self[e431])
                    - (right_dual_g3[2] * self[e412])
                    - (right_dual_g3[3] * self[e321])
                    - (right_dual_g6[0] * self[e23])
                    - (right_dual_g6[1] * self[e31])
                    - (right_dual_g6[2] * self[e12])
                    - (right_dual_g6[3] * self[e45])
                    - (other[e415] * self[e415])
                    - (other[e425] * self[e425])
                    - (other[e435] * self[e435])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
                right_dual_g0[1] * self[e12345],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g3[0] * self[e1234]) + (right_dual_g6[0] * self[e321]) + (right_dual_g6[3] * self[e415]) + (right_dual_g9[2] * self[e31]),
                (right_dual_g3[1] * self[e1234]) + (right_dual_g6[1] * self[e321]) + (right_dual_g6[3] * self[e425]) + (right_dual_g9[0] * self[e12]),
                (right_dual_g3[2] * self[e1234]) + (right_dual_g6[2] * self[e321]) + (right_dual_g6[3] * self[e435]) + (right_dual_g9[1] * self[e23]),
                -(right_dual_g10 * self[e45]) - (right_dual_g6[0] * self[e423]) - (right_dual_g6[1] * self[e431]) - (right_dual_g6[2] * self[e412]),
            ]) + (right_dual_g1 * Simd32x4::from(self[e12345]))
                + (Simd32x4::from(right_dual_g0[1]) * self.group1())
                + (self.group9().yzxx() * other.group6().zxy().with_w(other[e423]))
                + (self.group4() * right_dual_g9.www()).with_w(right_dual_g3[3] * self[e1234])
                + (right_dual_g7.zxy() * self.group8().yzx()).with_w(other[e431] * self[e4315])
                + (right_dual_g8.yzx() * self.group7().zxy()).with_w(other[e412] * self[e4125])
                - (right_dual_g9.yzxy() * self.group5().zxy().with_w(self[e42]))
                - (Simd32x3::from(right_dual_g10) * self.group3().xyz()).with_w(right_dual_g9[2] * self[e43])
                - (other.group7() * self.group9().www()).with_w(right_dual_g7[0] * self[e415])
                - (right_dual_g7.yzx() * self.group8().zxy()).with_w(right_dual_g7[2] * self[e435])
                - (right_dual_g8.zxy() * self.group7().yzx()).with_w(right_dual_g9[0] * self[e41])
                - (other.group6().yzx() * self.group9().zxy()).with_w(right_dual_g7[1] * self[e425]),
            // e5
            (right_dual_g0[1] * self[e5])
                + (right_dual_g9[0] * self[e15])
                + (right_dual_g9[1] * self[e25])
                + (right_dual_g9[2] * self[e35])
                + (right_dual_g9[3] * self[e45])
                + (other[e3215] * self[e12345])
                - (right_dual_g8[0] * self[e415])
                - (right_dual_g8[1] * self[e425])
                - (right_dual_g8[2] * self[e435])
                - (right_dual_g3[0] * self[e4235])
                - (right_dual_g3[1] * self[e4315])
                - (right_dual_g3[2] * self[e4125])
                - (right_dual_g3[3] * self[e3215])
                - (right_dual_g6[0] * self[e235])
                - (right_dual_g6[1] * self[e315])
                - (right_dual_g6[2] * self[e125]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g8[1] * self[e4125]) + (right_dual_g6[0] * self[e3215]) + (right_dual_g9[2] * self[e315]) + (right_dual_g9[3] * self[e415]),
                (right_dual_g8[2] * self[e4235]) + (right_dual_g6[1] * self[e3215]) + (right_dual_g9[0] * self[e125]) + (right_dual_g9[3] * self[e425]),
                (right_dual_g8[0] * self[e4315]) + (right_dual_g6[2] * self[e3215]) + (right_dual_g9[1] * self[e235]) + (right_dual_g9[3] * self[e435]),
                -(right_dual_g6[2] * self[e4125]) - (right_dual_g9[0] * self[e415]) - (right_dual_g9[1] * self[e425]) - (right_dual_g9[2] * self[e435]),
            ]) + (right_dual_g3 * Simd32x4::from(self[e12345]))
                + (Simd32x4::from(right_dual_g0[1]) * self.group3())
                - (self.group9().yzxx() * right_dual_g8.zxy().with_w(right_dual_g6[0]))
                - (self.group8().zxy() * right_dual_g9.yzx()).with_w(right_dual_g6[1] * self[e4315]),
            // e41, e42, e43
            (Simd32x3::from(right_dual_g10) * self.group6().xyz())
                + (Simd32x3::from(right_dual_g0[1]) * self.group4())
                + (Simd32x3::from(self[e12345]) * other.group7())
                + (Simd32x3::from(self[e1234]) * right_dual_g6.xyz())
                + (right_dual_g7.zxy() * self.group9().yzx())
                + (self.group7().zxy() * right_dual_g9.yzx())
                - (right_dual_g7.yzx() * self.group9().zxy())
                - (self.group7().yzx() * right_dual_g9.zxy()),
            // e23, e31, e12
            (right_dual_g7 * Simd32x3::from(self[e3215]))
                + (right_dual_g8 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(right_dual_g10) * self.group8())
                + (Simd32x3::from(right_dual_g0[1]) * self.group5())
                + (Simd32x3::from(right_dual_g9[3]) * self.group7())
                + (Simd32x3::from(self[e12345]) * other.group6().xyz())
                - (Simd32x3::from(right_dual_g6[3]) * self.group9().xyz())
                - (Simd32x3::from(self[e321]) * right_dual_g9.xyz()),
            // e415, e425, e435, e321
            (right_dual_g6 * Simd32x4::from(self[e12345])) + (Simd32x4::from(right_dual_g0[1]) * self.group6()) + (right_dual_g9.yzxw() * self.group9().zxy().with_w(self[e1234]))
                - (right_dual_g9.zxy() * self.group9().yzx()).with_w(right_dual_g10 * self[e3215]),
            // e423, e431, e412
            (right_dual_g7 * Simd32x3::from(self[e12345])) + (Simd32x3::from(right_dual_g0[1]) * self.group7()) + (Simd32x3::from(self[e1234]) * right_dual_g9.xyz())
                - (Simd32x3::from(right_dual_g10) * self.group9().xyz()),
            // e235, e315, e125
            (right_dual_g8 * Simd32x3::from(self[e12345])) + (Simd32x3::from(right_dual_g0[1]) * self.group8()) + (Simd32x3::from(right_dual_g9[3]) * self.group9().xyz())
                - (Simd32x3::from(self[e3215]) * right_dual_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (right_dual_g9 * Simd32x4::from(self[e12345])) + (Simd32x4::from(right_dual_g0[1]) * self.group9()),
            // e1234
            (right_dual_g10 * self[e12345]) + (right_dual_g0[1] * self[e1234]),
        )
    }
}
impl BulkContraction<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       12        0
    fn bulk_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g0[0] * self[e4235]) + (right_dual_g0[1] * self[e4315]) + (right_dual_g0[2] * self[e4125]) + (right_dual_g0[3] * self[e1234]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (right_dual_g0.xyz() * self.group0().yy().with_z(self[e12345])).with_w(0.0),
            // e5
            right_dual_g0[3] * self[e12345],
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
impl BulkContraction<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       29        0
    //    simd3        6       12        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       25       46        0
    //  no simd       49       85        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g1 * self[e5]) + (right_dual_g0[0] * self[e1]) + (right_dual_g0[1] * self[e2]) + (right_dual_g0[2] * self[e3]) + (right_dual_g0[3] * self[e4]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[2] * self[e31]) + (right_dual_g0[3] * self[e41]),
                (right_dual_g0[0] * self[e12]) + (right_dual_g0[3] * self[e42]),
                (right_dual_g0[1] * self[e23]) + (right_dual_g0[3] * self[e43]),
                -(right_dual_g1 * self[e45]) - (right_dual_g0[2] * self[e43]),
            ]) - (right_dual_g0.yzxx() * self.group5().zxy().with_w(self[e41]))
                - (Simd32x3::from(right_dual_g1) * self.group3().xyz()).with_w(right_dual_g0[1] * self[e42]),
            // e5
            (right_dual_g0[0] * self[e15]) + (right_dual_g0[1] * self[e25]) + (right_dual_g0[2] * self[e35]) + (right_dual_g0[3] * self[e45]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g0[3] * self[e415]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g0[3] * self[e425]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g0[3] * self[e435]),
                -(right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]),
            ]) - (right_dual_g0.yzxx() * self.group8().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(right_dual_g1) * self.group6().xyz()) + (self.group7().zxy() * right_dual_g0.yzx()) - (self.group7().yzx() * right_dual_g0.zxy()),
            // e23, e31, e12
            (Simd32x3::from(right_dual_g1) * self.group8()) + (Simd32x3::from(right_dual_g0[3]) * self.group7()) - (Simd32x3::from(self[e321]) * right_dual_g0.xyz()),
            // e415, e425, e435, e321
            (right_dual_g0.yzx() * self.group9().zxy()).with_w(right_dual_g0[3] * self[e1234]) - (self.group9().yzxw() * right_dual_g0.zxy().with_w(right_dual_g1)),
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual_g0.xyz()) - (Simd32x3::from(right_dual_g1) * self.group9().xyz()),
            // e235, e315, e125
            (Simd32x3::from(right_dual_g0[3]) * self.group9().xyz()) - (Simd32x3::from(self[e3215]) * right_dual_g0.xyz()),
            // e4235, e4315, e4125, e3215
            right_dual_g0 * Simd32x4::from(self[e12345]),
            // e1234
            right_dual_g1 * self[e12345],
        )
    }
}
impl BulkContraction<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       32        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
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
impl BulkContraction<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        4       14        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g0[0] * self[e4235])
                    + (right_dual_g0[1] * self[e4315])
                    + (right_dual_g0[2] * self[e4125])
                    + (right_dual_g0[3] * self[e3215])
                    + (self[e1234] * other[e3215]),
                0.0,
            ]),
            // e1, e2, e3, e4
            right_dual_g0 * Simd32x4::from(self[e12345]),
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
impl BulkContraction<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       39        0
    //    simd3        8       18        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       44       68        0
    //  no simd       90      137        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g0[3] * self[e12345])
                    + (right_dual_g2[3] * self[e5])
                    + (right_dual_g3[0] * self[e1])
                    + (right_dual_g3[1] * self[e2])
                    + (right_dual_g3[2] * self[e3])
                    + (right_dual_g3[3] * self[e4])
                    - (right_dual_g0[0] * self[e235])
                    - (right_dual_g0[1] * self[e315])
                    - (right_dual_g0[2] * self[e125])
                    - (right_dual_g1[0] * self[e415])
                    - (right_dual_g1[1] * self[e425])
                    - (right_dual_g1[2] * self[e435])
                    - (right_dual_g1[3] * self[e321])
                    - (right_dual_g2[0] * self[e423])
                    - (right_dual_g2[1] * self[e431])
                    - (right_dual_g2[2] * self[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * right_dual_g2.xyz().with_w(right_dual_g1[3]))
                + (self.group9().yzxz() * right_dual_g1.zxy().with_w(right_dual_g0[2]))
                + (self.group4() * right_dual_g3.www()).with_w(right_dual_g0[0] * self[e4235])
                + (self.group5().yzx() * right_dual_g3.zxy()).with_w(right_dual_g0[1] * self[e4315])
                - (right_dual_g3.yzxx() * self.group5().zxy().with_w(self[e41]))
                - (right_dual_g1.yzx() * self.group9().zxy()).with_w(right_dual_g3[2] * self[e43])
                - (right_dual_g0.xyz() * self.group9().www()).with_w(right_dual_g2[3] * self[e45])
                - (self.group3().xyz() * right_dual_g2.www()).with_w(right_dual_g3[1] * self[e42]),
            // e5
            (right_dual_g3[0] * self[e15]) + (right_dual_g3[1] * self[e25]) + (right_dual_g3[2] * self[e35]) + (right_dual_g3[3] * self[e45])
                - (right_dual_g1[3] * self[e3215])
                - (right_dual_g2[0] * self[e4235])
                - (right_dual_g2[1] * self[e4315])
                - (right_dual_g2[2] * self[e4125]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g3[2] * self[e315]) + (right_dual_g3[3] * self[e415]),
                (right_dual_g3[0] * self[e125]) + (right_dual_g3[3] * self[e425]),
                (right_dual_g3[1] * self[e235]) + (right_dual_g3[3] * self[e435]),
                -(right_dual_g3[1] * self[e425]) - (right_dual_g3[2] * self[e435]),
            ]) + (Simd32x4::from(self[e12345]) * right_dual_g2.xyz().with_w(right_dual_g1[3]))
                - (right_dual_g3.yzxx() * self.group8().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(right_dual_g2[3]) * self.group6().xyz()) + (Simd32x3::from(self[e12345]) * right_dual_g0.xyz()) + (self.group7().zxy() * right_dual_g3.yzx())
                - (self.group7().yzx() * right_dual_g3.zxy()),
            // e23, e31, e12
            (Simd32x3::from(right_dual_g2[3]) * self.group8()) + (Simd32x3::from(right_dual_g3[3]) * self.group7()) + (Simd32x3::from(self[e12345]) * right_dual_g1.xyz())
                - (Simd32x3::from(self[e321]) * right_dual_g3.xyz()),
            // e415, e425, e435, e321
            (right_dual_g3.yzx() * self.group9().zxy()).with_w(right_dual_g3[3] * self[e1234]) - (self.group9().yzxw() * right_dual_g3.zxy().with_w(right_dual_g2[3])),
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual_g3.xyz()) - (Simd32x3::from(right_dual_g2[3]) * self.group9().xyz()),
            // e235, e315, e125
            (Simd32x3::from(right_dual_g3[3]) * self.group9().xyz()) - (Simd32x3::from(self[e3215]) * right_dual_g3.xyz()),
            // e4235, e4315, e4125, e3215
            right_dual_g3 * Simd32x4::from(self[e12345]),
            // e1234
            right_dual_g2[3] * self[e12345],
        )
    }
}
impl BulkContraction<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       53        0
    //    simd3        8       15        0
    //    simd4        7       10        0
    // Totals...
    // yes simd       53       78        0
    //  no simd       90      138        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g0[3] * self[scalar])
                    + (right_dual_g2[3] * self[e1234])
                    + (right_dual_g3[0] * self[e4235])
                    + (right_dual_g3[1] * self[e4315])
                    + (right_dual_g3[2] * self[e4125])
                    + (right_dual_g3[3] * self[e3215])
                    - (right_dual_g0[0] * self[e15])
                    - (right_dual_g0[1] * self[e25])
                    - (right_dual_g0[2] * self[e35])
                    - (right_dual_g1[0] * self[e23])
                    - (right_dual_g1[1] * self[e31])
                    - (right_dual_g1[2] * self[e12])
                    - (right_dual_g1[3] * self[e45])
                    - (right_dual_g2[0] * self[e41])
                    - (right_dual_g2[1] * self[e42])
                    - (right_dual_g2[2] * self[e43]),
                right_dual_g0[3] * self[e12345],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g0[3] * self[e1]) + (right_dual_g1[0] * self[e321]) + (right_dual_g1[3] * self[e415]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g0[3] * self[e2]) + (right_dual_g1[1] * self[e321]) + (right_dual_g1[3] * self[e425]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g0[3] * self[e3]) + (right_dual_g1[2] * self[e321]) + (right_dual_g1[3] * self[e435]),
                -(right_dual_g0[0] * self[e415]) - (right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]) - (right_dual_g1[2] * self[e412]),
            ]) + (right_dual_g3 * Simd32x4::from(self[e12345]))
                + (self.group7().zxy() * right_dual_g2.yzx()).with_w(right_dual_g0[3] * self[e4])
                - (self.group7().yzx() * right_dual_g2.zxy()).with_w(right_dual_g1[0] * self[e423])
                - (self.group8().zxy() * right_dual_g0.yzx()).with_w(right_dual_g1[1] * self[e431]),
            // e5
            (right_dual_g0[3] * self[e5]) + (right_dual_g2[3] * self[e12345])
                - (right_dual_g1[0] * self[e235])
                - (right_dual_g1[1] * self[e315])
                - (right_dual_g1[2] * self[e125])
                - (right_dual_g2[0] * self[e415])
                - (right_dual_g2[1] * self[e425])
                - (right_dual_g2[2] * self[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g1[0] * self[e3215]) + (right_dual_g2[1] * self[e4125]),
                (right_dual_g1[1] * self[e3215]) + (right_dual_g2[2] * self[e4235]),
                (right_dual_g1[2] * self[e3215]) + (right_dual_g2[0] * self[e4315]),
                -(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125]),
            ]) + (Simd32x4::from(right_dual_g0[3]) * self.group3())
                - (self.group9().yzxx() * right_dual_g2.zxy().with_w(right_dual_g1[0])),
            // e41, e42, e43
            (Simd32x3::from(right_dual_g0[3]) * self.group4()) + (Simd32x3::from(self[e1234]) * right_dual_g1.xyz()) + (right_dual_g0.zxy() * self.group9().yzx())
                - (right_dual_g0.yzx() * self.group9().zxy()),
            // e23, e31, e12
            (Simd32x3::from(right_dual_g0[3]) * self.group5()) + (Simd32x3::from(self[e3215]) * right_dual_g0.xyz()) + (Simd32x3::from(self[e1234]) * right_dual_g2.xyz())
                - (Simd32x3::from(right_dual_g1[3]) * self.group9().xyz()),
            // e415, e425, e435, e321
            (right_dual_g1 * Simd32x4::from(self[e12345])) + (Simd32x4::from(right_dual_g0[3]) * self.group6()),
            // e423, e431, e412
            (Simd32x3::from(right_dual_g0[3]) * self.group7()) + (Simd32x3::from(self[e12345]) * right_dual_g0.xyz()),
            // e235, e315, e125
            (Simd32x3::from(right_dual_g0[3]) * self.group8()) + (Simd32x3::from(self[e12345]) * right_dual_g2.xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g0[3]) * self.group9(),
            // e1234
            right_dual_g0[3] * self[e1234],
        )
    }
}
impl std::ops::Div<BulkContractionInfix> for Plane {
    type Output = BulkContractionInfixPartial<Plane>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for Plane {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd3        1        6        0
    //    simd4        3        5        0
    // Totals...
    // yes simd        5       13        0
    //  no simd       16       40        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        DipoleInversion::from_groups(
            // e41, e42, e43
            (right_dual_g0.zxy() * self.group0().yzx()) - (right_dual_g0.yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            (Simd32x4::from([self[e3215], self[e3215], self[e3215], 1.0]) * right_dual_g0.with_w(-(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125])))
                - (right_dual_g1.wwwx() * self.group0().xyzx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(self[e3215]) * right_dual_g1.xyz()).with_w(0.0) + (right_dual_g2.yzx() * self.group0().zxy()).with_w(0.0)
                - (right_dual_g2.zxy() * self.group0().yzx()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g2[3]) * self.group0(),
        )
    }
}
impl BulkContraction<AntiDipoleInversion> for Plane {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        5        0
    //    simd4        3        8        0
    // Totals...
    // yes simd        6       19        0
    //  no simd       17       53        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual_g2[3]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            ((right_dual_g3.yzx() * self.group0().zxy()) - (right_dual_g3.zxy() * self.group0().yzx())).with_w(right_dual_g2[3] * self[e3215] * -1.0),
            // e235, e315, e125, e4
            (Simd32x4::from([self[e3215], self[e3215], self[e3215], 1.0])
                * right_dual_g3.xyz().with_w((other[e431] * self[e4315]) + (other[e412] * self[e4125]))
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
                + (self.group0().xyzx() * right_dual_g3.www().with_w(other[e423])),
            // e1, e2, e3, e5
            (right_dual_g1.zxy() * self.group0().yzx()).with_w(-(right_dual_g2[1] * self[e4315]) - (right_dual_g2[2] * self[e4125]))
                - (Simd32x4::from(self[e3215]) * other.group0().with_w(right_dual_g1[3]))
                - (self.group0().zxyx() * right_dual_g1.yzx().with_w(right_dual_g2[0])),
        )
    }
}
impl BulkContraction<AntiDualNum> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl BulkContraction<AntiFlatPoint> for Plane {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            -(right_dual_g0[0] * self[e4235]) - (right_dual_g0[1] * self[e4315]) - (right_dual_g0[2] * self[e4125]) - (right_dual_g0[3] * self[e3215]),
            0.0,
        ]))
    }
}
impl BulkContraction<AntiFlector> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        1        3        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        9       24        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((right_dual_g1.yzx() * self.group0().zxy()) - (right_dual_g1.zxy() * self.group0().yzx())).with_w(0.0),
            // e235, e315, e125, e5
            (self.group0().xyz() * right_dual_g1.www()).with_w(-(right_dual_g0[1] * self[e4315]) - (right_dual_g0[2] * self[e4125]) - (right_dual_g0[3] * self[e3215]))
                - (self.group0().wwwx() * right_dual_g1.xyz().with_w(right_dual_g0[0])),
        )
    }
}
impl BulkContraction<AntiLine> for Plane {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       18        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g0[0] * self[e3215]) + (right_dual_g1[1] * self[e4125]),
                (right_dual_g0[1] * self[e3215]) + (right_dual_g1[2] * self[e4235]),
                (right_dual_g0[2] * self[e3215]) + (right_dual_g1[0] * self[e4315]),
                -(right_dual_g0[1] * self[e4315]) - (right_dual_g0[2] * self[e4125]),
            ]) - (self.group0().yzxx() * right_dual_g1.zxy().with_w(right_dual_g0[0])),
        )
    }
}
impl BulkContraction<AntiMotor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        8       24        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g0[0] * self[e3215]) + (right_dual_g1[1] * self[e4125]),
                (right_dual_g0[1] * self[e3215]) + (right_dual_g1[2] * self[e4235]),
                (right_dual_g0[2] * self[e3215]) + (right_dual_g1[0] * self[e4315]),
                -(right_dual_g0[1] * self[e4315]) - (right_dual_g0[2] * self[e4125]),
            ]) - (self.group0().yzxx() * right_dual_g1.zxy().with_w(right_dual_g0[0])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g0[3]) * self.group0(),
        )
    }
}
impl BulkContraction<AntiPlane> for Plane {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        6       16        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Line::from_groups(
            // e415, e425, e435
            (right_dual_g0.yzx() * self.group0().zxy()) - (right_dual_g0.zxy() * self.group0().yzx()),
            // e235, e315, e125
            (Simd32x3::from(right_dual_g0[3]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * right_dual_g0.xyz()),
        )
    }
}
impl BulkContraction<Circle> for Plane {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        8       14        0
    //  no simd       11       20        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual_g1[1] * self[e4125]) - (other[e423] * self[e3215]),
                -(right_dual_g1[2] * self[e4235]) - (other[e431] * self[e3215]),
                -(right_dual_g1[0] * self[e4315]) - (other[e412] * self[e3215]),
                (other[e431] * self[e4315]) + (other[e412] * self[e4125]),
            ]) + (self.group0().yzxx() * right_dual_g1.zxy().with_w(other[e423])),
            // e5
            -(right_dual_g1[3] * self[e3215]) - (other[e235] * self[e4235]) - (other[e315] * self[e4315]) - (other[e125] * self[e4125]),
        )
    }
}
impl BulkContraction<CircleRotor> for Plane {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        8       15        0
    //  no simd       11       24        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual_g1[1] * self[e4125]) - (other[e423] * self[e3215]),
                -(right_dual_g1[2] * self[e4235]) - (other[e431] * self[e3215]),
                -(right_dual_g1[0] * self[e4315]) - (other[e412] * self[e3215]),
                (other[e431] * self[e4315]) + (other[e412] * self[e4125]),
            ]) + (self.group0().yzxx() * right_dual_g1.zxy().with_w(other[e423])),
            // e5
            -(right_dual_g1[3] * self[e3215]) - (right_dual_g2[0] * self[e4235]) - (right_dual_g2[1] * self[e4315]) - (right_dual_g2[2] * self[e4125]),
        )
    }
}
impl BulkContraction<Dipole> for Plane {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd3        3        7        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       12        0
    //  no simd       14       35        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        Dipole::from_groups(
            // e41, e42, e43
            (right_dual_g0.zxy() * self.group0().yzx()) - (right_dual_g0.yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            (Simd32x4::from([self[e3215], self[e3215], self[e3215], 1.0]) * right_dual_g0.with_w(-(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125])))
                - (right_dual_g1.wwwx() * self.group0().xyzx()),
            // e15, e25, e35
            (Simd32x3::from(self[e3215]) * right_dual_g1.xyz()) + (right_dual_g2.yzx() * self.group0().zxy()) - (right_dual_g2.zxy() * self.group0().yzx()),
        )
    }
}
impl BulkContraction<DipoleInversion> for Plane {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        1        5        0
    //    simd4        3        7        0
    // Totals...
    // yes simd        6       16        0
    //  no simd       17       47        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (right_dual_g0.zxy() * self.group0().yzx()) - (right_dual_g0.yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            (Simd32x4::from([self[e3215], self[e3215], self[e3215], 1.0]) * right_dual_g0.with_w(-(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125])))
                - (right_dual_g1.wwwx() * self.group0().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(self[e3215]) * right_dual_g1.xyz().with_w(right_dual_g2[3]))
                + (self.group0().zxyx() * right_dual_g2.yzx().with_w(right_dual_g3[0]))
                + (right_dual_g2.zxy() * self.group0().yzx() * Simd32x3::from(-1.0)).with_w((right_dual_g3[1] * self[e4315]) + (right_dual_g3[2] * self[e4125])),
        )
    }
}
impl BulkContraction<DualNum> for Plane {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        let right_dual_g0 = other.group0() * Simd32x2::from(-1.0);
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ (self.group0().xyz() * right_dual_g0.xx().with_z(right_dual_g0[0])).with_w(0.0))
    }
}
impl BulkContraction<FlatPoint> for Plane {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        5        0
    //  no simd        3       16        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(right_dual_g0[3]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e15, e25, e35
            (right_dual_g0.yzx() * self.group0().zxy()) - (right_dual_g0.zxy() * self.group0().yzx()),
        )
    }
}
impl BulkContraction<Flector> for Plane {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        1        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        5       23        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (self.group0().xyz() * right_dual_g0.www() * Simd32x3::from(-1.0))
                .with_w((right_dual_g1[0] * self[e4235]) + (right_dual_g1[1] * self[e4315]) + (right_dual_g1[2] * self[e4125])),
            // e15, e25, e35, e3215
            ((right_dual_g0.yzx() * self.group0().zxy()) - (right_dual_g0.zxy() * self.group0().yzx())).with_w(0.0),
        )
    }
}
impl BulkContraction<Line> for Plane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        5        9        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                other[e435] * self[e4315],
                other[e415] * self[e4125],
                other[e425] * self[e4235],
                -(other[e315] * self[e4315]) - (other[e125] * self[e4125]),
            ]) - (self.group0().zxyx() * other.group0().yzx().with_w(other[e235])),
        )
    }
}
impl BulkContraction<Motor> for Plane {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd3        0        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        5       20        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (self.group0().xyz() * right_dual_g1.www()).with_w(0.0),
            // e1, e2, e3, e5
            (right_dual_g0.zxy() * self.group0().yzx()).with_w(-(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125]))
                - (self.group0().zxyx() * right_dual_g0.yzx().with_w(right_dual_g1[0])),
        )
    }
}
impl BulkContraction<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       27        0
    //    simd3        4       12        0
    //    simd4        2        6        0
    // Totals...
    // yes simd       20       45        0
    //  no simd       34       87        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g6 = other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_dual_g9 = other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g1[0] * self[e4235]) + (right_dual_g1[1] * self[e4315]) + (right_dual_g1[2] * self[e4125]) + (right_dual_g1[3] * self[e3215]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                -(other[e425] * self[e4125]) - (other[e423] * self[e3215]),
                -(other[e435] * self[e4235]) - (other[e431] * self[e3215]),
                -(other[e415] * self[e4315]) - (other[e412] * self[e3215]),
                (other[e431] * self[e4315]) + (other[e412] * self[e4125]),
            ]) + (self.group0().yzxx() * other.group6().zxy().with_w(other[e423])),
            // e5
            (other[e321] * self[e3215]) - (other[e235] * self[e4235]) - (other[e315] * self[e4315]) - (other[e125] * self[e4125]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g8[1] * self[e4125]) + (right_dual_g6[0] * self[e3215]),
                (right_dual_g8[2] * self[e4235]) + (right_dual_g6[1] * self[e3215]),
                (right_dual_g8[0] * self[e4315]) + (right_dual_g6[2] * self[e3215]),
                -(right_dual_g6[1] * self[e4315]) - (right_dual_g6[2] * self[e4125]),
            ]) - (self.group0().yzxx() * right_dual_g8.zxy().with_w(right_dual_g6[0])),
            // e41, e42, e43
            (right_dual_g7.zxy() * self.group0().yzx()) - (right_dual_g7.yzx() * self.group0().zxy()),
            // e23, e31, e12
            (right_dual_g7 * Simd32x3::from(self[e3215])) - (Simd32x3::from(right_dual_g6[3]) * self.group0().xyz()),
            // e415, e425, e435, e321
            ((right_dual_g9.yzx() * self.group0().zxy()) - (right_dual_g9.zxy() * self.group0().yzx())).with_w(right_dual_g10 * self[e3215] * -1.0),
            // e423, e431, e412
            Simd32x3::from(right_dual_g10) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            (Simd32x3::from(right_dual_g9[3]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * right_dual_g9.xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other.group0().yx()[1]) * self.group0(),
            // e1234
            0.0,
        )
    }
}
impl BulkContraction<Plane> for Plane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn bulk_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Scalar::from_groups(/* scalar */ (right_dual_g0[0] * self[e4235]) + (right_dual_g0[1] * self[e4315]) + (right_dual_g0[2] * self[e4125]))
    }
}
impl BulkContraction<RoundPoint> for Plane {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        2        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2       10        0
    //  no simd        6       25        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other[e4] * -1.0;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual_g1) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            ((right_dual_g0.yzx() * self.group0().zxy()) - (right_dual_g0.zxy() * self.group0().yzx())).with_w(right_dual_g1 * self[e3215] * -1.0),
            // e235, e315, e125
            (Simd32x3::from(right_dual_g0[3]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * right_dual_g0.xyz()),
        )
    }
}
impl BulkContraction<Scalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl BulkContraction<Sphere> for Plane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Scalar::from_groups(
            // scalar
            (right_dual_g0[0] * self[e4235]) + (right_dual_g0[1] * self[e4315]) + (right_dual_g0[2] * self[e4125]) + (right_dual_g0[3] * self[e3215]),
        )
    }
}
impl BulkContraction<VersorEven> for Plane {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        5        0
    //    simd4        3        9        0
    // Totals...
    // yes simd        6       20        0
    //  no simd       17       57        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual_g2[3]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            ((right_dual_g3.yzx() * self.group0().zxy()) - (right_dual_g3.zxy() * self.group0().yzx())).with_w(right_dual_g2[3] * self[e3215] * -1.0),
            // e235, e315, e125, e4
            (Simd32x4::from([self[e3215], self[e3215], self[e3215], 1.0])
                * right_dual_g3.xyz().with_w((right_dual_g0[1] * self[e4315]) + (right_dual_g0[2] * self[e4125]))
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
                + (self.group0().xyzx() * right_dual_g3.www().with_w(right_dual_g0[0])),
            // e1, e2, e3, e5
            (right_dual_g1.zxy() * self.group0().yzx()).with_w(-(right_dual_g1[3] * self[e3215]) - (right_dual_g2[2] * self[e4125]))
                - (self.group0().zxyx() * right_dual_g1.yzx().with_w(right_dual_g2[0]))
                - (self.group0().wwwy() * right_dual_g0.xyz().with_w(right_dual_g2[1])),
        )
    }
}
impl BulkContraction<VersorOdd> for Plane {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        5        0
    //    simd4        4        8        0
    // Totals...
    // yes simd        7       18        0
    //  no simd       19       52        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0().yzxx() * right_dual_g0.zxy().with_w(right_dual_g3[0]))
                + (right_dual_g0.yzx() * self.group0().zxy() * Simd32x3::from(-1.0))
                    .with_w((right_dual_g3[1] * self[e4315]) + (right_dual_g3[2] * self[e4125]) + (right_dual_g3[3] * self[e3215])),
            // e23, e31, e12, e45
            (Simd32x4::from([self[e3215], self[e3215], self[e3215], 1.0]) * right_dual_g0.xyz().with_w(-(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125])))
                - (right_dual_g1.wwwx() * self.group0().xyzx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(self[e3215]) * right_dual_g1.xyz()).with_w(0.0) + (right_dual_g2.yzx() * self.group0().zxy()).with_w(0.0)
                - (right_dual_g2.zxy() * self.group0().yzx()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g0[3]) * self.group0(),
        )
    }
}
impl std::ops::Div<BulkContractionInfix> for RoundPoint {
    type Output = BulkContractionInfixPartial<RoundPoint>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(right_dual_g2[3]) * self.group0(), /* e5 */ right_dual_g2[3] * self[e5])
    }
}
impl BulkContraction<AntiDipoleInversion> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        6        0
    //  no simd        4        9        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g3 = other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(
            // scalar
            (right_dual_g3[0] * self[e1]) + (right_dual_g3[1] * self[e2]) + (right_dual_g3[2] * self[e3]) + (right_dual_g3[3] * self[e4]) - (other[e4] * self[e5]),
        )
    }
}
impl BulkContraction<AntiDualNum> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar]) * self.group0(), /* e5 */ other[scalar] * self[e5])
    }
}
impl BulkContraction<AntiFlector> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(
            // scalar
            (right_dual_g1[0] * self[e1]) + (right_dual_g1[1] * self[e2]) + (right_dual_g1[2] * self[e3]) + (right_dual_g1[3] * self[e4]),
        )
    }
}
impl BulkContraction<AntiMotor> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(right_dual_g0[3]) * self.group0(), /* e5 */ right_dual_g0[3] * self[e5])
    }
}
impl BulkContraction<AntiPlane> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(
            // scalar
            (right_dual_g0[0] * self[e1]) + (right_dual_g0[1] * self[e2]) + (right_dual_g0[2] * self[e3]) + (right_dual_g0[3] * self[e4]),
        )
    }
}
impl BulkContraction<DualNum> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e5] * self[e4] * -1.0)
    }
}
impl BulkContraction<Motor> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e5] * self[e4] * -1.0)
    }
}
impl BulkContraction<MultiVector> for RoundPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd2        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        4       16        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g9 = other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g9[0] * self[e1]) + (right_dual_g9[1] * self[e2]) + (right_dual_g9[2] * self[e3]) + (right_dual_g9[3] * self[e4]) - (other[e4] * self[e5]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual_g0[1]) * self.group0(),
            // e5
            right_dual_g0[1] * self[e5],
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
impl BulkContraction<RoundPoint> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        6        0
    //  no simd        4        9        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(
            // scalar
            (right_dual_g0[0] * self[e1]) + (right_dual_g0[1] * self[e2]) + (right_dual_g0[2] * self[e3]) + (right_dual_g0[3] * self[e4]) - (other[e4] * self[e5]),
        )
    }
}
impl BulkContraction<Scalar> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar]) * self.group0(), /* e5 */ self[e5] * other[scalar])
    }
}
impl BulkContraction<VersorEven> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        6        0
    //  no simd        4        9        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g3 = other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(
            // scalar
            (right_dual_g3[0] * self[e1]) + (right_dual_g3[1] * self[e2]) + (right_dual_g3[2] * self[e3]) + (right_dual_g3[3] * self[e4]) - (self[e5] * other[e4]),
        )
    }
}
impl BulkContraction<VersorOdd> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(right_dual_g0[3]) * self.group0(), /* e5 */ right_dual_g0[3] * self[e5])
    }
}
impl std::ops::Div<BulkContractionInfix> for Scalar {
    type Output = BulkContractionInfixPartial<Scalar>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[scalar] * self[scalar])
    }
}
impl BulkContraction<AntiDualNum> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[scalar] * self[scalar])
    }
}
impl BulkContraction<AntiMotor> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[scalar] * self[scalar])
    }
}
impl BulkContraction<MultiVector> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[scalar] * self[scalar])
    }
}
impl BulkContraction<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[scalar] * self[scalar])
    }
}
impl BulkContraction<VersorOdd> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * other[scalar])
    }
}
impl std::ops::Div<BulkContractionInfix> for Sphere {
    type Output = BulkContractionInfixPartial<Sphere>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for Sphere {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        4        7        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        9       20        0
    //  no simd       20       46        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_dual_g1.xyz()) + (right_dual_g0.zxy() * self.group0().yzx()) - (right_dual_g0.yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual_g0[0] * self[e3215]) + (right_dual_g2[0] * self[e1234]),
                (right_dual_g0[1] * self[e3215]) + (right_dual_g2[1] * self[e1234]),
                (right_dual_g0[2] * self[e3215]) + (right_dual_g2[2] * self[e1234]),
                -(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125]),
            ]) - (right_dual_g1.wwwx() * self.group0().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e3215]) * right_dual_g1.xyz()) + (right_dual_g2.yzx() * self.group0().zxy()) - (right_dual_g2.zxy() * self.group0().yzx()))
                .with_w(right_dual_g2[3] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g2[3]) * self.group0(),
        )
    }
}
impl BulkContraction<AntiDipoleInversion> for Sphere {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       11        0
    //    simd3        1        2        0
    //    simd4        4       10        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       25       57        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual_g3.xyz()) - (Simd32x3::from(right_dual_g2[3]) * self.group0().xyz()),
            // e415, e425, e435, e321
            (right_dual_g3.yzxw() * self.group0().zxy().with_w(self[e1234])) - (self.group0().yzxw() * right_dual_g3.zxy().with_w(right_dual_g2[3])),
            // e235, e315, e125, e4
            (Simd32x4::from([self[e3215], self[e3215], self[e3215], 1.0])
                * right_dual_g3.xyz().with_w((right_dual_g1[3] * self[e1234]) + (other[e431] * self[e4315]) + (other[e412] * self[e4125]))
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
                + (self.group0().xyzx() * right_dual_g3.www().with_w(other[e423])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g1[2] * self[e4315]) + (right_dual_g2[0] * self[e1234]),
                (right_dual_g1[0] * self[e4125]) + (right_dual_g2[1] * self[e1234]),
                (right_dual_g1[1] * self[e4235]) + (right_dual_g2[2] * self[e1234]),
                -(right_dual_g2[1] * self[e4315]) - (right_dual_g2[2] * self[e4125]),
            ]) - (Simd32x4::from(self[e3215]) * other.group0().with_w(right_dual_g1[3]))
                - (self.group0().zxyx() * right_dual_g1.yzx().with_w(right_dual_g2[0])),
        )
    }
}
impl BulkContraction<AntiDualNum> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
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
impl BulkContraction<AntiFlatPoint> for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            right_dual_g0 * Simd32x4::from(self[e1234]),
            // e5
            -(right_dual_g0[0] * self[e4235]) - (right_dual_g0[1] * self[e4315]) - (right_dual_g0[2] * self[e4125]) - (right_dual_g0[3] * self[e3215]),
        )
    }
}
impl BulkContraction<AntiFlector> for Sphere {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        2        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        5       14        0
    //  no simd        9       32        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * right_dual_g1.xyz(),
            // e415, e425, e435, e321
            ((right_dual_g1.yzx() * self.group0().zxy()) - (right_dual_g1.zxy() * self.group0().yzx())).with_w(right_dual_g1[3] * self[e1234]),
            // e235, e315, e125, e4
            ((Simd32x3::from(right_dual_g1[3]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * right_dual_g1.xyz())).with_w(right_dual_g0[3] * self[e1234]),
            // e1, e2, e3, e5
            (Simd32x3::from(self[e1234]) * right_dual_g0.xyz())
                .with_w(-(right_dual_g0[0] * self[e4235]) - (right_dual_g0[1] * self[e4315]) - (right_dual_g0[2] * self[e4125]) - (right_dual_g0[3] * self[e3215])),
        )
    }
}
impl BulkContraction<AntiLine> for Sphere {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        2        7        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        8       24        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        Dipole::from_groups(
            // e41, e42, e43
            right_dual_g0 * Simd32x3::from(self[e1234]),
            // e23, e31, e12, e45
            (right_dual_g1 * Simd32x3::from(self[e1234])).with_w(-(right_dual_g0[0] * self[e4235]) - (right_dual_g0[1] * self[e4315]) - (right_dual_g0[2] * self[e4125])),
            // e15, e25, e35
            (right_dual_g0 * Simd32x3::from(self[e3215])) + (right_dual_g1.yzx() * self.group0().zxy()) - (right_dual_g1.zxy() * self.group0().yzx()),
        )
    }
}
impl BulkContraction<AntiMotor> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        2        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        8       32        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(self[e1234]) * right_dual_g0.xyz().with_w(right_dual_g1[3]),
            // e23, e31, e12, e45
            (Simd32x3::from(self[e1234]) * right_dual_g1.xyz()).with_w(-(right_dual_g0[0] * self[e4235]) - (right_dual_g0[1] * self[e4315]) - (right_dual_g0[2] * self[e4125])),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e3215]) * right_dual_g0.xyz()) + (right_dual_g1.yzx() * self.group0().zxy()) - (right_dual_g1.zxy() * self.group0().yzx()))
                .with_w(right_dual_g0[3] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g0[3]) * self.group0(),
        )
    }
}
impl BulkContraction<AntiPlane> for Sphere {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        2        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        6       20        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * right_dual_g0.xyz(),
            // e415, e425, e435, e321
            ((right_dual_g0.yzx() * self.group0().zxy()) - (right_dual_g0.zxy() * self.group0().yzx())).with_w(right_dual_g0[3] * self[e1234]),
            // e235, e315, e125
            (Simd32x3::from(right_dual_g0[3]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * right_dual_g0.xyz()),
        )
    }
}
impl BulkContraction<Circle> for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       16        0
    //  no simd       15       24        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual_g1[1] * self[e4125]) - (other[e423] * self[e3215]),
                -(right_dual_g1[2] * self[e4235]) - (other[e431] * self[e3215]),
                -(right_dual_g1[0] * self[e4315]) - (other[e412] * self[e3215]),
                (right_dual_g1[3] * self[e1234]) + (other[e412] * self[e4125]),
            ]) + (self.group0().yzxy() * right_dual_g1.zxy().with_w(other[e431]))
                + (Simd32x3::from(self[e1234]) * other.group2()).with_w(other[e423] * self[e4235]),
            // e5
            -(right_dual_g1[3] * self[e3215]) - (other[e235] * self[e4235]) - (other[e315] * self[e4315]) - (other[e125] * self[e4125]),
        )
    }
}
impl BulkContraction<CircleRotor> for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       15       28        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual_g1[1] * self[e4125]) - (other[e423] * self[e3215]),
                -(right_dual_g1[2] * self[e4235]) - (other[e431] * self[e3215]),
                -(right_dual_g1[0] * self[e4315]) - (other[e412] * self[e3215]),
                (right_dual_g1[3] * self[e1234]) + (other[e412] * self[e4125]),
            ]) + (self.group0().yzxx() * right_dual_g1.zxy().with_w(other[e423]))
                + (Simd32x3::from(self[e1234]) * right_dual_g2.xyz()).with_w(other[e431] * self[e4315]),
            // e5
            -(right_dual_g1[3] * self[e3215]) - (right_dual_g2[0] * self[e4235]) - (right_dual_g2[1] * self[e4315]) - (right_dual_g2[2] * self[e4125]),
        )
    }
}
impl BulkContraction<Dipole> for Sphere {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       20       40        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        Dipole::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_dual_g1.xyz()) + (right_dual_g0.zxy() * self.group0().yzx()) - (right_dual_g0.yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual_g0[0] * self[e3215]) + (right_dual_g2[0] * self[e1234]),
                (right_dual_g0[1] * self[e3215]) + (right_dual_g2[1] * self[e1234]),
                (right_dual_g0[2] * self[e3215]) + (right_dual_g2[2] * self[e1234]),
                -(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125]),
            ]) - (right_dual_g1.wwwx() * self.group0().xyzx()),
            // e15, e25, e35
            (Simd32x3::from(self[e3215]) * right_dual_g1.xyz()) + (right_dual_g2.yzx() * self.group0().zxy()) - (right_dual_g2.zxy() * self.group0().yzx()),
        )
    }
}
impl BulkContraction<DipoleInversion> for Sphere {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       11        0
    //    simd3        2        6        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       24       53        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_dual_g1.xyz()) + (right_dual_g0.zxy() * self.group0().yzx()) - (right_dual_g0.yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual_g0[0] * self[e3215]) + (right_dual_g2[0] * self[e1234]),
                (right_dual_g0[1] * self[e3215]) + (right_dual_g2[1] * self[e1234]),
                (right_dual_g0[2] * self[e3215]) + (right_dual_g2[2] * self[e1234]),
                -(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125]),
            ]) - (right_dual_g1.wwwx() * self.group0().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(self[e3215]) * right_dual_g1.xyz().with_w(right_dual_g2[3]))
                + (self.group0().zxyx() * right_dual_g2.yzx().with_w(right_dual_g3[0]))
                + (right_dual_g2.zxy() * self.group0().yzx() * Simd32x3::from(-1.0))
                    .with_w((right_dual_g3[1] * self[e4315]) + (right_dual_g3[2] * self[e4125]) + (right_dual_g3[3] * self[e1234])),
        )
    }
}
impl BulkContraction<DualNum> for Sphere {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other[e5] * -1.0) * self.group0().xyz().with_w(self[e1234]))
    }
}
impl BulkContraction<FlatPoint> for Sphere {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        6       16        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiLine::from_groups(
            // e23, e31, e12
            (Simd32x3::from(self[e1234]) * right_dual_g0.xyz()) - (Simd32x3::from(right_dual_g0[3]) * self.group0().xyz()),
            // e15, e25, e35
            (right_dual_g0.yzx() * self.group0().zxy()) - (right_dual_g0.zxy() * self.group0().yzx()),
        )
    }
}
impl BulkContraction<Flector> for Sphere {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        2        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        9       24        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            ((Simd32x3::from(self[e1234]) * right_dual_g0.xyz()) - (self.group0().xyz() * right_dual_g0.www()))
                .with_w((right_dual_g1[0] * self[e4235]) + (right_dual_g1[1] * self[e4315]) + (right_dual_g1[2] * self[e4125]) + (right_dual_g1[3] * self[e1234])),
            // e15, e25, e35, e3215
            ((right_dual_g0.yzx() * self.group0().zxy()) - (right_dual_g0.zxy() * self.group0().yzx())).with_w(0.0),
        )
    }
}
impl BulkContraction<Line> for Sphere {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e435] * self[e4315]) + (other[e235] * self[e1234]),
                (other[e415] * self[e4125]) + (other[e315] * self[e1234]),
                (other[e425] * self[e4235]) + (other[e125] * self[e1234]),
                -(other[e315] * self[e4315]) - (other[e125] * self[e4125]),
            ]) - (self.group0().zxyx() * other.group0().yzx().with_w(other[e235])),
        )
    }
}
impl BulkContraction<Motor> for Sphere {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        8       24        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual_g1[3]) * self.group0().xyz().with_w(self[e1234]),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[2] * self[e4315]) + (right_dual_g1[0] * self[e1234]),
                (right_dual_g0[0] * self[e4125]) + (right_dual_g1[1] * self[e1234]),
                (right_dual_g0[1] * self[e4235]) + (right_dual_g1[2] * self[e1234]),
                -(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125]),
            ]) - (self.group0().zxyx() * right_dual_g0.yzx().with_w(right_dual_g1[0])),
        )
    }
}
impl BulkContraction<MultiVector> for Sphere {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       30        0
    //    simd2        0        1        0
    //    simd3        6       14        0
    //    simd4        4        7        0
    // Totals...
    // yes simd       25       52        0
    //  no simd       49      102        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1 = other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3_w = other[e321] * -1.0;
        let right_dual_g6 = other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_dual_g9 = other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g1[0] * self[e4235])
                    + (right_dual_g1[1] * self[e4315])
                    + (right_dual_g1[2] * self[e4125])
                    + (right_dual_g1[3] * self[e3215])
                    + (other[e3215] * self[e1234]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                -(other[e425] * self[e4125]) - (other[e423] * self[e3215]),
                -(other[e435] * self[e4235]) - (other[e431] * self[e3215]),
                -(other[e415] * self[e4315]) - (other[e412] * self[e3215]),
                (right_dual_g3_w * self[e1234]) + (other[e412] * self[e4125]),
            ]) + (self.group0().yzxx() * other.group6().zxy().with_w(other[e423]))
                + (Simd32x3::from(self[e1234]) * other.group8()).with_w(other[e431] * self[e4315]),
            // e5
            -(right_dual_g3_w * self[e3215]) - (other[e235] * self[e4235]) - (other[e315] * self[e4315]) - (other[e125] * self[e4125]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g8[1] * self[e4125]) + (right_dual_g6[0] * self[e3215]),
                (right_dual_g8[2] * self[e4235]) + (right_dual_g6[1] * self[e3215]),
                (right_dual_g8[0] * self[e4315]) + (right_dual_g6[2] * self[e3215]),
                -(right_dual_g6[1] * self[e4315]) - (right_dual_g6[2] * self[e4125]),
            ]) - (self.group0().yzxx() * right_dual_g8.zxy().with_w(right_dual_g6[0])),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_dual_g6.xyz()) + (right_dual_g7.zxy() * self.group0().yzx()) - (right_dual_g7.yzx() * self.group0().zxy()),
            // e23, e31, e12
            (right_dual_g7 * Simd32x3::from(self[e3215])) + (right_dual_g8 * Simd32x3::from(self[e1234])) - (Simd32x3::from(right_dual_g6[3]) * self.group0().xyz()),
            // e415, e425, e435, e321
            (right_dual_g9.yzxw() * self.group0().zxy().with_w(self[e1234])) - (right_dual_g9.zxy() * self.group0().yzx()).with_w(right_dual_g10 * self[e3215]),
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual_g9.xyz()) - (Simd32x3::from(right_dual_g10) * self.group0().xyz()),
            // e235, e315, e125
            (Simd32x3::from(right_dual_g9[3]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * right_dual_g9.xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g0[1]) * self.group0(),
            // e1234
            right_dual_g0[1] * self[e1234],
        )
    }
}
impl BulkContraction<Plane> for Sphere {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Scalar::from_groups(
            // scalar
            (right_dual_g0[0] * self[e4235]) + (right_dual_g0[1] * self[e4315]) + (right_dual_g0[2] * self[e4125]) + (right_dual_g0[3] * self[e1234]),
        )
    }
}
impl BulkContraction<RoundPoint> for Sphere {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        2        5        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        9        0
    //  no simd       10       25        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other[e4] * -1.0;
        Circle::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual_g0.xyz()) - (Simd32x3::from(right_dual_g1) * self.group0().xyz()),
            // e415, e425, e435, e321
            (right_dual_g0.yzxw() * self.group0().zxy().with_w(self[e1234])) - (right_dual_g0.zxy() * self.group0().yzx()).with_w(right_dual_g1 * self[e3215]),
            // e235, e315, e125
            (Simd32x3::from(right_dual_g0[3]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * right_dual_g0.xyz()),
        )
    }
}
impl BulkContraction<Scalar> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1234
            other[scalar] * self[e1234],
        )
    }
}
impl BulkContraction<Sphere> for Sphere {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        6        0
    //  no simd        4        9        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Scalar::from_groups(
            // scalar
            (right_dual_g0[0] * self[e4235])
                + (right_dual_g0[1] * self[e4315])
                + (right_dual_g0[2] * self[e4125])
                + (right_dual_g0[3] * self[e3215])
                + (other[e3215] * self[e1234]),
        )
    }
}
impl BulkContraction<VersorEven> for Sphere {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       12        0
    //    simd3        1        3        0
    //    simd4        4       10        0
    // Totals...
    // yes simd       11       25        0
    //  no simd       25       61        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual_g3.xyz()) - (Simd32x3::from(right_dual_g2[3]) * self.group0().xyz()),
            // e415, e425, e435, e321
            (right_dual_g3.yzx() * self.group0().zxy()).with_w(right_dual_g3[3] * self[e1234]) - (self.group0().yzxw() * right_dual_g3.zxy().with_w(right_dual_g2[3])),
            // e235, e315, e125, e4
            (Simd32x4::from([self[e3215], self[e3215], self[e3215], 1.0])
                * right_dual_g3
                    .xyz()
                    .with_w((right_dual_g0[1] * self[e4315]) + (right_dual_g0[2] * self[e4125]) + (right_dual_g1[3] * self[e1234]))
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
                + (self.group0().xyzx() * right_dual_g3.www().with_w(right_dual_g0[0])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g1[2] * self[e4315]) + (right_dual_g2[0] * self[e1234]),
                (right_dual_g1[0] * self[e4125]) + (right_dual_g2[1] * self[e1234]),
                (right_dual_g1[1] * self[e4235]) + (right_dual_g2[2] * self[e1234]),
                -(right_dual_g1[3] * self[e3215]) - (right_dual_g2[2] * self[e4125]),
            ]) - (self.group0().zxyx() * right_dual_g1.yzx().with_w(right_dual_g2[0]))
                - (self.group0().wwwy() * right_dual_g0.xyz().with_w(right_dual_g2[1])),
        )
    }
}
impl BulkContraction<VersorOdd> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       13        0
    //    simd3        2        6        0
    //    simd4        3        7        0
    // Totals...
    // yes simd       11       26        0
    //  no simd       24       59        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0().yzxx() * right_dual_g0.zxy().with_w(right_dual_g3[0]))
                + (right_dual_g0.yzx() * self.group0().zxy() * Simd32x3::from(-1.0))
                    .with_w((right_dual_g2[3] * self[e1234]) + (right_dual_g3[2] * self[e4125]) + (right_dual_g3[3] * self[e3215]))
                + (Simd32x3::from(self[e1234]) * right_dual_g1.xyz()).with_w(right_dual_g3[1] * self[e4315]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual_g0[0] * self[e3215]) + (right_dual_g2[0] * self[e1234]),
                (right_dual_g0[1] * self[e3215]) + (right_dual_g2[1] * self[e1234]),
                (right_dual_g0[2] * self[e3215]) + (right_dual_g2[2] * self[e1234]),
                -(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125]),
            ]) - (right_dual_g1.wwwx() * self.group0().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e3215]) * right_dual_g1.xyz()) + (right_dual_g2.yzx() * self.group0().zxy()) - (right_dual_g2.zxy() * self.group0().yzx()))
                .with_w(right_dual_g0[3] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g0[3]) * self.group0(),
        )
    }
}
impl std::ops::Div<BulkContractionInfix> for VersorEven {
    type Output = BulkContractionInfixPartial<VersorEven>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       26        0
    //    simd3        1        7        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       23       38        0
    //  no simd       40       67        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((right_dual_g0 * Simd32x3::from(self[e12345])) + (Simd32x3::from(right_dual_g2[3]) * self.group0().xyz())).with_w(right_dual_g2[3] * self[e12345]),
            // e415, e425, e435, e321
            (right_dual_g1 * Simd32x4::from(self[e12345])) + (Simd32x4::from(right_dual_g2[3]) * self.group1()),
            // e235, e315, e125, e5
            (right_dual_g2 * self.group0().www().with_w(self[e5]))
                + (self.group2().xyz() * right_dual_g2.www()).with_w(
                    -(right_dual_g1[0] * self[e235])
                        - (right_dual_g1[1] * self[e315])
                        - (right_dual_g1[2] * self[e125])
                        - (right_dual_g2[0] * self[e415])
                        - (right_dual_g2[1] * self[e425])
                        - (right_dual_g2[2] * self[e435]),
                ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g1[0] * self[e321]) + (right_dual_g1[3] * self[e415]) + (right_dual_g2[1] * self[e412]) + (right_dual_g2[3] * self[e1]),
                (right_dual_g1[1] * self[e321]) + (right_dual_g1[3] * self[e425]) + (right_dual_g2[2] * self[e423]) + (right_dual_g2[3] * self[e2]),
                (right_dual_g1[2] * self[e321]) + (right_dual_g1[3] * self[e435]) + (right_dual_g2[0] * self[e431]) + (right_dual_g2[3] * self[e3]),
                -(right_dual_g0[2] * self[e435]) - (right_dual_g1[0] * self[e423]) - (right_dual_g1[1] * self[e431]) - (right_dual_g1[2] * self[e412]),
            ]) + (right_dual_g0.zxy() * self.group2().yzx()).with_w(right_dual_g2[3] * self[e4])
                - (right_dual_g0.yzx() * self.group2().zxy()).with_w(right_dual_g0[0] * self[e415])
                - (right_dual_g2.zxy() * self.group0().yzx()).with_w(right_dual_g0[1] * self[e425]),
        )
    }
}
impl BulkContraction<AntiDipoleInversion> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        0
    //    simd3        3        7        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       23       37        0
    //  no simd       47       72        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (right_dual_g3.yzxy() * self.group0().zxy().with_w(self[e2]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual_g3[2] * self[e3]) + (right_dual_g3[3] * self[e4])
                        - (right_dual_g1[0] * self[e415])
                        - (right_dual_g1[1] * self[e425])
                        - (right_dual_g1[2] * self[e435])
                        - (right_dual_g1[3] * self[e321])
                        - (right_dual_g2[0] * self[e423])
                        - (right_dual_g2[1] * self[e431])
                        - (right_dual_g2[2] * self[e412])
                        - (other[e431] * self[e315])
                        - (other[e412] * self[e125]),
                )
                + (other.group0() * self.group0().www()).with_w(right_dual_g2[3] * self[e5])
                + (self.group1().xyz() * right_dual_g2.www()).with_w(right_dual_g3[0] * self[e1])
                - (right_dual_g3.zxy() * self.group0().yzx()).with_w(other[e423] * self[e235]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual_g2[3] * self[e235]) + (right_dual_g3[3] * self[e423]),
                (right_dual_g2[3] * self[e315]) + (right_dual_g3[3] * self[e431]),
                (right_dual_g2[3] * self[e125]) + (right_dual_g3[3] * self[e412]),
                -(right_dual_g3[1] * self[e425]) - (right_dual_g3[2] * self[e435]),
            ]) + (right_dual_g1 * Simd32x4::from(self[e12345]))
                - (right_dual_g3.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual_g3[3]) * self.group1().xyz()) + (Simd32x3::from(self[e12345]) * right_dual_g2.xyz()) + (right_dual_g3.zxy() * self.group2().yzx())
                - (right_dual_g3.yzx() * self.group2().zxy()))
            .with_w(right_dual_g2[3] * self[e12345]),
            // e4235, e4315, e4125, e3215
            right_dual_g3 * Simd32x4::from(self[e12345]),
        )
    }
}
impl BulkContraction<AntiDualNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1       18        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125, e5
            other.group0().yy().with_zw(other[scalar], (other[e3215] * self[e12345]) + (other[scalar] * self[e5])) * self.group2().xyz().with_w(1.0),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group3(),
        )
    }
}
impl BulkContraction<AntiFlatPoint> for VersorEven {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       12        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(right_dual_g0[3] * self[e12345]),
            // e15, e25, e35, scalar
            (Simd32x3::from(self[e12345]) * right_dual_g0.xyz())
                .with_w(-(right_dual_g0[0] * self[e423]) - (right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412]) - (right_dual_g0[3] * self[e321])),
        )
    }
}
impl BulkContraction<AntiFlector> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd3        0        4        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       13       19        0
    //  no simd       34       48        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (right_dual_g1.yzxx() * self.group0().zxy().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual_g1[1] * self[e2]) + (right_dual_g1[2] * self[e3]) + (right_dual_g1[3] * self[e4])
                        - (right_dual_g0[1] * self[e431])
                        - (right_dual_g0[2] * self[e412])
                        - (right_dual_g0[3] * self[e321]),
                )
                - (self.group0().yzxx() * right_dual_g1.zxy().with_w(right_dual_g0[0])),
            // e23, e31, e12, e45
            (self.group0() * right_dual_g1.www().with_w(right_dual_g0[3])) + Simd32x3::from(0.0).with_w(-(right_dual_g1[1] * self[e425]) - (right_dual_g1[2] * self[e435]))
                - (right_dual_g1.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(right_dual_g1[3]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e12345]) * right_dual_g0.xyz()).with_w(0.0)
                + (right_dual_g1.zxy() * self.group2().yzx()).with_w(0.0)
                - (right_dual_g1.yzx() * self.group2().zxy()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            right_dual_g1 * Simd32x4::from(self[e12345]),
        )
    }
}
impl BulkContraction<AntiLine> for VersorEven {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        5        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       13       30        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (right_dual_g0 * self.group0().www()).with_w(0.0),
            // e235, e315, e125, e4
            (right_dual_g1 * Simd32x3::from(self[e12345])).with_w(-(right_dual_g0[0] * self[e423]) - (right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[0] * self[e321]) + (right_dual_g1[1] * self[e412]),
                (right_dual_g0[1] * self[e321]) + (right_dual_g1[2] * self[e423]),
                (right_dual_g0[2] * self[e321]) + (right_dual_g1[0] * self[e431]),
                -(right_dual_g0[1] * self[e315])
                    - (right_dual_g0[2] * self[e125])
                    - (right_dual_g1[0] * self[e415])
                    - (right_dual_g1[1] * self[e425])
                    - (right_dual_g1[2] * self[e435]),
            ]) - (right_dual_g1.zxy() * self.group0().yzx()).with_w(right_dual_g0[0] * self[e235]),
        )
    }
}
impl BulkContraction<AntiMotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        1        2        0
    //    simd4        4        7        0
    // Totals...
    // yes simd       14       24        0
    //  no simd       28       49        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(right_dual_g0[3]) * self.group0(),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_dual_g0[3]) * self.group1().xyz()) + (Simd32x3::from(self[e12345]) * right_dual_g0.xyz())).with_w(right_dual_g0[3] * self[e321]),
            // e235, e315, e125, e5
            (right_dual_g1 * Simd32x4::from(self[e12345]))
                + (Simd32x4::from(right_dual_g0[3]) * self.group2())
                + Simd32x3::from(0.0).with_w(
                    -(right_dual_g0[0] * self[e235])
                        - (right_dual_g0[1] * self[e315])
                        - (right_dual_g0[2] * self[e125])
                        - (right_dual_g1[0] * self[e415])
                        - (right_dual_g1[1] * self[e425])
                        - (right_dual_g1[2] * self[e435]),
                ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g0[3] * self[e1]) + (right_dual_g1[1] * self[e412]),
                (right_dual_g0[3] * self[e2]) + (right_dual_g1[2] * self[e423]),
                (right_dual_g0[3] * self[e3]) + (right_dual_g1[0] * self[e431]),
                -(right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412]),
            ]) + (right_dual_g0 * self.group1().www().with_w(self[e4]))
                - (self.group0().yzxx() * right_dual_g1.zxy().with_w(right_dual_g0[0])),
        )
    }
}
impl BulkContraction<AntiPlane> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        6        0
    //    simd4        4        4        0
    // Totals...
    // yes simd        7       15        0
    //  no simd       19       39        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (right_dual_g0.yzxx() * self.group0().zxy().with_w(self[e1]))
                + (right_dual_g0.zxy() * self.group0().yzx() * Simd32x3::from(-1.0))
                    .with_w((right_dual_g0[1] * self[e2]) + (right_dual_g0[2] * self[e3]) + (right_dual_g0[3] * self[e4])),
            // e23, e31, e12, e45
            (self.group0().xyz() * right_dual_g0.www()).with_w(-(right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435])) - (right_dual_g0.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(right_dual_g0[3]) * self.group1().xyz()).with_w(0.0) + (right_dual_g0.zxy() * self.group2().yzx()).with_w(0.0)
                - (right_dual_g0.yzx() * self.group2().zxy()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            right_dual_g0 * Simd32x4::from(self[e12345]),
        )
    }
}
impl BulkContraction<AntiScalar> for VersorEven {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bulk_contraction(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e12345] * self[e12345] * -1.0)
    }
}
impl BulkContraction<Circle> for VersorEven {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd        9       24        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            right_dual_g1 * Simd32x4::from(self[e12345]),
            // e15, e25, e35, scalar
            (Simd32x3::from(self[e12345]) * other.group2()).with_w(
                -(right_dual_g1[0] * self[e415])
                    - (right_dual_g1[1] * self[e425])
                    - (right_dual_g1[2] * self[e435])
                    - (right_dual_g1[3] * self[e321])
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
impl BulkContraction<CircleRotor> for VersorEven {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       11        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       10       29        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            right_dual_g1 * Simd32x4::from(self[e12345]),
            // e15, e25, e35, scalar
            (Simd32x3::from(self[e12345]) * right_dual_g2.xyz()).with_w(
                (right_dual_g2[3] * self[e12345])
                    - (right_dual_g1[0] * self[e415])
                    - (right_dual_g1[1] * self[e425])
                    - (right_dual_g1[2] * self[e435])
                    - (right_dual_g1[3] * self[e321])
                    - (right_dual_g2[0] * self[e423])
                    - (right_dual_g2[1] * self[e431])
                    - (right_dual_g2[2] * self[e412])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
            ),
        )
    }
}
impl BulkContraction<Dipole> for VersorEven {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        6        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       19       32        0
    //  no simd       25       50        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            right_dual_g0 * Simd32x3::from(self[e12345]),
            // e415, e425, e435, e321
            right_dual_g1 * Simd32x4::from(self[e12345]),
            // e235, e315, e125, e4
            (right_dual_g2 * Simd32x3::from(self[e12345])).with_w(
                -(right_dual_g0[0] * self[e415])
                    - (right_dual_g0[1] * self[e425])
                    - (right_dual_g0[2] * self[e435])
                    - (right_dual_g1[0] * self[e423])
                    - (right_dual_g1[1] * self[e431])
                    - (right_dual_g1[2] * self[e412]),
            ),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[2] * self[e315]) + (right_dual_g2[1] * self[e412]) + (right_dual_g1[0] * self[e321]) + (right_dual_g1[3] * self[e415]),
                (right_dual_g0[0] * self[e125]) + (right_dual_g2[2] * self[e423]) + (right_dual_g1[1] * self[e321]) + (right_dual_g1[3] * self[e425]),
                (right_dual_g0[1] * self[e235]) + (right_dual_g2[0] * self[e431]) + (right_dual_g1[2] * self[e321]) + (right_dual_g1[3] * self[e435]),
                -(right_dual_g2[2] * self[e435]) - (right_dual_g1[0] * self[e235]) - (right_dual_g1[1] * self[e315]) - (right_dual_g1[2] * self[e125]),
            ]) - (right_dual_g0.yzx() * self.group2().zxy()).with_w(right_dual_g2[0] * self[e415])
                - (right_dual_g2.zxy() * self.group0().yzx()).with_w(right_dual_g2[1] * self[e425]),
        )
    }
}
impl BulkContraction<DipoleInversion> for VersorEven {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       25        0
    //    simd3        0        5        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       21       35        0
    //  no simd       30       60        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            right_dual_g0 * Simd32x3::from(self[e12345]),
            // e415, e425, e435, e321
            right_dual_g1 * Simd32x4::from(self[e12345]),
            // e235, e315, e125, e4
            (Simd32x3::from(self[e12345]) * right_dual_g2.xyz()).with_w(
                (right_dual_g2[3] * self[e12345])
                    - (right_dual_g0[0] * self[e415])
                    - (right_dual_g0[1] * self[e425])
                    - (right_dual_g0[2] * self[e435])
                    - (right_dual_g1[0] * self[e423])
                    - (right_dual_g1[1] * self[e431])
                    - (right_dual_g1[2] * self[e412]),
            ),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g1[0] * self[e321]) + (right_dual_g1[3] * self[e415]) + (right_dual_g2[1] * self[e412]) + (right_dual_g3[0] * self[e12345]),
                (right_dual_g1[1] * self[e321]) + (right_dual_g1[3] * self[e425]) + (right_dual_g2[2] * self[e423]) + (right_dual_g3[1] * self[e12345]),
                (right_dual_g1[2] * self[e321]) + (right_dual_g1[3] * self[e435]) + (right_dual_g2[0] * self[e431]) + (right_dual_g3[2] * self[e12345]),
                -(right_dual_g1[2] * self[e125]) - (right_dual_g2[0] * self[e415]) - (right_dual_g2[1] * self[e425]) - (right_dual_g2[2] * self[e435]),
            ]) + (right_dual_g0.zxy() * self.group2().yzx()).with_w(right_dual_g3[3] * self[e12345])
                - (self.group2().zxyx() * right_dual_g0.yzx().with_w(right_dual_g1[0]))
                - (right_dual_g2.zxy() * self.group0().yzx()).with_w(right_dual_g1[1] * self[e315]),
        )
    }
}
impl BulkContraction<DualNum> for VersorEven {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd2        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        5        0
    //  no simd        1       12        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x2::from(-1.0);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            right_dual_g0.xx().with_zw(right_dual_g0[0], (right_dual_g0[0] * self[e4]) + (right_dual_g0[1] * self[e12345])) * self.group0().xyz().with_w(1.0),
            // e15, e25, e35, e3215
            Simd32x4::from(right_dual_g0[0]) * self.group1().xyz().with_w(self[e12345]),
        )
    }
}
impl BulkContraction<FlatPoint> for VersorEven {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       20        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            right_dual_g0 * Simd32x4::from(self[e12345]),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[1] * self[e412]) + (right_dual_g0[3] * self[e415]),
                (right_dual_g0[2] * self[e423]) + (right_dual_g0[3] * self[e425]),
                (right_dual_g0[0] * self[e431]) + (right_dual_g0[3] * self[e435]),
                -(right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]),
            ]) - (right_dual_g0.zxyx() * self.group0().yzx().with_w(self[e415])),
        )
    }
}
impl BulkContraction<Flector> for VersorEven {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        6       13        0
    //  no simd       12       28        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            right_dual_g0 * Simd32x4::from(self[e12345]),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g0[3] * self[e415]) + (right_dual_g1[0] * self[e12345]),
                (right_dual_g0[3] * self[e425]) + (right_dual_g1[1] * self[e12345]),
                (right_dual_g0[3] * self[e435]) + (right_dual_g1[2] * self[e12345]),
                -(right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]),
            ]) + (self.group0().zxyw() * right_dual_g0.yzx().with_w(right_dual_g1[3]))
                - (right_dual_g0.zxyx() * self.group0().yzx().with_w(self[e415])),
        )
    }
}
impl BulkContraction<Line> for VersorEven {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
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
            (other.group1() * self.group0().www()).with_w(0.0),
        )
    }
}
impl BulkContraction<Motor> for VersorEven {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        2        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       16       29        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (right_dual_g0 * Simd32x4::from(self[e12345]))
                + (Simd32x4::from(right_dual_g1[3]) * self.group0().xyz().with_w(self[e4]))
                + Simd32x3::from(0.0).with_w(
                    -(right_dual_g0[0] * self[e415])
                        - (right_dual_g0[1] * self[e425])
                        - (right_dual_g0[2] * self[e435])
                        - (right_dual_g1[0] * self[e423])
                        - (right_dual_g1[1] * self[e431])
                        - (right_dual_g1[2] * self[e412]),
                ),
            // e15, e25, e35, e3215
            ((Simd32x3::from(right_dual_g1[3]) * self.group1().xyz()) + (Simd32x3::from(self[e12345]) * right_dual_g1.xyz())).with_w(right_dual_g1[3] * self[e12345]),
        )
    }
}
impl BulkContraction<MultiVector> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       55        0
    //    simd2        0        1        0
    //    simd3        8       17        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       53       82        0
    //  no simd       90      144        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1 = other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3 = other.group8().with_w(other[e321] * -1.0);
        let right_dual_g6 = other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_dual_g9 = other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g10 * self[e5])
                    + (right_dual_g0[0] * self[e12345])
                    + (right_dual_g9[0] * self[e1])
                    + (right_dual_g9[1] * self[e2])
                    + (right_dual_g9[2] * self[e3])
                    + (right_dual_g9[3] * self[e4])
                    - (right_dual_g3[0] * self[e423])
                    - (right_dual_g3[1] * self[e431])
                    - (right_dual_g3[2] * self[e412])
                    - (right_dual_g3[3] * self[e321])
                    - (other[e415] * self[e415])
                    - (other[e425] * self[e425])
                    - (other[e435] * self[e435])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
                right_dual_g0[1] * self[e12345],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g8[1] * self[e412]) + (right_dual_g1[0] * self[e12345]) + (right_dual_g6[0] * self[e321]) + (right_dual_g6[3] * self[e415]),
                (right_dual_g8[2] * self[e423]) + (right_dual_g1[1] * self[e12345]) + (right_dual_g6[1] * self[e321]) + (right_dual_g6[3] * self[e425]),
                (right_dual_g8[0] * self[e431]) + (right_dual_g1[2] * self[e12345]) + (right_dual_g6[2] * self[e321]) + (right_dual_g6[3] * self[e435]),
                -(right_dual_g7[2] * self[e435]) - (right_dual_g6[0] * self[e423]) - (right_dual_g6[1] * self[e431]) - (right_dual_g6[2] * self[e412]),
            ]) + (Simd32x4::from(right_dual_g0[1]) * self.group3())
                + (right_dual_g7.zxy() * self.group2().yzx()).with_w(right_dual_g1[3] * self[e12345])
                - (right_dual_g7.yzx() * self.group2().zxy()).with_w(right_dual_g7[0] * self[e415])
                - (right_dual_g8.zxy() * self.group0().yzx()).with_w(right_dual_g7[1] * self[e425]),
            // e5
            (right_dual_g0[1] * self[e5]) + (other[e3215] * self[e12345])
                - (right_dual_g8[0] * self[e415])
                - (right_dual_g8[1] * self[e425])
                - (right_dual_g8[2] * self[e435])
                - (right_dual_g6[0] * self[e235])
                - (right_dual_g6[1] * self[e315])
                - (right_dual_g6[2] * self[e125]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g9[2] * self[e315]) + (right_dual_g9[3] * self[e415]),
                (right_dual_g9[0] * self[e125]) + (right_dual_g9[3] * self[e425]),
                (right_dual_g9[1] * self[e235]) + (right_dual_g9[3] * self[e435]),
                -(right_dual_g9[1] * self[e425]) - (right_dual_g9[2] * self[e435]),
            ]) + (right_dual_g3 * Simd32x4::from(self[e12345]))
                - (right_dual_g9.yzxx() * self.group2().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(right_dual_g10) * self.group1().xyz()) + (Simd32x3::from(self[e12345]) * other.group7()) + (right_dual_g9.yzx() * self.group0().zxy())
                - (right_dual_g9.zxy() * self.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(right_dual_g10) * self.group2().xyz())
                + (Simd32x3::from(right_dual_g9[3]) * self.group0().xyz())
                + (Simd32x3::from(self[e12345]) * other.group6().xyz())
                - (Simd32x3::from(self[e321]) * right_dual_g9.xyz()),
            // e415, e425, e435, e321
            (right_dual_g6 * Simd32x4::from(self[e12345])) + (Simd32x4::from(right_dual_g0[1]) * self.group1()),
            // e423, e431, e412
            (right_dual_g7 * Simd32x3::from(self[e12345])) + (Simd32x3::from(right_dual_g0[1]) * self.group0().xyz()),
            // e235, e315, e125
            (right_dual_g8 * Simd32x3::from(self[e12345])) + (Simd32x3::from(right_dual_g0[1]) * self.group2().xyz()),
            // e4235, e4315, e4125, e3215
            right_dual_g9 * Simd32x4::from(self[e12345]),
            // e1234
            right_dual_g10 * self[e12345],
        )
    }
}
impl BulkContraction<Plane> for VersorEven {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn bulk_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(self[e12345]) * Simd32x4::from([other[e4235] * -1.0, other[e4315] * -1.0, other[e4125] * -1.0, other[e3215]]),
        )
    }
}
impl BulkContraction<RoundPoint> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       14        0
    //    simd3        2        6        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       11       24        0
    //  no simd       24       48        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other[e4] * -1.0;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (right_dual_g0.yzxx() * self.group0().zxy().with_w(self[e1]))
                + (right_dual_g0.zxy() * self.group0().yzx() * Simd32x3::from(-1.0))
                    .with_w((right_dual_g1 * self[e5]) + (right_dual_g0[2] * self[e3]) + (right_dual_g0[3] * self[e4]))
                + (Simd32x3::from(right_dual_g1) * self.group1().xyz()).with_w(right_dual_g0[1] * self[e2]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual_g1 * self[e235]) + (right_dual_g0[3] * self[e423]),
                (right_dual_g1 * self[e315]) + (right_dual_g0[3] * self[e431]),
                (right_dual_g1 * self[e125]) + (right_dual_g0[3] * self[e412]),
                -(right_dual_g0[1] * self[e425]) - (right_dual_g0[2] * self[e435]),
            ]) - (right_dual_g0.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual_g0[3]) * self.group1().xyz()) + (right_dual_g0.zxy() * self.group2().yzx()) - (right_dual_g0.yzx() * self.group2().zxy()))
                .with_w(right_dual_g1 * self[e12345]),
            // e4235, e4315, e4125, e3215
            right_dual_g0 * Simd32x4::from(self[e12345]),
        )
    }
}
impl BulkContraction<Scalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
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
impl BulkContraction<Sphere> for VersorEven {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215] * self[e12345],
        )
    }
}
impl BulkContraction<VersorEven> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       23        0
    //    simd3        3        6        0
    //    simd4        6        9        0
    // Totals...
    // yes simd       24       38        0
    //  no simd       48       77        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0().zxyw() * right_dual_g3.yzx().with_w(right_dual_g0[3]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual_g3[1] * self[e2]) + (right_dual_g3[2] * self[e3]) + (right_dual_g3[3] * self[e4])
                        - (right_dual_g0[0] * self[e235])
                        - (right_dual_g0[1] * self[e315])
                        - (right_dual_g0[2] * self[e125])
                        - (right_dual_g1[0] * self[e415])
                        - (right_dual_g1[1] * self[e425])
                        - (right_dual_g1[2] * self[e435])
                        - (right_dual_g1[3] * self[e321])
                        - (right_dual_g2[1] * self[e431])
                        - (right_dual_g2[2] * self[e412]),
                )
                + (right_dual_g0.xyz() * self.group0().www()).with_w(right_dual_g2[3] * self[e5])
                + (self.group1().xyz() * right_dual_g2.www()).with_w(right_dual_g3[0] * self[e1])
                - (self.group0().yzxx() * right_dual_g3.zxy().with_w(right_dual_g2[0])),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual_g1[0] * self[e12345]) + (right_dual_g2[3] * self[e235]),
                (right_dual_g1[1] * self[e12345]) + (right_dual_g2[3] * self[e315]),
                (right_dual_g1[2] * self[e12345]) + (right_dual_g2[3] * self[e125]),
                -(right_dual_g3[1] * self[e425]) - (right_dual_g3[2] * self[e435]),
            ]) + (self.group0() * right_dual_g3.www().with_w(right_dual_g1[3]))
                - (right_dual_g3.xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual_g3[3]) * self.group1().xyz()) + (Simd32x3::from(self[e12345]) * right_dual_g2.xyz()) + (right_dual_g3.zxy() * self.group2().yzx())
                - (right_dual_g3.yzx() * self.group2().zxy()))
            .with_w(right_dual_g2[3] * self[e12345]),
            // e4235, e4315, e4125, e3215
            right_dual_g3 * Simd32x4::from(self[e12345]),
        )
    }
}
impl BulkContraction<VersorOdd> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       25        0
    //    simd3        1        4        0
    //    simd4        7       10        0
    // Totals...
    // yes simd       25       39        0
    //  no simd       48       77        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x3::from(right_dual_g0[3]) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * right_dual_g0.xyz())).with_w(right_dual_g0[3] * self[e12345]),
            // e415, e425, e435, e321
            (right_dual_g1 * Simd32x4::from(self[e12345])) + (Simd32x4::from(right_dual_g0[3]) * self.group1()),
            // e235, e315, e125, e5
            (right_dual_g2 * Simd32x4::from(self[e12345]))
                + (Simd32x4::from(right_dual_g0[3]) * self.group2())
                + Simd32x3::from(0.0).with_w(
                    -(right_dual_g1[0] * self[e235])
                        - (right_dual_g1[1] * self[e315])
                        - (right_dual_g1[2] * self[e125])
                        - (right_dual_g2[0] * self[e415])
                        - (right_dual_g2[1] * self[e425])
                        - (right_dual_g2[2] * self[e435]),
                ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g1[0] * self[e321]) + (right_dual_g1[3] * self[e415]) + (right_dual_g2[1] * self[e412]) + (right_dual_g3[0] * self[e12345]),
                (right_dual_g1[1] * self[e321]) + (right_dual_g1[3] * self[e425]) + (right_dual_g2[2] * self[e423]) + (right_dual_g3[1] * self[e12345]),
                (right_dual_g1[2] * self[e321]) + (right_dual_g1[3] * self[e435]) + (right_dual_g2[0] * self[e431]) + (right_dual_g3[2] * self[e12345]),
                -(right_dual_g0[2] * self[e435]) - (right_dual_g1[0] * self[e423]) - (right_dual_g1[1] * self[e431]) - (right_dual_g1[2] * self[e412]),
            ]) + (right_dual_g0.zxyw() * self.group2().yzx().with_w(self[e4]))
                + (self.group3().xyz() * right_dual_g0.www()).with_w(right_dual_g3[3] * self[e12345])
                - (right_dual_g0.yzxx() * self.group2().zxy().with_w(self[e415]))
                - (right_dual_g2.zxy() * self.group0().yzx()).with_w(right_dual_g0[1] * self[e425]),
        )
    }
}
impl std::ops::Div<BulkContractionInfix> for VersorOdd {
    type Output = BulkContractionInfixPartial<VersorOdd>;
    fn div(self, _rhs: BulkContractionInfix) -> Self::Output {
        BulkContractionInfixPartial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       27        0
    //    simd3        3        8        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       22       39        0
    //  no simd       40       67        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (right_dual_g1[0] * self[e1234]) + (right_dual_g2[3] * self[e41]),
                (right_dual_g1[1] * self[e1234]) + (right_dual_g2[3] * self[e42]),
                (right_dual_g1[2] * self[e1234]) + (right_dual_g2[3] * self[e43]),
                -(right_dual_g0[1] * self[e25])
                    - (right_dual_g0[2] * self[e35])
                    - (right_dual_g1[0] * self[e23])
                    - (right_dual_g1[1] * self[e31])
                    - (right_dual_g1[2] * self[e12])
                    - (right_dual_g1[3] * self[e45])
                    - (right_dual_g2[0] * self[e41])
                    - (right_dual_g2[1] * self[e42])
                    - (right_dual_g2[2] * self[e43]),
            ]) + (right_dual_g0.zxy() * self.group3().yzx()).with_w(right_dual_g2[3] * self[scalar])
                - (right_dual_g0.yzx() * self.group3().zxy()).with_w(right_dual_g0[0] * self[e15]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual_g2[0] * self[e1234]) + (right_dual_g2[3] * self[e23]),
                (right_dual_g2[1] * self[e1234]) + (right_dual_g2[3] * self[e31]),
                (right_dual_g2[2] * self[e1234]) + (right_dual_g2[3] * self[e12]),
                -(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125]),
            ]) + (right_dual_g0 * self.group3().www()).with_w(right_dual_g2[3] * self[e45])
                - (right_dual_g1.wwwx() * self.group3().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual_g2[3]) * self.group2().xyz()) + (Simd32x3::from(self[e3215]) * right_dual_g1.xyz()) + (right_dual_g2.yzx() * self.group3().zxy())
                - (right_dual_g2.zxy() * self.group3().yzx()))
            .with_w(right_dual_g2[3] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g2[3]) * self.group3(),
        )
    }
}
impl BulkContraction<AntiDipoleInversion> for VersorOdd {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        1        5        0
    //    simd4       10       12        0
    // Totals...
    // yes simd       16       26        0
    //  no simd       48       72        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual_g3.xyz()) - (Simd32x3::from(right_dual_g2[3]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (right_dual_g3.yzxw() * self.group3().zxy().with_w(self[e1234])) - (self.group3().yzxw() * right_dual_g3.zxy().with_w(right_dual_g2[3])),
            // e235, e315, e125, e4
            (self.group3().xyzx() * right_dual_g3.www().with_w(other[e423]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual_g1[3] * self[e1234]) + (other[e431] * self[e4315]) + (other[e412] * self[e4125])
                        - (right_dual_g3[0] * self[e41])
                        - (right_dual_g3[1] * self[e42])
                        - (right_dual_g3[2] * self[e43]),
                )
                - (right_dual_g3.xyz() * self.group3().www()).with_w(right_dual_g2[3] * self[e45]),
            // e1, e2, e3, e5
            (Simd32x4::from(right_dual_g3[3]) * self.group0().xyz().with_w(self[e45]))
                + (right_dual_g3.zxyz() * self.group1().yzx().with_w(self[e35]))
                + (self.group2().wwwy() * right_dual_g2.xyz().with_w(right_dual_g3[1]))
                + (right_dual_g1.zxy() * self.group3().yzx()).with_w(right_dual_g3[0] * self[e15])
                - (Simd32x4::from(self[e3215]) * other.group0().with_w(right_dual_g1[3]))
                - (right_dual_g2.wwwy() * self.group2().xyz().with_w(self[e4315]))
                - (self.group3().zxyx() * right_dual_g1.yzx().with_w(right_dual_g2[0]))
                - (right_dual_g3.yzx() * self.group1().zxy()).with_w(right_dual_g2[2] * self[e4125]),
        )
    }
}
impl BulkContraction<AntiDualNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1       18        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0().yy().with_zw(other[scalar], (other[e3215] * self[e1234]) + (other[scalar] * self[scalar])) * self.group0().xyz().with_w(1.0),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[scalar]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group3(),
        )
    }
}
impl BulkContraction<AntiFlatPoint> for VersorOdd {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            right_dual_g0 * Simd32x4::from(self[e1234]),
            // e5
            -(right_dual_g0[0] * self[e4235]) - (right_dual_g0[1] * self[e4315]) - (right_dual_g0[2] * self[e4125]) - (right_dual_g0[3] * self[e3215]),
        )
    }
}
impl BulkContraction<AntiFlector> for VersorOdd {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        1        5        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       11       20        0
    //  no simd       31       48        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * right_dual_g1.xyz(),
            // e415, e425, e435, e321
            ((right_dual_g1.yzx() * self.group3().zxy()) - (right_dual_g1.zxy() * self.group3().yzx())).with_w(right_dual_g1[3] * self[e1234]),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(-(right_dual_g1[1] * self[e42]) - (right_dual_g1[2] * self[e43]))
                + (self.group3().xyz() * right_dual_g1.www()).with_w(right_dual_g0[3] * self[e1234])
                - (right_dual_g1.xyzx() * self.group3().www().with_w(self[e41])),
            // e1, e2, e3, e5
            (right_dual_g1.zxyy() * self.group1().yzx().with_w(self[e25]))
                + (right_dual_g1.wwwz() * self.group0().xyz().with_w(self[e35]))
                + (self.group2().wwwx() * right_dual_g0.xyz().with_w(right_dual_g1[0]))
                + Simd32x3::from(0.0)
                    .with_w((right_dual_g1[3] * self[e45]) - (right_dual_g0[1] * self[e4315]) - (right_dual_g0[2] * self[e4125]) - (right_dual_g0[3] * self[e3215]))
                - (right_dual_g1.yzx() * self.group1().zxy()).with_w(right_dual_g0[0] * self[e4235]),
        )
    }
}
impl BulkContraction<AntiLine> for VersorOdd {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        5        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       13       30        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            right_dual_g0 * Simd32x3::from(self[e1234]),
            // e23, e31, e12, e45
            (right_dual_g1 * Simd32x3::from(self[e1234])).with_w(-(right_dual_g0[0] * self[e4235]) - (right_dual_g0[1] * self[e4315]) - (right_dual_g0[2] * self[e4125])),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (right_dual_g0[0] * self[e3215]) + (right_dual_g1[1] * self[e4125]),
                (right_dual_g0[1] * self[e3215]) + (right_dual_g1[2] * self[e4235]),
                (right_dual_g0[2] * self[e3215]) + (right_dual_g1[0] * self[e4315]),
                -(right_dual_g0[1] * self[e31]) - (right_dual_g0[2] * self[e12]) - (right_dual_g1[0] * self[e41]) - (right_dual_g1[1] * self[e42]) - (right_dual_g1[2] * self[e43]),
            ]) - (right_dual_g1.zxy() * self.group3().yzx()).with_w(right_dual_g0[0] * self[e23]),
        )
    }
}
impl BulkContraction<AntiMotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       11        0
    //    simd3        3        5        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       28       50        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (right_dual_g0 * self.group2().www().with_w(self[scalar]))
                + Simd32x3::from(0.0).with_w(
                    -(right_dual_g0[0] * self[e23])
                        - (right_dual_g0[1] * self[e31])
                        - (right_dual_g0[2] * self[e12])
                        - (right_dual_g1[0] * self[e41])
                        - (right_dual_g1[1] * self[e42])
                        - (right_dual_g1[2] * self[e43]),
                )
                + (self.group0().xyz() * right_dual_g0.www()).with_w(right_dual_g1[3] * self[e1234]),
            // e23, e31, e12, e45
            (Simd32x4::from(right_dual_g0[3]) * self.group1())
                + (Simd32x4::from([self[e1234], self[e1234], self[e1234], 1.0])
                    * right_dual_g1
                        .xyz()
                        .with_w(-(right_dual_g0[0] * self[e4235]) - (right_dual_g0[1] * self[e4315]) - (right_dual_g0[2] * self[e4125]))),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual_g0[3]) * self.group2().xyz()) + (Simd32x3::from(self[e3215]) * right_dual_g0.xyz()) + (right_dual_g1.yzx() * self.group3().zxy())
                - (right_dual_g1.zxy() * self.group3().yzx()))
            .with_w(right_dual_g0[3] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g0[3]) * self.group3(),
        )
    }
}
impl BulkContraction<AntiPlane> for VersorOdd {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        5        0
    //    simd3        1        6        0
    //    simd4        3        4        0
    // Totals...
    // yes simd        6       15        0
    //  no simd       17       39        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * right_dual_g0.xyz(),
            // e415, e425, e435, e321
            ((right_dual_g0.yzx() * self.group3().zxy()) - (right_dual_g0.zxy() * self.group3().yzx())).with_w(right_dual_g0[3] * self[e1234]),
            // e235, e315, e125, e4
            (self.group3().xyz() * right_dual_g0.www()).with_w(-(right_dual_g0[1] * self[e42]) - (right_dual_g0[2] * self[e43]))
                - (right_dual_g0.xyzx() * self.group3().www().with_w(self[e41])),
            // e1, e2, e3, e5
            (right_dual_g0.zxyx() * self.group1().yzx().with_w(self[e15]))
                + (right_dual_g0.wwwy() * self.group0().xyz().with_w(self[e25]))
                + (right_dual_g0.yzx() * self.group1().zxy() * Simd32x3::from(-1.0)).with_w((right_dual_g0[2] * self[e35]) + (right_dual_g0[3] * self[e45])),
        )
    }
}
impl BulkContraction<Circle> for VersorOdd {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       16        0
    //  no simd       15       24        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual_g1[1] * self[e4125]) - (other[e423] * self[e3215]),
                -(right_dual_g1[2] * self[e4235]) - (other[e431] * self[e3215]),
                -(right_dual_g1[0] * self[e4315]) - (other[e412] * self[e3215]),
                (right_dual_g1[3] * self[e1234]) + (other[e412] * self[e4125]),
            ]) + (self.group3().yzxy() * right_dual_g1.zxy().with_w(other[e431]))
                + (other.group2() * self.group2().www()).with_w(other[e423] * self[e4235]),
            // e5
            -(right_dual_g1[3] * self[e3215]) - (other[e235] * self[e4235]) - (other[e315] * self[e4315]) - (other[e125] * self[e4125]),
        )
    }
}
impl BulkContraction<CircleRotor> for VersorOdd {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       15       28        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual_g1[1] * self[e4125]) - (other[e423] * self[e3215]),
                -(right_dual_g1[2] * self[e4235]) - (other[e431] * self[e3215]),
                -(right_dual_g1[0] * self[e4315]) - (other[e412] * self[e3215]),
                (right_dual_g1[3] * self[e1234]) + (other[e412] * self[e4125]),
            ]) + (self.group3().yzxx() * right_dual_g1.zxy().with_w(other[e423]))
                + (right_dual_g2.xyz() * self.group2().www()).with_w(other[e431] * self[e4315]),
            // e5
            -(right_dual_g1[3] * self[e3215]) - (right_dual_g2[0] * self[e4235]) - (right_dual_g2[1] * self[e4315]) - (right_dual_g2[2] * self[e4125]),
        )
    }
}
impl BulkContraction<Dipole> for VersorOdd {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       24        0
    //    simd3        2        6        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       19       32        0
    //  no simd       29       50        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_dual_g1.xyz()) + (right_dual_g0.zxy() * self.group3().yzx()) - (right_dual_g0.yzx() * self.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual_g0[0] * self[e3215]) + (right_dual_g2[0] * self[e1234]),
                (right_dual_g0[1] * self[e3215]) + (right_dual_g2[1] * self[e1234]),
                (right_dual_g0[2] * self[e3215]) + (right_dual_g2[2] * self[e1234]),
                -(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125]),
            ]) - (right_dual_g1.wwwx() * self.group3().xyzx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (right_dual_g2[1] * self[e4125]) + (right_dual_g1[0] * self[e3215]),
                (right_dual_g2[2] * self[e4235]) + (right_dual_g1[1] * self[e3215]),
                (right_dual_g2[0] * self[e4315]) + (right_dual_g1[2] * self[e3215]),
                -(right_dual_g0[1] * self[e25])
                    - (right_dual_g0[2] * self[e35])
                    - (right_dual_g2[0] * self[e41])
                    - (right_dual_g2[1] * self[e42])
                    - (right_dual_g2[2] * self[e43])
                    - (right_dual_g1[0] * self[e23])
                    - (right_dual_g1[1] * self[e31])
                    - (right_dual_g1[2] * self[e12])
                    - (right_dual_g1[3] * self[e45]),
            ]) - (right_dual_g2.zxy() * self.group3().yzx()).with_w(right_dual_g0[0] * self[e15]),
        )
    }
}
impl BulkContraction<DipoleInversion> for VersorOdd {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        2        5        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       21       32        0
    //  no simd       37       60        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3 = other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_dual_g1.xyz()) + (right_dual_g0.zxy() * self.group3().yzx()) - (right_dual_g0.yzx() * self.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual_g0[0] * self[e3215]) + (right_dual_g2[0] * self[e1234]),
                (right_dual_g0[1] * self[e3215]) + (right_dual_g2[1] * self[e1234]),
                (right_dual_g0[2] * self[e3215]) + (right_dual_g2[2] * self[e1234]),
                -(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125]),
            ]) - (right_dual_g1.wwwx() * self.group3().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(self[e3215]) * right_dual_g1.xyz().with_w(right_dual_g2[3]))
                + (self.group3().zxyx() * right_dual_g2.yzx().with_w(right_dual_g3[0]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual_g3[1] * self[e4315]) + (right_dual_g3[2] * self[e4125]) + (right_dual_g3[3] * self[e1234])
                        - (right_dual_g0[1] * self[e25])
                        - (right_dual_g0[2] * self[e35])
                        - (right_dual_g1[0] * self[e23])
                        - (right_dual_g1[1] * self[e31])
                        - (right_dual_g1[2] * self[e12])
                        - (right_dual_g1[3] * self[e45])
                        - (right_dual_g2[0] * self[e41])
                        - (right_dual_g2[1] * self[e42])
                        - (right_dual_g2[2] * self[e43]),
                )
                - (right_dual_g2.zxy() * self.group3().yzx()).with_w(right_dual_g0[0] * self[e15]),
        )
    }
}
impl BulkContraction<DualNum> for VersorOdd {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x2::from(-1.0);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual_g0[0]) * self.group3().xyz().with_w(self[e1234]),
            // e1, e2, e3, e5
            Simd32x4::from(right_dual_g0[0]) * self.group0().xyz().with_w(self[e45]),
        )
    }
}
impl BulkContraction<FlatPoint> for VersorOdd {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        1        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        9       21        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x4::from([self[e1234], self[e1234], self[e1234], 1.0])
                * right_dual_g0
                    .xyz()
                    .with_w(-(right_dual_g0[1] * self[e42]) - (right_dual_g0[2] * self[e43]) - (right_dual_g0[3] * self[e45])))
                - (right_dual_g0.wwwx() * self.group3().xyz().with_w(self[e41])),
            // e15, e25, e35, e3215
            ((right_dual_g0.yzx() * self.group3().zxy()) - (right_dual_g0.zxy() * self.group3().yzx())).with_w(0.0),
        )
    }
}
impl BulkContraction<Flector> for VersorOdd {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        3        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       16       28        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(
                (right_dual_g1[1] * self[e4315]) + (right_dual_g1[2] * self[e4125]) + (right_dual_g1[3] * self[e1234])
                    - (right_dual_g0[1] * self[e42])
                    - (right_dual_g0[2] * self[e43])
                    - (right_dual_g0[3] * self[e45]),
            ) + (right_dual_g0.xyz() * self.group2().www()).with_w(right_dual_g1[0] * self[e4235])
                - (right_dual_g0.wwwx() * self.group3().xyz().with_w(self[e41])),
            // e15, e25, e35, e3215
            ((right_dual_g0.yzx() * self.group3().zxy()) - (right_dual_g0.zxy() * self.group3().yzx())).with_w(0.0),
        )
    }
}
impl BulkContraction<Line> for VersorOdd {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e435] * self[e4315]) + (other[e235] * self[e1234]),
                (other[e415] * self[e4125]) + (other[e315] * self[e1234]),
                (other[e425] * self[e4235]) + (other[e125] * self[e1234]),
                -(other[e315] * self[e4315]) - (other[e125] * self[e4125]),
            ]) - (self.group3().zxyx() * other.group0().yzx().with_w(other[e235])),
        )
    }
}
impl BulkContraction<Motor> for VersorOdd {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        6       14        0
    //  no simd       12       28        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual_g1[3]) * self.group3().xyz().with_w(self[e1234]),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual_g1[0] * self[e1234]) + (right_dual_g1[3] * self[e41]),
                (right_dual_g1[1] * self[e1234]) + (right_dual_g1[3] * self[e42]),
                (right_dual_g1[2] * self[e1234]) + (right_dual_g1[3] * self[e43]),
                -(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125]),
            ]) + (right_dual_g0.zxy() * self.group3().yzx()).with_w(right_dual_g1[3] * self[e45])
                - (self.group3().zxyx() * right_dual_g0.yzx().with_w(right_dual_g1[0])),
        )
    }
}
impl BulkContraction<MultiVector> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       41        0
    //    simd2        0        1        0
    //    simd3        8       20        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       44       72        0
    //  no simd       90      143        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1 = other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3_w = other[e321] * -1.0;
        let right_dual_g6 = other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_dual_g9 = other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g0[1] * self[scalar])
                    + (right_dual_g1[0] * self[e4235])
                    + (right_dual_g1[1] * self[e4315])
                    + (right_dual_g1[2] * self[e4125])
                    + (right_dual_g1[3] * self[e3215])
                    + (other[e3215] * self[e1234])
                    - (right_dual_g7[0] * self[e15])
                    - (right_dual_g7[1] * self[e25])
                    - (right_dual_g7[2] * self[e35])
                    - (right_dual_g8[0] * self[e41])
                    - (right_dual_g8[1] * self[e42])
                    - (right_dual_g8[2] * self[e43])
                    - (right_dual_g6[0] * self[e23])
                    - (right_dual_g6[1] * self[e31])
                    - (right_dual_g6[2] * self[e12])
                    - (right_dual_g6[3] * self[e45]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (self.group3().yzxx() * other.group6().zxy().with_w(other[e423]))
                + (other.group8() * self.group2().www()).with_w(other[e431] * self[e4315])
                + (right_dual_g9.zxy() * self.group1().yzx()).with_w(other[e412] * self[e4125])
                + (self.group0().xyz() * right_dual_g9.www()).with_w(right_dual_g3_w * self[e1234])
                - (Simd32x4::from(right_dual_g10) * self.group2().xyz().with_w(self[e45]))
                - (right_dual_g9.yzxz() * self.group1().zxy().with_w(self[e43]))
                - (other.group7() * self.group3().www()).with_w(right_dual_g9[0] * self[e41])
                - (other.group6().yzx() * self.group3().zxy()).with_w(right_dual_g9[1] * self[e42]),
            // e5
            (right_dual_g9[0] * self[e15]) + (right_dual_g9[1] * self[e25]) + (right_dual_g9[2] * self[e35]) + (right_dual_g9[3] * self[e45])
                - (right_dual_g3_w * self[e3215])
                - (other[e235] * self[e4235])
                - (other[e315] * self[e4315])
                - (other[e125] * self[e4125]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual_g8[1] * self[e4125]) + (right_dual_g6[0] * self[e3215]),
                (right_dual_g8[2] * self[e4235]) + (right_dual_g6[1] * self[e3215]),
                (right_dual_g8[0] * self[e4315]) + (right_dual_g6[2] * self[e3215]),
                -(right_dual_g6[1] * self[e4315]) - (right_dual_g6[2] * self[e4125]),
            ]) + (Simd32x4::from(right_dual_g0[1]) * self.group2().xyz().with_w(self[e45]))
                - (self.group3().yzxx() * right_dual_g8.zxy().with_w(right_dual_g6[0])),
            // e41, e42, e43
            (Simd32x3::from(right_dual_g0[1]) * self.group0().xyz()) + (Simd32x3::from(self[e1234]) * right_dual_g6.xyz()) + (right_dual_g7.zxy() * self.group3().yzx())
                - (right_dual_g7.yzx() * self.group3().zxy()),
            // e23, e31, e12
            (right_dual_g7 * Simd32x3::from(self[e3215])) + (right_dual_g8 * Simd32x3::from(self[e1234])) + (Simd32x3::from(right_dual_g0[1]) * self.group1().xyz())
                - (Simd32x3::from(right_dual_g6[3]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (right_dual_g9.yzxw() * self.group3().zxy().with_w(self[e1234])) - (right_dual_g9.zxy() * self.group3().yzx()).with_w(right_dual_g10 * self[e3215]),
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual_g9.xyz()) - (Simd32x3::from(right_dual_g10) * self.group3().xyz()),
            // e235, e315, e125
            (Simd32x3::from(right_dual_g9[3]) * self.group3().xyz()) - (Simd32x3::from(self[e3215]) * right_dual_g9.xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g0[1]) * self.group3(),
            // e1234
            right_dual_g0[1] * self[e1234],
        )
    }
}
impl BulkContraction<Plane> for VersorOdd {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Scalar::from_groups(
            // scalar
            (right_dual_g0[0] * self[e4235]) + (right_dual_g0[1] * self[e4315]) + (right_dual_g0[2] * self[e4125]) + (right_dual_g0[3] * self[e1234]),
        )
    }
}
impl BulkContraction<RoundPoint> for VersorOdd {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       13        0
    //    simd3        1        4        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       11       22        0
    //  no simd       25       45        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other[e4] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual_g0.xyz()) - (Simd32x3::from(right_dual_g1) * self.group3().xyz()),
            // e415, e425, e435, e321
            (right_dual_g0.yzxw() * self.group3().zxy().with_w(self[e1234])) - (right_dual_g0.zxy() * self.group3().yzx()).with_w(right_dual_g1 * self[e3215]),
            // e235, e315, e125, e4
            (self.group3().xyz() * right_dual_g0.www()).with_w(-(right_dual_g1 * self[e45]) - (right_dual_g0[1] * self[e42]) - (right_dual_g0[2] * self[e43]))
                - (right_dual_g0.xyzx() * self.group3().www().with_w(self[e41])),
            // e1, e2, e3, e5
            Simd32x4::from([
                -(right_dual_g1 * self[e15]) - (right_dual_g0[1] * self[e12]),
                -(right_dual_g1 * self[e25]) - (right_dual_g0[2] * self[e23]),
                -(right_dual_g1 * self[e35]) - (right_dual_g0[0] * self[e31]),
                (right_dual_g0[2] * self[e35]) + (right_dual_g0[3] * self[e45]),
            ]) + (right_dual_g0.zxyx() * self.group1().yzx().with_w(self[e15]))
                + (right_dual_g0.wwwy() * self.group0().xyz().with_w(self[e25])),
        )
    }
}
impl BulkContraction<Scalar> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
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
impl BulkContraction<Sphere> for VersorOdd {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        6        0
    //  no simd        4        9        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Scalar::from_groups(
            // scalar
            (right_dual_g0[0] * self[e4235])
                + (right_dual_g0[1] * self[e4315])
                + (right_dual_g0[2] * self[e4125])
                + (right_dual_g0[3] * self[e3215])
                + (other[e3215] * self[e1234]),
        )
    }
}
impl BulkContraction<VersorEven> for VersorOdd {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        1        5        0
    //    simd4       10       13        0
    // Totals...
    // yes simd       16       27        0
    //  no simd       48       76        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual_g3.xyz()) - (Simd32x3::from(right_dual_g2[3]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (right_dual_g3.yzxw() * self.group3().zxy().with_w(self[e1234])) - (self.group3().yzxw() * right_dual_g3.zxy().with_w(right_dual_g2[3])),
            // e235, e315, e125, e4
            (self.group3().xyzx() * right_dual_g3.www().with_w(right_dual_g0[0]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual_g0[1] * self[e4315]) + (right_dual_g0[2] * self[e4125]) + (right_dual_g1[3] * self[e1234])
                        - (right_dual_g3[0] * self[e41])
                        - (right_dual_g3[1] * self[e42])
                        - (right_dual_g3[2] * self[e43]),
                )
                - (right_dual_g3.xyz() * self.group3().www()).with_w(right_dual_g2[3] * self[e45]),
            // e1, e2, e3, e5
            (Simd32x4::from(right_dual_g3[3]) * self.group0().xyz().with_w(self[e45]))
                + (right_dual_g3.zxyz() * self.group1().yzx().with_w(self[e35]))
                + (self.group2().wwwy() * right_dual_g2.xyz().with_w(right_dual_g3[1]))
                + (right_dual_g1.zxy() * self.group3().yzx()).with_w(right_dual_g3[0] * self[e15])
                - (Simd32x4::from(self[e3215]) * right_dual_g0.xyz().with_w(right_dual_g1[3]))
                - (right_dual_g2.wwwy() * self.group2().xyz().with_w(self[e4315]))
                - (self.group3().zxyx() * right_dual_g1.yzx().with_w(right_dual_g2[0]))
                - (right_dual_g3.yzx() * self.group1().zxy()).with_w(right_dual_g2[2] * self[e4125]),
        )
    }
}
impl BulkContraction<VersorOdd> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       23        0
    //    simd3        3        6        0
    //    simd4        6        9        0
    // Totals...
    // yes simd       24       38        0
    //  no simd       48       77        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3 = other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (right_dual_g0.zxyw() * self.group3().yzx().with_w(self[scalar]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual_g3[1] * self[e4315]) + (right_dual_g3[2] * self[e4125]) + (right_dual_g3[3] * self[e3215])
                        - (right_dual_g0[1] * self[e25])
                        - (right_dual_g0[2] * self[e35])
                        - (right_dual_g1[0] * self[e23])
                        - (right_dual_g1[1] * self[e31])
                        - (right_dual_g1[2] * self[e12])
                        - (right_dual_g1[3] * self[e45])
                        - (right_dual_g2[0] * self[e41])
                        - (right_dual_g2[1] * self[e42])
                        - (right_dual_g2[2] * self[e43]),
                )
                + (right_dual_g1.xyz() * self.group2().www()).with_w(right_dual_g3[0] * self[e4235])
                + (self.group0().xyz() * right_dual_g0.www()).with_w(right_dual_g2[3] * self[e1234])
                - (right_dual_g0.yzxx() * self.group3().zxy().with_w(self[e15])),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual_g0[3] * self[e23]) + (right_dual_g2[0] * self[e1234]),
                (right_dual_g0[3] * self[e31]) + (right_dual_g2[1] * self[e1234]),
                (right_dual_g0[3] * self[e12]) + (right_dual_g2[2] * self[e1234]),
                -(right_dual_g1[1] * self[e4315]) - (right_dual_g1[2] * self[e4125]),
            ]) + (right_dual_g0 * self.group3().www().with_w(self[e45]))
                - (right_dual_g1.wwwx() * self.group3().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual_g0[3]) * self.group2().xyz()) + (Simd32x3::from(self[e3215]) * right_dual_g1.xyz()) + (right_dual_g2.yzx() * self.group3().zxy())
                - (right_dual_g2.zxy() * self.group3().yzx()))
            .with_w(right_dual_g0[3] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g0[3]) * self.group3(),
        )
    }
}
