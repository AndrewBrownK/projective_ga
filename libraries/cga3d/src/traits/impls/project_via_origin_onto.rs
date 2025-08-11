// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 497
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0     N/A
//   Median:         7      15       0     N/A
//  Average:        14      24       0     N/A
//  Maximum:       202     245       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0       0
//   Median:        12      30       0       0
//  Average:        28      48       0       0
//  Maximum:       392     473       0       0
impl std::ops::Div<ProjectViaOriginOntoInfix> for AntiCircleRotor {
    type Output = ProjectViaOriginOntoInfixPartial<AntiCircleRotor>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiCircleRotor> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       20       23        0        0
    //    simd3        0        5        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd       20       31        0      N/A
    //  no simd       20       50        0        0
    fn project_via_origin_onto(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g0 = right_dual_g0 * Simd32x3::from(self[scalar]);
        let wedge_g1 = right_dual_g1 * Simd32x4::from(self[scalar]);
        let wedge_g2_xyz = Simd32x3::from(self[scalar] * -1.0) * other.group2().xyz();
        let wedge_g2_w = (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]) + (other[scalar] * self[scalar])
            - (right_dual_g0[0] * self[e15])
            - (right_dual_g0[1] * self[e25])
            - (right_dual_g0[2] * self[e35])
            - (right_dual_g1[0] * self[e23])
            - (right_dual_g1[1] * self[e31])
            - (right_dual_g1[2] * self[e12])
            - (right_dual_g1[3] * self[e45]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g2_w) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(wedge_g2_w) * other.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(wedge_g2_w) * other.group2().xyz()).with_w(
                (wedge_g2_w * other[scalar])
                    - (wedge_g0[0] * other[e15])
                    - (wedge_g0[1] * other[e25])
                    - (wedge_g0[2] * other[e35])
                    - (wedge_g2_xyz[0] * other[e41])
                    - (wedge_g2_xyz[1] * other[e42])
                    - (wedge_g2_xyz[2] * other[e43])
                    - (wedge_g1[0] * other[e23])
                    - (wedge_g1[1] * other[e31])
                    - (wedge_g1[2] * other[e12])
                    - (wedge_g1[3] * other[e45]),
            ),
        )
    }
}
impl ProjectViaOriginOnto<AntiDipoleInversion> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       15       20        0        0
    //    simd3       10       16        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd       27       39        0      N/A
    //  no simd       53       80        0        0
    fn project_via_origin_onto(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        let right_dual_g2_xyz = other.group2().xyz();
        let wedge_g0 = Simd32x3::from(self[scalar]) * other.group0();
        let wedge_g1 = right_dual_g1 * Simd32x4::from(self[scalar]);
        let wedge_g2_xyz = right_dual_g2_xyz * Simd32x3::from(self[scalar]);
        let wedge_g2_w = -(self[e41] * right_dual_g1[0])
            - (self[e42] * right_dual_g1[1])
            - (self[e43] * right_dual_g1[2])
            - (other[e423] * self[e23])
            - (other[e431] * self[e31])
            - (other[e412] * self[e12])
            - (self[scalar] * other[e4]);
        let wedge_g3_xyz = (Simd32x3::from(right_dual_g1[3]) * self.group1().xyz())
            + (Simd32x3::from(self[e45]) * right_dual_g1.xyz())
            + (Simd32x3::from(self[scalar]) * other.group3().xyz())
            + (right_dual_g2_xyz.zxy() * self.group0().yzx())
            + (other.group0().yzx() * self.group2().zxy())
            - (right_dual_g2_xyz.yzx() * self.group0().zxy())
            - (other.group0().zxy() * self.group2().yzx());
        let wedge_g3_w = self[scalar] * other[e5] * -1.0;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(wedge_g2_w) * other.group1().xyz()) + (wedge_g3_xyz.yzx() * other.group0().zxy()) - (wedge_g3_xyz.zxy() * other.group0().yzx()),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g2_w) * other.group2().xyz()) + (Simd32x3::from(wedge_g3_w) * other.group0()) - (wedge_g3_xyz * Simd32x3::from(other[e321]))).with_w(0.0),
            // e15, e25, e35, scalar
            (Simd32x4::from(wedge_g3_w) * other.group1().xyz().with_w(other[e4]))
                + (wedge_g3_xyz.zxy() * other.group2().yzx()).with_w(
                    (wedge_g3_xyz[0] * other[e1])
                        - (wedge_g0[1] * other[e315])
                        - (wedge_g0[2] * other[e125])
                        - (wedge_g2_xyz[0] * other[e423])
                        - (wedge_g2_xyz[1] * other[e431])
                        - (wedge_g2_xyz[2] * other[e412])
                        - (wedge_g1[0] * other[e415])
                        - (wedge_g1[1] * other[e425])
                        - (wedge_g1[2] * other[e435])
                        - (wedge_g1[3] * other[e321]),
                )
                - (other.group2().zxyx() * wedge_g3_xyz.yzx().with_w(wedge_g0[0])),
        )
    }
}
impl ProjectViaOriginOnto<AntiDualNum> for AntiCircleRotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        9        0        0
    fn project_via_origin_onto(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (self.group0() * Simd32x2::from(other[e3215]).with_z(other[e3215])).with_w(other[scalar] * self[scalar]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(other[scalar] * wedge_g0[3]),
            // e15, e25, e35, e3215
            wedge_g0 * Simd32x4::from(other[e3215]),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlatPoint> for AntiCircleRotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        3        7        0      N/A
    // Totals...
    // yes simd        3       11        0      N/A
    //  no simd        9       25        0        0
    fn project_via_origin_onto(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e321] * -1.0;
        let wedge_g1_xyz =
            (Simd32x3::from(right_dual_g0_w) * self.group1().xyz()) + (right_dual_g0_xyz.zxy() * self.group0().yzx()) - (right_dual_g0_xyz.yzx() * self.group0().zxy());
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (wedge_g1_xyz * Simd32x4::from(other[e321]).xyz() * Simd32x3::from(-1.0)).with_w(right_dual_g0_w * self[scalar] * other[e321] * -1.0),
            // e15, e25, e35, e3215
            ((wedge_g1_xyz.zxy() * other.group0().yzx()) - (wedge_g1_xyz.yzx() * other.group0().zxy())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlector> for AntiCircleRotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        4        8        0      N/A
    // Totals...
    // yes simd        7       14        0      N/A
    //  no simd       15       30        0        0
    fn project_via_origin_onto(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e321] * -1.0;
        let wedge_g1_xyz =
            (Simd32x3::from(right_dual_g0_w) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * other.group1().xyz()) + (right_dual_g0_xyz.zxy() * self.group0().yzx())
                - (right_dual_g0_xyz.yzx() * self.group0().zxy());
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (wedge_g1_xyz * Simd32x4::from(other[e321]).xyz() * Simd32x3::from(-1.0))
                .with_w((wedge_g1_xyz[0] * other[e1]) + (wedge_g1_xyz[1] * other[e2]) + (wedge_g1_xyz[2] * other[e3]) - (right_dual_g0_w * self[scalar] * other[e321])),
            // e15, e25, e35, e3215
            ((wedge_g1_xyz.zxy() * other.group0().yzx()) - (wedge_g1_xyz.yzx() * other.group0().zxy())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<AntiLine> for AntiCircleRotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7        9        0        0
    //    simd3        0        4        0      N/A
    // Totals...
    // yes simd        7       13        0      N/A
    //  no simd        7       21        0        0
    fn project_via_origin_onto(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = right_dual_g0 * Simd32x3::from(self[scalar]);
        let wedge_g0_w = (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35])
            - (right_dual_g0[0] * self[e23])
            - (right_dual_g0[1] * self[e31])
            - (right_dual_g0[2] * self[e12]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(wedge_g0_w) * other.group0()).with_w(-(wedge_g0_xyz[0] * other[e23]) - (wedge_g0_xyz[1] * other[e31]) - (wedge_g0_xyz[2] * other[e12])),
            // e15, e25, e35, e3215
            (Simd32x3::from(wedge_g0_w) * other.group1()).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<AntiMotor> for AntiCircleRotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        5       11        0      N/A
    //  no simd        9       21        0        0
    fn project_via_origin_onto(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = (Simd32x3::from(other[e3215]) * self.group0()) - (Simd32x3::from(self[scalar]) * other.group0().xyz());
        let wedge_g0_w = self[scalar] * other[scalar];
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(wedge_g0_w) * other.group0().xyz())
                .with_w((wedge_g0_w * other[scalar]) - (wedge_g0_xyz[0] * other[e23]) - (wedge_g0_xyz[1] * other[e31]) - (wedge_g0_xyz[2] * other[e12])),
            // e15, e25, e35, e3215
            ((wedge_g0_xyz * Simd32x3::from(other[e3215])) + (Simd32x3::from(wedge_g0_w) * other.group1().xyz())).with_w(wedge_g0_w * other[e3215]),
        )
    }
}
impl ProjectViaOriginOnto<AntiPlane> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2       11        0        0
    fn project_via_origin_onto(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(/* scalar */ (wedge_g0[0] * other[e1]) + (wedge_g0[1] * other[e2]) + (wedge_g0[2] * other[e3]))
    }
}
impl ProjectViaOriginOnto<AntiScalar> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        7        0      N/A
    //  no simd        0       15        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e12345] * -1.0;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual_g0 * other[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group2(),
        )
    }
}
impl ProjectViaOriginOnto<Circle> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        9       13        0      N/A
    // Totals...
    // yes simd       14       19        0      N/A
    //  no simd       32       45        0        0
    fn project_via_origin_onto(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g2_w = -(right_dual_g1_xyz[0] * self[e41])
            - (right_dual_g1_xyz[1] * self[e42])
            - (right_dual_g1_xyz[2] * self[e43])
            - (other[e423] * self[e23])
            - (other[e431] * self[e31])
            - (other[e412] * self[e12]);
        let wedge_g3_xyz = (right_dual_g1_xyz * Simd32x3::from(self[e45])) + (self.group0().yzx() * other.group2().zxy()) + (other.group0().yzx() * self.group2().zxy())
            - (Simd32x3::from(other[e321]) * self.group1().xyz())
            - (self.group0().zxy() * other.group2().yzx())
            - (other.group0().zxy() * self.group2().yzx());
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(wedge_g2_w) * other.group1().xyz()) + (wedge_g3_xyz.yzx() * other.group0().zxy()) - (wedge_g3_xyz.zxy() * other.group0().yzx()),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g2_w) * other.group2()) - (wedge_g3_xyz * Simd32x3::from(other[e321]))).with_w(0.0),
            // e15, e25, e35, scalar
            ((wedge_g3_xyz.zxy() * other.group2().yzx()) - (wedge_g3_xyz.yzx() * other.group2().zxy())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<CircleRotor> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1       11        0        0
    //    simd2        1        2        0      N/A
    //    simd3       13       16        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd       17       32        0      N/A
    //  no simd       50       75        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        let right_dual_g2_xyz = other.group2().xyz();
        let right_dual_g2_w = other[e12345] * -1.0;
        let wedge_g1 = (right_dual_g1 * Simd32x4::from(self[scalar])) + (Simd32x4::from(right_dual_g2_w) * self.group1());
        let wedge_g3 = ((Simd32x3::from(right_dual_g1[3]) * self.group1().xyz())
            + (Simd32x3::from(self[e45]) * right_dual_g1.xyz())
            + (right_dual_g2_xyz.zxy() * self.group0().yzx())
            + (other.group0().yzx() * self.group2().zxy())
            - (right_dual_g2_xyz.yzx() * self.group0().zxy())
            - (other.group0().zxy() * self.group2().yzx()))
        .with_w(0.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x3::from(right_dual_g2_w * other[e12345]) * self.group0())
                + (Simd32x3::from(self[scalar] * other[e12345]) * other.group0())
                + (other.group0().zxy() * wedge_g3.yzx())
                - (other.group0().yzx() * wedge_g3.zxy()))
            .with_w(right_dual_g2_w * self[scalar] * other[e12345]),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g3[3]) * other.group0()) + (Simd32x3::from(other[e12345]) * wedge_g1.xyz()) - (Simd32x3::from(other[e321]) * wedge_g3.xyz()))
                .with_w(wedge_g1[3] * other[e12345]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, (wedge_g3[1] * other[e235]) - (wedge_g3[0] * other[e315]), 0.0])
                + ((right_dual_g2_xyz * Simd32x3::from(self[scalar] * other[e12345]))
                    + (Simd32x3::from(wedge_g3[3]) * other.group1().xyz())
                    + (Simd32x3::from(right_dual_g2_w * other[e12345]) * self.group2().xyz())
                    + ((wedge_g3.zx() * other.group2().yz()) - (wedge_g3.yz() * other.group2().zx())).with_z(0.0))
                .with_w(0.0),
            // e4235, e4315, e4125, e3215
            wedge_g3 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Dipole> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       18       20        0        0
    //    simd3        0        6        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd       18       29        0      N/A
    //  no simd       18       50        0        0
    fn project_via_origin_onto(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        let wedge_g0 = right_dual_g0 * Simd32x3::from(self[scalar]);
        let wedge_g1 = right_dual_g1 * Simd32x4::from(self[scalar]);
        let wedge_g2_xyz = right_dual_g2 * Simd32x3::from(self[scalar]);
        let wedge_g2_w = -(right_dual_g0[0] * self[e15])
            - (right_dual_g0[1] * self[e25])
            - (right_dual_g0[2] * self[e35])
            - (right_dual_g2[0] * self[e41])
            - (right_dual_g2[1] * self[e42])
            - (right_dual_g2[2] * self[e43])
            - (right_dual_g1[0] * self[e23])
            - (right_dual_g1[1] * self[e31])
            - (right_dual_g1[2] * self[e12])
            - (right_dual_g1[3] * self[e45]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g2_w) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(wedge_g2_w) * other.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(wedge_g2_w) * other.group2()).with_w(
                -(wedge_g0[0] * other[e15])
                    - (wedge_g0[1] * other[e25])
                    - (wedge_g0[2] * other[e35])
                    - (wedge_g2_xyz[0] * other[e41])
                    - (wedge_g2_xyz[1] * other[e42])
                    - (wedge_g2_xyz[2] * other[e43])
                    - (wedge_g1[0] * other[e23])
                    - (wedge_g1[1] * other[e31])
                    - (wedge_g1[2] * other[e12])
                    - (wedge_g1[3] * other[e45]),
            ),
        )
    }
}
impl ProjectViaOriginOnto<DipoleInversion> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       15       25        0        0
    //    simd3       17       23        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       32       50        0      N/A
    //  no simd       66      102        0        0
    fn project_via_origin_onto(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (self.group0().zxy() * other.group3().yzx())
            - (Simd32x3::from(self[scalar]) * other.group0())
            - (self.group0().yzx() * other.group3().zxy());
        let wedge_g0_w = self[e45] * other[e45] * -1.0;
        let wedge_g1_xyz =
            (Simd32x3::from(self[e45]) * other.group3().xyz()) + (Simd32x3::from(other[e1234]) * self.group2().xyz()) + (Simd32x3::from(other[e3215]) * self.group0())
                - (Simd32x3::from(self[scalar]) * other.group1().xyz());
        let wedge_g1_w = self[scalar] * other[e45];
        let wedge_g2_xyz = Simd32x3::from([
            (self[e25] * other[e4125]) - (self[e35] * other[e4315]),
            (self[e35] * other[e4235]) - (self[e15] * other[e4125]),
            (self[e15] * other[e4315]) - (self[e25] * other[e4235]),
        ]) + (Simd32x3::from(other[e3215]) * self.group1().xyz())
            - (Simd32x3::from(self[scalar]) * other.group2().xyz());
        let wedge_g3 = Simd32x4::from(self[scalar]) * (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((wedge_g1_xyz * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0_w) * other.group0()) + (wedge_g0_xyz.zxy() * other.group3().yzx())
                - (wedge_g0_xyz.yzx() * other.group3().zxy()))
            .with_w(
                (wedge_g3[0] * other[e4235]) + (wedge_g3[1] * other[e4315]) + (self[scalar] * other[e1234] * other[e3215])
                    - (wedge_g1_w * other[e45])
                    - (wedge_g0_xyz[0] * other[e15])
                    - (wedge_g0_xyz[1] * other[e25])
                    - (wedge_g0_xyz[2] * other[e35])
                    - (wedge_g1_xyz[0] * other[e23])
                    - (wedge_g1_xyz[1] * other[e31])
                    - (wedge_g1_xyz[2] * other[e12])
                    - (wedge_g2_xyz[0] * other[e41])
                    - (wedge_g2_xyz[1] * other[e42])
                    - (wedge_g2_xyz[2] * other[e43]),
            ),
            // e23, e31, e12, e45
            ((wedge_g0_xyz * Simd32x3::from(other[e3215])) + (wedge_g2_xyz * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0_w) * other.group1().xyz())
                - (Simd32x3::from(wedge_g1_w) * other.group3().xyz()))
            .with_w(wedge_g0_w * other[e45]),
            // e15, e25, e35, e1234
            ((wedge_g1_xyz * Simd32x3::from(other[e3215])) + (Simd32x3::from(wedge_g0_w) * other.group2().xyz()) + (wedge_g2_xyz.yzx() * other.group3().zxy())
                - (wedge_g2_xyz.zxy() * other.group3().yzx()))
            .with_w(wedge_g0_w * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0_w) * other.group3(),
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        9        0        0
    //    simd3        0        4        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        1       17        0      N/A
    //  no simd        1       37        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(other[e12345] * -1.0) * self.group0().with_w(self[scalar]);
        let wedge_g2 = Simd32x4::from([1.0, 1.0, other[e12345], 0.0])
            * (self.group2().xyz() * Simd32x2::from(other[e12345] * -1.0).with_z(1.0) * Simd32x3::from([1.0, 1.0, -1.0])).with_w(0.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (wedge_g0.xyz() * Simd32x2::from(other[e12345]).with_z(other[e12345])).with_w((other[e5] * wedge_g2[3]) + (other[e12345] * wedge_g0[3])),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([1.0, 1.0, other[e12345], 0.0]) * (wedge_g2.xyz() * Simd32x2::from(other[e12345]).with_z(1.0)).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(other[e5] * other[e12345] * self[scalar] * -1.0),
        )
    }
}
impl ProjectViaOriginOnto<FlatPoint> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       14        0        0
    fn project_via_origin_onto(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let wedge_g2_w = (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]) - (self[e45] * other[e45]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(wedge_g2_w * other[e45]),
            // e15, e25, e35, scalar
            other.group0() * Simd32x3::from(wedge_g2_w).with_w(self[scalar] * other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl ProjectViaOriginOnto<Flector> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       18        0        0
    //    simd2        1        2        0      N/A
    //    simd3        6        9        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd       18       33        0      N/A
    //  no simd       40       65        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = ((self.group0().zxy() * other.group1().yzx()) - (self.group0().yzx() * other.group1().zxy())).with_w(self[e45] * other[e45] * -1.0);
        let wedge_g1 = (self.group1().wwwy() * other.group1().xyzy())
            + (Simd32x3::from(other[e3215]) * self.group0()).with_w((self[e23] * other[e4235]) + (self[e12] * other[e4125]) + (self[scalar] * other[e45]));
        let wedge_g2_xyz = Simd32x3::from([
            (self[e25] * other[e4125]) - (self[e35] * other[e4315]),
            (self[e35] * other[e4235]) - (self[e15] * other[e4125]),
            (self[e15] * other[e4315]) - (self[e25] * other[e4235]),
        ]) + (Simd32x3::from(other[e3215]) * self.group1().xyz())
            - (Simd32x3::from(self[scalar]) * other.group0().xyz());
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((wedge_g0.zx() * other.group1().yz()) - (wedge_g0.yz() * other.group1().zx())).with_zw(
                (wedge_g0[1] * other[e4235]) - (wedge_g0[0] * other[e4315]),
                -(wedge_g1[3] * other[e45]) - (other[e4235] * other[e4235] * self[scalar]),
            ),
            // e23, e31, e12, e45
            (wedge_g0 * Simd32x3::from(other[e3215]).with_w(other[e45])) + Simd32x3::from(0.0).with_w(-(wedge_g1[1] * other[e4315]) - (wedge_g1[2] * other[e4125]))
                - (wedge_g1.wwwx() * other.group1().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(wedge_g0[3]) * other.group0().xyz()) + (Simd32x3::from(other[e3215]) * wedge_g1.xyz()) + (wedge_g2_xyz.yzx() * other.group1().zxy())
                - (wedge_g2_xyz.zxy() * other.group1().yzx()))
            .with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0[3]) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Line> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        3        7        0      N/A
    // Totals...
    // yes simd        7       13        0      N/A
    //  no simd       13       27        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g2_w = -(self[e41] * other[e415]) - (self[e42] * other[e425]) - (self[e43] * other[e435]);
        let wedge_g3_xyz = (Simd32x3::from(self[e45]) * other.group0()) + (self.group0().yzx() * other.group1().zxy()) - (self.group0().zxy() * other.group1().yzx());
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g2_w) * other.group0(),
            // e23, e31, e12, e45
            (Simd32x3::from(wedge_g2_w) * other.group1()).with_w(-(wedge_g3_xyz[0] * other[e415]) - (wedge_g3_xyz[1] * other[e425]) - (wedge_g3_xyz[2] * other[e435])),
            // e15, e25, e35, scalar
            ((wedge_g3_xyz.zxy() * other.group1().yzx()) - (wedge_g3_xyz.yzx() * other.group1().zxy())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       16        0        0
    //    simd2        1        2        0      N/A
    //    simd3        6       10        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd       15       30        0      N/A
    //  no simd       31       58        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0 = Simd32x4::from(right_dual_g0_w) * self.group0().with_w(self[scalar]);
        let wedge_g1_xyz = (right_dual_g0_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_dual_g0_w) * self.group1().xyz());
        let wedge_g3 = ((right_dual_g0_xyz * Simd32x3::from(self[e45])) + (right_dual_g1_xyz.zxy() * self.group0().yzx()) - (right_dual_g1_xyz.yzx() * self.group0().zxy()))
            .with_w(self[scalar] * other[e5] * -1.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (wedge_g0.xyz() * Simd32x4::from(other[e12345]).xyz()).with_w(
                (wedge_g0[3] * other[e12345])
                    - (wedge_g1_xyz[0] * other[e415])
                    - (wedge_g1_xyz[1] * other[e425])
                    - (wedge_g1_xyz[2] * other[e435])
                    - (wedge_g0[0] * other[e235])
                    - (wedge_g0[1] * other[e315])
                    - (wedge_g0[2] * other[e125]),
            ),
            // e23, e31, e12, e45
            (wedge_g1_xyz * Simd32x4::from(other[e12345]).xyz()).with_w(right_dual_g0_w * self[e45] * other[e12345]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, (wedge_g3[1] * other[e235]) - (wedge_g3[0] * other[e315]), 0.0])
                + ((right_dual_g1_xyz * Simd32x3::from(self[scalar] * other[e12345]))
                    + (Simd32x3::from(wedge_g3[3]) * other.group0().xyz())
                    + (Simd32x3::from(right_dual_g0_w * other[e12345]) * self.group2().xyz())
                    + ((wedge_g3.zx() * other.group1().yz()) - (wedge_g3.yz() * other.group1().zx())).with_z(0.0))
                .with_w(0.0),
            // e4235, e4315, e4125, e3215
            wedge_g3 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       68       88        0        0
    //    simd2        0        2        0      N/A
    //    simd3       53       68        0      N/A
    //    simd4       10       13        0      N/A
    // Totals...
    // yes simd      131      171        0      N/A
    //  no simd      267      348        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1 = (other.group9().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let right_dual_g3 = other.group8().with_w(other[e321] * -1.0);
        let right_dual_g5 = other.group6().xyz();
        let right_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let wedge_g0_y = (right_dual_g0[1] * self[scalar])
            - (right_dual_g6_xyz[0] * self[e23])
            - (right_dual_g6_xyz[1] * self[e31])
            - (right_dual_g6_xyz[2] * self[e12])
            - (right_dual_g7[0] * self[e15])
            - (right_dual_g7[1] * self[e25])
            - (right_dual_g7[2] * self[e35])
            - (right_dual_g8[0] * self[e41])
            - (right_dual_g8[1] * self[e42])
            - (right_dual_g8[2] * self[e43])
            - (self[e45] * other[e45]);
        let wedge_g1 = right_dual_g1 * Simd32x4::from(self[scalar]);
        let wedge_g2 = self[scalar] * other[e3215];
        let wedge_g3 = (right_dual_g3 * Simd32x4::from(self[scalar])) + (Simd32x4::from(right_dual_g0[0]) * self.group2().xyz().with_w(self[e45]));
        let wedge_g4 = (Simd32x3::from(right_dual_g0[0]) * self.group0()) + (Simd32x3::from(self[scalar]) * other.group7());
        let wedge_g5 = (right_dual_g5 * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_dual_g0[0]) * self.group1().xyz());
        let wedge_g6 =
            ((right_dual_g6_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_dual_g1[3]) * self.group2().xyz()) + (Simd32x3::from(other[e3215]) * self.group0())
                - (Simd32x3::from(self[e45]) * right_dual_g1.xyz()))
            .with_w(self[scalar] * other[e45]);
        let wedge_g7 = (right_dual_g7 * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_dual_g1[3]) * self.group1().xyz()) + (self.group0().yzx() * right_dual_g1.zxy())
            - (self.group0().zxy() * right_dual_g1.yzx());
        let wedge_g8 = Simd32x3::from([
            (right_dual_g1[1] * self[e35]) - (right_dual_g1[2] * self[e25]),
            (right_dual_g1[2] * self[e15]) - (right_dual_g1[0] * self[e35]),
            (right_dual_g1[0] * self[e25]) - (right_dual_g1[1] * self[e15]),
        ]) + (right_dual_g8 * Simd32x3::from(self[scalar]))
            + (Simd32x3::from(other[e3215]) * self.group1().xyz());
        let wedge_g9 = ((right_dual_g5 * Simd32x3::from(self[e45]))
            + (Simd32x3::from(right_dual_g3[3]) * self.group1().xyz())
            + (Simd32x3::from(self[scalar]) * other.group1().xyz())
            + (self.group0().yzx() * right_dual_g3.zxy())
            + (other.group7().yzx() * self.group2().zxy())
            - (self.group0().zxy() * right_dual_g3.yzx())
            - (other.group7().zxy() * self.group2().yzx()))
        .with_w(self[scalar] * other[e5] * -1.0);
        let wedge_g10 = -(right_dual_g5[0] * self[e41])
            - (right_dual_g5[1] * self[e42])
            - (right_dual_g5[2] * self[e43])
            - (other[e423] * self[e23])
            - (other[e431] * self[e31])
            - (other[e412] * self[e12])
            - (self[scalar] * other[e4]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar])
                    + (wedge_g10 * other[e5])
                    + (wedge_g2 * other[e1234])
                    + (wedge_g1[0] * other[e4235])
                    + (wedge_g1[1] * other[e4315])
                    + (wedge_g1[2] * other[e4125])
                    + (wedge_g1[3] * other[e3215])
                    + (wedge_g9[0] * other[e1])
                    + (wedge_g9[1] * other[e2])
                    + (wedge_g9[2] * other[e3])
                    + (wedge_g9[3] * other[e4])
                    + (right_dual_g0[0] * other[e12345] * self[scalar])
                    - (wedge_g4[0] * other[e235])
                    - (wedge_g4[1] * other[e315])
                    - (wedge_g4[2] * other[e125])
                    - (wedge_g5[0] * other[e415])
                    - (wedge_g5[1] * other[e425])
                    - (wedge_g5[2] * other[e435])
                    - (wedge_g7[0] * other[e15])
                    - (wedge_g7[1] * other[e25])
                    - (wedge_g7[2] * other[e35])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43])
                    - (other[e23] * wedge_g6[0])
                    - (other[e31] * wedge_g6[1])
                    - (other[e12] * wedge_g6[2])
                    - (other[e423] * wedge_g3[0])
                    - (other[e431] * wedge_g3[1])
                    - (other[e412] * wedge_g3[2])
                    - (wedge_g3[3] * other[e321])
                    - (wedge_g6[3] * other[e45]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            (wedge_g1 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group1())
                + (other.group9().yzxx() * wedge_g5.zxy().with_w(wedge_g4[0]))
                + ((Simd32x3::from(wedge_g6[3]) * other.group6().xyz())
                    + (Simd32x3::from(wedge_g9[3]) * other.group4())
                    + (Simd32x3::from(other[e321]) * wedge_g6.xyz())
                    + (Simd32x3::from(other[e1234]) * wedge_g3.xyz())
                    + (wedge_g7.zxy() * other.group8().yzx())
                    + (wedge_g8.yzx() * other.group7().zxy())
                    + (other.group5().yzx() * wedge_g9.zxy())
                    - (wedge_g4 * Simd32x3::from(other[e3215]))
                    - (Simd32x3::from(wedge_g10) * other.group3().xyz())
                    - (wedge_g5.yzx() * other.group9().zxy())
                    - (wedge_g7.yzx() * other.group8().zxy())
                    - (wedge_g8.zxy() * other.group7().yzx())
                    - (other.group5().zxy() * wedge_g9.yzx()))
                .with_w((wedge_g4[1] * other[e4315]) + (wedge_g4[2] * other[e4125]) + (wedge_g3[3] * other[e1234])),
            // e5
            (wedge_g0_y * other[e5])
                + (wedge_g2 * other[e12345])
                + (wedge_g9[0] * other[e15])
                + (wedge_g9[1] * other[e25])
                + (wedge_g9[2] * other[e35])
                + (wedge_g9[3] * other[e45])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435])
                - (other[e235] * wedge_g6[0])
                - (other[e315] * wedge_g6[1])
                - (other[e125] * wedge_g6[2])
                - (wedge_g3[0] * other[e4235])
                - (wedge_g3[1] * other[e4315])
                - (wedge_g3[2] * other[e4125])
                - (wedge_g3[3] * other[e3215]),
            // e15, e25, e35, e45
            (wedge_g3 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group3())
                + ((Simd32x3::from(wedge_g9[3]) * other.group6().xyz())
                    + (Simd32x3::from(other[e3215]) * wedge_g6.xyz())
                    + (wedge_g8.yzx() * other.group9().zxy())
                    + (other.group8().yzx() * wedge_g9.zxy())
                    - (wedge_g8.zxy() * other.group9().yzx())
                    - (other.group8().zxy() * wedge_g9.yzx()))
                .with_w(0.0),
            // e41, e42, e43
            (wedge_g4 * Simd32x3::from(other[e12345]))
                + (Simd32x3::from(wedge_g0_y) * other.group4())
                + (Simd32x3::from(wedge_g10) * other.group6().xyz())
                + (Simd32x3::from(other[e1234]) * wedge_g6.xyz())
                + (wedge_g7.zxy() * other.group9().yzx())
                + (other.group7().zxy() * wedge_g9.yzx())
                - (wedge_g7.yzx() * other.group9().zxy())
                - (other.group7().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (wedge_g5 * Simd32x3::from(other[e12345]))
                + (wedge_g7 * Simd32x3::from(other[e3215]))
                + (wedge_g8 * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(wedge_g0_y) * other.group5())
                + (Simd32x3::from(wedge_g10) * other.group8())
                + (Simd32x3::from(wedge_g9[3]) * other.group7())
                - (Simd32x3::from(wedge_g6[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            (wedge_g6 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group6())
                + (wedge_g9.yzxw() * other.group9().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group9().yz()).with_zw(wedge_g9[1] * other[e4235] * -1.0, wedge_g10 * other[e3215] * -1.0),
            // e423, e431, e412
            (wedge_g7 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group7()) + (Simd32x3::from(other[e1234]) * wedge_g9.xyz())
                - (Simd32x3::from(wedge_g10) * other.group9().xyz()),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (wedge_g9 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group9()),
            // e1234
            (wedge_g0_y * other[e1234]) + (wedge_g10 * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       11        0        0
    //    simd2        1        2        0      N/A
    //    simd3        5        8        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd       11       21        0      N/A
    //  no simd       25       39        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (self.group0().zxy() * other.group0().yzx()) - (self.group0().yzx() * other.group0().zxy());
        let wedge_g2 = Simd32x4::from([0.0, 0.0, (self[e15] * other[e4315]) - (self[e25] * other[e4235]), 0.0])
            + ((Simd32x3::from(other[e3215]) * self.group1().xyz()) + ((self.group2().yz() * other.group0().zx()) - (self.group2().zx() * other.group0().yz())).with_z(0.0))
                .with_w(0.0);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (wedge_g0.zxy() * other.group0().yzx()) - (wedge_g0.yzx() * other.group0().zxy()),
            // e23, e31, e12, e45
            (wedge_g0 * Simd32x4::from(other[e3215]).xyz()).with_w(0.0),
            // e15, e25, e35, scalar
            (Simd32x3::from([
                (wedge_g2[1] * other[e4125]) - (wedge_g2[2] * other[e4315]),
                (wedge_g2[2] * other[e4235]) - (wedge_g2[0] * other[e4125]),
                (wedge_g2[0] * other[e4315]) - (wedge_g2[1] * other[e4235]),
            ]) + (Simd32x3::from(other[e3215] * other[e3215]) * self.group0())
                + (Simd32x3::from(self[e45] * other[e3215]) * other.group0().xyz()))
            .with_w(wedge_g2[3] * other[e3215]),
        )
    }
}
impl ProjectViaOriginOnto<RoundPoint> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        4       14        0        0
    fn project_via_origin_onto(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(
            // scalar
            (wedge_g0[0] * other[e1]) + (wedge_g0[1] * other[e2]) + (wedge_g0[2] * other[e3]) + (wedge_g0[3] * other[e4]) - (self[scalar] * other[e4] * other[e5]),
        )
    }
}
impl ProjectViaOriginOnto<Scalar> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn project_via_origin_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * other[scalar] * other[scalar])
    }
}
impl ProjectViaOriginOnto<Sphere> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd3       11       18        0      N/A
    // Totals...
    // yes simd       12       22        0      N/A
    //  no simd       34       58        0        0
    fn project_via_origin_onto(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (right_dual_g0_xyz.zxy() * self.group0().yzx()) - (right_dual_g0_xyz.yzx() * self.group0().zxy());
        let wedge_g1_xyz = (Simd32x3::from(other[e3215]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group2().xyz()) - (right_dual_g0_xyz * Simd32x3::from(self[e45]));
        let wedge_g2_xyz = (Simd32x3::from(other[e3215]) * self.group1().xyz()) + (right_dual_g0_xyz.yzx() * self.group2().zxy()) - (right_dual_g0_xyz.zxy() * self.group2().yzx());
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (wedge_g1_xyz * Simd32x3::from(other[e1234])) + (wedge_g0.zxy() * other.group0().yzx()) - (wedge_g0.yzx() * other.group0().zxy()),
            // e23, e31, e12, e45
            ((wedge_g0 * Simd32x3::from(other[e3215])) + (wedge_g2_xyz * Simd32x3::from(other[e1234]))).with_w(0.0),
            // e15, e25, e35, scalar
            ((wedge_g1_xyz * Simd32x3::from(other[e3215])) + (wedge_g2_xyz.yzx() * other.group0().zxy()) - (wedge_g2_xyz.zxy() * other.group0().yzx()))
                .with_w((right_dual_g0_xyz[0] * self[scalar] * other[e4235]) + (self[scalar] * other[e3215] * other[e1234])),
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       27        0        0
    //    simd2        0        1        0      N/A
    //    simd3       13       17        0      N/A
    //    simd4        4        6        0      N/A
    // Totals...
    // yes simd       29       51        0      N/A
    //  no simd       67      104        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let right_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        let right_dual_g2_xyz = other.group2().xyz();
        let wedge_g0 = ((right_dual_g0_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_dual_g0_w) * self.group0())).with_w(right_dual_g0_w * self[scalar]);
        let wedge_g1 = (right_dual_g1 * Simd32x4::from(self[scalar])) + (Simd32x4::from(right_dual_g0_w) * self.group1());
        let wedge_g2_xyz = (right_dual_g2_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_dual_g0_w) * self.group2().xyz());
        let wedge_g2_w = self[scalar] * other[e4] * -1.0;
        let wedge_g3 = ((Simd32x3::from(right_dual_g1[3]) * self.group1().xyz())
            + (Simd32x3::from(self[e45]) * right_dual_g1.xyz())
            + (Simd32x3::from(self[scalar]) * other.group3().xyz())
            + (right_dual_g0_xyz.yzx() * self.group2().zxy())
            + (right_dual_g2_xyz.zxy() * self.group0().yzx())
            - (right_dual_g0_xyz.zxy() * self.group2().yzx())
            - (right_dual_g2_xyz.yzx() * self.group0().zxy()))
        .with_w(self[scalar] * other[e5] * -1.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (wedge_g0 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g2_w) * other.group1().xyz().with_w(other[e5]))
                + (wedge_g3.yzxx() * other.group0().zxy().with_w(other[e1]))
                + -(wedge_g3.zx() * other.group0().yz()).with_zw(
                    wedge_g3[1] * other[e423] * -1.0,
                    -(wedge_g2_xyz[0] * other[e423])
                        - (wedge_g2_xyz[1] * other[e431])
                        - (wedge_g2_xyz[2] * other[e412])
                        - (wedge_g0[0] * other[e235])
                        - (wedge_g0[1] * other[e315])
                        - (wedge_g0[2] * other[e125])
                        - (wedge_g1[0] * other[e415])
                        - (wedge_g1[1] * other[e425])
                        - (wedge_g1[2] * other[e435])
                        - (wedge_g1[3] * other[e321]),
                ),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g2_w) * other.group2().xyz()) + (Simd32x3::from(wedge_g3[3]) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g1.xyz())
                - (Simd32x3::from(other[e321]) * wedge_g3.xyz()))
            .with_w(wedge_g1[3] * other[e12345]),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g3[2] * other[e315]) - (wedge_g3[1] * other[e125]),
                (wedge_g3[0] * other[e125]) - (wedge_g3[2] * other[e235]),
                (wedge_g3[1] * other[e235]) - (wedge_g3[0] * other[e315]),
            ]) + (wedge_g2_xyz * Simd32x3::from(other[e12345]))
                + (Simd32x3::from(wedge_g3[3]) * other.group1().xyz()))
            .with_w(wedge_g2_w * other[e12345]),
            // e4235, e4315, e4125, e3215
            wedge_g3 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<VersorOdd> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       18        0        0
    //    simd3       17       24        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd       29       44        0      N/A
    //  no simd       66       98        0        0
    fn project_via_origin_onto(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (right_dual_g3_xyz.zxy() * self.group0().yzx())
            - (Simd32x3::from(self[scalar]) * other.group0().xyz())
            - (right_dual_g3_xyz.yzx() * self.group0().zxy());
        let wedge_g0_w = self[scalar] * other[scalar];
        let wedge_g1_xyz = (Simd32x3::from(other[e1234]) * self.group2().xyz()) + (Simd32x3::from(other[e3215]) * self.group0())
            - (right_dual_g3_xyz * Simd32x3::from(self[e45]))
            - (Simd32x3::from(self[scalar]) * other.group1().xyz());
        let wedge_g1_w = self[scalar] * other[e45];
        let wedge_g2_xyz = (Simd32x3::from(other[e3215]) * self.group1().xyz()) + (right_dual_g3_xyz.yzx() * self.group2().zxy())
            - (Simd32x3::from(self[scalar]) * other.group2().xyz())
            - (right_dual_g3_xyz.zxy() * self.group2().yzx());
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from(wedge_g0_w) * other.group0())
                + ((wedge_g1_xyz * Simd32x3::from(other[e1234])) + (wedge_g0_xyz.zxy() * other.group3().yzx()) - (wedge_g0_xyz.yzx() * other.group3().zxy())).with_w(
                    (right_dual_g3_xyz[0] * self[scalar] * other[e4235]) + (self[scalar] * other[e1234] * other[e3215])
                        - (wedge_g1_w * other[e45])
                        - (wedge_g0_xyz[0] * other[e15])
                        - (wedge_g0_xyz[1] * other[e25])
                        - (wedge_g0_xyz[2] * other[e35])
                        - (wedge_g1_xyz[0] * other[e23])
                        - (wedge_g1_xyz[1] * other[e31])
                        - (wedge_g1_xyz[2] * other[e12])
                        - (wedge_g2_xyz[0] * other[e41])
                        - (wedge_g2_xyz[1] * other[e42])
                        - (wedge_g2_xyz[2] * other[e43]),
                ),
            // e23, e31, e12, e45
            ((wedge_g0_xyz * Simd32x3::from(other[e3215])) + (wedge_g2_xyz * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0_w) * other.group1().xyz())
                - (Simd32x3::from(wedge_g1_w) * other.group3().xyz()))
            .with_w(wedge_g0_w * other[e45]),
            // e15, e25, e35, e1234
            ((wedge_g1_xyz * Simd32x3::from(other[e3215])) + (Simd32x3::from(wedge_g0_w) * other.group2().xyz()) + (wedge_g2_xyz.yzx() * other.group3().zxy())
                - (wedge_g2_xyz.zxy() * other.group3().yzx()))
            .with_w(wedge_g0_w * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0_w) * other.group3(),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for AntiDipoleInversion {
    type Output = ProjectViaOriginOntoInfixPartial<AntiDipoleInversion>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiCircleRotor> for AntiDipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       12        0        0
    //    simd2        2        4        0      N/A
    //    simd3        7        8        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd       16       24        0      N/A
    //  no simd       38       44        0        0
    fn project_via_origin_onto(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0 = Simd32x4::from([0.0, 0.0, (other[e31] * self[e1]) - (other[e23] * self[e2]), 0.0])
            + (((other.group1().zx() * self.group3().yz()) - (other.group1().yz() * self.group3().zx())).with_z(0.0)
                - (right_dual_g0 * Simd32x3::from(self[e5]))
                - (Simd32x3::from(self[e4]) * other.group2().xyz()))
            .with_w(0.0);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, (wedge_g0[1] * other[e23]) - (wedge_g0[0] * other[e31]), 0.0])
                + ((Simd32x3::from(wedge_g0[3]) * other.group0()) + ((wedge_g0.zx() * other.group1().yz()) - (wedge_g0.yz() * other.group1().zx())).with_z(0.0)
                    - (Simd32x3::from(right_dual_g0[0] * self[e1]) * other.group2().xyz())
                    - (Simd32x3::from(right_dual_g0[1] * self[e2]) * other.group2().xyz())
                    - (Simd32x3::from(right_dual_g0[2] * self[e3]) * other.group2().xyz())
                    - (Simd32x3::from(other[e45] * self[e4]) * other.group2().xyz()))
                .with_w(0.0),
            // e5
            (wedge_g0[0] * other[e15]) + (wedge_g0[1] * other[e25]) + (wedge_g0[2] * other[e35]) + (wedge_g0[3] * other[e45]),
        )
    }
}
impl ProjectViaOriginOnto<AntiDipoleInversion> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       14        0        0
    //    simd3       11       17        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       22       32        0      N/A
    //  no simd       44       69        0        0
    fn project_via_origin_onto(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g2_xyz = other.group2().xyz();
        let wedge_g0 = (right_dual_g1_xyz * Simd32x3::from(self[e4])) + (other.group0().yzx() * self.group3().zxy()) - (other.group0().zxy() * self.group3().yzx());
        let wedge_g1_xyz = (right_dual_g2_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(other[e321]) * self.group3().xyz()) + (Simd32x3::from(self[e5]) * other.group0());
        let wedge_g2_xyz = (right_dual_g1_xyz * Simd32x3::from(self[e5])) + (right_dual_g2_xyz.zxy() * self.group3().yzx()) - (right_dual_g2_xyz.yzx() * self.group3().zxy());
        let wedge_g2_w = (other[e1] * self[e1])
            - (right_dual_g2_xyz[2] * self[e412])
            - (other[e423] * self[e235])
            - (other[e431] * self[e315])
            - (other[e412] * self[e125])
            - (other[e5] * self[e4]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(wedge_g2_w) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(wedge_g2_w) * other.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(wedge_g2_w) * other.group2().xyz()).with_w(
                (wedge_g2_w * other[e4])
                    - (wedge_g0[0] * other[e415])
                    - (wedge_g0[1] * other[e425])
                    - (wedge_g0[2] * other[e435])
                    - (wedge_g1_xyz[0] * other[e423])
                    - (wedge_g1_xyz[1] * other[e431])
                    - (wedge_g1_xyz[2] * other[e412]),
            ),
            // e1, e2, e3, e5
            ((wedge_g1_xyz * Simd32x3::from(other[e321]))
                + (Simd32x3::from(wedge_g2_w) * other.group3().xyz())
                + (wedge_g0.zxy() * other.group2().yzx())
                + (wedge_g2_xyz.yzx() * other.group0().zxy())
                - (wedge_g0.yzx() * other.group2().zxy())
                - (wedge_g2_xyz.zxy() * other.group0().yzx()))
            .with_w(wedge_g2_w * other[e5]),
        )
    }
}
impl ProjectViaOriginOnto<AntiDualNum> for AntiDipoleInversion {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        9        0      N/A
    //  no simd        0       16        0        0
    fn project_via_origin_onto(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([1.0, 1.0, other[e3215] * other[e3215], 0.0])
                * (self.group0() * Simd32x2::from(other[e3215] * other[e3215] * -1.0).with_z(1.0) * Simd32x3::from([1.0, 1.0, -1.0])).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(other[e3215] * other[e3215] * self[e4] * -1.0),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlatPoint> for AntiDipoleInversion {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        1        3        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3       10        0      N/A
    //  no simd        5       19        0        0
    fn project_via_origin_onto(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_w = other[e321] * -1.0;
        let wedge_g0_xyz = (Simd32x3::from(self[e4]) * other.group0().xyz()) - (Simd32x3::from(right_dual_g0_w) * self.group3().xyz());
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual_g0_w * self[e321] * -1.0) * other.group0(),
            // e1, e2, e3, e5
            (wedge_g0_xyz * Simd32x4::from(other[e321]).xyz()).with_w(-(wedge_g0_xyz[0] * other[e235]) - (wedge_g0_xyz[1] * other[e315]) - (wedge_g0_xyz[2] * other[e125])),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlector> for AntiDipoleInversion {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        1        2        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        3        8        0        0
    fn project_via_origin_onto(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            ((Simd32x3::from(other[e321] * other[e321]) * self.group3().xyz()) + (Simd32x3::from(self[e4] * other[e321]) * other.group0().xyz())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<AntiLine> for AntiDipoleInversion {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        3        6        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        9       19        0        0
    fn project_via_origin_onto(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = (right_dual_g0.yzx() * self.group3().zxy()) - (Simd32x3::from(self[e4]) * other.group1()) - (right_dual_g0.zxy() * self.group3().yzx());
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            ((wedge_g0_xyz.zxy() * other.group0().yzx()) - (wedge_g0_xyz.yzx() * other.group0().zxy())).with_w(wedge_g0_xyz[0] * other[e15]),
        )
    }
}
impl ProjectViaOriginOnto<AntiMotor> for AntiDipoleInversion {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        9        0        0
    //    simd3        3        6        0      N/A
    // Totals...
    // yes simd        7       15        0      N/A
    //  no simd       13       27        0        0
    fn project_via_origin_onto(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g1_xyz = Simd32x3::from([
            (self[e2] * other[e12]) - (self[e3] * other[e31]),
            (self[e3] * other[e23]) - (self[e1] * other[e12]),
            (self[e1] * other[e31]) - (self[e2] * other[e23]),
        ]) + (Simd32x3::from(other[e3215]) * self.group0())
            - (Simd32x3::from(self[e4]) * other.group1().xyz());
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (wedge_g1_xyz * Simd32x4::from(other[e3215]).xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e1, e2, e3, e5
            ((wedge_g1_xyz.zxy() * other.group0().yzx()) - (wedge_g1_xyz.yzx() * other.group0().zxy()))
                .with_w((wedge_g1_xyz[0] * other[e15]) - (other[e3215] * other[e3215] * self[e4])),
        )
    }
}
impl ProjectViaOriginOnto<AntiPlane> for AntiDipoleInversion {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        7        0        0
    //    simd3        0        1        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd        3       11        0      N/A
    //  no simd       12       22        0        0
    fn project_via_origin_onto(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x4::from(right_dual_g0_xyz[0] * self[e1]) * other.group0())
                + (Simd32x4::from(right_dual_g0_xyz[1] * self[e2]) * other.group0())
                + (Simd32x4::from(right_dual_g0_xyz[2] * self[e3]) * other.group0())
                + -(Simd32x3::from(self[e4] * other[e5]) * other.group0().xyz()).with_w(other[e5] * other[e5] * self[e4] * -1.0),
        )
    }
}
impl ProjectViaOriginOnto<AntiScalar> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        9        0      N/A
    //  no simd        0       20        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e12345] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual_g0 * other[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group3(),
        )
    }
}
impl ProjectViaOriginOnto<Circle> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3       10       14        0      N/A
    // Totals...
    // yes simd       15       20        0      N/A
    //  no simd       35       48        0        0
    fn project_via_origin_onto(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0 = (right_dual_g1_xyz * Simd32x3::from(self[e4])) + (other.group0().yzx() * self.group3().zxy()) - (other.group0().zxy() * self.group3().yzx());
        let wedge_g1_xyz = (Simd32x3::from(self[e4]) * other.group2()) + (Simd32x3::from(self[e5]) * other.group0()) + (Simd32x3::from(other[e321]) * self.group3().xyz());
        let wedge_g2_xyz = (right_dual_g1_xyz * Simd32x3::from(self[e5])) + (other.group2().zxy() * self.group3().yzx()) - (other.group2().yzx() * self.group3().zxy());
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(
                -(wedge_g0[0] * other[e415])
                    - (wedge_g0[1] * other[e425])
                    - (wedge_g0[2] * other[e435])
                    - (wedge_g1_xyz[0] * other[e423])
                    - (wedge_g1_xyz[1] * other[e431])
                    - (wedge_g1_xyz[2] * other[e412]),
            ),
            // e1, e2, e3, e5
            ((wedge_g1_xyz * Simd32x3::from(other[e321])) + (wedge_g0.zxy() * other.group2().yzx()) + (wedge_g2_xyz.yzx() * other.group0().zxy())
                - (wedge_g0.yzx() * other.group2().zxy())
                - (wedge_g2_xyz.zxy() * other.group0().yzx()))
            .with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<CircleRotor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        9        0        0
    //    simd3       17       23        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd       18       35        0      N/A
    //  no simd       55       90        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g1_w = other[e321] * -1.0;
        let right_dual_g2_xyz = other.group2().xyz();
        let right_dual_g2_w = other[e12345] * -1.0;
        let wedge_g0_xyz = (right_dual_g1_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_dual_g2_w) * self.group0()) + (other.group0().yzx() * self.group3().zxy())
            - (other.group0().zxy() * self.group3().yzx());
        let wedge_g0_w = right_dual_g1_w * self[e321] * -1.0;
        let wedge_g1 = ((right_dual_g2_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_dual_g2_w) * self.group1().xyz()) + (Simd32x3::from(self[e5]) * other.group0())
            - (Simd32x3::from(right_dual_g1_w) * self.group3().xyz()))
        .with_w(right_dual_g2_w * self[e321]);
        let wedge_g2_xyz =
            (right_dual_g1_xyz * Simd32x3::from(self[e5])) + (Simd32x3::from(right_dual_g2_w) * self.group2().xyz()) + (right_dual_g2_xyz.zxy() * self.group3().yzx())
                - (right_dual_g2_xyz.yzx() * self.group3().zxy());
        let wedge_g3 = Simd32x4::from(right_dual_g2_w) * self.group3().xyz().with_w(self[e4]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((wedge_g0_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_w) * other.group0())).with_w(wedge_g0_w * other[e12345]),
            // e415, e425, e435, e321
            (wedge_g1 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_w) * other.group1()),
            // e235, e315, e125, e5
            ((wedge_g2_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_w) * other.group2().xyz())).with_w(right_dual_g2_w * self[e5] * other[e12345]),
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g1[3]) * other.group1().xyz())
                + (Simd32x3::from(other[e321]) * wedge_g1.xyz())
                + (Simd32x3::from(other[e12345]) * wedge_g3.xyz())
                + (wedge_g0_xyz.zxy() * other.group2().yzx())
                + (wedge_g2_xyz.yzx() * other.group0().zxy())
                - (wedge_g0_xyz.yzx() * other.group2().zxy())
                - (wedge_g2_xyz.zxy() * other.group0().yzx()))
            .with_w(wedge_g3[3] * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Dipole> for AntiDipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       12        0        0
    //    simd2        2        4        0      N/A
    //    simd3        7        8        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd       16       24        0      N/A
    //  no simd       38       44        0        0
    fn project_via_origin_onto(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0 = Simd32x4::from([0.0, 0.0, (self[e1] * other[e31]) - (self[e2] * other[e23]), 0.0])
            + (((self.group3().yz() * other.group1().zx()) - (self.group3().zx() * other.group1().yz())).with_z(0.0)
                - (right_dual_g0 * Simd32x3::from(self[e5]))
                - (Simd32x3::from(self[e4]) * other.group2()))
            .with_w(0.0);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, (wedge_g0[1] * other[e23]) - (wedge_g0[0] * other[e31]), 0.0])
                + ((Simd32x3::from(wedge_g0[3]) * other.group0()) + ((wedge_g0.zx() * other.group1().yz()) - (wedge_g0.yz() * other.group1().zx())).with_z(0.0)
                    - (Simd32x3::from(right_dual_g0[0] * self[e1]) * other.group2())
                    - (Simd32x3::from(right_dual_g0[1] * self[e2]) * other.group2())
                    - (Simd32x3::from(right_dual_g0[2] * self[e3]) * other.group2())
                    - (Simd32x3::from(self[e4] * other[e45]) * other.group2()))
                .with_w(0.0),
            // e5
            (other[e15] * wedge_g0[0]) + (other[e25] * wedge_g0[1]) + (other[e35] * wedge_g0[2]) + (wedge_g0[3] * other[e45]),
        )
    }
}
impl ProjectViaOriginOnto<DipoleInversion> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       19        0        0
    //    simd2        2        4        0      N/A
    //    simd3        6       11        0      N/A
    //    simd4       12       14        0      N/A
    // Totals...
    // yes simd       33       48        0      N/A
    //  no simd       83      116        0        0
    fn project_via_origin_onto(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0 = -(Simd32x3::from(self[e4]) * other.group3().xyz()) - (Simd32x3::from(other[e1234]) * self.group3().xyz());
        let wedge_g1 = (other.group3().yzxw() * self.group3().zxy().with_w(self[e4])) - (self.group3().yzxw() * other.group3().zxy().with_w(other[e1234]));
        let wedge_g2 = (self.group3().xyzx() * Simd32x3::from(other[e3215]).with_w(right_dual_g0[0]))
            + (Simd32x3::from(self[e5]) * other.group3().xyz())
                .with_w((self[e423] * other[e4235]) + (self[e431] * other[e4315]) + (self[e412] * other[e4125]) - (self[e321] * other[e1234]));
        let wedge_g3 = (self.group3().yzxy() * other.group1().zxy().with_w(other[e25]))
            + ((Simd32x3::from(other[e3215]) * self.group0())
                + ((self.group1().yz() * other.group3().zx()) - (self.group3().zx() * other.group1().yz())).with_z((self[e415] * other[e4315]) - (self[e2] * other[e23]))
                - (right_dual_g0 * Simd32x3::from(self[e5]))
                - (Simd32x3::from(self[e4]) * other.group2().xyz()))
            .with_w((self[e321] * other[e3215]) + (self[e1] * other[e15]) - (self[e235] * other[e4235]) - (self[e5] * other[e45]))
            - (self.group2().xyzz() * Simd32x3::from(other[e1234]).with_w(other[e4125]))
            - (other.group3().yzxy() * self.group1().zxy().with_w(self[e315]));
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other[e1234]) * wedge_g3.xyz()) - (Simd32x3::from(wedge_g2[3]) * other.group3().xyz()),
            // e415, e425, e435, e321
            (wedge_g3.yzxw() * other.group3().zxy().with_w(other[e1234])) - (other.group3().yzxw() * wedge_g3.zxy().with_w(wedge_g2[3])),
            // e235, e315, e125, e4
            (Simd32x3::from(wedge_g3[3]) * other.group3().xyz())
                .with_w((wedge_g0[0] * other[e4235]) - (other[e42] * wedge_g3[1]) - (other[e43] * wedge_g3[2]) - (wedge_g2[3] * other[e45]))
                - (wedge_g3.xyzx() * Simd32x3::from(other[e3215]).with_w(other[e41])),
            // e1, e2, e3, e5
            (wedge_g3.zxyw() * other.group1().yzxw())
                + (wedge_g3.wwwx() * other.group0().with_w(other[e15]))
                + (other.group2().wwwz() * wedge_g2.xyz().with_w(wedge_g3[2]))
                + (((wedge_g1.zx() * other.group3().yz()) - (wedge_g3.yz() * other.group1().zx())).with_z((wedge_g1[1] * other[e4235]) - (wedge_g3[0] * other[e31]))
                    - (wedge_g0 * Simd32x3::from(other[e3215])))
                .with_w((wedge_g3[1] * other[e25]) - (wedge_g1[3] * other[e3215]) - (wedge_g2[2] * other[e4125]))
                - (wedge_g2.wwwy() * other.group2().xyz().with_w(other[e4315]))
                - (other.group3().zxyx() * wedge_g1.yzx().with_w(wedge_g2[0])),
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1       10        0        0
    //    simd3        0        3        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        1       17        0      N/A
    //  no simd        1       35        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (self.group0() * Simd32x2::from(other[e12345] * -1.0).with_z(other[e12345]) * Simd32x3::from([1.0, 1.0, -1.0])).with_w(other[e5] * self[e4] * -1.0);
        let wedge_g2 = Simd32x4::from(other[e12345] * -1.0) * self.group2().xyz().with_w(self[e5]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            wedge_g0 * Simd32x4::from(other[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group1(),
            // e235, e315, e125, e5
            (wedge_g2.xyz() * Simd32x2::from(other[e12345]).with_z(other[e12345])).with_w((other[e5] * wedge_g0[3]) + (other[e12345] * wedge_g2[3])),
            // e1, e2, e3, e4
            Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group3().xyz().with_w(self[e4]),
        )
    }
}
impl ProjectViaOriginOnto<FlatPoint> for AntiDipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       15        0        0
    //    simd3        0        4        0      N/A
    // Totals...
    // yes simd        6       19        0      N/A
    //  no simd        6       27        0        0
    fn project_via_origin_onto(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(self[e4] * -1.0) * other.group0().xyz();
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self[e4]).xyz() * Simd32x4::from(other[e45]).xyz() * other.group0().xyz() * Simd32x3::from(-1.0)).with_w(other[e45] * other[e45] * self[e4] * -1.0),
            // e5
            (wedge_g0_xyz[0] * other[e15])
                + (wedge_g0_xyz[1] * other[e25])
                + (wedge_g0_xyz[2] * other[e35])
                + (self[e1] * other[e15] * other[e45])
                + (self[e2] * other[e25] * other[e45])
                + (self[e3] * other[e35] * other[e45])
                - (other[e45] * other[e45] * self[e5]),
        )
    }
}
impl ProjectViaOriginOnto<Flector> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       15       30        0        0
    //    simd2        2        6        0      N/A
    //    simd3        3        7        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       24       46        0      N/A
    //  no simd       44       75        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[e4] * -1.0) * other.group1().xyz();
        let wedge_g1_xy = (self.group3().zx() * other.group1().yz()) - (self.group3().yz() * other.group1().zx());
        let wedge_g1_z = (self[e2] * other[e4235]) - (self[e1] * other[e4315]);
        let wedge_g2_xyz = (Simd32x3::from(self[e5]) * other.group1().xyz()) + (Simd32x3::from(other[e3215]) * self.group3().xyz());
        let wedge_g2_w = (self[e423] * other[e4235]) + (self[e431] * other[e4315]) + (self[e412] * other[e4125]) + (self[e4] * other[e45]);
        let wedge_g3 = ((Simd32x3::from(other[e3215]) * self.group0()) + (self.group1().yz() * other.group1().zx()).with_z(self[e415] * other[e4315]))
            .with_w((self[e321] * other[e3215]) + (self[e1] * other[e15]) - (self[e5] * other[e45]))
            - (self.group2().wwwy() * other.group0().xyz().with_w(other[e4315]))
            - (other.group1().yzxx() * self.group1().zxy().with_w(self[e235]));
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(wedge_g2_w * -1.0) * other.group1().xyz(),
            // e415, e425, e435, e321
            ((wedge_g3.yz() * other.group1().zx()) - (wedge_g3.zx() * other.group1().yz()))
                .with_zw((wedge_g3[0] * other[e4315]) - (wedge_g3[1] * other[e4235]), wedge_g2_w * other[e3215] * -1.0),
            // e235, e315, e125, e4
            ((Simd32x3::from(wedge_g3[3]) * other.group1().xyz()) - (Simd32x3::from(other[e3215]) * wedge_g3.xyz()))
                .with_w((wedge_g0[0] * other[e4235]) - (wedge_g2_w * other[e45])),
            // e1, e2, e3, e5
            Simd32x4::from([
                -(wedge_g2_w * other[e15]) - (wedge_g1_xy[1] * other[e4125]),
                -(wedge_g1_z * other[e4235]) - (wedge_g2_w * other[e25]),
                (wedge_g1_xy[1] * other[e4235]) - (wedge_g2_w * other[e35]) - (wedge_g1_xy[0] * other[e4315]),
                (wedge_g3[0] * other[e15]) - (wedge_g2_xyz[0] * other[e4235]) - (wedge_g2_xyz[1] * other[e4315]) - (other[e3215] * other[e3215] * self[e4]),
            ]) + (Simd32x2::from([wedge_g1_z, wedge_g1_xy[0]]) * other.group1().yz()).with_zw(0.0, 0.0)
                - (other.group1().wwwz() * wedge_g0.with_w(wedge_g2_xyz[2])),
        )
    }
}
impl ProjectViaOriginOnto<Line> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        4        9        0      N/A
    // Totals...
    // yes simd        6       12        0      N/A
    //  no simd       14       30        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[e4]) * other.group0();
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(-(wedge_g0[0] * other[e415]) - (wedge_g0[1] * other[e425]) - (wedge_g0[2] * other[e435])),
            // e1, e2, e3, e5
            ((wedge_g0.zxy() * other.group1().yzx())
                - (wedge_g0.yzx() * other.group1().zxy())
                - (other.group0() * other.group0() * self.group3().xyz())
                - (Simd32x3::from(other[e415]) * other.group0().yyz() * self.group3().yxx())
                - (Simd32x3::from(other[e435]) * other.group0().xyy() * self.group3().zzy()))
            .with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       19        0        0
    //    simd3        8       12        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd       18       35        0      N/A
    //  no simd       40       71        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0 = ((right_dual_g0_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_dual_g0_w) * self.group0())).with_w(self[e4] * other[e5] * -1.0);
        let wedge_g1_xyz = (right_dual_g1_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_dual_g0_w) * self.group1().xyz());
        let wedge_g1_w = right_dual_g0_w * self[e321];
        let wedge_g2 = ((right_dual_g0_xyz * Simd32x3::from(self[e5])) + (Simd32x3::from(right_dual_g0_w) * self.group2().xyz()) + (right_dual_g1_xyz.zxy() * self.group3().yzx())
            - (right_dual_g1_xyz.yzx() * self.group3().zxy()))
        .with_w(right_dual_g0_w * self[e5]);
        let wedge_g3 = Simd32x4::from(right_dual_g0_w) * self.group3().xyz().with_w(self[e4]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            wedge_g0 * Simd32x4::from(other[e12345]),
            // e415, e425, e435, e321
            ((wedge_g1_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0[3]) * other.group0().xyz())).with_w(wedge_g1_w * other[e12345]),
            // e235, e315, e125, e5
            (wedge_g2 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0[3]) * other.group1())
                + Simd32x3::from(0.0).with_w(
                    -(wedge_g1_xyz[0] * other[e235])
                        - (wedge_g1_xyz[1] * other[e315])
                        - (wedge_g1_xyz[2] * other[e125])
                        - (wedge_g2[0] * other[e415])
                        - (wedge_g2[1] * other[e425])
                        - (wedge_g2[2] * other[e435]),
                ),
            // e1, e2, e3, e4
            (Simd32x3::from([
                (wedge_g0[2] * other[e315]) - (wedge_g0[1] * other[e125]),
                (wedge_g0[0] * other[e125]) - (wedge_g0[2] * other[e235]),
                (wedge_g0[1] * other[e235]) - (wedge_g0[0] * other[e315]),
            ]) + (Simd32x3::from(wedge_g1_w) * other.group0().xyz())
                + (Simd32x3::from(other[e12345]) * wedge_g3.xyz()))
            .with_w(wedge_g3[3] * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       76       95        0        0
    //    simd2        1        4        0      N/A
    //    simd3       52       64        0      N/A
    //    simd4       11       14        0      N/A
    // Totals...
    // yes simd      140      177        0      N/A
    //  no simd      278      351        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1 = (other.group9().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let right_dual_g3_w = other[e321] * -1.0;
        let right_dual_g5 = other.group6().xyz();
        let right_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_dual_g9_xyz = other.group1().xyz();
        let wedge_g0_y = (right_dual_g9_xyz[0] * self[e1]) + (right_dual_g9_xyz[1] * self[e2]) + (right_dual_g9_xyz[2] * self[e3])
            - (right_dual_g3_w * self[e321])
            - (right_dual_g5[0] * self[e415])
            - (right_dual_g5[1] * self[e425])
            - (right_dual_g5[2] * self[e435])
            - (self[e423] * other[e235])
            - (self[e431] * other[e315])
            - (self[e412] * other[e125])
            - (other[e423] * self[e235])
            - (other[e431] * self[e315])
            - (other[e412] * self[e125])
            - (self[e4] * other[e5])
            - (self[e5] * other[e4]);
        let wedge_g1 = Simd32x4::from(right_dual_g0[0]) * self.group3().xyz().with_w(self[e4]);
        let wedge_g2 = right_dual_g0[0] * self[e5];
        let wedge_g3 = (Simd32x4::from(other[e3215]) * self.group3().xyz().with_w(self[e4])) - (right_dual_g1 * Simd32x4::from(self[e5]));
        let wedge_g4 = (Simd32x3::from(self[e4]) * right_dual_g1.xyz()) - (Simd32x3::from(right_dual_g1[3]) * self.group3().xyz());
        let wedge_g5 = Simd32x3::from([
            (right_dual_g1[2] * self[e2]) - (right_dual_g1[1] * self[e3]),
            (right_dual_g1[0] * self[e3]) - (right_dual_g1[2] * self[e1]),
            (right_dual_g1[1] * self[e1]) - (right_dual_g1[0] * self[e2]),
        ]);
        let wedge_g6 = ((Simd32x3::from(right_dual_g0[0]) * self.group1().xyz()) + (Simd32x3::from(self[e4]) * other.group8()) + (Simd32x3::from(self[e5]) * other.group7())
            - (Simd32x3::from(right_dual_g3_w) * self.group3().xyz()))
        .with_w(right_dual_g0[0] * self[e321]);
        let wedge_g7 = (right_dual_g5 * Simd32x3::from(self[e4])) + (Simd32x3::from(right_dual_g0[0]) * self.group0()) + (other.group7().yzx() * self.group3().zxy())
            - (other.group7().zxy() * self.group3().yzx());
        let wedge_g8 = (right_dual_g5 * Simd32x3::from(self[e5])) + (Simd32x3::from(right_dual_g0[0]) * self.group2().xyz()) + (other.group8().zxy() * self.group3().yzx())
            - (other.group8().yzx() * self.group3().zxy());
        let wedge_g9 = ((Simd32x3::from(self[e5]) * other.group4())
            + (Simd32x3::from(other[e3215]) * self.group0())
            + (right_dual_g6_xyz.yzx() * self.group3().zxy())
            + ((right_dual_g1.yz() * self.group1().zx()) - (right_dual_g1.zx() * self.group1().yz())).with_z((right_dual_g1[0] * self[e425]) - (right_dual_g1[1] * self[e415]))
            - (Simd32x3::from(right_dual_g1[3]) * self.group2().xyz())
            - (Simd32x3::from(self[e4]) * other.group3().xyz()))
        .with_w((right_dual_g1[0] * self[e235]) + (right_dual_g1[1] * self[e315]) + (right_dual_g1[2] * self[e125]) + (self[e321] * other[e3215]))
            - (self.group3().yzxw() * right_dual_g6_xyz.zxy().with_w(other[e45]));
        let wedge_g10 = (self[e4] * other[e45])
            - (self[e423] * right_dual_g1[0])
            - (self[e431] * right_dual_g1[1])
            - (self[e412] * right_dual_g1[2])
            - (other[e41] * self[e1])
            - (other[e42] * self[e2])
            - (other[e43] * self[e3])
            - (right_dual_g1[3] * self[e321]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar])
                    + (wedge_g10 * other[e5])
                    + (wedge_g2 * other[e1234])
                    + (wedge_g1[0] * other[e4235])
                    + (wedge_g1[1] * other[e4315])
                    + (wedge_g1[2] * other[e4125])
                    + (wedge_g1[3] * other[e3215])
                    + (wedge_g9[0] * other[e1])
                    + (wedge_g9[1] * other[e2])
                    + (wedge_g9[2] * other[e3])
                    + (wedge_g9[3] * other[e4])
                    - (wedge_g4[0] * other[e235])
                    - (wedge_g4[1] * other[e315])
                    - (wedge_g4[2] * other[e125])
                    - (wedge_g5[0] * other[e415])
                    - (wedge_g5[1] * other[e425])
                    - (wedge_g5[2] * other[e435])
                    - (wedge_g7[0] * other[e15])
                    - (wedge_g7[1] * other[e25])
                    - (wedge_g7[2] * other[e35])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43])
                    - (other[e23] * wedge_g6[0])
                    - (other[e31] * wedge_g6[1])
                    - (other[e12] * wedge_g6[2])
                    - (other[e423] * wedge_g3[0])
                    - (other[e431] * wedge_g3[1])
                    - (other[e412] * wedge_g3[2])
                    - (wedge_g3[3] * other[e321])
                    - (wedge_g6[3] * other[e45]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            (wedge_g1 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group1())
                + (other.group9().yzxx() * wedge_g5.zxy().with_w(wedge_g4[0]))
                + ((Simd32x3::from(wedge_g6[3]) * other.group6().xyz())
                    + (Simd32x3::from(wedge_g9[3]) * other.group4())
                    + (Simd32x3::from(other[e321]) * wedge_g6.xyz())
                    + (Simd32x3::from(other[e1234]) * wedge_g3.xyz())
                    + (wedge_g7.zxy() * other.group8().yzx())
                    + (wedge_g8.yzx() * other.group7().zxy())
                    + (other.group5().yzx() * wedge_g9.zxy())
                    - (wedge_g4 * Simd32x3::from(other[e3215]))
                    - (Simd32x3::from(wedge_g10) * other.group3().xyz())
                    - (wedge_g5.yzx() * other.group9().zxy())
                    - (wedge_g7.yzx() * other.group8().zxy())
                    - (wedge_g8.zxy() * other.group7().yzx())
                    - (other.group5().zxy() * wedge_g9.yzx()))
                .with_w((wedge_g4[1] * other[e4315]) + (wedge_g4[2] * other[e4125]) + (wedge_g3[3] * other[e1234])),
            // e5
            (wedge_g0_y * other[e5])
                + (wedge_g2 * other[e12345])
                + (wedge_g9[0] * other[e15])
                + (wedge_g9[1] * other[e25])
                + (wedge_g9[2] * other[e35])
                + (wedge_g9[3] * other[e45])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435])
                - (other[e235] * wedge_g6[0])
                - (other[e315] * wedge_g6[1])
                - (other[e125] * wedge_g6[2])
                - (wedge_g3[0] * other[e4235])
                - (wedge_g3[1] * other[e4315])
                - (wedge_g3[2] * other[e4125])
                - (wedge_g3[3] * other[e3215]),
            // e15, e25, e35, e45
            (wedge_g3 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group3())
                + ((Simd32x3::from(wedge_g9[3]) * other.group6().xyz())
                    + (Simd32x3::from(other[e3215]) * wedge_g6.xyz())
                    + (wedge_g8.yzx() * other.group9().zxy())
                    + (other.group8().yzx() * wedge_g9.zxy())
                    - (wedge_g8.zxy() * other.group9().yzx())
                    - (other.group8().zxy() * wedge_g9.yzx()))
                .with_w(0.0),
            // e41, e42, e43
            (wedge_g4 * Simd32x3::from(other[e12345]))
                + (Simd32x3::from(wedge_g0_y) * other.group4())
                + (Simd32x3::from(wedge_g10) * other.group6().xyz())
                + (Simd32x3::from(other[e1234]) * wedge_g6.xyz())
                + (wedge_g7.zxy() * other.group9().yzx())
                + (other.group7().zxy() * wedge_g9.yzx())
                - (wedge_g7.yzx() * other.group9().zxy())
                - (other.group7().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (wedge_g5 * Simd32x3::from(other[e12345]))
                + (wedge_g7 * Simd32x3::from(other[e3215]))
                + (wedge_g8 * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(wedge_g0_y) * other.group5())
                + (Simd32x3::from(wedge_g10) * other.group8())
                + (Simd32x3::from(wedge_g9[3]) * other.group7())
                - (Simd32x3::from(wedge_g6[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            (wedge_g6 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group6())
                + (wedge_g9.yzxw() * other.group9().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group9().yz()).with_zw(wedge_g9[1] * other[e4235] * -1.0, wedge_g10 * other[e3215] * -1.0),
            // e423, e431, e412
            (wedge_g7 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group7()) + (Simd32x3::from(other[e1234]) * wedge_g9.xyz())
                - (Simd32x3::from(wedge_g10) * other.group9().xyz()),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (wedge_g9 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group9()),
            // e1234
            (wedge_g0_y * other[e1234]) + (wedge_g10 * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       13        0        0
    //    simd2        2        4        0      N/A
    //    simd3        4        7        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd       12       24        0      N/A
    //  no simd       25       42        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[e4] * -1.0) * other.group0().xyz();
        let wedge_g1_xy = (self.group3().zx() * other.group0().yz()) - (self.group3().yz() * other.group0().zx());
        let wedge_g1_z = (self[e2] * other[e4235]) - (self[e1] * other[e4315]);
        let wedge_g3_xyz = Simd32x3::from([
            (self[e425] * other[e4125]) - (self[e435] * other[e4315]),
            (self[e435] * other[e4235]) - (self[e415] * other[e4125]),
            (self[e415] * other[e4315]) - (self[e425] * other[e4235]),
        ]) + (Simd32x3::from(other[e3215]) * self.group0());
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            ((wedge_g3_xyz.yzx() * other.group0().zxy()) - (wedge_g3_xyz.zxy() * other.group0().yzx())).with_w(0.0),
            // e235, e315, e125, e4
            ((Simd32x3::from(self[e321] * other[e3215]) * other.group0().xyz()) - (wedge_g3_xyz * Simd32x3::from(other[e3215]))).with_w(wedge_g0[0] * other[e4235]),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, (wedge_g1_xy[1] * other[e4235]) - (wedge_g1_xy[0] * other[e4315]), 0.0])
                + (((Simd32x2::from([wedge_g1_z, wedge_g1_xy[0]]) * other.group0().yz()) - (Simd32x2::from([wedge_g1_xy[1], wedge_g1_z]) * other.group0().zx())).with_z(0.0)
                    - (wedge_g0 * Simd32x3::from(other[e3215])))
                .with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<RoundPoint> for AntiDipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd        4       10        0        0
    fn project_via_origin_onto(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let wedge_g0 = (right_dual_g0_xyz[0] * self[e1]) + (right_dual_g0_xyz[1] * self[e2]) + (right_dual_g0_xyz[2] * self[e3]) - (self[e4] * other[e5]) - (self[e5] * other[e4]);
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(wedge_g0) * other.group0(), /* e5 */ wedge_g0 * other[e5])
    }
}
impl ProjectViaOriginOnto<Sphere> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2       11        0        0
    //    simd2        1        2        0      N/A
    //    simd3        9       16        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd       15       31        0      N/A
    //  no simd       43       71        0        0
    fn project_via_origin_onto(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = (right_dual_g0_xyz * Simd32x3::from(self[e4])) - (Simd32x3::from(other[e1234]) * self.group3().xyz());
        let wedge_g1 = (right_dual_g0_xyz.zxy() * self.group3().yzx()).with_w(self[e4] * other[e3215]) - (self.group3().zxyw() * right_dual_g0_xyz.yzx().with_w(other[e1234]));
        let wedge_g2_w = self[e321] * other[e1234] * -1.0;
        let wedge_g3_xyz = (Simd32x3::from(other[e3215]) * self.group0()) + (right_dual_g0_xyz.yzx() * self.group1().zxy())
            - (Simd32x3::from(other[e1234]) * self.group2().xyz())
            - (right_dual_g0_xyz.zxy() * self.group1().yzx());
        let wedge_g3_w = (right_dual_g0_xyz[0] * self[e235]) + (self[e321] * other[e3215]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (wedge_g3_xyz * Simd32x3::from(other[e1234])) - (Simd32x3::from(wedge_g2_w) * other.group0().xyz()),
            // e415, e425, e435, e321
            (wedge_g3_xyz.yzx() * other.group0().zxy()).with_w(wedge_g3_w * other[e1234]) - (other.group0().yzxw() * wedge_g3_xyz.zxy().with_w(wedge_g2_w)),
            // e235, e315, e125, e4
            ((Simd32x3::from(wedge_g3_w) * other.group0().xyz()) - (wedge_g3_xyz * Simd32x3::from(other[e3215]))).with_w(wedge_g0[0] * other[e4235]),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, (wedge_g1[1] * other[e4235]) - (wedge_g1[0] * other[e4315]), 0.0])
                + ((Simd32x3::from(other[e3215] * other[e1234]) * self.group3().xyz())
                    + ((wedge_g1.zx() * other.group0().yz()) - (wedge_g1.yz() * other.group0().zx())).with_z(0.0)
                    - (right_dual_g0_xyz * Simd32x3::from(self[e5] * other[e1234]))
                    - (wedge_g0 * Simd32x3::from(other[e3215])))
                .with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       18       27        0        0
    //    simd2        2        3        0      N/A
    //    simd3       13       17        0      N/A
    //    simd4        5        7        0      N/A
    // Totals...
    // yes simd       38       54        0      N/A
    //  no simd       81      112        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g1_w = other[e321] * -1.0;
        let right_dual_g2_xyz = other.group2().xyz();
        let right_dual_g3_xyz = other.group3().xyz();
        let wedge_g0 = (self.group3().zxyx() * right_dual_g0_xyz.yzx().with_w(right_dual_g3_xyz[0]))
            + ((right_dual_g1_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_dual_g0_w) * self.group0()) - (right_dual_g0_xyz.zxy() * self.group3().yzx())).with_w(
                (right_dual_g3_xyz[1] * self[e2])
                    - (right_dual_g1_w * self[e321])
                    - (right_dual_g0_xyz[0] * self[e235])
                    - (right_dual_g0_xyz[1] * self[e315])
                    - (right_dual_g0_xyz[2] * self[e125])
                    - (right_dual_g1_xyz[0] * self[e415])
                    - (right_dual_g1_xyz[1] * self[e425])
                    - (right_dual_g1_xyz[2] * self[e435])
                    - (right_dual_g2_xyz[0] * self[e423])
                    - (right_dual_g2_xyz[1] * self[e431])
                    - (right_dual_g2_xyz[2] * self[e412])
                    - (self[e4] * other[e5]),
            );
        let wedge_g1 = ((right_dual_g0_xyz * Simd32x3::from(self[e5])) + (right_dual_g2_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_dual_g0_w) * self.group1().xyz())
            - (Simd32x3::from(right_dual_g1_w) * self.group3().xyz()))
        .with_w(right_dual_g0_w * self[e321]);
        let wedge_g2 = ((right_dual_g1_xyz * Simd32x3::from(self[e5])) + (Simd32x3::from(right_dual_g0_w) * self.group2().xyz()) + (right_dual_g2_xyz.zxy() * self.group3().yzx())
            - (right_dual_g2_xyz.yzx() * self.group3().zxy()))
        .with_w(right_dual_g0_w * self[e5]);
        let wedge_g3 = Simd32x4::from(right_dual_g0_w) * self.group3().xyz().with_w(self[e4]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x3::from(wedge_g0[3]) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g0.xyz())).with_w(wedge_g0[3] * other[e12345]),
            // e415, e425, e435, e321
            (wedge_g1 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0[3]) * other.group1()),
            // e235, e315, e125, e5
            (wedge_g2 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0[3]) * other.group2())
                + Simd32x3::from(0.0).with_w(
                    -(wedge_g1[0] * other[e235])
                        - (wedge_g1[1] * other[e315])
                        - (wedge_g1[2] * other[e125])
                        - (wedge_g2[0] * other[e415])
                        - (wedge_g2[1] * other[e425])
                        - (wedge_g2[2] * other[e435]),
                ),
            // e1, e2, e3, e4
            (wedge_g0.zxyw() * other.group2().yzx().with_w(other[e4]))
                + ((Simd32x3::from(wedge_g0[3]) * other.group3().xyz())
                    + (Simd32x3::from(wedge_g1[3]) * other.group1().xyz())
                    + (Simd32x3::from(other[e12345]) * wedge_g3.xyz())
                    + (Simd32x3::from(other[e321]) * wedge_g1.xyz())
                    + ((wedge_g2.yz() * other.group0().zx()) - (wedge_g0.yz() * other.group2().zx()) - (wedge_g2.zx() * other.group0().yz()))
                        .with_z((wedge_g2[0] * other[e431]) - (wedge_g0[0] * other[e315]) - (wedge_g2[1] * other[e423])))
                .with_w(wedge_g3[3] * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<VersorOdd> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       16       26        0        0
    //    simd2        0        1        0      N/A
    //    simd3       12       19        0      N/A
    //    simd4        7        7        0      N/A
    // Totals...
    // yes simd       35       53        0      N/A
    //  no simd       80      113        0        0
    fn project_via_origin_onto(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = (right_dual_g3_xyz * Simd32x3::from(self[e4])) - (Simd32x3::from(other[e1234]) * self.group3().xyz());
        let wedge_g1 = (right_dual_g3_xyz.zxy() * self.group3().yzx()).with_w(self[e4] * other[e3215]) - (self.group3().zxyw() * right_dual_g3_xyz.yzx().with_w(other[e1234]));
        let wedge_g2_xyz = (Simd32x3::from(other[e3215]) * self.group3().xyz()) - (right_dual_g3_xyz * Simd32x3::from(self[e5]));
        let wedge_g2_w =
            (self[e4] * other[e45]) - (right_dual_g3_xyz[0] * self[e423]) - (right_dual_g3_xyz[1] * self[e431]) - (right_dual_g3_xyz[2] * self[e412]) - (self[e321] * other[e1234]);
        let wedge_g3_xyz = Simd32x3::from([
            (self[e2] * other[e12]) - (self[e3] * other[e31]),
            (self[e3] * other[e23]) - (self[e1] * other[e12]),
            (self[e1] * other[e31]) - (self[e2] * other[e23]),
        ]) + (Simd32x3::from(self[e5]) * other.group0().xyz())
            + (Simd32x3::from(other[e3215]) * self.group0())
            + (right_dual_g3_xyz.yzx() * self.group1().zxy())
            - (Simd32x3::from(self[e4]) * other.group2().xyz())
            - (Simd32x3::from(other[e1234]) * self.group2().xyz())
            - (right_dual_g3_xyz.zxy() * self.group1().yzx());
        let wedge_g3_w = (right_dual_g3_xyz[0] * self[e235]) + (right_dual_g3_xyz[1] * self[e315]) + (right_dual_g3_xyz[2] * self[e125]) + (self[e321] * other[e3215]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (wedge_g3_xyz * Simd32x3::from(other[e1234])) - (Simd32x3::from(wedge_g2_w) * other.group3().xyz()),
            // e415, e425, e435, e321
            (wedge_g3_xyz.yzx() * other.group3().zxy()).with_w(wedge_g3_w * other[e1234]) - (other.group3().yzxw() * wedge_g3_xyz.zxy().with_w(wedge_g2_w)),
            // e235, e315, e125, e4
            (other.group3().xyzx() * Simd32x3::from(wedge_g3_w).with_w(wedge_g0[0]))
                + -(wedge_g3_xyz * Simd32x3::from(other[e3215]))
                    .with_w(-(wedge_g2_w * other[e45]) - (wedge_g3_xyz[0] * other[e41]) - (wedge_g3_xyz[1] * other[e42]) - (wedge_g3_xyz[2] * other[e43])),
            // e1, e2, e3, e5
            (Simd32x4::from(wedge_g3_w) * other.group0().xyz().with_w(other[e45]))
                + (other.group2().wwwx() * wedge_g2_xyz.with_w(wedge_g3_xyz[0]))
                + ((wedge_g3_xyz.zxy() * other.group1().yzx()) + (wedge_g1.zx() * other.group3().yz()).with_z(wedge_g1[1] * other[e4235])
                    - (Simd32x3::from(wedge_g2_w) * other.group2().xyz())
                    - (wedge_g3_xyz.yzx() * other.group1().zxy()))
                .with_w((wedge_g3_xyz[1] * other[e25]) + (wedge_g3_xyz[2] * other[e35]) - (wedge_g2_xyz[0] * other[e4235]) - (wedge_g2_xyz[2] * other[e4125]))
                - (wedge_g1.yzxw() * other.group3().zxyw())
                - (other.group3().wwwy() * wedge_g0.with_w(wedge_g2_xyz[1])),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for AntiDualNum {
    type Output = ProjectViaOriginOntoInfixPartial<AntiDualNum>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiCircleRotor> for AntiDualNum {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       12        0        0
    //    simd3        0        3        0      N/A
    //    simd4        0        5        0      N/A
    // Totals...
    // yes simd       10       20        0      N/A
    //  no simd       10       41        0        0
    fn project_via_origin_onto(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[scalar] * -1.0) * other.group0();
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g2 = Simd32x4::from(self[scalar]) * other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g2[3]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(wedge_g2[3]) * other.group1(),
            // e15, e25, e35, scalar
            (Simd32x4::from(wedge_g2[3]).xyz() * other.group2().xyz()).with_w(
                (wedge_g2[3] * other[scalar])
                    - (wedge_g0[0] * other[e15])
                    - (wedge_g0[1] * other[e25])
                    - (wedge_g0[2] * other[e35])
                    - (other[e41] * wedge_g2[0])
                    - (other[e42] * wedge_g2[1])
                    - (other[e43] * wedge_g2[2])
                    - (wedge_g1[0] * other[e23])
                    - (wedge_g1[1] * other[e31])
                    - (wedge_g1[2] * other[e12])
                    - (wedge_g1[3] * other[e45]),
            ),
        )
    }
}
impl ProjectViaOriginOnto<AntiDipoleInversion> for AntiDualNum {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       12        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4        7        0      N/A
    //    simd4        2        8        0      N/A
    // Totals...
    // yes simd       15       28        0      N/A
    //  no simd       29       67        0        0
    fn project_via_origin_onto(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[scalar]) * other.group0();
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let wedge_g2 = Simd32x4::from(self[scalar]) * other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let wedge_g3 = Simd32x4::from(self[scalar]) * other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(wedge_g2[3]) * other.group1().xyz()) + (other.group0().zxy() * wedge_g3.yzx()) - (other.group0().yzx() * wedge_g3.zxy()),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g2[3]) * other.group2().xyz()) + (Simd32x3::from(wedge_g3[3]) * other.group0()) - (Simd32x3::from(other[e321]) * wedge_g3.xyz())).with_w(0.0),
            // e15, e25, e35, scalar
            (Simd32x4::from(wedge_g3[3]) * other.group1().xyz().with_w(other[e4]))
                + (wedge_g3.zxyx() * other.group2().yzx().with_w(other[e1]))
                + -(wedge_g3.yz() * other.group2().zx()).with_zw(
                    wedge_g3[0] * other[e315] * -1.0,
                    -(wedge_g0[0] * other[e235])
                        - (wedge_g0[1] * other[e315])
                        - (wedge_g0[2] * other[e125])
                        - (other[e423] * wedge_g2[0])
                        - (other[e431] * wedge_g2[1])
                        - (other[e412] * wedge_g2[2])
                        - (wedge_g1[0] * other[e415])
                        - (wedge_g1[1] * other[e425])
                        - (wedge_g1[2] * other[e435])
                        - (wedge_g1[3] * other[e321]),
                ),
        )
    }
}
impl ProjectViaOriginOnto<AntiDualNum> for AntiDualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        3        0        0
    fn project_via_origin_onto(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(other[scalar] * self[scalar]) * other.group0())
    }
}
impl ProjectViaOriginOnto<AntiFlatPoint> for AntiDualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn project_via_origin_onto(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * other[e321] * other[e321])
    }
}
impl ProjectViaOriginOnto<AntiFlector> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        7        0        0
    //    simd2        1        2        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd        6       13        0      N/A
    //  no simd       10       25        0        0
    fn project_via_origin_onto(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (wedge_g1.xyz() * Simd32x4::from(other[e321]).xyz() * Simd32x3::from(-1.0))
                .with_w((wedge_g1[0] * other[e1]) + (wedge_g1[1] * other[e2]) + (wedge_g1[2] * other[e3]) + (other[e321] * other[e321] * self[scalar])),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, (wedge_g1[1] * other[e235]) - (wedge_g1[0] * other[e315]), 0.0])
                + ((wedge_g1.zx() * other.group0().yz()) - (wedge_g1.yz() * other.group0().zx())).with_zw(0.0, 0.0),
        )
    }
}
impl ProjectViaOriginOnto<AntiLine> for AntiDualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        7        0        0
    fn project_via_origin_onto(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[scalar] * -1.0) * other.group0();
        Scalar::from_groups(/* scalar */ -(wedge_g0[0] * other[e23]) - (wedge_g0[1] * other[e31]) - (wedge_g0[2] * other[e12]))
    }
}
impl ProjectViaOriginOnto<AntiMotor> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        1        3        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd        6       22        0        0
    fn project_via_origin_onto(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x4::from(wedge_g0[3]).xyz() * other.group0().xyz())
                .with_w((wedge_g0[3] * other[scalar]) - (wedge_g0[0] * other[e23]) - (wedge_g0[1] * other[e31]) - (wedge_g0[2] * other[e12])),
            // e15, e25, e35, e3215
            ((Simd32x3::from(wedge_g0[3]) * other.group1().xyz()) + (Simd32x3::from(other[e3215]) * wedge_g0.xyz())).with_w(wedge_g0[3] * other[e3215]),
        )
    }
}
impl ProjectViaOriginOnto<AntiPlane> for AntiDualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2       11        0        0
    fn project_via_origin_onto(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(/* scalar */ (wedge_g0[0] * other[e1]) + (wedge_g0[1] * other[e2]) + (wedge_g0[2] * other[e3]))
    }
}
impl ProjectViaOriginOnto<AntiScalar> for AntiDualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        4        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(other[e12345] * other[e12345] * -1.0) * self.group0())
    }
}
impl ProjectViaOriginOnto<Circle> for AntiDualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       10        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd        9       24        0        0
    fn project_via_origin_onto(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[scalar]) * other.group0();
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let wedge_g2 = Simd32x3::from(self[scalar]) * other.group2();
        Scalar::from_groups(
            // scalar
            -(wedge_g0[0] * other[e235])
                - (wedge_g0[1] * other[e315])
                - (wedge_g0[2] * other[e125])
                - (wedge_g2[0] * other[e423])
                - (wedge_g2[1] * other[e431])
                - (wedge_g2[2] * other[e412])
                - (wedge_g1[0] * other[e415])
                - (wedge_g1[1] * other[e425])
                - (wedge_g1[2] * other[e435])
                - (wedge_g1[3] * other[e321]),
        )
    }
}
impl ProjectViaOriginOnto<CircleRotor> for AntiDualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd3        4        8        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        4       17        0      N/A
    //  no simd       12       45        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2_w = other[e12345] * -1.0;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0().with_w(right_dual_g2_w);
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let wedge_g2 = Simd32x4::from([1.0, 1.0, self[scalar], 0.0]) * (other.group2().xyz() * Simd32x2::from(self[scalar]).with_z(1.0)).with_w(0.0);
        let wedge_g3_w = right_dual_g2_w * self[e3215];
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x3::from(wedge_g2[3]) * other.group1().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g0.xyz())).with_w(wedge_g0[3] * other[e12345]),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g3_w) * other.group0()) + (Simd32x3::from(wedge_g2[3]) * other.group2().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g1.xyz()))
                .with_w(wedge_g1[3] * other[e12345]),
            // e15, e25, e35, e1234
            ((Simd32x3::from(wedge_g3_w) * other.group1().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g2.xyz())).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(wedge_g3_w * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Dipole> for AntiDualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       12        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        9       16        0      N/A
    //  no simd        9       26        0        0
    fn project_via_origin_onto(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[scalar] * -1.0) * other.group0();
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g2 = Simd32x3::from(self[scalar] * -1.0) * other.group2();
        Scalar::from_groups(
            // scalar
            -(wedge_g0[0] * other[e15])
                - (wedge_g0[1] * other[e25])
                - (wedge_g0[2] * other[e35])
                - (wedge_g2[0] * other[e41])
                - (wedge_g2[1] * other[e42])
                - (wedge_g2[2] * other[e43])
                - (wedge_g1[0] * other[e23])
                - (wedge_g1[1] * other[e31])
                - (wedge_g1[2] * other[e12])
                - (wedge_g1[3] * other[e45]),
        )
    }
}
impl ProjectViaOriginOnto<DipoleInversion> for AntiDualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       15       23        0        0
    //    simd3        8       13        0      N/A
    //    simd4        0        5        0      N/A
    // Totals...
    // yes simd       23       41        0      N/A
    //  no simd       39       82        0        0
    fn project_via_origin_onto(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(self[scalar] * -1.0) * other.group0();
        let wedge_g0_w = self[e3215] * other[e1234];
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g2 = Simd32x4::from(self[scalar]) * (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(other[e3215]);
        let wedge_g3 = Simd32x4::from(self[scalar]) * (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x3::from(wedge_g0_w) * other.group0()) + (Simd32x3::from(other[e1234]) * wedge_g1.xyz()) + (wedge_g0_xyz.zxy() * other.group3().yzx())
                - (wedge_g0_xyz.yzx() * other.group3().zxy()))
            .with_w(
                (wedge_g2[3] * other[e1234]) + (wedge_g3[0] * other[e4235]) + (wedge_g3[1] * other[e4315])
                    - (wedge_g0_xyz[0] * other[e15])
                    - (wedge_g0_xyz[1] * other[e25])
                    - (wedge_g0_xyz[2] * other[e35])
                    - (other[e41] * wedge_g2[0])
                    - (other[e42] * wedge_g2[1])
                    - (other[e43] * wedge_g2[2])
                    - (wedge_g1[0] * other[e23])
                    - (wedge_g1[1] * other[e31])
                    - (wedge_g1[2] * other[e12])
                    - (wedge_g1[3] * other[e45]),
            ),
            // e23, e31, e12, e45
            ((wedge_g0_xyz * Simd32x3::from(other[e3215])) + (Simd32x3::from(wedge_g0_w) * other.group1().xyz()) + (Simd32x3::from(other[e1234]) * wedge_g2.xyz())
                - (Simd32x3::from(wedge_g1[3]) * other.group3().xyz()))
            .with_w(wedge_g0_w * other[e45]),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g2[1] * other[e4125]) - (wedge_g2[2] * other[e4315]),
                (wedge_g2[2] * other[e4235]) - (wedge_g2[0] * other[e4125]),
                (wedge_g2[0] * other[e4315]) - (wedge_g2[1] * other[e4235]),
            ]) + (Simd32x3::from(wedge_g0_w) * other.group2().xyz())
                + (Simd32x3::from(other[e3215]) * wedge_g1.xyz()))
            .with_w(wedge_g0_w * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0_w) * other.group3(),
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for AntiDualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        7        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            -(other[e12345] * other[e12345] * self[e3215]) - (self[scalar] * other[e5] * other[e12345]),
            other[e12345] * other[e12345] * self[scalar] * -1.0,
        ]))
    }
}
impl ProjectViaOriginOnto<FlatPoint> for AntiDualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn project_via_origin_onto(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * other[e45] * other[e45] * -1.0)
    }
}
impl ProjectViaOriginOnto<Flector> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd2        1        2        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        1        4        0      N/A
    // Totals...
    // yes simd        6       14        0      N/A
    //  no simd       10       32        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x4::from(wedge_g0[3]).xyz() * other.group1().xyz() * Simd32x3::from(-1.0))
                .with_w((wedge_g1[0] * other[e4235]) + (wedge_g1[1] * other[e4315]) + (wedge_g1[2] * other[e4125]) - (wedge_g0[3] * other[e45])),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, (wedge_g0[0] * other[e4315]) - (wedge_g0[1] * other[e4235]), 0.0])
                + ((wedge_g0.yz() * other.group1().zx()) - (wedge_g0.zx() * other.group1().yz())).with_zw(0.0, 0.0),
        )
    }
}
impl ProjectViaOriginOnto<Line> for AntiDualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[scalar]) * other.group0();
        Scalar::from_groups(/* scalar */ -(wedge_g0[0] * other[e415]) - (wedge_g0[1] * other[e425]) - (wedge_g0[2] * other[e435]))
    }
}
impl ProjectViaOriginOnto<Motor> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        9        0        0
    //    simd3        1        3        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        5       13        0      N/A
    //  no simd        7       22        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e12345] * -1.0);
        let wedge_g0 = right_dual_g0 * Simd32x4::from(self[scalar]);
        let wedge_g1_w = (self[e3215] * right_dual_g0[3]) - (self[scalar] * other[e5]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (wedge_g0.xyz() * Simd32x4::from(other[e12345]).xyz())
                .with_w((wedge_g0[3] * other[e12345]) - (wedge_g0[0] * other[e415]) - (wedge_g0[1] * other[e425]) - (wedge_g0[2] * other[e435])),
            // e15, e25, e35, e3215
            ((Simd32x3::from(wedge_g1_w) * other.group0().xyz()) + (Simd32x3::from(self[scalar] * other[e12345]) * other.group1().xyz())).with_w(wedge_g1_w * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for AntiDualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       51       68        0        0
    //    simd2        0        2        0      N/A
    //    simd3       37       50        0      N/A
    //    simd4        9       14        0      N/A
    // Totals...
    // yes simd       97      134        0      N/A
    //  no simd      198      278        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1 = (other.group9().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let wedge_g0_y = (right_dual_g0[1] * self[scalar]) + (self[e3215] * right_dual_g1[3]);
        let wedge_g1 = right_dual_g1 * Simd32x4::from(self[scalar]);
        let wedge_g2 = self[scalar] * other[e3215];
        let wedge_g3 = Simd32x4::from(self[scalar]) * other.group8().with_w(other[e321]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let wedge_g4 = Simd32x3::from(self[scalar]) * other.group7();
        let wedge_g5 = Simd32x3::from(self[scalar]) * other.group6().xyz();
        let wedge_g6 = Simd32x4::from(self[scalar]) * (other.group5() * Simd32x3::from(-1.0)).with_w(other[e45]);
        let wedge_g7 = Simd32x3::from(self[scalar] * -1.0) * other.group4();
        let wedge_g8 = Simd32x3::from(self[scalar] * -1.0) * other.group3().xyz();
        let wedge_g9 = (other.group1().xyz() * Simd32x2::from(self[scalar]).with_z(self[scalar])).with_w((right_dual_g0[0] * self[e3215]) - (self[scalar] * other[e5]));
        let wedge_g10 = self[scalar] * other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar])
                    + (wedge_g10 * other[e5])
                    + (wedge_g2 * other[e1234])
                    + (wedge_g1[0] * other[e4235])
                    + (wedge_g1[1] * other[e4315])
                    + (wedge_g1[2] * other[e4125])
                    + (wedge_g1[3] * other[e3215])
                    + (wedge_g9[0] * other[e1])
                    + (wedge_g9[1] * other[e2])
                    + (wedge_g9[2] * other[e3])
                    + (wedge_g9[3] * other[e4])
                    + (right_dual_g0[0] * self[scalar] * other[e12345])
                    - (wedge_g4[0] * other[e235])
                    - (wedge_g4[1] * other[e315])
                    - (wedge_g4[2] * other[e125])
                    - (wedge_g5[0] * other[e415])
                    - (wedge_g5[1] * other[e425])
                    - (wedge_g5[2] * other[e435])
                    - (wedge_g7[0] * other[e15])
                    - (wedge_g7[1] * other[e25])
                    - (wedge_g7[2] * other[e35])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43])
                    - (other[e23] * wedge_g6[0])
                    - (other[e31] * wedge_g6[1])
                    - (other[e12] * wedge_g6[2])
                    - (other[e423] * wedge_g3[0])
                    - (other[e431] * wedge_g3[1])
                    - (other[e412] * wedge_g3[2])
                    - (wedge_g3[3] * other[e321])
                    - (wedge_g6[3] * other[e45]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            (wedge_g1 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group1())
                + (other.group9().yzxx() * wedge_g5.zxy().with_w(wedge_g4[0]))
                + ((Simd32x3::from(wedge_g6[3]) * other.group6().xyz())
                    + (Simd32x3::from(wedge_g9[3]) * other.group4())
                    + (Simd32x3::from(other[e321]) * wedge_g6.xyz())
                    + (Simd32x3::from(other[e1234]) * wedge_g3.xyz())
                    + (wedge_g7.zxy() * other.group8().yzx())
                    + (wedge_g8.yzx() * other.group7().zxy())
                    + (other.group5().yzx() * wedge_g9.zxy())
                    - (wedge_g4 * Simd32x3::from(other[e3215]))
                    - (Simd32x3::from(wedge_g10) * other.group3().xyz())
                    - (wedge_g5.yzx() * other.group9().zxy())
                    - (wedge_g7.yzx() * other.group8().zxy())
                    - (wedge_g8.zxy() * other.group7().yzx())
                    - (other.group5().zxy() * wedge_g9.yzx()))
                .with_w((wedge_g4[1] * other[e4315]) + (wedge_g4[2] * other[e4125]) + (wedge_g3[3] * other[e1234])),
            // e5
            (wedge_g0_y * other[e5])
                + (wedge_g2 * other[e12345])
                + (wedge_g9[0] * other[e15])
                + (wedge_g9[1] * other[e25])
                + (wedge_g9[2] * other[e35])
                + (wedge_g9[3] * other[e45])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435])
                - (other[e235] * wedge_g6[0])
                - (other[e315] * wedge_g6[1])
                - (other[e125] * wedge_g6[2])
                - (wedge_g3[0] * other[e4235])
                - (wedge_g3[1] * other[e4315])
                - (wedge_g3[2] * other[e4125])
                - (wedge_g3[3] * other[e3215]),
            // e15, e25, e35, e45
            (wedge_g3 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group3())
                + ((Simd32x3::from(wedge_g9[3]) * other.group6().xyz())
                    + (Simd32x3::from(other[e3215]) * wedge_g6.xyz())
                    + (wedge_g8.yzx() * other.group9().zxy())
                    + (other.group8().yzx() * wedge_g9.zxy())
                    - (wedge_g8.zxy() * other.group9().yzx())
                    - (other.group8().zxy() * wedge_g9.yzx()))
                .with_w(0.0),
            // e41, e42, e43
            (wedge_g4 * Simd32x3::from(other[e12345]))
                + (Simd32x3::from(wedge_g0_y) * other.group4())
                + (Simd32x3::from(wedge_g10) * other.group6().xyz())
                + (Simd32x3::from(other[e1234]) * wedge_g6.xyz())
                + (wedge_g7.zxy() * other.group9().yzx())
                + (other.group7().zxy() * wedge_g9.yzx())
                - (wedge_g7.yzx() * other.group9().zxy())
                - (other.group7().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (wedge_g5 * Simd32x3::from(other[e12345]))
                + (wedge_g7 * Simd32x3::from(other[e3215]))
                + (wedge_g8 * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(wedge_g0_y) * other.group5())
                + (Simd32x3::from(wedge_g10) * other.group8())
                + (Simd32x3::from(wedge_g9[3]) * other.group7())
                - (Simd32x3::from(wedge_g6[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            (wedge_g6 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group6())
                + (wedge_g9.yzxw() * other.group9().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group9().yz()).with_zw(wedge_g9[1] * other[e4235] * -1.0, wedge_g10 * other[e3215] * -1.0),
            // e423, e431, e412
            (wedge_g7 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group7()) + (Simd32x3::from(other[e1234]) * wedge_g9.xyz())
                - (Simd32x3::from(wedge_g10) * other.group9().xyz()),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (wedge_g9 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group9()),
            // e1234
            (wedge_g0_y * other[e1234]) + (wedge_g10 * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for AntiDualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2       11        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Scalar::from_groups(/* scalar */ (wedge_g0[0] * other[e4235]) + (wedge_g0[1] * other[e4315]) + (wedge_g0[2] * other[e4125]))
    }
}
impl ProjectViaOriginOnto<RoundPoint> for AntiDualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        4       14        0        0
    fn project_via_origin_onto(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(
            // scalar
            (wedge_g0[0] * other[e1]) + (wedge_g0[1] * other[e2]) + (wedge_g0[2] * other[e3]) + (wedge_g0[3] * other[e4]) - (self[scalar] * other[e4] * other[e5]),
        )
    }
}
impl ProjectViaOriginOnto<Scalar> for AntiDualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn project_via_origin_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * other[scalar] * other[scalar])
    }
}
impl ProjectViaOriginOnto<Sphere> for AntiDualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        4       11        0      N/A
    //  no simd        4       19        0        0
    fn project_via_origin_onto(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let wedge_g0_w = self[e3215] * right_dual_g0[3];
        let wedge_g3 = right_dual_g0 * Simd32x4::from(self[scalar]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(
                (wedge_g3[0] * other[e4235])
                    + (wedge_g3[1] * other[e4315])
                    + (wedge_g3[2] * other[e4125])
                    + (wedge_g3[3] * other[e3215])
                    + (self[scalar] * other[e3215] * other[e1234]),
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(wedge_g0_w * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0_w) * other.group0(),
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for AntiDualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       23        0        0
    //    simd2        0        1        0      N/A
    //    simd3        5        7        0      N/A
    //    simd4        3        9        0      N/A
    // Totals...
    // yes simd       21       40        0      N/A
    //  no simd       40       82        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e12345] * -1.0);
        let wedge_g0 = right_dual_g0 * Simd32x4::from(self[scalar]);
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let wedge_g2 = Simd32x4::from(self[scalar]) * other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let wedge_g3 = (other.group3().xyz() * Simd32x2::from(self[scalar]).with_z(self[scalar])).with_w((self[e3215] * right_dual_g0[3]) - (self[scalar] * other[e5]));
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (wedge_g0 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g2[3]) * other.group1().xyz().with_w(other[e5]))
                + (wedge_g3.yzxx() * other.group0().zxy().with_w(other[e1]))
                + -(wedge_g3.zx() * other.group0().yz()).with_zw(
                    wedge_g3[1] * other[e423] * -1.0,
                    -(wedge_g0[0] * other[e235])
                        - (wedge_g0[1] * other[e315])
                        - (wedge_g0[2] * other[e125])
                        - (wedge_g1[0] * other[e415])
                        - (wedge_g1[1] * other[e425])
                        - (wedge_g1[2] * other[e435])
                        - (wedge_g1[3] * other[e321])
                        - (wedge_g2[0] * other[e423])
                        - (wedge_g2[1] * other[e431])
                        - (wedge_g2[2] * other[e412]),
                ),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g2[3]) * other.group2().xyz()) + (Simd32x3::from(wedge_g3[3]) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g1.xyz())
                - (Simd32x3::from(other[e321]) * wedge_g3.xyz()))
            .with_w(wedge_g1[3] * other[e12345]),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g3[2] * other[e315]) - (wedge_g3[1] * other[e125]),
                (wedge_g3[0] * other[e125]) - (wedge_g3[2] * other[e235]),
                (wedge_g3[1] * other[e235]) - (wedge_g3[0] * other[e315]),
            ]) + (Simd32x3::from(wedge_g3[3]) * other.group1().xyz())
                + (Simd32x3::from(other[e12345]) * wedge_g2.xyz()))
            .with_w(wedge_g2[3] * other[e12345]),
            // e4235, e4315, e4125, e3215
            wedge_g3 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<VersorOdd> for AntiDualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       20        0        0
    //    simd3        7       11        0      N/A
    //    simd4        1        5        0      N/A
    // Totals...
    // yes simd       20       36        0      N/A
    //  no simd       37       73        0        0
    fn project_via_origin_onto(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(self[scalar] * -1.0) * other.group0().xyz();
        let wedge_g0_w = (self[e3215] * other[e1234]) + (self[scalar] * other[scalar]);
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g2 = Simd32x4::from(self[scalar]) * (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(other[e3215]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from(wedge_g0_w) * other.group0())
                + ((Simd32x3::from(other[e1234]) * wedge_g1.xyz()) + (wedge_g0_xyz.zxy() * other.group3().yzx()) - (wedge_g0_xyz.yzx() * other.group3().zxy())).with_w(
                    (wedge_g2[3] * other[e1234])
                        - (wedge_g0_xyz[2] * other[e35])
                        - (wedge_g1[0] * other[e23])
                        - (wedge_g1[1] * other[e31])
                        - (wedge_g1[2] * other[e12])
                        - (wedge_g1[3] * other[e45])
                        - (wedge_g2[0] * other[e41])
                        - (wedge_g2[1] * other[e42])
                        - (wedge_g2[2] * other[e43]),
                ),
            // e23, e31, e12, e45
            ((wedge_g0_xyz * Simd32x3::from(other[e3215])) + (Simd32x3::from(wedge_g0_w) * other.group1().xyz()) + (Simd32x3::from(other[e1234]) * wedge_g2.xyz())
                - (Simd32x3::from(wedge_g1[3]) * other.group3().xyz()))
            .with_w(wedge_g0_w * other[e45]),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g2[1] * other[e4125]) - (wedge_g2[2] * other[e4315]),
                (wedge_g2[2] * other[e4235]) - (wedge_g2[0] * other[e4125]),
                (wedge_g2[0] * other[e4315]) - (wedge_g2[1] * other[e4235]),
            ]) + (Simd32x3::from(wedge_g0_w) * other.group2().xyz())
                + (Simd32x3::from(other[e3215]) * wedge_g1.xyz()))
            .with_w(wedge_g0_w * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0_w) * other.group3(),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for AntiFlatPoint {
    type Output = ProjectViaOriginOntoInfixPartial<AntiFlatPoint>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiDipoleInversion> for AntiFlatPoint {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       19        0        0
    fn project_via_origin_onto(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (other[e321] * self[e321]) - (other[e423] * self[e235]) - (other[e431] * self[e315]) - (other[e412] * self[e125]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(wedge_g0) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(wedge_g0) * other.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(wedge_g0) * other.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(wedge_g0) * other.group3(),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlatPoint> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn project_via_origin_onto(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other[e321] * self[e321]) * other.group0())
    }
}
impl ProjectViaOriginOnto<AntiFlector> for AntiFlatPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        9        0        0
    fn project_via_origin_onto(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = self[e321] * other[e321];
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(wedge_g0) * other.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(wedge_g0) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<AntiScalar> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group0())
    }
}
impl ProjectViaOriginOnto<Circle> for AntiFlatPoint {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       14        0        0
    fn project_via_origin_onto(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (self[e321] * other[e321]) - (other[e423] * self[e235]) - (other[e431] * self[e315]) - (other[e412] * self[e125]);
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(wedge_g0) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(wedge_g0) * other.group1(),
            // e235, e315, e125
            Simd32x3::from(wedge_g0) * other.group2(),
        )
    }
}
impl ProjectViaOriginOnto<CircleRotor> for AntiFlatPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        4        7        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd        8       16        0      N/A
    //  no simd       19       39        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2_w = other[e12345] * -1.0;
        let wedge_g1 = Simd32x3::from(0.0).with_w(right_dual_g2_w * self[e321]);
        let wedge_g2_xyz = Simd32x3::from(right_dual_g2_w) * self.group0().xyz();
        let wedge_g2_w = (self[e321] * other[e321]) - (other[e423] * self[e235]) - (other[e431] * self[e315]) - (other[e412] * self[e125]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(wedge_g2_w) * other.group0().with_w(other[e12345]),
            // e415, e425, e435, e321
            (wedge_g1 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g2_w) * other.group1()),
            // e235, e315, e125, e5
            ((wedge_g2_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g2_w) * other.group2().xyz())).with_w(0.0),
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g1[3]) * other.group1().xyz()) + (Simd32x3::from(other[e321]) * wedge_g1.xyz()) + (wedge_g2_xyz.yzx() * other.group0().zxy())
                - (wedge_g2_xyz.zxy() * other.group0().yzx()))
            .with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<DipoleInversion> for AntiFlatPoint {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       11        0        0
    //    simd3        4        9        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        9       22        0      N/A
    //  no simd       23       46        0        0
    fn project_via_origin_onto(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(other[e1234] * -1.0) * self.group0().xyz();
        let wedge_g0_w = (self[e321] * other[e3215]) - (self[e235] * other[e4235]) - (self[e315] * other[e4315]) - (self[e125] * other[e4125]);
        let wedge_g1 = self[e321] * other[e1234] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (wedge_g0_xyz * Simd32x3::from(other[e1234])) - (Simd32x3::from(wedge_g1) * other.group3().xyz()),
            // e415, e425, e435, e321
            (wedge_g0_xyz.yzx() * other.group3().zxy()).with_w(wedge_g0_w * other[e1234]) - (other.group3().yzxw() * wedge_g0_xyz.zxy().with_w(wedge_g1)),
            // e235, e315, e125, e4
            ((Simd32x3::from(wedge_g0_w) * other.group3().xyz()) - (wedge_g0_xyz * Simd32x3::from(other[e3215]))).with_w(wedge_g1 * other[e45] * -1.0),
            // e1, e2, e3, e5
            (Simd32x4::from(wedge_g0_w) * other.group0().with_w(other[e45]))
                + ((wedge_g0_xyz.zxy() * other.group1().yzx()) - (Simd32x3::from(wedge_g1) * other.group2().xyz()) - (wedge_g0_xyz.yzx() * other.group1().zxy()))
                    .with_w(wedge_g0_xyz[0] * other[e15]),
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group0())
    }
}
impl ProjectViaOriginOnto<Flector> for AntiFlatPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3        8        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_x = (self[e321] * other[e3215]) - (self[e235] * other[e4235]) - (self[e315] * other[e4315]) - (self[e125] * other[e4125]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (Simd32x3::from(wedge_g0_x) * other.group1().xyz()).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(wedge_g0_x * other[e45]),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for AntiFlatPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        2       15        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(other[e12345] * -1.0) * self.group0();
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            wedge_g0 * Simd32x4::from(other[e12345]),
            // e1, e2, e3, e5
            (Simd32x4::from(wedge_g0[3]).xyz() * other.group0().xyz()).with_w(-(wedge_g0[0] * other[e415]) - (wedge_g0[1] * other[e425]) - (wedge_g0[2] * other[e435])),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for AntiFlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       29       45        0        0
    //    simd2        0        2        0      N/A
    //    simd3       28       37        0      N/A
    //    simd4        4        5        0      N/A
    // Totals...
    // yes simd       61       89        0      N/A
    //  no simd      129      180        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let wedge_g0_y = (self[e321] * other[e321]) - (other[e423] * self[e235]) - (other[e431] * self[e315]) - (other[e412] * self[e125]);
        let wedge_g6 = Simd32x3::from(0.0).with_w(right_dual_g0[0] * self[e321]);
        let wedge_g8 = Simd32x3::from(right_dual_g0[0]) * self.group0().xyz();
        let wedge_g9 = (Simd32x3::from(other[e1234] * -1.0) * self.group0().xyz())
            .with_w((right_dual_g1_xyz[0] * self[e235]) + (right_dual_g1_xyz[1] * self[e315]) + (right_dual_g1_xyz[2] * self[e125]) + (self[e321] * other[e3215]));
        let wedge_g10 = self[e321] * other[e1234] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar])
                    + (wedge_g10 * other[e5])
                    + (wedge_g9[0] * other[e1])
                    + (wedge_g9[1] * other[e2])
                    + (wedge_g9[2] * other[e3])
                    + (wedge_g9[3] * other[e4])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43])
                    - (other[e23] * wedge_g6[0])
                    - (other[e31] * wedge_g6[1])
                    - (other[e12] * wedge_g6[2])
                    - (wedge_g6[3] * other[e45]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g0_y) * other.group1().xyz())
                + (Simd32x3::from(wedge_g6[3]) * other.group6().xyz())
                + (Simd32x3::from(wedge_g9[3]) * other.group4())
                + (Simd32x3::from(other[e321]) * wedge_g6.xyz())
                + (wedge_g8.yzx() * other.group7().zxy())
                + (other.group5().yzx() * wedge_g9.zxy())
                - (Simd32x3::from(wedge_g10) * other.group3().xyz())
                - (wedge_g8.zxy() * other.group7().yzx())
                - (other.group5().zxy() * wedge_g9.yzx()))
            .with_w(wedge_g0_y * other[e4]),
            // e5
            (wedge_g0_y * other[e5]) + (wedge_g9[0] * other[e15]) + (wedge_g9[1] * other[e25]) + (wedge_g9[2] * other[e35]) + (wedge_g9[3] * other[e45])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435])
                - (other[e235] * wedge_g6[0])
                - (other[e315] * wedge_g6[1])
                - (other[e125] * wedge_g6[2]),
            // e15, e25, e35, e45
            ((Simd32x3::from(wedge_g0_y) * other.group3().xyz())
                + (Simd32x3::from(wedge_g9[3]) * other.group6().xyz())
                + (Simd32x3::from(other[e3215]) * wedge_g6.xyz())
                + (wedge_g8.yzx() * other.group9().zxy())
                + (other.group8().yzx() * wedge_g9.zxy())
                - (wedge_g8.zxy() * other.group9().yzx())
                - (other.group8().zxy() * wedge_g9.yzx()))
            .with_w(wedge_g0_y * other[e45]),
            // e41, e42, e43
            (Simd32x3::from(wedge_g0_y) * other.group4())
                + (Simd32x3::from(wedge_g10) * other.group6().xyz())
                + (Simd32x3::from(other[e1234]) * wedge_g6.xyz())
                + (other.group7().zxy() * wedge_g9.yzx())
                - (other.group7().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (wedge_g8 * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(wedge_g0_y) * other.group5())
                + (Simd32x3::from(wedge_g10) * other.group8())
                + (Simd32x3::from(wedge_g9[3]) * other.group7())
                - (Simd32x3::from(wedge_g6[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            (wedge_g6 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group6())
                + (wedge_g9.yzxw() * other.group9().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group9().yz()).with_zw(wedge_g9[1] * other[e4235] * -1.0, wedge_g10 * other[e3215] * -1.0),
            // e423, e431, e412
            (Simd32x3::from(wedge_g0_y) * other.group7()) + (Simd32x3::from(other[e1234]) * wedge_g9.xyz()) - (Simd32x3::from(wedge_g10) * other.group9().xyz()),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (wedge_g9 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group9()),
            // e1234
            (wedge_g0_y * other[e1234]) + (wedge_g10 * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       10        0        0
    //    simd2        0        2        0      N/A
    //    simd3        3        3        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        4       15        0      N/A
    //  no simd       13       23        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([0.0, 0.0, other[e4125] * other[e4125] * self[e125] * -1.0, 0.0])
                + ((Simd32x3::from(self[e321] * other[e3215]) * other.group0().xyz()) + -(other.group0().xy() * other.group0().xy() * self.group0().xy()).with_z(0.0)
                    - (Simd32x3::from(other[e4235]) * Simd32x3::from([self[e315] * other[e4315], self[e235] * other[e4315], self[e235] * other[e4125]]))
                    - (Simd32x3::from(other[e4125]) * Simd32x3::from([self[e125] * other[e4235], self[e125] * other[e4315], self[e315] * other[e4315]])))
                .with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<Sphere> for AntiFlatPoint {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd3        2        7        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        6       16        0      N/A
    //  no simd       13       33        0        0
    fn project_via_origin_onto(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = Simd32x3::from(other[e1234] * -1.0) * self.group0().xyz();
        let wedge_g0_w = (right_dual_g0_xyz[0] * self[e235]) + (right_dual_g0_xyz[1] * self[e315]) + (right_dual_g0_xyz[2] * self[e125]) + (self[e321] * other[e3215]);
        let wedge_g1 = self[e321] * other[e1234] * -1.0;
        Circle::from_groups(
            // e423, e431, e412
            (wedge_g0_xyz * Simd32x3::from(other[e1234])) - (Simd32x3::from(wedge_g1) * other.group0().xyz()),
            // e415, e425, e435, e321
            (wedge_g0_xyz.yzx() * other.group0().zxy()).with_w(wedge_g0_w * other[e1234]) - (other.group0().yzxw() * wedge_g0_xyz.zxy().with_w(wedge_g1)),
            // e235, e315, e125
            (Simd32x3::from(wedge_g0_w) * other.group0().xyz()) - (wedge_g0_xyz * Simd32x3::from(other[e3215])),
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for AntiFlatPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd3        5        8        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd        9       19        0      N/A
    //  no simd       22       44        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let wedge_g1 = Simd32x3::from(0.0).with_w(right_dual_g0_w * self[e321]);
        let wedge_g2_xyz = Simd32x3::from(right_dual_g0_w) * self.group0().xyz();
        let wedge_g2_w = (self[e321] * other[e321]) - (right_dual_g0_xyz[0] * self[e235]) - (right_dual_g0_xyz[1] * self[e315]) - (right_dual_g0_xyz[2] * self[e125]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(wedge_g2_w) * other.group0(),
            // e415, e425, e435, e321
            (wedge_g1 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g2_w) * other.group1()),
            // e235, e315, e125, e5
            ((wedge_g2_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g2_w) * other.group2().xyz())).with_w(wedge_g2_w * other[e5]),
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g2_w) * other.group3().xyz())
                + (Simd32x3::from(wedge_g1[3]) * other.group1().xyz())
                + (Simd32x3::from(other[e321]) * wedge_g1.xyz())
                + (wedge_g2_xyz.yzx() * other.group0().zxy())
                - (wedge_g2_xyz.zxy() * other.group0().yzx()))
            .with_w(wedge_g2_w * other[e4]),
        )
    }
}
impl ProjectViaOriginOnto<VersorOdd> for AntiFlatPoint {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       12        0        0
    //    simd3        5       11        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       10       24        0      N/A
    //  no simd       23       49        0        0
    fn project_via_origin_onto(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = Simd32x3::from(other[e1234] * -1.0) * self.group0().xyz();
        let wedge_g0_w = (right_dual_g3_xyz[0] * self[e235]) + (right_dual_g3_xyz[1] * self[e315]) + (right_dual_g3_xyz[2] * self[e125]) + (self[e321] * other[e3215]);
        let wedge_g1 = self[e321] * other[e1234] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (wedge_g0_xyz * Simd32x3::from(other[e1234])) - (Simd32x3::from(wedge_g1) * other.group3().xyz()),
            // e415, e425, e435, e321
            (wedge_g0_xyz.yzx() * other.group3().zxy()).with_w(wedge_g0_w * other[e1234]) - (other.group3().yzxw() * wedge_g0_xyz.zxy().with_w(wedge_g1)),
            // e235, e315, e125, e4
            ((Simd32x3::from(wedge_g0_w) * other.group3().xyz()) - (wedge_g0_xyz * Simd32x3::from(other[e3215]))).with_w(wedge_g1 * other[e45] * -1.0),
            // e1, e2, e3, e5
            ((Simd32x3::from(wedge_g0_w) * other.group0().xyz()) + (wedge_g0_xyz.zxy() * other.group1().yzx())
                - (Simd32x3::from(wedge_g1) * other.group2().xyz())
                - (wedge_g0_xyz.yzx() * other.group1().zxy()))
            .with_w((wedge_g0_xyz[0] * other[e15]) + (wedge_g0_xyz[1] * other[e25])),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for AntiFlector {
    type Output = ProjectViaOriginOntoInfixPartial<AntiFlector>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiCircleRotor> for AntiFlector {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       15        0        0
    //    simd3        6        8        0      N/A
    // Totals...
    // yes simd       12       23        0      N/A
    //  no simd       24       39        0        0
    fn project_via_origin_onto(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = Simd32x3::from([
            (other[e12] * self[e2]) - (other[e31] * self[e3]),
            (other[e23] * self[e3]) - (other[e12] * self[e1]),
            (other[e31] * self[e1]) - (other[e23] * self[e2]),
        ]) - (right_dual_g0 * Simd32x3::from(self[e5]));
        let wedge_g0_w = other[e45] * self[e5] * -1.0;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g0_w) * other.group0()) + (wedge_g0_xyz.zxy() * other.group1().yzx())
                - (Simd32x3::from(right_dual_g0[0] * self[e1]) * other.group2().xyz())
                - (Simd32x3::from(right_dual_g0[1] * self[e2]) * other.group2().xyz())
                - (Simd32x3::from(right_dual_g0[2] * self[e3]) * other.group2().xyz())
                - (wedge_g0_xyz.yzx() * other.group1().zxy()))
            .with_w(0.0),
            // e5
            (wedge_g0_w * other[e45]) + (wedge_g0_xyz[0] * other[e15]) + (wedge_g0_xyz[1] * other[e25]) + (wedge_g0_xyz[2] * other[e35]),
        )
    }
}
impl ProjectViaOriginOnto<AntiDipoleInversion> for AntiFlector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       14        0        0
    //    simd3        8       14        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd       19       30        0      N/A
    //  no simd       38       64        0        0
    fn project_via_origin_onto(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_w = other[e321] * -1.0;
        let right_dual_g2_xyz = other.group2().xyz();
        let right_dual_g3_xyz = other.group3().xyz();
        let wedge_g0 = (other.group0().yzx() * self.group1().zxy()) - (other.group0().zxy() * self.group1().yzx());
        let wedge_g1_xyz = (Simd32x3::from(self[e5]) * other.group0()) - (Simd32x3::from(right_dual_g1_w) * self.group1().xyz());
        let wedge_g2 = (self.group1().yzxx() * right_dual_g2_xyz.zxy().with_w(right_dual_g3_xyz[0]))
            + ((Simd32x3::from(self[e5]) * other.group1().xyz()) - (right_dual_g2_xyz.yzx() * self.group1().zxy()))
                .with_w((right_dual_g3_xyz[1] * self[e2]) - (right_dual_g1_w * self[e321]) - (other[e423] * self[e235]) - (other[e431] * self[e315]) - (other[e412] * self[e125]));
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(wedge_g2[3]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(wedge_g2[3]) * other.group1(),
            // e235, e315, e125, e4
            (Simd32x4::from(wedge_g2[3]).xyz() * other.group2().xyz()).with_w(
                (wedge_g2[3] * other[e4])
                    - (wedge_g0[0] * other[e415])
                    - (wedge_g0[1] * other[e425])
                    - (wedge_g0[2] * other[e435])
                    - (wedge_g1_xyz[0] * other[e423])
                    - (wedge_g1_xyz[1] * other[e431])
                    - (wedge_g1_xyz[2] * other[e412]),
            ),
            // e1, e2, e3, e5
            ((wedge_g1_xyz * Simd32x3::from(other[e321]))
                + (Simd32x3::from(wedge_g2[3]) * other.group3().xyz())
                + (wedge_g0.zxy() * other.group2().yzx())
                + (other.group0().zxy() * wedge_g2.yzx())
                - (wedge_g0.yzx() * other.group2().zxy())
                - (other.group0().yzx() * wedge_g2.zxy()))
            .with_w(wedge_g2[3] * other[e5]),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlatPoint> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        2       14        0        0
    fn project_via_origin_onto(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(other[e321]) * self.group1().xyz().with_w(self[e321]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(wedge_g0[3]) * other.group0(),
            // e1, e2, e3, e5
            (wedge_g0.xyz() * Simd32x4::from(other[e321]).xyz()).with_w(-(wedge_g0[0] * other[e235]) - (wedge_g0[1] * other[e315]) - (wedge_g0[2] * other[e125])),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlector> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd3        1        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd        6       17        0        0
    fn project_via_origin_onto(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_w = other[e321] * -1.0;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0_w = (right_dual_g1_xyz[0] * self[e1]) + (right_dual_g1_xyz[1] * self[e2]) + (right_dual_g1_xyz[2] * self[e3]) - (right_dual_g0_w * self[e321]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(wedge_g0_w) * other.group0(),
            // e1, e2, e3, e5
            ((Simd32x3::from(wedge_g0_w) * other.group1().xyz()) - (Simd32x3::from(right_dual_g0_w * other[e321]) * self.group1().xyz())).with_w(wedge_g0_w * other[e5]),
        )
    }
}
impl ProjectViaOriginOnto<AntiLine> for AntiFlector {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        6       16        0        0
    fn project_via_origin_onto(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = (right_dual_g0.yzx() * self.group1().zxy()) - (right_dual_g0.zxy() * self.group1().yzx());
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            ((wedge_g0_xyz.zxy() * other.group0().yzx()) - (wedge_g0_xyz.yzx() * other.group0().zxy())).with_w(wedge_g0_xyz[0] * other[e15]),
        )
    }
}
impl ProjectViaOriginOnto<AntiMotor> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd2        2        5        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4       13        0      N/A
    //  no simd        6       25        0        0
    fn project_via_origin_onto(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g1_xy = (self.group1().yz() * other.group0().zx()) - (self.group1().zx() * other.group0().yz());
        let wedge_g1_z = (self[e1] * other[e31]) - (self[e2] * other[e23]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([1.0, 1.0, wedge_g1_z, 0.0])
                * (Simd32x4::from(other[e3215]).xyz() * (wedge_g1_xy * Simd32x2::from(-1.0)).with_z(1.0) * Simd32x3::from([1.0, 1.0, -1.0])).with_w(0.0),
            // e1, e2, e3, e5
            ((Simd32x2::from([wedge_g1_z, wedge_g1_xy[0]]) * other.group0().yz()) - (Simd32x2::from([wedge_g1_xy[1], wedge_g1_z]) * other.group0().zx()))
                .with_zw((wedge_g1_xy[1] * other[e23]) - (wedge_g1_xy[0] * other[e31]), wedge_g1_xy[0] * other[e15]),
        )
    }
}
impl ProjectViaOriginOnto<AntiPlane> for AntiFlector {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        8       15        0        0
    fn project_via_origin_onto(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x4::from(right_dual_g0_xyz[0] * self[e1]) * other.group0())
                + (Simd32x4::from(right_dual_g0_xyz[1] * self[e2]) * other.group0())
                + (Simd32x4::from(right_dual_g0_xyz[2] * self[e3]) * other.group0()),
        )
    }
}
impl ProjectViaOriginOnto<AntiScalar> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       11        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e12345] * -1.0;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Circle> for AntiFlector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        8       12        0      N/A
    // Totals...
    // yes simd       13       18        0      N/A
    //  no simd       29       42        0        0
    fn project_via_origin_onto(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (other.group0().yzx() * self.group1().zxy()) - (other.group0().zxy() * self.group1().yzx());
        let wedge_g1_xyz = (Simd32x3::from(self[e5]) * other.group0()) + (Simd32x3::from(other[e321]) * self.group1().xyz());
        let wedge_g2_xyz = (Simd32x3::from(self[e5]) * other.group1().xyz()) + (other.group2().zxy() * self.group1().yzx()) - (other.group2().yzx() * self.group1().zxy());
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(
                -(wedge_g0[0] * other[e415])
                    - (wedge_g0[1] * other[e425])
                    - (wedge_g0[2] * other[e435])
                    - (wedge_g1_xyz[0] * other[e423])
                    - (wedge_g1_xyz[1] * other[e431])
                    - (wedge_g1_xyz[2] * other[e412]),
            ),
            // e1, e2, e3, e5
            ((wedge_g1_xyz * Simd32x3::from(other[e321])) + (wedge_g0.zxy() * other.group2().yzx()) + (wedge_g2_xyz.yzx() * other.group0().zxy())
                - (wedge_g0.yzx() * other.group2().zxy())
                - (wedge_g2_xyz.zxy() * other.group0().yzx()))
            .with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<CircleRotor> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2       11        0        0
    //    simd3       12       18        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd       16       32        0      N/A
    //  no simd       46       77        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g1_w = other[e321] * -1.0;
        let right_dual_g2_xyz = other.group2().xyz();
        let right_dual_g2_w = other[e12345] * -1.0;
        let wedge_g0_xyz = (other.group0().yzx() * self.group1().zxy()) - (other.group0().zxy() * self.group1().yzx());
        let wedge_g0_w = right_dual_g1_w * self[e321] * -1.0;
        let wedge_g1 = (Simd32x3::from(self[e5]) * other.group0()).with_w((right_dual_g2_w * self[e321]) - (right_dual_g1_xyz[1] * self[e2]) - (right_dual_g1_xyz[2] * self[e3]))
            - (self.group1().xyzx() * Simd32x3::from(right_dual_g1_w).with_w(right_dual_g1_xyz[0]));
        let wedge_g2_xyz =
            (right_dual_g1_xyz * Simd32x3::from(self[e5])) + (Simd32x3::from(right_dual_g2_w) * self.group0().xyz()) + (right_dual_g2_xyz.zxy() * self.group1().yzx())
                - (right_dual_g2_xyz.yzx() * self.group1().zxy());
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((wedge_g0_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_w) * other.group0())).with_w(wedge_g0_w * other[e12345]),
            // e415, e425, e435, e321
            (wedge_g1 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_w) * other.group1()),
            // e235, e315, e125, e5
            ((wedge_g2_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_w) * other.group2().xyz())).with_w(right_dual_g2_w * self[e5] * other[e12345]),
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g1[3]) * other.group1().xyz())
                + (Simd32x3::from(other[e321]) * wedge_g1.xyz())
                + (Simd32x3::from(right_dual_g2_w * other[e12345]) * self.group1().xyz())
                + (wedge_g0_xyz.zxy() * other.group2().yzx())
                + (wedge_g2_xyz.yzx() * other.group0().zxy())
                - (wedge_g0_xyz.yzx() * other.group2().zxy())
                - (wedge_g2_xyz.zxy() * other.group0().yzx()))
            .with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<Dipole> for AntiFlector {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       15        0        0
    //    simd3        6        8        0      N/A
    // Totals...
    // yes simd       12       23        0      N/A
    //  no simd       24       39        0        0
    fn project_via_origin_onto(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = Simd32x3::from([
            (self[e2] * other[e12]) - (self[e3] * other[e31]),
            (self[e3] * other[e23]) - (self[e1] * other[e12]),
            (self[e1] * other[e31]) - (self[e2] * other[e23]),
        ]) - (right_dual_g0 * Simd32x3::from(self[e5]));
        let wedge_g0_w = self[e5] * other[e45] * -1.0;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g0_w) * other.group0()) + (wedge_g0_xyz.zxy() * other.group1().yzx())
                - (Simd32x3::from(right_dual_g0[0] * self[e1]) * other.group2())
                - (Simd32x3::from(right_dual_g0[1] * self[e2]) * other.group2())
                - (Simd32x3::from(right_dual_g0[2] * self[e3]) * other.group2())
                - (wedge_g0_xyz.yzx() * other.group1().zxy()))
            .with_w(0.0),
            // e5
            (wedge_g0_w * other[e45]) + (wedge_g0_xyz[0] * other[e15]) + (wedge_g0_xyz[1] * other[e25]) + (wedge_g0_xyz[2] * other[e35]),
        )
    }
}
impl ProjectViaOriginOnto<DipoleInversion> for AntiFlector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       15       30        0        0
    //    simd2        2        6        0      N/A
    //    simd3        6       11        0      N/A
    //    simd4        6        5        0      N/A
    // Totals...
    // yes simd       29       52        0      N/A
    //  no simd       61       95        0        0
    fn project_via_origin_onto(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0 = Simd32x3::from(other[e1234] * -1.0) * self.group1().xyz();
        let wedge_g1_xy = (self.group1().zx() * other.group3().yz()) - (self.group1().yz() * other.group3().zx());
        let wedge_g1_z = (self[e2] * other[e4235]) - (self[e1] * other[e4315]);
        let wedge_g2_xyz = (Simd32x3::from(self[e5]) * other.group3().xyz()) + (Simd32x3::from(other[e3215]) * self.group1().xyz());
        let wedge_g2_w = (right_dual_g0[0] * self[e1]) - (self[e321] * other[e1234]);
        let wedge_g3 = (self.group1().yzxx() * other.group1().zxy().with_w(other[e15]))
            + (-(self.group1().zx() * other.group1().yz()).with_z(self[e2] * other[e23] * -1.0)
                - (right_dual_g0 * Simd32x3::from(self[e5]))
                - (Simd32x3::from(other[e1234]) * self.group0().xyz()))
            .with_w(-(self[e235] * other[e4235]) - (self[e5] * other[e45]));
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other[e1234]) * wedge_g3.xyz()) - (Simd32x3::from(wedge_g2_w) * other.group3().xyz()),
            // e415, e425, e435, e321
            (wedge_g3.yzxw() * other.group3().zxy().with_w(other[e1234]))
                + -(wedge_g3.zx() * other.group3().yz()).with_zw(wedge_g3[1] * other[e4235] * -1.0, wedge_g2_w * other[e3215] * -1.0),
            // e235, e315, e125, e4
            ((Simd32x3::from(wedge_g3[3]) * other.group3().xyz()) - (Simd32x3::from(other[e3215]) * wedge_g3.xyz()))
                .with_w((wedge_g0[0] * other[e4235]) - (wedge_g2_w * other[e45]) - (other[e41] * wedge_g3[0]) - (other[e42] * wedge_g3[1]) - (other[e43] * wedge_g3[2])),
            // e1, e2, e3, e5
            Simd32x4::from([
                -(wedge_g2_w * other[e15]) - (wedge_g1_xy[1] * other[e4125]),
                -(wedge_g1_z * other[e4235]) - (wedge_g2_w * other[e25]),
                (wedge_g1_xy[1] * other[e4235]) - (wedge_g2_w * other[e35]) - (wedge_g1_xy[0] * other[e4315]) - (wedge_g3[0] * other[e31]),
                (wedge_g3[0] * other[e15]) + (wedge_g3[1] * other[e25]) - (wedge_g2_xyz[0] * other[e4235]) - (wedge_g2_xyz[1] * other[e4315]),
            ]) + (wedge_g3.zxyw() * other.group1().yzxw())
                + (wedge_g3.wwwz() * other.group0().with_w(other[e35]))
                + ((wedge_g2_xyz * Simd32x3::from(other[e1234]))
                    + ((Simd32x2::from([wedge_g1_z, wedge_g1_xy[0]]) * other.group3().yz()) - (wedge_g3.yz() * other.group1().zx())).with_z(0.0))
                .with_w(0.0)
                - (other.group3().wwwz() * wedge_g0.with_w(wedge_g2_xyz[2])),
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       12        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group1(),
        )
    }
}
impl ProjectViaOriginOnto<FlatPoint> for AntiFlector {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn project_via_origin_onto(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (self[e1] * other[e15] * other[e45]) + (self[e2] * other[e25] * other[e45]) + (self[e3] * other[e35] * other[e45]) - (self[e5] * other[e45] * other[e45]),
            0.0,
        ]))
    }
}
impl ProjectViaOriginOnto<Flector> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        9        0        0
    //    simd2        1        3        0      N/A
    //    simd3        0        1        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       10       16        0      N/A
    //  no simd       23       30        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([0.0, 0.0, (self[e2] * other[e4235]) - (self[e1] * other[e4315]), 0.0])
            + ((self.group1().zx() * other.group1().yz()) - (self.group1().yz() * other.group1().zx())).with_zw(0.0, 0.0);
        let wedge_g1 = (self.group1().xyzx() * Simd32x3::from(other[e3215]).with_w(other[e15]))
            + (self.group1().wwwy() * other.group1().xyz().with_w(other[e25]))
            + Simd32x3::from(0.0).with_w((self[e3] * other[e35]) - (self[e235] * other[e4235]) - (self[e5] * other[e45]));
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (Simd32x4::from(wedge_g1[3]).xyz() * other.group1().xyz()).with_w(0.0),
            // e1, e2, e3, e5
            (wedge_g0.zx() * other.group1().yz()).with_zw(wedge_g0[1] * other[e4235], (wedge_g1[3] * other[e45]) - (wedge_g1[1] * other[e4315]) - (wedge_g1[2] * other[e4125]))
                - (other.group1().zxyx() * wedge_g0.yzx().with_w(wedge_g1[0])),
        )
    }
}
impl ProjectViaOriginOnto<Line> for AntiFlector {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        2        3        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd        8       12        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = (Simd32x3::from(self[e5]) * other.group0()) + (other.group1().zxy() * self.group1().yzx()) - (other.group1().yzx() * self.group1().zxy());
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(wedge_g0_xyz[0] * other[e415]) - (wedge_g0_xyz[1] * other[e425]) - (wedge_g0_xyz[2] * other[e435])),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        4        6        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        4       11        0      N/A
    //  no simd       12       29        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_w = other[e12345] * -1.0;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0 =
            ((Simd32x3::from(right_dual_g0_w) * self.group0().xyz()) + (Simd32x3::from(self[e5]) * other.group0().xyz()) + (right_dual_g1_xyz.zxy() * self.group1().yzx())
                - (right_dual_g1_xyz.yzx() * self.group1().zxy()))
            .with_w(right_dual_g0_w * self[e321]);
        let wedge_g1 = Simd32x4::from(right_dual_g0_w) * self.group1();
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            wedge_g0 * Simd32x4::from(other[e12345]),
            // e1, e2, e3, e5
            ((Simd32x3::from(wedge_g0[3]) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g1.xyz())).with_w(wedge_g1[3] * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for AntiFlector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       60       78        0        0
    //    simd2        0        2        0      N/A
    //    simd3       45       61        0      N/A
    //    simd4       11       13        0      N/A
    // Totals...
    // yes simd      116      154        0      N/A
    //  no simd      239      317        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let right_dual_g3_w = other[e321] * -1.0;
        let right_dual_g5 = other.group6().xyz();
        let right_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_dual_g9_xyz = other.group1().xyz();
        let wedge_g0_y = (right_dual_g9_xyz[0] * self[e1]) + (right_dual_g9_xyz[1] * self[e2]) + (right_dual_g9_xyz[2] * self[e3])
            - (right_dual_g3_w * self[e321])
            - (other[e423] * self[e235])
            - (other[e431] * self[e315])
            - (other[e412] * self[e125])
            - (self[e5] * other[e4]);
        let wedge_g1 = Simd32x4::from([1.0, 1.0, right_dual_g0[0], 0.0]) * (self.group1().xyz() * Simd32x2::from(right_dual_g0[0]).with_z(1.0)).with_w(0.0);
        let wedge_g2 = right_dual_g0[0] * self[e5];
        let wedge_g3 = ((Simd32x3::from(other[e3215]) * self.group1().xyz()) - (right_dual_g1_xyz * Simd32x3::from(self[e5]))).with_w(self[e5] * other[e1234] * -1.0);
        let wedge_g4 = Simd32x3::from(other[e1234] * -1.0) * self.group1().xyz();
        let wedge_g5 = (right_dual_g1_xyz.zxy() * self.group1().yzx()) - (right_dual_g1_xyz.yzx() * self.group1().zxy());
        let wedge_g6 = (Simd32x3::from(self[e5]) * other.group7()).with_w((right_dual_g0[0] * self[e321]) - (right_dual_g5[1] * self[e2]) - (right_dual_g5[2] * self[e3]))
            - (self.group1().xyzx() * Simd32x3::from(right_dual_g3_w).with_w(right_dual_g5[0]));
        let wedge_g7 = (other.group7().yzx() * self.group1().zxy()) - (other.group7().zxy() * self.group1().yzx());
        let wedge_g8 = (right_dual_g5 * Simd32x3::from(self[e5])) + (Simd32x3::from(right_dual_g0[0]) * self.group0().xyz()) + (other.group8().zxy() * self.group1().yzx())
            - (other.group8().yzx() * self.group1().zxy());
        let wedge_g9 = ((Simd32x3::from(self[e5]) * other.group4()) + (right_dual_g6_xyz.yzx() * self.group1().zxy()) - (Simd32x3::from(other[e1234]) * self.group0().xyz()))
            .with_w(right_dual_g1_xyz[0] * self[e235])
            - (self.group1().yzxw() * right_dual_g6_xyz.zxy().with_w(other[e45]));
        let wedge_g10 = -(other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3]) - (self[e321] * other[e1234]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar])
                    + (wedge_g10 * other[e5])
                    + (wedge_g2 * other[e1234])
                    + (wedge_g1[0] * other[e4235])
                    + (wedge_g1[1] * other[e4315])
                    + (wedge_g1[2] * other[e4125])
                    + (wedge_g1[3] * other[e3215])
                    + (wedge_g9[0] * other[e1])
                    + (wedge_g9[1] * other[e2])
                    + (wedge_g9[2] * other[e3])
                    + (wedge_g9[3] * other[e4])
                    - (wedge_g4[0] * other[e235])
                    - (wedge_g4[1] * other[e315])
                    - (wedge_g4[2] * other[e125])
                    - (wedge_g5[0] * other[e415])
                    - (wedge_g5[1] * other[e425])
                    - (wedge_g5[2] * other[e435])
                    - (wedge_g7[0] * other[e15])
                    - (wedge_g7[1] * other[e25])
                    - (wedge_g7[2] * other[e35])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43])
                    - (other[e23] * wedge_g6[0])
                    - (other[e31] * wedge_g6[1])
                    - (other[e12] * wedge_g6[2])
                    - (other[e423] * wedge_g3[0])
                    - (other[e431] * wedge_g3[1])
                    - (other[e412] * wedge_g3[2])
                    - (wedge_g3[3] * other[e321])
                    - (wedge_g6[3] * other[e45]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            (wedge_g1 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group1())
                + (other.group9().yzxx() * wedge_g5.zxy().with_w(wedge_g4[0]))
                + ((Simd32x3::from(wedge_g6[3]) * other.group6().xyz())
                    + (Simd32x3::from(wedge_g9[3]) * other.group4())
                    + (Simd32x3::from(other[e321]) * wedge_g6.xyz())
                    + (Simd32x3::from(other[e1234]) * wedge_g3.xyz())
                    + (wedge_g7.zxy() * other.group8().yzx())
                    + (wedge_g8.yzx() * other.group7().zxy())
                    + (other.group5().yzx() * wedge_g9.zxy())
                    - (wedge_g4 * Simd32x3::from(other[e3215]))
                    - (Simd32x3::from(wedge_g10) * other.group3().xyz())
                    - (wedge_g5.yzx() * other.group9().zxy())
                    - (wedge_g7.yzx() * other.group8().zxy())
                    - (wedge_g8.zxy() * other.group7().yzx())
                    - (other.group5().zxy() * wedge_g9.yzx()))
                .with_w((wedge_g4[1] * other[e4315]) + (wedge_g4[2] * other[e4125]) + (wedge_g3[3] * other[e1234])),
            // e5
            (wedge_g0_y * other[e5])
                + (wedge_g2 * other[e12345])
                + (wedge_g9[0] * other[e15])
                + (wedge_g9[1] * other[e25])
                + (wedge_g9[2] * other[e35])
                + (wedge_g9[3] * other[e45])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435])
                - (other[e235] * wedge_g6[0])
                - (other[e315] * wedge_g6[1])
                - (other[e125] * wedge_g6[2])
                - (wedge_g3[0] * other[e4235])
                - (wedge_g3[1] * other[e4315])
                - (wedge_g3[2] * other[e4125])
                - (wedge_g3[3] * other[e3215]),
            // e15, e25, e35, e45
            (wedge_g3 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group3())
                + ((Simd32x3::from(wedge_g9[3]) * other.group6().xyz())
                    + (Simd32x3::from(other[e3215]) * wedge_g6.xyz())
                    + (wedge_g8.yzx() * other.group9().zxy())
                    + (other.group8().yzx() * wedge_g9.zxy())
                    - (wedge_g8.zxy() * other.group9().yzx())
                    - (other.group8().zxy() * wedge_g9.yzx()))
                .with_w(0.0),
            // e41, e42, e43
            (wedge_g4 * Simd32x3::from(other[e12345]))
                + (Simd32x3::from(wedge_g0_y) * other.group4())
                + (Simd32x3::from(wedge_g10) * other.group6().xyz())
                + (Simd32x3::from(other[e1234]) * wedge_g6.xyz())
                + (wedge_g7.zxy() * other.group9().yzx())
                + (other.group7().zxy() * wedge_g9.yzx())
                - (wedge_g7.yzx() * other.group9().zxy())
                - (other.group7().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (wedge_g5 * Simd32x3::from(other[e12345]))
                + (wedge_g7 * Simd32x3::from(other[e3215]))
                + (wedge_g8 * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(wedge_g0_y) * other.group5())
                + (Simd32x3::from(wedge_g10) * other.group8())
                + (Simd32x3::from(wedge_g9[3]) * other.group7())
                - (Simd32x3::from(wedge_g6[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            (wedge_g6 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group6())
                + (wedge_g9.yzxw() * other.group9().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group9().yz()).with_zw(wedge_g9[1] * other[e4235] * -1.0, wedge_g10 * other[e3215] * -1.0),
            // e423, e431, e412
            (wedge_g7 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group7()) + (Simd32x3::from(other[e1234]) * wedge_g9.xyz())
                - (Simd32x3::from(wedge_g10) * other.group9().xyz()),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (wedge_g9 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group9()),
            // e1234
            (wedge_g0_y * other[e1234]) + (wedge_g10 * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2       13        0        0
    //    simd2        2        4        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        6       17        0      N/A
    //  no simd       14       21        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([0.0, 0.0, (self[e2] * other[e4235]) - (self[e1] * other[e4315]), 0.0])
            + ((self.group1().zx() * other.group0().yz()) - (self.group1().yz() * other.group0().zx())).with_zw(0.0, 0.0);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x3::from([
                other[e4235] * other[e4235] * self[e235] * -1.0,
                self[e235] * other[e4235] * other[e4315] * -1.0,
                self[e235] * other[e4235] * other[e4125] * -1.0,
            ])
            .with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, (wedge_g0[1] * other[e4235]) - (wedge_g0[0] * other[e4315]), 0.0])
                + ((wedge_g0.zx() * other.group0().yz()) - (wedge_g0.yz() * other.group0().zx())).with_zw(0.0, 0.0),
        )
    }
}
impl ProjectViaOriginOnto<RoundPoint> for AntiFlector {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3        9        0        0
    fn project_via_origin_onto(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let wedge_g0 = (right_dual_g0_xyz[0] * self[e1]) + (right_dual_g0_xyz[1] * self[e2]) + (right_dual_g0_xyz[2] * self[e3]) - (self[e5] * other[e4]);
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(wedge_g0) * other.group0(), /* e5 */ wedge_g0 * other[e5])
    }
}
impl ProjectViaOriginOnto<Sphere> for AntiFlector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       16        0        0
    //    simd2        2        4        0      N/A
    //    simd3        5       11        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd       14       32        0      N/A
    //  no simd       32       61        0        0
    fn project_via_origin_onto(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = Simd32x3::from(other[e1234] * -1.0) * self.group1().xyz();
        let wedge_g1_xy = (self.group1().yz() * right_dual_g0_xyz.zx()) - (self.group1().zx() * right_dual_g0_xyz.yz());
        let wedge_g1_z = (right_dual_g0_xyz[1] * self[e1]) - (right_dual_g0_xyz[0] * self[e2]);
        let wedge_g2_w = self[e321] * other[e1234] * -1.0;
        let wedge_g3_xyz = Simd32x3::from(other[e1234] * -1.0) * self.group0().xyz();
        let wedge_g3_w = (right_dual_g0_xyz[0] * self[e235]) + (right_dual_g0_xyz[1] * self[e315]) + (right_dual_g0_xyz[2] * self[e125]) + (self[e321] * other[e3215]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (wedge_g3_xyz * Simd32x3::from(other[e1234])) - (Simd32x3::from(wedge_g2_w) * other.group0().xyz()),
            // e415, e425, e435, e321
            (wedge_g3_xyz.yzx() * other.group0().zxy()).with_w(wedge_g3_w * other[e1234]) - (other.group0().yzxw() * wedge_g3_xyz.zxy().with_w(wedge_g2_w)),
            // e235, e315, e125, e4
            ((Simd32x3::from(wedge_g3_w) * other.group0().xyz()) - (wedge_g3_xyz * Simd32x3::from(other[e3215]))).with_w(wedge_g0[0] * other[e4235]),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, (wedge_g1_xy[1] * other[e4235]) - (wedge_g1_xy[0] * other[e4315]), 0.0])
                + ((Simd32x3::from(other[e3215] * other[e1234]) * self.group1().xyz())
                    + ((Simd32x2::from([wedge_g1_z, wedge_g1_xy[0]]) * other.group0().yz()) - (Simd32x2::from([wedge_g1_xy[1], wedge_g1_z]) * other.group0().zx())).with_z(0.0)
                    - (right_dual_g0_xyz * Simd32x3::from(self[e5] * other[e1234]))
                    - (wedge_g0 * Simd32x3::from(other[e3215])))
                .with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       20       33        0        0
    //    simd2        1        2        0      N/A
    //    simd3        8       10        0      N/A
    //    simd4        5        6        0      N/A
    // Totals...
    // yes simd       34       51        0      N/A
    //  no simd       66       91        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e12345] * -1.0);
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g1_w = other[e321] * -1.0;
        let right_dual_g2_xyz = other.group2().xyz();
        let wedge_g0_xy = (right_dual_g0.yz() * self.group1().zx()) - (right_dual_g0.zx() * self.group1().yz());
        let wedge_g0_z = (right_dual_g0[0] * self[e2]) - (right_dual_g0[1] * self[e1]);
        let wedge_g0_w =
            (self[e1] * other[e1]) - (right_dual_g1_w * self[e321]) - (right_dual_g0[0] * self[e235]) - (right_dual_g0[1] * self[e315]) - (right_dual_g0[2] * self[e125]);
        let wedge_g1 = (right_dual_g0 * Simd32x3::from(self[e5]).with_w(self[e321]))
            + Simd32x3::from(0.0).with_w(-(right_dual_g1_xyz[1] * self[e2]) - (right_dual_g1_xyz[2] * self[e3]))
            - (self.group1().xyzx() * Simd32x3::from(right_dual_g1_w).with_w(right_dual_g1_xyz[0]));
        let wedge_g2 =
            ((right_dual_g1_xyz * Simd32x3::from(self[e5])) + (Simd32x3::from(right_dual_g0[3]) * self.group0().xyz()) + (right_dual_g2_xyz.zxy() * self.group1().yzx())
                - (right_dual_g2_xyz.yzx() * self.group1().zxy()))
            .with_w(right_dual_g0[3] * self[e5]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x3::from(wedge_g0_w) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g0_xy.with_z(wedge_g0_z))).with_w(wedge_g0_w * other[e12345]),
            // e415, e425, e435, e321
            (wedge_g1 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_w) * other.group1()),
            // e235, e315, e125, e5
            (wedge_g2 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_w) * other.group2())
                + Simd32x3::from(0.0).with_w(
                    -(wedge_g1[0] * other[e235])
                        - (wedge_g1[1] * other[e315])
                        - (wedge_g1[2] * other[e125])
                        - (wedge_g2[0] * other[e415])
                        - (wedge_g2[1] * other[e425])
                        - (wedge_g2[2] * other[e435]),
                ),
            // e1, e2, e3, e4
            (Simd32x3::from([
                (wedge_g0_z * other[e315]) + (wedge_g2[1] * other[e412]) - (wedge_g0_xy[1] * other[e125]) - (wedge_g2[2] * other[e431]),
                (wedge_g0_xy[0] * other[e125]) + (wedge_g2[2] * other[e423]) - (wedge_g0_z * other[e235]) - (wedge_g2[0] * other[e412]),
                (wedge_g0_xy[1] * other[e235]) + (wedge_g2[0] * other[e431]) - (wedge_g0_xy[0] * other[e315]) - (wedge_g2[1] * other[e423]),
            ]) + (Simd32x3::from(wedge_g0_w) * other.group3().xyz())
                + (Simd32x3::from(wedge_g1[3]) * other.group1().xyz())
                + (Simd32x3::from(other[e321]) * wedge_g1.xyz())
                + (Simd32x3::from(right_dual_g0[3] * other[e12345]) * self.group1().xyz()))
            .with_w(wedge_g0_w * other[e4]),
        )
    }
}
impl ProjectViaOriginOnto<VersorOdd> for AntiFlector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       25        0        0
    //    simd2        1        2        0      N/A
    //    simd3        5       12        0      N/A
    //    simd4        5        4        0      N/A
    // Totals...
    // yes simd       24       43        0      N/A
    //  no simd       50       81        0        0
    fn project_via_origin_onto(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = Simd32x3::from(other[e1234] * -1.0) * self.group1().xyz();
        let wedge_g1_xy = (self.group1().yz() * right_dual_g3_xyz.zx()) - (self.group1().zx() * right_dual_g3_xyz.yz());
        let wedge_g1_z = (right_dual_g3_xyz[1] * self[e1]) - (right_dual_g3_xyz[0] * self[e2]);
        let wedge_g2_xyz = (Simd32x3::from(other[e3215]) * self.group1().xyz()) - (right_dual_g3_xyz * Simd32x3::from(self[e5]));
        let wedge_g3_xyz = Simd32x3::from([
            (self[e2] * other[e12]) - (self[e3] * other[e31]),
            (self[e3] * other[e23]) - (self[e1] * other[e12]),
            (self[e1] * other[e31]) - (self[e2] * other[e23]),
        ]) + (Simd32x3::from(self[e5]) * other.group0().xyz())
            - (Simd32x3::from(other[e1234]) * self.group0().xyz());
        let wedge_g3_w = (right_dual_g3_xyz[0] * self[e235]) - (self[e5] * other[e45]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            wedge_g3_xyz * Simd32x3::from(other[e1234]),
            // e415, e425, e435, e321
            ((wedge_g3_xyz.yzx() * other.group3().zxy()) - (wedge_g3_xyz.zxy() * other.group3().yzx())).with_w(wedge_g3_w * other[e1234]),
            // e235, e315, e125, e4
            (other.group3().xyzx() * Simd32x3::from(wedge_g3_w).with_w(wedge_g0[0]))
                + -(wedge_g3_xyz * Simd32x3::from(other[e3215])).with_w(-(wedge_g3_xyz[0] * other[e41]) - (wedge_g3_xyz[1] * other[e42]) - (wedge_g3_xyz[2] * other[e43])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (wedge_g1_z * other[e4315]) + (wedge_g3_w * other[e41]),
                (wedge_g3_w * other[e42]) + (wedge_g1_xy[0] * other[e4125]),
                (wedge_g3_w * other[e43]) + (wedge_g1_xy[1] * other[e4235]),
                (wedge_g3_w * other[e45]) + (wedge_g3_xyz[0] * other[e15]) + (wedge_g3_xyz[2] * other[e35]) - (wedge_g2_xyz[2] * other[e4125]),
            ]) + (other.group2().wwwy() * wedge_g2_xyz.with_w(wedge_g3_xyz[1]))
                + ((wedge_g3_xyz.zxy() * other.group1().yzx()) - (wedge_g3_xyz.yzx() * other.group1().zxy())).with_w(0.0)
                - (Simd32x4::from([wedge_g1_xy[1], wedge_g1_z, wedge_g1_xy[0], wedge_g2_xyz[0]]) * other.group3().zxyx())
                - (other.group3().wwwy() * wedge_g0.with_w(wedge_g2_xyz[1])),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for AntiLine {
    type Output = ProjectViaOriginOntoInfixPartial<AntiLine>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiCircleRotor> for AntiLine {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        5        9        0      N/A
    //  no simd        5       17        0        0
    fn project_via_origin_onto(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 =
            (other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) + (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g0) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(wedge_g0) * other.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(wedge_g0) * other.group2(),
        )
    }
}
impl ProjectViaOriginOnto<AntiDipoleInversion> for AntiLine {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        6       10        0      N/A
    // Totals...
    // yes simd        8       14        0      N/A
    //  no simd       20       34        0        0
    fn project_via_origin_onto(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = (other.group0().yzx() * self.group1().zxy()) - (Simd32x3::from(other[e321]) * self.group0()) - (other.group0().zxy() * self.group1().yzx());
        let wedge_g1 = -(other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(wedge_g1) * other.group1().xyz()) + (wedge_g0_xyz.yzx() * other.group0().zxy()) - (wedge_g0_xyz.zxy() * other.group0().yzx()),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g1) * other.group2().xyz()) - (wedge_g0_xyz * Simd32x3::from(other[e321]))).with_w(0.0),
            // e15, e25, e35, scalar
            ((wedge_g0_xyz.zxy() * other.group2().yzx()) - (wedge_g0_xyz.yzx() * other.group2().zxy())).with_w(wedge_g0_xyz[0] * other[e1]),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlatPoint> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        1        4        0      N/A
    // Totals...
    // yes simd        1        6        0      N/A
    //  no simd        3       14        0        0
    fn project_via_origin_onto(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(other[e321] * -1.0) * self.group0();
        AntiLine::from_groups(
            // e23, e31, e12
            wedge_g0_xyz * Simd32x3::from(other[e321] * -1.0),
            // e15, e25, e35
            (wedge_g0_xyz.zxy() * other.group0().yzx()) - (wedge_g0_xyz.yzx() * other.group0().zxy()),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlector> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        1        5        0      N/A
    // Totals...
    // yes simd        3        9        0      N/A
    //  no simd        5       19        0        0
    fn project_via_origin_onto(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(other[e321] * -1.0) * self.group0();
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (wedge_g0_xyz * Simd32x4::from(other[e321]).xyz() * Simd32x3::from(-1.0))
                .with_w((wedge_g0_xyz[0] * other[e1]) + (wedge_g0_xyz[1] * other[e2]) + (wedge_g0_xyz[2] * other[e3])),
            // e15, e25, e35, e3215
            ((wedge_g0_xyz.zxy() * other.group0().yzx()) - (wedge_g0_xyz.yzx() * other.group0().zxy())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<AntiLine> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        9        0        0
    fn project_via_origin_onto(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]);
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(wedge_g0) * other.group0(),
            // e15, e25, e35
            Simd32x3::from(wedge_g0) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<AntiMotor> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2       11        0        0
    fn project_via_origin_onto(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_w = (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(wedge_g0_w) * other.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(wedge_g0_w) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<AntiScalar> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0        9        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e12345] * -1.0;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(right_dual_g0 * other[e12345]) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(right_dual_g0 * other[e12345]) * self.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Circle> for AntiLine {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        6       10        0      N/A
    // Totals...
    // yes simd        8       13        0      N/A
    //  no simd       20       33        0        0
    fn project_via_origin_onto(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = (self.group1().zxy() * other.group0().yzx()) - (Simd32x3::from(other[e321]) * self.group0()) - (self.group1().yzx() * other.group0().zxy());
        let wedge_g1 = -(self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]);
        Dipole::from_groups(
            // e41, e42, e43
            (Simd32x3::from(wedge_g1) * other.group1().xyz()) + (wedge_g0_xyz.yzx() * other.group0().zxy()) - (wedge_g0_xyz.zxy() * other.group0().yzx()),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g1) * other.group2()) - (wedge_g0_xyz * Simd32x3::from(other[e321]))).with_w(0.0),
            // e15, e25, e35
            (wedge_g0_xyz.zxy() * other.group2().yzx()) - (wedge_g0_xyz.yzx() * other.group2().zxy()),
        )
    }
}
impl ProjectViaOriginOnto<CircleRotor> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       13        0        0
    //    simd3        9       12        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       14       26        0      N/A
    //  no simd       32       53        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2_w = other[e12345] * -1.0;
        let wedge_g2_w = -(self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]);
        let wedge_g3 = ((self.group1().zxy() * other.group0().yzx()) - (Simd32x3::from(other[e321]) * self.group0()) - (self.group1().yzx() * other.group0().zxy())).with_w(0.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x3::from(wedge_g2_w) * other.group1().xyz()) + (other.group0().zxy() * wedge_g3.yzx()) - (other.group0().yzx() * wedge_g3.zxy())).with_w(0.0),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g2_w) * other.group2().xyz())
                + (Simd32x3::from(wedge_g3[3]) * other.group0())
                + (Simd32x3::from(right_dual_g2_w * other[e12345]) * self.group0())
                - (Simd32x3::from(other[e321]) * wedge_g3.xyz()))
            .with_w(0.0),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g3[2] * other[e315]) - (wedge_g3[1] * other[e125]),
                (wedge_g3[0] * other[e125]) - (wedge_g3[2] * other[e235]),
                (wedge_g3[1] * other[e235]) - (wedge_g3[0] * other[e315]),
            ]) + (Simd32x3::from(wedge_g3[3]) * other.group1().xyz())
                + (Simd32x3::from(right_dual_g2_w * other[e12345]) * self.group1()))
            .with_w(wedge_g2_w * other[e12345]),
            // e4235, e4315, e4125, e3215
            wedge_g3 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Dipole> for AntiLine {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        5        9        0      N/A
    //  no simd        5       16        0        0
    fn project_via_origin_onto(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 =
            (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]) + (self[e15] * other[e41]) + (self[e25] * other[e42]) + (self[e35] * other[e43]);
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g0) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(wedge_g0) * other.group1(),
            // e15, e25, e35
            Simd32x3::from(wedge_g0) * other.group2(),
        )
    }
}
impl ProjectViaOriginOnto<DipoleInversion> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd3        8       14        0      N/A
    // Totals...
    // yes simd       10       19        0      N/A
    //  no simd       26       47        0        0
    fn project_via_origin_onto(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(other[e1234]) * self.group0();
        let wedge_g1_xyz = Simd32x3::from(other[e1234]) * self.group1();
        let wedge_g1_w = (self[e23] * other[e4235]) + (self[e31] * other[e4315]) + (self[e12] * other[e4125]);
        let wedge_g2_xyz = (Simd32x3::from(other[e3215]) * self.group0()) + (self.group1().yzx() * other.group3().zxy()) - (self.group1().zxy() * other.group3().yzx());
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((wedge_g1_xyz * Simd32x3::from(other[e1234])) + (wedge_g0.zxy() * other.group3().yzx()) - (wedge_g0.yzx() * other.group3().zxy()))
                .with_w(wedge_g1_w * other[e45] * -1.0),
            // e23, e31, e12, e45
            ((wedge_g0 * Simd32x3::from(other[e3215])) + (wedge_g2_xyz * Simd32x3::from(other[e1234])) - (Simd32x3::from(wedge_g1_w) * other.group3().xyz())).with_w(0.0),
            // e15, e25, e35, e1234
            ((wedge_g1_xyz * Simd32x3::from(other[e3215])) + (wedge_g2_xyz.yzx() * other.group3().zxy()) - (wedge_g2_xyz.zxy() * other.group3().yzx())).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       10        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(other[e12345] * other[e12345] * -1.0) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(other[e12345] * other[e12345] * -1.0) * self.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Flector> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        3        5        0      N/A
    // no simd        9       15        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = (Simd32x3::from(other[e3215]) * self.group0()) + (self.group1().yzx() * other.group1().zxy()) - (self.group1().zxy() * other.group1().yzx());
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
            // e15, e25, e35, e3215
            ((wedge_g0_xyz.yzx() * other.group1().zxy()) - (wedge_g0_xyz.zxy() * other.group1().yzx())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<Line> for AntiLine {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        5        9        0      N/A
    // Totals...
    // yes simd        5       12        0      N/A
    //  no simd       15       30        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            (-(Simd32x3::from(self[e23] * other[e235]) * other.group0())
                - (Simd32x3::from(self[e31] * other[e315]) * other.group0())
                - (Simd32x3::from(self[e12] * other[e125]) * other.group0())
                - (other.group0() * other.group0() * self.group1())
                - (Simd32x3::from(other[e415]) * self.group1().yxx() * other.group0().yyz())
                - (Simd32x3::from(other[e435]) * self.group1().zzy() * other.group0().xyy()))
            .with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       12        0        0
    //    simd3        1        4        0      N/A
    // Totals...
    // yes simd        8       16        0      N/A
    //  no simd       10       24        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0_xyz = Simd32x3::from(right_dual_g0_w) * self.group0();
        let wedge_g1_w = -(right_dual_g0_xyz[0] * self[e15])
            - (right_dual_g0_xyz[1] * self[e25])
            - (right_dual_g0_xyz[2] * self[e35])
            - (right_dual_g1_xyz[0] * self[e23])
            - (right_dual_g1_xyz[1] * self[e31])
            - (right_dual_g1_xyz[2] * self[e12]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (wedge_g0_xyz * Simd32x4::from(other[e12345]).xyz()).with_w(-(wedge_g0_xyz[0] * other[e415]) - (wedge_g0_xyz[1] * other[e425]) - (wedge_g0_xyz[2] * other[e435])),
            // e15, e25, e35, e3215
            ((Simd32x3::from(wedge_g1_w) * other.group0().xyz()) + (Simd32x3::from(right_dual_g0_w * other[e12345]) * self.group1())).with_w(wedge_g1_w * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       46       58        0        0
    //    simd2        0        2        0      N/A
    //    simd3       40       53        0      N/A
    //    simd4        7        9        0      N/A
    // Totals...
    // yes simd       93      122        0      N/A
    //  no simd      194      257        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let wedge_g0_y =
            (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]) + (self[e15] * other[e41]) + (self[e25] * other[e42]) + (self[e35] * other[e43]);
        let wedge_g3 = Simd32x4::from([1.0, 1.0, right_dual_g0[0], 0.0]) * (self.group1() * Simd32x2::from(right_dual_g0[0]).with_z(1.0)).with_w(0.0);
        let wedge_g5 = Simd32x3::from(right_dual_g0[0]) * self.group0();
        let wedge_g6 =
            (Simd32x3::from(other[e1234]) * self.group1()).with_w(-(right_dual_g1_xyz[0] * self[e23]) - (right_dual_g1_xyz[1] * self[e31]) - (right_dual_g1_xyz[2] * self[e12]));
        let wedge_g7 = Simd32x3::from(other[e1234]) * self.group0();
        let wedge_g8 = (Simd32x3::from(other[e3215]) * self.group0()) + (right_dual_g1_xyz.yzx() * self.group1().zxy()) - (right_dual_g1_xyz.zxy() * self.group1().yzx());
        let wedge_g9 = ((self.group1().zxy() * other.group7().yzx()) - (Simd32x3::from(other[e321]) * self.group0()) - (self.group1().yzx() * other.group7().zxy())).with_w(0.0);
        let wedge_g10 = -(self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar])
                    + (wedge_g10 * other[e5])
                    + (wedge_g9[0] * other[e1])
                    + (wedge_g9[1] * other[e2])
                    + (wedge_g9[2] * other[e3])
                    + (wedge_g9[3] * other[e4])
                    - (wedge_g5[0] * other[e415])
                    - (wedge_g5[1] * other[e425])
                    - (wedge_g5[2] * other[e435])
                    - (wedge_g7[0] * other[e15])
                    - (wedge_g7[1] * other[e25])
                    - (wedge_g7[2] * other[e35])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43])
                    - (other[e23] * wedge_g6[0])
                    - (other[e31] * wedge_g6[1])
                    - (other[e12] * wedge_g6[2])
                    - (other[e423] * wedge_g3[0])
                    - (other[e431] * wedge_g3[1])
                    - (other[e412] * wedge_g3[2])
                    - (wedge_g3[3] * other[e321])
                    - (wedge_g6[3] * other[e45]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(wedge_g0_y) * other.group1())
                + ((Simd32x3::from(wedge_g6[3]) * other.group6().xyz())
                    + (Simd32x3::from(wedge_g9[3]) * other.group4())
                    + (Simd32x3::from(other[e321]) * wedge_g6.xyz())
                    + (Simd32x3::from(other[e1234]) * wedge_g3.xyz())
                    + (wedge_g5.zxy() * other.group9().yzx())
                    + (wedge_g7.zxy() * other.group8().yzx())
                    + (wedge_g8.yzx() * other.group7().zxy())
                    + (other.group5().yzx() * wedge_g9.zxy())
                    - (Simd32x3::from(wedge_g10) * other.group3().xyz())
                    - (wedge_g5.yzx() * other.group9().zxy())
                    - (wedge_g7.yzx() * other.group8().zxy())
                    - (wedge_g8.zxy() * other.group7().yzx())
                    - (other.group5().zxy() * wedge_g9.yzx()))
                .with_w(wedge_g3[3] * other[e1234]),
            // e5
            (wedge_g0_y * other[e5]) + (wedge_g9[0] * other[e15]) + (wedge_g9[1] * other[e25]) + (wedge_g9[2] * other[e35]) + (wedge_g9[3] * other[e45])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435])
                - (other[e235] * wedge_g6[0])
                - (other[e315] * wedge_g6[1])
                - (other[e125] * wedge_g6[2])
                - (wedge_g3[0] * other[e4235])
                - (wedge_g3[1] * other[e4315])
                - (wedge_g3[2] * other[e4125])
                - (wedge_g3[3] * other[e3215]),
            // e15, e25, e35, e45
            (wedge_g3 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group3())
                + ((Simd32x3::from(wedge_g9[3]) * other.group6().xyz())
                    + (Simd32x3::from(other[e3215]) * wedge_g6.xyz())
                    + (wedge_g8.yzx() * other.group9().zxy())
                    + (other.group8().yzx() * wedge_g9.zxy())
                    - (wedge_g8.zxy() * other.group9().yzx())
                    - (other.group8().zxy() * wedge_g9.yzx()))
                .with_w(0.0),
            // e41, e42, e43
            (Simd32x3::from(wedge_g0_y) * other.group4())
                + (Simd32x3::from(wedge_g10) * other.group6().xyz())
                + (Simd32x3::from(other[e1234]) * wedge_g6.xyz())
                + (wedge_g7.zxy() * other.group9().yzx())
                + (other.group7().zxy() * wedge_g9.yzx())
                - (wedge_g7.yzx() * other.group9().zxy())
                - (other.group7().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (wedge_g5 * Simd32x3::from(other[e12345]))
                + (wedge_g7 * Simd32x3::from(other[e3215]))
                + (wedge_g8 * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(wedge_g0_y) * other.group5())
                + (Simd32x3::from(wedge_g10) * other.group8())
                + (Simd32x3::from(wedge_g9[3]) * other.group7())
                - (Simd32x3::from(wedge_g6[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            (wedge_g6 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group6())
                + (wedge_g9.yzxw() * other.group9().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group9().yz()).with_zw(wedge_g9[1] * other[e4235] * -1.0, wedge_g10 * other[e3215] * -1.0),
            // e423, e431, e412
            (wedge_g7 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group7()) + (Simd32x3::from(other[e1234]) * wedge_g9.xyz())
                - (Simd32x3::from(wedge_g10) * other.group9().xyz()),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (wedge_g9 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group9()),
            // e1234
            (wedge_g0_y * other[e1234]) + (wedge_g10 * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        3        5        0      N/A
    // no simd        9       15        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = (Simd32x3::from(other[e3215]) * self.group0()) + (self.group1().yzx() * other.group0().zxy()) - (self.group1().zxy() * other.group0().yzx());
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35
            (wedge_g0_xyz.yzx() * other.group0().zxy()) - (wedge_g0_xyz.zxy() * other.group0().yzx()),
        )
    }
}
impl ProjectViaOriginOnto<Sphere> for AntiLine {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3       10       17        0      N/A
    // Totals...
    // yes simd       10       20        0      N/A
    //  no simd       30       54        0        0
    fn project_via_origin_onto(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = Simd32x3::from(other[e1234]) * self.group0();
        let wedge_g1_xyz = Simd32x3::from(other[e1234]) * self.group1();
        let wedge_g2 = (Simd32x3::from(other[e3215]) * self.group0()) + (right_dual_g0_xyz.yzx() * self.group1().zxy()) - (right_dual_g0_xyz.zxy() * self.group1().yzx());
        Dipole::from_groups(
            // e41, e42, e43
            (wedge_g1_xyz * Simd32x3::from(other[e1234])) + (wedge_g0.zxy() * other.group0().yzx()) - (wedge_g0.yzx() * other.group0().zxy()),
            // e23, e31, e12, e45
            ((wedge_g0 * Simd32x3::from(other[e3215]))
                + (wedge_g2 * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(right_dual_g0_xyz[0] * self[e23]) * other.group0().xyz())
                + (Simd32x3::from(right_dual_g0_xyz[1] * self[e31]) * other.group0().xyz())
                + (Simd32x3::from(right_dual_g0_xyz[2] * self[e12]) * other.group0().xyz()))
            .with_w(0.0),
            // e15, e25, e35
            (wedge_g1_xyz * Simd32x3::from(other[e3215])) + (wedge_g2.yzx() * other.group0().zxy()) - (wedge_g2.zxy() * other.group0().yzx()),
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       19        0        0
    //    simd2        0        1        0      N/A
    //    simd3        7       11        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd       19       34        0      N/A
    //  no simd       39       66        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let wedge_g1_xyz = Simd32x3::from(right_dual_g0_w) * self.group0();
        let wedge_g2_xyz = Simd32x3::from(right_dual_g0_w) * self.group1();
        let wedge_g2_w = -(right_dual_g0_xyz[0] * self[e23]) - (right_dual_g0_xyz[1] * self[e31]) - (right_dual_g0_xyz[2] * self[e12]);
        let wedge_g3 =
            ((right_dual_g0_xyz.yzx() * self.group1().zxy()) - (Simd32x3::from(other[e321]) * self.group0()) - (right_dual_g0_xyz.zxy() * self.group1().yzx())).with_w(0.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from(wedge_g2_w) * other.group1().xyz().with_w(other[e5]))
                + (wedge_g3.yzxx() * other.group0().zxy().with_w(other[e1]))
                + -(wedge_g3.zx() * other.group0().yz()).with_zw(
                    wedge_g3[1] * other[e423] * -1.0,
                    -(wedge_g1_xyz[0] * other[e415])
                        - (wedge_g1_xyz[1] * other[e425])
                        - (wedge_g1_xyz[2] * other[e435])
                        - (wedge_g2_xyz[0] * other[e423])
                        - (wedge_g2_xyz[1] * other[e431])
                        - (wedge_g2_xyz[2] * other[e412]),
                ),
            // e23, e31, e12, e45
            ((wedge_g1_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g2_w) * other.group2().xyz()) + (Simd32x3::from(wedge_g3[3]) * other.group0().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g3.xyz()))
            .with_w(0.0),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g3[2] * other[e315]) - (wedge_g3[1] * other[e125]),
                (wedge_g3[0] * other[e125]) - (wedge_g3[2] * other[e235]),
                (wedge_g3[1] * other[e235]) - (wedge_g3[0] * other[e315]),
            ]) + (wedge_g2_xyz * Simd32x3::from(other[e12345]))
                + (Simd32x3::from(wedge_g3[3]) * other.group1().xyz()))
            .with_w(wedge_g2_w * other[e12345]),
            // e4235, e4315, e4125, e3215
            wedge_g3 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<VersorOdd> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3       10       17        0      N/A
    // Totals...
    // yes simd       10       20        0      N/A
    //  no simd       30       54        0        0
    fn project_via_origin_onto(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = Simd32x3::from(other[e1234]) * self.group0();
        let wedge_g1_xyz = Simd32x3::from(other[e1234]) * self.group1();
        let wedge_g2_xyz = (Simd32x3::from(other[e3215]) * self.group0()) + (right_dual_g3_xyz.yzx() * self.group1().zxy()) - (right_dual_g3_xyz.zxy() * self.group1().yzx());
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((wedge_g1_xyz * Simd32x3::from(other[e1234])) + (wedge_g0.zxy() * other.group3().yzx()) - (wedge_g0.yzx() * other.group3().zxy())).with_w(0.0),
            // e23, e31, e12, e45
            ((wedge_g0 * Simd32x3::from(other[e3215]))
                + (wedge_g2_xyz * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(right_dual_g3_xyz[0] * self[e23]) * other.group3().xyz())
                + (Simd32x3::from(right_dual_g3_xyz[1] * self[e31]) * other.group3().xyz())
                + (Simd32x3::from(right_dual_g3_xyz[2] * self[e12]) * other.group3().xyz()))
            .with_w(0.0),
            // e15, e25, e35, e1234
            ((wedge_g1_xyz * Simd32x3::from(other[e3215])) + (wedge_g2_xyz.yzx() * other.group3().zxy()) - (wedge_g2_xyz.zxy() * other.group3().yzx())).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for AntiMotor {
    type Output = ProjectViaOriginOntoInfixPartial<AntiMotor>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiCircleRotor> for AntiMotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       16       19        0        0
    //    simd3        0        5        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd       16       27        0      N/A
    //  no simd       16       46        0        0
    fn project_via_origin_onto(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g0 = right_dual_g0 * Simd32x3::from(self[scalar]);
        let wedge_g1 = right_dual_g1 * Simd32x4::from(self[scalar]);
        let wedge_g2_xyz = Simd32x3::from(self[scalar] * -1.0) * other.group2().xyz();
        let wedge_g2_w = (other[scalar] * self[scalar])
            - (right_dual_g0[0] * self[e15])
            - (right_dual_g0[1] * self[e25])
            - (right_dual_g0[2] * self[e35])
            - (right_dual_g1[0] * self[e23])
            - (right_dual_g1[1] * self[e31])
            - (right_dual_g1[2] * self[e12]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g2_w) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(wedge_g2_w) * other.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(wedge_g2_w) * other.group2().xyz()).with_w(
                (wedge_g2_w * other[scalar])
                    - (wedge_g0[0] * other[e15])
                    - (wedge_g0[1] * other[e25])
                    - (wedge_g0[2] * other[e35])
                    - (wedge_g2_xyz[0] * other[e41])
                    - (wedge_g2_xyz[1] * other[e42])
                    - (wedge_g2_xyz[2] * other[e43])
                    - (wedge_g1[0] * other[e23])
                    - (wedge_g1[1] * other[e31])
                    - (wedge_g1[2] * other[e12])
                    - (wedge_g1[3] * other[e45]),
            ),
        )
    }
}
impl ProjectViaOriginOnto<AntiDipoleInversion> for AntiMotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       17        0        0
    //    simd3        7       13        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd       21       33        0      N/A
    //  no simd       41       68        0        0
    fn project_via_origin_onto(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        let wedge_g0 = Simd32x3::from(self[scalar]) * other.group0();
        let wedge_g1 = right_dual_g1 * Simd32x4::from(self[scalar]);
        let wedge_g2_xyz = Simd32x3::from(self[scalar]) * other.group2().xyz();
        let wedge_g2_w = -(other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12]) - (other[e4] * self[scalar]);
        let wedge_g3_xyz =
            (Simd32x3::from(right_dual_g1[3]) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * other.group3().xyz()) + (other.group0().yzx() * self.group1().zxy())
                - (other.group0().zxy() * self.group1().yzx());
        let wedge_g3_w = other[e5] * self[scalar] * -1.0;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(wedge_g2_w) * other.group1().xyz()) + (wedge_g3_xyz.yzx() * other.group0().zxy()) - (wedge_g3_xyz.zxy() * other.group0().yzx()),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g2_w) * other.group2().xyz()) + (Simd32x3::from(wedge_g3_w) * other.group0()) - (wedge_g3_xyz * Simd32x3::from(other[e321]))).with_w(0.0),
            // e15, e25, e35, scalar
            (Simd32x4::from(wedge_g3_w) * other.group1().xyz().with_w(other[e4]))
                + (wedge_g3_xyz.zxy() * other.group2().yzx()).with_w(
                    (wedge_g3_xyz[0] * other[e1])
                        - (wedge_g0[1] * other[e315])
                        - (wedge_g0[2] * other[e125])
                        - (wedge_g2_xyz[0] * other[e423])
                        - (wedge_g2_xyz[1] * other[e431])
                        - (wedge_g2_xyz[2] * other[e412])
                        - (wedge_g1[0] * other[e415])
                        - (wedge_g1[1] * other[e425])
                        - (wedge_g1[2] * other[e435])
                        - (wedge_g1[3] * other[e321]),
                )
                - (other.group2().zxyx() * wedge_g3_xyz.yzx().with_w(wedge_g0[0])),
        )
    }
}
impl ProjectViaOriginOnto<AntiDualNum> for AntiMotor {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        3        0        0
    fn project_via_origin_onto(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(other[scalar] * self[scalar]) * other.group0())
    }
}
impl ProjectViaOriginOnto<AntiFlatPoint> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        1        3        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        7        0      N/A
    //  no simd        3       16        0        0
    fn project_via_origin_onto(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_w = other[e321] * -1.0;
        let wedge_g1_xyz = Simd32x3::from(right_dual_g0_w) * self.group0().xyz();
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other[e321] * -1.0) * wedge_g1_xyz.with_w(right_dual_g0_w * self[scalar]),
            // e15, e25, e35, e3215
            ((wedge_g1_xyz.zxy() * other.group0().yzx()) - (wedge_g1_xyz.yzx() * other.group0().zxy())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlector> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        2        6        0      N/A
    // Totals...
    // yes simd        5       12        0      N/A
    //  no simd        9       24        0        0
    fn project_via_origin_onto(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_w = other[e321] * -1.0;
        let wedge_g1_xyz = (Simd32x3::from(right_dual_g0_w) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * other.group1().xyz());
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (wedge_g1_xyz * Simd32x4::from(other[e321]).xyz() * Simd32x3::from(-1.0))
                .with_w((wedge_g1_xyz[0] * other[e1]) + (wedge_g1_xyz[1] * other[e2]) + (wedge_g1_xyz[2] * other[e3]) - (right_dual_g0_w * other[e321] * self[scalar])),
            // e15, e25, e35, e3215
            ((wedge_g1_xyz.zxy() * other.group0().yzx()) - (wedge_g1_xyz.yzx() * other.group0().zxy())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<AntiLine> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        4        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd        4       18        0        0
    fn project_via_origin_onto(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = right_dual_g0 * Simd32x3::from(self[scalar]);
        let wedge_g0_w = -(right_dual_g0[0] * self[e23]) - (right_dual_g0[1] * self[e31]) - (right_dual_g0[2] * self[e12]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(wedge_g0_w) * other.group0()).with_w(-(wedge_g0_xyz[0] * other[e23]) - (wedge_g0_xyz[1] * other[e31]) - (wedge_g0_xyz[2] * other[e12])),
            // e15, e25, e35, e3215
            (Simd32x3::from(wedge_g0_w) * other.group1()).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<AntiMotor> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       10        0        0
    //    simd3        1        4        0      N/A
    // Totals...
    // yes simd        7       14        0      N/A
    //  no simd        9       22        0        0
    fn project_via_origin_onto(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(self[scalar] * -1.0) * other.group0().xyz();
        let wedge_g0_w = (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[scalar] * self[scalar]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(wedge_g0_w) * other.group0().xyz())
                .with_w((wedge_g0_w * other[scalar]) - (wedge_g0_xyz[0] * other[e23]) - (wedge_g0_xyz[1] * other[e31]) - (wedge_g0_xyz[2] * other[e12])),
            // e15, e25, e35, e3215
            ((wedge_g0_xyz * Simd32x3::from(other[e3215])) + (Simd32x3::from(wedge_g0_w) * other.group1().xyz())).with_w(wedge_g0_w * other[e3215]),
        )
    }
}
impl ProjectViaOriginOnto<AntiPlane> for AntiMotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2       11        0        0
    fn project_via_origin_onto(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(/* scalar */ (wedge_g0[0] * other[e1]) + (wedge_g0[1] * other[e2]) + (wedge_g0[2] * other[e3]))
    }
}
impl ProjectViaOriginOnto<AntiScalar> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       11        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e12345] * -1.0;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Circle> for AntiMotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        6       10        0      N/A
    // Totals...
    // yes simd        8       13        0      N/A
    //  no simd       20       33        0        0
    fn project_via_origin_onto(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let wedge_g2_w = -(other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12]);
        let wedge_g3_xyz = (other.group0().yzx() * self.group1().zxy()) - (Simd32x3::from(other[e321]) * self.group0().xyz()) - (other.group0().zxy() * self.group1().yzx());
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(wedge_g2_w) * other.group1().xyz()) + (wedge_g3_xyz.yzx() * other.group0().zxy()) - (wedge_g3_xyz.zxy() * other.group0().yzx()),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g2_w) * other.group2()) - (wedge_g3_xyz * Simd32x3::from(other[e321]))).with_w(0.0),
            // e15, e25, e35, scalar
            ((wedge_g3_xyz.zxy() * other.group2().yzx()) - (wedge_g3_xyz.yzx() * other.group2().zxy())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<CircleRotor> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1       14        0        0
    //    simd2        1        2        0      N/A
    //    simd3       10       13        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       13       30        0      N/A
    //  no simd       37       61        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_w = other[e321] * -1.0;
        let right_dual_g2_w = other[e12345] * -1.0;
        let wedge_g3 = ((Simd32x3::from(right_dual_g1_w) * self.group0().xyz()) + (other.group0().yzx() * self.group1().zxy()) - (other.group0().zxy() * self.group1().yzx()))
            .with_w(right_dual_g2_w * self[e3215]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x3::from(self[scalar] * other[e12345]) * other.group0()) + (other.group0().zxy() * wedge_g3.yzx()) - (other.group0().yzx() * wedge_g3.zxy()))
                .with_w(right_dual_g2_w * self[scalar] * other[e12345]),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g3[3]) * other.group0())
                + (Simd32x3::from(right_dual_g2_w * other[e12345]) * self.group0().xyz())
                + (Simd32x3::from(self[scalar] * other[e12345]) * other.group1().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g3.xyz()))
            .with_w(right_dual_g1_w * self[scalar] * other[e12345]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, (wedge_g3[1] * other[e235]) - (wedge_g3[0] * other[e315]), 0.0])
                + ((Simd32x3::from(wedge_g3[3]) * other.group1().xyz())
                    + (Simd32x3::from(right_dual_g2_w * other[e12345]) * self.group1().xyz())
                    + (Simd32x3::from(self[scalar] * other[e12345]) * other.group2().xyz())
                    + ((wedge_g3.zx() * other.group2().yz()) - (wedge_g3.yz() * other.group2().zx())).with_z(0.0))
                .with_w(0.0),
            // e4235, e4315, e4125, e3215
            wedge_g3 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Dipole> for AntiMotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       14       17        0        0
    //    simd3        0        5        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd       14       25        0      N/A
    //  no simd       14       44        0        0
    fn project_via_origin_onto(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g0 = right_dual_g0 * Simd32x3::from(self[scalar]);
        let wedge_g1 = right_dual_g1 * Simd32x4::from(self[scalar]);
        let wedge_g2_xyz = Simd32x3::from(self[scalar] * -1.0) * other.group2();
        let wedge_g2_w = -(right_dual_g0[0] * self[e15])
            - (right_dual_g0[1] * self[e25])
            - (right_dual_g0[2] * self[e35])
            - (right_dual_g1[0] * self[e23])
            - (right_dual_g1[1] * self[e31])
            - (right_dual_g1[2] * self[e12]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g2_w) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(wedge_g2_w) * other.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(wedge_g2_w) * other.group2()).with_w(
                -(wedge_g0[0] * other[e15])
                    - (wedge_g0[1] * other[e25])
                    - (wedge_g0[2] * other[e35])
                    - (wedge_g2_xyz[0] * other[e41])
                    - (wedge_g2_xyz[1] * other[e42])
                    - (wedge_g2_xyz[2] * other[e43])
                    - (wedge_g1[0] * other[e23])
                    - (wedge_g1[1] * other[e31])
                    - (wedge_g1[2] * other[e12])
                    - (wedge_g1[3] * other[e45]),
            ),
        )
    }
}
impl ProjectViaOriginOnto<DipoleInversion> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       15       24        0        0
    //    simd3       13       19        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       28       45        0      N/A
    //  no simd       54       89        0        0
    fn project_via_origin_onto(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = (Simd32x3::from(other[e1234]) * self.group0().xyz()) - (Simd32x3::from(self[scalar]) * other.group0());
        let wedge_g0_w = self[e3215] * other[e1234];
        let wedge_g1_xyz = (Simd32x3::from(other[e1234]) * self.group1().xyz()) - (Simd32x3::from(self[scalar]) * other.group1().xyz());
        let wedge_g1_w = self[scalar] * other[e45];
        let wedge_g2_xyz = Simd32x3::from([
            (self[e25] * other[e4125]) - (self[e35] * other[e4315]),
            (self[e35] * other[e4235]) - (self[e15] * other[e4125]),
            (self[e15] * other[e4315]) - (self[e25] * other[e4235]),
        ]) + (Simd32x3::from(other[e3215]) * self.group0().xyz())
            - (Simd32x3::from(self[scalar]) * other.group2().xyz());
        let wedge_g3 = Simd32x4::from(self[scalar]) * (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((wedge_g1_xyz * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0_w) * other.group0()) + (wedge_g0_xyz.zxy() * other.group3().yzx())
                - (wedge_g0_xyz.yzx() * other.group3().zxy()))
            .with_w(
                (wedge_g3[0] * other[e4235]) + (wedge_g3[1] * other[e4315]) + (self[scalar] * other[e1234] * other[e3215])
                    - (wedge_g1_w * other[e45])
                    - (wedge_g0_xyz[0] * other[e15])
                    - (wedge_g0_xyz[1] * other[e25])
                    - (wedge_g0_xyz[2] * other[e35])
                    - (wedge_g1_xyz[0] * other[e23])
                    - (wedge_g1_xyz[1] * other[e31])
                    - (wedge_g1_xyz[2] * other[e12])
                    - (wedge_g2_xyz[0] * other[e41])
                    - (wedge_g2_xyz[1] * other[e42])
                    - (wedge_g2_xyz[2] * other[e43]),
            ),
            // e23, e31, e12, e45
            ((wedge_g0_xyz * Simd32x3::from(other[e3215])) + (wedge_g2_xyz * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0_w) * other.group1().xyz())
                - (Simd32x3::from(wedge_g1_w) * other.group3().xyz()))
            .with_w(wedge_g0_w * other[e45]),
            // e15, e25, e35, e1234
            ((wedge_g1_xyz * Simd32x3::from(other[e3215])) + (Simd32x3::from(wedge_g0_w) * other.group2().xyz()) + (wedge_g2_xyz.yzx() * other.group3().zxy())
                - (wedge_g2_xyz.zxy() * other.group3().yzx()))
            .with_w(wedge_g0_w * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0_w) * other.group3(),
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        9        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1       12        0      N/A
    //  no simd        1       19        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group0(),
            // e15, e25, e35, e3215
            (self.group1().xyz() * Simd32x2::from(other[e12345] * other[e12345] * -1.0).with_z(other[e12345] * other[e12345]) * Simd32x3::from([1.0, 1.0, -1.0]))
                .with_w(-(other[e12345] * other[e12345] * self[e3215]) - (other[e5] * other[e12345] * self[scalar])),
        )
    }
}
impl ProjectViaOriginOnto<FlatPoint> for AntiMotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn project_via_origin_onto(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * other[e45] * other[e45] * -1.0)
    }
}
impl ProjectViaOriginOnto<Flector> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       12        0        0
    //    simd3        3        5        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        9       19        0      N/A
    //  no simd       15       35        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g0_xyz = Simd32x3::from([
            (right_dual_g1[1] * self[e35]) - (right_dual_g1[2] * self[e25]),
            (right_dual_g1[2] * self[e15]) - (right_dual_g1[0] * self[e35]),
            (right_dual_g1[0] * self[e25]) - (right_dual_g1[1] * self[e15]),
        ]) + (Simd32x3::from(right_dual_g1[3]) * self.group0().xyz())
            - (Simd32x3::from(self[scalar]) * other.group0().xyz());
        let wedge_g0_w = self[scalar] * other[e45];
        let wedge_g1 = right_dual_g1 * Simd32x4::from(self[scalar]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(wedge_g0_w * -1.0) * other.group1().xyz())
                .with_w((wedge_g1[0] * other[e4235]) + (wedge_g1[1] * other[e4315]) + (wedge_g1[2] * other[e4125]) - (wedge_g0_w * other[e45])),
            // e15, e25, e35, e3215
            ((wedge_g0_xyz.yzx() * other.group1().zxy()) - (wedge_g0_xyz.zxy() * other.group1().yzx())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<Line> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        5       10        0      N/A
    // Totals...
    // yes simd        7       16        0      N/A
    //  no simd       17       36        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(self[scalar]) * other.group0();
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(wedge_g0_xyz[0] * other[e415]) - (wedge_g0_xyz[1] * other[e425]) - (wedge_g0_xyz[2] * other[e435])),
            // e15, e25, e35, e3215
            (-(Simd32x3::from(other[e235] * self[e23]) * other.group0())
                - (Simd32x3::from(other[e315] * self[e31]) * other.group0())
                - (Simd32x3::from(other[e125] * self[e12]) * other.group0())
                - (other.group0() * other.group0() * self.group1().xyz())
                - (Simd32x3::from(other[e415]) * other.group0().yyz() * self.group1().yxx())
                - (Simd32x3::from(other[e435]) * other.group0().xyy() * self.group1().zzy()))
            .with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       14        0        0
    //    simd3        2        5        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       12       21        0      N/A
    //  no simd       22       37        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let right_dual_g1 = other.group1().xyz().with_w(other[e5] * -1.0);
        let wedge_g0_xyz = (right_dual_g0_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_dual_g0_w) * self.group0().xyz());
        let wedge_g1 = (right_dual_g1 * Simd32x4::from(self[scalar]))
            + (Simd32x4::from(right_dual_g0_w) * self.group1())
            + Simd32x3::from(0.0).with_w(
                -(right_dual_g0_xyz[0] * self[e15])
                    - (right_dual_g0_xyz[1] * self[e25])
                    - (right_dual_g0_xyz[2] * self[e35])
                    - (right_dual_g1[0] * self[e23])
                    - (right_dual_g1[1] * self[e31])
                    - (right_dual_g1[2] * self[e12]),
            );
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (wedge_g0_xyz * Simd32x4::from(other[e12345]).xyz())
                .with_w((right_dual_g0_w * self[scalar] * other[e12345]) - (wedge_g0_xyz[0] * other[e415]) - (wedge_g0_xyz[1] * other[e425]) - (wedge_g0_xyz[2] * other[e435])),
            // e15, e25, e35, e3215
            ((Simd32x3::from(wedge_g1[3]) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g1.xyz())).with_w(wedge_g1[3] * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for AntiMotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       62       82        0        0
    //    simd2        0        2        0      N/A
    //    simd3       46       61        0      N/A
    //    simd4        9       11        0      N/A
    // Totals...
    // yes simd      117      156        0      N/A
    //  no simd      236      313        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1 = (other.group9().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let right_dual_g3_w = other[e321] * -1.0;
        let right_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let wedge_g0_y = (right_dual_g0[1] * self[scalar]) + (right_dual_g1[3] * self[e3215])
            - (right_dual_g6_xyz[0] * self[e23])
            - (right_dual_g6_xyz[1] * self[e31])
            - (right_dual_g6_xyz[2] * self[e12])
            - (right_dual_g7[0] * self[e15])
            - (right_dual_g7[1] * self[e25])
            - (right_dual_g7[2] * self[e35]);
        let wedge_g1 = right_dual_g1 * Simd32x4::from(self[scalar]);
        let wedge_g2 = self[scalar] * other[e3215];
        let wedge_g3 = ((Simd32x3::from(right_dual_g0[0]) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * other.group8())).with_w(right_dual_g3_w * self[scalar]);
        let wedge_g4 = Simd32x3::from(self[scalar]) * other.group7();
        let wedge_g5 = (Simd32x3::from(right_dual_g0[0]) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * other.group6().xyz());
        let wedge_g6 = ((right_dual_g6_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_dual_g1[3]) * self.group1().xyz())).with_w(self[scalar] * other[e45]);
        let wedge_g7 = (right_dual_g7 * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_dual_g1[3]) * self.group0().xyz());
        let wedge_g8 = Simd32x3::from([
            (right_dual_g1[1] * self[e35]) - (right_dual_g1[2] * self[e25]),
            (right_dual_g1[2] * self[e15]) - (right_dual_g1[0] * self[e35]),
            (right_dual_g1[0] * self[e25]) - (right_dual_g1[1] * self[e15]),
        ]) + (Simd32x3::from(other[e3215]) * self.group0().xyz())
            - (Simd32x3::from(self[scalar]) * other.group3().xyz());
        let wedge_g9 =
            ((Simd32x3::from(right_dual_g3_w) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * other.group1().xyz()) + (other.group7().yzx() * self.group1().zxy())
                - (other.group7().zxy() * self.group1().yzx()))
            .with_w(right_dual_g0[0] * self[e3215]);
        let wedge_g10 = -(other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12]) - (self[scalar] * other[e4]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar])
                    + (wedge_g10 * other[e5])
                    + (wedge_g2 * other[e1234])
                    + (wedge_g1[0] * other[e4235])
                    + (wedge_g1[1] * other[e4315])
                    + (wedge_g1[2] * other[e4125])
                    + (wedge_g1[3] * other[e3215])
                    + (wedge_g9[0] * other[e1])
                    + (wedge_g9[1] * other[e2])
                    + (wedge_g9[2] * other[e3])
                    + (wedge_g9[3] * other[e4])
                    + (right_dual_g0[0] * other[e12345] * self[scalar])
                    - (wedge_g4[0] * other[e235])
                    - (wedge_g4[1] * other[e315])
                    - (wedge_g4[2] * other[e125])
                    - (wedge_g5[0] * other[e415])
                    - (wedge_g5[1] * other[e425])
                    - (wedge_g5[2] * other[e435])
                    - (wedge_g7[0] * other[e15])
                    - (wedge_g7[1] * other[e25])
                    - (wedge_g7[2] * other[e35])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43])
                    - (other[e23] * wedge_g6[0])
                    - (other[e31] * wedge_g6[1])
                    - (other[e12] * wedge_g6[2])
                    - (other[e423] * wedge_g3[0])
                    - (other[e431] * wedge_g3[1])
                    - (other[e412] * wedge_g3[2])
                    - (wedge_g3[3] * other[e321])
                    - (wedge_g6[3] * other[e45]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            (wedge_g1 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group1())
                + (other.group9().yzxx() * wedge_g5.zxy().with_w(wedge_g4[0]))
                + ((Simd32x3::from(wedge_g6[3]) * other.group6().xyz())
                    + (Simd32x3::from(wedge_g9[3]) * other.group4())
                    + (Simd32x3::from(other[e321]) * wedge_g6.xyz())
                    + (Simd32x3::from(other[e1234]) * wedge_g3.xyz())
                    + (wedge_g7.zxy() * other.group8().yzx())
                    + (wedge_g8.yzx() * other.group7().zxy())
                    + (other.group5().yzx() * wedge_g9.zxy())
                    - (wedge_g4 * Simd32x3::from(other[e3215]))
                    - (Simd32x3::from(wedge_g10) * other.group3().xyz())
                    - (wedge_g5.yzx() * other.group9().zxy())
                    - (wedge_g7.yzx() * other.group8().zxy())
                    - (wedge_g8.zxy() * other.group7().yzx())
                    - (other.group5().zxy() * wedge_g9.yzx()))
                .with_w((wedge_g4[1] * other[e4315]) + (wedge_g4[2] * other[e4125]) + (wedge_g3[3] * other[e1234])),
            // e5
            (wedge_g0_y * other[e5])
                + (wedge_g2 * other[e12345])
                + (wedge_g9[0] * other[e15])
                + (wedge_g9[1] * other[e25])
                + (wedge_g9[2] * other[e35])
                + (wedge_g9[3] * other[e45])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435])
                - (other[e235] * wedge_g6[0])
                - (other[e315] * wedge_g6[1])
                - (other[e125] * wedge_g6[2])
                - (wedge_g3[0] * other[e4235])
                - (wedge_g3[1] * other[e4315])
                - (wedge_g3[2] * other[e4125])
                - (wedge_g3[3] * other[e3215]),
            // e15, e25, e35, e45
            (wedge_g3 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group3())
                + ((Simd32x3::from(wedge_g9[3]) * other.group6().xyz())
                    + (Simd32x3::from(other[e3215]) * wedge_g6.xyz())
                    + (wedge_g8.yzx() * other.group9().zxy())
                    + (other.group8().yzx() * wedge_g9.zxy())
                    - (wedge_g8.zxy() * other.group9().yzx())
                    - (other.group8().zxy() * wedge_g9.yzx()))
                .with_w(0.0),
            // e41, e42, e43
            (wedge_g4 * Simd32x3::from(other[e12345]))
                + (Simd32x3::from(wedge_g0_y) * other.group4())
                + (Simd32x3::from(wedge_g10) * other.group6().xyz())
                + (Simd32x3::from(other[e1234]) * wedge_g6.xyz())
                + (wedge_g7.zxy() * other.group9().yzx())
                + (other.group7().zxy() * wedge_g9.yzx())
                - (wedge_g7.yzx() * other.group9().zxy())
                - (other.group7().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (wedge_g5 * Simd32x3::from(other[e12345]))
                + (wedge_g7 * Simd32x3::from(other[e3215]))
                + (wedge_g8 * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(wedge_g0_y) * other.group5())
                + (Simd32x3::from(wedge_g10) * other.group8())
                + (Simd32x3::from(wedge_g9[3]) * other.group7())
                - (Simd32x3::from(wedge_g6[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            (wedge_g6 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group6())
                + (wedge_g9.yzxw() * other.group9().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group9().yz()).with_zw(wedge_g9[1] * other[e4235] * -1.0, wedge_g10 * other[e3215] * -1.0),
            // e423, e431, e412
            (wedge_g7 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group7()) + (Simd32x3::from(other[e1234]) * wedge_g9.xyz())
                - (Simd32x3::from(wedge_g10) * other.group9().xyz()),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (wedge_g9 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group9()),
            // e1234
            (wedge_g0_y * other[e1234]) + (wedge_g10 * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        7        0        0
    //    simd2        2        4        0      N/A
    //    simd3        1        3        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        9       16        0      N/A
    //  no simd       19       32        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g0 = Simd32x4::from([0.0, 0.0, (right_dual_g0[0] * self[e25]) - (right_dual_g0[1] * self[e15]), 0.0])
            + ((Simd32x3::from(right_dual_g0[3]) * self.group0().xyz()) + ((right_dual_g0.yz() * self.group1().zx()) - (right_dual_g0.zx() * self.group1().yz())).with_z(0.0))
                .with_w(0.0);
        let wedge_g1 = right_dual_g0 * Simd32x4::from(self[scalar]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x4::from(wedge_g0[3]).xyz() * other.group0().xyz() * Simd32x3::from(-1.0))
                .with_w((wedge_g1[0] * other[e4235]) + (wedge_g1[1] * other[e4315]) + (wedge_g1[2] * other[e4125])),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, (wedge_g0[0] * other[e4315]) - (wedge_g0[1] * other[e4235]), 0.0])
                + ((wedge_g0.yz() * other.group0().zx()) - (wedge_g0.zx() * other.group0().yz())).with_zw(0.0, 0.0),
        )
    }
}
impl ProjectViaOriginOnto<RoundPoint> for AntiMotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        4       14        0        0
    fn project_via_origin_onto(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(
            // scalar
            (wedge_g0[0] * other[e1]) + (wedge_g0[1] * other[e2]) + (wedge_g0[2] * other[e3]) + (wedge_g0[3] * other[e4]) - (self[scalar] * other[e4] * other[e5]),
        )
    }
}
impl ProjectViaOriginOnto<Scalar> for AntiMotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn project_via_origin_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * other[scalar] * other[scalar])
    }
}
impl ProjectViaOriginOnto<Sphere> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       13        0        0
    //    simd2        0        1        0      N/A
    //    simd3        8       12        0      N/A
    //    simd4        1        4        0      N/A
    // Totals...
    // yes simd       12       30        0      N/A
    //  no simd       31       67        0        0
    fn project_via_origin_onto(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let wedge_g0 = Simd32x4::from(right_dual_g0[3]) * self.group0().xyz().with_w(self[e3215]);
        let wedge_g1_xyz = Simd32x3::from(right_dual_g0[3]) * self.group1().xyz();
        let wedge_g2_xyz = Simd32x3::from([
            (right_dual_g0[1] * self[e35]) - (right_dual_g0[2] * self[e25]),
            (right_dual_g0[2] * self[e15]) - (right_dual_g0[0] * self[e35]),
            (right_dual_g0[0] * self[e25]) - (right_dual_g0[1] * self[e15]),
        ]) + (Simd32x3::from(other[e3215]) * self.group0().xyz());
        let wedge_g3 = right_dual_g0 * Simd32x4::from(self[scalar]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0().yzxy() * wedge_g0.zxy().with_w(wedge_g3[1]))
                + ((wedge_g1_xyz * Simd32x3::from(other[e1234])) + -(wedge_g0.yz() * other.group0().zx()).with_z(wedge_g0[0] * other[e4315] * -1.0))
                    .with_w(wedge_g3[0] * other[e4235]),
            // e23, e31, e12, e45
            ((wedge_g2_xyz * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(other[e3215]) * wedge_g0.xyz())
                + (Simd32x3::from(right_dual_g0[0] * self[e23]) * other.group0().xyz())
                + (Simd32x3::from(right_dual_g0[1] * self[e31]) * other.group0().xyz())
                + (Simd32x3::from(right_dual_g0[2] * self[e12]) * other.group0().xyz()))
            .with_w(0.0),
            // e15, e25, e35, e1234
            ((wedge_g1_xyz * Simd32x3::from(other[e3215])) + (wedge_g2_xyz.yzx() * other.group0().zxy()) - (wedge_g2_xyz.zxy() * other.group0().yzx()))
                .with_w(wedge_g0[3] * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0[3]) * other.group0(),
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       15       32        0        0
    //    simd2        0        1        0      N/A
    //    simd3        9       12        0      N/A
    //    simd4        3        5        0      N/A
    // Totals...
    // yes simd       27       50        0      N/A
    //  no simd       54       90        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e12345] * -1.0);
        let right_dual_g1_w = other[e321] * -1.0;
        let wedge_g0 = right_dual_g0 * Simd32x4::from(self[scalar]);
        let wedge_g1_xyz = (Simd32x3::from(right_dual_g0[3]) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * other.group1().xyz());
        let wedge_g1_w = right_dual_g1_w * self[scalar];
        let wedge_g2_xyz = (Simd32x3::from(right_dual_g0[3]) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * other.group2().xyz());
        let wedge_g2_w = self[scalar] * other[e4] * -1.0;
        let wedge_g3 = (Simd32x3::from([
            (right_dual_g0[1] * self[e35]) - (right_dual_g0[2] * self[e25]),
            (right_dual_g0[2] * self[e15]) - (right_dual_g0[0] * self[e35]),
            (right_dual_g0[0] * self[e25]) - (right_dual_g0[1] * self[e15]),
        ]) + (Simd32x3::from(right_dual_g1_w) * self.group0().xyz())
            + (Simd32x3::from(self[scalar]) * other.group3().xyz()))
        .with_w(right_dual_g0[3] * self[e3215]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (wedge_g0 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g2_w) * other.group1().xyz().with_w(other[e5]))
                + (wedge_g3.yzxx() * other.group0().zxy().with_w(other[e1]))
                + -(wedge_g3.zx() * other.group0().yz()).with_zw(
                    wedge_g3[1] * other[e423] * -1.0,
                    -(wedge_g1_w * other[e321])
                        - (wedge_g1_xyz[0] * other[e415])
                        - (wedge_g1_xyz[1] * other[e425])
                        - (wedge_g1_xyz[2] * other[e435])
                        - (wedge_g2_xyz[0] * other[e423])
                        - (wedge_g2_xyz[1] * other[e431])
                        - (wedge_g2_xyz[2] * other[e412])
                        - (wedge_g0[0] * other[e235])
                        - (wedge_g0[1] * other[e315])
                        - (wedge_g0[2] * other[e125]),
                ),
            // e23, e31, e12, e45
            ((wedge_g1_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g2_w) * other.group2().xyz()) + (Simd32x3::from(wedge_g3[3]) * other.group0().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g3.xyz()))
            .with_w(wedge_g1_w * other[e12345]),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g3[2] * other[e315]) - (wedge_g3[1] * other[e125]),
                (wedge_g3[0] * other[e125]) - (wedge_g3[2] * other[e235]),
                (wedge_g3[1] * other[e235]) - (wedge_g3[0] * other[e315]),
            ]) + (wedge_g2_xyz * Simd32x3::from(other[e12345]))
                + (Simd32x3::from(wedge_g3[3]) * other.group1().xyz()))
            .with_w(wedge_g2_w * other[e12345]),
            // e4235, e4315, e4125, e3215
            wedge_g3 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<VersorOdd> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       16       25        0        0
    //    simd2        0        1        0      N/A
    //    simd3       12       17        0      N/A
    //    simd4        3        5        0      N/A
    // Totals...
    // yes simd       31       48        0      N/A
    //  no simd       64       98        0        0
    fn project_via_origin_onto(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = (right_dual_g0 * Simd32x4::from(self[scalar]))
            + (self.group0().xyzx() * Simd32x3::from(other[e1234]).with_w(other[e23]))
            + Simd32x3::from(0.0).with_w(
                (self[e31] * other[e31]) + (self[e12] * other[e12]) + (self[e3215] * other[e1234])
                    - (right_dual_g0[0] * self[e15])
                    - (right_dual_g0[1] * self[e25])
                    - (right_dual_g0[2] * self[e35]),
            );
        let wedge_g1_xyz = (Simd32x3::from(other[e1234]) * self.group1().xyz()) - (Simd32x3::from(self[scalar]) * other.group1().xyz());
        let wedge_g1_w = self[scalar] * other[e45];
        let wedge_g2_xyz = (Simd32x3::from(other[e3215]) * self.group0().xyz()) + (right_dual_g3_xyz.yzx() * self.group1().zxy())
            - (Simd32x3::from(self[scalar]) * other.group2().xyz())
            - (right_dual_g3_xyz.zxy() * self.group1().yzx());
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group3().yzxx() * wedge_g0.zxy().with_w(right_dual_g3_xyz[0] * self[scalar]))
                + ((wedge_g1_xyz * Simd32x3::from(other[e1234]))
                    + (Simd32x3::from(wedge_g0[3]) * other.group0().xyz())
                    + -(wedge_g0.yz() * other.group3().zx()).with_z(wedge_g0[0] * other[e4315] * -1.0))
                .with_w(
                    (wedge_g0[3] * other[scalar]) + (self[scalar] * other[e1234] * other[e3215])
                        - (wedge_g1_w * other[e45])
                        - (wedge_g1_xyz[0] * other[e23])
                        - (wedge_g1_xyz[1] * other[e31])
                        - (wedge_g1_xyz[2] * other[e12])
                        - (wedge_g2_xyz[0] * other[e41])
                        - (wedge_g2_xyz[1] * other[e42])
                        - (wedge_g2_xyz[2] * other[e43])
                        - (wedge_g0[0] * other[e15])
                        - (wedge_g0[1] * other[e25])
                        - (wedge_g0[2] * other[e35]),
                ),
            // e23, e31, e12, e45
            ((wedge_g2_xyz * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0[3]) * other.group1().xyz()) + (Simd32x3::from(other[e3215]) * wedge_g0.xyz())
                - (Simd32x3::from(wedge_g1_w) * other.group3().xyz()))
            .with_w(wedge_g0[3] * other[e45]),
            // e15, e25, e35, e1234
            ((wedge_g1_xyz * Simd32x3::from(other[e3215])) + (Simd32x3::from(wedge_g0[3]) * other.group2().xyz()) + (wedge_g2_xyz.yzx() * other.group3().zxy())
                - (wedge_g2_xyz.zxy() * other.group3().yzx()))
            .with_w(wedge_g0[3] * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0[3]) * other.group3(),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for AntiPlane {
    type Output = ProjectViaOriginOntoInfixPartial<AntiPlane>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiCircleRotor> for AntiPlane {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       15        0        0
    //    simd3        6        8        0      N/A
    // Totals...
    // yes simd       12       23        0      N/A
    //  no simd       24       39        0        0
    fn project_via_origin_onto(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = Simd32x3::from([
            (other[e12] * self[e2]) - (other[e31] * self[e3]),
            (other[e23] * self[e3]) - (other[e12] * self[e1]),
            (other[e31] * self[e1]) - (other[e23] * self[e2]),
        ]) - (right_dual_g0 * Simd32x3::from(self[e5]));
        let wedge_g0_w = other[e45] * self[e5] * -1.0;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g0_w) * other.group0()) + (wedge_g0_xyz.zxy() * other.group1().yzx())
                - (Simd32x3::from(right_dual_g0[0] * self[e1]) * other.group2().xyz())
                - (Simd32x3::from(right_dual_g0[1] * self[e2]) * other.group2().xyz())
                - (Simd32x3::from(right_dual_g0[2] * self[e3]) * other.group2().xyz())
                - (wedge_g0_xyz.yzx() * other.group1().zxy()))
            .with_w(0.0),
            // e5
            (wedge_g0_w * other[e45]) + (wedge_g0_xyz[0] * other[e15]) + (wedge_g0_xyz[1] * other[e25]) + (wedge_g0_xyz[2] * other[e35]),
        )
    }
}
impl ProjectViaOriginOnto<AntiDipoleInversion> for AntiPlane {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        8       14        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd       15       25        0      N/A
    //  no simd       34       59        0        0
    fn project_via_origin_onto(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2_xyz = other.group2().xyz();
        let right_dual_g3_xyz = other.group3().xyz();
        let wedge_g0 = (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx());
        let wedge_g1_xyz = (Simd32x3::from(other[e321]) * self.group0().xyz()) + (Simd32x3::from(self[e5]) * other.group0());
        let wedge_g2 = (self.group0().yzxx() * right_dual_g2_xyz.zxy().with_w(right_dual_g3_xyz[0]))
            + ((Simd32x3::from(self[e5]) * other.group1().xyz()) - (right_dual_g2_xyz.yzx() * self.group0().zxy())).with_w(right_dual_g3_xyz[1] * self[e2]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(wedge_g2[3]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(wedge_g2[3]) * other.group1(),
            // e235, e315, e125, e4
            (Simd32x4::from(wedge_g2[3]).xyz() * other.group2().xyz()).with_w(
                (wedge_g2[3] * other[e4])
                    - (wedge_g0[0] * other[e415])
                    - (wedge_g0[1] * other[e425])
                    - (wedge_g0[2] * other[e435])
                    - (wedge_g1_xyz[0] * other[e423])
                    - (wedge_g1_xyz[1] * other[e431])
                    - (wedge_g1_xyz[2] * other[e412]),
            ),
            // e1, e2, e3, e5
            ((wedge_g1_xyz * Simd32x3::from(other[e321]))
                + (Simd32x3::from(wedge_g2[3]) * other.group3().xyz())
                + (wedge_g0.zxy() * other.group2().yzx())
                + (other.group0().zxy() * wedge_g2.yzx())
                - (wedge_g0.yzx() * other.group2().zxy())
                - (other.group0().yzx() * wedge_g2.zxy()))
            .with_w(wedge_g2[3] * other[e5]),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlatPoint> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        9        0        0
    fn project_via_origin_onto(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(other[e321]) * self.group0().xyz();
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (wedge_g0 * Simd32x4::from(other[e321]).xyz()).with_w(-(wedge_g0[0] * other[e235]) - (wedge_g0[1] * other[e315]) - (wedge_g0[2] * other[e125])),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlector> for AntiPlane {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd3        1        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        5       15        0        0
    fn project_via_origin_onto(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0_w = (right_dual_g1_xyz[0] * self[e1]) + (right_dual_g1_xyz[1] * self[e2]) + (right_dual_g1_xyz[2] * self[e3]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(wedge_g0_w) * other.group0(),
            // e1, e2, e3, e5
            ((Simd32x3::from(wedge_g0_w) * other.group1().xyz()) + (Simd32x3::from(other[e321] * other[e321]) * self.group0().xyz())).with_w(wedge_g0_w * other[e5]),
        )
    }
}
impl ProjectViaOriginOnto<AntiLine> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        6       16        0        0
    fn project_via_origin_onto(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = (right_dual_g0.yzx() * self.group0().zxy()) - (right_dual_g0.zxy() * self.group0().yzx());
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            ((wedge_g0_xyz.zxy() * other.group0().yzx()) - (wedge_g0_xyz.yzx() * other.group0().zxy())).with_w(wedge_g0_xyz[0] * other[e15]),
        )
    }
}
impl ProjectViaOriginOnto<AntiMotor> for AntiPlane {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd2        2        4        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        5       11        0      N/A
    //  no simd       10       19        0        0
    fn project_via_origin_onto(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g1 = Simd32x4::from([0.0, 0.0, (other[e31] * self[e1]) - (other[e23] * self[e2]), 0.0])
            + ((other.group0().zx() * self.group0().yz()) - (other.group0().yz() * self.group0().zx())).with_zw(0.0, 0.0);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (wedge_g1.xyz() * Simd32x4::from(other[e3215]).xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e1, e2, e3, e5
            ((wedge_g1.zx() * other.group0().yz()) - (wedge_g1.yz() * other.group0().zx()))
                .with_zw((wedge_g1[1] * other[e23]) - (wedge_g1[0] * other[e31]), wedge_g1[0] * other[e15]),
        )
    }
}
impl ProjectViaOriginOnto<AntiPlane> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        8       15        0        0
    fn project_via_origin_onto(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x4::from(right_dual_g0_xyz[0] * self[e1]) * other.group0())
                + (Simd32x4::from(right_dual_g0_xyz[1] * self[e2]) * other.group0())
                + (Simd32x4::from(right_dual_g0_xyz[2] * self[e3]) * other.group0()),
        )
    }
}
impl ProjectViaOriginOnto<AntiScalar> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group0())
    }
}
impl ProjectViaOriginOnto<Circle> for AntiPlane {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        8       12        0      N/A
    // Totals...
    // yes simd       13       18        0      N/A
    //  no simd       29       42        0        0
    fn project_via_origin_onto(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx());
        let wedge_g1_xyz = (Simd32x3::from(self[e5]) * other.group0()) + (Simd32x3::from(other[e321]) * self.group0().xyz());
        let wedge_g2 = (Simd32x3::from(self[e5]) * other.group1().xyz()) + (other.group2().zxy() * self.group0().yzx()) - (other.group2().yzx() * self.group0().zxy());
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            ((wedge_g1_xyz * Simd32x3::from(other[e321])) + (wedge_g0.zxy() * other.group2().yzx()) + (wedge_g2.yzx() * other.group0().zxy())
                - (wedge_g0.yzx() * other.group2().zxy())
                - (wedge_g2.zxy() * other.group0().yzx()))
            .with_w(0.0),
            // e5
            -(wedge_g1_xyz[0] * other[e235])
                - (wedge_g1_xyz[1] * other[e315])
                - (wedge_g1_xyz[2] * other[e125])
                - (wedge_g2[0] * other[e415])
                - (wedge_g2[1] * other[e425])
                - (wedge_g2[2] * other[e435]),
        )
    }
}
impl ProjectViaOriginOnto<CircleRotor> for AntiPlane {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        8        0        0
    //    simd3       10       16        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       15       26        0      N/A
    //  no simd       35       64        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2_xyz = other.group2().xyz();
        let wedge_g0 = (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx());
        let wedge_g1 = ((Simd32x3::from(self[e5]) * other.group0()) + (Simd32x3::from(other[e321]) * self.group0().xyz())).with_w(0.0);
        let wedge_g2_xyz = (Simd32x3::from(self[e5]) * other.group1().xyz()) + (right_dual_g2_xyz.zxy() * self.group0().yzx()) - (right_dual_g2_xyz.yzx() * self.group0().zxy());
        let wedge_g3 = Simd32x4::from(other[e12345] * -1.0) * self.group0();
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            wedge_g0 * Simd32x3::from(other[e12345]),
            // e415, e425, e435, e321
            wedge_g1 * Simd32x4::from(other[e12345]),
            // e235, e315, e125, e4
            (wedge_g2_xyz * Simd32x4::from(other[e12345]).xyz()).with_w(
                -(wedge_g0[0] * other[e415])
                    - (wedge_g0[1] * other[e425])
                    - (wedge_g0[2] * other[e435])
                    - (other[e423] * wedge_g1[0])
                    - (other[e431] * wedge_g1[1])
                    - (other[e412] * wedge_g1[2]),
            ),
            // e1, e2, e3, e5
            ((Simd32x3::from(wedge_g1[3]) * other.group1().xyz())
                + (Simd32x3::from(other[e321]) * wedge_g1.xyz())
                + (Simd32x3::from(other[e12345]) * wedge_g3.xyz())
                + (wedge_g0.zxy() * other.group2().yzx())
                + (wedge_g2_xyz.yzx() * other.group0().zxy())
                - (wedge_g0.yzx() * other.group2().zxy())
                - (wedge_g2_xyz.zxy() * other.group0().yzx()))
            .with_w(wedge_g3[3] * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Dipole> for AntiPlane {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       15        0        0
    //    simd3        6        8        0      N/A
    // Totals...
    // yes simd       12       23        0      N/A
    //  no simd       24       39        0        0
    fn project_via_origin_onto(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = Simd32x3::from([
            (self[e2] * other[e12]) - (self[e3] * other[e31]),
            (self[e3] * other[e23]) - (self[e1] * other[e12]),
            (self[e1] * other[e31]) - (self[e2] * other[e23]),
        ]) - (right_dual_g0 * Simd32x3::from(self[e5]));
        let wedge_g0_w = self[e5] * other[e45] * -1.0;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g0_w) * other.group0()) + (wedge_g0_xyz.zxy() * other.group1().yzx())
                - (Simd32x3::from(right_dual_g0[0] * self[e1]) * other.group2())
                - (Simd32x3::from(right_dual_g0[1] * self[e2]) * other.group2())
                - (Simd32x3::from(right_dual_g0[2] * self[e3]) * other.group2())
                - (wedge_g0_xyz.yzx() * other.group1().zxy()))
            .with_w(0.0),
            // e5
            (wedge_g0_w * other[e45]) + (wedge_g0_xyz[0] * other[e15]) + (wedge_g0_xyz[1] * other[e25]) + (wedge_g0_xyz[2] * other[e35]),
        )
    }
}
impl ProjectViaOriginOnto<DipoleInversion> for AntiPlane {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       20       33        0        0
    //    simd2        1        2        0      N/A
    //    simd3        4       11        0      N/A
    //    simd4        5        4        0      N/A
    // Totals...
    // yes simd       30       50        0      N/A
    //  no simd       54       86        0        0
    fn project_via_origin_onto(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0 = Simd32x3::from(other[e1234] * -1.0) * self.group0().xyz();
        let wedge_g1_xy = (self.group0().zx() * other.group3().yz()) - (self.group0().yz() * other.group3().zx());
        let wedge_g1_z = (self[e2] * other[e4235]) - (self[e1] * other[e4315]);
        let wedge_g2_xyz = (Simd32x3::from(self[e5]) * other.group3().xyz()) + (Simd32x3::from(other[e3215]) * self.group0().xyz());
        let wedge_g2_w = right_dual_g0[0] * self[e1];
        let wedge_g3_xyz = Simd32x3::from([
            (self[e2] * other[e12]) - (self[e3] * other[e31]),
            (self[e3] * other[e23]) - (self[e1] * other[e12]),
            (self[e1] * other[e31]) - (self[e2] * other[e23]),
        ]) - (right_dual_g0 * Simd32x3::from(self[e5]));
        let wedge_g3_w = self[e3] * other[e35];
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (wedge_g3_xyz * Simd32x3::from(other[e1234])) - (Simd32x3::from(wedge_g2_w) * other.group3().xyz()),
            // e415, e425, e435, e321
            (wedge_g3_xyz.yzx() * other.group3().zxy()).with_w(wedge_g3_w * other[e1234]) - (other.group3().yzxw() * wedge_g3_xyz.zxy().with_w(wedge_g2_w)),
            // e235, e315, e125, e4
            (other.group3().xyzx() * Simd32x3::from(wedge_g3_w).with_w(wedge_g0[0]))
                + -(wedge_g3_xyz * Simd32x3::from(other[e3215]))
                    .with_w(-(wedge_g2_w * other[e45]) - (wedge_g3_xyz[0] * other[e41]) - (wedge_g3_xyz[1] * other[e42]) - (wedge_g3_xyz[2] * other[e43])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (wedge_g1_z * other[e4315]) + (wedge_g3_w * other[e41]) - (wedge_g2_w * other[e15]) - (wedge_g1_xy[1] * other[e4125]),
                (wedge_g3_w * other[e42]) + (wedge_g1_xy[0] * other[e4125]) - (wedge_g1_z * other[e4235]) - (wedge_g2_w * other[e25]),
                (wedge_g3_w * other[e43]) + (wedge_g1_xy[1] * other[e4235]) - (wedge_g2_w * other[e35]) - (wedge_g1_xy[0] * other[e4315]),
                (wedge_g3_w * other[e45]) + (wedge_g3_xyz[0] * other[e15]) + (wedge_g3_xyz[2] * other[e35]) - (wedge_g2_xyz[0] * other[e4235]) - (wedge_g2_xyz[1] * other[e4315]),
            ]) + (other.group2().wwwy() * wedge_g2_xyz.with_w(wedge_g3_xyz[1]))
                + ((wedge_g3_xyz.zxy() * other.group1().yzx()) - (wedge_g3_xyz.yzx() * other.group1().zxy())).with_w(0.0)
                - (other.group3().wwwz() * wedge_g0.with_w(wedge_g2_xyz[2])),
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group0())
    }
}
impl ProjectViaOriginOnto<FlatPoint> for AntiPlane {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn project_via_origin_onto(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (self[e1] * other[e15] * other[e45]) + (self[e2] * other[e25] * other[e45]) + (self[e3] * other[e35] * other[e45]) - (self[e5] * other[e45] * other[e45]),
            0.0,
        ]))
    }
}
impl ProjectViaOriginOnto<Flector> for AntiPlane {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       10        0        0
    //    simd2        2        4        0      N/A
    //    simd3        1        3        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        9       17        0      N/A
    //  no simd       16       27        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([0.0, 0.0, (self[e2] * other[e4235]) - (self[e1] * other[e4315]), 0.0])
            + ((self.group0().zx() * other.group1().yz()) - (self.group0().yz() * other.group1().zx())).with_zw(0.0, 0.0);
        let wedge_g1_xyz = (Simd32x3::from(self[e5]) * other.group1().xyz()) + (Simd32x3::from(other[e3215]) * self.group0().xyz());
        let wedge_g1_w = self[e5] * other[e45] * -1.0;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (Simd32x3::from(wedge_g1_w) * other.group1().xyz()).with_w(0.0),
            // e1, e2, e3, e5
            ((wedge_g0.zx() * other.group1().yz()) - (wedge_g0.yz() * other.group1().zx())).with_zw(
                (wedge_g0[1] * other[e4235]) - (wedge_g0[0] * other[e4315]),
                (wedge_g1_w * other[e45]) - (wedge_g1_xyz[0] * other[e4235]) - (wedge_g1_xyz[1] * other[e4315]) - (wedge_g1_xyz[2] * other[e4125]),
            ),
        )
    }
}
impl ProjectViaOriginOnto<Line> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        2        3        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd        8       12        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = (Simd32x3::from(self[e5]) * other.group0()) + (other.group1().zxy() * self.group0().yzx()) - (other.group1().yzx() * self.group0().zxy());
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(wedge_g0_xyz[0] * other[e415]) - (wedge_g0_xyz[1] * other[e425]) - (wedge_g0_xyz[2] * other[e435])),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for AntiPlane {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        3        5        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        9        0      N/A
    //  no simd        9       25        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0 =
            ((Simd32x3::from(self[e5]) * other.group0().xyz()) + (right_dual_g1_xyz.zxy() * self.group0().yzx()) - (right_dual_g1_xyz.yzx() * self.group0().zxy())).with_w(0.0);
        let wedge_g1 = Simd32x4::from(other[e12345] * -1.0) * self.group0();
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            wedge_g0 * Simd32x4::from(other[e12345]),
            // e1, e2, e3, e5
            ((Simd32x3::from(wedge_g0[3]) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g1.xyz())).with_w(wedge_g1[3] * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for AntiPlane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       53       70        0        0
    //    simd2        0        2        0      N/A
    //    simd3       45       62        0      N/A
    //    simd4        9       11        0      N/A
    // Totals...
    // yes simd      107      145        0      N/A
    //  no simd      224      304        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let right_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g9_xyz = other.group1().xyz();
        let wedge_g0_y = (right_dual_g9_xyz[0] * self[e1]) + (right_dual_g9_xyz[1] * self[e2]) + (right_dual_g9_xyz[2] * self[e3]) - (self[e5] * other[e4]);
        let wedge_g1 = Simd32x4::from([1.0, 1.0, right_dual_g0[0], 0.0]) * (self.group0().xyz() * Simd32x2::from(right_dual_g0[0]).with_z(1.0)).with_w(0.0);
        let wedge_g2 = right_dual_g0[0] * self[e5];
        let wedge_g3 = ((Simd32x3::from(other[e3215]) * self.group0().xyz()) - (right_dual_g1_xyz * Simd32x3::from(self[e5]))).with_w(self[e5] * other[e1234] * -1.0);
        let wedge_g4 = Simd32x3::from(other[e1234] * -1.0) * self.group0().xyz();
        let wedge_g5 = (right_dual_g1_xyz.zxy() * self.group0().yzx()) - (right_dual_g1_xyz.yzx() * self.group0().zxy());
        let wedge_g6 = ((Simd32x3::from(self[e5]) * other.group7()) + (Simd32x3::from(other[e321]) * self.group0().xyz())).with_w(0.0);
        let wedge_g7 = (other.group7().yzx() * self.group0().zxy()) - (other.group7().zxy() * self.group0().yzx());
        let wedge_g8 = (Simd32x3::from(self[e5]) * other.group6().xyz()) + (other.group8().zxy() * self.group0().yzx()) - (other.group8().yzx() * self.group0().zxy());
        let wedge_g9 = ((right_dual_g6_xyz.yzx() * self.group0().zxy()) - (right_dual_g7 * Simd32x3::from(self[e5])) - (right_dual_g6_xyz.zxy() * self.group0().yzx()))
            .with_w(self[e5] * other[e45] * -1.0);
        let wedge_g10 = (right_dual_g7[0] * self[e1]) + (right_dual_g7[1] * self[e2]) + (right_dual_g7[2] * self[e3]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar])
                    + (wedge_g10 * other[e5])
                    + (wedge_g2 * other[e1234])
                    + (wedge_g1[0] * other[e4235])
                    + (wedge_g1[1] * other[e4315])
                    + (wedge_g1[2] * other[e4125])
                    + (wedge_g1[3] * other[e3215])
                    + (wedge_g9[0] * other[e1])
                    + (wedge_g9[1] * other[e2])
                    + (wedge_g9[2] * other[e3])
                    + (wedge_g9[3] * other[e4])
                    - (wedge_g4[0] * other[e235])
                    - (wedge_g4[1] * other[e315])
                    - (wedge_g4[2] * other[e125])
                    - (wedge_g5[0] * other[e415])
                    - (wedge_g5[1] * other[e425])
                    - (wedge_g5[2] * other[e435])
                    - (wedge_g7[0] * other[e15])
                    - (wedge_g7[1] * other[e25])
                    - (wedge_g7[2] * other[e35])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43])
                    - (other[e23] * wedge_g6[0])
                    - (other[e31] * wedge_g6[1])
                    - (other[e12] * wedge_g6[2])
                    - (other[e423] * wedge_g3[0])
                    - (other[e431] * wedge_g3[1])
                    - (other[e412] * wedge_g3[2])
                    - (wedge_g3[3] * other[e321])
                    - (wedge_g6[3] * other[e45]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            (wedge_g1 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group1())
                + (other.group9().yzxx() * wedge_g5.zxy().with_w(wedge_g4[0]))
                + ((Simd32x3::from(wedge_g6[3]) * other.group6().xyz())
                    + (Simd32x3::from(wedge_g9[3]) * other.group4())
                    + (Simd32x3::from(other[e321]) * wedge_g6.xyz())
                    + (Simd32x3::from(other[e1234]) * wedge_g3.xyz())
                    + (wedge_g7.zxy() * other.group8().yzx())
                    + (wedge_g8.yzx() * other.group7().zxy())
                    + (other.group5().yzx() * wedge_g9.zxy())
                    - (wedge_g4 * Simd32x3::from(other[e3215]))
                    - (Simd32x3::from(wedge_g10) * other.group3().xyz())
                    - (wedge_g5.yzx() * other.group9().zxy())
                    - (wedge_g7.yzx() * other.group8().zxy())
                    - (wedge_g8.zxy() * other.group7().yzx())
                    - (other.group5().zxy() * wedge_g9.yzx()))
                .with_w((wedge_g4[1] * other[e4315]) + (wedge_g4[2] * other[e4125]) + (wedge_g3[3] * other[e1234])),
            // e5
            (wedge_g0_y * other[e5])
                + (wedge_g2 * other[e12345])
                + (wedge_g9[0] * other[e15])
                + (wedge_g9[1] * other[e25])
                + (wedge_g9[2] * other[e35])
                + (wedge_g9[3] * other[e45])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435])
                - (other[e235] * wedge_g6[0])
                - (other[e315] * wedge_g6[1])
                - (other[e125] * wedge_g6[2])
                - (wedge_g3[0] * other[e4235])
                - (wedge_g3[1] * other[e4315])
                - (wedge_g3[2] * other[e4125])
                - (wedge_g3[3] * other[e3215]),
            // e15, e25, e35, e45
            (wedge_g3 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group3())
                + ((Simd32x3::from(wedge_g9[3]) * other.group6().xyz())
                    + (Simd32x3::from(other[e3215]) * wedge_g6.xyz())
                    + (wedge_g8.yzx() * other.group9().zxy())
                    + (other.group8().yzx() * wedge_g9.zxy())
                    - (wedge_g8.zxy() * other.group9().yzx())
                    - (other.group8().zxy() * wedge_g9.yzx()))
                .with_w(0.0),
            // e41, e42, e43
            (wedge_g4 * Simd32x3::from(other[e12345]))
                + (Simd32x3::from(wedge_g0_y) * other.group4())
                + (Simd32x3::from(wedge_g10) * other.group6().xyz())
                + (Simd32x3::from(other[e1234]) * wedge_g6.xyz())
                + (wedge_g7.zxy() * other.group9().yzx())
                + (other.group7().zxy() * wedge_g9.yzx())
                - (wedge_g7.yzx() * other.group9().zxy())
                - (other.group7().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (wedge_g5 * Simd32x3::from(other[e12345]))
                + (wedge_g7 * Simd32x3::from(other[e3215]))
                + (wedge_g8 * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(wedge_g0_y) * other.group5())
                + (Simd32x3::from(wedge_g10) * other.group8())
                + (Simd32x3::from(wedge_g9[3]) * other.group7())
                - (Simd32x3::from(wedge_g6[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            (wedge_g6 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group6())
                + (wedge_g9.yzxw() * other.group9().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group9().yz()).with_zw(wedge_g9[1] * other[e4235] * -1.0, wedge_g10 * other[e3215] * -1.0),
            // e423, e431, e412
            (wedge_g7 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group7()) + (Simd32x3::from(other[e1234]) * wedge_g9.xyz())
                - (Simd32x3::from(wedge_g10) * other.group9().xyz()),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (wedge_g9 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group9()),
            // e1234
            (wedge_g0_y * other[e1234]) + (wedge_g10 * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        1        2        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        6       12        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from([
            (self[e3] * other[e4315]) - (self[e2] * other[e4125]),
            (self[e1] * other[e4125]) - (self[e3] * other[e4235]),
            (self[e2] * other[e4235]) - (self[e1] * other[e4315]),
        ]);
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            ((wedge_g0.zxy() * other.group0().yzx()) - (wedge_g0.yzx() * other.group0().zxy())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<RoundPoint> for AntiPlane {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3        9        0        0
    fn project_via_origin_onto(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let wedge_g0 = (right_dual_g0_xyz[0] * self[e1]) + (right_dual_g0_xyz[1] * self[e2]) + (right_dual_g0_xyz[2] * self[e3]) - (self[e5] * other[e4]);
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(wedge_g0) * other.group0(), /* e5 */ wedge_g0 * other[e5])
    }
}
impl ProjectViaOriginOnto<Sphere> for AntiPlane {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       11        0        0
    //    simd2        1        3        0      N/A
    //    simd3        3        6        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        9       21        0      N/A
    //  no simd       19       39        0        0
    fn project_via_origin_onto(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = Simd32x3::from(other[e1234] * -1.0) * self.group0().xyz();
        let wedge_g1_xy = (self.group0().yz() * right_dual_g0_xyz.zx()) - (self.group0().zx() * right_dual_g0_xyz.yz());
        let wedge_g1_z = (right_dual_g0_xyz[1] * self[e1]) - (right_dual_g0_xyz[0] * self[e2]);
        let wedge_g2 = (Simd32x3::from(other[e3215]) * self.group0().xyz()) - (right_dual_g0_xyz * Simd32x3::from(self[e5]));
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([wedge_g1_z, wedge_g1_xy[0], wedge_g1_xy[1], wedge_g0[0]]) * other.group0().yzxx())
                + ((wedge_g2 * Simd32x3::from(other[e1234])) + -(Simd32x2::from([wedge_g1_xy[1], wedge_g1_z]) * other.group0().zx()).with_z(wedge_g1_xy[0] * other[e4315] * -1.0)
                    - (wedge_g0 * Simd32x3::from(other[e3215])))
                .with_w(wedge_g0[1] * other[e4315]),
            // e5
            (self[e5] * other[e3215] * other[e1234]) - (wedge_g2[0] * other[e4235]) - (wedge_g2[1] * other[e4315]) - (wedge_g2[2] * other[e4125]),
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for AntiPlane {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       15       26        0        0
    //    simd2        1        2        0      N/A
    //    simd3        8       11        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd       27       43        0      N/A
    //  no simd       53       79        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let right_dual_g2_xyz = other.group2().xyz();
        let wedge_g0_xy = (self.group0().zx() * right_dual_g0_xyz.yz()) - (self.group0().yz() * right_dual_g0_xyz.zx());
        let wedge_g0_z = (right_dual_g0_xyz[0] * self[e2]) - (right_dual_g0_xyz[1] * self[e1]);
        let wedge_g0_w = self[e1] * other[e1];
        let wedge_g1 = ((right_dual_g0_xyz * Simd32x3::from(self[e5])) + (Simd32x3::from(other[e321]) * self.group0().xyz())).with_w(0.0);
        let wedge_g2 = ((Simd32x3::from(self[e5]) * other.group1().xyz()) + (right_dual_g2_xyz.zxy() * self.group0().yzx()) - (right_dual_g2_xyz.yzx() * self.group0().zxy()))
            .with_w(right_dual_g0_w * self[e5]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x3::from(wedge_g0_w) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g0_xy.with_z(wedge_g0_z))).with_w(wedge_g0_w * other[e12345]),
            // e415, e425, e435, e321
            (wedge_g1 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_w) * other.group1()),
            // e235, e315, e125, e5
            (wedge_g2 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_w) * other.group2())
                + Simd32x3::from(0.0).with_w(
                    -(wedge_g1[0] * other[e235])
                        - (wedge_g1[1] * other[e315])
                        - (wedge_g1[2] * other[e125])
                        - (wedge_g2[0] * other[e415])
                        - (wedge_g2[1] * other[e425])
                        - (wedge_g2[2] * other[e435]),
                ),
            // e1, e2, e3, e4
            (Simd32x3::from([
                (wedge_g0_z * other[e315]) + (wedge_g2[1] * other[e412]) - (wedge_g0_xy[1] * other[e125]) - (wedge_g2[2] * other[e431]),
                (wedge_g0_xy[0] * other[e125]) + (wedge_g2[2] * other[e423]) - (wedge_g0_z * other[e235]) - (wedge_g2[0] * other[e412]),
                (wedge_g0_xy[1] * other[e235]) + (wedge_g2[0] * other[e431]) - (wedge_g0_xy[0] * other[e315]) - (wedge_g2[1] * other[e423]),
            ]) + (Simd32x3::from(wedge_g0_w) * other.group3().xyz())
                + (Simd32x3::from(wedge_g1[3]) * other.group1().xyz())
                + (Simd32x3::from(other[e321]) * wedge_g1.xyz())
                + (Simd32x3::from(right_dual_g0_w * other[e12345]) * self.group0().xyz()))
            .with_w(wedge_g0_w * other[e4]),
        )
    }
}
impl ProjectViaOriginOnto<VersorOdd> for AntiPlane {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       20       35        0        0
    //    simd2        1        2        0      N/A
    //    simd3        4       11        0      N/A
    //    simd4        5        4        0      N/A
    // Totals...
    // yes simd       30       52        0      N/A
    //  no simd       54       88        0        0
    fn project_via_origin_onto(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = Simd32x3::from(other[e1234] * -1.0) * self.group0().xyz();
        let wedge_g1_xy = (self.group0().yz() * right_dual_g3_xyz.zx()) - (self.group0().zx() * right_dual_g3_xyz.yz());
        let wedge_g1_z = (right_dual_g3_xyz[1] * self[e1]) - (right_dual_g3_xyz[0] * self[e2]);
        let wedge_g2_xyz = (Simd32x3::from(other[e3215]) * self.group0().xyz()) - (right_dual_g3_xyz * Simd32x3::from(self[e5]));
        let wedge_g2_w = self[e1] * other[e41] * -1.0;
        let wedge_g3_xyz = Simd32x3::from([
            (self[e2] * other[e12]) - (self[e3] * other[e31]),
            (self[e3] * other[e23]) - (self[e1] * other[e12]),
            (self[e1] * other[e31]) - (self[e2] * other[e23]),
        ]) + (Simd32x3::from(self[e5]) * other.group0().xyz());
        let wedge_g3_w = self[e5] * other[e45] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (wedge_g3_xyz * Simd32x3::from(other[e1234])) - (Simd32x3::from(wedge_g2_w) * other.group3().xyz()),
            // e415, e425, e435, e321
            (wedge_g3_xyz.yzx() * other.group3().zxy()).with_w(wedge_g3_w * other[e1234]) - (other.group3().yzxw() * wedge_g3_xyz.zxy().with_w(wedge_g2_w)),
            // e235, e315, e125, e4
            (other.group3().xyzx() * Simd32x3::from(wedge_g3_w).with_w(wedge_g0[0]))
                + -(wedge_g3_xyz * Simd32x3::from(other[e3215]))
                    .with_w(-(wedge_g2_w * other[e45]) - (wedge_g3_xyz[0] * other[e41]) - (wedge_g3_xyz[1] * other[e42]) - (wedge_g3_xyz[2] * other[e43])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (wedge_g1_z * other[e4315]) + (wedge_g3_w * other[e41]) - (wedge_g2_w * other[e15]) - (wedge_g1_xy[1] * other[e4125]),
                (wedge_g3_w * other[e42]) + (wedge_g1_xy[0] * other[e4125]) - (wedge_g1_z * other[e4235]) - (wedge_g2_w * other[e25]),
                (wedge_g3_w * other[e43]) + (wedge_g1_xy[1] * other[e4235]) - (wedge_g2_w * other[e35]) - (wedge_g1_xy[0] * other[e4315]),
                (wedge_g3_w * other[e45]) + (wedge_g3_xyz[0] * other[e15]) + (wedge_g3_xyz[2] * other[e35]) - (wedge_g2_xyz[0] * other[e4235]) - (wedge_g2_xyz[1] * other[e4315]),
            ]) + (other.group2().wwwy() * wedge_g2_xyz.with_w(wedge_g3_xyz[1]))
                + ((wedge_g3_xyz.zxy() * other.group1().yzx()) - (wedge_g3_xyz.yzx() * other.group1().zxy())).with_w(0.0)
                - (other.group3().wwwz() * wedge_g0.with_w(wedge_g2_xyz[2])),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for AntiScalar {
    type Output = ProjectViaOriginOntoInfixPartial<AntiScalar>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiScalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[e12345] * other[e12345] * self[e12345] * -1.0)
    }
}
impl ProjectViaOriginOnto<CircleRotor> for AntiScalar {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       13        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = other[e12345] * self[e12345] * -1.0;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(wedge_g0) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(wedge_g0) * other.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(wedge_g0) * other.group2(),
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for AntiScalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        4        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(other[e12345] * self[e12345] * -1.0) * other.group0())
    }
}
impl ProjectViaOriginOnto<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = other[e12345] * self[e12345] * -1.0;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(wedge_g0) * other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(wedge_g0) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        4        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0       13        0      N/A
    //  no simd        0       34        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = other[e12345] * self[e12345] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(wedge_g0) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(wedge_g0) * other.group1(),
            // e5
            wedge_g0 * other[e5],
            // e15, e25, e35, e45
            Simd32x4::from(wedge_g0) * other.group3(),
            // e41, e42, e43
            Simd32x3::from(wedge_g0) * other.group4(),
            // e23, e31, e12
            Simd32x3::from(wedge_g0) * other.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(wedge_g0) * other.group6(),
            // e423, e431, e412
            Simd32x3::from(wedge_g0) * other.group7(),
            // e235, e315, e125
            Simd32x3::from(wedge_g0) * other.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0) * other.group9(),
            // e1234
            wedge_g0 * other[e1234],
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for AntiScalar {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       18        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = other[e12345] * self[e12345] * -1.0;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(wedge_g0) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(wedge_g0) * other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(wedge_g0) * other.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(wedge_g0) * other.group3(),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for Circle {
    type Output = ProjectViaOriginOntoInfixPartial<Circle>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiDipoleInversion> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       10        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd        9       25        0        0
    fn project_via_origin_onto(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g2_xyz = other.group2().xyz();
        let wedge_g0 = (other[e321] * self[e321])
            - (right_dual_g1_xyz[0] * self[e415])
            - (right_dual_g1_xyz[1] * self[e425])
            - (right_dual_g1_xyz[2] * self[e435])
            - (right_dual_g2_xyz[0] * self[e423])
            - (right_dual_g2_xyz[1] * self[e431])
            - (right_dual_g2_xyz[2] * self[e412])
            - (other[e423] * self[e235])
            - (other[e431] * self[e315])
            - (other[e412] * self[e125]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(wedge_g0) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(wedge_g0) * other.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(wedge_g0) * other.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(wedge_g0) * other.group3(),
        )
    }
}
impl ProjectViaOriginOnto<AntiDualNum> for Circle {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       13        0        0
    fn project_via_origin_onto(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([1.0, 1.0, other[e3215] * other[e3215], 0.0])
                * (self.group0() * Simd32x2::from(other[e3215] * other[e3215] * -1.0).with_z(1.0) * Simd32x3::from([1.0, 1.0, -1.0])).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlatPoint> for Circle {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd       12       20        0        0
    fn project_via_origin_onto(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            (Simd32x4::from(other[e321] * self[e321]) * other.group0())
                - (Simd32x4::from(right_dual_g0_xyz[0] * self[e423]) * other.group0())
                - (Simd32x4::from(right_dual_g0_xyz[1] * self[e431]) * other.group0())
                - (Simd32x4::from(right_dual_g0_xyz[2] * self[e412]) * other.group0()),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlector> for Circle {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3       12        0        0
    fn project_via_origin_onto(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let wedge_g0 = (other[e321] * self[e321]) - (right_dual_g0_xyz[0] * self[e423]) - (right_dual_g0_xyz[1] * self[e431]) - (right_dual_g0_xyz[2] * self[e412]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(wedge_g0) * other.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(wedge_g0) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<AntiMotor> for Circle {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd2        1        2        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        8        0      N/A
    //  no simd        3       17        0        0
    fn project_via_origin_onto(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(other[e3215]) * self.group0().with_w(self[e321]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (wedge_g0.xyz() * Simd32x4::from(other[e3215]).xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e1, e2, e3, e5
            ((wedge_g0.zx() * other.group0().yz()) - (wedge_g0.yz() * other.group0().zx()))
                .with_zw((wedge_g0[1] * other[e23]) - (wedge_g0[0] * other[e31]), wedge_g0[0] * other[e15]),
        )
    }
}
impl ProjectViaOriginOnto<AntiScalar> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        7        0      N/A
    //  no simd        0       14        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e12345] * -1.0;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual_g0 * other[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group1(),
            // e235, e315, e125
            Simd32x3::from(right_dual_g0 * other[e12345]) * self.group2(),
        )
    }
}
impl ProjectViaOriginOnto<Circle> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       10        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        9       13        0      N/A
    //  no simd        9       20        0        0
    fn project_via_origin_onto(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0 = (other[e321] * self[e321])
            - (right_dual_g1_xyz[0] * self[e415])
            - (right_dual_g1_xyz[1] * self[e425])
            - (right_dual_g1_xyz[2] * self[e435])
            - (other[e423] * self[e235])
            - (other[e431] * self[e315])
            - (other[e412] * self[e125])
            - (other[e235] * self[e423])
            - (other[e315] * self[e431])
            - (other[e125] * self[e412]);
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(wedge_g0) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(wedge_g0) * other.group1(),
            // e235, e315, e125
            Simd32x3::from(wedge_g0) * other.group2(),
        )
    }
}
impl ProjectViaOriginOnto<CircleRotor> for Circle {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       12        0        0
    //    simd3        7       12        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd       17       27        0      N/A
    //  no simd       34       60        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g2_xyz = other.group2().xyz();
        let right_dual_g2_w = other[e12345] * -1.0;
        let wedge_g0 = Simd32x3::from(right_dual_g2_w) * self.group0();
        let wedge_g1 = Simd32x4::from(right_dual_g2_w) * self.group1();
        let wedge_g2_xyz = Simd32x3::from(right_dual_g2_w) * self.group2();
        let wedge_g2_w = (self[e321] * other[e321])
            - (right_dual_g1_xyz[0] * self[e415])
            - (right_dual_g1_xyz[1] * self[e425])
            - (right_dual_g1_xyz[2] * self[e435])
            - (right_dual_g2_xyz[0] * self[e423])
            - (right_dual_g2_xyz[1] * self[e431])
            - (right_dual_g2_xyz[2] * self[e412])
            - (self[e235] * other[e423])
            - (self[e315] * other[e431])
            - (self[e125] * other[e412]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((wedge_g0 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g2_w) * other.group0())).with_w(wedge_g2_w * other[e12345]),
            // e415, e425, e435, e321
            (wedge_g1 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g2_w) * other.group1()),
            // e235, e315, e125, e5
            ((wedge_g2_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g2_w) * other.group2().xyz())).with_w(0.0),
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g1[3]) * other.group1().xyz())
                + (Simd32x3::from(other[e321]) * wedge_g1.xyz())
                + (wedge_g0.zxy() * other.group2().yzx())
                + (wedge_g2_xyz.yzx() * other.group0().zxy())
                - (wedge_g0.yzx() * other.group2().zxy())
                - (wedge_g2_xyz.zxy() * other.group0().yzx()))
            .with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<DipoleInversion> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       14        0        0
    //    simd2        1        4        0      N/A
    //    simd3        5        7        0      N/A
    //    simd4        5        3        0      N/A
    // Totals...
    // yes simd       15       28        0      N/A
    //  no simd       41       55        0        0
    fn project_via_origin_onto(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([0.0, 0.0, (self[e415] * other[e4315]) - (self[e425] * other[e4235]), 0.0])
            + ((Simd32x3::from(other[e3215]) * self.group0()) + ((self.group1().yz() * other.group3().zx()) - (self.group1().zx() * other.group3().yz())).with_z(0.0)
                - (Simd32x3::from(other[e1234]) * self.group2()))
            .with_w(0.0);
        let wedge_g1 = (self[e423] * other[e4235]) + (self[e431] * other[e4315]) + (self[e412] * other[e4125]) - (self[e321] * other[e1234]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other[e1234]) * wedge_g0.xyz()) - (Simd32x3::from(wedge_g1) * other.group3().xyz()),
            // e415, e425, e435, e321
            (wedge_g0.yzxw() * other.group3().zxy().with_w(other[e1234]))
                + -(wedge_g0.zx() * other.group3().yz()).with_zw(wedge_g0[1] * other[e4235] * -1.0, wedge_g1 * other[e3215] * -1.0),
            // e235, e315, e125, e4
            ((Simd32x3::from(wedge_g0[3]) * other.group3().xyz()) - (Simd32x3::from(other[e3215]) * wedge_g0.xyz())).with_w(wedge_g1 * other[e45] * -1.0),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, wedge_g0[0] * other[e31] * -1.0, 0.0])
                + (wedge_g0.zxyw() * other.group1().yzxw())
                + (wedge_g0.wwwx() * other.group0().with_w(other[e15]))
                + (-(wedge_g0.yz() * other.group1().zx()).with_z(0.0) - (Simd32x3::from(wedge_g1) * other.group2().xyz())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        9        0      N/A
    //  no simd        0       16        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e12345] * other[e12345] * -1.0) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group1(),
            // e235, e315, e125
            Simd32x3::from(other[e12345] * other[e12345] * -1.0) * self.group2(),
        )
    }
}
impl ProjectViaOriginOnto<Flector> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       17        0        0
    //    simd2        2        4        0      N/A
    //    simd3        2        5        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd       12       26        0      N/A
    //  no simd       21       40        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([0.0, 0.0, (self[e415] * other[e4315]) - (self[e425] * other[e4235]), 0.0])
            + ((Simd32x3::from(other[e3215]) * self.group0()) + ((self.group1().yz() * other.group1().zx()) - (self.group1().zx() * other.group1().yz())).with_z(0.0)).with_w(0.0);
        let wedge_g1 = (self[e423] * other[e4235]) + (self[e431] * other[e4315]) + (self[e412] * other[e4125]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(wedge_g1 * -1.0) * other.group1().xyz(),
            // e415, e425, e435, e321
            ((wedge_g0.yz() * other.group1().zx()) - (wedge_g0.zx() * other.group1().yz()))
                .with_zw((wedge_g0[0] * other[e4315]) - (wedge_g0[1] * other[e4235]), wedge_g1 * other[e3215] * -1.0),
            // e235, e315, e125, e4
            ((Simd32x3::from(wedge_g0[3]) * other.group1().xyz()) - (Simd32x3::from(other[e3215]) * wedge_g0.xyz())).with_w(wedge_g1 * other[e45] * -1.0),
            // e1, e2, e3, e5
            (Simd32x3::from(wedge_g1 * -1.0) * other.group0().xyz())
                .with_w((wedge_g0[0] * other[e15]) + (wedge_g0[1] * other[e25]) + (wedge_g0[2] * other[e35]) + (wedge_g0[3] * other[e45])),
        )
    }
}
impl ProjectViaOriginOnto<Line> for Circle {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       12        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = -(self[e423] * other[e235])
            - (self[e431] * other[e315])
            - (self[e412] * other[e125])
            - (other[e415] * self[e415])
            - (other[e425] * self[e425])
            - (other[e435] * self[e435]);
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(wedge_g0) * other.group0(),
            // e235, e315, e125
            Simd32x3::from(wedge_g0) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for Circle {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       10        0        0
    //    simd3        4        8        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        9       20        0      N/A
    //  no simd       17       42        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0 = Simd32x3::from(right_dual_g0_w) * self.group0();
        let wedge_g1 = Simd32x4::from(right_dual_g0_w) * self.group1();
        let wedge_g2_w = -(right_dual_g0_xyz[0] * self[e415])
            - (right_dual_g0_xyz[1] * self[e425])
            - (right_dual_g0_xyz[2] * self[e435])
            - (right_dual_g1_xyz[0] * self[e423])
            - (right_dual_g1_xyz[1] * self[e431])
            - (right_dual_g1_xyz[2] * self[e412]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other[e12345]) * wedge_g0.with_w(wedge_g2_w),
            // e415, e425, e435, e321
            ((Simd32x3::from(wedge_g2_w) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g1.xyz())).with_w(wedge_g1[3] * other[e12345]),
            // e235, e315, e125, e5
            ((Simd32x3::from(wedge_g2_w) * other.group1().xyz()) + (Simd32x3::from(right_dual_g0_w * other[e12345]) * self.group2())).with_w(wedge_g2_w * other[e5]),
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g1[3]) * other.group0().xyz()) + (wedge_g0.zxy() * other.group1().yzx()) - (wedge_g0.yzx() * other.group1().zxy())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for Circle {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       39       52        0        0
    //    simd2        0        2        0      N/A
    //    simd3       37       47        0      N/A
    //    simd4        4        6        0      N/A
    // Totals...
    // yes simd       80      107        0      N/A
    //  no simd      166      221        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let right_dual_g5 = other.group6().xyz();
        let wedge_g0_y = (self[e321] * other[e321])
            - (right_dual_g5[0] * self[e415])
            - (right_dual_g5[1] * self[e425])
            - (right_dual_g5[2] * self[e435])
            - (self[e423] * other[e235])
            - (self[e431] * other[e315])
            - (self[e412] * other[e125])
            - (self[e235] * other[e423])
            - (self[e315] * other[e431])
            - (self[e125] * other[e412]);
        let wedge_g6 = Simd32x4::from(right_dual_g0[0]) * self.group1();
        let wedge_g7 = Simd32x3::from(right_dual_g0[0]) * self.group0();
        let wedge_g8 = Simd32x3::from(right_dual_g0[0]) * self.group2();
        let wedge_g9 = ((Simd32x3::from(other[e3215]) * self.group0()) + (right_dual_g1_xyz.yzx() * self.group1().zxy())
            - (Simd32x3::from(other[e1234]) * self.group2())
            - (right_dual_g1_xyz.zxy() * self.group1().yzx()))
        .with_w((right_dual_g1_xyz[0] * self[e235]) + (right_dual_g1_xyz[1] * self[e315]));
        let wedge_g10 = -(right_dual_g1_xyz[0] * self[e423]) - (right_dual_g1_xyz[1] * self[e431]) - (right_dual_g1_xyz[2] * self[e412]) - (self[e321] * other[e1234]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar])
                    + (wedge_g10 * other[e5])
                    + (wedge_g9[0] * other[e1])
                    + (wedge_g9[1] * other[e2])
                    + (wedge_g9[2] * other[e3])
                    + (wedge_g9[3] * other[e4])
                    - (wedge_g7[0] * other[e15])
                    - (wedge_g7[1] * other[e25])
                    - (wedge_g7[2] * other[e35])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43])
                    - (other[e23] * wedge_g6[0])
                    - (other[e31] * wedge_g6[1])
                    - (other[e12] * wedge_g6[2])
                    - (wedge_g6[3] * other[e45]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g0_y) * other.group1().xyz())
                + (Simd32x3::from(wedge_g6[3]) * other.group6().xyz())
                + (Simd32x3::from(wedge_g9[3]) * other.group4())
                + (Simd32x3::from(other[e321]) * wedge_g6.xyz())
                + (wedge_g7.zxy() * other.group8().yzx())
                + (wedge_g8.yzx() * other.group7().zxy())
                + (other.group5().yzx() * wedge_g9.zxy())
                - (Simd32x3::from(wedge_g10) * other.group3().xyz())
                - (wedge_g7.yzx() * other.group8().zxy())
                - (wedge_g8.zxy() * other.group7().yzx())
                - (other.group5().zxy() * wedge_g9.yzx()))
            .with_w(wedge_g0_y * other[e4]),
            // e5
            (wedge_g0_y * other[e5]) + (wedge_g9[0] * other[e15]) + (wedge_g9[1] * other[e25]) + (wedge_g9[2] * other[e35]) + (wedge_g9[3] * other[e45])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435])
                - (other[e235] * wedge_g6[0])
                - (other[e315] * wedge_g6[1])
                - (other[e125] * wedge_g6[2]),
            // e15, e25, e35, e45
            ((Simd32x3::from(wedge_g0_y) * other.group3().xyz())
                + (Simd32x3::from(wedge_g9[3]) * other.group6().xyz())
                + (Simd32x3::from(other[e3215]) * wedge_g6.xyz())
                + (wedge_g8.yzx() * other.group9().zxy())
                + (other.group8().yzx() * wedge_g9.zxy())
                - (wedge_g8.zxy() * other.group9().yzx())
                - (other.group8().zxy() * wedge_g9.yzx()))
            .with_w(wedge_g0_y * other[e45]),
            // e41, e42, e43
            (Simd32x3::from(wedge_g0_y) * other.group4())
                + (Simd32x3::from(wedge_g10) * other.group6().xyz())
                + (Simd32x3::from(other[e1234]) * wedge_g6.xyz())
                + (wedge_g7.zxy() * other.group9().yzx())
                + (other.group7().zxy() * wedge_g9.yzx())
                - (wedge_g7.yzx() * other.group9().zxy())
                - (other.group7().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (wedge_g7 * Simd32x3::from(other[e3215]))
                + (wedge_g8 * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(wedge_g0_y) * other.group5())
                + (Simd32x3::from(wedge_g10) * other.group8())
                + (Simd32x3::from(wedge_g9[3]) * other.group7())
                - (Simd32x3::from(wedge_g6[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            (wedge_g6 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group6())
                + (wedge_g9.yzxw() * other.group9().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group9().yz()).with_zw(wedge_g9[1] * other[e4235] * -1.0, wedge_g10 * other[e3215] * -1.0),
            // e423, e431, e412
            (wedge_g7 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group7()) + (Simd32x3::from(other[e1234]) * wedge_g9.xyz())
                - (Simd32x3::from(wedge_g10) * other.group9().xyz()),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (wedge_g9 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group9()),
            // e1234
            (wedge_g0_y * other[e1234]) + (wedge_g10 * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       10        0        0
    //    simd2        2        4        0      N/A
    //    simd3        2        4        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        9       18        0      N/A
    //  no simd       18       30        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([0.0, 0.0, (self[e415] * other[e4315]) - (self[e425] * other[e4235]), 0.0])
            + ((Simd32x3::from(other[e3215]) * self.group0()) + ((self.group1().yz() * other.group0().zx()) - (self.group1().zx() * other.group0().yz())).with_z(0.0)).with_w(0.0);
        let wedge_g1 = (self[e423] * other[e4235]) + (self[e431] * other[e4315]) + (self[e412] * other[e4125]);
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(wedge_g1 * -1.0) * other.group0().xyz(),
            // e415, e425, e435, e321
            ((wedge_g0.yz() * other.group0().zx()) - (wedge_g0.zx() * other.group0().yz()))
                .with_zw((wedge_g0[0] * other[e4315]) - (wedge_g0[1] * other[e4235]), wedge_g1 * other[e3215] * -1.0),
            // e235, e315, e125
            (Simd32x3::from(wedge_g0[3]) * other.group0().xyz()) - (Simd32x3::from(other[e3215]) * wedge_g0.xyz()),
        )
    }
}
impl ProjectViaOriginOnto<Sphere> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        7        0        0
    //    simd3        5       10        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       10       18        0      N/A
    //  no simd       23       41        0        0
    fn project_via_origin_onto(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = (Simd32x3::from(other[e3215]) * self.group0()) + (right_dual_g0_xyz.yzx() * self.group1().zxy())
            - (Simd32x3::from(other[e1234]) * self.group2())
            - (right_dual_g0_xyz.zxy() * self.group1().yzx());
        let wedge_g0_w = (right_dual_g0_xyz[0] * self[e235]) + (right_dual_g0_xyz[1] * self[e315]);
        let wedge_g1 = -(right_dual_g0_xyz[0] * self[e423]) - (right_dual_g0_xyz[1] * self[e431]) - (right_dual_g0_xyz[2] * self[e412]) - (self[e321] * other[e1234]);
        Circle::from_groups(
            // e423, e431, e412
            (wedge_g0_xyz * Simd32x3::from(other[e1234])) - (Simd32x3::from(wedge_g1) * other.group0().xyz()),
            // e415, e425, e435, e321
            (wedge_g0_xyz.yzx() * other.group0().zxy()).with_w(wedge_g0_w * other[e1234]) - (other.group0().yzxw() * wedge_g0_xyz.zxy().with_w(wedge_g1)),
            // e235, e315, e125
            (Simd32x3::from(wedge_g0_w) * other.group0().xyz()) - (wedge_g0_xyz * Simd32x3::from(other[e3215])),
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for Circle {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       14        0        0
    //    simd3        8       13        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd       18       30        0      N/A
    //  no simd       37       65        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g2_xyz = other.group2().xyz();
        let wedge_g0 = Simd32x3::from(right_dual_g0_w) * self.group0();
        let wedge_g1 = Simd32x4::from(right_dual_g0_w) * self.group1();
        let wedge_g2_xyz = Simd32x3::from(right_dual_g0_w) * self.group2();
        let wedge_g2_w = (self[e321] * other[e321])
            - (right_dual_g0_xyz[0] * self[e235])
            - (right_dual_g0_xyz[1] * self[e315])
            - (right_dual_g0_xyz[2] * self[e125])
            - (right_dual_g1_xyz[0] * self[e415])
            - (right_dual_g1_xyz[1] * self[e425])
            - (right_dual_g1_xyz[2] * self[e435])
            - (right_dual_g2_xyz[0] * self[e423])
            - (right_dual_g2_xyz[1] * self[e431])
            - (right_dual_g2_xyz[2] * self[e412]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((wedge_g0 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g2_w) * other.group0().xyz())).with_w(wedge_g2_w * other[e12345]),
            // e415, e425, e435, e321
            (wedge_g1 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g2_w) * other.group1()),
            // e235, e315, e125, e5
            ((wedge_g2_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g2_w) * other.group2().xyz())).with_w(wedge_g2_w * other[e5]),
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g2_w) * other.group3().xyz())
                + (Simd32x3::from(wedge_g1[3]) * other.group1().xyz())
                + (Simd32x3::from(other[e321]) * wedge_g1.xyz())
                + (wedge_g0.zxy() * other.group2().yzx())
                + (wedge_g2_xyz.yzx() * other.group0().zxy())
                - (wedge_g0.yzx() * other.group2().zxy())
                - (wedge_g2_xyz.zxy() * other.group0().yzx()))
            .with_w(wedge_g2_w * other[e4]),
        )
    }
}
impl ProjectViaOriginOnto<VersorOdd> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       11        0        0
    //    simd3        8       14        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       14       26        0      N/A
    //  no simd       33       57        0        0
    fn project_via_origin_onto(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = (Simd32x3::from(other[e3215]) * self.group0()) + (right_dual_g3_xyz.yzx() * self.group1().zxy())
            - (Simd32x3::from(other[e1234]) * self.group2())
            - (right_dual_g3_xyz.zxy() * self.group1().yzx());
        let wedge_g0_w = (right_dual_g3_xyz[0] * self[e235]) + (right_dual_g3_xyz[1] * self[e315]);
        let wedge_g1 = -(right_dual_g3_xyz[0] * self[e423]) - (right_dual_g3_xyz[1] * self[e431]) - (right_dual_g3_xyz[2] * self[e412]) - (self[e321] * other[e1234]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (wedge_g0_xyz * Simd32x3::from(other[e1234])) - (Simd32x3::from(wedge_g1) * other.group3().xyz()),
            // e415, e425, e435, e321
            (wedge_g0_xyz.yzx() * other.group3().zxy()).with_w(wedge_g0_w * other[e1234]) - (other.group3().yzxw() * wedge_g0_xyz.zxy().with_w(wedge_g1)),
            // e235, e315, e125, e4
            ((Simd32x3::from(wedge_g0_w) * other.group3().xyz()) - (wedge_g0_xyz * Simd32x3::from(other[e3215]))).with_w(wedge_g1 * other[e45] * -1.0),
            // e1, e2, e3, e5
            ((Simd32x3::from(wedge_g0_w) * other.group0().xyz()) + (wedge_g0_xyz.zxy() * other.group1().yzx())
                - (Simd32x3::from(wedge_g1) * other.group2().xyz())
                - (wedge_g0_xyz.yzx() * other.group1().zxy()))
            .with_w((wedge_g0_xyz[0] * other[e15]) + (wedge_g0_xyz[1] * other[e25])),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for CircleRotor {
    type Output = ProjectViaOriginOntoInfixPartial<CircleRotor>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiDipoleInversion> for CircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       10        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd        9       25        0        0
    fn project_via_origin_onto(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g2_xyz = other.group2().xyz();
        let wedge_g0 = (other[e321] * self[e321])
            - (right_dual_g1_xyz[0] * self[e415])
            - (right_dual_g1_xyz[1] * self[e425])
            - (right_dual_g1_xyz[2] * self[e435])
            - (right_dual_g2_xyz[0] * self[e423])
            - (right_dual_g2_xyz[1] * self[e431])
            - (right_dual_g2_xyz[2] * self[e412])
            - (other[e423] * self[e235])
            - (other[e431] * self[e315])
            - (other[e412] * self[e125]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(wedge_g0) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(wedge_g0) * other.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(wedge_g0) * other.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(wedge_g0) * other.group3(),
        )
    }
}
impl ProjectViaOriginOnto<AntiDualNum> for CircleRotor {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       13        0        0
    fn project_via_origin_onto(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([1.0, 1.0, other[e3215] * other[e3215], 0.0])
                * (self.group0() * Simd32x2::from(other[e3215] * other[e3215] * -1.0).with_z(1.0) * Simd32x3::from([1.0, 1.0, -1.0])).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlatPoint> for CircleRotor {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd       12       20        0        0
    fn project_via_origin_onto(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            (Simd32x4::from(other[e321] * self[e321]) * other.group0())
                - (Simd32x4::from(right_dual_g0_xyz[0] * self[e423]) * other.group0())
                - (Simd32x4::from(right_dual_g0_xyz[1] * self[e431]) * other.group0())
                - (Simd32x4::from(right_dual_g0_xyz[2] * self[e412]) * other.group0()),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlector> for CircleRotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3       12        0        0
    fn project_via_origin_onto(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let wedge_g0 = (other[e321] * self[e321]) - (right_dual_g0_xyz[0] * self[e423]) - (right_dual_g0_xyz[1] * self[e431]) - (right_dual_g0_xyz[2] * self[e412]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(wedge_g0) * other.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(wedge_g0) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<AntiMotor> for CircleRotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd2        1        2        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        8        0      N/A
    //  no simd        3       17        0        0
    fn project_via_origin_onto(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(other[e3215]) * self.group0().with_w(self[e321]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (wedge_g0.xyz() * Simd32x4::from(other[e3215]).xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e1, e2, e3, e5
            ((wedge_g0.zx() * other.group0().yz()) - (wedge_g0.yz() * other.group0().zx()))
                .with_zw((wedge_g0[1] * other[e23]) - (wedge_g0[0] * other[e31]), wedge_g0[0] * other[e15]),
        )
    }
}
impl ProjectViaOriginOnto<AntiScalar> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        7        0      N/A
    //  no simd        0       15        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e12345] * -1.0;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual_g0 * other[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group2(),
        )
    }
}
impl ProjectViaOriginOnto<Circle> for CircleRotor {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       10        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        9       13        0      N/A
    //  no simd        9       20        0        0
    fn project_via_origin_onto(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0 = (other[e321] * self[e321])
            - (right_dual_g1_xyz[0] * self[e415])
            - (right_dual_g1_xyz[1] * self[e425])
            - (right_dual_g1_xyz[2] * self[e435])
            - (other[e423] * self[e235])
            - (other[e431] * self[e315])
            - (other[e412] * self[e125])
            - (other[e235] * self[e423])
            - (other[e315] * self[e431])
            - (other[e125] * self[e412]);
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(wedge_g0) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(wedge_g0) * other.group1(),
            // e235, e315, e125
            Simd32x3::from(wedge_g0) * other.group2(),
        )
    }
}
impl ProjectViaOriginOnto<CircleRotor> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       13        0        0
    //    simd3        7       12        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd       18       28        0      N/A
    //  no simd       35       61        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g2_xyz = other.group2().xyz();
        let right_dual_g2_w = other[e12345] * -1.0;
        let wedge_g0 = Simd32x3::from(right_dual_g2_w) * self.group0();
        let wedge_g1 = Simd32x4::from(right_dual_g2_w) * self.group1();
        let wedge_g2_xyz = Simd32x3::from(right_dual_g2_w) * self.group2().xyz();
        let wedge_g2_w = (right_dual_g2_w * self[e12345]) + (other[e321] * self[e321])
            - (right_dual_g1_xyz[0] * self[e415])
            - (right_dual_g1_xyz[1] * self[e425])
            - (right_dual_g1_xyz[2] * self[e435])
            - (right_dual_g2_xyz[0] * self[e423])
            - (right_dual_g2_xyz[1] * self[e431])
            - (right_dual_g2_xyz[2] * self[e412])
            - (other[e423] * self[e235])
            - (other[e431] * self[e315])
            - (other[e412] * self[e125]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((wedge_g0 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g2_w) * other.group0())).with_w(wedge_g2_w * other[e12345]),
            // e415, e425, e435, e321
            (wedge_g1 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g2_w) * other.group1()),
            // e235, e315, e125, e5
            ((wedge_g2_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g2_w) * other.group2().xyz())).with_w(0.0),
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g1[3]) * other.group1().xyz())
                + (Simd32x3::from(other[e321]) * wedge_g1.xyz())
                + (wedge_g0.zxy() * other.group2().yzx())
                + (wedge_g2_xyz.yzx() * other.group0().zxy())
                - (wedge_g0.yzx() * other.group2().zxy())
                - (wedge_g2_xyz.zxy() * other.group0().yzx()))
            .with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<DipoleInversion> for CircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       14        0        0
    //    simd2        1        4        0      N/A
    //    simd3        5        7        0      N/A
    //    simd4        5        3        0      N/A
    // Totals...
    // yes simd       15       28        0      N/A
    //  no simd       41       55        0        0
    fn project_via_origin_onto(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([0.0, 0.0, (self[e415] * other[e4315]) - (self[e425] * other[e4235]), 0.0])
            + ((Simd32x3::from(other[e3215]) * self.group0()) + ((self.group1().yz() * other.group3().zx()) - (self.group1().zx() * other.group3().yz())).with_z(0.0)
                - (Simd32x3::from(other[e1234]) * self.group2().xyz()))
            .with_w(0.0);
        let wedge_g1 = (self[e423] * other[e4235]) + (self[e431] * other[e4315]) + (self[e412] * other[e4125]) - (self[e321] * other[e1234]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other[e1234]) * wedge_g0.xyz()) - (Simd32x3::from(wedge_g1) * other.group3().xyz()),
            // e415, e425, e435, e321
            (wedge_g0.yzxw() * other.group3().zxy().with_w(other[e1234]))
                + -(wedge_g0.zx() * other.group3().yz()).with_zw(wedge_g0[1] * other[e4235] * -1.0, wedge_g1 * other[e3215] * -1.0),
            // e235, e315, e125, e4
            ((Simd32x3::from(wedge_g0[3]) * other.group3().xyz()) - (Simd32x3::from(other[e3215]) * wedge_g0.xyz())).with_w(wedge_g1 * other[e45] * -1.0),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, wedge_g0[0] * other[e31] * -1.0, 0.0])
                + (wedge_g0.zxyw() * other.group1().yzxw())
                + (wedge_g0.wwwx() * other.group0().with_w(other[e15]))
                + (-(wedge_g0.yz() * other.group1().zx()).with_z(0.0) - (Simd32x3::from(wedge_g1) * other.group2().xyz())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0       10        0      N/A
    //  no simd        0       26        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let wedge_g2 = Simd32x4::from(other[e12345] * -1.0) * self.group2();
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other[e12345]) * (self.group0() * Simd32x2::from(other[e12345] * -1.0).with_z(other[e12345]) * Simd32x3::from([1.0, 1.0, -1.0])).with_w(wedge_g2[3]),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group1(),
            // e235, e315, e125, e5
            wedge_g2 * Simd32x2::from(other[e12345]).with_zw(other[e12345], other[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}
impl ProjectViaOriginOnto<Flector> for CircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       17        0        0
    //    simd2        2        4        0      N/A
    //    simd3        2        5        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd       12       26        0      N/A
    //  no simd       21       40        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([0.0, 0.0, (self[e415] * other[e4315]) - (self[e425] * other[e4235]), 0.0])
            + ((Simd32x3::from(other[e3215]) * self.group0()) + ((self.group1().yz() * other.group1().zx()) - (self.group1().zx() * other.group1().yz())).with_z(0.0)).with_w(0.0);
        let wedge_g1 = (self[e423] * other[e4235]) + (self[e431] * other[e4315]) + (self[e412] * other[e4125]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(wedge_g1 * -1.0) * other.group1().xyz(),
            // e415, e425, e435, e321
            ((wedge_g0.yz() * other.group1().zx()) - (wedge_g0.zx() * other.group1().yz()))
                .with_zw((wedge_g0[0] * other[e4315]) - (wedge_g0[1] * other[e4235]), wedge_g1 * other[e3215] * -1.0),
            // e235, e315, e125, e4
            ((Simd32x3::from(wedge_g0[3]) * other.group1().xyz()) - (Simd32x3::from(other[e3215]) * wedge_g0.xyz())).with_w(wedge_g1 * other[e45] * -1.0),
            // e1, e2, e3, e5
            (Simd32x3::from(wedge_g1 * -1.0) * other.group0().xyz())
                .with_w((wedge_g0[0] * other[e15]) + (wedge_g0[1] * other[e25]) + (wedge_g0[2] * other[e35]) + (wedge_g0[3] * other[e45])),
        )
    }
}
impl ProjectViaOriginOnto<Line> for CircleRotor {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       12        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = -(self[e423] * other[e235])
            - (self[e431] * other[e315])
            - (self[e412] * other[e125])
            - (other[e415] * self[e415])
            - (other[e425] * self[e425])
            - (other[e435] * self[e435]);
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(wedge_g0) * other.group0(),
            // e235, e315, e125
            Simd32x3::from(wedge_g0) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       11        0        0
    //    simd3        4        8        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       10       21        0      N/A
    //  no simd       18       43        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0 = Simd32x3::from(right_dual_g0_w) * self.group0();
        let wedge_g1 = Simd32x4::from(right_dual_g0_w) * self.group1();
        let wedge_g2_w = (right_dual_g0_w * self[e12345])
            - (right_dual_g0_xyz[0] * self[e415])
            - (right_dual_g0_xyz[1] * self[e425])
            - (right_dual_g0_xyz[2] * self[e435])
            - (right_dual_g1_xyz[0] * self[e423])
            - (right_dual_g1_xyz[1] * self[e431])
            - (right_dual_g1_xyz[2] * self[e412]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other[e12345]) * wedge_g0.with_w(wedge_g2_w),
            // e415, e425, e435, e321
            ((Simd32x3::from(wedge_g2_w) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g1.xyz())).with_w(wedge_g1[3] * other[e12345]),
            // e235, e315, e125, e5
            ((Simd32x3::from(wedge_g2_w) * other.group1().xyz()) + (Simd32x3::from(right_dual_g0_w * other[e12345]) * self.group2().xyz())).with_w(wedge_g2_w * other[e5]),
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g1[3]) * other.group0().xyz()) + (wedge_g0.zxy() * other.group1().yzx()) - (wedge_g0.yzx() * other.group1().zxy())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       40       53        0        0
    //    simd2        0        2        0      N/A
    //    simd3       37       47        0      N/A
    //    simd4        4        6        0      N/A
    // Totals...
    // yes simd       81      108        0      N/A
    //  no simd      167      222        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let right_dual_g5 = other.group6().xyz();
        let wedge_g0_y = (right_dual_g0[0] * self[e12345]) + (self[e321] * other[e321])
            - (right_dual_g5[0] * self[e415])
            - (right_dual_g5[1] * self[e425])
            - (right_dual_g5[2] * self[e435])
            - (self[e423] * other[e235])
            - (self[e431] * other[e315])
            - (self[e412] * other[e125])
            - (other[e423] * self[e235])
            - (other[e431] * self[e315])
            - (other[e412] * self[e125]);
        let wedge_g6 = Simd32x4::from(right_dual_g0[0]) * self.group1();
        let wedge_g7 = Simd32x3::from(right_dual_g0[0]) * self.group0();
        let wedge_g8 = Simd32x3::from(right_dual_g0[0]) * self.group2().xyz();
        let wedge_g9 = ((Simd32x3::from(other[e3215]) * self.group0()) + (right_dual_g1_xyz.yzx() * self.group1().zxy())
            - (Simd32x3::from(other[e1234]) * self.group2().xyz())
            - (right_dual_g1_xyz.zxy() * self.group1().yzx()))
        .with_w((right_dual_g1_xyz[0] * self[e235]) + (self[e321] * other[e3215]));
        let wedge_g10 = -(right_dual_g1_xyz[0] * self[e423]) - (right_dual_g1_xyz[1] * self[e431]) - (right_dual_g1_xyz[2] * self[e412]) - (self[e321] * other[e1234]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar])
                    + (wedge_g10 * other[e5])
                    + (wedge_g9[0] * other[e1])
                    + (wedge_g9[1] * other[e2])
                    + (wedge_g9[2] * other[e3])
                    + (wedge_g9[3] * other[e4])
                    - (wedge_g7[0] * other[e15])
                    - (wedge_g7[1] * other[e25])
                    - (wedge_g7[2] * other[e35])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43])
                    - (other[e23] * wedge_g6[0])
                    - (other[e31] * wedge_g6[1])
                    - (other[e12] * wedge_g6[2])
                    - (wedge_g6[3] * other[e45]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g0_y) * other.group1().xyz())
                + (Simd32x3::from(wedge_g6[3]) * other.group6().xyz())
                + (Simd32x3::from(wedge_g9[3]) * other.group4())
                + (Simd32x3::from(other[e321]) * wedge_g6.xyz())
                + (wedge_g7.zxy() * other.group8().yzx())
                + (wedge_g8.yzx() * other.group7().zxy())
                + (other.group5().yzx() * wedge_g9.zxy())
                - (Simd32x3::from(wedge_g10) * other.group3().xyz())
                - (wedge_g7.yzx() * other.group8().zxy())
                - (wedge_g8.zxy() * other.group7().yzx())
                - (other.group5().zxy() * wedge_g9.yzx()))
            .with_w(wedge_g0_y * other[e4]),
            // e5
            (wedge_g0_y * other[e5]) + (wedge_g9[0] * other[e15]) + (wedge_g9[1] * other[e25]) + (wedge_g9[2] * other[e35]) + (wedge_g9[3] * other[e45])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435])
                - (other[e235] * wedge_g6[0])
                - (other[e315] * wedge_g6[1])
                - (other[e125] * wedge_g6[2]),
            // e15, e25, e35, e45
            ((Simd32x3::from(wedge_g0_y) * other.group3().xyz())
                + (Simd32x3::from(wedge_g9[3]) * other.group6().xyz())
                + (Simd32x3::from(other[e3215]) * wedge_g6.xyz())
                + (wedge_g8.yzx() * other.group9().zxy())
                + (other.group8().yzx() * wedge_g9.zxy())
                - (wedge_g8.zxy() * other.group9().yzx())
                - (other.group8().zxy() * wedge_g9.yzx()))
            .with_w(wedge_g0_y * other[e45]),
            // e41, e42, e43
            (Simd32x3::from(wedge_g0_y) * other.group4())
                + (Simd32x3::from(wedge_g10) * other.group6().xyz())
                + (Simd32x3::from(other[e1234]) * wedge_g6.xyz())
                + (wedge_g7.zxy() * other.group9().yzx())
                + (other.group7().zxy() * wedge_g9.yzx())
                - (wedge_g7.yzx() * other.group9().zxy())
                - (other.group7().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (wedge_g7 * Simd32x3::from(other[e3215]))
                + (wedge_g8 * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(wedge_g0_y) * other.group5())
                + (Simd32x3::from(wedge_g10) * other.group8())
                + (Simd32x3::from(wedge_g9[3]) * other.group7())
                - (Simd32x3::from(wedge_g6[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            (wedge_g6 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group6())
                + (wedge_g9.yzxw() * other.group9().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group9().yz()).with_zw(wedge_g9[1] * other[e4235] * -1.0, wedge_g10 * other[e3215] * -1.0),
            // e423, e431, e412
            (wedge_g7 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group7()) + (Simd32x3::from(other[e1234]) * wedge_g9.xyz())
                - (Simd32x3::from(wedge_g10) * other.group9().xyz()),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (wedge_g9 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group9()),
            // e1234
            (wedge_g0_y * other[e1234]) + (wedge_g10 * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for CircleRotor {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       10        0        0
    //    simd2        2        4        0      N/A
    //    simd3        2        4        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        9       18        0      N/A
    //  no simd       18       30        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([0.0, 0.0, (self[e415] * other[e4315]) - (self[e425] * other[e4235]), 0.0])
            + ((Simd32x3::from(other[e3215]) * self.group0()) + ((self.group1().yz() * other.group0().zx()) - (self.group1().zx() * other.group0().yz())).with_z(0.0)).with_w(0.0);
        let wedge_g1 = (self[e423] * other[e4235]) + (self[e431] * other[e4315]) + (self[e412] * other[e4125]);
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(wedge_g1 * -1.0) * other.group0().xyz(),
            // e415, e425, e435, e321
            ((wedge_g0.yz() * other.group0().zx()) - (wedge_g0.zx() * other.group0().yz()))
                .with_zw((wedge_g0[0] * other[e4315]) - (wedge_g0[1] * other[e4235]), wedge_g1 * other[e3215] * -1.0),
            // e235, e315, e125
            (Simd32x3::from(wedge_g0[3]) * other.group0().xyz()) - (Simd32x3::from(other[e3215]) * wedge_g0.xyz()),
        )
    }
}
impl ProjectViaOriginOnto<Sphere> for CircleRotor {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        7        0        0
    //    simd3        5       10        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       10       18        0      N/A
    //  no simd       23       41        0        0
    fn project_via_origin_onto(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = (Simd32x3::from(other[e3215]) * self.group0()) + (right_dual_g0_xyz.yzx() * self.group1().zxy())
            - (Simd32x3::from(other[e1234]) * self.group2().xyz())
            - (right_dual_g0_xyz.zxy() * self.group1().yzx());
        let wedge_g0_w = (right_dual_g0_xyz[0] * self[e235]) + (self[e321] * other[e3215]);
        let wedge_g1 = -(right_dual_g0_xyz[0] * self[e423]) - (right_dual_g0_xyz[1] * self[e431]) - (right_dual_g0_xyz[2] * self[e412]) - (self[e321] * other[e1234]);
        Circle::from_groups(
            // e423, e431, e412
            (wedge_g0_xyz * Simd32x3::from(other[e1234])) - (Simd32x3::from(wedge_g1) * other.group0().xyz()),
            // e415, e425, e435, e321
            (wedge_g0_xyz.yzx() * other.group0().zxy()).with_w(wedge_g0_w * other[e1234]) - (other.group0().yzxw() * wedge_g0_xyz.zxy().with_w(wedge_g1)),
            // e235, e315, e125
            (Simd32x3::from(wedge_g0_w) * other.group0().xyz()) - (wedge_g0_xyz * Simd32x3::from(other[e3215])),
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       15        0        0
    //    simd3        8       13        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd       19       31        0      N/A
    //  no simd       38       66        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g2_xyz = other.group2().xyz();
        let wedge_g0 = Simd32x3::from(right_dual_g0_w) * self.group0();
        let wedge_g1 = Simd32x4::from(right_dual_g0_w) * self.group1();
        let wedge_g2_xyz = Simd32x3::from(right_dual_g0_w) * self.group2().xyz();
        let wedge_g2_w = (right_dual_g0_w * self[e12345]) + (self[e321] * other[e321])
            - (right_dual_g0_xyz[0] * self[e235])
            - (right_dual_g0_xyz[1] * self[e315])
            - (right_dual_g0_xyz[2] * self[e125])
            - (right_dual_g1_xyz[0] * self[e415])
            - (right_dual_g1_xyz[1] * self[e425])
            - (right_dual_g1_xyz[2] * self[e435])
            - (right_dual_g2_xyz[0] * self[e423])
            - (right_dual_g2_xyz[1] * self[e431])
            - (right_dual_g2_xyz[2] * self[e412]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((wedge_g0 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g2_w) * other.group0().xyz())).with_w(wedge_g2_w * other[e12345]),
            // e415, e425, e435, e321
            (wedge_g1 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g2_w) * other.group1()),
            // e235, e315, e125, e5
            ((wedge_g2_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g2_w) * other.group2().xyz())).with_w(wedge_g2_w * other[e5]),
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g2_w) * other.group3().xyz())
                + (Simd32x3::from(wedge_g1[3]) * other.group1().xyz())
                + (Simd32x3::from(other[e321]) * wedge_g1.xyz())
                + (wedge_g0.zxy() * other.group2().yzx())
                + (wedge_g2_xyz.yzx() * other.group0().zxy())
                - (wedge_g0.yzx() * other.group2().zxy())
                - (wedge_g2_xyz.zxy() * other.group0().yzx()))
            .with_w(wedge_g2_w * other[e4]),
        )
    }
}
impl ProjectViaOriginOnto<VersorOdd> for CircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       11        0        0
    //    simd3        8       14        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       14       26        0      N/A
    //  no simd       33       57        0        0
    fn project_via_origin_onto(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = (Simd32x3::from(other[e3215]) * self.group0()) + (right_dual_g3_xyz.yzx() * self.group1().zxy())
            - (Simd32x3::from(other[e1234]) * self.group2().xyz())
            - (right_dual_g3_xyz.zxy() * self.group1().yzx());
        let wedge_g0_w = (right_dual_g3_xyz[0] * self[e235]) + (self[e321] * other[e3215]);
        let wedge_g1 = -(right_dual_g3_xyz[0] * self[e423]) - (right_dual_g3_xyz[1] * self[e431]) - (right_dual_g3_xyz[2] * self[e412]) - (self[e321] * other[e1234]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (wedge_g0_xyz * Simd32x3::from(other[e1234])) - (Simd32x3::from(wedge_g1) * other.group3().xyz()),
            // e415, e425, e435, e321
            (wedge_g0_xyz.yzx() * other.group3().zxy()).with_w(wedge_g0_w * other[e1234]) - (other.group3().yzxw() * wedge_g0_xyz.zxy().with_w(wedge_g1)),
            // e235, e315, e125, e4
            ((Simd32x3::from(wedge_g0_w) * other.group3().xyz()) - (wedge_g0_xyz * Simd32x3::from(other[e3215]))).with_w(wedge_g1 * other[e45] * -1.0),
            // e1, e2, e3, e5
            ((Simd32x3::from(wedge_g0_w) * other.group0().xyz()) + (wedge_g0_xyz.zxy() * other.group1().yzx())
                - (Simd32x3::from(wedge_g1) * other.group2().xyz())
                - (wedge_g0_xyz.yzx() * other.group1().zxy()))
            .with_w((wedge_g0_xyz[0] * other[e15]) + (wedge_g0_xyz[1] * other[e25])),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for Dipole {
    type Output = ProjectViaOriginOntoInfixPartial<Dipole>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiCircleRotor> for Dipole {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       10        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        9       13        0      N/A
    //  no simd        9       21        0        0
    fn project_via_origin_onto(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (other[e41] * self[e15])
            + (other[e42] * self[e25])
            + (other[e43] * self[e35])
            + (self[e41] * other[e15])
            + (self[e42] * other[e25])
            + (self[e43] * other[e35])
            + (other[e23] * self[e23])
            + (other[e31] * self[e31])
            + (other[e12] * self[e12])
            - (other[e45] * self[e45]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g0) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(wedge_g0) * other.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(wedge_g0) * other.group2(),
        )
    }
}
impl ProjectViaOriginOnto<AntiDipoleInversion> for Dipole {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        8        0        0
    //    simd3        9       13        0      N/A
    // Totals...
    // yes simd       14       21        0      N/A
    //  no simd       32       47        0        0
    fn project_via_origin_onto(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g2 = other.group2().xyz().with_w(other[e4] * -1.0);
        let wedge_g0_xyz = (right_dual_g1_xyz * Simd32x3::from(self[e45])) + (other.group0().yzx() * self.group2().zxy()) + (self.group0().yzx() * right_dual_g2.zxy())
            - (Simd32x3::from(other[e321]) * self.group1().xyz())
            - (other.group0().zxy() * self.group2().yzx())
            - (self.group0().zxy() * right_dual_g2.yzx());
        let wedge_g1 = -(right_dual_g1_xyz[0] * self[e41])
            - (right_dual_g1_xyz[1] * self[e42])
            - (right_dual_g1_xyz[2] * self[e43])
            - (other[e423] * self[e23])
            - (other[e431] * self[e31])
            - (other[e412] * self[e12]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(wedge_g1) * other.group1().xyz()) + (wedge_g0_xyz.yzx() * other.group0().zxy()) - (wedge_g0_xyz.zxy() * other.group0().yzx()),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g1) * other.group2().xyz()) - (wedge_g0_xyz * Simd32x3::from(other[e321]))).with_w(0.0),
            // e15, e25, e35, scalar
            ((wedge_g0_xyz.zxy() * other.group2().yzx()) - (wedge_g0_xyz.yzx() * other.group2().zxy())).with_w(wedge_g0_xyz[0] * other[e1]),
        )
    }
}
impl ProjectViaOriginOnto<AntiDualNum> for Dipole {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        9        0        0
    fn project_via_origin_onto(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([1.0, 1.0, other[e3215] * other[e3215], 0.0]) * (self.group0() * Simd32x2::from(other[e3215] * other[e3215]).with_z(1.0)).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlatPoint> for Dipole {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        3        6        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        9       19        0        0
    fn project_via_origin_onto(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let wedge_g0_xyz = (right_dual_g0_xyz.zxy() * self.group0().yzx()) - (Simd32x3::from(other[e321]) * self.group1().xyz()) - (right_dual_g0_xyz.yzx() * self.group0().zxy());
        AntiLine::from_groups(
            // e23, e31, e12
            wedge_g0_xyz * Simd32x3::from(other[e321] * -1.0),
            // e15, e25, e35
            (wedge_g0_xyz.zxy() * other.group0().yzx()) - (wedge_g0_xyz.yzx() * other.group0().zxy()),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlector> for Dipole {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        3        7        0      N/A
    // Totals...
    // yes simd        5       10        0      N/A
    //  no simd       11       24        0        0
    fn project_via_origin_onto(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let wedge_g0_xyz = (right_dual_g0_xyz.zxy() * self.group0().yzx()) - (Simd32x3::from(other[e321]) * self.group1().xyz()) - (right_dual_g0_xyz.yzx() * self.group0().zxy());
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (wedge_g0_xyz * Simd32x4::from(other[e321]).xyz() * Simd32x3::from(-1.0))
                .with_w((wedge_g0_xyz[0] * other[e1]) + (wedge_g0_xyz[1] * other[e2]) + (wedge_g0_xyz[2] * other[e3])),
            // e15, e25, e35, e3215
            ((wedge_g0_xyz.zxy() * other.group0().yzx()) - (wedge_g0_xyz.yzx() * other.group0().zxy())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<AntiLine> for Dipole {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       12        0        0
    fn project_via_origin_onto(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 =
            (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43]);
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(wedge_g0) * other.group0(),
            // e15, e25, e35
            Simd32x3::from(wedge_g0) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<AntiMotor> for Dipole {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       11        0        0
    //    simd3        1        4        0      N/A
    // Totals...
    // yes simd        9       15        0      N/A
    //  no simd       11       23        0        0
    fn project_via_origin_onto(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(other[e3215]) * self.group0();
        let wedge_g0_w =
            (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]) + (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(wedge_g0_w) * other.group0().xyz())
                .with_w((wedge_g0_w * other[scalar]) - (wedge_g0_xyz[0] * other[e23]) - (wedge_g0_xyz[1] * other[e31]) - (wedge_g0_xyz[2] * other[e12])),
            // e15, e25, e35, e3215
            ((wedge_g0_xyz * Simd32x3::from(other[e3215])) + (Simd32x3::from(wedge_g0_w) * other.group1().xyz())).with_w(wedge_g0_w * other[e3215]),
        )
    }
}
impl ProjectViaOriginOnto<AntiScalar> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        7        0      N/A
    //  no simd        0       14        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e12345] * -1.0;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual_g0 * other[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group1(),
            // e15, e25, e35
            Simd32x3::from(right_dual_g0 * other[e12345]) * self.group2(),
        )
    }
}
impl ProjectViaOriginOnto<Circle> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        9       13        0      N/A
    // Totals...
    // yes simd       14       19        0      N/A
    //  no simd       32       45        0        0
    fn project_via_origin_onto(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0_xyz = (right_dual_g1_xyz * Simd32x3::from(self[e45])) + (other.group0().yzx() * self.group2().zxy()) + (other.group2().zxy() * self.group0().yzx())
            - (Simd32x3::from(other[e321]) * self.group1().xyz())
            - (other.group0().zxy() * self.group2().yzx())
            - (other.group2().yzx() * self.group0().zxy());
        let wedge_g1 = -(right_dual_g1_xyz[0] * self[e41])
            - (right_dual_g1_xyz[1] * self[e42])
            - (right_dual_g1_xyz[2] * self[e43])
            - (other[e423] * self[e23])
            - (other[e431] * self[e31])
            - (other[e412] * self[e12]);
        Dipole::from_groups(
            // e41, e42, e43
            (Simd32x3::from(wedge_g1) * other.group1().xyz()) + (wedge_g0_xyz.yzx() * other.group0().zxy()) - (wedge_g0_xyz.zxy() * other.group0().yzx()),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g1) * other.group2()) - (wedge_g0_xyz * Simd32x3::from(other[e321]))).with_w(0.0),
            // e15, e25, e35
            (wedge_g0_xyz.zxy() * other.group2().yzx()) - (wedge_g0_xyz.yzx() * other.group2().zxy()),
        )
    }
}
impl ProjectViaOriginOnto<CircleRotor> for Dipole {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       19        0        0
    //    simd3       13       16        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       21       37        0      N/A
    //  no simd       47       75        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g2_xyz = other.group2().xyz();
        let right_dual_g2_w = other[e12345] * -1.0;
        let wedge_g1 = Simd32x4::from(right_dual_g2_w) * self.group1();
        let wedge_g2_w = -(right_dual_g1_xyz[0] * self[e41])
            - (right_dual_g1_xyz[1] * self[e42])
            - (right_dual_g1_xyz[2] * self[e43])
            - (other[e423] * self[e23])
            - (other[e431] * self[e31])
            - (other[e412] * self[e12]);
        let wedge_g3 = ((right_dual_g1_xyz * Simd32x3::from(self[e45])) + (right_dual_g2_xyz.zxy() * self.group0().yzx()) + (other.group0().yzx() * self.group2().zxy())
            - (Simd32x3::from(other[e321]) * self.group1().xyz())
            - (right_dual_g2_xyz.yzx() * self.group0().zxy())
            - (other.group0().zxy() * self.group2().yzx()))
        .with_w(0.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x3::from(wedge_g2_w) * other.group1().xyz()) + (Simd32x3::from(right_dual_g2_w * other[e12345]) * self.group0()) + (other.group0().zxy() * wedge_g3.yzx())
                - (other.group0().yzx() * wedge_g3.zxy()))
            .with_w(wedge_g1[3] * other[e321] * -1.0),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g2_w) * other.group2().xyz()) + (Simd32x3::from(wedge_g3[3]) * other.group0()) + (Simd32x3::from(other[e12345]) * wedge_g1.xyz())
                - (Simd32x3::from(other[e321]) * wedge_g3.xyz()))
            .with_w(wedge_g1[3] * other[e12345]),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g3[2] * other[e315]) - (wedge_g3[1] * other[e125]),
                (wedge_g3[0] * other[e125]) - (wedge_g3[2] * other[e235]),
                (wedge_g3[1] * other[e235]) - (wedge_g3[0] * other[e315]),
            ]) + (Simd32x3::from(wedge_g3[3]) * other.group1().xyz())
                + (Simd32x3::from(right_dual_g2_w * other[e12345]) * self.group2()))
            .with_w(wedge_g2_w * other[e12345]),
            // e4235, e4315, e4125, e3215
            wedge_g3 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Dipole> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       10        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        9       13        0      N/A
    //  no simd        9       20        0        0
    fn project_via_origin_onto(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (other[e41] * self[e15])
            + (other[e42] * self[e25])
            + (other[e43] * self[e35])
            + (other[e15] * self[e41])
            + (other[e25] * self[e42])
            + (other[e35] * self[e43])
            + (other[e23] * self[e23])
            + (other[e31] * self[e31])
            + (other[e12] * self[e12])
            - (other[e45] * self[e45]);
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g0) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(wedge_g0) * other.group1(),
            // e15, e25, e35
            Simd32x3::from(wedge_g0) * other.group2(),
        )
    }
}
impl ProjectViaOriginOnto<DipoleInversion> for Dipole {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3       11       17        0      N/A
    // no simd       33       51        0        0
    fn project_via_origin_onto(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (self.group0().zxy() * other.group3().yzx()) - (self.group0().yzx() * other.group3().zxy());
        let wedge_g1_xyz = (Simd32x3::from(self[e45]) * other.group3().xyz()) + (Simd32x3::from(other[e1234]) * self.group2()) + (Simd32x3::from(other[e3215]) * self.group0());
        let wedge_g2_xyz = (Simd32x3::from(other[e3215]) * self.group1().xyz()) + (self.group2().yzx() * other.group3().zxy()) - (self.group2().zxy() * other.group3().yzx());
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((wedge_g1_xyz * Simd32x3::from(other[e1234])) + (wedge_g0.zxy() * other.group3().yzx()) - (wedge_g0.yzx() * other.group3().zxy())).with_w(0.0),
            // e23, e31, e12, e45
            ((wedge_g0 * Simd32x3::from(other[e3215])) + (wedge_g2_xyz * Simd32x3::from(other[e1234]))).with_w(0.0),
            // e15, e25, e35, e1234
            ((wedge_g1_xyz * Simd32x3::from(other[e3215])) + (wedge_g2_xyz.yzx() * other.group3().zxy()) - (wedge_g2_xyz.zxy() * other.group3().yzx())).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        9        0      N/A
    //  no simd        0       16        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e12345] * other[e12345] * -1.0) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group1(),
            // e15, e25, e35
            Simd32x3::from(other[e12345] * other[e12345] * -1.0) * self.group2(),
        )
    }
}
impl ProjectViaOriginOnto<FlatPoint> for Dipole {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        8        0        0
    //    simd3        0        1        0      N/A
    //    simd4        3        5        0      N/A
    // Totals...
    // yes simd        3       14        0      N/A
    //  no simd       12       31        0        0
    fn project_via_origin_onto(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            (Simd32x4::from([self[e41] * other[e15], self[e42] * other[e25], self[e43] * other[e35], self[e41] * other[e45]]) * other.group0().xyzx())
                + (self.group0().yxxy() * other.group0().xxxy() * other.group0().yyzw())
                + (self.group0().zzyz() * other.group0().xyyz() * other.group0().zzzw())
                + -(Simd32x3::from(self[e45] * other[e45]) * other.group0().xyz()).with_w(other[e45] * other[e45] * self[e45] * -1.0),
        )
    }
}
impl ProjectViaOriginOnto<Flector> for Dipole {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        7       13        0      N/A
    // Totals...
    // yes simd        9       16        0      N/A
    //  no simd       23       42        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (self.group0().zxy() * other.group1().yzx()) - (self.group0().yzx() * other.group1().zxy());
        let wedge_g1_xyz = (Simd32x3::from(self[e45]) * other.group1().xyz()) + (Simd32x3::from(other[e3215]) * self.group0());
        let wedge_g2_xyz = (Simd32x3::from(other[e3215]) * self.group1().xyz()) + (self.group2().yzx() * other.group1().zxy()) - (self.group2().zxy() * other.group1().yzx());
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((wedge_g0.zxy() * other.group1().yzx()) - (wedge_g0.yzx() * other.group1().zxy())).with_w(0.0),
            // e23, e31, e12, e45
            (wedge_g0 * Simd32x4::from(other[e3215]).xyz()).with_w(-(wedge_g1_xyz[0] * other[e4235]) - (wedge_g1_xyz[1] * other[e4315]) - (wedge_g1_xyz[2] * other[e4125])),
            // e15, e25, e35, e1234
            ((wedge_g1_xyz * Simd32x3::from(other[e3215])) + (wedge_g2_xyz.yzx() * other.group1().zxy()) - (wedge_g2_xyz.zxy() * other.group1().yzx())).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}
impl ProjectViaOriginOnto<Line> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        3        7        0      N/A
    // Totals...
    // yes simd        7       13        0      N/A
    //  no simd       13       27        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = (Simd32x3::from(self[e45]) * other.group0()) + (self.group0().yzx() * other.group1().zxy()) - (self.group0().zxy() * other.group1().yzx());
        let wedge_g1 = -(self[e41] * other[e415]) - (self[e42] * other[e425]) - (self[e43] * other[e435]);
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g1) * other.group0(),
            // e23, e31, e12, e45
            (Simd32x3::from(wedge_g1) * other.group1()).with_w(-(wedge_g0_xyz[0] * other[e415]) - (wedge_g0_xyz[1] * other[e425]) - (wedge_g0_xyz[2] * other[e435])),
            // e15, e25, e35
            (wedge_g0_xyz.zxy() * other.group1().yzx()) - (wedge_g0_xyz.yzx() * other.group1().zxy()),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for Dipole {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       16        0        0
    //    simd3        6        9        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       11       27        0      N/A
    //  no simd       23       51        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let right_dual_g1 = other.group1().xyz().with_w(other[e5] * -1.0);
        let wedge_g1 = Simd32x4::from(right_dual_g0_w) * self.group1();
        let wedge_g2_w = -(right_dual_g0_xyz[0] * self[e41]) - (right_dual_g0_xyz[1] * self[e42]) - (right_dual_g0_xyz[2] * self[e43]);
        let wedge_g3 = ((right_dual_g0_xyz * Simd32x3::from(self[e45])) + (self.group0().yzx() * right_dual_g1.zxy()) - (self.group0().zxy() * right_dual_g1.yzx())).with_w(0.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x3::from(wedge_g2_w) * other.group0().xyz()) + (Simd32x3::from(right_dual_g0_w * other[e12345]) * self.group0())).with_w(wedge_g2_w * other[e5]),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g2_w) * other.group1().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g1.xyz())).with_w(wedge_g1[3] * other[e12345]),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g3[2] * other[e315]) - (wedge_g3[1] * other[e125]),
                (wedge_g3[0] * other[e125]) - (wedge_g3[2] * other[e235]),
                (wedge_g3[1] * other[e235]) - (wedge_g3[0] * other[e315]),
            ]) + (Simd32x3::from(wedge_g3[3]) * other.group0().xyz())
                + (Simd32x3::from(right_dual_g0_w * other[e12345]) * self.group2()))
            .with_w(wedge_g2_w * other[e12345]),
            // e4235, e4315, e4125, e3215
            wedge_g3 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for Dipole {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       56       67        0        0
    //    simd2        0        2        0      N/A
    //    simd3       48       62        0      N/A
    //    simd4        8       10        0      N/A
    // Totals...
    // yes simd      112      141        0      N/A
    //  no simd      232      297        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let right_dual_g5 = other.group6().xyz();
        let right_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let wedge_g0_y =
            (self[e15] * other[e41]) + (self[e25] * other[e42]) + (self[e35] * other[e43]) + (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12])
                - (right_dual_g8[0] * self[e41])
                - (right_dual_g8[1] * self[e42])
                - (right_dual_g8[2] * self[e43])
                - (self[e45] * other[e45]);
        let wedge_g3 = Simd32x4::from(right_dual_g0[0]) * self.group2().with_w(self[e45]);
        let wedge_g4 = Simd32x3::from(right_dual_g0[0]) * self.group0();
        let wedge_g5 = Simd32x3::from(right_dual_g0[0]) * self.group1().xyz();
        let wedge_g6 =
            ((Simd32x3::from(other[e3215]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group2()) - (right_dual_g1_xyz * Simd32x3::from(self[e45]))).with_w(0.0);
        let wedge_g7 = (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (right_dual_g1_xyz.zxy() * self.group0().yzx()) - (right_dual_g1_xyz.yzx() * self.group0().zxy());
        let wedge_g8 = (Simd32x3::from(other[e3215]) * self.group1().xyz()) + (right_dual_g1_xyz.yzx() * self.group2().zxy()) - (right_dual_g1_xyz.zxy() * self.group2().yzx());
        let wedge_g9 = ((right_dual_g5 * Simd32x3::from(self[e45])) + (self.group0().yzx() * other.group8().zxy()) + (self.group2().zxy() * other.group7().yzx())
            - (Simd32x3::from(other[e321]) * self.group1().xyz())
            - (self.group0().zxy() * other.group8().yzx())
            - (self.group2().yzx() * other.group7().zxy()))
        .with_w(0.0);
        let wedge_g10 = -(right_dual_g5[0] * self[e41])
            - (right_dual_g5[1] * self[e42])
            - (right_dual_g5[2] * self[e43])
            - (other[e423] * self[e23])
            - (other[e431] * self[e31])
            - (other[e412] * self[e12]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar])
                    + (wedge_g10 * other[e5])
                    + (wedge_g9[0] * other[e1])
                    + (wedge_g9[1] * other[e2])
                    + (wedge_g9[2] * other[e3])
                    + (wedge_g9[3] * other[e4])
                    - (wedge_g4[0] * other[e235])
                    - (wedge_g4[1] * other[e315])
                    - (wedge_g4[2] * other[e125])
                    - (wedge_g5[0] * other[e415])
                    - (wedge_g5[1] * other[e425])
                    - (wedge_g5[2] * other[e435])
                    - (wedge_g7[0] * other[e15])
                    - (wedge_g7[1] * other[e25])
                    - (wedge_g7[2] * other[e35])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43])
                    - (other[e23] * wedge_g6[0])
                    - (other[e31] * wedge_g6[1])
                    - (other[e12] * wedge_g6[2])
                    - (other[e423] * wedge_g3[0])
                    - (other[e431] * wedge_g3[1])
                    - (other[e412] * wedge_g3[2])
                    - (wedge_g3[3] * other[e321])
                    - (wedge_g6[3] * other[e45]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(wedge_g0_y) * other.group1())
                + (other.group9().yzxx() * wedge_g5.zxy().with_w(wedge_g4[0]))
                + ((Simd32x3::from(wedge_g6[3]) * other.group6().xyz())
                    + (Simd32x3::from(wedge_g9[3]) * other.group4())
                    + (Simd32x3::from(other[e321]) * wedge_g6.xyz())
                    + (Simd32x3::from(other[e1234]) * wedge_g3.xyz())
                    + (wedge_g7.zxy() * other.group8().yzx())
                    + (wedge_g8.yzx() * other.group7().zxy())
                    + (other.group5().yzx() * wedge_g9.zxy())
                    - (wedge_g4 * Simd32x3::from(other[e3215]))
                    - (Simd32x3::from(wedge_g10) * other.group3().xyz())
                    - (wedge_g5.yzx() * other.group9().zxy())
                    - (wedge_g7.yzx() * other.group8().zxy())
                    - (wedge_g8.zxy() * other.group7().yzx())
                    - (other.group5().zxy() * wedge_g9.yzx()))
                .with_w((wedge_g4[1] * other[e4315]) + (wedge_g4[2] * other[e4125]) + (wedge_g3[3] * other[e1234])),
            // e5
            (wedge_g0_y * other[e5]) + (wedge_g9[0] * other[e15]) + (wedge_g9[1] * other[e25]) + (wedge_g9[2] * other[e35]) + (wedge_g9[3] * other[e45])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435])
                - (other[e235] * wedge_g6[0])
                - (other[e315] * wedge_g6[1])
                - (other[e125] * wedge_g6[2])
                - (wedge_g3[0] * other[e4235])
                - (wedge_g3[1] * other[e4315])
                - (wedge_g3[2] * other[e4125])
                - (wedge_g3[3] * other[e3215]),
            // e15, e25, e35, e45
            (wedge_g3 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group3())
                + ((Simd32x3::from(wedge_g9[3]) * other.group6().xyz())
                    + (Simd32x3::from(other[e3215]) * wedge_g6.xyz())
                    + (wedge_g8.yzx() * other.group9().zxy())
                    + (other.group8().yzx() * wedge_g9.zxy())
                    - (wedge_g8.zxy() * other.group9().yzx())
                    - (other.group8().zxy() * wedge_g9.yzx()))
                .with_w(0.0),
            // e41, e42, e43
            (wedge_g4 * Simd32x3::from(other[e12345]))
                + (Simd32x3::from(wedge_g0_y) * other.group4())
                + (Simd32x3::from(wedge_g10) * other.group6().xyz())
                + (Simd32x3::from(other[e1234]) * wedge_g6.xyz())
                + (wedge_g7.zxy() * other.group9().yzx())
                + (other.group7().zxy() * wedge_g9.yzx())
                - (wedge_g7.yzx() * other.group9().zxy())
                - (other.group7().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (wedge_g5 * Simd32x3::from(other[e12345]))
                + (wedge_g7 * Simd32x3::from(other[e3215]))
                + (wedge_g8 * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(wedge_g0_y) * other.group5())
                + (Simd32x3::from(wedge_g10) * other.group8())
                + (Simd32x3::from(wedge_g9[3]) * other.group7())
                - (Simd32x3::from(wedge_g6[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            (wedge_g6 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group6())
                + (wedge_g9.yzxw() * other.group9().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group9().yz()).with_zw(wedge_g9[1] * other[e4235] * -1.0, wedge_g10 * other[e3215] * -1.0),
            // e423, e431, e412
            (wedge_g7 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group7()) + (Simd32x3::from(other[e1234]) * wedge_g9.xyz())
                - (Simd32x3::from(wedge_g10) * other.group9().xyz()),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (wedge_g9 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group9()),
            // e1234
            (wedge_g0_y * other[e1234]) + (wedge_g10 * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        7       12        0      N/A
    // Totals...
    // yes simd        7       14        0      N/A
    //  no simd       21       38        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (self.group0().zxy() * other.group0().yzx()) - (self.group0().yzx() * other.group0().zxy());
        let wedge_g2 = (Simd32x3::from(other[e3215]) * self.group1().xyz()) + (self.group2().yzx() * other.group0().zxy()) - (self.group2().zxy() * other.group0().yzx());
        Dipole::from_groups(
            // e41, e42, e43
            (wedge_g0.zxy() * other.group0().yzx()) - (wedge_g0.yzx() * other.group0().zxy()),
            // e23, e31, e12, e45
            (wedge_g0 * Simd32x4::from(other[e3215]).xyz()).with_w(0.0),
            // e15, e25, e35
            (Simd32x3::from(other[e3215] * other[e3215]) * self.group0())
                + (Simd32x3::from(self[e45] * other[e3215]) * other.group0().xyz())
                + (wedge_g2.yzx() * other.group0().zxy())
                - (wedge_g2.zxy() * other.group0().yzx()),
        )
    }
}
impl ProjectViaOriginOnto<Sphere> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3       11       18        0      N/A
    // no simd       33       54        0        0
    fn project_via_origin_onto(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (right_dual_g0_xyz.zxy() * self.group0().yzx()) - (right_dual_g0_xyz.yzx() * self.group0().zxy());
        let wedge_g1_xyz = (Simd32x3::from(other[e3215]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group2()) - (right_dual_g0_xyz * Simd32x3::from(self[e45]));
        let wedge_g2 = (Simd32x3::from(other[e3215]) * self.group1().xyz()) + (right_dual_g0_xyz.yzx() * self.group2().zxy()) - (right_dual_g0_xyz.zxy() * self.group2().yzx());
        Dipole::from_groups(
            // e41, e42, e43
            (wedge_g1_xyz * Simd32x3::from(other[e1234])) + (wedge_g0.zxy() * other.group0().yzx()) - (wedge_g0.yzx() * other.group0().zxy()),
            // e23, e31, e12, e45
            ((wedge_g0 * Simd32x3::from(other[e3215])) + (wedge_g2 * Simd32x3::from(other[e1234]))).with_w(0.0),
            // e15, e25, e35
            (wedge_g1_xyz * Simd32x3::from(other[e3215])) + (wedge_g2.yzx() * other.group0().zxy()) - (wedge_g2.zxy() * other.group0().yzx()),
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for Dipole {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       18       29        0        0
    //    simd2        0        1        0      N/A
    //    simd3       11       15        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd       31       49        0      N/A
    //  no simd       59       92        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g2 = other.group2().xyz().with_w(other[e4] * -1.0);
        let wedge_g0 = Simd32x3::from(right_dual_g0_w) * self.group0();
        let wedge_g1 = Simd32x4::from(right_dual_g0_w) * self.group1();
        let wedge_g2_xyz = Simd32x3::from(right_dual_g0_w) * self.group2();
        let wedge_g2_w = -(right_dual_g0_xyz[0] * self[e23])
            - (right_dual_g0_xyz[1] * self[e31])
            - (right_dual_g0_xyz[2] * self[e12])
            - (right_dual_g1_xyz[0] * self[e41])
            - (right_dual_g1_xyz[1] * self[e42])
            - (right_dual_g1_xyz[2] * self[e43]);
        let wedge_g3 = ((right_dual_g1_xyz * Simd32x3::from(self[e45])) + (right_dual_g0_xyz.yzx() * self.group2().zxy()) + (self.group0().yzx() * right_dual_g2.zxy())
            - (Simd32x3::from(other[e321]) * self.group1().xyz())
            - (right_dual_g0_xyz.zxy() * self.group2().yzx())
            - (self.group0().zxy() * right_dual_g2.yzx()))
        .with_w(0.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from(wedge_g2_w) * other.group1().xyz().with_w(other[e5]))
                + (wedge_g3.yzxy() * other.group0().zxy().with_w(other[e2]))
                + ((wedge_g0 * Simd32x3::from(other[e12345])) + -(wedge_g3.zx() * other.group0().yz()).with_z(wedge_g3[1] * other[e423] * -1.0)).with_w(
                    (wedge_g3[0] * other[e1])
                        - (wedge_g0[0] * other[e235])
                        - (wedge_g0[1] * other[e315])
                        - (wedge_g0[2] * other[e125])
                        - (wedge_g2_xyz[0] * other[e423])
                        - (wedge_g2_xyz[1] * other[e431])
                        - (wedge_g2_xyz[2] * other[e412])
                        - (wedge_g1[0] * other[e415])
                        - (wedge_g1[1] * other[e425])
                        - (wedge_g1[2] * other[e435])
                        - (wedge_g1[3] * other[e321]),
                ),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g2_w) * other.group2().xyz()) + (Simd32x3::from(wedge_g3[3]) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g1.xyz())
                - (Simd32x3::from(other[e321]) * wedge_g3.xyz()))
            .with_w(wedge_g1[3] * other[e12345]),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g3[2] * other[e315]) - (wedge_g3[1] * other[e125]),
                (wedge_g3[0] * other[e125]) - (wedge_g3[2] * other[e235]),
                (wedge_g3[1] * other[e235]) - (wedge_g3[0] * other[e315]),
            ]) + (wedge_g2_xyz * Simd32x3::from(other[e12345]))
                + (Simd32x3::from(wedge_g3[3]) * other.group1().xyz()))
            .with_w(wedge_g2_w * other[e12345]),
            // e4235, e4315, e4125, e3215
            wedge_g3 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<VersorOdd> for Dipole {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3       11       18        0      N/A
    // no simd       33       54        0        0
    fn project_via_origin_onto(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (right_dual_g3_xyz.zxy() * self.group0().yzx()) - (right_dual_g3_xyz.yzx() * self.group0().zxy());
        let wedge_g1_xyz = (Simd32x3::from(other[e1234]) * self.group2()) + (Simd32x3::from(other[e3215]) * self.group0()) - (right_dual_g3_xyz * Simd32x3::from(self[e45]));
        let wedge_g2_xyz = (Simd32x3::from(other[e3215]) * self.group1().xyz()) + (right_dual_g3_xyz.yzx() * self.group2().zxy()) - (right_dual_g3_xyz.zxy() * self.group2().yzx());
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((wedge_g1_xyz * Simd32x3::from(other[e1234])) + (wedge_g0.zxy() * other.group3().yzx()) - (wedge_g0.yzx() * other.group3().zxy())).with_w(0.0),
            // e23, e31, e12, e45
            ((wedge_g0 * Simd32x3::from(other[e3215])) + (wedge_g2_xyz * Simd32x3::from(other[e1234]))).with_w(0.0),
            // e15, e25, e35, e1234
            ((wedge_g1_xyz * Simd32x3::from(other[e3215])) + (wedge_g2_xyz.yzx() * other.group3().zxy()) - (wedge_g2_xyz.zxy() * other.group3().yzx())).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for DipoleInversion {
    type Output = ProjectViaOriginOntoInfixPartial<DipoleInversion>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiCircleRotor> for DipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       10        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        9       13        0      N/A
    //  no simd        9       21        0        0
    fn project_via_origin_onto(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (other[e41] * self[e15])
            + (other[e42] * self[e25])
            + (other[e43] * self[e35])
            + (self[e41] * other[e15])
            + (self[e42] * other[e25])
            + (self[e43] * other[e35])
            + (other[e23] * self[e23])
            + (other[e31] * self[e31])
            + (other[e12] * self[e12])
            - (other[e45] * self[e45]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g0) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(wedge_g0) * other.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(wedge_g0) * other.group2(),
        )
    }
}
impl ProjectViaOriginOnto<AntiDipoleInversion> for DipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        8        0        0
    //    simd3        9       13        0      N/A
    // Totals...
    // yes simd       14       21        0      N/A
    //  no simd       32       47        0        0
    fn project_via_origin_onto(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g2 = other.group2().xyz().with_w(other[e4] * -1.0);
        let wedge_g0_xyz = (right_dual_g1_xyz * Simd32x3::from(self[e45])) + (other.group0().yzx() * self.group2().zxy()) + (self.group0().yzx() * right_dual_g2.zxy())
            - (Simd32x3::from(other[e321]) * self.group1().xyz())
            - (other.group0().zxy() * self.group2().yzx())
            - (self.group0().zxy() * right_dual_g2.yzx());
        let wedge_g1 = -(right_dual_g1_xyz[0] * self[e41])
            - (right_dual_g1_xyz[1] * self[e42])
            - (right_dual_g1_xyz[2] * self[e43])
            - (other[e423] * self[e23])
            - (other[e431] * self[e31])
            - (other[e412] * self[e12]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(wedge_g1) * other.group1().xyz()) + (wedge_g0_xyz.yzx() * other.group0().zxy()) - (wedge_g0_xyz.zxy() * other.group0().yzx()),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g1) * other.group2().xyz()) - (wedge_g0_xyz * Simd32x3::from(other[e321]))).with_w(0.0),
            // e15, e25, e35, scalar
            ((wedge_g0_xyz.zxy() * other.group2().yzx()) - (wedge_g0_xyz.yzx() * other.group2().zxy())).with_w(wedge_g0_xyz[0] * other[e1]),
        )
    }
}
impl ProjectViaOriginOnto<AntiDualNum> for DipoleInversion {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        9        0        0
    fn project_via_origin_onto(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(other[e3215]) * self.group0().with_w(self[e1234]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(other[scalar] * wedge_g0[3]),
            // e15, e25, e35, e3215
            wedge_g0 * Simd32x4::from(other[e3215]),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlatPoint> for DipoleInversion {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        3        6        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        9       19        0        0
    fn project_via_origin_onto(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let wedge_g0_xyz = (right_dual_g0_xyz.zxy() * self.group0().yzx()) - (Simd32x3::from(other[e321]) * self.group1().xyz()) - (right_dual_g0_xyz.yzx() * self.group0().zxy());
        AntiLine::from_groups(
            // e23, e31, e12
            wedge_g0_xyz * Simd32x3::from(other[e321] * -1.0),
            // e15, e25, e35
            (wedge_g0_xyz.zxy() * other.group0().yzx()) - (wedge_g0_xyz.yzx() * other.group0().zxy()),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlector> for DipoleInversion {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        3        7        0      N/A
    // Totals...
    // yes simd        5       10        0      N/A
    //  no simd       11       24        0        0
    fn project_via_origin_onto(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let wedge_g0_xyz = (right_dual_g0_xyz.zxy() * self.group0().yzx()) - (Simd32x3::from(other[e321]) * self.group1().xyz()) - (right_dual_g0_xyz.yzx() * self.group0().zxy());
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (wedge_g0_xyz * Simd32x4::from(other[e321]).xyz() * Simd32x3::from(-1.0))
                .with_w((wedge_g0_xyz[0] * other[e1]) + (wedge_g0_xyz[1] * other[e2]) + (wedge_g0_xyz[2] * other[e3])),
            // e15, e25, e35, e3215
            ((wedge_g0_xyz.zxy() * other.group0().yzx()) - (wedge_g0_xyz.yzx() * other.group0().zxy())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<AntiLine> for DipoleInversion {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       12        0        0
    fn project_via_origin_onto(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 =
            (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43]);
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(wedge_g0) * other.group0(),
            // e15, e25, e35
            Simd32x3::from(wedge_g0) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<AntiMotor> for DipoleInversion {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       12        0        0
    //    simd3        1        4        0      N/A
    // Totals...
    // yes simd       10       16        0      N/A
    //  no simd       12       24        0        0
    fn project_via_origin_onto(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(other[e3215]) * self.group0();
        let wedge_g0_w = (self[e41] * other[e15])
            + (self[e42] * other[e25])
            + (self[e43] * other[e35])
            + (other[e23] * self[e23])
            + (other[e31] * self[e31])
            + (other[e12] * self[e12])
            + (other[e3215] * self[e1234]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(wedge_g0_w) * other.group0().xyz())
                .with_w((wedge_g0_w * other[scalar]) - (wedge_g0_xyz[0] * other[e23]) - (wedge_g0_xyz[1] * other[e31]) - (wedge_g0_xyz[2] * other[e12])),
            // e15, e25, e35, e3215
            ((wedge_g0_xyz * Simd32x3::from(other[e3215])) + (Simd32x3::from(wedge_g0_w) * other.group1().xyz())).with_w(wedge_g0_w * other[e3215]),
        )
    }
}
impl ProjectViaOriginOnto<AntiScalar> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        9        0      N/A
    //  no simd        0       20        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e12345] * -1.0;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual_g0 * other[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group3(),
        )
    }
}
impl ProjectViaOriginOnto<Circle> for DipoleInversion {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        9       13        0      N/A
    // Totals...
    // yes simd       14       19        0      N/A
    //  no simd       32       45        0        0
    fn project_via_origin_onto(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0_xyz = (right_dual_g1_xyz * Simd32x3::from(self[e45])) + (other.group0().yzx() * self.group2().zxy()) + (other.group2().zxy() * self.group0().yzx())
            - (Simd32x3::from(other[e321]) * self.group1().xyz())
            - (other.group0().zxy() * self.group2().yzx())
            - (other.group2().yzx() * self.group0().zxy());
        let wedge_g1 = -(right_dual_g1_xyz[0] * self[e41])
            - (right_dual_g1_xyz[1] * self[e42])
            - (right_dual_g1_xyz[2] * self[e43])
            - (other[e423] * self[e23])
            - (other[e431] * self[e31])
            - (other[e412] * self[e12]);
        Dipole::from_groups(
            // e41, e42, e43
            (Simd32x3::from(wedge_g1) * other.group1().xyz()) + (wedge_g0_xyz.yzx() * other.group0().zxy()) - (wedge_g0_xyz.zxy() * other.group0().yzx()),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g1) * other.group2()) - (wedge_g0_xyz * Simd32x3::from(other[e321]))).with_w(0.0),
            // e15, e25, e35
            (wedge_g0_xyz.zxy() * other.group2().yzx()) - (wedge_g0_xyz.yzx() * other.group2().zxy()),
        )
    }
}
impl ProjectViaOriginOnto<CircleRotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       21        0        0
    //    simd3       14       17        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       23       40        0      N/A
    //  no simd       51       80        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g2_xyz = other.group2().xyz();
        let right_dual_g2_w = other[e12345] * -1.0;
        let wedge_g1 = Simd32x4::from(right_dual_g2_w) * self.group1();
        let wedge_g2_w = (right_dual_g2_w * self[e1234])
            - (right_dual_g1_xyz[0] * self[e41])
            - (right_dual_g1_xyz[1] * self[e42])
            - (right_dual_g1_xyz[2] * self[e43])
            - (other[e423] * self[e23])
            - (other[e431] * self[e31])
            - (other[e412] * self[e12]);
        let wedge_g3 = ((right_dual_g1_xyz * Simd32x3::from(self[e45]))
            + (Simd32x3::from(right_dual_g2_w) * self.group3().xyz())
            + (right_dual_g2_xyz.zxy() * self.group0().yzx())
            + (other.group0().yzx() * self.group2().zxy())
            - (Simd32x3::from(other[e321]) * self.group1().xyz())
            - (right_dual_g2_xyz.yzx() * self.group0().zxy())
            - (other.group0().zxy() * self.group2().yzx()))
        .with_w(right_dual_g2_w * self[e3215]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x3::from(wedge_g2_w) * other.group1().xyz()) + (Simd32x3::from(right_dual_g2_w * other[e12345]) * self.group0()) + (other.group0().zxy() * wedge_g3.yzx())
                - (other.group0().yzx() * wedge_g3.zxy()))
            .with_w(wedge_g1[3] * other[e321] * -1.0),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g2_w) * other.group2().xyz()) + (Simd32x3::from(wedge_g3[3]) * other.group0()) + (Simd32x3::from(other[e12345]) * wedge_g1.xyz())
                - (Simd32x3::from(other[e321]) * wedge_g3.xyz()))
            .with_w(wedge_g1[3] * other[e12345]),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g3[2] * other[e315]) - (wedge_g3[1] * other[e125]),
                (wedge_g3[0] * other[e125]) - (wedge_g3[2] * other[e235]),
                (wedge_g3[1] * other[e235]) - (wedge_g3[0] * other[e315]),
            ]) + (Simd32x3::from(wedge_g3[3]) * other.group1().xyz())
                + (Simd32x3::from(right_dual_g2_w * other[e12345]) * self.group2().xyz()))
            .with_w(wedge_g2_w * other[e12345]),
            // e4235, e4315, e4125, e3215
            wedge_g3 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Dipole> for DipoleInversion {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       10        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        9       13        0      N/A
    //  no simd        9       20        0        0
    fn project_via_origin_onto(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (other[e41] * self[e15])
            + (other[e42] * self[e25])
            + (other[e43] * self[e35])
            + (other[e15] * self[e41])
            + (other[e25] * self[e42])
            + (other[e35] * self[e43])
            + (other[e23] * self[e23])
            + (other[e31] * self[e31])
            + (other[e12] * self[e12])
            - (other[e45] * self[e45]);
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g0) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(wedge_g0) * other.group1(),
            // e15, e25, e35
            Simd32x3::from(wedge_g0) * other.group2(),
        )
    }
}
impl ProjectViaOriginOnto<DipoleInversion> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       17        0        0
    //    simd3       13       18        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       24       36        0      N/A
    //  no simd       50       75        0        0
    fn project_via_origin_onto(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (self.group0().zxy() * other.group3().yzx()) - (self.group0().yzx() * other.group3().zxy());
        let wedge_g1_xyz =
            (Simd32x3::from(other[e1234]) * self.group2().xyz()) + (Simd32x3::from(other[e3215]) * self.group0()) + (Simd32x3::from(self[e45]) * other.group3().xyz());
        let wedge_g2_xyz = Simd32x3::from([
            (other[e4125] * self[e25]) - (other[e4315] * self[e35]),
            (other[e4235] * self[e35]) - (other[e4125] * self[e15]),
            (other[e4315] * self[e15]) - (other[e4235] * self[e25]),
        ]) + (Simd32x3::from(other[e3215]) * self.group1().xyz());
        let wedge_g2_w = (other[e41] * self[e15])
            + (other[e42] * self[e25])
            + (self[e43] * other[e35])
            + (other[e23] * self[e23])
            + (other[e31] * self[e31])
            + (other[e12] * self[e12])
            + (other[e1234] * self[e3215])
            - (other[e45] * self[e45])
            - (other[e4235] * self[e4235]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((wedge_g1_xyz * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g2_w) * other.group0()) + (wedge_g0.zxy() * other.group3().yzx())
                - (wedge_g0.yzx() * other.group3().zxy()))
            .with_w(0.0),
            // e23, e31, e12, e45
            ((wedge_g0 * Simd32x3::from(other[e3215])) + (wedge_g2_xyz * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g2_w) * other.group1().xyz()))
                .with_w(wedge_g2_w * other[e45]),
            // e15, e25, e35, e1234
            ((wedge_g1_xyz * Simd32x3::from(other[e3215])) + (Simd32x3::from(wedge_g2_w) * other.group2().xyz()) + (wedge_g2_xyz.yzx() * other.group3().zxy())
                - (wedge_g2_xyz.zxy() * other.group3().yzx()))
            .with_w(wedge_g2_w * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g2_w) * other.group3(),
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        9        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0       15        0      N/A
    //  no simd        0       31        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let wedge_g2 = Simd32x4::from(other[e12345] * -1.0) * self.group2();
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() * Simd32x2::from(other[e12345] * other[e12345] * -1.0).with_z(other[e12345] * other[e12345]) * Simd32x3::from([1.0, 1.0, -1.0]))
                .with_w(other[e5] * wedge_g2[3]),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group1(),
            // e15, e25, e35, e1234
            wedge_g2 * Simd32x4::from(other[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group3(),
        )
    }
}
impl ProjectViaOriginOnto<FlatPoint> for DipoleInversion {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        8        0        0
    //    simd3        0        1        0      N/A
    //    simd4        3        5        0      N/A
    // Totals...
    // yes simd        3       14        0      N/A
    //  no simd       12       31        0        0
    fn project_via_origin_onto(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            (Simd32x4::from([self[e41] * other[e15], self[e42] * other[e25], self[e43] * other[e35], self[e41] * other[e45]]) * other.group0().xyzx())
                + (self.group0().yxxy() * other.group0().xxxy() * other.group0().yyzw())
                + (self.group0().zzyz() * other.group0().xyyz() * other.group0().zzzw())
                + -(Simd32x3::from(self[e45] * other[e45]) * other.group0().xyz()).with_w(other[e45] * other[e45] * self[e45] * -1.0),
        )
    }
}
impl ProjectViaOriginOnto<Flector> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       15        0        0
    //    simd3        7       12        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       17       28        0      N/A
    //  no simd       31       55        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (self.group0().zxy() * other.group1().yzx()) - (self.group0().yzx() * other.group1().zxy());
        let wedge_g1_xyz = (Simd32x3::from(self[e45]) * other.group1().xyz()) + (Simd32x3::from(other[e3215]) * self.group0());
        let wedge_g2_xyz = Simd32x3::from([
            (self[e25] * other[e4125]) - (self[e35] * other[e4315]),
            (self[e35] * other[e4235]) - (self[e15] * other[e4125]),
            (self[e15] * other[e4315]) - (self[e25] * other[e4235]),
        ]) + (Simd32x3::from(other[e3215]) * self.group1().xyz());
        let wedge_g2_w = (self[e41] * other[e15]) + (self[e42] * other[e25]) - (self[e45] * other[e45]) - (self[e4235] * other[e4235]) - (self[e4315] * other[e4315]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((wedge_g0.zxy() * other.group1().yzx()) - (wedge_g0.yzx() * other.group1().zxy())).with_w(0.0),
            // e23, e31, e12, e45
            (wedge_g0 * Simd32x4::from(other[e3215]).xyz())
                .with_w((wedge_g2_w * other[e45]) - (wedge_g1_xyz[0] * other[e4235]) - (wedge_g1_xyz[1] * other[e4315]) - (wedge_g1_xyz[2] * other[e4125])),
            // e15, e25, e35, e1234
            ((wedge_g1_xyz * Simd32x3::from(other[e3215])) + (Simd32x3::from(wedge_g2_w) * other.group0().xyz()) + (wedge_g2_xyz.yzx() * other.group1().zxy())
                - (wedge_g2_xyz.zxy() * other.group1().yzx()))
            .with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g2_w) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Line> for DipoleInversion {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        3        7        0      N/A
    // Totals...
    // yes simd        7       13        0      N/A
    //  no simd       13       27        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = (Simd32x3::from(self[e45]) * other.group0()) + (self.group0().yzx() * other.group1().zxy()) - (self.group0().zxy() * other.group1().yzx());
        let wedge_g1 = -(self[e41] * other[e415]) - (self[e42] * other[e425]) - (self[e43] * other[e435]);
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g1) * other.group0(),
            // e23, e31, e12, e45
            (Simd32x3::from(wedge_g1) * other.group1()).with_w(-(wedge_g0_xyz[0] * other[e415]) - (wedge_g0_xyz[1] * other[e425]) - (wedge_g0_xyz[2] * other[e435])),
            // e15, e25, e35
            (wedge_g0_xyz.zxy() * other.group1().yzx()) - (wedge_g0_xyz.yzx() * other.group1().zxy()),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       18        0        0
    //    simd3        7       10        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       13       30        0      N/A
    //  no simd       27       56        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let right_dual_g1 = other.group1().xyz().with_w(other[e5] * -1.0);
        let wedge_g1 = Simd32x4::from(right_dual_g0_w) * self.group1();
        let wedge_g2_w = (right_dual_g0_w * self[e1234]) - (right_dual_g0_xyz[0] * self[e41]) - (right_dual_g0_xyz[1] * self[e42]) - (right_dual_g0_xyz[2] * self[e43]);
        let wedge_g3 = ((right_dual_g0_xyz * Simd32x3::from(self[e45])) + (Simd32x3::from(right_dual_g0_w) * self.group3().xyz()) + (self.group0().yzx() * right_dual_g1.zxy())
            - (self.group0().zxy() * right_dual_g1.yzx()))
        .with_w(right_dual_g0_w * self[e3215]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x3::from(wedge_g2_w) * other.group0().xyz()) + (Simd32x3::from(right_dual_g0_w * other[e12345]) * self.group0())).with_w(wedge_g2_w * other[e5]),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g2_w) * other.group1().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g1.xyz())).with_w(wedge_g1[3] * other[e12345]),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g3[2] * other[e315]) - (wedge_g3[1] * other[e125]),
                (wedge_g3[0] * other[e125]) - (wedge_g3[2] * other[e235]),
                (wedge_g3[1] * other[e235]) - (wedge_g3[0] * other[e315]),
            ]) + (Simd32x3::from(wedge_g3[3]) * other.group0().xyz())
                + (Simd32x3::from(right_dual_g0_w * other[e12345]) * self.group2().xyz()))
            .with_w(wedge_g2_w * other[e12345]),
            // e4235, e4315, e4125, e3215
            wedge_g3 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       62       74        0        0
    //    simd2        0        2        0      N/A
    //    simd3       49       63        0      N/A
    //    simd4        8       10        0      N/A
    // Totals...
    // yes simd      119      149        0      N/A
    //  no simd      241      307        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let right_dual_g5 = other.group6().xyz();
        let right_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let wedge_g0_y = (right_dual_g1_xyz[0] * self[e4235])
            + (right_dual_g1_xyz[1] * self[e4315])
            + (right_dual_g1_xyz[2] * self[e4125])
            + (other[e41] * self[e15])
            + (other[e42] * self[e25])
            + (other[e43] * self[e35])
            + (other[e23] * self[e23])
            + (other[e31] * self[e31])
            + (other[e12] * self[e12])
            + (self[e1234] * other[e3215])
            + (self[e3215] * other[e1234])
            - (right_dual_g8[0] * self[e41])
            - (right_dual_g8[1] * self[e42])
            - (right_dual_g8[2] * self[e43])
            - (self[e45] * other[e45]);
        let wedge_g3 = Simd32x4::from(right_dual_g0[0]) * self.group2().xyz().with_w(self[e45]);
        let wedge_g4 = Simd32x3::from(right_dual_g0[0]) * self.group0();
        let wedge_g5 = Simd32x3::from(right_dual_g0[0]) * self.group1().xyz();
        let wedge_g6 =
            ((Simd32x3::from(other[e3215]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group2().xyz()) - (right_dual_g1_xyz * Simd32x3::from(self[e45]))).with_w(0.0);
        let wedge_g7 = (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (right_dual_g1_xyz.zxy() * self.group0().yzx()) - (right_dual_g1_xyz.yzx() * self.group0().zxy());
        let wedge_g8 = (Simd32x3::from(other[e3215]) * self.group1().xyz()) + (right_dual_g1_xyz.yzx() * self.group2().zxy()) - (right_dual_g1_xyz.zxy() * self.group2().yzx());
        let wedge_g9 = ((right_dual_g5 * Simd32x3::from(self[e45]))
            + (Simd32x3::from(right_dual_g0[0]) * self.group3().xyz())
            + (self.group0().yzx() * other.group8().zxy())
            + (other.group7().yzx() * self.group2().zxy())
            - (Simd32x3::from(other[e321]) * self.group1().xyz())
            - (self.group0().zxy() * other.group8().yzx())
            - (other.group7().zxy() * self.group2().yzx()))
        .with_w(right_dual_g0[0] * self[e3215]);
        let wedge_g10 = (right_dual_g0[0] * self[e1234])
            - (right_dual_g5[0] * self[e41])
            - (right_dual_g5[1] * self[e42])
            - (right_dual_g5[2] * self[e43])
            - (other[e423] * self[e23])
            - (other[e431] * self[e31])
            - (other[e412] * self[e12]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar])
                    + (wedge_g10 * other[e5])
                    + (wedge_g9[0] * other[e1])
                    + (wedge_g9[1] * other[e2])
                    + (wedge_g9[2] * other[e3])
                    + (wedge_g9[3] * other[e4])
                    - (wedge_g4[0] * other[e235])
                    - (wedge_g4[1] * other[e315])
                    - (wedge_g4[2] * other[e125])
                    - (wedge_g5[0] * other[e415])
                    - (wedge_g5[1] * other[e425])
                    - (wedge_g5[2] * other[e435])
                    - (wedge_g7[0] * other[e15])
                    - (wedge_g7[1] * other[e25])
                    - (wedge_g7[2] * other[e35])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43])
                    - (other[e23] * wedge_g6[0])
                    - (other[e31] * wedge_g6[1])
                    - (other[e12] * wedge_g6[2])
                    - (other[e423] * wedge_g3[0])
                    - (other[e431] * wedge_g3[1])
                    - (other[e412] * wedge_g3[2])
                    - (wedge_g3[3] * other[e321])
                    - (wedge_g6[3] * other[e45]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(wedge_g0_y) * other.group1())
                + (other.group9().yzxx() * wedge_g5.zxy().with_w(wedge_g4[0]))
                + ((Simd32x3::from(wedge_g6[3]) * other.group6().xyz())
                    + (Simd32x3::from(wedge_g9[3]) * other.group4())
                    + (Simd32x3::from(other[e321]) * wedge_g6.xyz())
                    + (Simd32x3::from(other[e1234]) * wedge_g3.xyz())
                    + (wedge_g7.zxy() * other.group8().yzx())
                    + (wedge_g8.yzx() * other.group7().zxy())
                    + (other.group5().yzx() * wedge_g9.zxy())
                    - (wedge_g4 * Simd32x3::from(other[e3215]))
                    - (Simd32x3::from(wedge_g10) * other.group3().xyz())
                    - (wedge_g5.yzx() * other.group9().zxy())
                    - (wedge_g7.yzx() * other.group8().zxy())
                    - (wedge_g8.zxy() * other.group7().yzx())
                    - (other.group5().zxy() * wedge_g9.yzx()))
                .with_w((wedge_g4[1] * other[e4315]) + (wedge_g4[2] * other[e4125]) + (wedge_g3[3] * other[e1234])),
            // e5
            (wedge_g0_y * other[e5]) + (wedge_g9[0] * other[e15]) + (wedge_g9[1] * other[e25]) + (wedge_g9[2] * other[e35]) + (wedge_g9[3] * other[e45])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435])
                - (other[e235] * wedge_g6[0])
                - (other[e315] * wedge_g6[1])
                - (other[e125] * wedge_g6[2])
                - (wedge_g3[0] * other[e4235])
                - (wedge_g3[1] * other[e4315])
                - (wedge_g3[2] * other[e4125])
                - (wedge_g3[3] * other[e3215]),
            // e15, e25, e35, e45
            (wedge_g3 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group3())
                + ((Simd32x3::from(wedge_g9[3]) * other.group6().xyz())
                    + (Simd32x3::from(other[e3215]) * wedge_g6.xyz())
                    + (wedge_g8.yzx() * other.group9().zxy())
                    + (other.group8().yzx() * wedge_g9.zxy())
                    - (wedge_g8.zxy() * other.group9().yzx())
                    - (other.group8().zxy() * wedge_g9.yzx()))
                .with_w(0.0),
            // e41, e42, e43
            (wedge_g4 * Simd32x3::from(other[e12345]))
                + (Simd32x3::from(wedge_g0_y) * other.group4())
                + (Simd32x3::from(wedge_g10) * other.group6().xyz())
                + (Simd32x3::from(other[e1234]) * wedge_g6.xyz())
                + (wedge_g7.zxy() * other.group9().yzx())
                + (other.group7().zxy() * wedge_g9.yzx())
                - (wedge_g7.yzx() * other.group9().zxy())
                - (other.group7().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (wedge_g5 * Simd32x3::from(other[e12345]))
                + (wedge_g7 * Simd32x3::from(other[e3215]))
                + (wedge_g8 * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(wedge_g0_y) * other.group5())
                + (Simd32x3::from(wedge_g10) * other.group8())
                + (Simd32x3::from(wedge_g9[3]) * other.group7())
                - (Simd32x3::from(wedge_g6[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            (wedge_g6 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group6())
                + (wedge_g9.yzxw() * other.group9().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group9().yz()).with_zw(wedge_g9[1] * other[e4235] * -1.0, wedge_g10 * other[e3215] * -1.0),
            // e423, e431, e412
            (wedge_g7 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group7()) + (Simd32x3::from(other[e1234]) * wedge_g9.xyz())
                - (Simd32x3::from(wedge_g10) * other.group9().xyz()),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (wedge_g9 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group9()),
            // e1234
            (wedge_g0_y * other[e1234]) + (wedge_g10 * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd2        2        4        0      N/A
    //    simd3        5        8        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd       11       19        0      N/A
    //  no simd       29       42        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (self.group0().zxy() * other.group0().yzx()) - (self.group0().yzx() * other.group0().zxy());
        let wedge_g2 = Simd32x4::from([0.0, 0.0, (self[e15] * other[e4315]) - (self[e25] * other[e4235]), 0.0])
            + ((Simd32x3::from(other[e3215]) * self.group1().xyz()) + ((self.group2().yz() * other.group0().zx()) - (self.group2().zx() * other.group0().yz())).with_z(0.0))
                .with_w(0.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            (wedge_g0.zxy() * other.group0().yzx()) - (wedge_g0.yzx() * other.group0().zxy()),
            // e23, e31, e12, e45
            (wedge_g0 * Simd32x4::from(other[e3215]).xyz()).with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, (wedge_g2[0] * other[e4315]) - (wedge_g2[1] * other[e4235]), 0.0])
                + ((Simd32x3::from(other[e3215] * other[e3215]) * self.group0())
                    + (Simd32x3::from(self[e45] * other[e3215]) * other.group0().xyz())
                    + ((wedge_g2.yz() * other.group0().zx()) - (wedge_g2.zx() * other.group0().yz())).with_z(0.0))
                .with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g2[3]) * other.group0(),
        )
    }
}
impl ProjectViaOriginOnto<Sphere> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd3        9       15        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd       13       25        0      N/A
    //  no simd       34       61        0        0
    fn project_via_origin_onto(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (right_dual_g0_xyz.zxy() * self.group0().yzx()) - (right_dual_g0_xyz.yzx() * self.group0().zxy());
        let wedge_g1_xyz = (Simd32x3::from(other[e3215]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group2().xyz()) - (right_dual_g0_xyz * Simd32x3::from(self[e45]));
        let wedge_g2 = (Simd32x4::from(other[e3215]) * self.group1().xyz().with_w(self[e1234]))
            + ((right_dual_g0_xyz.yzx() * self.group2().zxy()) - (right_dual_g0_xyz.zxy() * self.group2().yzx())).with_w(right_dual_g0_xyz[0] * self[e4235]);
        DipoleInversion::from_groups(
            // e41, e42, e43
            (wedge_g1_xyz * Simd32x3::from(other[e1234])) + (wedge_g0.zxy() * other.group0().yzx()) - (wedge_g0.yzx() * other.group0().zxy()),
            // e23, e31, e12, e45
            ((wedge_g0 * Simd32x3::from(other[e3215])) + (Simd32x3::from(other[e1234]) * wedge_g2.xyz())).with_w(0.0),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g2[1] * other[e4125]) - (wedge_g2[2] * other[e4315]),
                (wedge_g2[2] * other[e4235]) - (wedge_g2[0] * other[e4125]),
                (wedge_g2[0] * other[e4315]) - (wedge_g2[1] * other[e4235]),
            ]) + (wedge_g1_xyz * Simd32x3::from(other[e3215])))
            .with_w(wedge_g2[3] * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g2[3]) * other.group0(),
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       19       31        0        0
    //    simd2        0        1        0      N/A
    //    simd3       12       16        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd       33       52        0      N/A
    //  no simd       63       97        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g2 = other.group2().xyz().with_w(other[e4] * -1.0);
        let wedge_g0 = Simd32x3::from(right_dual_g0_w) * self.group0();
        let wedge_g1 = Simd32x4::from(right_dual_g0_w) * self.group1();
        let wedge_g2_xyz = Simd32x3::from(right_dual_g0_w) * self.group2().xyz();
        let wedge_g2_w = (right_dual_g0_w * self[e1234])
            - (right_dual_g0_xyz[0] * self[e23])
            - (right_dual_g0_xyz[1] * self[e31])
            - (right_dual_g0_xyz[2] * self[e12])
            - (right_dual_g1_xyz[0] * self[e41])
            - (right_dual_g1_xyz[1] * self[e42])
            - (right_dual_g1_xyz[2] * self[e43]);
        let wedge_g3 = ((right_dual_g1_xyz * Simd32x3::from(self[e45]))
            + (Simd32x3::from(right_dual_g0_w) * self.group3().xyz())
            + (right_dual_g0_xyz.yzx() * self.group2().zxy())
            + (self.group0().yzx() * right_dual_g2.zxy())
            - (Simd32x3::from(other[e321]) * self.group1().xyz())
            - (right_dual_g0_xyz.zxy() * self.group2().yzx())
            - (self.group0().zxy() * right_dual_g2.yzx()))
        .with_w(right_dual_g0_w * self[e3215]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from(wedge_g2_w) * other.group1().xyz().with_w(other[e5]))
                + (wedge_g3.yzxy() * other.group0().zxy().with_w(other[e2]))
                + ((wedge_g0 * Simd32x3::from(other[e12345])) + -(wedge_g3.zx() * other.group0().yz()).with_z(wedge_g3[1] * other[e423] * -1.0)).with_w(
                    (wedge_g3[0] * other[e1])
                        - (wedge_g0[0] * other[e235])
                        - (wedge_g0[1] * other[e315])
                        - (wedge_g0[2] * other[e125])
                        - (wedge_g2_xyz[0] * other[e423])
                        - (wedge_g2_xyz[1] * other[e431])
                        - (wedge_g2_xyz[2] * other[e412])
                        - (wedge_g1[0] * other[e415])
                        - (wedge_g1[1] * other[e425])
                        - (wedge_g1[2] * other[e435])
                        - (wedge_g1[3] * other[e321]),
                ),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g2_w) * other.group2().xyz()) + (Simd32x3::from(wedge_g3[3]) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g1.xyz())
                - (Simd32x3::from(other[e321]) * wedge_g3.xyz()))
            .with_w(wedge_g1[3] * other[e12345]),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g3[2] * other[e315]) - (wedge_g3[1] * other[e125]),
                (wedge_g3[0] * other[e125]) - (wedge_g3[2] * other[e235]),
                (wedge_g3[1] * other[e235]) - (wedge_g3[0] * other[e315]),
            ]) + (wedge_g2_xyz * Simd32x3::from(other[e12345]))
                + (Simd32x3::from(wedge_g3[3]) * other.group1().xyz()))
            .with_w(wedge_g2_w * other[e12345]),
            // e4235, e4315, e4125, e3215
            wedge_g3 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<VersorOdd> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       17        0        0
    //    simd3       12       19        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd       23       38        0      N/A
    //  no simd       50       82        0        0
    fn project_via_origin_onto(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2_xyz = other.group2().xyz() * Simd32x3::from(-1.0);
        let right_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (right_dual_g3_xyz.zxy() * self.group0().yzx()) - (right_dual_g3_xyz.yzx() * self.group0().zxy());
        let wedge_g1_xyz = (Simd32x3::from(other[e1234]) * self.group2().xyz()) + (Simd32x3::from(other[e3215]) * self.group0()) - (right_dual_g3_xyz * Simd32x3::from(self[e45]));
        let wedge_g2 = (self.group1().xyzx() * Simd32x3::from(other[e3215]).with_w(other[e23]))
            + ((right_dual_g3_xyz.yzx() * self.group2().zxy()) - (right_dual_g3_xyz.zxy() * self.group2().yzx())).with_w(
                (right_dual_g3_xyz[0] * self[e4235]) + (self[e25] * other[e42]) + (self[e35] * other[e43]) + (self[e1234] * other[e3215])
                    - (right_dual_g2_xyz[0] * self[e41])
                    - (right_dual_g2_xyz[1] * self[e42])
                    - (right_dual_g2_xyz[2] * self[e43])
                    - (self[e45] * other[e45]),
            );
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((wedge_g1_xyz * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g2[3]) * other.group0().xyz()) + (wedge_g0.zxy() * other.group3().yzx())
                - (wedge_g0.yzx() * other.group3().zxy()))
            .with_w(wedge_g2[3] * other[scalar]),
            // e23, e31, e12, e45
            ((wedge_g0 * Simd32x3::from(other[e3215])) + (Simd32x3::from(wedge_g2[3]) * other.group1().xyz()) + (Simd32x3::from(other[e1234]) * wedge_g2.xyz()))
                .with_w(wedge_g2[3] * other[e45]),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g2[1] * other[e4125]) - (wedge_g2[2] * other[e4315]),
                (wedge_g2[2] * other[e4235]) - (wedge_g2[0] * other[e4125]),
                (wedge_g2[0] * other[e4315]) - (wedge_g2[1] * other[e4235]),
            ]) + (wedge_g1_xyz * Simd32x3::from(other[e3215]))
                + (Simd32x3::from(wedge_g2[3]) * other.group2().xyz()))
            .with_w(wedge_g2[3] * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g2[3]) * other.group3(),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for DualNum {
    type Output = ProjectViaOriginOntoInfixPartial<DualNum>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiCircleRotor> for DualNum {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd2        1        2        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd        7       11        0      N/A
    //  no simd       13       21        0        0
    fn project_via_origin_onto(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[e5]) * other.group0().with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, (wedge_g0[1] * other[e23]) - (wedge_g0[0] * other[e31]), 0.0])
                + ((Simd32x3::from(wedge_g0[3]) * other.group0()) + ((wedge_g0.zx() * other.group1().yz()) - (wedge_g0.yz() * other.group1().zx())).with_z(0.0)).with_w(0.0),
            // e5
            (wedge_g0[0] * other[e15]) + (wedge_g0[1] * other[e25]) + (wedge_g0[2] * other[e35]) + (wedge_g0[3] * other[e45]),
        )
    }
}
impl ProjectViaOriginOnto<AntiDipoleInversion> for DualNum {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        3        7        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        6       16        0      N/A
    //  no simd       12       42        0        0
    fn project_via_origin_onto(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[e5]) * other.group0().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let wedge_g1 = Simd32x4::from([1.0, 1.0, self[e5], 0.0]) * (other.group1().xyz() * Simd32x2::from(self[e5]).with_z(1.0)).with_w(0.0);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(wedge_g0[3]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(wedge_g0[3]) * other.group1(),
            // e235, e315, e125, e4
            (Simd32x4::from(wedge_g0[3]).xyz() * other.group2().xyz())
                .with_w((wedge_g0[3] * other[e4]) - (other[e423] * wedge_g0[0]) - (other[e431] * wedge_g0[1]) - (other[e412] * wedge_g0[2])),
            // e1, e2, e3, e5
            ((Simd32x3::from(wedge_g0[3]) * other.group3().xyz()) + (Simd32x3::from(other[e321]) * wedge_g0.xyz()) + (other.group0().zxy() * wedge_g1.yzx())
                - (other.group0().yzx() * wedge_g1.zxy()))
            .with_w(wedge_g0[3] * other[e5]),
        )
    }
}
impl ProjectViaOriginOnto<AntiScalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        4        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(other[e12345] * other[e12345] * -1.0) * self.group0())
    }
}
impl ProjectViaOriginOnto<Circle> for DualNum {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        7       11        0      N/A
    //  no simd       11       21        0        0
    fn project_via_origin_onto(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[e5]) * other.group0();
        let wedge_g1 = Simd32x3::from(self[e5]) * other.group1().xyz();
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            ((wedge_g0 * Simd32x3::from(other[e321])) + (wedge_g1.yzx() * other.group0().zxy()) - (wedge_g1.zxy() * other.group0().yzx())).with_w(0.0),
            // e5
            -(wedge_g0[0] * other[e235])
                - (wedge_g0[1] * other[e315])
                - (wedge_g0[2] * other[e125])
                - (wedge_g1[0] * other[e415])
                - (wedge_g1[1] * other[e425])
                - (wedge_g1[2] * other[e435]),
        )
    }
}
impl ProjectViaOriginOnto<CircleRotor> for DualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        4        8        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        4       14        0      N/A
    //  no simd       12       36        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2_w = other[e12345] * -1.0;
        let wedge_g0_xyz = Simd32x3::from(self[e5]) * other.group0();
        let wedge_g0_w = right_dual_g2_w * self[e12345];
        let wedge_g1 = Simd32x4::from(self[e5]) * other.group1().xyz().with_w(right_dual_g2_w);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(wedge_g0_w) * other.group0().with_w(other[e12345]),
            // e415, e425, e435, e321
            ((wedge_g0_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_w) * other.group1().xyz())).with_w(wedge_g0_w * other[e321]),
            // e235, e315, e125, e5
            ((Simd32x3::from(wedge_g0_w) * other.group2().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g1.xyz())).with_w(wedge_g1[3] * other[e12345]),
            // e1, e2, e3, e4
            ((wedge_g0_xyz * Simd32x3::from(other[e321])) + (other.group0().zxy() * wedge_g1.yzx()) - (other.group0().yzx() * wedge_g1.zxy())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<Dipole> for DualNum {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd2        1        2        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd        7       11        0      N/A
    //  no simd       13       21        0        0
    fn project_via_origin_onto(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[e5]) * other.group0().with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, (wedge_g0[1] * other[e23]) - (wedge_g0[0] * other[e31]), 0.0])
                + ((Simd32x3::from(wedge_g0[3]) * other.group0()) + ((wedge_g0.zx() * other.group1().yz()) - (wedge_g0.yz() * other.group1().zx())).with_z(0.0)).with_w(0.0),
            // e5
            (other[e15] * wedge_g0[0]) + (other[e25] * wedge_g0[1]) + (other[e35] * wedge_g0[2]) + (wedge_g0[3] * other[e45]),
        )
    }
}
impl ProjectViaOriginOnto<DipoleInversion> for DualNum {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       12        0        0
    //    simd2        1        3        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        4        8        0      N/A
    // Totals...
    // yes simd       11       25        0      N/A
    //  no simd       24       56        0        0
    fn project_via_origin_onto(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[e5]) * other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let wedge_g1 = Simd32x4::from(self[e5]) * other.group0().with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e1234]) * wedge_g1.xyz(),
            // e415, e425, e435, e321
            ((wedge_g1.yz() * other.group3().zx()) - (wedge_g1.zx() * other.group3().yz()))
                .with_zw((wedge_g1[0] * other[e4315]) - (wedge_g1[1] * other[e4235]), wedge_g1[3] * other[e1234]),
            // e235, e315, e125, e4
            (Simd32x3::from(wedge_g1[3]) * other.group3().xyz()).with_w((wedge_g0[3] * other[e1234]) - (other[e42] * wedge_g1[1]) - (other[e43] * wedge_g1[2]))
                - (wedge_g1.xyzx() * Simd32x3::from(other[e3215]).with_w(other[e41])),
            // e1, e2, e3, e5
            (wedge_g1.zxyw() * other.group1().yzxw())
                + (wedge_g1.wwwx() * other.group0().with_w(other[e15]))
                + (other.group2().wwwy() * wedge_g0.xyz().with_w(wedge_g1[1]))
                + -(wedge_g1.yz() * other.group1().zx()).with_zw(
                    wedge_g1[0] * other[e31] * -1.0,
                    -(wedge_g0[0] * other[e4235]) - (wedge_g0[1] * other[e4315]) - (wedge_g0[2] * other[e4125]) - (wedge_g0[3] * other[e3215]),
                ),
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        1        5        0      N/A
    //  no simd        1        6        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x2::from(other[e12345] * -1.0) * self.group0();
        DualNum::from_groups(
            // e5, e12345
            Simd32x2::from([(wedge_g0[0] * other[e12345]) + (wedge_g0[1] * other[e5]), wedge_g0[1] * other[e12345]]),
        )
    }
}
impl ProjectViaOriginOnto<FlatPoint> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn project_via_origin_onto(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([other[e45] * other[e45] * self[e5] * -1.0, 0.0]))
    }
}
impl ProjectViaOriginOnto<Flector> for DualNum {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        7        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd        4       17        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([1.0, 1.0, self[e5], 0.0]) * (other.group1().xyz() * Simd32x2::from(self[e5]).with_z(1.0)).with_w(0.0);
        let wedge_g1_w = self[e5] * other[e45] * -1.0;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (Simd32x3::from(wedge_g1_w) * other.group1().xyz()).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0)
                .with_w((wedge_g1_w * other[e45]) - (wedge_g0[0] * other[e4235]) - (wedge_g0[1] * other[e4315]) - (wedge_g0[2] * other[e4125]) - (wedge_g0[3] * other[e3215])),
        )
    }
}
impl ProjectViaOriginOnto<Line> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2       10        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([1.0, 1.0, self[e5], 0.0]) * (other.group0() * Simd32x2::from(self[e5]).with_z(1.0)).with_w(0.0);
        DualNum::from_groups(
            // e5, e12345
            Simd32x2::from([-(other[e415] * wedge_g0[0]) - (other[e425] * wedge_g0[1]) - (other[e435] * wedge_g0[2]), 0.0]),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd        4        9        0      N/A
    //  no simd       10       21        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e12345] * -1.0);
        let wedge_g0_w = self[e12345] * right_dual_g0[3];
        let wedge_g1 = right_dual_g0 * Simd32x4::from(self[e5]);
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(wedge_g0_w) * other.group0(),
            // e235, e315, e125, e5
            (wedge_g1 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_w) * other.group1())
                + Simd32x3::from(0.0).with_w(-(wedge_g1[0] * other[e415]) - (wedge_g1[1] * other[e425]) - (wedge_g1[2] * other[e435])),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       32       41        0        0
    //    simd2        0        2        0      N/A
    //    simd3       23       31        0      N/A
    //    simd4        8       13        0      N/A
    // Totals...
    // yes simd       63       87        0      N/A
    //  no simd      133      190        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let wedge_g0_y = (right_dual_g0[0] * self[e12345]) - (self[e5] * other[e4]);
        let wedge_g2 = right_dual_g0[0] * self[e5];
        let wedge_g3 = Simd32x4::from(self[e5]) * other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let wedge_g6 = Simd32x4::from([1.0, 1.0, self[e5], 0.0]) * (other.group7() * Simd32x2::from(self[e5]).with_z(1.0)).with_w(0.0);
        let wedge_g8 = Simd32x3::from(self[e5]) * other.group6().xyz();
        let wedge_g9 = Simd32x4::from(self[e5]) * other.group4().with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar])
                    + (wedge_g2 * other[e1234])
                    + (wedge_g9[0] * other[e1])
                    + (wedge_g9[1] * other[e2])
                    + (wedge_g9[2] * other[e3])
                    + (wedge_g9[3] * other[e4])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43])
                    - (other[e23] * wedge_g6[0])
                    - (other[e31] * wedge_g6[1])
                    - (other[e12] * wedge_g6[2])
                    - (other[e423] * wedge_g3[0])
                    - (other[e431] * wedge_g3[1])
                    - (other[e412] * wedge_g3[2])
                    - (wedge_g3[3] * other[e321])
                    - (wedge_g6[3] * other[e45]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(wedge_g0_y) * other.group1())
                + ((Simd32x3::from(wedge_g6[3]) * other.group6().xyz())
                    + (Simd32x3::from(wedge_g9[3]) * other.group4())
                    + (Simd32x3::from(other[e321]) * wedge_g6.xyz())
                    + (Simd32x3::from(other[e1234]) * wedge_g3.xyz())
                    + (wedge_g8.yzx() * other.group7().zxy())
                    + (other.group5().yzx() * wedge_g9.zxy())
                    - (wedge_g8.zxy() * other.group7().yzx())
                    - (other.group5().zxy() * wedge_g9.yzx()))
                .with_w(wedge_g3[3] * other[e1234]),
            // e5
            (wedge_g0_y * other[e5])
                + (wedge_g2 * other[e12345])
                + (wedge_g9[0] * other[e15])
                + (wedge_g9[1] * other[e25])
                + (wedge_g9[2] * other[e35])
                + (wedge_g9[3] * other[e45])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435])
                - (other[e235] * wedge_g6[0])
                - (other[e315] * wedge_g6[1])
                - (other[e125] * wedge_g6[2])
                - (wedge_g3[0] * other[e4235])
                - (wedge_g3[1] * other[e4315])
                - (wedge_g3[2] * other[e4125])
                - (wedge_g3[3] * other[e3215]),
            // e15, e25, e35, e45
            (wedge_g3 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group3())
                + ((Simd32x3::from(wedge_g9[3]) * other.group6().xyz())
                    + (Simd32x3::from(other[e3215]) * wedge_g6.xyz())
                    + (wedge_g8.yzx() * other.group9().zxy())
                    + (other.group8().yzx() * wedge_g9.zxy())
                    - (wedge_g8.zxy() * other.group9().yzx())
                    - (other.group8().zxy() * wedge_g9.yzx()))
                .with_w(0.0),
            // e41, e42, e43
            (Simd32x3::from(wedge_g0_y) * other.group4()) + (Simd32x3::from(other[e1234]) * wedge_g6.xyz()) + (other.group7().zxy() * wedge_g9.yzx())
                - (other.group7().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (wedge_g8 * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0_y) * other.group5()) + (Simd32x3::from(wedge_g9[3]) * other.group7())
                - (Simd32x3::from(wedge_g6[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, wedge_g9[1] * other[e4235] * -1.0, 0.0])
                + (wedge_g6 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group6())
                + (wedge_g9.yzxw() * other.group9().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group9().yz()).with_zw(0.0, 0.0),
            // e423, e431, e412
            (Simd32x3::from(wedge_g0_y) * other.group7()) + (Simd32x3::from(other[e1234]) * wedge_g9.xyz()),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (wedge_g9 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group9()),
            // e1234
            wedge_g0_y * other[e1234],
        )
    }
}
impl ProjectViaOriginOnto<Plane> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3       11        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([1.0, 1.0, self[e5], 0.0]) * (other.group0().xyz() * Simd32x2::from(self[e5]).with_z(1.0)).with_w(0.0);
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            -(wedge_g0[0] * other[e4235]) - (wedge_g0[1] * other[e4315]) - (wedge_g0[2] * other[e4125]) - (wedge_g0[3] * other[e3215]),
            0.0,
        ]))
    }
}
impl ProjectViaOriginOnto<RoundPoint> for DualNum {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        7        0        0
    fn project_via_origin_onto(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = self[e5] * other[e4] * -1.0;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(wedge_g0) * other.group0(), /* e5 */ wedge_g0 * other[e5])
    }
}
impl ProjectViaOriginOnto<Sphere> for DualNum {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       16        0        0
    fn project_via_origin_onto(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[e5]) * other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            wedge_g0 * Simd32x4::from(other[e1234]),
            // e5
            -(wedge_g0[0] * other[e4235]) - (wedge_g0[1] * other[e4315]) - (wedge_g0[2] * other[e4125]) - (wedge_g0[3] * other[e3215]),
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for DualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       17        0        0
    //    simd3        3        5        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd       14       26        0      N/A
    //  no simd       26       48        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_w = other[e12345] * -1.0;
        let wedge_g0_xyz = Simd32x3::from(self[e5]) * other.group0().xyz();
        let wedge_g0_w = (right_dual_g0_w * self[e12345]) - (self[e5] * other[e4]);
        let wedge_g1 = Simd32x4::from(self[e5]) * other.group1().xyz().with_w(right_dual_g0_w);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(wedge_g0_w) * other.group0(),
            // e415, e425, e435, e321
            ((wedge_g0_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_w) * other.group1().xyz())).with_w(wedge_g0_w * other[e321]),
            // e235, e315, e125, e5
            (wedge_g1 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_w) * other.group2())
                + Simd32x3::from(0.0).with_w(
                    -(wedge_g0_xyz[0] * other[e235])
                        - (wedge_g0_xyz[1] * other[e315])
                        - (wedge_g0_xyz[2] * other[e125])
                        - (wedge_g1[0] * other[e415])
                        - (wedge_g1[1] * other[e425])
                        - (wedge_g1[2] * other[e435]),
                ),
            // e1, e2, e3, e4
            (Simd32x3::from([
                (wedge_g1[1] * other[e412]) - (wedge_g1[2] * other[e431]),
                (wedge_g1[2] * other[e423]) - (wedge_g1[0] * other[e412]),
                (wedge_g1[0] * other[e431]) - (wedge_g1[1] * other[e423]),
            ]) + (wedge_g0_xyz * Simd32x3::from(other[e321]))
                + (Simd32x3::from(wedge_g0_w) * other.group3().xyz()))
            .with_w(wedge_g0_w * other[e4]),
        )
    }
}
impl ProjectViaOriginOnto<VersorOdd> for DualNum {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       12        0        0
    //    simd2        1        3        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        4        8        0      N/A
    // Totals...
    // yes simd       11       25        0      N/A
    //  no simd       24       56        0        0
    fn project_via_origin_onto(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[e5]) * other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let wedge_g1 = Simd32x4::from(self[e5]) * other.group0().xyz().with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e1234]) * wedge_g1.xyz(),
            // e415, e425, e435, e321
            ((wedge_g1.yz() * other.group3().zx()) - (wedge_g1.zx() * other.group3().yz()))
                .with_zw((wedge_g1[0] * other[e4315]) - (wedge_g1[1] * other[e4235]), wedge_g1[3] * other[e1234]),
            // e235, e315, e125, e4
            (Simd32x3::from(wedge_g1[3]) * other.group3().xyz()).with_w((wedge_g0[3] * other[e1234]) - (wedge_g1[1] * other[e42]) - (wedge_g1[2] * other[e43]))
                - (wedge_g1.xyzx() * Simd32x3::from(other[e3215]).with_w(other[e41])),
            // e1, e2, e3, e5
            (wedge_g1.zxyy() * other.group1().yzx().with_w(other[e25]))
                + (wedge_g1.wwwz() * other.group0().xyz().with_w(other[e35]))
                + (other.group2().wwwx() * wedge_g0.xyz().with_w(wedge_g1[0]))
                + -(wedge_g1.yz() * other.group1().zx()).with_zw(
                    wedge_g1[0] * other[e31] * -1.0,
                    -(wedge_g0[0] * other[e4235]) - (wedge_g0[1] * other[e4315]) - (wedge_g0[2] * other[e4125]) - (wedge_g0[3] * other[e3215]),
                ),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for FlatPoint {
    type Output = ProjectViaOriginOntoInfixPartial<FlatPoint>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiCircleRotor> for FlatPoint {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       15        0        0
    fn project_via_origin_onto(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) - (other[e45] * self[e45]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g0) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(wedge_g0) * other.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(wedge_g0) * other.group2(),
        )
    }
}
impl ProjectViaOriginOnto<AntiDipoleInversion> for FlatPoint {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        4        9        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd       12       28        0        0
    fn project_via_origin_onto(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = (Simd32x3::from(self[e45]) * other.group1().xyz()) + (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx());
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (wedge_g0_xyz.yzx() * other.group0().zxy()) - (wedge_g0_xyz.zxy() * other.group0().yzx()),
            // e23, e31, e12, e45
            (wedge_g0_xyz * Simd32x4::from(other[e321]).xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35, scalar
            ((wedge_g0_xyz.zxy() * other.group2().yzx()) - (wedge_g0_xyz.yzx() * other.group2().zxy())).with_w(wedge_g0_xyz[0] * other[e1]),
        )
    }
}
impl ProjectViaOriginOnto<AntiScalar> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group0())
    }
}
impl ProjectViaOriginOnto<Circle> for FlatPoint {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        4        9        0      N/A
    // no simd       12       27        0        0
    fn project_via_origin_onto(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = (Simd32x3::from(self[e45]) * other.group1().xyz()) + (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx());
        Dipole::from_groups(
            // e41, e42, e43
            (wedge_g0_xyz.yzx() * other.group0().zxy()) - (wedge_g0_xyz.zxy() * other.group0().yzx()),
            // e23, e31, e12, e45
            (wedge_g0_xyz * Simd32x4::from(other[e321]).xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            (wedge_g0_xyz.zxy() * other.group2().yzx()) - (wedge_g0_xyz.yzx() * other.group2().zxy()),
        )
    }
}
impl ProjectViaOriginOnto<CircleRotor> for FlatPoint {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd2        1        2        0      N/A
    //    simd3        5        8        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd       11       21        0      N/A
    //  no simd       28       48        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(other[e12345] * -1.0) * self.group0();
        let wedge_g1 =
            ((Simd32x3::from(self[e45]) * other.group1().xyz()) + (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx())).with_w(0.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((other.group0().zxy() * wedge_g1.yzx()) - (other.group0().yzx() * wedge_g1.zxy())).with_w(wedge_g0[3] * other[e321] * -1.0),
            // e23, e31, e12, e45
            (Simd32x3::from(wedge_g1[3]) * other.group0()).with_w((wedge_g0[3] * other[e12345]) - (wedge_g1[1] * other[e425]) - (wedge_g1[2] * other[e435]))
                - (wedge_g1.xyzx() * other.group1().wwwx()),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, (wedge_g1[1] * other[e235]) - (wedge_g1[0] * other[e315]), 0.0])
                + ((Simd32x3::from(wedge_g1[3]) * other.group1().xyz())
                    + (Simd32x3::from(other[e12345]) * wedge_g0.xyz())
                    + ((wedge_g1.zx() * other.group2().yz()) - (wedge_g1.yz() * other.group2().zx())).with_z(0.0))
                .with_w(0.0),
            // e4235, e4315, e4125, e3215
            wedge_g1 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Dipole> for FlatPoint {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       14        0        0
    fn project_via_origin_onto(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) - (other[e45] * self[e45]);
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g0) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(wedge_g0) * other.group1(),
            // e15, e25, e35
            Simd32x3::from(wedge_g0) * other.group2(),
        )
    }
}
impl ProjectViaOriginOnto<DipoleInversion> for FlatPoint {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       13        0        0
    //    simd2        1        2        0      N/A
    //    simd3        5        8        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       11       24        0      N/A
    //  no simd       25       45        0        0
    fn project_via_origin_onto(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = (Simd32x3::from(other[e1234]) * self.group0().xyz()) + (Simd32x3::from(self[e45]) * other.group3().xyz());
        let wedge_g0_w = other[e45] * self[e45] * -1.0;
        let wedge_g1 = Simd32x4::from([0.0, 0.0, (other[e4315] * self[e15]) - (other[e4235] * self[e25]), 0.0])
            + ((other.group3().zx() * self.group0().yz()) - (other.group3().yz() * self.group0().zx())).with_zw(0.0, 0.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((wedge_g0_xyz * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0_w) * other.group0())).with_w(wedge_g1[3] * other[e1234]),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g0_w) * other.group1().xyz()) + (Simd32x3::from(other[e1234]) * wedge_g1.xyz())).with_w(wedge_g0_w * other[e45]),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g1[1] * other[e4125]) - (wedge_g1[2] * other[e4315]),
                (wedge_g1[2] * other[e4235]) - (wedge_g1[0] * other[e4125]),
                (wedge_g1[0] * other[e4315]) - (wedge_g1[1] * other[e4235]),
            ]) + (wedge_g0_xyz * Simd32x3::from(other[e3215]))
                + (Simd32x3::from(wedge_g0_w) * other.group2().xyz()))
            .with_w(wedge_g0_w * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0_w) * other.group3(),
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group0())
    }
}
impl ProjectViaOriginOnto<FlatPoint> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
    fn project_via_origin_onto(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other[e45] * self[e45] * -1.0) * other.group0())
    }
}
impl ProjectViaOriginOnto<Flector> for FlatPoint {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        9        0        0
    //    simd2        1        2        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd        8       16        0      N/A
    //  no simd       16       31        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[e45]) * other.group1().xyz().with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let wedge_g1 = Simd32x4::from([0.0, 0.0, (self[e15] * other[e4315]) - (self[e25] * other[e4235]), 0.0])
            + ((self.group0().yz() * other.group1().zx()) - (self.group0().zx() * other.group1().yz())).with_zw(0.0, 0.0);
        Flector::from_groups(
            // e15, e25, e35, e45
            (Simd32x3::from([
                (wedge_g1[1] * other[e4125]) - (wedge_g1[2] * other[e4315]),
                (wedge_g1[2] * other[e4235]) - (wedge_g1[0] * other[e4125]),
                (wedge_g1[0] * other[e4315]) - (wedge_g1[1] * other[e4235]),
            ]) + (Simd32x3::from(wedge_g0[3]) * other.group0().xyz())
                + (Simd32x3::from(other[e3215]) * wedge_g0.xyz()))
            .with_w(wedge_g0[3] * other[e45]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0[3]) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Line> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        4        9        0      N/A
    // no simd       12       27        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(self[e45]) * other.group0();
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            ((wedge_g0_xyz.zxy() * other.group1().yzx())
                - (wedge_g0_xyz.yzx() * other.group1().zxy())
                - (other.group0() * other.group0() * self.group0().xyz())
                - (Simd32x3::from(other[e415]) * other.group0().yyz() * self.group0().yxx())
                - (Simd32x3::from(other[e435]) * other.group0().xyy() * self.group0().zzy()))
            .with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for FlatPoint {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       11        0        0
    //    simd3        2        3        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        7       16        0      N/A
    //  no simd       11       28        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let wedge_g0 = Simd32x4::from(other[e12345] * -1.0) * self.group0();
        let wedge_g1 = (right_dual_g0_xyz * Simd32x4::from(self[e45]).xyz())
            .with_w(-(right_dual_g0_xyz[0] * self[e15]) - (right_dual_g0_xyz[1] * self[e25]) - (right_dual_g0_xyz[2] * self[e35]));
        Flector::from_groups(
            // e15, e25, e35, e45
            (Simd32x3::from([
                (wedge_g1[2] * other[e315]) - (wedge_g1[1] * other[e125]),
                (wedge_g1[0] * other[e125]) - (wedge_g1[2] * other[e235]),
                (wedge_g1[1] * other[e235]) - (wedge_g1[0] * other[e315]),
            ]) + (Simd32x3::from(wedge_g1[3]) * other.group0().xyz())
                + (Simd32x3::from(other[e12345]) * wedge_g0.xyz()))
            .with_w(wedge_g0[3] * other[e12345]),
            // e4235, e4315, e4125, e3215
            wedge_g1 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for FlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       32       41        0        0
    //    simd2        0        1        0      N/A
    //    simd3       27       37        0      N/A
    //    simd4        8        9        0      N/A
    // Totals...
    // yes simd       67       88        0      N/A
    //  no simd      145      190        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let wedge_g0_y = (other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) - (self[e45] * other[e45]);
        let wedge_g3 = Simd32x4::from(other[e12345] * -1.0) * self.group0();
        let wedge_g6 = ((Simd32x3::from(other[e1234]) * self.group0().xyz()) - (right_dual_g1_xyz * Simd32x3::from(self[e45]))).with_w(0.0);
        let wedge_g8 = (right_dual_g1_xyz.yzx() * self.group0().zxy()) - (right_dual_g1_xyz.zxy() * self.group0().yzx());
        let wedge_g9 =
            ((Simd32x3::from(self[e45]) * other.group6().xyz()) + (other.group7().yzx() * self.group0().zxy()) - (other.group7().zxy() * self.group0().yzx())).with_w(0.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar]) + (wedge_g9[0] * other[e1]) + (wedge_g9[1] * other[e2]) + (wedge_g9[2] * other[e3]) + (wedge_g9[3] * other[e4])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43])
                    - (other[e23] * wedge_g6[0])
                    - (other[e31] * wedge_g6[1])
                    - (other[e12] * wedge_g6[2])
                    - (other[e423] * wedge_g3[0])
                    - (other[e431] * wedge_g3[1])
                    - (other[e412] * wedge_g3[2])
                    - (wedge_g3[3] * other[e321])
                    - (wedge_g6[3] * other[e45]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(wedge_g0_y) * other.group1())
                + ((Simd32x3::from(wedge_g6[3]) * other.group6().xyz())
                    + (Simd32x3::from(wedge_g9[3]) * other.group4())
                    + (Simd32x3::from(other[e321]) * wedge_g6.xyz())
                    + (Simd32x3::from(other[e1234]) * wedge_g3.xyz())
                    + (wedge_g8.yzx() * other.group7().zxy())
                    + (other.group5().yzx() * wedge_g9.zxy())
                    - (wedge_g8.zxy() * other.group7().yzx())
                    - (other.group5().zxy() * wedge_g9.yzx()))
                .with_w(wedge_g3[3] * other[e1234]),
            // e5
            (wedge_g0_y * other[e5]) + (wedge_g9[0] * other[e15]) + (wedge_g9[1] * other[e25]) + (wedge_g9[2] * other[e35]) + (wedge_g9[3] * other[e45])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435])
                - (other[e235] * wedge_g6[0])
                - (other[e315] * wedge_g6[1])
                - (other[e125] * wedge_g6[2])
                - (wedge_g3[0] * other[e4235])
                - (wedge_g3[1] * other[e4315])
                - (wedge_g3[2] * other[e4125])
                - (wedge_g3[3] * other[e3215]),
            // e15, e25, e35, e45
            (wedge_g3 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group3())
                + ((Simd32x3::from(wedge_g9[3]) * other.group6().xyz())
                    + (Simd32x3::from(other[e3215]) * wedge_g6.xyz())
                    + (wedge_g8.yzx() * other.group9().zxy())
                    + (other.group8().yzx() * wedge_g9.zxy())
                    - (wedge_g8.zxy() * other.group9().yzx())
                    - (other.group8().zxy() * wedge_g9.yzx()))
                .with_w(0.0),
            // e41, e42, e43
            (Simd32x3::from(wedge_g0_y) * other.group4()) + (Simd32x3::from(other[e1234]) * wedge_g6.xyz()) + (other.group7().zxy() * wedge_g9.yzx())
                - (other.group7().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (wedge_g8 * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0_y) * other.group5()) + (Simd32x3::from(wedge_g9[3]) * other.group7())
                - (Simd32x3::from(wedge_g6[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, wedge_g9[1] * other[e4235] * -1.0, 0.0])
                + (wedge_g6 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group6())
                + (wedge_g9.yzxw() * other.group9().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group9().yz()).with_zw(0.0, 0.0),
            // e423, e431, e412
            (Simd32x3::from(wedge_g0_y) * other.group7()) + (Simd32x3::from(other[e1234]) * wedge_g9.xyz()),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (wedge_g9 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group9()),
            // e1234
            wedge_g0_y * other[e1234],
        )
    }
}
impl ProjectViaOriginOnto<Plane> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd3        2        3        0      N/A
    // Totals...
    // yes simd        5       10        0      N/A
    //  no simd        9       16        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g1 = Simd32x3::from([
            (self[e25] * other[e4125]) - (self[e35] * other[e4315]),
            (self[e35] * other[e4235]) - (self[e15] * other[e4125]),
            (self[e15] * other[e4315]) - (self[e25] * other[e4235]),
        ]);
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            ((Simd32x3::from(self[e45] * other[e3215]) * other.group0().xyz()) + (wedge_g1.yzx() * other.group0().zxy()) - (wedge_g1.zxy() * other.group0().yzx())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<Sphere> for FlatPoint {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        4       10        0      N/A
    // Totals...
    // yes simd        6       13        0      N/A
    //  no simd       14       33        0        0
    fn project_via_origin_onto(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = (Simd32x3::from(other[e1234]) * self.group0().xyz()) - (right_dual_g0_xyz * Simd32x3::from(self[e45]));
        let wedge_g1 = (right_dual_g0_xyz.yzx() * self.group0().zxy()) - (right_dual_g0_xyz.zxy() * self.group0().yzx());
        Dipole::from_groups(
            // e41, e42, e43
            wedge_g0 * Simd32x3::from(other[e1234]),
            // e23, e31, e12, e45
            (wedge_g1 * Simd32x3::from(other[e1234])).with_w(-(wedge_g0[0] * other[e4235]) - (wedge_g0[1] * other[e4315]) - (wedge_g0[2] * other[e4125])),
            // e15, e25, e35
            (wedge_g0 * Simd32x3::from(other[e3215])) + (wedge_g1.yzx() * other.group0().zxy()) - (wedge_g1.zxy() * other.group0().yzx()),
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for FlatPoint {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd2        1        2        0      N/A
    //    simd3        4        5        0      N/A
    //    simd4        5        6        0      N/A
    // Totals...
    // yes simd       14       21        0      N/A
    //  no simd       38       51        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let wedge_g0 = Simd32x4::from(other[e12345] * -1.0) * self.group0();
        let wedge_g1 =
            ((Simd32x3::from(self[e45]) * other.group1().xyz()) + (right_dual_g0_xyz.yzx() * self.group0().zxy()) - (right_dual_g0_xyz.zxy() * self.group0().yzx())).with_w(0.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (wedge_g1.yzxx() * other.group0().zxy().with_w(other[e1]))
                + Simd32x3::from(0.0).with_w(-(wedge_g0[1] * other[e431]) - (wedge_g0[2] * other[e412]) - (wedge_g0[3] * other[e321]))
                - (other.group0().yzxx() * wedge_g1.zxy().with_w(wedge_g0[0])),
            // e23, e31, e12, e45
            (other.group0() * Simd32x3::from(wedge_g1[3]).with_w(wedge_g0[3])) + Simd32x3::from(0.0).with_w(-(wedge_g1[1] * other[e425]) - (wedge_g1[2] * other[e435]))
                - (wedge_g1.xyzx() * other.group1().wwwx()),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, (wedge_g1[1] * other[e235]) - (wedge_g1[0] * other[e315]), 0.0])
                + ((Simd32x3::from(wedge_g1[3]) * other.group1().xyz())
                    + (Simd32x3::from(other[e12345]) * wedge_g0.xyz())
                    + ((wedge_g1.zx() * other.group2().yz()) - (wedge_g1.yz() * other.group2().zx())).with_z(0.0))
                .with_w(0.0),
            // e4235, e4315, e4125, e3215
            wedge_g1 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<VersorOdd> for FlatPoint {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        7        0        0
    //    simd3        6       12        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd        9       21        0      N/A
    //  no simd       24       51        0        0
    fn project_via_origin_onto(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = ((Simd32x3::from(other[e1234]) * self.group0().xyz()) - (right_dual_g3_xyz * Simd32x3::from(self[e45]))).with_w(self[e45] * other[e45] * -1.0);
        let wedge_g1_xyz = (right_dual_g3_xyz.yzx() * self.group0().zxy()) - (right_dual_g3_xyz.zxy() * self.group0().yzx());
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (wedge_g0 * Simd32x3::from(other[e1234]).with_w(other[scalar]))
                + (Simd32x3::from(wedge_g0[3]) * other.group0().xyz()).with_w(-(wedge_g0[0] * other[e23]) - (wedge_g0[1] * other[e31]) - (wedge_g0[2] * other[e12])),
            // e23, e31, e12, e45
            ((wedge_g1_xyz * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0[3]) * other.group1().xyz())).with_w(wedge_g0[3] * other[e45]),
            // e15, e25, e35, e1234
            ((Simd32x3::from(wedge_g0[3]) * other.group2().xyz()) + (Simd32x3::from(other[e3215]) * wedge_g0.xyz()) + (wedge_g1_xyz.yzx() * other.group3().zxy())
                - (wedge_g1_xyz.zxy() * other.group3().yzx()))
            .with_w(wedge_g0[3] * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0[3]) * other.group3(),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for Flector {
    type Output = ProjectViaOriginOntoInfixPartial<Flector>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiCircleRotor> for Flector {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       15        0        0
    fn project_via_origin_onto(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) - (other[e45] * self[e45]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g0) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(wedge_g0) * other.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(wedge_g0) * other.group2(),
        )
    }
}
impl ProjectViaOriginOnto<AntiDipoleInversion> for Flector {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        4        9        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd       12       28        0        0
    fn project_via_origin_onto(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = (Simd32x3::from(self[e45]) * other.group1().xyz()) + (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx());
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (wedge_g0_xyz.yzx() * other.group0().zxy()) - (wedge_g0_xyz.zxy() * other.group0().yzx()),
            // e23, e31, e12, e45
            (wedge_g0_xyz * Simd32x4::from(other[e321]).xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35, scalar
            ((wedge_g0_xyz.zxy() * other.group2().yzx()) - (wedge_g0_xyz.yzx() * other.group2().zxy())).with_w(wedge_g0_xyz[0] * other[e1]),
        )
    }
}
impl ProjectViaOriginOnto<AntiScalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       11        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e12345] * -1.0;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Circle> for Flector {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        4        9        0      N/A
    // no simd       12       27        0        0
    fn project_via_origin_onto(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = (Simd32x3::from(self[e45]) * other.group1().xyz()) + (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx());
        Dipole::from_groups(
            // e41, e42, e43
            (wedge_g0_xyz.yzx() * other.group0().zxy()) - (wedge_g0_xyz.zxy() * other.group0().yzx()),
            // e23, e31, e12, e45
            (wedge_g0_xyz * Simd32x4::from(other[e321]).xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e15, e25, e35
            (wedge_g0_xyz.zxy() * other.group2().yzx()) - (wedge_g0_xyz.yzx() * other.group2().zxy()),
        )
    }
}
impl ProjectViaOriginOnto<CircleRotor> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        9        0        0
    //    simd2        1        2        0      N/A
    //    simd3        6        9        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd       12       23        0      N/A
    //  no simd       31       52        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2_w = other[e12345] * -1.0;
        let wedge_g0 = Simd32x4::from(right_dual_g2_w) * self.group0();
        let wedge_g1 =
            ((Simd32x3::from(right_dual_g2_w) * self.group1().xyz()) + (Simd32x3::from(self[e45]) * other.group1().xyz()) + (other.group0().yzx() * self.group0().zxy())
                - (other.group0().zxy() * self.group0().yzx()))
            .with_w(right_dual_g2_w * self[e3215]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((other.group0().zxy() * wedge_g1.yzx()) - (other.group0().yzx() * wedge_g1.zxy())).with_w(wedge_g0[3] * other[e321] * -1.0),
            // e23, e31, e12, e45
            (Simd32x3::from(wedge_g1[3]) * other.group0()).with_w((wedge_g0[3] * other[e12345]) - (wedge_g1[1] * other[e425]) - (wedge_g1[2] * other[e435]))
                - (wedge_g1.xyzx() * other.group1().wwwx()),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, (wedge_g1[1] * other[e235]) - (wedge_g1[0] * other[e315]), 0.0])
                + ((Simd32x3::from(wedge_g1[3]) * other.group1().xyz())
                    + (Simd32x3::from(other[e12345]) * wedge_g0.xyz())
                    + ((wedge_g1.zx() * other.group2().yz()) - (wedge_g1.yz() * other.group2().zx())).with_z(0.0))
                .with_w(0.0),
            // e4235, e4315, e4125, e3215
            wedge_g1 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Dipole> for Flector {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       14        0        0
    fn project_via_origin_onto(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) - (other[e45] * self[e45]);
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g0) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(wedge_g0) * other.group1(),
            // e15, e25, e35
            Simd32x3::from(wedge_g0) * other.group2(),
        )
    }
}
impl ProjectViaOriginOnto<DipoleInversion> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       16        0        0
    //    simd2        1        2        0      N/A
    //    simd3        5        8        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       15       27        0      N/A
    //  no simd       29       48        0        0
    fn project_via_origin_onto(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = (Simd32x3::from(other[e1234]) * self.group0().xyz()) + (Simd32x3::from(self[e45]) * other.group3().xyz());
        let wedge_g0_w = (other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) + (other[e1234] * self[e3215]) - (other[e45] * self[e45]);
        let wedge_g1 = Simd32x4::from([0.0, 0.0, (other[e4315] * self[e15]) - (other[e4235] * self[e25]), 0.0])
            + ((other.group3().zx() * self.group0().yz()) - (other.group3().yz() * self.group0().zx())).with_zw(0.0, 0.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((wedge_g0_xyz * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0_w) * other.group0())).with_w(wedge_g1[3] * other[e1234]),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g0_w) * other.group1().xyz()) + (Simd32x3::from(other[e1234]) * wedge_g1.xyz())).with_w(wedge_g0_w * other[e45]),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g1[1] * other[e4125]) - (wedge_g1[2] * other[e4315]),
                (wedge_g1[2] * other[e4235]) - (wedge_g1[0] * other[e4125]),
                (wedge_g1[0] * other[e4315]) - (wedge_g1[1] * other[e4235]),
            ]) + (wedge_g0_xyz * Simd32x3::from(other[e3215]))
                + (Simd32x3::from(wedge_g0_w) * other.group2().xyz()))
            .with_w(wedge_g0_w * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0_w) * other.group3(),
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       12        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group1(),
        )
    }
}
impl ProjectViaOriginOnto<FlatPoint> for Flector {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
    fn project_via_origin_onto(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other[e45] * self[e45] * -1.0) * other.group0())
    }
}
impl ProjectViaOriginOnto<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       14        0        0
    //    simd2        1        2        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       11       19        0      N/A
    //  no simd       19       28        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_w = -(other[e45] * self[e45]) - (other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]);
        let wedge_g1 = Simd32x4::from([0.0, 0.0, (other[e4315] * self[e15]) - (other[e4235] * self[e25]), 0.0])
            + ((other.group1().zx() * self.group0().yz()) - (other.group1().yz() * self.group0().zx())).with_zw(0.0, 0.0);
        Flector::from_groups(
            // e15, e25, e35, e45
            (Simd32x3::from([
                (wedge_g1[1] * other[e4125]) - (wedge_g1[2] * other[e4315]),
                (wedge_g1[2] * other[e4235]) - (wedge_g1[0] * other[e4125]),
                (wedge_g1[0] * other[e4315]) - (wedge_g1[1] * other[e4235]),
            ]) + (Simd32x3::from(wedge_g0_w) * other.group0().xyz())
                + (Simd32x3::from(other[e3215] * self[e45]) * other.group1().xyz()))
            .with_w(wedge_g0_w * other[e45]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0_w) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Line> for Flector {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        4        9        0      N/A
    // no simd       12       27        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(self[e45]) * other.group0();
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            ((wedge_g0_xyz.zxy() * other.group1().yzx())
                - (wedge_g0_xyz.yzx() * other.group1().zxy())
                - (other.group0() * other.group0() * self.group0().xyz())
                - (Simd32x3::from(other[e415]) * other.group0().yyz() * self.group0().yxx())
                - (Simd32x3::from(other[e435]) * other.group0().xyy() * self.group0().zzy()))
            .with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        9        0        0
    //    simd3        3        4        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        6       15        0      N/A
    //  no simd       12       29        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_w = other[e12345] * -1.0;
        let wedge_g0 = Simd32x4::from(right_dual_g0_w) * self.group0();
        let wedge_g1 = ((Simd32x3::from(right_dual_g0_w) * self.group1().xyz()) + (Simd32x3::from(self[e45]) * other.group0().xyz())).with_w(right_dual_g0_w * self[e3215]);
        Flector::from_groups(
            // e15, e25, e35, e45
            (Simd32x3::from([
                (wedge_g1[2] * other[e315]) - (wedge_g1[1] * other[e125]),
                (wedge_g1[0] * other[e125]) - (wedge_g1[2] * other[e235]),
                (wedge_g1[1] * other[e235]) - (wedge_g1[0] * other[e315]),
            ]) + (Simd32x3::from(wedge_g1[3]) * other.group0().xyz())
                + (Simd32x3::from(other[e12345]) * wedge_g0.xyz()))
            .with_w(wedge_g0[3] * other[e12345]),
            // e4235, e4315, e4125, e3215
            wedge_g1 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       36       45        0        0
    //    simd2        0        2        0      N/A
    //    simd3       28       38        0      N/A
    //    simd4        8        9        0      N/A
    // Totals...
    // yes simd       72       94        0      N/A
    //  no simd      152      199        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let wedge_g0_y = (right_dual_g1_xyz[0] * self[e4235])
            + (right_dual_g1_xyz[1] * self[e4315])
            + (right_dual_g1_xyz[2] * self[e4125])
            + (other[e41] * self[e15])
            + (other[e42] * self[e25])
            + (other[e43] * self[e35])
            + (self[e3215] * other[e1234])
            - (self[e45] * other[e45]);
        let wedge_g3 = Simd32x4::from(right_dual_g0[0]) * self.group0();
        let wedge_g6 = ((Simd32x3::from(other[e1234]) * self.group0().xyz()) - (right_dual_g1_xyz * Simd32x3::from(self[e45]))).with_w(0.0);
        let wedge_g8 = (right_dual_g1_xyz.yzx() * self.group0().zxy()) - (right_dual_g1_xyz.zxy() * self.group0().yzx());
        let wedge_g9 =
            ((Simd32x3::from(right_dual_g0[0]) * self.group1().xyz()) + (Simd32x3::from(self[e45]) * other.group6().xyz()) + (other.group7().yzx() * self.group0().zxy())
                - (other.group7().zxy() * self.group0().yzx()))
            .with_w(right_dual_g0[0] * self[e3215]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar]) + (wedge_g9[0] * other[e1]) + (wedge_g9[1] * other[e2]) + (wedge_g9[2] * other[e3]) + (wedge_g9[3] * other[e4])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43])
                    - (other[e23] * wedge_g6[0])
                    - (other[e31] * wedge_g6[1])
                    - (other[e12] * wedge_g6[2])
                    - (other[e423] * wedge_g3[0])
                    - (other[e431] * wedge_g3[1])
                    - (other[e412] * wedge_g3[2])
                    - (wedge_g3[3] * other[e321])
                    - (wedge_g6[3] * other[e45]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(wedge_g0_y) * other.group1())
                + ((Simd32x3::from(wedge_g6[3]) * other.group6().xyz())
                    + (Simd32x3::from(wedge_g9[3]) * other.group4())
                    + (Simd32x3::from(other[e321]) * wedge_g6.xyz())
                    + (Simd32x3::from(other[e1234]) * wedge_g3.xyz())
                    + (wedge_g8.yzx() * other.group7().zxy())
                    + (other.group5().yzx() * wedge_g9.zxy())
                    - (wedge_g8.zxy() * other.group7().yzx())
                    - (other.group5().zxy() * wedge_g9.yzx()))
                .with_w(wedge_g3[3] * other[e1234]),
            // e5
            (wedge_g0_y * other[e5]) + (wedge_g9[0] * other[e15]) + (wedge_g9[1] * other[e25]) + (wedge_g9[2] * other[e35]) + (wedge_g9[3] * other[e45])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435])
                - (other[e235] * wedge_g6[0])
                - (other[e315] * wedge_g6[1])
                - (other[e125] * wedge_g6[2])
                - (wedge_g3[0] * other[e4235])
                - (wedge_g3[1] * other[e4315])
                - (wedge_g3[2] * other[e4125])
                - (wedge_g3[3] * other[e3215]),
            // e15, e25, e35, e45
            (wedge_g3 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group3())
                + ((Simd32x3::from(wedge_g9[3]) * other.group6().xyz())
                    + (Simd32x3::from(other[e3215]) * wedge_g6.xyz())
                    + (wedge_g8.yzx() * other.group9().zxy())
                    + (other.group8().yzx() * wedge_g9.zxy())
                    - (wedge_g8.zxy() * other.group9().yzx())
                    - (other.group8().zxy() * wedge_g9.yzx()))
                .with_w(0.0),
            // e41, e42, e43
            (Simd32x3::from(wedge_g0_y) * other.group4()) + (Simd32x3::from(other[e1234]) * wedge_g6.xyz()) + (other.group7().zxy() * wedge_g9.yzx())
                - (other.group7().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (wedge_g8 * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0_y) * other.group5()) + (Simd32x3::from(wedge_g9[3]) * other.group7())
                - (Simd32x3::from(wedge_g6[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, wedge_g9[1] * other[e4235] * -1.0, 0.0])
                + (wedge_g6 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group6())
                + (wedge_g9.yzxw() * other.group9().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group9().yz()).with_zw(0.0, 0.0),
            // e423, e431, e412
            (Simd32x3::from(wedge_g0_y) * other.group7()) + (Simd32x3::from(other[e1234]) * wedge_g9.xyz()),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (wedge_g9 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group9()),
            // e1234
            wedge_g0_y * other[e1234],
        )
    }
}
impl ProjectViaOriginOnto<Plane> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2       11        0        0
    //    simd2        2        6        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        4        4        0      N/A
    // Totals...
    // yes simd        9       22        0      N/A
    //  no simd       25       42        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g1 = Simd32x4::from([0.0, 0.0, (self[e15] * other[e4315]) - (self[e25] * other[e4235]), 0.0])
            + ((self.group0().yz() * other.group0().zx()) - (self.group0().zx() * other.group0().yz())).with_zw(0.0, 0.0);
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, (wedge_g1[0] * other[e4315]) - (wedge_g1[1] * other[e4235]), 0.0])
                + ((Simd32x3::from(self[e45] * other[e3215]) * other.group0().xyz()) + ((wedge_g1.yz() * other.group0().zx()) - (wedge_g1.zx() * other.group0().yz())).with_z(0.0))
                    .with_w(0.0),
            // e4235, e4315, e4125, e3215
            -(other.group0().xy() * other.group0().xy() * self.group1().xy())
                .with_zw(other[e4125] * other[e4125] * self[e4125] * -1.0, self[e4235] * other[e4235] * other[e3215] * -1.0)
                - (self.group1().yxxy() * other.group0().xxxy() * other.group0().yyzw())
                - (self.group1().zzyz() * other.group0().xyyz() * other.group0().zzzw()),
        )
    }
}
impl ProjectViaOriginOnto<Sphere> for Flector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd3        4       10        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6       16        0      N/A
    //  no simd       14       39        0        0
    fn project_via_origin_onto(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = (Simd32x3::from(other[e1234]) * self.group0().xyz()) - (right_dual_g0_xyz * Simd32x3::from(self[e45]));
        let wedge_g0_w = right_dual_g0_xyz[0] * self[e4235];
        let wedge_g1_xyz = (right_dual_g0_xyz.yzx() * self.group0().zxy()) - (right_dual_g0_xyz.zxy() * self.group0().yzx());
        DipoleInversion::from_groups(
            // e41, e42, e43
            wedge_g0_xyz * Simd32x3::from(other[e1234]),
            // e23, e31, e12, e45
            (wedge_g1_xyz * Simd32x3::from(other[e1234])).with_w(-(wedge_g0_xyz[0] * other[e4235]) - (wedge_g0_xyz[1] * other[e4315]) - (wedge_g0_xyz[2] * other[e4125])),
            // e15, e25, e35, e1234
            ((wedge_g0_xyz * Simd32x3::from(other[e3215])) + (wedge_g1_xyz.yzx() * other.group0().zxy()) - (wedge_g1_xyz.zxy() * other.group0().yzx()))
                .with_w(wedge_g0_w * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0_w) * other.group0(),
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        9        0        0
    //    simd2        1        2        0      N/A
    //    simd3        5        6        0      N/A
    //    simd4        5        6        0      N/A
    // Totals...
    // yes simd       15       23        0      N/A
    //  no simd       41       55        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let wedge_g0 = Simd32x4::from(right_dual_g0_w) * self.group0();
        let wedge_g1 =
            ((Simd32x3::from(right_dual_g0_w) * self.group1().xyz()) + (Simd32x3::from(self[e45]) * other.group1().xyz()) + (right_dual_g0_xyz.yzx() * self.group0().zxy())
                - (right_dual_g0_xyz.zxy() * self.group0().yzx()))
            .with_w(right_dual_g0_w * self[e3215]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (wedge_g1.yzxx() * other.group0().zxy().with_w(other[e1]))
                + Simd32x3::from(0.0).with_w(-(wedge_g0[1] * other[e431]) - (wedge_g0[2] * other[e412]) - (wedge_g0[3] * other[e321]))
                - (other.group0().yzxx() * wedge_g1.zxy().with_w(wedge_g0[0])),
            // e23, e31, e12, e45
            (other.group0() * Simd32x3::from(wedge_g1[3]).with_w(wedge_g0[3])) + Simd32x3::from(0.0).with_w(-(wedge_g1[1] * other[e425]) - (wedge_g1[2] * other[e435]))
                - (wedge_g1.xyzx() * other.group1().wwwx()),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, (wedge_g1[1] * other[e235]) - (wedge_g1[0] * other[e315]), 0.0])
                + ((Simd32x3::from(wedge_g1[3]) * other.group1().xyz())
                    + (Simd32x3::from(other[e12345]) * wedge_g0.xyz())
                    + ((wedge_g1.zx() * other.group2().yz()) - (wedge_g1.yz() * other.group2().zx())).with_z(0.0))
                .with_w(0.0),
            // e4235, e4315, e4125, e3215
            wedge_g1 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<VersorOdd> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        5       11        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        9       20        0      N/A
    //  no simd       25       51        0        0
    fn project_via_origin_onto(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 =
            (Simd32x3::from(other[e1234]) * self.group0().xyz()).with_w(right_dual_g3_xyz[0] * self[e4235]) - (Simd32x4::from(self[e45]) * right_dual_g3_xyz.with_w(other[e45]));
        let wedge_g1_xyz = (right_dual_g3_xyz.yzx() * self.group0().zxy()) - (right_dual_g3_xyz.zxy() * self.group0().yzx());
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (wedge_g0 * Simd32x3::from(other[e1234]).with_w(other[scalar]))
                + (Simd32x3::from(wedge_g0[3]) * other.group0().xyz()).with_w(-(wedge_g0[0] * other[e23]) - (wedge_g0[1] * other[e31]) - (wedge_g0[2] * other[e12])),
            // e23, e31, e12, e45
            ((wedge_g1_xyz * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0[3]) * other.group1().xyz())).with_w(wedge_g0[3] * other[e45]),
            // e15, e25, e35, e1234
            ((Simd32x3::from(wedge_g0[3]) * other.group2().xyz()) + (Simd32x3::from(other[e3215]) * wedge_g0.xyz()) + (wedge_g1_xyz.yzx() * other.group3().zxy())
                - (wedge_g1_xyz.zxy() * other.group3().yzx()))
            .with_w(wedge_g0[3] * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0[3]) * other.group3(),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for Line {
    type Output = ProjectViaOriginOntoInfixPartial<Line>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiDipoleInversion> for Line {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        5       10        0      N/A
    //  no simd        5       21        0        0
    fn project_via_origin_onto(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0 = -(right_dual_g1_xyz[0] * self[e415])
            - (right_dual_g1_xyz[1] * self[e425])
            - (right_dual_g1_xyz[2] * self[e435])
            - (other[e423] * self[e235])
            - (other[e431] * self[e315])
            - (other[e412] * self[e125]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(wedge_g0) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(wedge_g0) * other.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(wedge_g0) * other.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(wedge_g0) * other.group3(),
        )
    }
}
impl ProjectViaOriginOnto<AntiScalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0        9        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e12345] * -1.0;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(right_dual_g0 * other[e12345]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(right_dual_g0 * other[e12345]) * self.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Circle> for Line {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        5        9        0      N/A
    //  no simd        5       16        0        0
    fn project_via_origin_onto(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0 = -(right_dual_g1_xyz[0] * self[e415])
            - (right_dual_g1_xyz[1] * self[e425])
            - (right_dual_g1_xyz[2] * self[e435])
            - (other[e423] * self[e235])
            - (other[e431] * self[e315])
            - (other[e412] * self[e125]);
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(wedge_g0) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(wedge_g0) * other.group1(),
            // e235, e315, e125
            Simd32x3::from(wedge_g0) * other.group2(),
        )
    }
}
impl ProjectViaOriginOnto<CircleRotor> for Line {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        8        0        0
    //    simd3        4        9        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        9       18        0      N/A
    //  no simd       17       39        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g2_w = other[e12345] * -1.0;
        let wedge_g0_xyz = Simd32x3::from(right_dual_g2_w) * self.group0();
        let wedge_g0_w = -(right_dual_g1_xyz[0] * self[e415])
            - (right_dual_g1_xyz[1] * self[e425])
            - (right_dual_g1_xyz[2] * self[e435])
            - (other[e423] * self[e235])
            - (other[e431] * self[e315])
            - (other[e412] * self[e125]);
        let wedge_g1_xyz = Simd32x3::from(right_dual_g2_w) * self.group1();
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(wedge_g0_w) * other.group0().with_w(other[e12345]),
            // e415, e425, e435, e321
            ((wedge_g0_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_w) * other.group1().xyz())).with_w(wedge_g0_w * other[e321]),
            // e235, e315, e125, e5
            ((wedge_g1_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_w) * other.group2().xyz())).with_w(0.0),
            // e1, e2, e3, e4
            ((wedge_g0_xyz * Simd32x3::from(other[e321])) + (wedge_g1_xyz.yzx() * other.group0().zxy()) - (wedge_g1_xyz.zxy() * other.group0().yzx())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<DipoleInversion> for Line {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        5       10        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        6       15        0      N/A
    //  no simd       19       38        0        0
    fn project_via_origin_onto(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = (self.group0().yzx() * other.group3().zxy()) - (Simd32x3::from(other[e1234]) * self.group1()) - (self.group0().zxy() * other.group3().yzx());
        let wedge_g0_w = self[e235] * other[e4235] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            wedge_g0_xyz * Simd32x3::from(other[e1234]),
            // e415, e425, e435, e321
            ((wedge_g0_xyz.yzx() * other.group3().zxy()) - (wedge_g0_xyz.zxy() * other.group3().yzx())).with_w(wedge_g0_w * other[e1234]),
            // e235, e315, e125, e4
            ((Simd32x3::from(wedge_g0_w) * other.group3().xyz()) - (wedge_g0_xyz * Simd32x3::from(other[e3215]))).with_w(0.0),
            // e1, e2, e3, e5
            (Simd32x4::from(wedge_g0_w) * other.group0().with_w(other[e45]))
                + ((wedge_g0_xyz.zxy() * other.group1().yzx()) - (wedge_g0_xyz.yzx() * other.group1().zxy())).with_w(wedge_g0_xyz[0] * other[e15]),
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       10        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(other[e12345] * other[e12345] * -1.0) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other[e12345] * other[e12345] * -1.0) * self.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Flector> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        3        6        0      N/A
    // Totals...
    // yes simd        3       10        0      N/A
    //  no simd        9       22        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = (self.group0().yzx() * other.group1().zxy()) - (self.group0().zxy() * other.group1().yzx());
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((wedge_g0_xyz.yzx() * other.group1().zxy()) - (wedge_g0_xyz.zxy() * other.group1().yzx())).with_w(0.0),
            // e235, e315, e125, e5
            (-(wedge_g0_xyz * Simd32x3::from(other[e3215]))
                - (Simd32x3::from(other[e4235]) * Simd32x3::from([self[e235] * other[e4235], self[e235] * other[e4315], self[e235] * other[e4125]])))
            .with_w(wedge_g0_xyz[0] * other[e15]),
        )
    }
}
impl ProjectViaOriginOnto<Line> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        9        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = -(other[e415] * self[e415]) - (other[e425] * self[e425]) - (other[e435] * self[e435]);
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(wedge_g0) * other.group0(),
            // e235, e315, e125
            Simd32x3::from(wedge_g0) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       11        0        0
    //    simd3        1        4        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       10       17        0      N/A
    //  no simd       18       31        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let wedge_g0_xyz = Simd32x3::from(right_dual_g0_w) * self.group0();
        let wedge_g0_w = -(right_dual_g0_xyz[0] * self[e415]) - (right_dual_g0_xyz[1] * self[e425]) - (right_dual_g0_xyz[2] * self[e435]);
        let wedge_g1 = (Simd32x3::from(right_dual_g0_w) * self.group1()).with_w(0.0);
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((wedge_g0_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_w) * other.group0().xyz())).with_w(wedge_g0_w * other[e12345]),
            // e235, e315, e125, e5
            (wedge_g1 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_w) * other.group1())
                + Simd32x3::from(0.0).with_w(
                    -(wedge_g0_xyz[0] * other[e235])
                        - (wedge_g0_xyz[1] * other[e315])
                        - (wedge_g0_xyz[2] * other[e125])
                        - (wedge_g1[0] * other[e415])
                        - (wedge_g1[1] * other[e425])
                        - (wedge_g1[2] * other[e435]),
                ),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       26       36        0        0
    //    simd2        0        2        0      N/A
    //    simd3       26       36        0      N/A
    //    simd4        5        6        0      N/A
    // Totals...
    // yes simd       57       80        0      N/A
    //  no simd      124      172        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let right_dual_g5 = other.group6().xyz();
        let wedge_g0_y = -(right_dual_g5[0] * self[e415])
            - (right_dual_g5[1] * self[e425])
            - (right_dual_g5[2] * self[e435])
            - (self[e235] * other[e423])
            - (self[e315] * other[e431])
            - (self[e125] * other[e412]);
        let wedge_g6 = Simd32x4::from([1.0, 1.0, right_dual_g0[0], 0.0]) * (self.group0() * Simd32x2::from(right_dual_g0[0]).with_z(1.0)).with_w(0.0);
        let wedge_g8 = Simd32x3::from(right_dual_g0[0]) * self.group1();
        let wedge_g9 = ((right_dual_g1_xyz.yzx() * self.group0().zxy()) - (Simd32x3::from(other[e1234]) * self.group1()) - (right_dual_g1_xyz.zxy() * self.group0().yzx()))
            .with_w(right_dual_g1_xyz[0] * self[e235]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar]) + (wedge_g9[0] * other[e1]) + (wedge_g9[1] * other[e2]) + (wedge_g9[2] * other[e3]) + (wedge_g9[3] * other[e4])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43])
                    - (other[e23] * wedge_g6[0])
                    - (other[e31] * wedge_g6[1])
                    - (other[e12] * wedge_g6[2])
                    - (wedge_g6[3] * other[e45]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g0_y) * other.group1().xyz())
                + (Simd32x3::from(wedge_g6[3]) * other.group6().xyz())
                + (Simd32x3::from(wedge_g9[3]) * other.group4())
                + (Simd32x3::from(other[e321]) * wedge_g6.xyz())
                + (wedge_g8.yzx() * other.group7().zxy())
                + (other.group5().yzx() * wedge_g9.zxy())
                - (wedge_g8.zxy() * other.group7().yzx())
                - (other.group5().zxy() * wedge_g9.yzx()))
            .with_w(wedge_g0_y * other[e4]),
            // e5
            (wedge_g0_y * other[e5]) + (wedge_g9[0] * other[e15]) + (wedge_g9[1] * other[e25]) + (wedge_g9[2] * other[e35]) + (wedge_g9[3] * other[e45])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435])
                - (other[e235] * wedge_g6[0])
                - (other[e315] * wedge_g6[1])
                - (other[e125] * wedge_g6[2]),
            // e15, e25, e35, e45
            ((Simd32x3::from(wedge_g0_y) * other.group3().xyz())
                + (Simd32x3::from(wedge_g9[3]) * other.group6().xyz())
                + (Simd32x3::from(other[e3215]) * wedge_g6.xyz())
                + (wedge_g8.yzx() * other.group9().zxy())
                + (other.group8().yzx() * wedge_g9.zxy())
                - (wedge_g8.zxy() * other.group9().yzx())
                - (other.group8().zxy() * wedge_g9.yzx()))
            .with_w(wedge_g0_y * other[e45]),
            // e41, e42, e43
            (Simd32x3::from(wedge_g0_y) * other.group4()) + (Simd32x3::from(other[e1234]) * wedge_g6.xyz()) + (other.group7().zxy() * wedge_g9.yzx())
                - (other.group7().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (wedge_g8 * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0_y) * other.group5()) + (Simd32x3::from(wedge_g9[3]) * other.group7())
                - (Simd32x3::from(wedge_g6[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, wedge_g9[1] * other[e4235] * -1.0, 0.0])
                + (wedge_g6 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group6())
                + (wedge_g9.yzxw() * other.group9().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group9().yz()).with_zw(0.0, 0.0),
            // e423, e431, e412
            (Simd32x3::from(wedge_g0_y) * other.group7()) + (Simd32x3::from(other[e1234]) * wedge_g9.xyz()),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (wedge_g9 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group9()),
            // e1234
            wedge_g0_y * other[e1234],
        )
    }
}
impl ProjectViaOriginOnto<Plane> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        3        6        0      N/A
    // Totals...
    // yes simd        3        9        0      N/A
    //  no simd        9       21        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = (self.group0().yzx() * other.group0().zxy()) - (self.group0().zxy() * other.group0().yzx());
        Line::from_groups(
            // e415, e425, e435
            (wedge_g0_xyz.yzx() * other.group0().zxy()) - (wedge_g0_xyz.zxy() * other.group0().yzx()),
            // e235, e315, e125
            -(wedge_g0_xyz * Simd32x3::from(other[e3215]))
                - (Simd32x3::from(other[e4235]) * Simd32x3::from([self[e235] * other[e4235], self[e235] * other[e4315], self[e235] * other[e4125]])),
        )
    }
}
impl ProjectViaOriginOnto<Sphere> for Line {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        4        9        0      N/A
    // Totals...
    // yes simd        4       11        0      N/A
    //  no simd       12       29        0        0
    fn project_via_origin_onto(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = (right_dual_g0_xyz.yzx() * self.group0().zxy()) - (Simd32x3::from(other[e1234]) * self.group1()) - (right_dual_g0_xyz.zxy() * self.group0().yzx());
        let wedge_g0_w = right_dual_g0_xyz[0] * self[e235];
        Circle::from_groups(
            // e423, e431, e412
            wedge_g0_xyz * Simd32x3::from(other[e1234]),
            // e415, e425, e435, e321
            ((wedge_g0_xyz.yzx() * other.group0().zxy()) - (wedge_g0_xyz.zxy() * other.group0().yzx())).with_w(wedge_g0_w * other[e1234]),
            // e235, e315, e125
            (Simd32x3::from(wedge_g0_w) * other.group0().xyz()) - (wedge_g0_xyz * Simd32x3::from(other[e3215])),
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for Line {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       21        0        0
    //    simd3        3        6        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd       18       30        0      N/A
    //  no simd       30       51        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0_xyz = Simd32x3::from(right_dual_g0_w) * self.group0();
        let wedge_g0_w = -(right_dual_g0_xyz[0] * self[e235])
            - (right_dual_g0_xyz[1] * self[e315])
            - (right_dual_g0_xyz[2] * self[e125])
            - (right_dual_g1_xyz[0] * self[e415])
            - (right_dual_g1_xyz[1] * self[e425])
            - (right_dual_g1_xyz[2] * self[e435]);
        let wedge_g1 = (Simd32x3::from(right_dual_g0_w) * self.group1()).with_w(0.0);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(wedge_g0_w) * other.group0(),
            // e415, e425, e435, e321
            ((wedge_g0_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_w) * other.group1().xyz())).with_w(wedge_g0_w * other[e321]),
            // e235, e315, e125, e5
            (wedge_g1 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_w) * other.group2())
                + Simd32x3::from(0.0).with_w(
                    -(wedge_g0_xyz[0] * other[e235])
                        - (wedge_g0_xyz[1] * other[e315])
                        - (wedge_g0_xyz[2] * other[e125])
                        - (wedge_g1[0] * other[e415])
                        - (wedge_g1[1] * other[e425])
                        - (wedge_g1[2] * other[e435]),
                ),
            // e1, e2, e3, e4
            (Simd32x3::from([
                (wedge_g1[1] * other[e412]) - (wedge_g1[2] * other[e431]),
                (wedge_g1[2] * other[e423]) - (wedge_g1[0] * other[e412]),
                (wedge_g1[0] * other[e431]) - (wedge_g1[1] * other[e423]),
            ]) + (wedge_g0_xyz * Simd32x3::from(other[e321]))
                + (Simd32x3::from(wedge_g0_w) * other.group3().xyz()))
            .with_w(wedge_g0_w * other[e4]),
        )
    }
}
impl ProjectViaOriginOnto<VersorOdd> for Line {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd3        6       12        0      N/A
    // Totals...
    // yes simd        7       16        0      N/A
    //  no simd       19       40        0        0
    fn project_via_origin_onto(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = (right_dual_g3_xyz.yzx() * self.group0().zxy()) - (Simd32x3::from(other[e1234]) * self.group1()) - (right_dual_g3_xyz.zxy() * self.group0().yzx());
        let wedge_g0_w = right_dual_g3_xyz[0] * self[e235];
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            wedge_g0_xyz * Simd32x3::from(other[e1234]),
            // e415, e425, e435, e321
            ((wedge_g0_xyz.yzx() * other.group3().zxy()) - (wedge_g0_xyz.zxy() * other.group3().yzx())).with_w(wedge_g0_w * other[e1234]),
            // e235, e315, e125, e4
            ((Simd32x3::from(wedge_g0_w) * other.group3().xyz()) - (wedge_g0_xyz * Simd32x3::from(other[e3215]))).with_w(0.0),
            // e1, e2, e3, e5
            ((Simd32x3::from(wedge_g0_w) * other.group0().xyz()) + (wedge_g0_xyz.zxy() * other.group1().yzx()) - (wedge_g0_xyz.yzx() * other.group1().zxy()))
                .with_w((wedge_g0_xyz[0] * other[e15]) + (wedge_g0_xyz[1] * other[e25])),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for Motor {
    type Output = ProjectViaOriginOntoInfixPartial<Motor>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiCircleRotor> for Motor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        2        4        0      N/A
    // Totals...
    // yes simd        5       10        0      N/A
    //  no simd        9       18        0        0
    fn project_via_origin_onto(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(self[e5]) * other.group0();
        let wedge_g0_w = other[e45] * self[e5] * -1.0;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g0_w) * other.group0()) + (wedge_g0_xyz.zxy() * other.group1().yzx()) - (wedge_g0_xyz.yzx() * other.group1().zxy())).with_w(0.0),
            // e5
            (wedge_g0_w * other[e45]) + (wedge_g0_xyz[0] * other[e15]) + (wedge_g0_xyz[1] * other[e25]) + (wedge_g0_xyz[2] * other[e35]),
        )
    }
}
impl ProjectViaOriginOnto<AntiDipoleInversion> for Motor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       12        0        0
    //    simd3        3        8        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       12       21        0      N/A
    //  no simd       18       40        0        0
    fn project_via_origin_onto(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0_xyz = Simd32x3::from(self[e5]) * other.group0();
        let wedge_g0_w = -(right_dual_g1_xyz[0] * self[e415])
            - (right_dual_g1_xyz[1] * self[e425])
            - (right_dual_g1_xyz[2] * self[e435])
            - (other[e423] * self[e235])
            - (other[e431] * self[e315])
            - (other[e412] * self[e125])
            - (other[e4] * self[e5]);
        let wedge_g1 = (right_dual_g1_xyz * Simd32x4::from(self[e5]).xyz()).with_w(0.0);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(wedge_g0_w) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(wedge_g0_w) * other.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(wedge_g0_w) * other.group2().xyz())
                .with_w((wedge_g0_w * other[e4]) - (wedge_g0_xyz[0] * other[e423]) - (wedge_g0_xyz[1] * other[e431]) - (wedge_g0_xyz[2] * other[e412])),
            // e1, e2, e3, e5
            ((wedge_g0_xyz * Simd32x3::from(other[e321])) + (Simd32x3::from(wedge_g0_w) * other.group3().xyz()) + (other.group0().zxy() * wedge_g1.yzx())
                - (other.group0().yzx() * wedge_g1.zxy()))
            .with_w(wedge_g0_w * other[e5]),
        )
    }
}
impl ProjectViaOriginOnto<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       11        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e12345] * -1.0;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Circle> for Motor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7        9        0        0
    //    simd3        2        7        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        9       17        0      N/A
    //  no simd       13       34        0        0
    fn project_via_origin_onto(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0_xyz = Simd32x3::from(self[e5]) * other.group0();
        let wedge_g0_w = -(right_dual_g1_xyz[0] * self[e415])
            - (right_dual_g1_xyz[1] * self[e425])
            - (right_dual_g1_xyz[2] * self[e435])
            - (other[e423] * self[e235])
            - (other[e431] * self[e315])
            - (other[e412] * self[e125]);
        let wedge_g1 = (right_dual_g1_xyz * Simd32x4::from(self[e5]).xyz()).with_w(0.0);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(wedge_g0_w) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(wedge_g0_w) * other.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(wedge_g0_w) * other.group2()).with_w(-(wedge_g0_xyz[0] * other[e423]) - (wedge_g0_xyz[1] * other[e431]) - (wedge_g0_xyz[2] * other[e412])),
            // e1, e2, e3, e5
            ((wedge_g0_xyz * Simd32x3::from(other[e321])) + (other.group0().zxy() * wedge_g1.yzx()) - (other.group0().yzx() * wedge_g1.zxy())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<CircleRotor> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd3        6       11        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6       17        0      N/A
    //  no simd       18       42        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2_w = other[e12345] * -1.0;
        let wedge_g0_xyz = (Simd32x3::from(right_dual_g2_w) * self.group0().xyz()) + (Simd32x3::from(self[e5]) * other.group0());
        let wedge_g0_w = right_dual_g2_w * self[e12345];
        let wedge_g1_xyz = (Simd32x3::from(right_dual_g2_w) * self.group1().xyz()) + (Simd32x3::from(self[e5]) * other.group1().xyz());
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(wedge_g0_w) * other.group0().with_w(other[e12345]),
            // e415, e425, e435, e321
            ((wedge_g0_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_w) * other.group1().xyz())).with_w(wedge_g0_w * other[e321]),
            // e235, e315, e125, e5
            ((wedge_g1_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_w) * other.group2().xyz())).with_w(right_dual_g2_w * other[e12345] * self[e5]),
            // e1, e2, e3, e4
            ((wedge_g0_xyz * Simd32x3::from(other[e321])) + (wedge_g1_xyz.yzx() * other.group0().zxy()) - (wedge_g1_xyz.zxy() * other.group0().yzx())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<Dipole> for Motor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        2        4        0      N/A
    // Totals...
    // yes simd        5       10        0      N/A
    //  no simd        9       18        0        0
    fn project_via_origin_onto(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(self[e5]) * other.group0();
        let wedge_g0_w = other[e45] * self[e5] * -1.0;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g0_w) * other.group0()) + (wedge_g0_xyz.zxy() * other.group1().yzx()) - (wedge_g0_xyz.yzx() * other.group1().zxy())).with_w(0.0),
            // e5
            (wedge_g0_w * other[e45]) + (wedge_g0_xyz[0] * other[e15]) + (wedge_g0_xyz[1] * other[e25]) + (wedge_g0_xyz[2] * other[e35]),
        )
    }
}
impl ProjectViaOriginOnto<DipoleInversion> for Motor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       19        0        0
    //    simd3        6       10        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd       18       32        0      N/A
    //  no simd       33       61        0        0
    fn project_via_origin_onto(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[e5]) * other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let wedge_g1_xyz = Simd32x3::from([
            (other[e4125] * self[e425]) - (other[e4315] * self[e435]),
            (other[e4235] * self[e435]) - (other[e4125] * self[e415]),
            (other[e4315] * self[e415]) - (other[e4235] * self[e425]),
        ]) + (Simd32x3::from(self[e5]) * other.group0())
            - (Simd32x3::from(other[e1234]) * self.group1().xyz());
        let wedge_g1_w = other[e4235] * self[e235] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            wedge_g1_xyz * Simd32x3::from(other[e1234]),
            // e415, e425, e435, e321
            ((wedge_g1_xyz.yzx() * other.group3().zxy()) - (wedge_g1_xyz.zxy() * other.group3().yzx())).with_w(wedge_g1_w * other[e1234]),
            // e235, e315, e125, e4
            ((Simd32x3::from(wedge_g1_w) * other.group3().xyz()) - (wedge_g1_xyz * Simd32x3::from(other[e3215])))
                .with_w((wedge_g0[3] * other[e1234]) - (wedge_g1_xyz[0] * other[e41]) - (wedge_g1_xyz[1] * other[e42]) - (wedge_g1_xyz[2] * other[e43])),
            // e1, e2, e3, e5
            (Simd32x4::from(wedge_g1_w) * other.group0().with_w(other[e45]))
                + ((Simd32x3::from(other[e1234]) * wedge_g0.xyz()) + (wedge_g1_xyz.zxy() * other.group1().yzx()) - (wedge_g1_xyz.yzx() * other.group1().zxy())).with_w(
                    (wedge_g1_xyz[0] * other[e15]) + (wedge_g1_xyz[1] * other[e25])
                        - (wedge_g0[0] * other[e4235])
                        - (wedge_g0[1] * other[e4315])
                        - (wedge_g0[2] * other[e4125])
                        - (wedge_g0[3] * other[e3215]),
                ),
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        1        8        0      N/A
    //  no simd        1       19        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(other[e12345] * -1.0) * self.group0();
        let wedge_g1 = Simd32x4::from(other[e12345] * -1.0) * self.group1();
        Motor::from_groups(
            // e415, e425, e435, e12345
            wedge_g0 * Simd32x4::from(other[e12345]),
            // e235, e315, e125, e5
            (wedge_g1.xyz() * Simd32x2::from(other[e12345]).with_z(other[e12345])).with_w((other[e5] * wedge_g0[3]) + (other[e12345] * wedge_g1[3])),
        )
    }
}
impl ProjectViaOriginOnto<FlatPoint> for Motor {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn project_via_origin_onto(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([other[e45] * other[e45] * self[e5] * -1.0, 0.0]))
    }
}
impl ProjectViaOriginOnto<Flector> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        7        0        0
    //    simd2        2        4        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd       20       25        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(self[e5]) * other.group1().xyz();
        let wedge_g1 = Simd32x4::from([0.0, 0.0, (other[e4315] * self[e415]) - (other[e4235] * self[e425]), 0.0])
            + ((other.group1().zx() * self.group0().yz()) - (other.group1().yz() * self.group0().zx())).with_zw(0.0, 0.0);
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, (wedge_g1[0] * other[e4315]) - (wedge_g1[1] * other[e4235]), 0.0])
                + ((wedge_g1.yz() * other.group1().zx()) - (wedge_g1.zx() * other.group1().yz())).with_zw(0.0, 0.0),
            // e235, e315, e125, e5
            (wedge_g1.wwwx() * other.group1().xyz().with_w(other[e15]))
                + -(Simd32x3::from(other[e3215]) * wedge_g1.xyz()).with_w(-(wedge_g0_xyz[0] * other[e4235]) - (wedge_g0_xyz[1] * other[e4315]) - (wedge_g0_xyz[2] * other[e4125])),
        )
    }
}
impl ProjectViaOriginOnto<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        3        0      N/A
    // Totals...
    // yes simd        4        9        0      N/A
    //  no simd        4       15        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_w = -(other[e415] * self[e415]) - (other[e425] * self[e425]) - (other[e435] * self[e435]);
        let wedge_g1_xyz = Simd32x3::from(self[e5]) * other.group0();
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x3::from(wedge_g0_w) * other.group0()).with_w(0.0),
            // e235, e315, e125, e5
            (Simd32x3::from(wedge_g0_w) * other.group1()).with_w(-(wedge_g1_xyz[0] * other[e415]) - (wedge_g1_xyz[1] * other[e425]) - (wedge_g1_xyz[2] * other[e435])),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       13        0        0
    //    simd3        2        5        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       12       20        0      N/A
    //  no simd       22       36        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let wedge_g0_xyz = Simd32x3::from(right_dual_g0_w) * self.group0().xyz();
        let wedge_g0_w = (right_dual_g0_w * self[e12345]) - (right_dual_g0_xyz[0] * self[e415]) - (right_dual_g0_xyz[1] * self[e425]) - (right_dual_g0_xyz[2] * self[e435]);
        let wedge_g1 = ((right_dual_g0_xyz * Simd32x3::from(self[e5])) + (Simd32x3::from(right_dual_g0_w) * self.group1().xyz())).with_w(right_dual_g0_w * self[e5]);
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((wedge_g0_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_w) * other.group0().xyz())).with_w(wedge_g0_w * other[e12345]),
            // e235, e315, e125, e5
            (wedge_g1 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_w) * other.group1())
                + Simd32x3::from(0.0).with_w(
                    -(wedge_g0_xyz[0] * other[e235])
                        - (wedge_g0_xyz[1] * other[e315])
                        - (wedge_g0_xyz[2] * other[e125])
                        - (wedge_g1[0] * other[e415])
                        - (wedge_g1[1] * other[e425])
                        - (wedge_g1[2] * other[e435]),
                ),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       41       55        0        0
    //    simd2        0        2        0      N/A
    //    simd3       27       36        0      N/A
    //    simd4        8        9        0      N/A
    // Totals...
    // yes simd       76      102        0      N/A
    //  no simd      154      203        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1 = (other.group9().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let right_dual_g5 = other.group6().xyz();
        let wedge_g0_y = (right_dual_g0[0] * self[e12345])
            - (right_dual_g5[0] * self[e415])
            - (right_dual_g5[1] * self[e425])
            - (right_dual_g5[2] * self[e435])
            - (other[e423] * self[e235])
            - (other[e431] * self[e315])
            - (other[e412] * self[e125])
            - (self[e5] * other[e4]);
        let wedge_g2 = right_dual_g0[0] * self[e5];
        let wedge_g3 = right_dual_g1 * Simd32x4::from(self[e5] * -1.0);
        let wedge_g6 = ((Simd32x3::from(right_dual_g0[0]) * self.group0().xyz()) + (Simd32x3::from(self[e5]) * other.group7())).with_w(0.0);
        let wedge_g8 = (right_dual_g5 * Simd32x3::from(self[e5])) + (Simd32x3::from(right_dual_g0[0]) * self.group1().xyz());
        let wedge_g9 = (Simd32x3::from([
            (right_dual_g1[1] * self[e435]) - (right_dual_g1[2] * self[e425]),
            (right_dual_g1[2] * self[e415]) - (right_dual_g1[0] * self[e435]),
            (right_dual_g1[0] * self[e425]) - (right_dual_g1[1] * self[e415]),
        ]) + (Simd32x3::from(self[e5]) * other.group4())
            - (Simd32x3::from(right_dual_g1[3]) * self.group1().xyz()))
        .with_w(right_dual_g1[0] * self[e235]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar])
                    + (wedge_g2 * other[e1234])
                    + (wedge_g9[0] * other[e1])
                    + (wedge_g9[1] * other[e2])
                    + (wedge_g9[2] * other[e3])
                    + (wedge_g9[3] * other[e4])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43])
                    - (other[e23] * wedge_g6[0])
                    - (other[e31] * wedge_g6[1])
                    - (other[e12] * wedge_g6[2])
                    - (other[e423] * wedge_g3[0])
                    - (other[e431] * wedge_g3[1])
                    - (other[e412] * wedge_g3[2])
                    - (wedge_g3[3] * other[e321])
                    - (wedge_g6[3] * other[e45]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(wedge_g0_y) * other.group1())
                + ((Simd32x3::from(wedge_g6[3]) * other.group6().xyz())
                    + (Simd32x3::from(wedge_g9[3]) * other.group4())
                    + (Simd32x3::from(other[e321]) * wedge_g6.xyz())
                    + (Simd32x3::from(other[e1234]) * wedge_g3.xyz())
                    + (wedge_g8.yzx() * other.group7().zxy())
                    + (other.group5().yzx() * wedge_g9.zxy())
                    - (wedge_g8.zxy() * other.group7().yzx())
                    - (other.group5().zxy() * wedge_g9.yzx()))
                .with_w(wedge_g3[3] * other[e1234]),
            // e5
            (wedge_g0_y * other[e5])
                + (wedge_g2 * other[e12345])
                + (wedge_g9[0] * other[e15])
                + (wedge_g9[1] * other[e25])
                + (wedge_g9[2] * other[e35])
                + (wedge_g9[3] * other[e45])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435])
                - (other[e235] * wedge_g6[0])
                - (other[e315] * wedge_g6[1])
                - (other[e125] * wedge_g6[2])
                - (wedge_g3[0] * other[e4235])
                - (wedge_g3[1] * other[e4315])
                - (wedge_g3[2] * other[e4125])
                - (wedge_g3[3] * other[e3215]),
            // e15, e25, e35, e45
            (wedge_g3 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group3())
                + ((Simd32x3::from(wedge_g9[3]) * other.group6().xyz())
                    + (Simd32x3::from(other[e3215]) * wedge_g6.xyz())
                    + (wedge_g8.yzx() * other.group9().zxy())
                    + (other.group8().yzx() * wedge_g9.zxy())
                    - (wedge_g8.zxy() * other.group9().yzx())
                    - (other.group8().zxy() * wedge_g9.yzx()))
                .with_w(0.0),
            // e41, e42, e43
            (Simd32x3::from(wedge_g0_y) * other.group4()) + (Simd32x3::from(other[e1234]) * wedge_g6.xyz()) + (other.group7().zxy() * wedge_g9.yzx())
                - (other.group7().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (wedge_g8 * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0_y) * other.group5()) + (Simd32x3::from(wedge_g9[3]) * other.group7())
                - (Simd32x3::from(wedge_g6[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, wedge_g9[1] * other[e4235] * -1.0, 0.0])
                + (wedge_g6 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group6())
                + (wedge_g9.yzxw() * other.group9().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group9().yz()).with_zw(0.0, 0.0),
            // e423, e431, e412
            (Simd32x3::from(wedge_g0_y) * other.group7()) + (Simd32x3::from(other[e1234]) * wedge_g9.xyz()),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (wedge_g9 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group9()),
            // e1234
            wedge_g0_y * other[e1234],
        )
    }
}
impl ProjectViaOriginOnto<Plane> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2       13        0        0
    //    simd2        2        4        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        6       18        0      N/A
    //  no simd       13       24        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g1_xy = (self.group0().yz() * other.group0().zx()) - (self.group0().zx() * other.group0().yz());
        let wedge_g1_z = (self[e415] * other[e4315]) - (self[e425] * other[e4235]);
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, (wedge_g1_xy[0] * other[e4315]) - (wedge_g1_xy[1] * other[e4235]), 0.0])
                + ((Simd32x2::from([wedge_g1_xy[1], wedge_g1_z]) * other.group0().zx()) - (Simd32x2::from([wedge_g1_z, wedge_g1_xy[0]]) * other.group0().yz())).with_zw(0.0, 0.0),
            // e235, e315, e125, e5
            (Simd32x3::from([
                (other[e4235] * other[e4235] * self[e235] * -1.0),
                (self[e235] * other[e4235] * other[e4315] * -1.0),
                (self[e235] * other[e4235] * other[e4125] * -1.0),
            ]) - (Simd32x3::from(other[e3215]) * wedge_g1_xy.with_z(wedge_g1_z)))
            .with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<RoundPoint> for Motor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        7        0        0
    fn project_via_origin_onto(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = self[e5] * other[e4] * -1.0;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(wedge_g0) * other.group0(), /* e5 */ wedge_g0 * other[e5])
    }
}
impl ProjectViaOriginOnto<Sphere> for Motor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       14        0        0
    //    simd3        3        8        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        9       23        0      N/A
    //  no simd       15       42        0        0
    fn project_via_origin_onto(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let wedge_g0 = right_dual_g0 * Simd32x4::from(self[e5] * -1.0);
        let wedge_g1_xyz = Simd32x3::from([
            (right_dual_g0[1] * self[e435]) - (right_dual_g0[2] * self[e425]),
            (right_dual_g0[2] * self[e415]) - (right_dual_g0[0] * self[e435]),
            (right_dual_g0[0] * self[e425]) - (right_dual_g0[1] * self[e415]),
        ]) - (Simd32x3::from(right_dual_g0[3]) * self.group1().xyz());
        let wedge_g1_w = right_dual_g0[0] * self[e235];
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            wedge_g1_xyz * Simd32x3::from(other[e1234]),
            // e415, e425, e435, e321
            ((wedge_g1_xyz.yzx() * other.group0().zxy()) - (wedge_g1_xyz.zxy() * other.group0().yzx())).with_w(wedge_g1_w * other[e1234]),
            // e235, e315, e125, e4
            ((Simd32x3::from(wedge_g1_w) * other.group0().xyz()) - (wedge_g1_xyz * Simd32x3::from(other[e3215]))).with_w(wedge_g0[3] * other[e1234]),
            // e1, e2, e3, e5
            (Simd32x3::from(other[e1234]) * wedge_g0.xyz())
                .with_w(-(wedge_g0[0] * other[e4235]) - (wedge_g0[1] * other[e4315]) - (wedge_g0[2] * other[e4125]) - (wedge_g0[3] * other[e3215])),
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       18        0        0
    //    simd3        4        7        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd       15       29        0      N/A
    //  no simd       32       55        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e12345] * -1.0);
        let wedge_g0 =
            (right_dual_g0 * Simd32x3::from(self[e5]).with_w(self[e12345])) + (Simd32x3::from(right_dual_g0[3]) * self.group0().xyz()).with_w(self[e5] * other[e4] * -1.0);
        let wedge_g1 = ((Simd32x3::from(right_dual_g0[3]) * self.group1().xyz()) + (Simd32x3::from(self[e5]) * other.group1().xyz())).with_w(right_dual_g0[3] * self[e5]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(wedge_g0[3]) * other.group0(),
            // e415, e425, e435, e321
            ((Simd32x3::from(wedge_g0[3]) * other.group1().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g0.xyz())).with_w(wedge_g0[3] * other[e321]),
            // e235, e315, e125, e5
            (wedge_g1 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0[3]) * other.group2())
                + Simd32x3::from(0.0).with_w(
                    -(wedge_g0[0] * other[e235])
                        - (wedge_g0[1] * other[e315])
                        - (wedge_g0[2] * other[e125])
                        - (wedge_g1[0] * other[e415])
                        - (wedge_g1[1] * other[e425])
                        - (wedge_g1[2] * other[e435]),
                ),
            // e1, e2, e3, e4
            (Simd32x3::from([
                (wedge_g1[1] * other[e412]) - (wedge_g1[2] * other[e431]),
                (wedge_g1[2] * other[e423]) - (wedge_g1[0] * other[e412]),
                (wedge_g1[0] * other[e431]) - (wedge_g1[1] * other[e423]),
            ]) + (Simd32x3::from(wedge_g0[3]) * other.group3().xyz())
                + (Simd32x3::from(other[e321]) * wedge_g0.xyz()))
            .with_w(wedge_g0[3] * other[e4]),
        )
    }
}
impl ProjectViaOriginOnto<VersorOdd> for Motor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       15        0        0
    //    simd2        1        4        0      N/A
    //    simd3        2        5        0      N/A
    //    simd4        6        6        0      N/A
    // Totals...
    // yes simd       15       30        0      N/A
    //  no simd       38       62        0        0
    fn project_via_origin_onto(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g3 = (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let wedge_g0 = right_dual_g3 * Simd32x4::from(self[e5] * -1.0);
        let wedge_g1 = Simd32x4::from([0.0, 0.0, right_dual_g3[1] * self[e415] * -1.0, 0.0])
            + (right_dual_g3.yzxx() * self.group0().zxy().with_w(self[e235]))
            + ((Simd32x3::from(self[e5]) * other.group0().xyz()) + -(right_dual_g3.zx() * self.group0().yz()).with_z(0.0)
                - (Simd32x3::from(right_dual_g3[3]) * self.group1().xyz()))
            .with_w(0.0);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e1234]) * wedge_g1.xyz(),
            // e415, e425, e435, e321
            ((wedge_g1.yz() * other.group3().zx()) - (wedge_g1.zx() * other.group3().yz()))
                .with_zw((wedge_g1[0] * other[e4315]) - (wedge_g1[1] * other[e4235]), wedge_g1[3] * other[e1234]),
            // e235, e315, e125, e4
            (Simd32x3::from(wedge_g1[3]) * other.group3().xyz()).with_w((wedge_g0[3] * other[e1234]) - (wedge_g1[1] * other[e42]) - (wedge_g1[2] * other[e43]))
                - (wedge_g1.xyzx() * Simd32x3::from(other[e3215]).with_w(other[e41])),
            // e1, e2, e3, e5
            (wedge_g1.zxyy() * other.group1().yzx().with_w(other[e25]))
                + (wedge_g1.wwwz() * other.group0().xyz().with_w(other[e35]))
                + (other.group2().wwwx() * wedge_g0.xyz().with_w(wedge_g1[0]))
                + -(wedge_g1.yz() * other.group1().zx()).with_zw(
                    wedge_g1[0] * other[e31] * -1.0,
                    -(wedge_g0[0] * other[e4235]) - (wedge_g0[1] * other[e4315]) - (wedge_g0[2] * other[e4125]) - (wedge_g0[3] * other[e3215]),
                ),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for MultiVector {
    type Output = ProjectViaOriginOntoInfixPartial<MultiVector>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       25       35        0        0
    //    simd2        2        4        0      N/A
    //    simd3        7       12        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd       36       54        0      N/A
    //  no simd       58       91        0        0
    fn project_via_origin_onto(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g0_y = (self[scalar] * other[scalar]) + (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35])
            - (right_dual_g0[0] * self[e15])
            - (right_dual_g0[1] * self[e25])
            - (right_dual_g0[2] * self[e35])
            - (self[e23] * right_dual_g1[0])
            - (self[e31] * right_dual_g1[1])
            - (self[e12] * right_dual_g1[2])
            - (right_dual_g1[3] * self[e45]);
        let wedge_g6 = right_dual_g1 * Simd32x4::from(self[scalar]);
        let wedge_g7 = right_dual_g0 * Simd32x3::from(self[scalar]);
        let wedge_g8 = Simd32x3::from(self[scalar] * -1.0) * other.group2().xyz();
        let wedge_g9 = Simd32x4::from([0.0, 0.0, (right_dual_g1[0] * self[e2]) - (right_dual_g1[1] * self[e1]), 0.0])
            + (((right_dual_g1.yz() * self.group1().zx()) - (right_dual_g1.zx() * self.group1().yz())).with_z(0.0)
                - (right_dual_g0 * Simd32x3::from(self[e5]))
                - (Simd32x3::from(self[e4]) * other.group2().xyz()))
            .with_w(0.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar])
                    - (wedge_g7[0] * other[e15])
                    - (wedge_g7[1] * other[e25])
                    - (wedge_g7[2] * other[e35])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43])
                    - (wedge_g6[0] * other[e23])
                    - (wedge_g6[1] * other[e31])
                    - (wedge_g6[2] * other[e12])
                    - (wedge_g6[3] * other[e45]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, (wedge_g9[1] * other[e23]) - (wedge_g9[0] * other[e31]), 0.0])
                + ((Simd32x3::from(wedge_g9[3]) * other.group0()) + ((wedge_g9.zx() * other.group1().yz()) - (wedge_g9.yz() * other.group1().zx())).with_z(0.0)
                    - (Simd32x3::from(right_dual_g0[0] * self[e1]) * other.group2().xyz())
                    - (Simd32x3::from(right_dual_g0[1] * self[e2]) * other.group2().xyz())
                    - (Simd32x3::from(right_dual_g0[2] * self[e3]) * other.group2().xyz())
                    - (Simd32x3::from(right_dual_g1[3] * self[e4]) * other.group2().xyz()))
                .with_w(0.0),
            // e5
            (wedge_g9[0] * other[e15]) + (wedge_g9[1] * other[e25]) + (wedge_g9[2] * other[e35]) + (wedge_g9[3] * other[e45]),
            // e15, e25, e35, e45
            Simd32x4::from(wedge_g0_y) * other.group2().xyz().with_w(other[e45]),
            // e41, e42, e43
            Simd32x3::from(wedge_g0_y) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(wedge_g0_y) * other.group1().xyz(),
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
impl ProjectViaOriginOnto<AntiDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       40       49        0        0
    //    simd3       23       35        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       63       86        0      N/A
    //  no simd      109      162        0        0
    fn project_via_origin_onto(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g1_w = other[e321] * -1.0;
        let right_dual_g2_xyz = other.group2().xyz();
        let right_dual_g2_w = other[e4] * -1.0;
        let right_dual_g3_xyz = other.group3().xyz();
        let right_dual_g3_w = other[e5] * -1.0;
        let wedge_g0_y =
            (right_dual_g2_w * self[e5]) + (right_dual_g3_w * self[e4]) + (right_dual_g3_xyz[0] * self[e1]) + (right_dual_g3_xyz[1] * self[e2]) + (right_dual_g3_xyz[2] * self[e3])
                - (right_dual_g1_w * self[e321])
                - (right_dual_g1_xyz[0] * self[e415])
                - (right_dual_g1_xyz[1] * self[e425])
                - (right_dual_g1_xyz[2] * self[e435])
                - (right_dual_g2_xyz[0] * self[e423])
                - (right_dual_g2_xyz[1] * self[e431])
                - (right_dual_g2_xyz[2] * self[e412])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125]);
        let wedge_g3 = Simd32x4::from(self[scalar]) * right_dual_g2_xyz.with_w(right_dual_g1_w);
        let wedge_g4 = Simd32x3::from(self[scalar]) * other.group0();
        let wedge_g5 = right_dual_g1_xyz * Simd32x3::from(self[scalar]);
        let wedge_g6_xyz = (right_dual_g2_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(self[e5]) * other.group0()) - (Simd32x3::from(right_dual_g1_w) * self.group1().xyz());
        let wedge_g7 = (right_dual_g1_xyz * Simd32x3::from(self[e4])) + (other.group0().yzx() * self.group1().zxy()) - (other.group0().zxy() * self.group1().yzx());
        let wedge_g8 = (right_dual_g1_xyz * Simd32x3::from(self[e5])) + (right_dual_g2_xyz.zxy() * self.group1().yzx()) - (right_dual_g2_xyz.yzx() * self.group1().zxy());
        let wedge_g9_xyz = (right_dual_g1_xyz * Simd32x3::from(self[e45]))
            + (right_dual_g3_xyz * Simd32x3::from(self[scalar]))
            + (Simd32x3::from(right_dual_g1_w) * self.group5())
            + (right_dual_g2_xyz.zxy() * self.group4().yzx())
            + (other.group0().yzx() * self.group3().zxy())
            - (right_dual_g2_xyz.yzx() * self.group4().zxy())
            - (other.group0().zxy() * self.group3().yzx());
        let wedge_g9_w = right_dual_g3_w * self[scalar];
        let wedge_g10 = (right_dual_g2_w * self[scalar])
            - (right_dual_g1_xyz[0] * self[e41])
            - (right_dual_g1_xyz[1] * self[e42])
            - (right_dual_g1_xyz[2] * self[e43])
            - (other[e423] * self[e23])
            - (other[e431] * self[e31])
            - (other[e412] * self[e12]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g10 * other[e5]) + (wedge_g9_w * other[e4]) + (wedge_g9_xyz[0] * other[e1]) + (wedge_g9_xyz[1] * other[e2]) + (wedge_g9_xyz[2] * other[e3])
                    - (wedge_g4[0] * other[e235])
                    - (wedge_g4[1] * other[e315])
                    - (wedge_g4[2] * other[e125])
                    - (wedge_g5[0] * other[e415])
                    - (wedge_g5[1] * other[e425])
                    - (wedge_g5[2] * other[e435])
                    - (other[e423] * wedge_g3[0])
                    - (other[e431] * wedge_g3[1])
                    - (other[e412] * wedge_g3[2])
                    - (wedge_g3[3] * other[e321]),
                0.0,
            ]),
            // e1, e2, e3, e4
            ((wedge_g6_xyz * Simd32x3::from(other[e321]))
                + (Simd32x3::from(wedge_g0_y) * other.group3().xyz())
                + (wedge_g7.zxy() * other.group2().yzx())
                + (wedge_g8.yzx() * other.group0().zxy())
                - (wedge_g7.yzx() * other.group2().zxy())
                - (wedge_g8.zxy() * other.group0().yzx()))
            .with_w(wedge_g0_y * other[e4]),
            // e5
            (wedge_g0_y * other[e5])
                - (wedge_g6_xyz[0] * other[e235])
                - (wedge_g6_xyz[1] * other[e315])
                - (wedge_g6_xyz[2] * other[e125])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435]),
            // e15, e25, e35, e45
            ((Simd32x3::from(wedge_g9_w) * other.group1().xyz()) + (wedge_g9_xyz.zxy() * other.group2().yzx()) - (wedge_g9_xyz.yzx() * other.group2().zxy())).with_w(0.0),
            // e41, e42, e43
            (Simd32x3::from(wedge_g10) * other.group1().xyz()) + (wedge_g9_xyz.yzx() * other.group0().zxy()) - (wedge_g9_xyz.zxy() * other.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(wedge_g10) * other.group2().xyz()) + (Simd32x3::from(wedge_g9_w) * other.group0()) - (wedge_g9_xyz * Simd32x3::from(other[e321])),
            // e415, e425, e435, e321
            Simd32x4::from(wedge_g0_y) * other.group1(),
            // e423, e431, e412
            Simd32x3::from(wedge_g0_y) * other.group0(),
            // e235, e315, e125
            Simd32x3::from(wedge_g0_y) * other.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl ProjectViaOriginOnto<AntiDualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1       11        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1       14        0      N/A
    //  no simd        1       21        0        0
    fn project_via_origin_onto(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = (other[e3215] * self[e1234]) + (other[scalar] * self[scalar]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([wedge_g0_y * other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e3215] * other[e3215] * self[e4] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from([1.0, 1.0, other[e3215] * other[e3215], 0.0]) * (self.group4() * Simd32x2::from(other[e3215] * other[e3215]).with_z(1.0)).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(other[e3215] * other[e3215] * -1.0) * self.group7(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(wedge_g0_y * other[e3215]),
            // e1234
            0.0,
        )
    }
}
impl ProjectViaOriginOnto<AntiFlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       13        0        0
    //    simd3        4       10        0      N/A
    // Totals...
    // yes simd        9       23        0      N/A
    //  no simd       17       43        0        0
    fn project_via_origin_onto(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e321] * -1.0;
        let wedge_g0_y = -(right_dual_g0_w * self[e321]) - (right_dual_g0_xyz[0] * self[e423]) - (right_dual_g0_xyz[1] * self[e431]) - (right_dual_g0_xyz[2] * self[e412]);
        let wedge_g6_xyz = (right_dual_g0_xyz * Simd32x3::from(self[e4])) - (Simd32x3::from(right_dual_g0_w) * self.group1().xyz());
        let wedge_g9_xyz = (Simd32x3::from(right_dual_g0_w) * self.group5()) + (right_dual_g0_xyz.zxy() * self.group4().yzx()) - (right_dual_g0_xyz.yzx() * self.group4().zxy());
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([right_dual_g0_w * self[scalar] * other[e321] * -1.0, 0.0]),
            // e1, e2, e3, e4
            (wedge_g6_xyz * Simd32x4::from(other[e321]).xyz()).with_w(0.0),
            // e5
            -(wedge_g6_xyz[0] * other[e235]) - (wedge_g6_xyz[1] * other[e315]) - (wedge_g6_xyz[2] * other[e125]),
            // e15, e25, e35, e45
            ((wedge_g9_xyz.zxy() * other.group0().yzx()) - (wedge_g9_xyz.yzx() * other.group0().zxy())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            wedge_g9_xyz * Simd32x3::from(other[e321] * -1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(wedge_g0_y * other[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(wedge_g0_y) * other.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl ProjectViaOriginOnto<AntiFlector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       20        0        0
    //    simd3        6       12        0      N/A
    // Totals...
    // yes simd       19       32        0      N/A
    //  no simd       31       56        0        0
    fn project_via_origin_onto(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e321] * -1.0;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0_y = (right_dual_g1_xyz[0] * self[e1]) + (right_dual_g1_xyz[1] * self[e2]) + (right_dual_g1_xyz[2] * self[e3])
            - (right_dual_g0_w * self[e321])
            - (right_dual_g0_xyz[0] * self[e423])
            - (right_dual_g0_xyz[1] * self[e431])
            - (right_dual_g0_xyz[2] * self[e412])
            - (other[e5] * self[e4]);
        let wedge_g6_xyz = (right_dual_g0_xyz * Simd32x3::from(self[e4])) - (Simd32x3::from(right_dual_g0_w) * self.group1().xyz());
        let wedge_g9_xyz = (right_dual_g1_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_dual_g0_w) * self.group5()) + (right_dual_g0_xyz.zxy() * self.group4().yzx())
            - (right_dual_g0_xyz.yzx() * self.group4().zxy());
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g9_xyz[0] * other[e1]) + (wedge_g9_xyz[1] * other[e2]) + (wedge_g9_xyz[2] * other[e3]) - (right_dual_g0_w * self[scalar] * other[e321]),
                0.0,
            ]),
            // e1, e2, e3, e4
            ((wedge_g6_xyz * Simd32x3::from(other[e321])) + (Simd32x3::from(wedge_g0_y) * other.group1().xyz())).with_w(0.0),
            // e5
            (wedge_g0_y * other[e5]) - (wedge_g6_xyz[0] * other[e235]) - (wedge_g6_xyz[1] * other[e315]) - (wedge_g6_xyz[2] * other[e125]),
            // e15, e25, e35, e45
            ((wedge_g9_xyz.zxy() * other.group0().yzx()) - (wedge_g9_xyz.yzx() * other.group0().zxy())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            wedge_g9_xyz * Simd32x3::from(other[e321] * -1.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(wedge_g0_y * other[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(wedge_g0_y) * other.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl ProjectViaOriginOnto<AntiLine> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       12        0        0
    //    simd3        3       10        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       12       23        0      N/A
    //  no simd       18       46        0        0
    fn project_via_origin_onto(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        let wedge_g0_y = -(right_dual_g0[0] * self[e23])
            - (right_dual_g0[1] * self[e31])
            - (right_dual_g0[2] * self[e12])
            - (right_dual_g1[0] * self[e41])
            - (right_dual_g1[1] * self[e42])
            - (right_dual_g1[2] * self[e43]);
        let wedge_g6 = Simd32x4::from([1.0, 1.0, self[scalar], 0.0]) * (right_dual_g0 * Simd32x2::from(self[scalar]).with_z(1.0)).with_w(0.0);
        let wedge_g9_xyz = (right_dual_g1 * Simd32x3::from(self[e4])) + (right_dual_g0.yzx() * self.group1().zxy()) - (right_dual_g0.zxy() * self.group1().yzx());
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([-(other[e23] * wedge_g6[0]) - (other[e31] * wedge_g6[1]) - (other[e12] * wedge_g6[2]), 0.0]),
            // e1, e2, e3, e4
            ((wedge_g9_xyz.zxy() * other.group0().yzx()) - (wedge_g9_xyz.yzx() * other.group0().zxy())).with_w(0.0),
            // e5
            (wedge_g9_xyz[0] * other[e15]) + (wedge_g9_xyz[1] * other[e25]) + (wedge_g9_xyz[2] * other[e35]),
            // e15, e25, e35, e45
            (Simd32x3::from(wedge_g0_y) * other.group1()).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(wedge_g0_y) * other.group0(),
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
impl ProjectViaOriginOnto<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       16       25        0        0
    //    simd3        5       10        0      N/A
    // Totals...
    // yes simd       21       35        0      N/A
    //  no simd       31       55        0        0
    fn project_via_origin_onto(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = (self[scalar] * other[scalar])
            + (self[e41] * other[e15])
            + (self[e42] * other[e25])
            + (self[e43] * other[e35])
            + (self[e23] * other[e23])
            + (self[e31] * other[e31])
            + (self[e12] * other[e12])
            + (other[e3215] * self[e1234]);
        let wedge_g6_xyz = (Simd32x3::from(other[e3215]) * self.group4()) - (Simd32x3::from(self[scalar]) * other.group0().xyz());
        let wedge_g9_xyz = Simd32x3::from([
            (other[e12] * self[e2]) - (other[e31] * self[e3]),
            (other[e23] * self[e3]) - (other[e12] * self[e1]),
            (other[e31] * self[e1]) - (other[e23] * self[e2]),
        ]) + (Simd32x3::from(other[e3215]) * self.group7())
            - (Simd32x3::from(self[e4]) * other.group1().xyz());
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar]) - (wedge_g6_xyz[0] * other[e23]) - (wedge_g6_xyz[1] * other[e31]) - (wedge_g6_xyz[2] * other[e12]),
                0.0,
            ]),
            // e1, e2, e3, e4
            ((wedge_g9_xyz.zxy() * other.group0().yzx()) - (wedge_g9_xyz.yzx() * other.group0().zxy())).with_w(0.0),
            // e5
            (wedge_g9_xyz[0] * other[e15]) + (wedge_g9_xyz[1] * other[e25]) + (wedge_g9_xyz[2] * other[e35]) - (other[e3215] * other[e3215] * self[e4]),
            // e15, e25, e35, e45
            ((wedge_g6_xyz * Simd32x3::from(other[e3215])) + (Simd32x3::from(wedge_g0_y) * other.group1().xyz())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(wedge_g0_y) * other.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            wedge_g9_xyz * Simd32x3::from(other[e3215] * -1.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(wedge_g0_y * other[e3215]),
            // e1234
            0.0,
        )
    }
}
impl ProjectViaOriginOnto<AntiPlane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        9        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        5       11        0      N/A
    //  no simd        5       16        0        0
    fn project_via_origin_onto(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e5] * -1.0);
        let wedge_g0_y = (right_dual_g0[0] * self[e1]) + (right_dual_g0[1] * self[e2]) + (right_dual_g0[2] * self[e3]) + (right_dual_g0[3] * self[e4]);
        let wedge_g9 = right_dual_g0 * Simd32x4::from(self[scalar]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(wedge_g9[0] * other[e1]) + (wedge_g9[1] * other[e2]) + (wedge_g9[2] * other[e3]), 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(wedge_g0_y) * other.group0().xyz()).with_w(0.0),
            // e5
            wedge_g0_y * other[e5],
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
impl ProjectViaOriginOnto<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       14        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        4        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0       23        0      N/A
    //  no simd        0       44        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e12345] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(right_dual_g0 * other[e12345]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group1(),
            // e5
            right_dual_g0 * other[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group3(),
            // e41, e42, e43
            Simd32x3::from(right_dual_g0 * other[e12345]) * self.group4(),
            // e23, e31, e12
            Simd32x3::from(right_dual_g0 * other[e12345]) * self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group6(),
            // e423, e431, e412
            Simd32x3::from(right_dual_g0 * other[e12345]) * self.group7(),
            // e235, e315, e125
            Simd32x3::from(right_dual_g0 * other[e12345]) * self.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group9(),
            // e1234
            right_dual_g0 * other[e12345] * self[e1234],
        )
    }
}
impl ProjectViaOriginOnto<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       28       33        0        0
    //    simd3       19       31        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       47       66        0      N/A
    //  no simd       85      134        0        0
    fn project_via_origin_onto(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g1_w = other[e321] * -1.0;
        let wedge_g0_y = -(right_dual_g1_w * self[e321])
            - (right_dual_g1_xyz[0] * self[e415])
            - (right_dual_g1_xyz[1] * self[e425])
            - (right_dual_g1_xyz[2] * self[e435])
            - (other[e423] * self[e235])
            - (other[e431] * self[e315])
            - (other[e412] * self[e125])
            - (other[e235] * self[e423])
            - (other[e315] * self[e431])
            - (other[e125] * self[e412]);
        let wedge_g3 = Simd32x4::from(self[scalar]) * other.group2().with_w(right_dual_g1_w);
        let wedge_g4 = Simd32x3::from(self[scalar]) * other.group0();
        let wedge_g5 = right_dual_g1_xyz * Simd32x3::from(self[scalar]);
        let wedge_g6_xyz = (Simd32x3::from(self[e4]) * other.group2()) + (Simd32x3::from(self[e5]) * other.group0()) - (Simd32x3::from(right_dual_g1_w) * self.group1().xyz());
        let wedge_g7 = (right_dual_g1_xyz * Simd32x3::from(self[e4])) + (other.group0().yzx() * self.group1().zxy()) - (other.group0().zxy() * self.group1().yzx());
        let wedge_g8 = (right_dual_g1_xyz * Simd32x3::from(self[e5])) + (other.group2().zxy() * self.group1().yzx()) - (other.group2().yzx() * self.group1().zxy());
        let wedge_g9_xyz = (right_dual_g1_xyz * Simd32x3::from(self[e45]))
            + (Simd32x3::from(right_dual_g1_w) * self.group5())
            + (other.group0().yzx() * self.group3().zxy())
            + (other.group2().zxy() * self.group4().yzx())
            - (other.group0().zxy() * self.group3().yzx())
            - (other.group2().yzx() * self.group4().zxy());
        let wedge_g10 = -(right_dual_g1_xyz[0] * self[e41])
            - (right_dual_g1_xyz[1] * self[e42])
            - (right_dual_g1_xyz[2] * self[e43])
            - (other[e423] * self[e23])
            - (other[e431] * self[e31])
            - (other[e412] * self[e12]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(wedge_g4[0] * other[e235])
                    - (wedge_g4[1] * other[e315])
                    - (wedge_g4[2] * other[e125])
                    - (wedge_g5[0] * other[e415])
                    - (wedge_g5[1] * other[e425])
                    - (wedge_g5[2] * other[e435])
                    - (other[e423] * wedge_g3[0])
                    - (other[e431] * wedge_g3[1])
                    - (other[e412] * wedge_g3[2])
                    - (wedge_g3[3] * other[e321]),
                0.0,
            ]),
            // e1, e2, e3, e4
            ((wedge_g6_xyz * Simd32x3::from(other[e321])) + (wedge_g7.zxy() * other.group2().yzx()) + (wedge_g8.yzx() * other.group0().zxy())
                - (wedge_g7.yzx() * other.group2().zxy())
                - (wedge_g8.zxy() * other.group0().yzx()))
            .with_w(0.0),
            // e5
            -(wedge_g6_xyz[0] * other[e235])
                - (wedge_g6_xyz[1] * other[e315])
                - (wedge_g6_xyz[2] * other[e125])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435]),
            // e15, e25, e35, e45
            ((wedge_g9_xyz.zxy() * other.group2().yzx()) - (wedge_g9_xyz.yzx() * other.group2().zxy())).with_w(0.0),
            // e41, e42, e43
            (Simd32x3::from(wedge_g10) * other.group1().xyz()) + (wedge_g9_xyz.yzx() * other.group0().zxy()) - (wedge_g9_xyz.zxy() * other.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(wedge_g10) * other.group2()) - (wedge_g9_xyz * Simd32x3::from(other[e321])),
            // e415, e425, e435, e321
            Simd32x4::from(wedge_g0_y) * other.group1(),
            // e423, e431, e412
            Simd32x3::from(wedge_g0_y) * other.group0(),
            // e235, e315, e125
            Simd32x3::from(wedge_g0_y) * other.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl ProjectViaOriginOnto<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       36       54        0        0
    //    simd3       34       46        0      N/A
    //    simd4        1        4        0      N/A
    // Totals...
    // yes simd       71      104        0      N/A
    //  no simd      142      208        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g1_w = other[e321] * -1.0;
        let right_dual_g2_xyz = other.group2().xyz();
        let right_dual_g2_w = other[e12345] * -1.0;
        let wedge_g0_y = (right_dual_g2_w * self[e12345])
            - (right_dual_g1_w * self[e321])
            - (right_dual_g1_xyz[0] * self[e415])
            - (right_dual_g1_xyz[1] * self[e425])
            - (right_dual_g1_xyz[2] * self[e435])
            - (right_dual_g2_xyz[0] * self[e423])
            - (right_dual_g2_xyz[1] * self[e431])
            - (right_dual_g2_xyz[2] * self[e412])
            - (other[e423] * self[e235])
            - (other[e431] * self[e315])
            - (other[e412] * self[e125]);
        let wedge_g1 = Simd32x4::from(right_dual_g2_w) * self.group1();
        let wedge_g3_xyz = (right_dual_g2_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_dual_g2_w) * self.group3().xyz());
        let wedge_g3_w = (right_dual_g1_w * self[scalar]) + (right_dual_g2_w * self[e45]);
        let wedge_g4 = (Simd32x3::from(right_dual_g2_w) * self.group4()) + (Simd32x3::from(self[scalar]) * other.group0());
        let wedge_g5 = (right_dual_g1_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_dual_g2_w) * self.group5());
        let wedge_g6 = ((right_dual_g2_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_dual_g2_w) * self.group6().xyz()) + (Simd32x3::from(self[e5]) * other.group0())
            - (Simd32x3::from(right_dual_g1_w) * self.group1().xyz()))
        .with_w(right_dual_g2_w * self[e321]);
        let wedge_g7 = (right_dual_g1_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_dual_g2_w) * self.group7()) + (other.group0().yzx() * self.group1().zxy())
            - (other.group0().zxy() * self.group1().yzx());
        let wedge_g8 = (right_dual_g1_xyz * Simd32x3::from(self[e5])) + (Simd32x3::from(right_dual_g2_w) * self.group8()) + (right_dual_g2_xyz.zxy() * self.group1().yzx())
            - (right_dual_g2_xyz.yzx() * self.group1().zxy());
        let wedge_g9 = ((right_dual_g1_xyz * Simd32x3::from(self[e45]))
            + (Simd32x3::from(right_dual_g1_w) * self.group5())
            + (Simd32x3::from(right_dual_g2_w) * self.group9().xyz())
            + (right_dual_g2_xyz.zxy() * self.group4().yzx())
            + (other.group0().yzx() * self.group3().zxy())
            - (right_dual_g2_xyz.yzx() * self.group4().zxy())
            - (other.group0().zxy() * self.group3().yzx()))
        .with_w(right_dual_g2_w * self[e3215]);
        let wedge_g10 = (right_dual_g2_w * self[e1234])
            - (right_dual_g1_xyz[0] * self[e41])
            - (right_dual_g1_xyz[1] * self[e42])
            - (right_dual_g1_xyz[2] * self[e43])
            - (other[e423] * self[e23])
            - (other[e431] * self[e31])
            - (other[e412] * self[e12]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual_g2_w * self[scalar] * other[e12345])
                    - (wedge_g3_w * other[e321])
                    - (wedge_g3_xyz[0] * other[e423])
                    - (wedge_g3_xyz[1] * other[e431])
                    - (wedge_g3_xyz[2] * other[e412])
                    - (wedge_g4[0] * other[e235])
                    - (wedge_g4[1] * other[e315])
                    - (wedge_g4[2] * other[e125])
                    - (wedge_g5[0] * other[e415])
                    - (wedge_g5[1] * other[e425])
                    - (wedge_g5[2] * other[e435]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g6[3]) * other.group1().xyz())
                + (Simd32x3::from(other[e321]) * wedge_g6.xyz())
                + (Simd32x3::from(other[e12345]) * wedge_g1.xyz())
                + (wedge_g7.zxy() * other.group2().yzx())
                + (wedge_g8.yzx() * other.group0().zxy())
                - (wedge_g7.yzx() * other.group2().zxy())
                - (wedge_g8.zxy() * other.group0().yzx()))
            .with_w(wedge_g1[3] * other[e12345]),
            // e5
            (right_dual_g2_w * other[e12345] * self[e5])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435])
                - (wedge_g6[0] * other[e235])
                - (wedge_g6[1] * other[e315])
                - (wedge_g6[2] * other[e125]),
            // e15, e25, e35, e45
            (Simd32x3::from([
                (wedge_g9[2] * other[e315]) - (wedge_g9[1] * other[e125]),
                (wedge_g9[0] * other[e125]) - (wedge_g9[2] * other[e235]),
                (wedge_g9[1] * other[e235]) - (wedge_g9[0] * other[e315]),
            ]) + (wedge_g3_xyz * Simd32x3::from(other[e12345]))
                + (Simd32x3::from(wedge_g9[3]) * other.group1().xyz()))
            .with_w(wedge_g3_w * other[e12345]),
            // e41, e42, e43
            (wedge_g4 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g10) * other.group1().xyz()) + (other.group0().zxy() * wedge_g9.yzx())
                - (other.group0().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (wedge_g5 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g10) * other.group2().xyz()) + (Simd32x3::from(wedge_g9[3]) * other.group0())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            (wedge_g6 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group1()),
            // e423, e431, e412
            (wedge_g7 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group0()),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group2().xyz()),
            // e4235, e4315, e4125, e3215
            wedge_g9 * Simd32x4::from(other[e12345]),
            // e1234
            wedge_g10 * other[e12345],
        )
    }
}
impl ProjectViaOriginOnto<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       23       32        0        0
    //    simd2        2        4        0      N/A
    //    simd3        7       13        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd       34       52        0      N/A
    //  no simd       56       91        0        0
    fn project_via_origin_onto(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        let wedge_g0_y = -(right_dual_g0[0] * self[e15])
            - (right_dual_g0[1] * self[e25])
            - (right_dual_g0[2] * self[e35])
            - (right_dual_g2[0] * self[e41])
            - (right_dual_g2[1] * self[e42])
            - (right_dual_g2[2] * self[e43])
            - (self[e23] * right_dual_g1[0])
            - (self[e31] * right_dual_g1[1])
            - (self[e12] * right_dual_g1[2])
            - (right_dual_g1[3] * self[e45]);
        let wedge_g6 = right_dual_g1 * Simd32x4::from(self[scalar]);
        let wedge_g7 = right_dual_g0 * Simd32x3::from(self[scalar]);
        let wedge_g8 = right_dual_g2 * Simd32x3::from(self[scalar]);
        let wedge_g9 = Simd32x4::from([0.0, 0.0, (right_dual_g1[0] * self[e2]) - (right_dual_g1[1] * self[e1]), 0.0])
            + ((right_dual_g2 * Simd32x3::from(self[e4])) + ((right_dual_g1.yz() * self.group1().zx()) - (right_dual_g1.zx() * self.group1().yz())).with_z(0.0)
                - (right_dual_g0 * Simd32x3::from(self[e5])))
            .with_w(0.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(wedge_g7[0] * other[e15])
                    - (wedge_g7[1] * other[e25])
                    - (wedge_g7[2] * other[e35])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43])
                    - (wedge_g6[0] * other[e23])
                    - (wedge_g6[1] * other[e31])
                    - (wedge_g6[2] * other[e12])
                    - (wedge_g6[3] * other[e45]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, (wedge_g9[1] * other[e23]) - (wedge_g9[0] * other[e31]), 0.0])
                + ((Simd32x3::from(wedge_g9[3]) * other.group0()) + ((wedge_g9.zx() * other.group1().yz()) - (wedge_g9.yz() * other.group1().zx())).with_z(0.0)
                    - (Simd32x3::from(right_dual_g0[0] * self[e1]) * other.group2())
                    - (Simd32x3::from(right_dual_g0[1] * self[e2]) * other.group2())
                    - (Simd32x3::from(right_dual_g0[2] * self[e3]) * other.group2())
                    - (Simd32x3::from(right_dual_g1[3] * self[e4]) * other.group2()))
                .with_w(0.0),
            // e5
            (other[e15] * wedge_g9[0]) + (other[e25] * wedge_g9[1]) + (other[e35] * wedge_g9[2]) + (wedge_g9[3] * other[e45]),
            // e15, e25, e35, e45
            Simd32x4::from(wedge_g0_y) * other.group2().with_w(other[e45]),
            // e41, e42, e43
            Simd32x3::from(wedge_g0_y) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(wedge_g0_y) * other.group1().xyz(),
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
impl ProjectViaOriginOnto<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       59       81        0        0
    //    simd2        2        5        0      N/A
    //    simd3       28       39        0      N/A
    //    simd4        6        8        0      N/A
    // Totals...
    // yes simd       95      133        0      N/A
    //  no simd      171      240        0        0
    fn project_via_origin_onto(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0_y = (self[e41] * other[e15])
            + (self[e42] * other[e25])
            + (self[e43] * other[e35])
            + (self[e23] * other[e23])
            + (self[e31] * other[e31])
            + (self[e12] * other[e12])
            + (other[e1234] * self[e3215])
            + (other[e3215] * self[e1234])
            - (right_dual_g0[0] * self[e15])
            - (right_dual_g0[1] * self[e25])
            - (right_dual_g0[2] * self[e35])
            - (other[e45] * self[e45])
            - (other[e4235] * self[e4235])
            - (other[e4315] * self[e4315])
            - (other[e4125] * self[e4125]);
        let wedge_g1 = Simd32x4::from(self[scalar]) * (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let wedge_g3 = (other.group3() * Simd32x3::from(self[e5]).with_w(self[e4])) + (Simd32x3::from(other[e3215]) * self.group1().xyz()).with_w(0.0);
        let wedge_g4 = -(Simd32x3::from(other[e1234]) * self.group1().xyz()) - (Simd32x3::from(self[e4]) * other.group3().xyz());
        let wedge_g5 = Simd32x3::from([
            (other[e4315] * self[e3]) - (other[e4125] * self[e2]),
            (other[e4125] * self[e1]) - (other[e4235] * self[e3]),
            (other[e4235] * self[e2]) - (other[e4315] * self[e1]),
        ]);
        let wedge_g6_xyz =
            (Simd32x3::from(other[e1234]) * self.group3().xyz()) + (Simd32x3::from(other[e3215]) * self.group4()) + (Simd32x3::from(self[e45]) * other.group3().xyz())
                - (Simd32x3::from(self[scalar]) * other.group1().xyz());
        let wedge_g6_w = self[scalar] * other[e45];
        let wedge_g7 = (right_dual_g0 * Simd32x3::from(self[scalar])) + (Simd32x3::from(other[e1234]) * self.group5()) + (self.group4().zxy() * other.group3().yzx())
            - (self.group4().yzx() * other.group3().zxy());
        let wedge_g8 = Simd32x3::from([
            (other[e4125] * self[e25]) - (other[e4315] * self[e35]),
            (other[e4235] * self[e35]) - (other[e4125] * self[e15]),
            (other[e4315] * self[e15]) - (other[e4235] * self[e25]),
        ]) + (Simd32x3::from(other[e3215]) * self.group5())
            - (Simd32x3::from(self[scalar]) * other.group2().xyz());
        let wedge_g9 = (self.group1().yzxy() * other.group1().zxy().with_w(other[e25]))
            + ((Simd32x3::from(other[e3215]) * self.group7())
                + ((other.group3().zx() * self.group6().yz()) - (other.group1().yz() * self.group1().zx()) - (other.group3().yz() * self.group6().zx()))
                    .with_z((other[e4315] * self[e415]) - (other[e23] * self[e2]) - (other[e4235] * self[e425]))
                - (right_dual_g0 * Simd32x3::from(self[e5]))
                - (Simd32x3::from(other[e1234]) * self.group8())
                - (Simd32x3::from(self[e4]) * other.group2().xyz()))
            .with_w(
                (other[e15] * self[e1]) + (other[e35] * self[e3])
                    - (self[e235] * other[e4235])
                    - (self[e315] * other[e4315])
                    - (self[e125] * other[e4125])
                    - (other[e45] * self[e5]),
            );
        let wedge_g10 = (right_dual_g0[0] * self[e1])
            + (right_dual_g0[1] * self[e2])
            + (right_dual_g0[2] * self[e3])
            + (self[e423] * other[e4235])
            + (self[e431] * other[e4315])
            + (self[e412] * other[e4125])
            + (other[e45] * self[e4])
            - (other[e1234] * self[e321]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g1[0] * other[e4235])
                    + (wedge_g1[1] * other[e4315])
                    + (wedge_g1[2] * other[e4125])
                    + (wedge_g1[3] * other[e3215])
                    + (self[scalar] * other[e1234] * other[e3215])
                    - (wedge_g6_w * other[e45])
                    - (wedge_g6_xyz[0] * other[e23])
                    - (wedge_g6_xyz[1] * other[e31])
                    - (wedge_g6_xyz[2] * other[e12])
                    - (wedge_g7[0] * other[e15])
                    - (wedge_g7[1] * other[e25])
                    - (wedge_g7[2] * other[e35])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (other.group3().yzxx() * wedge_g5.zxy().with_w(wedge_g4[0]))
                + ((Simd32x3::from(wedge_g9[3]) * other.group0())
                    + (Simd32x3::from(other[e1234]) * wedge_g3.xyz())
                    + (wedge_g9.zx() * other.group1().yz()).with_z(wedge_g9[1] * other[e23])
                    - (wedge_g4 * Simd32x3::from(other[e3215]))
                    - (wedge_g5.yzx() * other.group3().zxy()))
                .with_w((wedge_g4[1] * other[e4315]) + (wedge_g4[2] * other[e4125]) + (wedge_g3[3] * other[e1234]) - (other[e41] * wedge_g9[0]) - (other[e42] * wedge_g9[1]))
                - (Simd32x4::from(wedge_g10) * other.group2().xyz().with_w(other[e45]))
                - (wedge_g9.yzxz() * other.group1().zxy().with_w(other[e43])),
            // e5
            (wedge_g9[0] * other[e15]) + (wedge_g9[1] * other[e25]) + (wedge_g9[2] * other[e35]) + (wedge_g9[3] * other[e45])
                - (wedge_g3[0] * other[e4235])
                - (wedge_g3[1] * other[e4315])
                - (wedge_g3[2] * other[e4125])
                - (wedge_g3[3] * other[e3215]),
            // e15, e25, e35, e45
            ((wedge_g6_xyz * Simd32x3::from(other[e3215])) + (Simd32x3::from(wedge_g0_y) * other.group2().xyz()) + (wedge_g8.yzx() * other.group3().zxy())
                - (wedge_g8.zxy() * other.group3().yzx()))
            .with_w(wedge_g0_y * other[e45]),
            // e41, e42, e43
            (wedge_g6_xyz * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0_y) * other.group0()) + (wedge_g7.zxy() * other.group3().yzx())
                - (wedge_g7.yzx() * other.group3().zxy()),
            // e23, e31, e12
            (wedge_g7 * Simd32x3::from(other[e3215])) + (wedge_g8 * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0_y) * other.group1().xyz())
                - (Simd32x3::from(wedge_g6_w) * other.group3().xyz()),
            // e415, e425, e435, e321
            (wedge_g9.yzxw() * other.group3().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group3().yz()).with_zw(wedge_g9[1] * other[e4235] * -1.0, wedge_g10 * other[e3215] * -1.0),
            // e423, e431, e412
            (Simd32x3::from(other[e1234]) * wedge_g9.xyz()) - (Simd32x3::from(wedge_g10) * other.group3().xyz()),
            // e235, e315, e125
            (Simd32x3::from(wedge_g9[3]) * other.group3().xyz()) - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0_y) * other.group3(),
            // e1234
            wedge_g0_y * other[e1234],
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       33        0        0
    //    simd3        0        6        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        4       42        0      N/A
    //  no simd        4       63        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = -(other[e5] * self[e4]) - (other[e12345] * self[e12345]);
        let wedge_g10 = other[e12345] * self[e1234] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(wedge_g10 * other[e5]) - (other[e12345] * other[e12345] * self[scalar]), wedge_g0_y * other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group1(),
            // e5
            (wedge_g0_y * other[e5]) - (other[e12345] * other[e12345] * self[e5]),
            // e15, e25, e35, e45
            Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group3(),
            // e41, e42, e43
            Simd32x3::from(other[e12345] * other[e12345] * -1.0) * self.group4(),
            // e23, e31, e12
            Simd32x3::from(other[e12345] * other[e12345] * -1.0) * self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group6(),
            // e423, e431, e412
            Simd32x3::from(other[e12345] * other[e12345] * -1.0) * self.group7(),
            // e235, e315, e125
            Simd32x3::from(other[e12345] * other[e12345] * -1.0) * self.group8(),
            // e4235, e4315, e4125, e3215
            (self.group9().xyz() * Simd32x2::from(other[e12345] * other[e12345] * -1.0).with_z(other[e12345] * other[e12345]) * Simd32x3::from([1.0, 1.0, -1.0]))
                .with_w(-(other[e12345] * other[e12345] * self[e3215]) - (other[e5] * other[e12345] * self[scalar])),
            // e1234
            wedge_g10 * other[e12345],
        )
    }
}
impl ProjectViaOriginOnto<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       22        0        0
    //    simd3        0        1        0      N/A
    //    simd4        3        7        0      N/A
    // Totals...
    // yes simd        9       30        0      N/A
    //  no simd       18       53        0        0
    fn project_via_origin_onto(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let wedge_g9_xyz = Simd32x3::from(self[e4] * -1.0) * other.group0().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[e45] * other[e45] * self[scalar] * -1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(other[e45] * self[e4] * -1.0) * other.group0(),
            // e5
            (wedge_g9_xyz[0] * other[e15])
                + (wedge_g9_xyz[1] * other[e25])
                + (wedge_g9_xyz[2] * other[e35])
                + (other[e15] * other[e45] * self[e1])
                + (other[e25] * other[e45] * self[e2])
                + (other[e35] * other[e45] * self[e3])
                - (other[e45] * other[e45] * self[e5]),
            // e15, e25, e35, e45
            (Simd32x4::from([self[e41] * other[e15], self[e42] * other[e25], self[e43] * other[e35], self[e41] * other[e45]]) * other.group0().xyzx())
                + (self.group4().yxxy() * other.group0().xxxy() * other.group0().yyzw())
                + (self.group4().zzyz() * other.group0().xyyz() * other.group0().zzzw())
                - (Simd32x4::from(other[e45] * self[e45]) * other.group0()),
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
impl ProjectViaOriginOnto<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       37       58        0        0
    //    simd2        1        3        0      N/A
    //    simd3       13       25        0      N/A
    //    simd4        4        6        0      N/A
    // Totals...
    // yes simd       55       92        0      N/A
    //  no simd       94      163        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]) + (other[e3215] * self[e1234])
            - (other[e45] * self[e45])
            - (other[e4235] * self[e4235])
            - (other[e4315] * self[e4315])
            - (other[e4125] * self[e4125]);
        let wedge_g1 =
            Simd32x4::from([1.0, 1.0, self[scalar], 0.0]) * (other.group1().xyz() * Simd32x2::from(self[scalar] * -1.0).with_z(1.0) * Simd32x3::from([1.0, 1.0, -1.0])).with_w(0.0);
        let wedge_g3_xyz = (Simd32x3::from(other[e3215]) * self.group1().xyz()) + (Simd32x3::from(self[e5]) * other.group1().xyz());
        let wedge_g4 = Simd32x3::from(self[e4] * -1.0) * other.group1().xyz();
        let wedge_g5 = Simd32x3::from([
            (other[e4315] * self[e3]) - (other[e4125] * self[e2]),
            (other[e4125] * self[e1]) - (other[e4235] * self[e3]),
            (other[e4235] * self[e2]) - (other[e4315] * self[e1]),
        ]);
        let wedge_g6 = (other.group1().xyzx() * Simd32x3::from(self[e45]).with_w(self[e23]))
            + (Simd32x3::from(other[e3215]) * self.group4()).with_w((self[scalar] * other[e45]) + (self[e31] * other[e4315]) + (self[e12] * other[e4125]));
        let wedge_g7 = (self.group4().zxy() * other.group1().yzx()) - (self.group4().yzx() * other.group1().zxy());
        let wedge_g8 = Simd32x3::from([
            (other[e4125] * self[e25]) - (other[e4315] * self[e35]),
            (other[e4235] * self[e35]) - (other[e4125] * self[e15]),
            (other[e4315] * self[e15]) - (other[e4235] * self[e25]),
        ]) + (Simd32x3::from(other[e3215]) * self.group5())
            - (Simd32x3::from(self[scalar]) * other.group0().xyz());
        let wedge_g9 = ((Simd32x3::from(other[e3215]) * self.group7()) + (other.group1().zx() * self.group6().yz()).with_z(other[e4315] * self[e415])
            - (Simd32x3::from(self[e4]) * other.group0().xyz()))
        .with_w((other[e15] * self[e1]) + (other[e25] * self[e2]) - (self[e235] * other[e4235]) - (self[e125] * other[e4125]) - (other[e45] * self[e5]))
            - (other.group1().yzxy() * self.group6().zxy().with_w(self[e315]));
        let wedge_g10 = (self[e423] * other[e4235]) + (self[e431] * other[e4315]) + (self[e412] * other[e4125]) + (other[e45] * self[e4]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g1[0] * other[e4235]) + (wedge_g1[1] * other[e4315]) + (wedge_g1[2] * other[e4125]) + (wedge_g1[3] * other[e3215])
                    - (wedge_g7[0] * other[e15])
                    - (wedge_g7[1] * other[e25])
                    - (wedge_g7[2] * other[e35])
                    - (wedge_g6[3] * other[e45]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (other.group1().yzxx() * wedge_g5.zxy().with_w(wedge_g4[0])) + (-(wedge_g4 * Simd32x3::from(other[e3215])) - (wedge_g5.yzx() * other.group1().zxy())).with_w(0.0)
                - (Simd32x4::from(wedge_g10) * other.group0()),
            // e5
            (wedge_g9[0] * other[e15]) + (wedge_g9[1] * other[e25]) + (wedge_g9[2] * other[e35]) + (wedge_g9[3] * other[e45])
                - (wedge_g3_xyz[0] * other[e4235])
                - (wedge_g3_xyz[1] * other[e4315])
                - (wedge_g3_xyz[2] * other[e4125])
                - (other[e3215] * other[e3215] * self[e4]),
            // e15, e25, e35, e45
            ((Simd32x3::from(wedge_g0_y) * other.group0().xyz()) + (Simd32x3::from(other[e3215]) * wedge_g6.xyz()) + (wedge_g8.yzx() * other.group1().zxy())
                - (wedge_g8.zxy() * other.group1().yzx()))
            .with_w(wedge_g0_y * other[e45]),
            // e41, e42, e43
            (wedge_g7.zxy() * other.group1().yzx()) - (wedge_g7.yzx() * other.group1().zxy()),
            // e23, e31, e12
            (wedge_g7 * Simd32x3::from(other[e3215])) - (Simd32x3::from(wedge_g6[3]) * other.group1().xyz()),
            // e415, e425, e435, e321
            ((wedge_g9.yz() * other.group1().zx()) - (wedge_g9.zx() * other.group1().yz()))
                .with_zw((wedge_g9[0] * other[e4315]) - (wedge_g9[1] * other[e4235]), wedge_g10 * other[e3215] * -1.0),
            // e423, e431, e412
            Simd32x3::from(wedge_g10 * -1.0) * other.group1().xyz(),
            // e235, e315, e125
            (Simd32x3::from(wedge_g9[3]) * other.group1().xyz()) - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0_y) * other.group1(),
            // e1234
            0.0,
        )
    }
}
impl ProjectViaOriginOnto<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       14       18        0        0
    //    simd3        9       23        0      N/A
    // Totals...
    // yes simd       23       41        0      N/A
    //  no simd       41       87        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = -(other[e415] * self[e415])
            - (other[e425] * self[e425])
            - (other[e435] * self[e435])
            - (other[e235] * self[e423])
            - (other[e315] * self[e431])
            - (other[e125] * self[e412]);
        let wedge_g5 = Simd32x3::from(self[scalar]) * other.group0();
        let wedge_g6_xyz = Simd32x3::from(self[e4]) * other.group1();
        let wedge_g7 = Simd32x3::from(self[e4]) * other.group0();
        let wedge_g8 = (Simd32x3::from(self[e5]) * other.group0()) + (other.group1().zxy() * self.group1().yzx()) - (other.group1().yzx() * self.group1().zxy());
        let wedge_g9_xyz = (Simd32x3::from(self[e45]) * other.group0()) + (other.group1().zxy() * self.group4().yzx()) - (other.group1().yzx() * self.group4().zxy());
        let wedge_g10 = -(other[e415] * self[e41]) - (other[e425] * self[e42]) - (other[e435] * self[e43]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([-(wedge_g5[0] * other[e415]) - (wedge_g5[1] * other[e425]) - (wedge_g5[2] * other[e435]), 0.0]),
            // e1, e2, e3, e4
            ((wedge_g7.zxy() * other.group1().yzx())
                - (wedge_g7.yzx() * other.group1().zxy())
                - (other.group0() * other.group0() * self.group1().xyz())
                - (Simd32x3::from(other[e415]) * other.group0().yyz() * self.group1().yxx())
                - (Simd32x3::from(other[e435]) * other.group0().xyy() * self.group1().zzy()))
            .with_w(0.0),
            // e5
            -(wedge_g6_xyz[0] * other[e235])
                - (wedge_g6_xyz[1] * other[e315])
                - (wedge_g6_xyz[2] * other[e125])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435]),
            // e15, e25, e35, e45
            ((wedge_g9_xyz.zxy() * other.group1().yzx()) - (wedge_g9_xyz.yzx() * other.group1().zxy())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(wedge_g10) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(wedge_g10) * other.group1(),
            // e415, e425, e435, e321
            (Simd32x3::from(wedge_g0_y) * other.group0()).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(wedge_g0_y) * other.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl ProjectViaOriginOnto<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       27       48        0        0
    //    simd3       18       30        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd       46       81        0      N/A
    //  no simd       85      150        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e12345] * -1.0);
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g1_w = other[e5] * -1.0;
        let wedge_g0_y = (right_dual_g1_w * self[e4]) + (self[e12345] * right_dual_g0[3])
            - (right_dual_g1_xyz[0] * self[e423])
            - (right_dual_g1_xyz[1] * self[e431])
            - (right_dual_g1_xyz[2] * self[e412])
            - (right_dual_g0[0] * self[e415])
            - (right_dual_g0[1] * self[e425])
            - (right_dual_g0[2] * self[e435]);
        let wedge_g1 = Simd32x4::from(right_dual_g0[3]) * self.group1();
        let wedge_g4 = Simd32x3::from(right_dual_g0[3]) * self.group4();
        let wedge_g5 = (Simd32x3::from(self[scalar]) * right_dual_g0.xyz()) + (Simd32x3::from(right_dual_g0[3]) * self.group5());
        let wedge_g6_xyz = (right_dual_g1_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_dual_g0[3]) * self.group6().xyz());
        let wedge_g6_w = right_dual_g0[3] * self[e321];
        let wedge_g7 = (Simd32x3::from(right_dual_g0[3]) * self.group7()) + (Simd32x3::from(self[e4]) * right_dual_g0.xyz());
        let wedge_g8 = (Simd32x3::from(right_dual_g0[3]) * self.group8()) + (Simd32x3::from(self[e5]) * right_dual_g0.xyz()) + (right_dual_g1_xyz.zxy() * self.group1().yzx())
            - (right_dual_g1_xyz.yzx() * self.group1().zxy());
        let wedge_g9 = (right_dual_g0 * Simd32x3::from(self[e45]).with_w(self[e3215]))
            + ((Simd32x3::from(right_dual_g0[3]) * self.group9().xyz()) + (right_dual_g1_xyz.zxy() * self.group4().yzx()) - (right_dual_g1_xyz.yzx() * self.group4().zxy()))
                .with_w(right_dual_g1_w * self[scalar]);
        let wedge_g10 = (right_dual_g0[3] * self[e1234]) - (self[e41] * right_dual_g0[0]) - (self[e42] * right_dual_g0[1]) - (self[e43] * right_dual_g0[2]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g10 * other[e5]) + (self[scalar] * right_dual_g0[3] * other[e12345])
                    - (wedge_g4[0] * other[e235])
                    - (wedge_g4[1] * other[e315])
                    - (wedge_g4[2] * other[e125])
                    - (wedge_g5[0] * other[e415])
                    - (wedge_g5[1] * other[e425])
                    - (wedge_g5[2] * other[e435]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g6_w) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g1.xyz()) + (wedge_g7.zxy() * other.group1().yzx())
                - (wedge_g7.yzx() * other.group1().zxy()))
            .with_w(wedge_g1[3] * other[e12345]),
            // e5
            (wedge_g0_y * other[e5]) + (right_dual_g0[3] * other[e12345] * self[e5])
                - (wedge_g6_xyz[0] * other[e235])
                - (wedge_g6_xyz[1] * other[e315])
                - (wedge_g6_xyz[2] * other[e125])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435]),
            // e15, e25, e35, e45
            (Simd32x3::from([
                (wedge_g9[2] * other[e315]) - (wedge_g9[1] * other[e125]),
                (wedge_g9[0] * other[e125]) - (wedge_g9[2] * other[e235]),
                (wedge_g9[1] * other[e235]) - (wedge_g9[0] * other[e315]),
            ]) + (right_dual_g1_xyz * Simd32x3::from(self[scalar] * other[e12345]))
                + (Simd32x3::from(wedge_g9[3]) * other.group0().xyz())
                + (Simd32x3::from(right_dual_g0[3] * other[e12345]) * self.group3().xyz()))
            .with_w(right_dual_g0[3] * other[e12345] * self[e45]),
            // e41, e42, e43
            (wedge_g4 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g10) * other.group0().xyz()),
            // e23, e31, e12
            (wedge_g5 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g10) * other.group1().xyz()),
            // e415, e425, e435, e321
            ((wedge_g6_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group0().xyz())).with_w(wedge_g6_w * other[e12345]),
            // e423, e431, e412
            wedge_g7 * Simd32x3::from(other[e12345]),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group1().xyz()),
            // e4235, e4315, e4125, e3215
            wedge_g9 * Simd32x4::from(other[e12345]),
            // e1234
            wedge_g10 * other[e12345],
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32      114      138        0        0
    //    simd2        3        6        0      N/A
    //    simd3       68       81        0      N/A
    //    simd4       17       20        0      N/A
    // Totals...
    // yes simd      202      245        0      N/A
    //  no simd      392      473        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1 = (other.group9().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let right_dual_g3 = other.group8().with_w(other[e321] * -1.0);
        let right_dual_g5 = other.group6().xyz();
        let right_dual_g6 = (other.group5() * Simd32x3::from(-1.0)).with_w(other[e45]);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_dual_g9 = other.group1().xyz().with_w(other[e5] * -1.0);
        let right_dual_g10 = other[e4] * -1.0;
        let wedge_g0_y = (right_dual_g10 * self[e5])
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
            - (right_dual_g5[0] * self[e415])
            - (right_dual_g5[1] * self[e425])
            - (right_dual_g5[2] * self[e435])
            - (right_dual_g7[0] * self[e15])
            - (right_dual_g7[1] * self[e25])
            - (right_dual_g7[2] * self[e35])
            - (right_dual_g8[0] * self[e41])
            - (right_dual_g8[1] * self[e42])
            - (right_dual_g8[2] * self[e43])
            - (other[e423] * self[e235])
            - (other[e431] * self[e315])
            - (other[e412] * self[e125])
            - (self[e23] * right_dual_g6[0])
            - (self[e31] * right_dual_g6[1])
            - (self[e12] * right_dual_g6[2])
            - (self[e423] * right_dual_g3[0])
            - (self[e431] * right_dual_g3[1])
            - (self[e412] * right_dual_g3[2])
            - (right_dual_g3[3] * self[e321])
            - (right_dual_g6[3] * self[e45]);
        let wedge_g1 = (right_dual_g1 * Simd32x4::from(self[scalar])) + (Simd32x4::from(right_dual_g0[0]) * self.group1());
        let wedge_g2 = (right_dual_g0[0] * self[e5]) + (self[scalar] * other[e3215]);
        let wedge_g3 = (right_dual_g3 * Simd32x4::from(self[scalar])) + (Simd32x4::from(right_dual_g0[0]) * self.group3()) + (Simd32x4::from(other[e3215]) * self.group1())
            - (right_dual_g1 * Simd32x4::from(self[e5]));
        let wedge_g4 = (Simd32x3::from(right_dual_g0[0]) * self.group4()) + (Simd32x3::from(self[scalar]) * other.group7()) + (Simd32x3::from(self[e4]) * right_dual_g1.xyz())
            - (Simd32x3::from(right_dual_g1[3]) * self.group1().xyz());
        let wedge_g5 = Simd32x3::from([
            (right_dual_g1[2] * self[e2]) - (right_dual_g1[1] * self[e3]),
            (right_dual_g1[0] * self[e3]) - (right_dual_g1[2] * self[e1]),
            (right_dual_g1[1] * self[e1]) - (right_dual_g1[0] * self[e2]),
        ]) + (right_dual_g5 * Simd32x3::from(self[scalar]))
            + (Simd32x3::from(right_dual_g0[0]) * self.group5());
        let wedge_g6 = (right_dual_g6 * Simd32x4::from(self[scalar]))
            + (Simd32x4::from(right_dual_g0[0]) * self.group6())
            + ((Simd32x3::from(right_dual_g1[3]) * self.group3().xyz())
                + (Simd32x3::from(other[e3215]) * self.group4())
                + (Simd32x3::from(self[e4]) * right_dual_g3.xyz())
                + (Simd32x3::from(self[e5]) * other.group7())
                - (Simd32x3::from(right_dual_g3[3]) * self.group1().xyz())
                - (Simd32x3::from(self[e45]) * right_dual_g1.xyz()))
            .with_w(0.0);
        let wedge_g7 = (right_dual_g5 * Simd32x3::from(self[e4]))
            + (right_dual_g7 * Simd32x3::from(self[scalar]))
            + (Simd32x3::from(right_dual_g0[0]) * self.group7())
            + (Simd32x3::from(right_dual_g1[3]) * self.group5())
            + (other.group7().yzx() * self.group1().zxy())
            + (self.group4().yzx() * right_dual_g1.zxy())
            - (other.group7().zxy() * self.group1().yzx())
            - (self.group4().zxy() * right_dual_g1.yzx());
        let wedge_g8 = Simd32x3::from([
            (right_dual_g1[1] * self[e35]) + (right_dual_g3[2] * self[e2]) - (right_dual_g1[2] * self[e25]) - (right_dual_g3[1] * self[e3]),
            (right_dual_g1[2] * self[e15]) + (right_dual_g3[0] * self[e3]) - (right_dual_g1[0] * self[e35]) - (right_dual_g3[2] * self[e1]),
            (right_dual_g1[0] * self[e25]) + (right_dual_g3[1] * self[e1]) - (right_dual_g1[1] * self[e15]) - (right_dual_g3[0] * self[e2]),
        ]) + (right_dual_g5 * Simd32x3::from(self[e5]))
            + (right_dual_g8 * Simd32x3::from(self[scalar]))
            + (Simd32x3::from(right_dual_g0[0]) * self.group8())
            + (Simd32x3::from(other[e3215]) * self.group5());
        let wedge_g9 = (right_dual_g9 * Simd32x4::from(self[scalar]))
            + (Simd32x4::from(right_dual_g0[0]) * self.group9())
            + ((right_dual_g5 * Simd32x3::from(self[e45]))
                + (right_dual_g8 * Simd32x3::from(self[e4]))
                + (Simd32x3::from(right_dual_g3[3]) * self.group5())
                + (Simd32x3::from(other[e3215]) * self.group7())
                + (other.group7().yzx() * self.group3().zxy())
                + (self.group4().yzx() * right_dual_g3.zxy())
                + ((right_dual_g1.yz() * self.group6().zx()) + (right_dual_g6.yz() * self.group1().zx())
                    - (right_dual_g1.zx() * self.group6().yz())
                    - (right_dual_g6.zx() * self.group1().yz()))
                .with_z((right_dual_g1[0] * self[e425]) + (right_dual_g6[0] * self[e2]) - (right_dual_g1[1] * self[e415]) - (right_dual_g6[1] * self[e1]))
                - (right_dual_g7 * Simd32x3::from(self[e5]))
                - (Simd32x3::from(right_dual_g1[3]) * self.group8())
                - (other.group7().zxy() * self.group3().yzx())
                - (self.group4().zxy() * right_dual_g3.yzx()))
            .with_w((self[e235] * right_dual_g1[0]) + (self[e315] * right_dual_g1[1]) + (self[e125] * right_dual_g1[2]) + (other[e3215] * self[e321]));
        let wedge_g10 = (right_dual_g10 * self[scalar])
            + (right_dual_g0[0] * self[e1234])
            + (right_dual_g7[0] * self[e1])
            + (right_dual_g7[1] * self[e2])
            + (right_dual_g7[2] * self[e3])
            + (right_dual_g6[3] * self[e4])
            - (right_dual_g5[0] * self[e41])
            - (right_dual_g5[1] * self[e42])
            - (right_dual_g5[2] * self[e43])
            - (other[e423] * self[e23])
            - (other[e431] * self[e31])
            - (other[e412] * self[e12])
            - (self[e423] * right_dual_g1[0])
            - (self[e431] * right_dual_g1[1])
            - (self[e412] * right_dual_g1[2])
            - (right_dual_g1[3] * self[e321]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar])
                    + (wedge_g10 * other[e5])
                    + (wedge_g2 * other[e1234])
                    + (wedge_g1[0] * other[e4235])
                    + (wedge_g1[1] * other[e4315])
                    + (wedge_g1[2] * other[e4125])
                    + (wedge_g1[3] * other[e3215])
                    + (wedge_g9[0] * other[e1])
                    + (wedge_g9[1] * other[e2])
                    + (wedge_g9[2] * other[e3])
                    + (wedge_g9[3] * other[e4])
                    + (right_dual_g0[0] * other[e12345] * self[scalar])
                    - (wedge_g4[0] * other[e235])
                    - (wedge_g4[1] * other[e315])
                    - (wedge_g4[2] * other[e125])
                    - (wedge_g5[0] * other[e415])
                    - (wedge_g5[1] * other[e425])
                    - (wedge_g5[2] * other[e435])
                    - (wedge_g7[0] * other[e15])
                    - (wedge_g7[1] * other[e25])
                    - (wedge_g7[2] * other[e35])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43])
                    - (other[e23] * wedge_g6[0])
                    - (other[e31] * wedge_g6[1])
                    - (other[e12] * wedge_g6[2])
                    - (other[e423] * wedge_g3[0])
                    - (other[e431] * wedge_g3[1])
                    - (other[e412] * wedge_g3[2])
                    - (wedge_g3[3] * other[e321])
                    - (wedge_g6[3] * other[e45]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            (wedge_g1 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group1())
                + (other.group9().yzxx() * wedge_g5.zxy().with_w(wedge_g4[0]))
                + ((Simd32x3::from(wedge_g6[3]) * other.group6().xyz())
                    + (Simd32x3::from(wedge_g9[3]) * other.group4())
                    + (Simd32x3::from(other[e321]) * wedge_g6.xyz())
                    + (Simd32x3::from(other[e1234]) * wedge_g3.xyz())
                    + (wedge_g7.zxy() * other.group8().yzx())
                    + (wedge_g8.yzx() * other.group7().zxy())
                    + (other.group5().yzx() * wedge_g9.zxy())
                    - (wedge_g4 * Simd32x3::from(other[e3215]))
                    - (Simd32x3::from(wedge_g10) * other.group3().xyz())
                    - (wedge_g5.yzx() * other.group9().zxy())
                    - (wedge_g7.yzx() * other.group8().zxy())
                    - (wedge_g8.zxy() * other.group7().yzx())
                    - (other.group5().zxy() * wedge_g9.yzx()))
                .with_w((wedge_g4[1] * other[e4315]) + (wedge_g4[2] * other[e4125]) + (wedge_g3[3] * other[e1234])),
            // e5
            (wedge_g0_y * other[e5])
                + (wedge_g2 * other[e12345])
                + (wedge_g9[0] * other[e15])
                + (wedge_g9[1] * other[e25])
                + (wedge_g9[2] * other[e35])
                + (wedge_g9[3] * other[e45])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435])
                - (other[e235] * wedge_g6[0])
                - (other[e315] * wedge_g6[1])
                - (other[e125] * wedge_g6[2])
                - (wedge_g3[0] * other[e4235])
                - (wedge_g3[1] * other[e4315])
                - (wedge_g3[2] * other[e4125])
                - (wedge_g3[3] * other[e3215]),
            // e15, e25, e35, e45
            (wedge_g3 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group3())
                + ((Simd32x3::from(wedge_g9[3]) * other.group6().xyz())
                    + (Simd32x3::from(other[e3215]) * wedge_g6.xyz())
                    + (wedge_g8.yzx() * other.group9().zxy())
                    + (other.group8().yzx() * wedge_g9.zxy())
                    - (wedge_g8.zxy() * other.group9().yzx())
                    - (other.group8().zxy() * wedge_g9.yzx()))
                .with_w(0.0),
            // e41, e42, e43
            (wedge_g4 * Simd32x3::from(other[e12345]))
                + (Simd32x3::from(wedge_g0_y) * other.group4())
                + (Simd32x3::from(wedge_g10) * other.group6().xyz())
                + (Simd32x3::from(other[e1234]) * wedge_g6.xyz())
                + (wedge_g7.zxy() * other.group9().yzx())
                + (other.group7().zxy() * wedge_g9.yzx())
                - (wedge_g7.yzx() * other.group9().zxy())
                - (other.group7().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (wedge_g5 * Simd32x3::from(other[e12345]))
                + (wedge_g7 * Simd32x3::from(other[e3215]))
                + (wedge_g8 * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(wedge_g0_y) * other.group5())
                + (Simd32x3::from(wedge_g10) * other.group8())
                + (Simd32x3::from(wedge_g9[3]) * other.group7())
                - (Simd32x3::from(wedge_g6[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            (wedge_g6 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group6())
                + (wedge_g9.yzxw() * other.group9().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group9().yz()).with_zw(wedge_g9[1] * other[e4235] * -1.0, wedge_g10 * other[e3215] * -1.0),
            // e423, e431, e412
            (wedge_g7 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group7()) + (Simd32x3::from(other[e1234]) * wedge_g9.xyz())
                - (Simd32x3::from(wedge_g10) * other.group9().xyz()),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (wedge_g9 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group9()),
            // e1234
            (wedge_g0_y * other[e1234]) + (wedge_g10 * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       16       43        0        0
    //    simd2        2        6        0      N/A
    //    simd3       11       22        0      N/A
    //    simd4        4        6        0      N/A
    // Totals...
    // yes simd       33       77        0      N/A
    //  no simd       69      145        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g1 =
            Simd32x4::from([1.0, 1.0, self[scalar], 0.0]) * (other.group0().xyz() * Simd32x2::from(self[scalar] * -1.0).with_z(1.0) * Simd32x3::from([1.0, 1.0, -1.0])).with_w(0.0);
        let wedge_g3_xyz = (Simd32x3::from(other[e3215]) * self.group1().xyz()) + (Simd32x3::from(self[e5]) * other.group0().xyz());
        let wedge_g4 = Simd32x3::from(self[e4] * -1.0) * other.group0().xyz();
        let wedge_g5 = Simd32x3::from([
            (self[e3] * other[e4315]) - (self[e2] * other[e4125]),
            (self[e1] * other[e4125]) - (self[e3] * other[e4235]),
            (self[e2] * other[e4235]) - (self[e1] * other[e4315]),
        ]);
        let wedge_g7 = (self.group4().zxy() * other.group0().yzx()) - (self.group4().yzx() * other.group0().zxy());
        let wedge_g8 = Simd32x3::from([
            (self[e25] * other[e4125]) - (self[e35] * other[e4315]),
            (self[e35] * other[e4235]) - (self[e15] * other[e4125]),
            (self[e15] * other[e4315]) - (self[e25] * other[e4235]),
        ]) + (Simd32x3::from(other[e3215]) * self.group5());
        let wedge_g9 = Simd32x4::from([0.0, 0.0, (self[e415] * other[e4315]) - (self[e425] * other[e4235]), 0.0])
            + ((Simd32x3::from(other[e3215]) * self.group7()) + ((self.group6().yz() * other.group0().zx()) - (self.group6().zx() * other.group0().yz())).with_z(0.0)).with_w(0.0);
        let wedge_g10 = (self[e423] * other[e4235]) + (self[e431] * other[e4315]) + (self[e412] * other[e4125]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g1[0] * other[e4235]) + (wedge_g1[1] * other[e4315]) + (wedge_g1[2] * other[e4125]) + (wedge_g1[3] * other[e3215]),
                0.0,
            ]),
            // e1, e2, e3, e4
            ((wedge_g5.zxy() * other.group0().yzx()) - (wedge_g4 * Simd32x3::from(other[e3215])) - (wedge_g5.yzx() * other.group0().zxy())).with_w(wedge_g4[0] * other[e4235]),
            // e5
            -(wedge_g3_xyz[0] * other[e4235]) - (wedge_g3_xyz[1] * other[e4315]) - (wedge_g3_xyz[2] * other[e4125]) - (other[e3215] * other[e3215] * self[e4]),
            // e15, e25, e35, e45
            ((Simd32x3::from(other[e3215] * other[e3215]) * self.group4())
                + (Simd32x3::from(self[e45] * other[e3215]) * other.group0().xyz())
                + (wedge_g8.yzx() * other.group0().zxy())
                - (wedge_g8.zxy() * other.group0().yzx()))
            .with_w(0.0),
            // e41, e42, e43
            (wedge_g7.zxy() * other.group0().yzx()) - (wedge_g7.yzx() * other.group0().zxy()),
            // e23, e31, e12
            wedge_g7 * Simd32x3::from(other[e3215]),
            // e415, e425, e435, e321
            ((wedge_g9.yz() * other.group0().zx()) - (wedge_g9.zx() * other.group0().yz()))
                .with_zw((wedge_g9[0] * other[e4315]) - (wedge_g9[1] * other[e4235]), wedge_g10 * other[e3215] * -1.0),
            // e423, e431, e412
            Simd32x3::from(wedge_g10 * -1.0) * other.group0().xyz(),
            // e235, e315, e125
            (Simd32x3::from(wedge_g9[3]) * other.group0().xyz()) - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(other[e3215] * self[e1234]) * other.group0())
                + -(other.group0().xy() * other.group0().xy() * self.group9().xy())
                    .with_zw(other[e4125] * other[e4125] * self[e4125] * -1.0, self[e4235] * other[e4235] * other[e3215] * -1.0)
                - (self.group9().yxxy() * other.group0().xxxy() * other.group0().yyzw())
                - (self.group9().zzyz() * other.group0().xyyz() * other.group0().zzzw()),
            // e1234
            0.0,
        )
    }
}
impl ProjectViaOriginOnto<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       14        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        8       16        0      N/A
    //  no simd        8       22        0        0
    fn project_via_origin_onto(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().xyz().with_w(other[e5] * -1.0);
        let right_dual_g1 = other[e4] * -1.0;
        let wedge_g0_y = (right_dual_g1 * self[e5]) + (right_dual_g0[0] * self[e1]) + (right_dual_g0[1] * self[e2]) + (right_dual_g0[2] * self[e3]) + (right_dual_g0[3] * self[e4]);
        let wedge_g9 = right_dual_g0 * Simd32x4::from(self[scalar]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g9[0] * other[e1]) + (wedge_g9[1] * other[e2]) + (wedge_g9[2] * other[e3]) + (wedge_g9[3] * other[e4]) + (right_dual_g1 * self[scalar] * other[e5]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(wedge_g0_y) * other.group0(),
            // e5
            wedge_g0_y * other[e5],
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
impl ProjectViaOriginOnto<Scalar> for MultiVector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn project_via_origin_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * other[scalar] * other[scalar])
    }
}
impl ProjectViaOriginOnto<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       20       40        0        0
    //    simd2        0        2        0      N/A
    //    simd3       17       27        0      N/A
    //    simd4        4        7        0      N/A
    // Totals...
    // yes simd       41       76        0      N/A
    //  no simd       87      153        0        0
    fn project_via_origin_onto(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let wedge_g0_y = (right_dual_g0[0] * self[e4235])
            + (right_dual_g0[1] * self[e4315])
            + (right_dual_g0[2] * self[e4125])
            + (right_dual_g0[3] * self[e3215])
            + (other[e3215] * self[e1234]);
        let wedge_g1 = right_dual_g0 * Simd32x4::from(self[scalar]);
        let wedge_g3 = (Simd32x4::from(other[e3215]) * self.group1()) - (right_dual_g0 * Simd32x4::from(self[e5]));
        let wedge_g4 = (Simd32x3::from(self[e4]) * right_dual_g0.xyz()) - (Simd32x3::from(right_dual_g0[3]) * self.group1().xyz());
        let wedge_g5 = Simd32x3::from([
            (right_dual_g0[2] * self[e2]) - (right_dual_g0[1] * self[e3]),
            (right_dual_g0[0] * self[e3]) - (right_dual_g0[2] * self[e1]),
            (right_dual_g0[1] * self[e1]) - (right_dual_g0[0] * self[e2]),
        ]);
        let wedge_g6_xyz =
            (Simd32x3::from(right_dual_g0[3]) * self.group3().xyz()) + (Simd32x3::from(other[e3215]) * self.group4()) - (Simd32x3::from(self[e45]) * right_dual_g0.xyz());
        let wedge_g7 = (Simd32x3::from(right_dual_g0[3]) * self.group5()) + (self.group4().yzx() * right_dual_g0.zxy()) - (self.group4().zxy() * right_dual_g0.yzx());
        let wedge_g8 = Simd32x3::from([
            (right_dual_g0[1] * self[e35]) - (right_dual_g0[2] * self[e25]),
            (right_dual_g0[2] * self[e15]) - (right_dual_g0[0] * self[e35]),
            (right_dual_g0[0] * self[e25]) - (right_dual_g0[1] * self[e15]),
        ]) + (Simd32x3::from(other[e3215]) * self.group5());
        let wedge_g9 = (right_dual_g0.yzxy() * self.group6().zxy().with_w(self[e315]))
            + ((Simd32x3::from(other[e3215]) * self.group7()) + -(right_dual_g0.zx() * self.group6().yz()).with_z(right_dual_g0[1] * self[e415] * -1.0)
                - (Simd32x3::from(right_dual_g0[3]) * self.group8()))
            .with_w(self[e235] * right_dual_g0[0]);
        let wedge_g10 = -(self[e423] * right_dual_g0[0]) - (self[e431] * right_dual_g0[1]) - (self[e412] * right_dual_g0[2]) - (right_dual_g0[3] * self[e321]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g1[0] * other[e4235])
                    + (wedge_g1[1] * other[e4315])
                    + (wedge_g1[2] * other[e4125])
                    + (wedge_g1[3] * other[e3215])
                    + (self[scalar] * other[e3215] * other[e1234]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (other.group0().yzxx() * wedge_g5.zxy().with_w(wedge_g4[0]))
                + ((Simd32x3::from(other[e1234]) * wedge_g3.xyz()) - (wedge_g4 * Simd32x3::from(other[e3215])) - (wedge_g5.yzx() * other.group0().zxy()))
                    .with_w(wedge_g4[1] * other[e4315]),
            // e5
            -(wedge_g3[0] * other[e4235]) - (wedge_g3[1] * other[e4315]) - (wedge_g3[2] * other[e4125]) - (wedge_g3[3] * other[e3215]),
            // e15, e25, e35, e45
            ((wedge_g6_xyz * Simd32x3::from(other[e3215])) + (wedge_g8.yzx() * other.group0().zxy()) - (wedge_g8.zxy() * other.group0().yzx())).with_w(0.0),
            // e41, e42, e43
            (wedge_g6_xyz * Simd32x3::from(other[e1234])) + (wedge_g7.zxy() * other.group0().yzx()) - (wedge_g7.yzx() * other.group0().zxy()),
            // e23, e31, e12
            (wedge_g7 * Simd32x3::from(other[e3215])) + (wedge_g8 * Simd32x3::from(other[e1234])),
            // e415, e425, e435, e321
            (wedge_g9.yzxw() * other.group0().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group0().yz()).with_zw(wedge_g9[1] * other[e4235] * -1.0, wedge_g10 * other[e3215] * -1.0),
            // e423, e431, e412
            (Simd32x3::from(other[e1234]) * wedge_g9.xyz()) - (Simd32x3::from(wedge_g10) * other.group0().xyz()),
            // e235, e315, e125
            (Simd32x3::from(wedge_g9[3]) * other.group0().xyz()) - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0_y) * other.group0(),
            // e1234
            wedge_g0_y * other[e1234],
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       50       72        0        0
    //    simd3       32       42        0      N/A
    //    simd4        4        8        0      N/A
    // Totals...
    // yes simd       86      122        0      N/A
    //  no simd      162      230        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g1_w = other[e321] * -1.0;
        let right_dual_g2_xyz = other.group2().xyz();
        let right_dual_g2_w = other[e4] * -1.0;
        let right_dual_g3_xyz = other.group3().xyz();
        let right_dual_g3_w = other[e5] * -1.0;
        let wedge_g0_y = (right_dual_g0_w * self[e12345])
            + (right_dual_g2_w * self[e5])
            + (right_dual_g3_w * self[e4])
            + (right_dual_g3_xyz[0] * self[e1])
            + (right_dual_g3_xyz[1] * self[e2])
            + (right_dual_g3_xyz[2] * self[e3])
            - (right_dual_g1_w * self[e321])
            - (right_dual_g0_xyz[0] * self[e235])
            - (right_dual_g0_xyz[1] * self[e315])
            - (right_dual_g0_xyz[2] * self[e125])
            - (right_dual_g1_xyz[0] * self[e415])
            - (right_dual_g1_xyz[1] * self[e425])
            - (right_dual_g1_xyz[2] * self[e435])
            - (right_dual_g2_xyz[0] * self[e423])
            - (right_dual_g2_xyz[1] * self[e431])
            - (right_dual_g2_xyz[2] * self[e412]);
        let wedge_g1 = Simd32x4::from(right_dual_g0_w) * self.group1();
        let wedge_g3 = (Simd32x4::from(right_dual_g0_w) * self.group3()) + (Simd32x4::from(self[scalar]) * right_dual_g2_xyz.with_w(right_dual_g1_w));
        let wedge_g4 = (right_dual_g0_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_dual_g0_w) * self.group4());
        let wedge_g5 = (right_dual_g1_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_dual_g0_w) * self.group5());
        let wedge_g6 = ((right_dual_g0_xyz * Simd32x3::from(self[e5])) + (right_dual_g2_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_dual_g0_w) * self.group6().xyz())
            - (Simd32x3::from(right_dual_g1_w) * self.group1().xyz()))
        .with_w(right_dual_g0_w * self[e321]);
        let wedge_g7 = (right_dual_g1_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_dual_g0_w) * self.group7()) + (right_dual_g0_xyz.yzx() * self.group1().zxy())
            - (right_dual_g0_xyz.zxy() * self.group1().yzx());
        let wedge_g8 = (right_dual_g1_xyz * Simd32x3::from(self[e5])) + (Simd32x3::from(right_dual_g0_w) * self.group8()) + (right_dual_g2_xyz.zxy() * self.group1().yzx())
            - (right_dual_g2_xyz.yzx() * self.group1().zxy());
        let wedge_g9 = (Simd32x4::from(right_dual_g0_w) * self.group9())
            + ((right_dual_g1_xyz * Simd32x3::from(self[e45]))
                + (right_dual_g3_xyz * Simd32x3::from(self[scalar]))
                + (Simd32x3::from(right_dual_g1_w) * self.group5())
                + (right_dual_g0_xyz.yzx() * self.group3().zxy())
                + (right_dual_g2_xyz.zxy() * self.group4().yzx())
                - (right_dual_g0_xyz.zxy() * self.group3().yzx())
                - (right_dual_g2_xyz.yzx() * self.group4().zxy()))
            .with_w(right_dual_g3_w * self[scalar]);
        let wedge_g10 = (right_dual_g0_w * self[e1234]) + (right_dual_g2_w * self[scalar])
            - (right_dual_g0_xyz[0] * self[e23])
            - (right_dual_g0_xyz[1] * self[e31])
            - (right_dual_g0_xyz[2] * self[e12])
            - (right_dual_g1_xyz[0] * self[e41])
            - (right_dual_g1_xyz[1] * self[e42])
            - (right_dual_g1_xyz[2] * self[e43]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g10 * other[e5])
                    + (wedge_g9[0] * other[e1])
                    + (wedge_g9[1] * other[e2])
                    + (wedge_g9[2] * other[e3])
                    + (wedge_g9[3] * other[e4])
                    + (right_dual_g0_w * self[scalar] * other[e12345])
                    - (wedge_g4[0] * other[e235])
                    - (wedge_g4[1] * other[e315])
                    - (wedge_g4[2] * other[e125])
                    - (wedge_g5[0] * other[e415])
                    - (wedge_g5[1] * other[e425])
                    - (wedge_g5[2] * other[e435])
                    - (wedge_g3[0] * other[e423])
                    - (wedge_g3[1] * other[e431])
                    - (wedge_g3[2] * other[e412])
                    - (wedge_g3[3] * other[e321]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(wedge_g0_y) * other.group3())
                + ((Simd32x3::from(wedge_g6[3]) * other.group1().xyz())
                    + (Simd32x3::from(other[e12345]) * wedge_g1.xyz())
                    + (Simd32x3::from(other[e321]) * wedge_g6.xyz())
                    + (wedge_g7.zxy() * other.group2().yzx())
                    + (wedge_g8.yzx() * other.group0().zxy())
                    - (wedge_g7.yzx() * other.group2().zxy())
                    - (wedge_g8.zxy() * other.group0().yzx()))
                .with_w(wedge_g1[3] * other[e12345]),
            // e5
            (wedge_g0_y * other[e5]) + (right_dual_g0_w * other[e12345] * self[e5])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435])
                - (wedge_g6[0] * other[e235])
                - (wedge_g6[1] * other[e315])
                - (wedge_g6[2] * other[e125]),
            // e15, e25, e35, e45
            (Simd32x3::from([
                (wedge_g9[2] * other[e315]) - (wedge_g9[1] * other[e125]),
                (wedge_g9[0] * other[e125]) - (wedge_g9[2] * other[e235]),
                (wedge_g9[1] * other[e235]) - (wedge_g9[0] * other[e315]),
            ]) + (Simd32x3::from(wedge_g9[3]) * other.group1().xyz())
                + (Simd32x3::from(other[e12345]) * wedge_g3.xyz()))
            .with_w(wedge_g3[3] * other[e12345]),
            // e41, e42, e43
            Simd32x3::from([
                (wedge_g9[1] * other[e412]) - (wedge_g9[2] * other[e431]),
                (wedge_g9[2] * other[e423]) - (wedge_g9[0] * other[e412]),
                (wedge_g9[0] * other[e431]) - (wedge_g9[1] * other[e423]),
            ]) + (wedge_g4 * Simd32x3::from(other[e12345]))
                + (Simd32x3::from(wedge_g10) * other.group1().xyz()),
            // e23, e31, e12
            (wedge_g5 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g10) * other.group2().xyz()) + (Simd32x3::from(wedge_g9[3]) * other.group0().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            (wedge_g6 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group1()),
            // e423, e431, e412
            (wedge_g7 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group0().xyz()),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group2().xyz()),
            // e4235, e4315, e4125, e3215
            wedge_g9 * Simd32x4::from(other[e12345]),
            // e1234
            wedge_g10 * other[e12345],
        )
    }
}
impl ProjectViaOriginOnto<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       59       81        0        0
    //    simd2        2        5        0      N/A
    //    simd3       28       38        0      N/A
    //    simd4        6        9        0      N/A
    // Totals...
    // yes simd       95      133        0      N/A
    //  no simd      171      241        0        0
    fn project_via_origin_onto(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2_xyz = other.group2().xyz() * Simd32x3::from(-1.0);
        let right_dual_g3 = (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let wedge_g0_y = (self[scalar] * other[scalar])
            + (self[e23] * other[e23])
            + (self[e31] * other[e31])
            + (self[e12] * other[e12])
            + (right_dual_g3[0] * self[e4235])
            + (right_dual_g3[1] * self[e4315])
            + (right_dual_g3[2] * self[e4125])
            + (right_dual_g3[3] * self[e3215])
            + (self[e15] * other[e41])
            + (self[e25] * other[e42])
            + (self[e35] * other[e43])
            + (other[e3215] * self[e1234])
            - (right_dual_g2_xyz[0] * self[e41])
            - (right_dual_g2_xyz[1] * self[e42])
            - (right_dual_g2_xyz[2] * self[e43])
            - (self[e45] * other[e45]);
        let wedge_g1 = right_dual_g3 * Simd32x4::from(self[scalar]);
        let wedge_g3 = (Simd32x4::from(other[e3215]) * self.group1()) - (right_dual_g3 * Simd32x4::from(self[e5]));
        let wedge_g4 = (Simd32x3::from(self[e4]) * right_dual_g3.xyz()) - (Simd32x3::from(right_dual_g3[3]) * self.group1().xyz());
        let wedge_g5 = Simd32x3::from([
            (right_dual_g3[2] * self[e2]) - (right_dual_g3[1] * self[e3]),
            (right_dual_g3[0] * self[e3]) - (right_dual_g3[2] * self[e1]),
            (right_dual_g3[1] * self[e1]) - (right_dual_g3[0] * self[e2]),
        ]);
        let wedge_g6_xyz = (Simd32x3::from(right_dual_g3[3]) * self.group3().xyz()) + (Simd32x3::from(other[e3215]) * self.group4())
            - (Simd32x3::from(self[scalar]) * other.group1().xyz())
            - (Simd32x3::from(self[e45]) * right_dual_g3.xyz());
        let wedge_g6_w = self[scalar] * other[e45];
        let wedge_g7 = (Simd32x3::from(right_dual_g3[3]) * self.group5()) + (self.group4().yzx() * right_dual_g3.zxy())
            - (Simd32x3::from(self[scalar]) * other.group0().xyz())
            - (self.group4().zxy() * right_dual_g3.yzx());
        let wedge_g8 = Simd32x3::from([
            (right_dual_g3[1] * self[e35]) - (right_dual_g3[2] * self[e25]),
            (right_dual_g3[2] * self[e15]) - (right_dual_g3[0] * self[e35]),
            (right_dual_g3[0] * self[e25]) - (right_dual_g3[1] * self[e15]),
        ]) + (right_dual_g2_xyz * Simd32x3::from(self[scalar]))
            + (Simd32x3::from(other[e3215]) * self.group5());
        let wedge_g9 = (right_dual_g3.yzxz() * self.group6().zxy().with_w(self[e125]))
            + ((right_dual_g2_xyz * Simd32x3::from(self[e4]))
                + (Simd32x3::from(other[e3215]) * self.group7())
                + (Simd32x3::from(self[e5]) * other.group0().xyz())
                + ((self.group1().yz() * other.group1().zx()) - (right_dual_g3.zx() * self.group6().yz()) - (self.group1().zx() * other.group1().yz()))
                    .with_z((self[e1] * other[e31]) - (right_dual_g3[1] * self[e415]) - (self[e2] * other[e23]))
                - (Simd32x3::from(right_dual_g3[3]) * self.group8()))
            .with_w((self[e235] * right_dual_g3[0]) + (self[e315] * right_dual_g3[1]) + (self[e321] * other[e3215]) - (other[e45] * self[e5]));
        let wedge_g10 = (self[e4] * other[e45])
            - (self[e423] * right_dual_g3[0])
            - (self[e431] * right_dual_g3[1])
            - (self[e412] * right_dual_g3[2])
            - (right_dual_g3[3] * self[e321])
            - (self[e1] * other[e41])
            - (self[e2] * other[e42])
            - (self[e3] * other[e43]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar])
                    + (wedge_g1[0] * other[e4235])
                    + (wedge_g1[1] * other[e4315])
                    + (wedge_g1[2] * other[e4125])
                    + (wedge_g1[3] * other[e3215])
                    + (self[scalar] * other[e1234] * other[e3215])
                    - (wedge_g6_w * other[e45])
                    - (wedge_g6_xyz[0] * other[e23])
                    - (wedge_g6_xyz[1] * other[e31])
                    - (wedge_g6_xyz[2] * other[e12])
                    - (wedge_g7[0] * other[e15])
                    - (wedge_g7[1] * other[e25])
                    - (wedge_g7[2] * other[e35])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (other.group3().yzxx() * wedge_g5.zxy().with_w(wedge_g4[0]))
                + ((Simd32x3::from(wedge_g9[3]) * other.group0().xyz())
                    + (Simd32x3::from(other[e1234]) * wedge_g3.xyz())
                    + (wedge_g9.zx() * other.group1().yz()).with_z(wedge_g9[1] * other[e23])
                    - (wedge_g4 * Simd32x3::from(other[e3215]))
                    - (wedge_g5.yzx() * other.group3().zxy()))
                .with_w((wedge_g4[1] * other[e4315]) + (wedge_g4[2] * other[e4125]) + (wedge_g3[3] * other[e1234]) - (wedge_g9[0] * other[e41]) - (wedge_g9[1] * other[e42]))
                - (Simd32x4::from(wedge_g10) * other.group2().xyz().with_w(other[e45]))
                - (wedge_g9.yzxz() * other.group1().zxy().with_w(other[e43])),
            // e5
            (wedge_g9[0] * other[e15]) + (wedge_g9[1] * other[e25]) + (wedge_g9[2] * other[e35]) + (wedge_g9[3] * other[e45])
                - (wedge_g3[0] * other[e4235])
                - (wedge_g3[1] * other[e4315])
                - (wedge_g3[2] * other[e4125])
                - (wedge_g3[3] * other[e3215]),
            // e15, e25, e35, e45
            ((wedge_g6_xyz * Simd32x3::from(other[e3215])) + (Simd32x3::from(wedge_g0_y) * other.group2().xyz()) + (wedge_g8.yzx() * other.group3().zxy())
                - (wedge_g8.zxy() * other.group3().yzx()))
            .with_w(wedge_g0_y * other[e45]),
            // e41, e42, e43
            (wedge_g6_xyz * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0_y) * other.group0().xyz()) + (wedge_g7.zxy() * other.group3().yzx())
                - (wedge_g7.yzx() * other.group3().zxy()),
            // e23, e31, e12
            (wedge_g7 * Simd32x3::from(other[e3215])) + (wedge_g8 * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0_y) * other.group1().xyz())
                - (Simd32x3::from(wedge_g6_w) * other.group3().xyz()),
            // e415, e425, e435, e321
            (wedge_g9.yzxw() * other.group3().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group3().yz()).with_zw(wedge_g9[1] * other[e4235] * -1.0, wedge_g10 * other[e3215] * -1.0),
            // e423, e431, e412
            (Simd32x3::from(other[e1234]) * wedge_g9.xyz()) - (Simd32x3::from(wedge_g10) * other.group3().xyz()),
            // e235, e315, e125
            (Simd32x3::from(wedge_g9[3]) * other.group3().xyz()) - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0_y) * other.group3(),
            // e1234
            wedge_g0_y * other[e1234],
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for Plane {
    type Output = ProjectViaOriginOntoInfixPartial<Plane>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiScalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group0())
    }
}
impl ProjectViaOriginOnto<CircleRotor> for Plane {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd2        1        2        0      N/A
    //    simd3        3        5        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd        6       12        0      N/A
    //  no simd       16       30        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(other[e12345] * -1.0) * self.group0();
        DipoleInversion::from_groups(
            // e41, e42, e43
            (other.group0().zxy() * wedge_g0.yzx()) - (other.group0().yzx() * wedge_g0.zxy()),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g0[3]) * other.group0()) - (Simd32x3::from(other[e321]) * wedge_g0.xyz())).with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, (wedge_g0[1] * other[e235]) - (wedge_g0[0] * other[e315]), 0.0])
                + ((Simd32x3::from(wedge_g0[3]) * other.group1().xyz()) + ((wedge_g0.zx() * other.group2().yz()) - (wedge_g0.yz() * other.group2().zx())).with_z(0.0)).with_w(0.0),
            // e4235, e4315, e4125, e3215
            wedge_g0 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<DipoleInversion> for Plane {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       19        0        0
    fn project_via_origin_onto(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (other[e1234] * self[e3215]) - (other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]);
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g0) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(wedge_g0) * other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(wedge_g0) * other.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0) * other.group3(),
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group0())
    }
}
impl ProjectViaOriginOnto<Flector> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2       11        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = -(other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]);
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(wedge_g0) * other.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd2        1        2        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd       10       18        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(other[e12345] * -1.0) * self.group0();
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, (wedge_g0[1] * other[e235]) - (wedge_g0[0] * other[e315]), 0.0])
                + ((Simd32x3::from(wedge_g0[3]) * other.group0().xyz()) + ((wedge_g0.zx() * other.group1().yz()) - (wedge_g0.yz() * other.group1().zx())).with_z(0.0)).with_w(0.0),
            // e4235, e4315, e4125, e3215
            wedge_g0 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       21        0        0
    //    simd2        0        1        0      N/A
    //    simd3       13       20        0      N/A
    //    simd4        4        5        0      N/A
    // Totals...
    // yes simd       28       47        0      N/A
    //  no simd       66      103        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let wedge_g0_y = (right_dual_g1_xyz[0] * self[e4235]) + (right_dual_g1_xyz[1] * self[e4315]) + (right_dual_g1_xyz[2] * self[e4125]) + (self[e3215] * other[e1234]);
        let wedge_g9 = Simd32x4::from(other[e12345] * -1.0) * self.group0();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar]) + (wedge_g9[0] * other[e1]) + (wedge_g9[1] * other[e2]) + (wedge_g9[2] * other[e3]) + (wedge_g9[3] * other[e4]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g0_y) * other.group1().xyz()) + (Simd32x3::from(wedge_g9[3]) * other.group4()) + (other.group5().yzx() * wedge_g9.zxy())
                - (other.group5().zxy() * wedge_g9.yzx()))
            .with_w(wedge_g0_y * other[e4]),
            // e5
            (wedge_g0_y * other[e5]) + (wedge_g9[0] * other[e15]) + (wedge_g9[1] * other[e25]) + (wedge_g9[2] * other[e35]) + (wedge_g9[3] * other[e45]),
            // e15, e25, e35, e45
            ((Simd32x3::from(wedge_g0_y) * other.group3().xyz()) + (Simd32x3::from(wedge_g9[3]) * other.group6().xyz()) + (other.group8().yzx() * wedge_g9.zxy())
                - (other.group8().zxy() * wedge_g9.yzx()))
            .with_w(wedge_g0_y * other[e45]),
            // e41, e42, e43
            (Simd32x3::from(wedge_g0_y) * other.group4()) + (other.group7().zxy() * wedge_g9.yzx()) - (other.group7().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (Simd32x3::from(wedge_g0_y) * other.group5()) + (Simd32x3::from(wedge_g9[3]) * other.group7()) - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, wedge_g9[1] * other[e4235] * -1.0, 0.0])
                + (Simd32x4::from(wedge_g0_y) * other.group6())
                + (wedge_g9.yzxw() * other.group9().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group9().yz()).with_zw(0.0, 0.0),
            // e423, e431, e412
            (Simd32x3::from(wedge_g0_y) * other.group7()) + (Simd32x3::from(other[e1234]) * wedge_g9.xyz()),
            // e235, e315, e125
            (Simd32x3::from(wedge_g0_y) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group9().xyz()) - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (wedge_g9 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group9()),
            // e1234
            wedge_g0_y * other[e1234],
        )
    }
}
impl ProjectViaOriginOnto<Plane> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd4        2        5        0      N/A
    // Totals...
    // yes simd        2        9        0      N/A
    //  no simd        8       24        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            -(Simd32x4::from([other[e4235] * self[e4235], other[e4315] * self[e4315], other[e4125] * self[e4125], other[e3215] * self[e4235]]) * other.group0().xyzx())
                - (other.group0().xxxy() * other.group0().yyzw() * self.group0().yxxy())
                - (other.group0().xyyz() * other.group0().zzzw() * self.group0().zzyz()),
        )
    }
}
impl ProjectViaOriginOnto<Sphere> for Plane {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       12        0        0
    fn project_via_origin_onto(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = (right_dual_g0_xyz[0] * self[e4235]) + (right_dual_g0_xyz[1] * self[e4315]) + (right_dual_g0_xyz[2] * self[e4125]) + (self[e3215] * other[e1234]);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0) * other.group0(),
            // e1234
            wedge_g0 * other[e1234],
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for Plane {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd2        2        4        0      N/A
    //    simd3        2        3        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd        7       15        0      N/A
    //  no simd       16       31        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(other[e12345] * -1.0) * self.group0();
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((wedge_g0.yz() * other.group0().zx()) - (wedge_g0.zx() * other.group0().yz()))
                .with_zw((wedge_g0[0] * other[e431]) - (wedge_g0[1] * other[e423]), wedge_g0[0] * other[e1]),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g0[3]) * other.group0().xyz()) - (Simd32x3::from(other[e321]) * wedge_g0.xyz())).with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, (wedge_g0[1] * other[e235]) - (wedge_g0[0] * other[e315]), 0.0])
                + ((Simd32x3::from(wedge_g0[3]) * other.group1().xyz()) + ((wedge_g0.zx() * other.group2().yz()) - (wedge_g0.yz() * other.group2().zx())).with_z(0.0)).with_w(0.0),
            // e4235, e4315, e4125, e3215
            wedge_g0 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<VersorOdd> for Plane {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        3        9        0      N/A
    //  no simd        3       23        0        0
    fn project_via_origin_onto(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = (right_dual_g3_xyz[0] * self[e4235]) + (right_dual_g3_xyz[1] * self[e4315]) + (right_dual_g3_xyz[2] * self[e4125]) + (self[e3215] * other[e1234]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(wedge_g0) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(wedge_g0) * other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(wedge_g0) * other.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0) * other.group3(),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for RoundPoint {
    type Output = ProjectViaOriginOntoInfixPartial<RoundPoint>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiCircleRotor> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       12        0        0
    //    simd2        2        4        0      N/A
    //    simd3        7        8        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd       16       24        0      N/A
    //  no simd       38       44        0        0
    fn project_via_origin_onto(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0 = Simd32x4::from([0.0, 0.0, (other[e31] * self[e1]) - (other[e23] * self[e2]), 0.0])
            + (((other.group1().zx() * self.group0().yz()) - (other.group1().yz() * self.group0().zx())).with_z(0.0)
                - (right_dual_g0 * Simd32x3::from(self[e5]))
                - (Simd32x3::from(self[e4]) * other.group2().xyz()))
            .with_w(0.0);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, (wedge_g0[1] * other[e23]) - (wedge_g0[0] * other[e31]), 0.0])
                + ((Simd32x3::from(wedge_g0[3]) * other.group0()) + ((wedge_g0.zx() * other.group1().yz()) - (wedge_g0.yz() * other.group1().zx())).with_z(0.0)
                    - (Simd32x3::from(right_dual_g0[0] * self[e1]) * other.group2().xyz())
                    - (Simd32x3::from(right_dual_g0[1] * self[e2]) * other.group2().xyz())
                    - (Simd32x3::from(right_dual_g0[2] * self[e3]) * other.group2().xyz())
                    - (Simd32x3::from(other[e45] * self[e4]) * other.group2().xyz()))
                .with_w(0.0),
            // e5
            (wedge_g0[0] * other[e15]) + (wedge_g0[1] * other[e25]) + (wedge_g0[2] * other[e35]) + (wedge_g0[3] * other[e45]),
        )
    }
}
impl ProjectViaOriginOnto<AntiDipoleInversion> for RoundPoint {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3       11       17        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       17       27        0      N/A
    //  no simd       39       64        0        0
    fn project_via_origin_onto(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g2_xyz = other.group2().xyz();
        let wedge_g0 = (right_dual_g1_xyz * Simd32x3::from(self[e4])) + (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx());
        let wedge_g1_xyz = (right_dual_g2_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(other[e321]) * self.group0().xyz()) + (Simd32x3::from(self[e5]) * other.group0());
        let wedge_g2_xyz = (right_dual_g1_xyz * Simd32x3::from(self[e5])) + (right_dual_g2_xyz.zxy() * self.group0().yzx()) - (right_dual_g2_xyz.yzx() * self.group0().zxy());
        let wedge_g2_w = other[e1] * self[e1];
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(wedge_g2_w) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(wedge_g2_w) * other.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(wedge_g2_w) * other.group2().xyz()).with_w(
                (wedge_g2_w * other[e4])
                    - (wedge_g0[0] * other[e415])
                    - (wedge_g0[1] * other[e425])
                    - (wedge_g0[2] * other[e435])
                    - (wedge_g1_xyz[0] * other[e423])
                    - (wedge_g1_xyz[1] * other[e431])
                    - (wedge_g1_xyz[2] * other[e412]),
            ),
            // e1, e2, e3, e5
            ((wedge_g1_xyz * Simd32x3::from(other[e321]))
                + (Simd32x3::from(wedge_g2_w) * other.group3().xyz())
                + (wedge_g0.zxy() * other.group2().yzx())
                + (wedge_g2_xyz.yzx() * other.group0().zxy())
                - (wedge_g0.yzx() * other.group2().zxy())
                - (wedge_g2_xyz.zxy() * other.group0().yzx()))
            .with_w(wedge_g2_w * other[e5]),
        )
    }
}
impl ProjectViaOriginOnto<AntiDualNum> for RoundPoint {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn project_via_origin_onto(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([other[e3215] * other[e3215] * self[e4] * -1.0, 0.0]))
    }
}
impl ProjectViaOriginOnto<AntiFlatPoint> for RoundPoint {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        5       12        0        0
    fn project_via_origin_onto(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (Simd32x3::from(other[e321]) * self.group0().xyz()) + (Simd32x3::from(self[e4]) * other.group0().xyz());
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (wedge_g0 * Simd32x4::from(other[e321]).xyz()).with_w(-(wedge_g0[0] * other[e235]) - (wedge_g0[1] * other[e315]) - (wedge_g0[2] * other[e125])),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlector> for RoundPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        2        3        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        8        0      N/A
    //  no simd        6       17        0        0
    fn project_via_origin_onto(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_w = other[e1] * self[e1];
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(wedge_g0_w) * other.group0(),
            // e1, e2, e3, e5
            ((Simd32x3::from(wedge_g0_w) * other.group1().xyz())
                + (Simd32x3::from(other[e321] * other[e321]) * self.group0().xyz())
                + (Simd32x3::from(other[e321] * self[e4]) * other.group0().xyz()))
            .with_w(wedge_g0_w * other[e5]),
        )
    }
}
impl ProjectViaOriginOnto<AntiLine> for RoundPoint {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        3        6        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        9       19        0        0
    fn project_via_origin_onto(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = (right_dual_g0.yzx() * self.group0().zxy()) - (Simd32x3::from(self[e4]) * other.group1()) - (right_dual_g0.zxy() * self.group0().yzx());
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            ((wedge_g0_xyz.zxy() * other.group0().yzx()) - (wedge_g0_xyz.yzx() * other.group0().zxy())).with_w(wedge_g0_xyz[0] * other[e15]),
        )
    }
}
impl ProjectViaOriginOnto<AntiMotor> for RoundPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        7        0        0
    //    simd2        1        3        0      N/A
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        5       14        0      N/A
    //  no simd       14       26        0        0
    fn project_via_origin_onto(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g1 = Simd32x4::from([0.0, 0.0, (other[e31] * self[e1]) - (other[e23] * self[e2]), 0.0])
            + (((other.group0().zx() * self.group0().yz()) - (other.group0().yz() * self.group0().zx())).with_z(0.0) - (Simd32x3::from(self[e4]) * other.group1().xyz()))
                .with_w(0.0);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (wedge_g1.xyz() * Simd32x4::from(other[e3215]).xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e1, e2, e3, e5
            (wedge_g1.zxyx() * other.group0().yzx().with_w(other[e15]))
                + -(wedge_g1.yz() * other.group0().zx()).with_zw(wedge_g1[0] * other[e31] * -1.0, other[e3215] * other[e3215] * self[e4] * -1.0),
        )
    }
}
impl ProjectViaOriginOnto<AntiPlane> for RoundPoint {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd       12       20        0        0
    fn project_via_origin_onto(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x4::from(right_dual_g0_xyz[0] * self[e1]) * other.group0())
                + (Simd32x4::from(right_dual_g0_xyz[1] * self[e2]) * other.group0())
                + (Simd32x4::from(right_dual_g0_xyz[2] * self[e3]) * other.group0())
                - (Simd32x4::from(other[e5] * self[e4]) * other.group0()),
        )
    }
}
impl ProjectViaOriginOnto<AntiScalar> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0        8        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e12345] * -1.0;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group0(),
            // e5
            right_dual_g0 * other[e12345] * self[e5],
        )
    }
}
impl ProjectViaOriginOnto<Circle> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3       10       14        0      N/A
    // Totals...
    // yes simd       15       20        0      N/A
    //  no simd       35       48        0        0
    fn project_via_origin_onto(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0 = (right_dual_g1_xyz * Simd32x3::from(self[e4])) + (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx());
        let wedge_g1_xyz = (Simd32x3::from(other[e321]) * self.group0().xyz()) + (Simd32x3::from(self[e4]) * other.group2()) + (Simd32x3::from(self[e5]) * other.group0());
        let wedge_g2 = (right_dual_g1_xyz * Simd32x3::from(self[e5])) + (other.group2().zxy() * self.group0().yzx()) - (other.group2().yzx() * self.group0().zxy());
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            ((wedge_g1_xyz * Simd32x3::from(other[e321])) + (wedge_g0.zxy() * other.group2().yzx()) + (wedge_g2.yzx() * other.group0().zxy())
                - (wedge_g0.yzx() * other.group2().zxy())
                - (wedge_g2.zxy() * other.group0().yzx()))
            .with_w(0.0),
            // e5
            -(wedge_g1_xyz[0] * other[e235])
                - (wedge_g1_xyz[1] * other[e315])
                - (wedge_g1_xyz[2] * other[e125])
                - (wedge_g2[0] * other[e415])
                - (wedge_g2[1] * other[e425])
                - (wedge_g2[2] * other[e435]),
        )
    }
}
impl ProjectViaOriginOnto<CircleRotor> for RoundPoint {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       10        0        0
    //    simd3       12       18        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       18       30        0      N/A
    //  no simd       42       72        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g2_xyz = other.group2().xyz();
        let right_dual_g2_w = other[e12345] * -1.0;
        let wedge_g0 = (right_dual_g1_xyz * Simd32x3::from(self[e4])) + (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx());
        let wedge_g1 =
            ((right_dual_g2_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(other[e321]) * self.group0().xyz()) + (Simd32x3::from(self[e5]) * other.group0())).with_w(0.0);
        let wedge_g2_xyz = (right_dual_g1_xyz * Simd32x3::from(self[e5])) + (right_dual_g2_xyz.zxy() * self.group0().yzx()) - (right_dual_g2_xyz.yzx() * self.group0().zxy());
        let wedge_g3 = Simd32x4::from(right_dual_g2_w) * self.group0().xyz().with_w(self[e5]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            wedge_g0 * Simd32x3::from(other[e12345]),
            // e415, e425, e435, e321
            wedge_g1 * Simd32x4::from(other[e12345]),
            // e235, e315, e125, e4
            (wedge_g2_xyz * Simd32x4::from(other[e12345]).xyz()).with_w(
                (right_dual_g2_w * other[e12345] * self[e4])
                    - (wedge_g0[0] * other[e415])
                    - (wedge_g0[1] * other[e425])
                    - (wedge_g0[2] * other[e435])
                    - (other[e423] * wedge_g1[0])
                    - (other[e431] * wedge_g1[1])
                    - (other[e412] * wedge_g1[2]),
            ),
            // e1, e2, e3, e5
            ((Simd32x3::from(wedge_g1[3]) * other.group1().xyz())
                + (Simd32x3::from(other[e321]) * wedge_g1.xyz())
                + (Simd32x3::from(other[e12345]) * wedge_g3.xyz())
                + (wedge_g0.zxy() * other.group2().yzx())
                + (wedge_g2_xyz.yzx() * other.group0().zxy())
                - (wedge_g0.yzx() * other.group2().zxy())
                - (wedge_g2_xyz.zxy() * other.group0().yzx()))
            .with_w(wedge_g3[3] * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Dipole> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       12        0        0
    //    simd2        2        4        0      N/A
    //    simd3        7        8        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd       16       24        0      N/A
    //  no simd       38       44        0        0
    fn project_via_origin_onto(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0 = Simd32x4::from([0.0, 0.0, (other[e31] * self[e1]) - (other[e23] * self[e2]), 0.0])
            + (((other.group1().zx() * self.group0().yz()) - (other.group1().yz() * self.group0().zx())).with_z(0.0)
                - (right_dual_g0 * Simd32x3::from(self[e5]))
                - (Simd32x3::from(self[e4]) * other.group2()))
            .with_w(0.0);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, (wedge_g0[1] * other[e23]) - (wedge_g0[0] * other[e31]), 0.0])
                + ((Simd32x3::from(wedge_g0[3]) * other.group0()) + ((wedge_g0.zx() * other.group1().yz()) - (wedge_g0.yz() * other.group1().zx())).with_z(0.0)
                    - (Simd32x3::from(right_dual_g0[0] * self[e1]) * other.group2())
                    - (Simd32x3::from(right_dual_g0[1] * self[e2]) * other.group2())
                    - (Simd32x3::from(right_dual_g0[2] * self[e3]) * other.group2())
                    - (Simd32x3::from(other[e45] * self[e4]) * other.group2()))
                .with_w(0.0),
            // e5
            (other[e15] * wedge_g0[0]) + (other[e25] * wedge_g0[1]) + (other[e35] * wedge_g0[2]) + (wedge_g0[3] * other[e45]),
        )
    }
}
impl ProjectViaOriginOnto<DipoleInversion> for RoundPoint {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       24        0        0
    //    simd2        3        7        0      N/A
    //    simd3        8       13        0      N/A
    //    simd4        6        5        0      N/A
    // Totals...
    // yes simd       28       49        0      N/A
    //  no simd       65       97        0        0
    fn project_via_origin_onto(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0 = -(Simd32x3::from(other[e1234]) * self.group0().xyz()) - (Simd32x3::from(self[e4]) * other.group3().xyz());
        let wedge_g1 =
            (other.group3().yzxw() * self.group0().zxyw()) + -(other.group3().zx() * self.group0().yz()).with_zw(other[e4315] * self[e1] * -1.0, other[e1234] * self[e5] * -1.0);
        let wedge_g2_xyz = (Simd32x3::from(other[e3215]) * self.group0().xyz()) + (Simd32x3::from(self[e5]) * other.group3().xyz());
        let wedge_g2_w = right_dual_g0[0] * self[e1];
        let wedge_g3 = Simd32x4::from([0.0, 0.0, (other[e31] * self[e1]) - (other[e23] * self[e2]), 0.0])
            + (((other.group1().zx() * self.group0().yz()) - (other.group1().yz() * self.group0().zx())).with_z(0.0)
                - (right_dual_g0 * Simd32x3::from(self[e5]))
                - (Simd32x3::from(self[e4]) * other.group2().xyz()))
            .with_w(0.0);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other[e1234]) * wedge_g3.xyz()) - (Simd32x3::from(wedge_g2_w) * other.group3().xyz()),
            // e415, e425, e435, e321
            (wedge_g3.yzxw() * other.group3().zxy().with_w(other[e1234]))
                + -(wedge_g3.zx() * other.group3().yz()).with_zw(wedge_g3[1] * other[e4235] * -1.0, wedge_g2_w * other[e3215] * -1.0),
            // e235, e315, e125, e4
            ((Simd32x3::from(wedge_g3[3]) * other.group3().xyz()) - (Simd32x3::from(other[e3215]) * wedge_g3.xyz()))
                .with_w((wedge_g0[0] * other[e4235]) - (wedge_g2_w * other[e45]) - (other[e41] * wedge_g3[0]) - (other[e42] * wedge_g3[1]) - (other[e43] * wedge_g3[2])),
            // e1, e2, e3, e5
            (wedge_g3.zxyw() * other.group1().yzxw())
                + (wedge_g3.wwwy() * other.group0().with_w(other[e25]))
                + ((wedge_g2_xyz * Simd32x3::from(other[e1234]))
                    + ((wedge_g1.zx() * other.group3().yz()) - (wedge_g1.yz() * other.group3().zx()) - (wedge_g3.yz() * other.group1().zx()))
                        .with_z((wedge_g1[1] * other[e4235]) - (wedge_g1[0] * other[e4315]) - (wedge_g3[0] * other[e31]))
                    - (Simd32x3::from(wedge_g2_w) * other.group2().xyz()))
                .with_w(
                    (wedge_g3[0] * other[e15]) + (wedge_g3[2] * other[e35]) - (wedge_g2_xyz[0] * other[e4235]) - (wedge_g2_xyz[2] * other[e4125]) - (wedge_g1[3] * other[e3215]),
                )
                - (other.group3().wwwy() * wedge_g0.with_w(wedge_g2_xyz[1])),
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for RoundPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        8        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        9        0      N/A
    //  no simd        1       12        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_w = other[e5] * self[e4] * -1.0;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(wedge_g0_w * other[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w((wedge_g0_w * other[e5]) - (other[e12345] * other[e12345] * self[e5])),
            // e1, e2, e3, e4
            Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group0(),
        )
    }
}
impl ProjectViaOriginOnto<FlatPoint> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       14        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6       16        0      N/A
    //  no simd        6       21        0        0
    fn project_via_origin_onto(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(self[e4] * -1.0) * other.group0().xyz();
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e45] * self[e4] * -1.0) * other.group0(),
            // e5
            (wedge_g0_xyz[0] * other[e15])
                + (wedge_g0_xyz[1] * other[e25])
                + (wedge_g0_xyz[2] * other[e35])
                + (other[e15] * other[e45] * self[e1])
                + (other[e25] * other[e45] * self[e2])
                + (other[e35] * other[e45] * self[e3])
                - (other[e45] * other[e45] * self[e5]),
        )
    }
}
impl ProjectViaOriginOnto<Flector> for RoundPoint {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       25        0        0
    //    simd2        1        3        0      N/A
    //    simd3        6       12        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd       17       41        0      N/A
    //  no simd       36       71        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[e4] * -1.0) * other.group1().xyz();
        let wedge_g1_xy = (other.group1().yz() * self.group0().zx()) - (other.group1().zx() * self.group0().yz());
        let wedge_g1_z = (other[e4235] * self[e2]) - (other[e4315] * self[e1]);
        let wedge_g2_xyz = (Simd32x3::from(other[e3215]) * self.group0().xyz()) + (Simd32x3::from(self[e5]) * other.group1().xyz());
        let wedge_g2_w = other[e45] * self[e4];
        let wedge_g3_xyz = Simd32x3::from(self[e4] * -1.0) * other.group0().xyz();
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(wedge_g2_w * -1.0) * other.group1().xyz(),
            // e415, e425, e435, e321
            ((wedge_g3_xyz.yzx() * other.group1().zxy()) - (wedge_g3_xyz.zxy() * other.group1().yzx())).with_w(wedge_g2_w * other[e3215] * -1.0),
            // e235, e315, e125, e4
            ((Simd32x3::from(other[e15] * self[e1]) * other.group1().xyz())
                + (Simd32x3::from(other[e25] * self[e2]) * other.group1().xyz())
                + (Simd32x3::from(other[e35] * self[e3]) * other.group1().xyz())
                - (wedge_g3_xyz * Simd32x3::from(other[e3215]))
                - (Simd32x3::from(other[e45] * self[e5]) * other.group1().xyz()))
            .with_w(wedge_g0[0] * other[e4235]),
            // e1, e2, e3, e5
            Simd32x4::from([
                -(wedge_g2_w * other[e15]) - (wedge_g1_xy[1] * other[e4125]),
                -(wedge_g1_z * other[e4235]) - (wedge_g2_w * other[e25]),
                (wedge_g1_xy[1] * other[e4235]) - (wedge_g2_w * other[e35]) - (wedge_g1_xy[0] * other[e4315]),
                (wedge_g3_xyz[0] * other[e15]) - (wedge_g2_xyz[0] * other[e4235]) - (wedge_g2_xyz[1] * other[e4315]) - (other[e3215] * other[e3215] * self[e4]),
            ]) + (Simd32x2::from([wedge_g1_z, wedge_g1_xy[0]]) * other.group1().yz()).with_zw(0.0, 0.0)
                - (other.group1().wwwz() * wedge_g0.with_w(wedge_g2_xyz[2])),
        )
    }
}
impl ProjectViaOriginOnto<Line> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        6       13        0      N/A
    // Totals...
    // yes simd       11       19        0      N/A
    //  no simd       23       45        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[e4]) * other.group0();
        let wedge_g1_xyz = Simd32x3::from(self[e4]) * other.group1();
        let wedge_g2 = (Simd32x3::from(self[e5]) * other.group0()) + (other.group1().zxy() * self.group0().yzx()) - (other.group1().yzx() * self.group0().zxy());
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            ((wedge_g0.zxy() * other.group1().yzx())
                - (wedge_g0.yzx() * other.group1().zxy())
                - (other.group0() * other.group0() * self.group0().xyz())
                - (Simd32x3::from(other[e415]) * other.group0().yyz() * self.group0().yxx())
                - (Simd32x3::from(other[e435]) * other.group0().xyy() * self.group0().zzy()))
            .with_w(0.0),
            // e5
            -(wedge_g1_xyz[0] * other[e235])
                - (wedge_g1_xyz[1] * other[e315])
                - (wedge_g1_xyz[2] * other[e125])
                - (wedge_g2[0] * other[e415])
                - (wedge_g2[1] * other[e425])
                - (wedge_g2[2] * other[e435]),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for RoundPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       21        0        0
    //    simd3        5        9        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd       17       34        0      N/A
    //  no simd       33       64        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0 = (right_dual_g0_xyz * Simd32x4::from(self[e4]).xyz()).with_w(other[e5] * self[e4] * -1.0);
        let wedge_g1_xyz = right_dual_g1_xyz * Simd32x3::from(self[e4]);
        let wedge_g1_w = -(right_dual_g0_xyz[0] * self[e1]) - (right_dual_g0_xyz[1] * self[e2]) - (right_dual_g0_xyz[2] * self[e3]);
        let wedge_g2 = ((right_dual_g0_xyz * Simd32x3::from(self[e5])) + (right_dual_g1_xyz.zxy() * self.group0().yzx()) - (right_dual_g1_xyz.yzx() * self.group0().zxy()))
            .with_w(right_dual_g0_w * self[e5]);
        let wedge_g3 = Simd32x4::from(right_dual_g0_w) * self.group0();
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            wedge_g0 * Simd32x4::from(other[e12345]),
            // e415, e425, e435, e321
            ((wedge_g1_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0[3]) * other.group0().xyz())).with_w(wedge_g1_w * other[e12345]),
            // e235, e315, e125, e5
            (wedge_g2 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0[3]) * other.group1())
                + Simd32x3::from(0.0).with_w(
                    -(wedge_g1_xyz[0] * other[e235])
                        - (wedge_g1_xyz[1] * other[e315])
                        - (wedge_g1_xyz[2] * other[e125])
                        - (wedge_g2[0] * other[e415])
                        - (wedge_g2[1] * other[e425])
                        - (wedge_g2[2] * other[e435]),
                ),
            // e1, e2, e3, e4
            (Simd32x3::from([
                (wedge_g0[2] * other[e315]) - (wedge_g0[1] * other[e125]),
                (wedge_g0[0] * other[e125]) - (wedge_g0[2] * other[e235]),
                (wedge_g0[1] * other[e235]) - (wedge_g0[0] * other[e315]),
            ]) + (Simd32x3::from(wedge_g1_w) * other.group0().xyz())
                + (Simd32x3::from(other[e12345]) * wedge_g3.xyz()))
            .with_w(wedge_g3[3] * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for RoundPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       58       73        0        0
    //    simd2        0        2        0      N/A
    //    simd3       47       61        0      N/A
    //    simd4       10       13        0      N/A
    // Totals...
    // yes simd      115      149        0      N/A
    //  no simd      239      312        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1 = (other.group9().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let right_dual_g5 = other.group6().xyz();
        let right_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g9_xyz = other.group1().xyz();
        let wedge_g0_y =
            (right_dual_g9_xyz[0] * self[e1]) + (right_dual_g9_xyz[1] * self[e2]) + (right_dual_g9_xyz[2] * self[e3]) - (other[e4] * self[e5]) - (self[e4] * other[e5]);
        let wedge_g1 = Simd32x4::from(right_dual_g0[0]) * self.group0();
        let wedge_g2 = right_dual_g0[0] * self[e5];
        let wedge_g3 = (Simd32x4::from(other[e3215]) * self.group0()) - (right_dual_g1 * Simd32x4::from(self[e5]));
        let wedge_g4 = (Simd32x3::from(self[e4]) * right_dual_g1.xyz()) - (Simd32x3::from(right_dual_g1[3]) * self.group0().xyz());
        let wedge_g5 = Simd32x3::from([
            (right_dual_g1[2] * self[e2]) - (right_dual_g1[1] * self[e3]),
            (right_dual_g1[0] * self[e3]) - (right_dual_g1[2] * self[e1]),
            (right_dual_g1[1] * self[e1]) - (right_dual_g1[0] * self[e2]),
        ]);
        let wedge_g6 =
            ((Simd32x3::from(other[e321]) * self.group0().xyz()) + (Simd32x3::from(self[e4]) * other.group8()) + (Simd32x3::from(self[e5]) * other.group7())).with_w(0.0);
        let wedge_g7 = (right_dual_g5 * Simd32x3::from(self[e4])) + (other.group7().yzx() * self.group0().zxy()) - (other.group7().zxy() * self.group0().yzx());
        let wedge_g8 = (right_dual_g5 * Simd32x3::from(self[e5])) + (other.group8().zxy() * self.group0().yzx()) - (other.group8().yzx() * self.group0().zxy());
        let wedge_g9 = ((right_dual_g6_xyz.yzx() * self.group0().zxy())
            - (right_dual_g7 * Simd32x3::from(self[e5]))
            - (Simd32x3::from(self[e4]) * other.group3().xyz())
            - (right_dual_g6_xyz.zxy() * self.group0().yzx()))
        .with_w(0.0);
        let wedge_g10 = (right_dual_g7[0] * self[e1]) + (right_dual_g7[1] * self[e2]) + (right_dual_g7[2] * self[e3]) + (other[e45] * self[e4]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar])
                    + (wedge_g10 * other[e5])
                    + (wedge_g2 * other[e1234])
                    + (wedge_g1[0] * other[e4235])
                    + (wedge_g1[1] * other[e4315])
                    + (wedge_g1[2] * other[e4125])
                    + (wedge_g1[3] * other[e3215])
                    + (wedge_g9[0] * other[e1])
                    + (wedge_g9[1] * other[e2])
                    + (wedge_g9[2] * other[e3])
                    + (wedge_g9[3] * other[e4])
                    - (wedge_g4[0] * other[e235])
                    - (wedge_g4[1] * other[e315])
                    - (wedge_g4[2] * other[e125])
                    - (wedge_g5[0] * other[e415])
                    - (wedge_g5[1] * other[e425])
                    - (wedge_g5[2] * other[e435])
                    - (wedge_g7[0] * other[e15])
                    - (wedge_g7[1] * other[e25])
                    - (wedge_g7[2] * other[e35])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43])
                    - (other[e23] * wedge_g6[0])
                    - (other[e31] * wedge_g6[1])
                    - (other[e12] * wedge_g6[2])
                    - (other[e423] * wedge_g3[0])
                    - (other[e431] * wedge_g3[1])
                    - (other[e412] * wedge_g3[2])
                    - (wedge_g3[3] * other[e321])
                    - (wedge_g6[3] * other[e45]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            (wedge_g1 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group1())
                + (other.group9().yzxx() * wedge_g5.zxy().with_w(wedge_g4[0]))
                + ((Simd32x3::from(wedge_g6[3]) * other.group6().xyz())
                    + (Simd32x3::from(wedge_g9[3]) * other.group4())
                    + (Simd32x3::from(other[e321]) * wedge_g6.xyz())
                    + (Simd32x3::from(other[e1234]) * wedge_g3.xyz())
                    + (wedge_g7.zxy() * other.group8().yzx())
                    + (wedge_g8.yzx() * other.group7().zxy())
                    + (other.group5().yzx() * wedge_g9.zxy())
                    - (wedge_g4 * Simd32x3::from(other[e3215]))
                    - (Simd32x3::from(wedge_g10) * other.group3().xyz())
                    - (wedge_g5.yzx() * other.group9().zxy())
                    - (wedge_g7.yzx() * other.group8().zxy())
                    - (wedge_g8.zxy() * other.group7().yzx())
                    - (other.group5().zxy() * wedge_g9.yzx()))
                .with_w((wedge_g4[1] * other[e4315]) + (wedge_g4[2] * other[e4125]) + (wedge_g3[3] * other[e1234])),
            // e5
            (wedge_g0_y * other[e5])
                + (wedge_g2 * other[e12345])
                + (wedge_g9[0] * other[e15])
                + (wedge_g9[1] * other[e25])
                + (wedge_g9[2] * other[e35])
                + (wedge_g9[3] * other[e45])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435])
                - (other[e235] * wedge_g6[0])
                - (other[e315] * wedge_g6[1])
                - (other[e125] * wedge_g6[2])
                - (wedge_g3[0] * other[e4235])
                - (wedge_g3[1] * other[e4315])
                - (wedge_g3[2] * other[e4125])
                - (wedge_g3[3] * other[e3215]),
            // e15, e25, e35, e45
            (wedge_g3 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group3())
                + ((Simd32x3::from(wedge_g9[3]) * other.group6().xyz())
                    + (Simd32x3::from(other[e3215]) * wedge_g6.xyz())
                    + (wedge_g8.yzx() * other.group9().zxy())
                    + (other.group8().yzx() * wedge_g9.zxy())
                    - (wedge_g8.zxy() * other.group9().yzx())
                    - (other.group8().zxy() * wedge_g9.yzx()))
                .with_w(0.0),
            // e41, e42, e43
            (wedge_g4 * Simd32x3::from(other[e12345]))
                + (Simd32x3::from(wedge_g0_y) * other.group4())
                + (Simd32x3::from(wedge_g10) * other.group6().xyz())
                + (Simd32x3::from(other[e1234]) * wedge_g6.xyz())
                + (wedge_g7.zxy() * other.group9().yzx())
                + (other.group7().zxy() * wedge_g9.yzx())
                - (wedge_g7.yzx() * other.group9().zxy())
                - (other.group7().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (wedge_g5 * Simd32x3::from(other[e12345]))
                + (wedge_g7 * Simd32x3::from(other[e3215]))
                + (wedge_g8 * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(wedge_g0_y) * other.group5())
                + (Simd32x3::from(wedge_g10) * other.group8())
                + (Simd32x3::from(wedge_g9[3]) * other.group7())
                - (Simd32x3::from(wedge_g6[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            (wedge_g6 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group6())
                + (wedge_g9.yzxw() * other.group9().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group9().yz()).with_zw(wedge_g9[1] * other[e4235] * -1.0, wedge_g10 * other[e3215] * -1.0),
            // e423, e431, e412
            (wedge_g7 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group7()) + (Simd32x3::from(other[e1234]) * wedge_g9.xyz())
                - (Simd32x3::from(wedge_g10) * other.group9().xyz()),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (wedge_g9 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group9()),
            // e1234
            (wedge_g0_y * other[e1234]) + (wedge_g10 * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       15        0        0
    //    simd2        1        2        0      N/A
    //    simd3        2        4        0      N/A
    // Totals...
    // yes simd       10       21        0      N/A
    //  no simd       15       31        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[e4] * -1.0) * other.group0().xyz();
        let wedge_g1_xy = (other.group0().yz() * self.group0().zx()) - (other.group0().zx() * self.group0().yz());
        let wedge_g1_z = (other[e4235] * self[e2]) - (other[e4315] * self[e1]);
        let wedge_g2 = (Simd32x3::from(other[e3215]) * self.group0().xyz()) + (Simd32x3::from(self[e5]) * other.group0().xyz());
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from([
                (wedge_g1_z * other[e4315]) - (wedge_g1_xy[1] * other[e4125]),
                (wedge_g1_xy[0] * other[e4125]) - (wedge_g1_z * other[e4235]),
                (wedge_g1_xy[1] * other[e4235]) - (wedge_g1_xy[0] * other[e4315]),
            ]) - (wedge_g0 * Simd32x3::from(other[e3215])))
            .with_w(wedge_g0[0] * other[e4235]),
            // e5
            -(wedge_g2[0] * other[e4235]) - (wedge_g2[1] * other[e4315]) - (wedge_g2[2] * other[e4125]) - (other[e3215] * other[e3215] * self[e4]),
        )
    }
}
impl ProjectViaOriginOnto<RoundPoint> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd        4       10        0        0
    fn project_via_origin_onto(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let wedge_g0 = (right_dual_g0_xyz[0] * self[e1]) + (right_dual_g0_xyz[1] * self[e2]) + (right_dual_g0_xyz[2] * self[e3]) - (other[e4] * self[e5]) - (self[e4] * other[e5]);
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(wedge_g0) * other.group0(), /* e5 */ wedge_g0 * other[e5])
    }
}
impl ProjectViaOriginOnto<Sphere> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       10        0        0
    //    simd2        1        2        0      N/A
    //    simd3        4        8        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       11       21        0      N/A
    //  no simd       23       42        0        0
    fn project_via_origin_onto(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = (right_dual_g0_xyz * Simd32x3::from(self[e4])) - (Simd32x3::from(other[e1234]) * self.group0().xyz());
        let wedge_g1 =
            (self.group0().yzxw() * right_dual_g0_xyz.zxy().with_w(other[e3215])) + -(right_dual_g0_xyz.yzx() * self.group0().zxy()).with_w(self[e5] * other[e1234] * -1.0);
        let wedge_g2 = (Simd32x3::from(other[e3215]) * self.group0().xyz()) - (right_dual_g0_xyz * Simd32x3::from(self[e5]));
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            ((wedge_g2 * Simd32x3::from(other[e1234]))
                + ((wedge_g1.zx() * other.group0().yz()) - (wedge_g1.yz() * other.group0().zx())).with_z((wedge_g1[1] * other[e4235]) - (wedge_g1[0] * other[e4315]))
                - (wedge_g0 * Simd32x3::from(other[e3215])))
            .with_w((wedge_g0[0] * other[e4235]) + (wedge_g0[1] * other[e4315])),
            // e5
            -(wedge_g2[0] * other[e4235]) - (wedge_g2[1] * other[e4315]) - (wedge_g2[2] * other[e4125]) - (wedge_g1[3] * other[e3215]),
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for RoundPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       13        0        0
    //    simd2        2        3        0      N/A
    //    simd3        9       13        0      N/A
    //    simd4        6        8        0      N/A
    // Totals...
    // yes simd       24       37        0      N/A
    //  no simd       62       90        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g2_xyz = other.group2().xyz();
        let right_dual_g3_xyz = other.group3().xyz();
        let wedge_g0 = (self.group0().zxyx() * right_dual_g0_xyz.yzx().with_w(right_dual_g3_xyz[0]))
            + (self.group0().wwwy() * right_dual_g1_xyz.with_w(right_dual_g3_xyz[1]))
            + -(right_dual_g0_xyz.zxy() * self.group0().yzx()).with_w(0.0);
        let wedge_g1 =
            ((right_dual_g0_xyz * Simd32x3::from(self[e5])) + (right_dual_g2_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(other[e321]) * self.group0().xyz())).with_w(0.0);
        let wedge_g2 = ((right_dual_g1_xyz * Simd32x3::from(self[e5])) + (right_dual_g2_xyz.zxy() * self.group0().yzx()) - (right_dual_g2_xyz.yzx() * self.group0().zxy()))
            .with_w(right_dual_g0_w * self[e5]);
        let wedge_g3 = Simd32x4::from(right_dual_g0_w) * self.group0();
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x3::from(wedge_g0[3]) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g0.xyz())).with_w(wedge_g0[3] * other[e12345]),
            // e415, e425, e435, e321
            (wedge_g1 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0[3]) * other.group1()),
            // e235, e315, e125, e5
            (wedge_g2 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0[3]) * other.group2())
                + Simd32x3::from(0.0).with_w(
                    -(wedge_g1[0] * other[e235])
                        - (wedge_g1[1] * other[e315])
                        - (wedge_g1[2] * other[e125])
                        - (wedge_g2[0] * other[e415])
                        - (wedge_g2[1] * other[e425])
                        - (wedge_g2[2] * other[e435]),
                ),
            // e1, e2, e3, e4
            (wedge_g0.zxyw() * other.group2().yzx().with_w(other[e4]))
                + ((Simd32x3::from(wedge_g0[3]) * other.group3().xyz())
                    + (Simd32x3::from(wedge_g1[3]) * other.group1().xyz())
                    + (Simd32x3::from(other[e12345]) * wedge_g3.xyz())
                    + (Simd32x3::from(other[e321]) * wedge_g1.xyz())
                    + ((wedge_g2.yz() * other.group0().zx()) - (wedge_g0.yz() * other.group2().zx()) - (wedge_g2.zx() * other.group0().yz()))
                        .with_z((wedge_g2[0] * other[e431]) - (wedge_g0[0] * other[e315]) - (wedge_g2[1] * other[e423])))
                .with_w(wedge_g3[3] * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<VersorOdd> for RoundPoint {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       23        0        0
    //    simd2        3        6        0      N/A
    //    simd3        8       14        0      N/A
    //    simd4        6        5        0      N/A
    // Totals...
    // yes simd       28       48        0      N/A
    //  no simd       65       97        0        0
    fn project_via_origin_onto(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = (right_dual_g3_xyz * Simd32x3::from(self[e4])) - (Simd32x3::from(other[e1234]) * self.group0().xyz());
        let wedge_g1 =
            (self.group0().yzxw() * right_dual_g3_xyz.zxy().with_w(other[e3215])) + -(right_dual_g3_xyz.yzx() * self.group0().zxy()).with_w(other[e1234] * self[e5] * -1.0);
        let wedge_g2_xyz = (Simd32x3::from(other[e3215]) * self.group0().xyz()) - (right_dual_g3_xyz * Simd32x3::from(self[e5]));
        let wedge_g2_w = self[e1] * other[e41] * -1.0;
        let wedge_g3 = Simd32x4::from([0.0, 0.0, (self[e1] * other[e31]) - (self[e2] * other[e23]), 0.0])
            + ((Simd32x3::from(self[e5]) * other.group0().xyz()) + ((self.group0().yz() * other.group1().zx()) - (self.group0().zx() * other.group1().yz())).with_z(0.0)
                - (Simd32x3::from(self[e4]) * other.group2().xyz()))
            .with_w(0.0);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other[e1234]) * wedge_g3.xyz()) - (Simd32x3::from(wedge_g2_w) * other.group3().xyz()),
            // e415, e425, e435, e321
            (wedge_g3.yzxw() * other.group3().zxy().with_w(other[e1234]))
                + -(wedge_g3.zx() * other.group3().yz()).with_zw(wedge_g3[1] * other[e4235] * -1.0, wedge_g2_w * other[e3215] * -1.0),
            // e235, e315, e125, e4
            ((Simd32x3::from(wedge_g3[3]) * other.group3().xyz()) - (Simd32x3::from(other[e3215]) * wedge_g3.xyz()))
                .with_w((wedge_g0[0] * other[e4235]) - (wedge_g2_w * other[e45]) - (wedge_g3[0] * other[e41]) - (wedge_g3[1] * other[e42]) - (wedge_g3[2] * other[e43])),
            // e1, e2, e3, e5
            (Simd32x4::from(wedge_g3[3]) * other.group0().xyz().with_w(other[e45]))
                + (wedge_g3.zxyz() * other.group1().yzx().with_w(other[e35]))
                + ((wedge_g2_xyz * Simd32x3::from(other[e1234]))
                    + ((wedge_g1.zx() * other.group3().yz()) - (wedge_g1.yz() * other.group3().zx()) - (wedge_g3.yz() * other.group1().zx()))
                        .with_z((wedge_g1[1] * other[e4235]) - (wedge_g1[0] * other[e4315]) - (wedge_g3[0] * other[e31]))
                    - (Simd32x3::from(wedge_g2_w) * other.group2().xyz()))
                .with_w(
                    (wedge_g3[0] * other[e15]) + (wedge_g3[1] * other[e25]) - (wedge_g2_xyz[0] * other[e4235]) - (wedge_g2_xyz[2] * other[e4125]) - (wedge_g1[3] * other[e3215]),
                )
                - (other.group3().wwwy() * wedge_g0.with_w(wedge_g2_xyz[1])),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for Scalar {
    type Output = ProjectViaOriginOntoInfixPartial<Scalar>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiCircleRotor> for Scalar {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       12        0        0
    //    simd3        0        3        0      N/A
    //    simd4        0        5        0      N/A
    // Totals...
    // yes simd       10       20        0      N/A
    //  no simd       10       41        0        0
    fn project_via_origin_onto(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[scalar] * -1.0) * other.group0();
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g2 = Simd32x4::from(self[scalar]) * other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g2[3]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(wedge_g2[3]) * other.group1(),
            // e15, e25, e35, scalar
            (Simd32x4::from(wedge_g2[3]).xyz() * other.group2().xyz()).with_w(
                (wedge_g2[3] * other[scalar])
                    - (wedge_g0[0] * other[e15])
                    - (wedge_g0[1] * other[e25])
                    - (wedge_g0[2] * other[e35])
                    - (other[e41] * wedge_g2[0])
                    - (other[e42] * wedge_g2[1])
                    - (other[e43] * wedge_g2[2])
                    - (wedge_g1[0] * other[e23])
                    - (wedge_g1[1] * other[e31])
                    - (wedge_g1[2] * other[e12])
                    - (wedge_g1[3] * other[e45]),
            ),
        )
    }
}
impl ProjectViaOriginOnto<AntiDipoleInversion> for Scalar {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       12        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4        7        0      N/A
    //    simd4        2        8        0      N/A
    // Totals...
    // yes simd       15       28        0      N/A
    //  no simd       29       67        0        0
    fn project_via_origin_onto(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[scalar]) * other.group0();
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let wedge_g2 = Simd32x4::from(self[scalar]) * other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let wedge_g3 = Simd32x4::from(self[scalar]) * other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(wedge_g2[3]) * other.group1().xyz()) + (other.group0().zxy() * wedge_g3.yzx()) - (other.group0().yzx() * wedge_g3.zxy()),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g2[3]) * other.group2().xyz()) + (Simd32x3::from(wedge_g3[3]) * other.group0()) - (Simd32x3::from(other[e321]) * wedge_g3.xyz())).with_w(0.0),
            // e15, e25, e35, scalar
            (Simd32x4::from(wedge_g3[3]) * other.group1().xyz().with_w(other[e4]))
                + (wedge_g3.zxyx() * other.group2().yzx().with_w(other[e1]))
                + -(wedge_g3.yz() * other.group2().zx()).with_zw(
                    wedge_g3[0] * other[e315] * -1.0,
                    -(wedge_g0[0] * other[e235])
                        - (wedge_g0[1] * other[e315])
                        - (wedge_g0[2] * other[e125])
                        - (other[e423] * wedge_g2[0])
                        - (other[e431] * wedge_g2[1])
                        - (other[e412] * wedge_g2[2])
                        - (wedge_g1[0] * other[e415])
                        - (wedge_g1[1] * other[e425])
                        - (wedge_g1[2] * other[e435])
                        - (wedge_g1[3] * other[e321]),
                ),
        )
    }
}
impl ProjectViaOriginOnto<AntiDualNum> for Scalar {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        3        0        0
    fn project_via_origin_onto(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(other[scalar] * self[scalar]) * other.group0())
    }
}
impl ProjectViaOriginOnto<AntiFlatPoint> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn project_via_origin_onto(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e321] * other[e321] * self[scalar])
    }
}
impl ProjectViaOriginOnto<AntiFlector> for Scalar {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        7        0        0
    //    simd2        1        2        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd        6       13        0      N/A
    //  no simd       10       25        0        0
    fn project_via_origin_onto(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (wedge_g1.xyz() * Simd32x4::from(other[e321]).xyz() * Simd32x3::from(-1.0))
                .with_w((wedge_g1[0] * other[e1]) + (wedge_g1[1] * other[e2]) + (wedge_g1[2] * other[e3]) + (other[e321] * other[e321] * self[scalar])),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, (wedge_g1[1] * other[e235]) - (wedge_g1[0] * other[e315]), 0.0])
                + ((wedge_g1.zx() * other.group0().yz()) - (wedge_g1.yz() * other.group0().zx())).with_zw(0.0, 0.0),
        )
    }
}
impl ProjectViaOriginOnto<AntiLine> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        7        0        0
    fn project_via_origin_onto(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[scalar] * -1.0) * other.group0();
        Scalar::from_groups(/* scalar */ -(wedge_g0[0] * other[e23]) - (wedge_g0[1] * other[e31]) - (wedge_g0[2] * other[e12]))
    }
}
impl ProjectViaOriginOnto<AntiMotor> for Scalar {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        1        3        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd        6       22        0        0
    fn project_via_origin_onto(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x4::from(wedge_g0[3]).xyz() * other.group0().xyz())
                .with_w((wedge_g0[3] * other[scalar]) - (wedge_g0[0] * other[e23]) - (wedge_g0[1] * other[e31]) - (wedge_g0[2] * other[e12])),
            // e15, e25, e35, e3215
            ((Simd32x3::from(wedge_g0[3]) * other.group1().xyz()) + (Simd32x3::from(other[e3215]) * wedge_g0.xyz())).with_w(wedge_g0[3] * other[e3215]),
        )
    }
}
impl ProjectViaOriginOnto<AntiPlane> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2       11        0        0
    fn project_via_origin_onto(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(/* scalar */ (wedge_g0[0] * other[e1]) + (wedge_g0[1] * other[e2]) + (wedge_g0[2] * other[e3]))
    }
}
impl ProjectViaOriginOnto<AntiScalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e12345] * other[e12345] * self[scalar] * -1.0)
    }
}
impl ProjectViaOriginOnto<Circle> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       10        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd        9       24        0        0
    fn project_via_origin_onto(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[scalar]) * other.group0();
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let wedge_g2 = Simd32x3::from(self[scalar]) * other.group2();
        Scalar::from_groups(
            // scalar
            -(wedge_g0[0] * other[e235])
                - (wedge_g0[1] * other[e315])
                - (wedge_g0[2] * other[e125])
                - (wedge_g2[0] * other[e423])
                - (wedge_g2[1] * other[e431])
                - (wedge_g2[2] * other[e412])
                - (wedge_g1[0] * other[e415])
                - (wedge_g1[1] * other[e425])
                - (wedge_g1[2] * other[e435])
                - (wedge_g1[3] * other[e321]),
        )
    }
}
impl ProjectViaOriginOnto<CircleRotor> for Scalar {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       11        0        0
    //    simd3        0        3        0      N/A
    //    simd4        0        5        0      N/A
    // Totals...
    // yes simd       10       19        0      N/A
    //  no simd       10       40        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[scalar]) * other.group0();
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let wedge_g2 = Simd32x4::from(self[scalar]) * other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            wedge_g0 * Simd32x3::from(other[e12345]),
            // e23, e31, e12, e45
            wedge_g1 * Simd32x4::from(other[e12345]),
            // e15, e25, e35, scalar
            (wedge_g2.xyz() * Simd32x4::from(other[e12345]).xyz()).with_w(
                (wedge_g2[3] * other[e12345])
                    - (wedge_g0[0] * other[e235])
                    - (wedge_g0[1] * other[e315])
                    - (wedge_g0[2] * other[e125])
                    - (other[e423] * wedge_g2[0])
                    - (other[e431] * wedge_g2[1])
                    - (other[e412] * wedge_g2[2])
                    - (wedge_g1[0] * other[e415])
                    - (wedge_g1[1] * other[e425])
                    - (wedge_g1[2] * other[e435])
                    - (wedge_g1[3] * other[e321]),
            ),
        )
    }
}
impl ProjectViaOriginOnto<Dipole> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       12        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        9       16        0      N/A
    //  no simd        9       26        0        0
    fn project_via_origin_onto(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[scalar] * -1.0) * other.group0();
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g2 = Simd32x3::from(self[scalar] * -1.0) * other.group2();
        Scalar::from_groups(
            // scalar
            -(wedge_g0[0] * other[e15])
                - (wedge_g0[1] * other[e25])
                - (wedge_g0[2] * other[e35])
                - (wedge_g2[0] * other[e41])
                - (wedge_g2[1] * other[e42])
                - (wedge_g2[2] * other[e43])
                - (wedge_g1[0] * other[e23])
                - (wedge_g1[1] * other[e31])
                - (wedge_g1[2] * other[e12])
                - (wedge_g1[3] * other[e45]),
        )
    }
}
impl ProjectViaOriginOnto<DipoleInversion> for Scalar {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       10        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4        7        0      N/A
    //    simd4        2        6        0      N/A
    // Totals...
    // yes simd       12       24        0      N/A
    //  no simd       26       57        0        0
    fn project_via_origin_onto(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[scalar] * -1.0) * other.group0();
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g2 = Simd32x4::from(self[scalar]) * other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * wedge_g1.xyz()) + (wedge_g0.zxy() * other.group3().yzx()) - (wedge_g0.yzx() * other.group3().zxy()),
            // e23, e31, e12, e45
            ((wedge_g0 * Simd32x3::from(other[e3215])) + (Simd32x3::from(other[e1234]) * wedge_g2.xyz()) - (Simd32x3::from(wedge_g1[3]) * other.group3().xyz())).with_w(0.0),
            // e15, e25, e35, scalar
            (Simd32x4::from(other[e3215]) * wedge_g1.xyz().with_w(wedge_g2[3]))
                + (wedge_g2.yz() * other.group3().zx()).with_zw(
                    wedge_g2[0] * other[e4315],
                    -(other[e42] * wedge_g2[1])
                        - (other[e43] * wedge_g2[2])
                        - (wedge_g1[0] * other[e23])
                        - (wedge_g1[1] * other[e31])
                        - (wedge_g1[2] * other[e12])
                        - (wedge_g1[3] * other[e45])
                        - (other[e4235] * other[e4235] * self[scalar]),
                )
                - (wedge_g2.zxyx() * other.group3().yzx().with_w(other[e41])),
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for Scalar {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        4        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(other[e12345] * self[scalar] * -1.0) * other.group0())
    }
}
impl ProjectViaOriginOnto<FlatPoint> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn project_via_origin_onto(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e45] * other[e45] * self[scalar] * -1.0)
    }
}
impl ProjectViaOriginOnto<Flector> for Scalar {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd2        1        2        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        1        4        0      N/A
    // Totals...
    // yes simd        6       14        0      N/A
    //  no simd       10       32        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x4::from(wedge_g0[3]).xyz() * other.group1().xyz() * Simd32x3::from(-1.0))
                .with_w((wedge_g1[0] * other[e4235]) + (wedge_g1[1] * other[e4315]) + (wedge_g1[2] * other[e4125]) - (wedge_g0[3] * other[e45])),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, (wedge_g0[0] * other[e4315]) - (wedge_g0[1] * other[e4235]), 0.0])
                + ((wedge_g0.yz() * other.group1().zx()) - (wedge_g0.zx() * other.group1().yz())).with_zw(0.0, 0.0),
        )
    }
}
impl ProjectViaOriginOnto<Line> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[scalar]) * other.group0();
        Scalar::from_groups(/* scalar */ -(wedge_g0[0] * other[e415]) - (wedge_g0[1] * other[e425]) - (wedge_g0[2] * other[e435]))
    }
}
impl ProjectViaOriginOnto<Motor> for Scalar {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        1        3        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        4       12        0      N/A
    //  no simd        6       30        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (wedge_g0.xyz() * Simd32x4::from(other[e12345]).xyz())
                .with_w((wedge_g0[3] * other[e12345]) - (wedge_g0[0] * other[e415]) - (wedge_g0[1] * other[e425]) - (wedge_g0[2] * other[e435])),
            // e15, e25, e35, e3215
            ((Simd32x3::from(wedge_g1[3]) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g1.xyz())).with_w(wedge_g1[3] * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       49       67        0        0
    //    simd2        0        3        0      N/A
    //    simd3       37       50        0      N/A
    //    simd4        9       13        0      N/A
    // Totals...
    // yes simd       95      133        0      N/A
    //  no simd      196      275        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x2::from(self[scalar]) * other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let wedge_g1 = (Simd32x3::from(self[scalar] * -1.0) * other.group9().xyz()).with_w(other[e1234] * self[scalar]);
        let wedge_g2 = other[e3215] * self[scalar];
        let wedge_g3 = Simd32x4::from(self[scalar]) * other.group8().with_w(other[e321]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let wedge_g4 = Simd32x3::from(self[scalar]) * other.group7();
        let wedge_g5 = Simd32x3::from(self[scalar]) * other.group6().xyz();
        let wedge_g6 = Simd32x4::from(self[scalar]) * (other.group5() * Simd32x3::from(-1.0)).with_w(other[e45]);
        let wedge_g7 = Simd32x3::from(self[scalar] * -1.0) * other.group4();
        let wedge_g8 = Simd32x3::from(self[scalar] * -1.0) * other.group3().xyz();
        let wedge_g9 = (Simd32x3::from(self[scalar]) * other.group1().xyz()).with_w(other[e5] * self[scalar] * -1.0);
        let wedge_g10 = other[e4] * self[scalar] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g10 * other[e5])
                    + (wedge_g2 * other[e1234])
                    + (wedge_g0[0] * other[e12345])
                    + (wedge_g0[1] * other[scalar])
                    + (wedge_g1[0] * other[e4235])
                    + (wedge_g1[1] * other[e4315])
                    + (wedge_g1[2] * other[e4125])
                    + (wedge_g1[3] * other[e3215])
                    + (wedge_g9[0] * other[e1])
                    + (wedge_g9[1] * other[e2])
                    + (wedge_g9[2] * other[e3])
                    + (wedge_g9[3] * other[e4])
                    - (wedge_g4[0] * other[e235])
                    - (wedge_g4[1] * other[e315])
                    - (wedge_g4[2] * other[e125])
                    - (wedge_g5[0] * other[e415])
                    - (wedge_g5[1] * other[e425])
                    - (wedge_g5[2] * other[e435])
                    - (wedge_g7[0] * other[e15])
                    - (wedge_g7[1] * other[e25])
                    - (wedge_g7[2] * other[e35])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43])
                    - (other[e23] * wedge_g6[0])
                    - (other[e31] * wedge_g6[1])
                    - (other[e12] * wedge_g6[2])
                    - (other[e423] * wedge_g3[0])
                    - (other[e431] * wedge_g3[1])
                    - (other[e412] * wedge_g3[2])
                    - (wedge_g3[3] * other[e321])
                    - (wedge_g6[3] * other[e45]),
                wedge_g0[1] * other[e12345],
            ]),
            // e1, e2, e3, e4
            (wedge_g1 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0[1]) * other.group1())
                + (other.group9().yzxx() * wedge_g5.zxy().with_w(wedge_g4[0]))
                + ((Simd32x3::from(wedge_g6[3]) * other.group6().xyz())
                    + (Simd32x3::from(wedge_g9[3]) * other.group4())
                    + (Simd32x3::from(other[e321]) * wedge_g6.xyz())
                    + (Simd32x3::from(other[e1234]) * wedge_g3.xyz())
                    + (wedge_g7.zxy() * other.group8().yzx())
                    + (wedge_g8.yzx() * other.group7().zxy())
                    + (other.group5().yzx() * wedge_g9.zxy())
                    - (wedge_g4 * Simd32x3::from(other[e3215]))
                    - (Simd32x3::from(wedge_g10) * other.group3().xyz())
                    - (wedge_g5.yzx() * other.group9().zxy())
                    - (wedge_g7.yzx() * other.group8().zxy())
                    - (wedge_g8.zxy() * other.group7().yzx())
                    - (other.group5().zxy() * wedge_g9.yzx()))
                .with_w((wedge_g4[1] * other[e4315]) + (wedge_g4[2] * other[e4125]) + (wedge_g3[3] * other[e1234])),
            // e5
            (wedge_g2 * other[e12345])
                + (wedge_g0[1] * other[e5])
                + (wedge_g9[0] * other[e15])
                + (wedge_g9[1] * other[e25])
                + (wedge_g9[2] * other[e35])
                + (wedge_g9[3] * other[e45])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435])
                - (other[e235] * wedge_g6[0])
                - (other[e315] * wedge_g6[1])
                - (other[e125] * wedge_g6[2])
                - (wedge_g3[0] * other[e4235])
                - (wedge_g3[1] * other[e4315])
                - (wedge_g3[2] * other[e4125])
                - (wedge_g3[3] * other[e3215]),
            // e15, e25, e35, e45
            (wedge_g3 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0[1]) * other.group3())
                + ((Simd32x3::from(wedge_g9[3]) * other.group6().xyz())
                    + (Simd32x3::from(other[e3215]) * wedge_g6.xyz())
                    + (wedge_g8.yzx() * other.group9().zxy())
                    + (other.group8().yzx() * wedge_g9.zxy())
                    - (wedge_g8.zxy() * other.group9().yzx())
                    - (other.group8().zxy() * wedge_g9.yzx()))
                .with_w(0.0),
            // e41, e42, e43
            (wedge_g4 * Simd32x3::from(other[e12345]))
                + (Simd32x3::from(wedge_g10) * other.group6().xyz())
                + (Simd32x3::from(wedge_g0[1]) * other.group4())
                + (Simd32x3::from(other[e1234]) * wedge_g6.xyz())
                + (wedge_g7.zxy() * other.group9().yzx())
                + (other.group7().zxy() * wedge_g9.yzx())
                - (wedge_g7.yzx() * other.group9().zxy())
                - (other.group7().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (wedge_g5 * Simd32x3::from(other[e12345]))
                + (wedge_g7 * Simd32x3::from(other[e3215]))
                + (wedge_g8 * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(wedge_g10) * other.group8())
                + (Simd32x3::from(wedge_g0[1]) * other.group5())
                + (Simd32x3::from(wedge_g9[3]) * other.group7())
                - (Simd32x3::from(wedge_g6[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            (wedge_g6 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0[1]) * other.group6())
                + (wedge_g9.yzxw() * other.group9().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group9().yz()).with_zw(wedge_g9[1] * other[e4235] * -1.0, wedge_g10 * other[e3215] * -1.0),
            // e423, e431, e412
            (wedge_g7 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0[1]) * other.group7()) + (Simd32x3::from(other[e1234]) * wedge_g9.xyz())
                - (Simd32x3::from(wedge_g10) * other.group9().xyz()),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0[1]) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (wedge_g9 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0[1]) * other.group9()),
            // e1234
            (wedge_g10 * other[e12345]) + (wedge_g0[1] * other[e1234]),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2       11        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Scalar::from_groups(/* scalar */ (wedge_g0[0] * other[e4235]) + (wedge_g0[1] * other[e4315]) + (wedge_g0[2] * other[e4125]))
    }
}
impl ProjectViaOriginOnto<RoundPoint> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3        9        0        0
    fn project_via_origin_onto(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(self[scalar]) * other.group0().xyz();
        Scalar::from_groups(
            // scalar
            (wedge_g0_xyz[0] * other[e1]) + (wedge_g0_xyz[1] * other[e2]) + (wedge_g0_xyz[2] * other[e3]) - 2.0 * (other[e4] * other[e5] * self[scalar]),
        )
    }
}
impl ProjectViaOriginOnto<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn project_via_origin_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[scalar] * other[scalar] * self[scalar])
    }
}
impl ProjectViaOriginOnto<Sphere> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        4       13        0        0
    fn project_via_origin_onto(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        Scalar::from_groups(
            // scalar
            (wedge_g0[0] * other[e4235])
                + (wedge_g0[1] * other[e4315])
                + (wedge_g0[2] * other[e4125])
                + (wedge_g0[3] * other[e3215])
                + (other[e3215] * self[scalar] * other[e1234]),
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for Scalar {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       20        0        0
    //    simd2        0        1        0      N/A
    //    simd3        5        6        0      N/A
    //    simd4        3       12        0      N/A
    // Totals...
    // yes simd       20       39        0      N/A
    //  no simd       39       88        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let wedge_g2 = Simd32x4::from(self[scalar]) * other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let wedge_g3 = Simd32x4::from(self[scalar]) * other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (wedge_g0 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g2[3]) * other.group1().xyz().with_w(other[e5]))
                + (wedge_g3.yzxx() * other.group0().zxy().with_w(other[e1]))
                + -(wedge_g3.zx() * other.group0().yz()).with_zw(
                    wedge_g3[1] * other[e423] * -1.0,
                    -(wedge_g0[0] * other[e235])
                        - (wedge_g0[1] * other[e315])
                        - (wedge_g0[2] * other[e125])
                        - (wedge_g1[0] * other[e415])
                        - (wedge_g1[1] * other[e425])
                        - (wedge_g1[2] * other[e435])
                        - (wedge_g1[3] * other[e321])
                        - (wedge_g2[0] * other[e423])
                        - (wedge_g2[1] * other[e431])
                        - (wedge_g2[2] * other[e412]),
                ),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g2[3]) * other.group2().xyz()) + (Simd32x3::from(wedge_g3[3]) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g1.xyz())
                - (Simd32x3::from(other[e321]) * wedge_g3.xyz()))
            .with_w(wedge_g1[3] * other[e12345]),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g3[2] * other[e315]) - (wedge_g3[1] * other[e125]),
                (wedge_g3[0] * other[e125]) - (wedge_g3[2] * other[e235]),
                (wedge_g3[1] * other[e235]) - (wedge_g3[0] * other[e315]),
            ]) + (Simd32x3::from(wedge_g3[3]) * other.group1().xyz())
                + (Simd32x3::from(other[e12345]) * wedge_g2.xyz()))
            .with_w(wedge_g2[3] * other[e12345]),
            // e4235, e4315, e4125, e3215
            wedge_g3 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<VersorOdd> for Scalar {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       20        0        0
    //    simd2        0        1        0      N/A
    //    simd3        7        9        0      N/A
    //    simd4        1        7        0      N/A
    // Totals...
    // yes simd       19       37        0      N/A
    //  no simd       36       77        0        0
    fn project_via_origin_onto(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g2 = Simd32x4::from(self[scalar]) * (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(other[e3215]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (wedge_g0.zxyw() * other.group3().yzx().with_w(other[scalar]))
                + ((Simd32x3::from(wedge_g0[3]) * other.group0().xyz())
                    + (Simd32x3::from(other[e1234]) * wedge_g1.xyz())
                    + -(wedge_g0.yz() * other.group3().zx()).with_z(wedge_g0[0] * other[e4315] * -1.0))
                .with_w(
                    (wedge_g2[3] * other[e1234])
                        - (wedge_g1[0] * other[e23])
                        - (wedge_g1[1] * other[e31])
                        - (wedge_g1[2] * other[e12])
                        - (wedge_g1[3] * other[e45])
                        - (wedge_g2[0] * other[e41])
                        - (wedge_g2[1] * other[e42])
                        - (wedge_g2[2] * other[e43])
                        - (other[e4235] * other[e4235] * self[scalar]),
                ),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g0[3]) * other.group1().xyz()) + (Simd32x3::from(other[e1234]) * wedge_g2.xyz()) + (Simd32x3::from(other[e3215]) * wedge_g0.xyz())
                - (Simd32x3::from(wedge_g1[3]) * other.group3().xyz()))
            .with_w(wedge_g0[3] * other[e45]),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g2[1] * other[e4125]) - (wedge_g2[2] * other[e4315]),
                (wedge_g2[2] * other[e4235]) - (wedge_g2[0] * other[e4125]),
                (wedge_g2[0] * other[e4315]) - (wedge_g2[1] * other[e4235]),
            ]) + (Simd32x3::from(wedge_g0[3]) * other.group2().xyz())
                + (Simd32x3::from(other[e3215]) * wedge_g1.xyz()))
            .with_w(wedge_g0[3] * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0[3]) * other.group3(),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for Sphere {
    type Output = ProjectViaOriginOntoInfixPartial<Sphere>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiDualNum> for Sphere {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        3        0        0
    fn project_via_origin_onto(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(other[e3215] * self[e1234]) * other.group0())
    }
}
impl ProjectViaOriginOnto<AntiMotor> for Sphere {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        9        0        0
    fn project_via_origin_onto(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = other[e3215] * self[e1234];
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(wedge_g0) * other.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(wedge_g0) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<AntiScalar> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0        8        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e12345] * -1.0;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group0(),
            // e1234
            right_dual_g0 * other[e12345] * self[e1234],
        )
    }
}
impl ProjectViaOriginOnto<CircleRotor> for Sphere {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        9        0        0
    //    simd3        5        7        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        8       18        0      N/A
    //  no simd       18       38        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2_w = other[e12345] * -1.0;
        let wedge_g0 = Simd32x4::from(right_dual_g2_w) * self.group0();
        let wedge_g1 = right_dual_g2_w * self[e1234];
        DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(wedge_g1) * other.group1().xyz()) + (other.group0().zxy() * wedge_g0.yzx()) - (other.group0().yzx() * wedge_g0.zxy()),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g1) * other.group2().xyz()) + (Simd32x3::from(wedge_g0[3]) * other.group0()) - (Simd32x3::from(other[e321]) * wedge_g0.xyz())).with_w(0.0),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g0[2] * other[e315]) - (wedge_g0[1] * other[e125]),
                (wedge_g0[0] * other[e125]) - (wedge_g0[2] * other[e235]),
                (wedge_g0[1] * other[e235]) - (wedge_g0[0] * other[e315]),
            ]) + (Simd32x3::from(wedge_g0[3]) * other.group1().xyz()))
            .with_w(wedge_g1 * other[e12345]),
            // e4235, e4315, e4125, e3215
            wedge_g0 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<DipoleInversion> for Sphere {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        5        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        4        9        0      N/A
    //  no simd        4       20        0        0
    fn project_via_origin_onto(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (other[e1234] * self[e3215]) + (other[e3215] * self[e1234]) - (other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]);
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g0) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(wedge_g0) * other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(wedge_g0) * other.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0) * other.group3(),
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        6        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        7        0      N/A
    //  no simd        0       10        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let wedge_g1 = other[e12345] * self[e1234] * -1.0;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(wedge_g1 * other[e5]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(wedge_g1 * other[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group0(),
        )
    }
}
impl ProjectViaOriginOnto<Flector> for Sphere {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3       12        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (other[e3215] * self[e1234]) - (other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]);
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(wedge_g0) * other.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       12        0        0
    //    simd3        1        2        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        6       17        0      N/A
    //  no simd        8       30        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_w = other[e12345] * -1.0;
        let wedge_g0 = Simd32x4::from(right_dual_g0_w) * self.group0();
        let wedge_g1 = right_dual_g0_w * self[e1234];
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(wedge_g1) * other.group0().xyz().with_w(other[e5]),
            // e23, e31, e12, e45
            (Simd32x3::from(wedge_g1) * other.group1().xyz()).with_w(-(wedge_g0[0] * other[e415]) - (wedge_g0[1] * other[e425]) - (wedge_g0[2] * other[e435])),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g0[2] * other[e315]) - (wedge_g0[1] * other[e125]),
                (wedge_g0[0] * other[e125]) - (wedge_g0[2] * other[e235]),
                (wedge_g0[1] * other[e235]) - (wedge_g0[0] * other[e315]),
            ]) + (Simd32x3::from(wedge_g0[3]) * other.group0().xyz()))
            .with_w(wedge_g1 * other[e12345]),
            // e4235, e4315, e4125, e3215
            wedge_g0 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for Sphere {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       14       26        0        0
    //    simd2        0        2        0      N/A
    //    simd3       17       24        0      N/A
    //    simd4        3        5        0      N/A
    // Totals...
    // yes simd       34       57        0      N/A
    //  no simd       77      122        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let wedge_g0_y = (right_dual_g1_xyz[0] * self[e4235])
            + (right_dual_g1_xyz[1] * self[e4315])
            + (right_dual_g1_xyz[2] * self[e4125])
            + (other[e3215] * self[e1234])
            + (self[e3215] * other[e1234]);
        let wedge_g9 = Simd32x4::from(right_dual_g0[0]) * self.group0();
        let wedge_g10 = right_dual_g0[0] * self[e1234];
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar])
                    + (wedge_g10 * other[e5])
                    + (wedge_g9[0] * other[e1])
                    + (wedge_g9[1] * other[e2])
                    + (wedge_g9[2] * other[e3])
                    + (wedge_g9[3] * other[e4]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g0_y) * other.group1().xyz()) + (Simd32x3::from(wedge_g9[3]) * other.group4()) + (other.group5().yzx() * wedge_g9.zxy())
                - (Simd32x3::from(wedge_g10) * other.group3().xyz())
                - (other.group5().zxy() * wedge_g9.yzx()))
            .with_w(wedge_g0_y * other[e4]),
            // e5
            (wedge_g0_y * other[e5]) + (wedge_g9[0] * other[e15]) + (wedge_g9[1] * other[e25]) + (wedge_g9[2] * other[e35]) + (wedge_g9[3] * other[e45]),
            // e15, e25, e35, e45
            ((Simd32x3::from(wedge_g0_y) * other.group3().xyz()) + (Simd32x3::from(wedge_g9[3]) * other.group6().xyz()) + (other.group8().yzx() * wedge_g9.zxy())
                - (other.group8().zxy() * wedge_g9.yzx()))
            .with_w(wedge_g0_y * other[e45]),
            // e41, e42, e43
            (Simd32x3::from(wedge_g0_y) * other.group4()) + (Simd32x3::from(wedge_g10) * other.group6().xyz()) + (other.group7().zxy() * wedge_g9.yzx())
                - (other.group7().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (Simd32x3::from(wedge_g0_y) * other.group5()) + (Simd32x3::from(wedge_g10) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group7())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            (Simd32x4::from(wedge_g0_y) * other.group6())
                + (wedge_g9.yzxw() * other.group9().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group9().yz()).with_zw(wedge_g9[1] * other[e4235] * -1.0, wedge_g10 * other[e3215] * -1.0),
            // e423, e431, e412
            (Simd32x3::from(wedge_g0_y) * other.group7()) + (Simd32x3::from(other[e1234]) * wedge_g9.xyz()) - (Simd32x3::from(wedge_g10) * other.group9().xyz()),
            // e235, e315, e125
            (Simd32x3::from(wedge_g0_y) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group9().xyz()) - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (wedge_g9 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group9()),
            // e1234
            (wedge_g0_y * other[e1234]) + (wedge_g10 * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for Sphere {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd4        3        6        0      N/A
    // Totals...
    // yes simd        3       11        0      N/A
    //  no simd       12       29        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(other[e3215] * self[e1234]) * other.group0())
                - (Simd32x4::from([other[e4235] * self[e4235], other[e4315] * self[e4315], other[e4125] * self[e4125], other[e3215] * self[e4235]]) * other.group0().xyzx())
                - (other.group0().xxxy() * other.group0().yyzw() * self.group0().yxxy())
                - (other.group0().xyyz() * other.group0().zzzw() * self.group0().zzyz()),
        )
    }
}
impl ProjectViaOriginOnto<Sphere> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        4       13        0        0
    fn project_via_origin_onto(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = (right_dual_g0_xyz[0] * self[e4235])
            + (right_dual_g0_xyz[1] * self[e4315])
            + (right_dual_g0_xyz[2] * self[e4125])
            + (other[e3215] * self[e1234])
            + (self[e3215] * other[e1234]);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0) * other.group0(),
            // e1234
            wedge_g0 * other[e1234],
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       12        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4        5        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd        8       21        0      N/A
    //  no simd       19       41        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_w = other[e12345] * -1.0;
        let wedge_g0 = Simd32x4::from(right_dual_g0_w) * self.group0();
        let wedge_g1 = right_dual_g0_w * self[e1234];
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (wedge_g0.yzxy() * other.group0().zxy().with_w(other[e2]))
                + ((Simd32x3::from(wedge_g1) * other.group1().xyz()) + -(wedge_g0.zx() * other.group0().yz()).with_z(wedge_g0[1] * other[e423] * -1.0))
                    .with_w(wedge_g0[0] * other[e1]),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g1) * other.group2().xyz()) + (Simd32x3::from(wedge_g0[3]) * other.group0().xyz()) - (Simd32x3::from(other[e321]) * wedge_g0.xyz())).with_w(0.0),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g0[2] * other[e315]) - (wedge_g0[1] * other[e125]),
                (wedge_g0[0] * other[e125]) - (wedge_g0[2] * other[e235]),
                (wedge_g0[1] * other[e235]) - (wedge_g0[0] * other[e315]),
            ]) + (Simd32x3::from(wedge_g0[3]) * other.group1().xyz()))
            .with_w(wedge_g1 * other[e12345]),
            // e4235, e4315, e4125, e3215
            wedge_g0 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<VersorOdd> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        5        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd        4       24        0        0
    fn project_via_origin_onto(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = (right_dual_g3_xyz[0] * self[e4235])
            + (right_dual_g3_xyz[1] * self[e4315])
            + (right_dual_g3_xyz[2] * self[e4125])
            + (self[e3215] * other[e1234])
            + (other[e3215] * self[e1234]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(wedge_g0) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(wedge_g0) * other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(wedge_g0) * other.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0) * other.group3(),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for VersorEven {
    type Output = ProjectViaOriginOntoInfixPartial<VersorEven>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiCircleRotor> for VersorEven {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       12        0        0
    //    simd2        2        4        0      N/A
    //    simd3        7        8        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd       16       24        0      N/A
    //  no simd       38       44        0        0
    fn project_via_origin_onto(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0 = Simd32x4::from([0.0, 0.0, (other[e31] * self[e1]) - (other[e23] * self[e2]), 0.0])
            + (((other.group1().zx() * self.group3().yz()) - (other.group1().yz() * self.group3().zx())).with_z(0.0)
                - (right_dual_g0 * Simd32x3::from(self[e5]))
                - (Simd32x3::from(self[e4]) * other.group2().xyz()))
            .with_w(0.0);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, (wedge_g0[1] * other[e23]) - (wedge_g0[0] * other[e31]), 0.0])
                + ((Simd32x3::from(wedge_g0[3]) * other.group0()) + ((wedge_g0.zx() * other.group1().yz()) - (wedge_g0.yz() * other.group1().zx())).with_z(0.0)
                    - (Simd32x3::from(right_dual_g0[0] * self[e1]) * other.group2().xyz())
                    - (Simd32x3::from(right_dual_g0[1] * self[e2]) * other.group2().xyz())
                    - (Simd32x3::from(right_dual_g0[2] * self[e3]) * other.group2().xyz())
                    - (Simd32x3::from(other[e45] * self[e4]) * other.group2().xyz()))
                .with_w(0.0),
            // e5
            (wedge_g0[0] * other[e15]) + (wedge_g0[1] * other[e25]) + (wedge_g0[2] * other[e35]) + (wedge_g0[3] * other[e45]),
        )
    }
}
impl ProjectViaOriginOnto<AntiDipoleInversion> for VersorEven {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       14        0        0
    //    simd3       11       17        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       22       32        0      N/A
    //  no simd       44       69        0        0
    fn project_via_origin_onto(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g2_xyz = other.group2().xyz();
        let wedge_g0 = (right_dual_g1_xyz * Simd32x3::from(self[e4])) + (other.group0().yzx() * self.group3().zxy()) - (other.group0().zxy() * self.group3().yzx());
        let wedge_g1_xyz = (right_dual_g2_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(other[e321]) * self.group3().xyz()) + (Simd32x3::from(self[e5]) * other.group0());
        let wedge_g2_xyz = (right_dual_g1_xyz * Simd32x3::from(self[e5])) + (right_dual_g2_xyz.zxy() * self.group3().yzx()) - (right_dual_g2_xyz.yzx() * self.group3().zxy());
        let wedge_g2_w = (other[e1] * self[e1])
            - (right_dual_g2_xyz[2] * self[e412])
            - (other[e423] * self[e235])
            - (other[e431] * self[e315])
            - (other[e412] * self[e125])
            - (other[e4] * self[e5]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(wedge_g2_w) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(wedge_g2_w) * other.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(wedge_g2_w) * other.group2().xyz()).with_w(
                (wedge_g2_w * other[e4])
                    - (wedge_g0[0] * other[e415])
                    - (wedge_g0[1] * other[e425])
                    - (wedge_g0[2] * other[e435])
                    - (wedge_g1_xyz[0] * other[e423])
                    - (wedge_g1_xyz[1] * other[e431])
                    - (wedge_g1_xyz[2] * other[e412]),
            ),
            // e1, e2, e3, e5
            ((wedge_g1_xyz * Simd32x3::from(other[e321]))
                + (Simd32x3::from(wedge_g2_w) * other.group3().xyz())
                + (wedge_g0.zxy() * other.group2().yzx())
                + (wedge_g2_xyz.yzx() * other.group0().zxy())
                - (wedge_g0.yzx() * other.group2().zxy())
                - (wedge_g2_xyz.zxy() * other.group0().yzx()))
            .with_w(wedge_g2_w * other[e5]),
        )
    }
}
impl ProjectViaOriginOnto<AntiDualNum> for VersorEven {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        9        0      N/A
    //  no simd        0       16        0        0
    fn project_via_origin_onto(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([1.0, 1.0, other[e3215] * other[e3215], 0.0])
                * (self.group0().xyz() * Simd32x2::from(other[e3215] * other[e3215] * -1.0).with_z(1.0) * Simd32x3::from([1.0, 1.0, -1.0])).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(other[e3215] * other[e3215] * self[e4] * -1.0),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlatPoint> for VersorEven {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        1        3        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3       10        0      N/A
    //  no simd        5       19        0        0
    fn project_via_origin_onto(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_w = other[e321] * -1.0;
        let wedge_g0_xyz = (Simd32x3::from(self[e4]) * other.group0().xyz()) - (Simd32x3::from(right_dual_g0_w) * self.group3().xyz());
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual_g0_w * self[e321] * -1.0) * other.group0(),
            // e1, e2, e3, e5
            (wedge_g0_xyz * Simd32x4::from(other[e321]).xyz()).with_w(-(wedge_g0_xyz[0] * other[e235]) - (wedge_g0_xyz[1] * other[e315]) - (wedge_g0_xyz[2] * other[e125])),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlector> for VersorEven {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        1        3        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd        5       10        0      N/A
    //  no simd       10       22        0        0
    fn project_via_origin_onto(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let wedge_g0 = (self.group3().xyzx() * Simd32x3::from(other[e321]).with_w(other[e1]))
            + (right_dual_g0_xyz * Simd32x3::from(self[e4]))
                .with_w((other[e321] * self[e321]) - (right_dual_g0_xyz[0] * self[e423]) - (right_dual_g0_xyz[1] * self[e431]) - (right_dual_g0_xyz[2] * self[e412]));
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(wedge_g0[3]) * other.group0(),
            // e1, e2, e3, e5
            ((Simd32x3::from(wedge_g0[3]) * other.group1().xyz()) + (Simd32x3::from(other[e321]) * wedge_g0.xyz())).with_w(wedge_g0[3] * other[e5]),
        )
    }
}
impl ProjectViaOriginOnto<AntiLine> for VersorEven {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        3        6        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        9       19        0        0
    fn project_via_origin_onto(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = (right_dual_g0.yzx() * self.group3().zxy()) - (Simd32x3::from(self[e4]) * other.group1()) - (right_dual_g0.zxy() * self.group3().yzx());
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            ((wedge_g0_xyz.zxy() * other.group0().yzx()) - (wedge_g0_xyz.yzx() * other.group0().zxy())).with_w(wedge_g0_xyz[0] * other[e15]),
        )
    }
}
impl ProjectViaOriginOnto<AntiMotor> for VersorEven {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        9        0        0
    //    simd3        3        6        0      N/A
    // Totals...
    // yes simd        7       15        0      N/A
    //  no simd       13       27        0        0
    fn project_via_origin_onto(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let wedge_g1_xyz = Simd32x3::from([
            (other[e12] * self[e2]) - (other[e31] * self[e3]),
            (other[e23] * self[e3]) - (other[e12] * self[e1]),
            (other[e31] * self[e1]) - (other[e23] * self[e2]),
        ]) + (Simd32x3::from(other[e3215]) * self.group0().xyz())
            - (Simd32x3::from(self[e4]) * other.group1().xyz());
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (wedge_g1_xyz * Simd32x4::from(other[e3215]).xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e1, e2, e3, e5
            ((wedge_g1_xyz.zxy() * other.group0().yzx()) - (wedge_g1_xyz.yzx() * other.group0().zxy()))
                .with_w((wedge_g1_xyz[0] * other[e15]) - (other[e3215] * other[e3215] * self[e4])),
        )
    }
}
impl ProjectViaOriginOnto<AntiPlane> for VersorEven {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd       12       20        0        0
    fn project_via_origin_onto(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x4::from(right_dual_g0_xyz[0] * self[e1]) * other.group0())
                + (Simd32x4::from(right_dual_g0_xyz[1] * self[e2]) * other.group0())
                + (Simd32x4::from(right_dual_g0_xyz[2] * self[e3]) * other.group0())
                - (Simd32x4::from(other[e5] * self[e4]) * other.group0()),
        )
    }
}
impl ProjectViaOriginOnto<AntiScalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0        9        0      N/A
    //  no simd        0       21        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e12345] * -1.0;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group3(),
        )
    }
}
impl ProjectViaOriginOnto<Circle> for VersorEven {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3       10       14        0      N/A
    // Totals...
    // yes simd       15       20        0      N/A
    //  no simd       35       48        0        0
    fn project_via_origin_onto(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0 = (right_dual_g1_xyz * Simd32x3::from(self[e4])) + (other.group0().yzx() * self.group3().zxy()) - (other.group0().zxy() * self.group3().yzx());
        let wedge_g1_xyz = (Simd32x3::from(other[e321]) * self.group3().xyz()) + (Simd32x3::from(self[e5]) * other.group0()) + (Simd32x3::from(self[e4]) * other.group2());
        let wedge_g2_xyz = (right_dual_g1_xyz * Simd32x3::from(self[e5])) + (other.group2().zxy() * self.group3().yzx()) - (other.group2().yzx() * self.group3().zxy());
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(
                -(wedge_g0[0] * other[e415])
                    - (wedge_g0[1] * other[e425])
                    - (wedge_g0[2] * other[e435])
                    - (wedge_g1_xyz[0] * other[e423])
                    - (wedge_g1_xyz[1] * other[e431])
                    - (wedge_g1_xyz[2] * other[e412]),
            ),
            // e1, e2, e3, e5
            ((wedge_g1_xyz * Simd32x3::from(other[e321])) + (wedge_g0.zxy() * other.group2().yzx()) + (wedge_g2_xyz.yzx() * other.group0().zxy())
                - (wedge_g0.yzx() * other.group2().zxy())
                - (wedge_g2_xyz.zxy() * other.group0().yzx()))
            .with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<CircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        7        0        0
    //    simd3       17       23        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd       18       33        0      N/A
    //  no simd       55       88        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g2_xyz = other.group2().xyz();
        let right_dual_g2_w = other[e12345] * -1.0;
        let wedge_g0_xyz = (right_dual_g1_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_dual_g2_w) * self.group0().xyz()) + (other.group0().yzx() * self.group3().zxy())
            - (other.group0().zxy() * self.group3().yzx());
        let wedge_g0_w = right_dual_g2_w * self[e12345];
        let wedge_g1 = ((right_dual_g2_xyz * Simd32x3::from(self[e4]))
            + (Simd32x3::from(right_dual_g2_w) * self.group1().xyz())
            + (Simd32x3::from(other[e321]) * self.group3().xyz())
            + (Simd32x3::from(self[e5]) * other.group0()))
        .with_w(right_dual_g2_w * self[e321]);
        let wedge_g2_xyz =
            (right_dual_g1_xyz * Simd32x3::from(self[e5])) + (Simd32x3::from(right_dual_g2_w) * self.group2().xyz()) + (right_dual_g2_xyz.zxy() * self.group3().yzx())
                - (right_dual_g2_xyz.yzx() * self.group3().zxy());
        let wedge_g3 = Simd32x4::from(right_dual_g2_w) * self.group3();
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((wedge_g0_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_w) * other.group0())).with_w(wedge_g0_w * other[e12345]),
            // e415, e425, e435, e321
            (wedge_g1 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_w) * other.group1()),
            // e235, e315, e125, e5
            ((wedge_g2_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_w) * other.group2().xyz())).with_w(right_dual_g2_w * other[e12345] * self[e5]),
            // e1, e2, e3, e4
            ((Simd32x3::from(wedge_g1[3]) * other.group1().xyz())
                + (Simd32x3::from(other[e321]) * wedge_g1.xyz())
                + (Simd32x3::from(other[e12345]) * wedge_g3.xyz())
                + (wedge_g0_xyz.zxy() * other.group2().yzx())
                + (wedge_g2_xyz.yzx() * other.group0().zxy())
                - (wedge_g0_xyz.yzx() * other.group2().zxy())
                - (wedge_g2_xyz.zxy() * other.group0().yzx()))
            .with_w(wedge_g3[3] * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Dipole> for VersorEven {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       12        0        0
    //    simd2        2        4        0      N/A
    //    simd3        7        8        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd       16       24        0      N/A
    //  no simd       38       44        0        0
    fn project_via_origin_onto(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0 = Simd32x4::from([0.0, 0.0, (other[e31] * self[e1]) - (other[e23] * self[e2]), 0.0])
            + (((other.group1().zx() * self.group3().yz()) - (other.group1().yz() * self.group3().zx())).with_z(0.0)
                - (right_dual_g0 * Simd32x3::from(self[e5]))
                - (Simd32x3::from(self[e4]) * other.group2()))
            .with_w(0.0);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, (wedge_g0[1] * other[e23]) - (wedge_g0[0] * other[e31]), 0.0])
                + ((Simd32x3::from(wedge_g0[3]) * other.group0()) + ((wedge_g0.zx() * other.group1().yz()) - (wedge_g0.yz() * other.group1().zx())).with_z(0.0)
                    - (Simd32x3::from(right_dual_g0[0] * self[e1]) * other.group2())
                    - (Simd32x3::from(right_dual_g0[1] * self[e2]) * other.group2())
                    - (Simd32x3::from(right_dual_g0[2] * self[e3]) * other.group2())
                    - (Simd32x3::from(other[e45] * self[e4]) * other.group2()))
                .with_w(0.0),
            // e5
            (other[e15] * wedge_g0[0]) + (other[e25] * wedge_g0[1]) + (other[e35] * wedge_g0[2]) + (wedge_g0[3] * other[e45]),
        )
    }
}
impl ProjectViaOriginOnto<DipoleInversion> for VersorEven {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       20       34        0        0
    //    simd2        4        8        0      N/A
    //    simd3        9       14        0      N/A
    //    simd4        7        7        0      N/A
    // Totals...
    // yes simd       40       63        0      N/A
    //  no simd       83      120        0        0
    fn project_via_origin_onto(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0 = -(Simd32x3::from(other[e1234]) * self.group3().xyz()) - (Simd32x3::from(self[e4]) * other.group3().xyz());
        let wedge_g1 =
            (other.group3().yzxw() * self.group3().zxyw()) + -(other.group3().zx() * self.group3().yz()).with_zw(other[e4315] * self[e1] * -1.0, other[e1234] * self[e5] * -1.0);
        let wedge_g2_xyz = (Simd32x3::from(other[e3215]) * self.group3().xyz()) + (Simd32x3::from(self[e5]) * other.group3().xyz());
        let wedge_g2_w = (right_dual_g0[0] * self[e1]) + (other[e4235] * self[e423]) + (other[e4315] * self[e431]) + (other[e4125] * self[e412]) - (other[e1234] * self[e321]);
        let wedge_g3 = (self.group3().yzxx() * other.group1().zxy().with_w(other[e15]))
            + ((Simd32x3::from(other[e3215]) * self.group0().xyz())
                + ((other.group3().zx() * self.group1().yz()) - (other.group1().yz() * self.group3().zx()) - (other.group3().yz() * self.group1().zx()))
                    .with_z((other[e4315] * self[e415]) - (other[e23] * self[e2]) - (other[e4235] * self[e425]))
                - (right_dual_g0 * Simd32x3::from(self[e5]))
                - (Simd32x3::from(self[e4]) * other.group2().xyz()))
            .with_w((other[e25] * self[e2]) + (other[e35] * self[e3]) - (other[e45] * self[e5]) - (other[e4235] * self[e235]) - (other[e4315] * self[e315]))
            - (self.group2().xyzz() * Simd32x3::from(other[e1234]).with_w(other[e4125]));
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other[e1234]) * wedge_g3.xyz()) - (Simd32x3::from(wedge_g2_w) * other.group3().xyz()),
            // e415, e425, e435, e321
            (wedge_g3.yzxw() * other.group3().zxy().with_w(other[e1234]))
                + -(wedge_g3.zx() * other.group3().yz()).with_zw(wedge_g3[1] * other[e4235] * -1.0, wedge_g2_w * other[e3215] * -1.0),
            // e235, e315, e125, e4
            ((Simd32x3::from(wedge_g3[3]) * other.group3().xyz()) - (Simd32x3::from(other[e3215]) * wedge_g3.xyz()))
                .with_w((wedge_g0[0] * other[e4235]) - (wedge_g2_w * other[e45]) - (other[e41] * wedge_g3[0]) - (other[e42] * wedge_g3[1]) - (other[e43] * wedge_g3[2])),
            // e1, e2, e3, e5
            (wedge_g3.zxyw() * other.group1().yzxw())
                + (wedge_g3.wwwy() * other.group0().with_w(other[e25]))
                + ((wedge_g2_xyz * Simd32x3::from(other[e1234]))
                    + ((wedge_g1.zx() * other.group3().yz()) - (wedge_g1.yz() * other.group3().zx()) - (wedge_g3.yz() * other.group1().zx()))
                        .with_z((wedge_g1[1] * other[e4235]) - (wedge_g1[0] * other[e4315]) - (wedge_g3[0] * other[e31]))
                    - (Simd32x3::from(wedge_g2_w) * other.group2().xyz()))
                .with_w(
                    (wedge_g3[0] * other[e15]) + (wedge_g3[2] * other[e35]) - (wedge_g2_xyz[0] * other[e4235]) - (wedge_g2_xyz[2] * other[e4125]) - (wedge_g1[3] * other[e3215]),
                )
                - (other.group3().wwwy() * wedge_g0.with_w(wedge_g2_xyz[1])),
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2       10        0        0
    //    simd3        0        3        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        2       17        0      N/A
    //  no simd        2       35        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (self.group0().xyz() * Simd32x2::from(other[e12345] * -1.0).with_z(other[e12345]) * Simd32x3::from([1.0, 1.0, -1.0]))
            .with_w(-(other[e5] * self[e4]) - (other[e12345] * self[e12345]));
        let wedge_g2 = Simd32x4::from(other[e12345] * -1.0) * self.group2();
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            wedge_g0 * Simd32x4::from(other[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group1(),
            // e235, e315, e125, e5
            (wedge_g2.xyz() * Simd32x2::from(other[e12345]).with_z(other[e12345])).with_w((other[e5] * wedge_g0[3]) + (other[e12345] * wedge_g2[3])),
            // e1, e2, e3, e4
            Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group3(),
        )
    }
}
impl ProjectViaOriginOnto<FlatPoint> for VersorEven {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       14        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6       16        0      N/A
    //  no simd        6       21        0        0
    fn project_via_origin_onto(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(self[e4] * -1.0) * other.group0().xyz();
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e45] * self[e4] * -1.0) * other.group0(),
            // e5
            (wedge_g0_xyz[0] * other[e15])
                + (wedge_g0_xyz[1] * other[e25])
                + (wedge_g0_xyz[2] * other[e35])
                + (other[e15] * other[e45] * self[e1])
                + (other[e25] * other[e45] * self[e2])
                + (other[e35] * other[e45] * self[e3])
                - (other[e45] * other[e45] * self[e5]),
        )
    }
}
impl ProjectViaOriginOnto<Flector> for VersorEven {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       25        0        0
    //    simd2        3        7        0      N/A
    //    simd3        3        7        0      N/A
    //    simd4        4        4        0      N/A
    // Totals...
    // yes simd       22       43        0      N/A
    //  no simd       43       76        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[e4] * -1.0) * other.group1().xyz();
        let wedge_g1_xy = (other.group1().yz() * self.group3().zx()) - (other.group1().zx() * self.group3().yz());
        let wedge_g1_z = (other[e4235] * self[e2]) - (other[e4315] * self[e1]);
        let wedge_g2 = (other.group1().wwwx() * self.group3().xyz().with_w(self[e423]))
            + (Simd32x3::from(self[e5]) * other.group1().xyz()).with_w((other[e45] * self[e4]) + (other[e4315] * self[e431]) + (other[e4125] * self[e412]));
        let wedge_g3 = ((Simd32x3::from(other[e3215]) * self.group0().xyz()) + (other.group1().zx() * self.group1().yz()).with_z(other[e4315] * self[e415]))
            .with_w((other[e15] * self[e1]) + (other[e25] * self[e2]) - (other[e4315] * self[e315]) - (other[e4125] * self[e125]))
            - (other.group0() * Simd32x3::from(self[e4]).with_w(self[e5]))
            - (other.group1().yzxx() * self.group1().zxy().with_w(self[e235]));
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(wedge_g2[3] * -1.0) * other.group1().xyz(),
            // e415, e425, e435, e321
            ((wedge_g3.yz() * other.group1().zx()) - (wedge_g3.zx() * other.group1().yz()))
                .with_zw((wedge_g3[0] * other[e4315]) - (wedge_g3[1] * other[e4235]), wedge_g2[3] * other[e3215] * -1.0),
            // e235, e315, e125, e4
            ((Simd32x3::from(wedge_g3[3]) * other.group1().xyz()) - (Simd32x3::from(other[e3215]) * wedge_g3.xyz()))
                .with_w((wedge_g0[0] * other[e4235]) - (wedge_g2[3] * other[e45])),
            // e1, e2, e3, e5
            (((Simd32x2::from([wedge_g1_z, wedge_g1_xy[0]]) * other.group1().yz()) - (Simd32x2::from([wedge_g1_xy[1], wedge_g1_z]) * other.group1().zx()))
                .with_z((wedge_g1_xy[1] * other[e4235]) - (wedge_g1_xy[0] * other[e4315]))
                - (wedge_g0 * Simd32x3::from(other[e3215])))
            .with_w((wedge_g3[0] * other[e15]) - (wedge_g2[0] * other[e4235]) - (wedge_g2[1] * other[e4315]) - (other[e3215] * other[e3215] * self[e4]))
                - (wedge_g2.wwwz() * other.group0().xyz().with_w(other[e4125])),
        )
    }
}
impl ProjectViaOriginOnto<Line> for VersorEven {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        4        9        0      N/A
    // Totals...
    // yes simd        6       12        0      N/A
    //  no simd       14       30        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[e4]) * other.group0();
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(-(wedge_g0[0] * other[e415]) - (wedge_g0[1] * other[e425]) - (wedge_g0[2] * other[e435])),
            // e1, e2, e3, e5
            ((wedge_g0.zxy() * other.group1().yzx())
                - (wedge_g0.yzx() * other.group1().zxy())
                - (other.group0() * other.group0() * self.group3().xyz())
                - (Simd32x3::from(other[e415]) * other.group0().yyz() * self.group3().yxx())
                - (Simd32x3::from(other[e435]) * other.group0().xyy() * self.group3().zzy()))
            .with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       21        0        0
    //    simd3        7       11        0      N/A
    //    simd4        3        5        0      N/A
    // Totals...
    // yes simd       21       37        0      N/A
    //  no simd       44       74        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0 = (Simd32x4::from(right_dual_g0_w) * self.group0())
            + (right_dual_g0_xyz * Simd32x3::from(self[e4]))
                .with_w(-(right_dual_g1_xyz[0] * self[e423]) - (right_dual_g1_xyz[1] * self[e431]) - (right_dual_g1_xyz[2] * self[e412]) - (other[e5] * self[e4]));
        let wedge_g1_xyz = (right_dual_g1_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_dual_g0_w) * self.group1().xyz());
        let wedge_g1_w = right_dual_g0_w * self[e321];
        let wedge_g2 = ((right_dual_g0_xyz * Simd32x3::from(self[e5])) + (Simd32x3::from(right_dual_g0_w) * self.group2().xyz()) + (right_dual_g1_xyz.zxy() * self.group3().yzx())
            - (right_dual_g1_xyz.yzx() * self.group3().zxy()))
        .with_w(right_dual_g0_w * self[e5]);
        let wedge_g3 = Simd32x4::from(right_dual_g0_w) * self.group3();
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            wedge_g0 * Simd32x4::from(other[e12345]),
            // e415, e425, e435, e321
            ((wedge_g1_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0[3]) * other.group0().xyz())).with_w(wedge_g1_w * other[e12345]),
            // e235, e315, e125, e5
            (wedge_g2 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0[3]) * other.group1())
                + Simd32x3::from(0.0).with_w(
                    -(wedge_g1_xyz[0] * other[e235])
                        - (wedge_g1_xyz[1] * other[e315])
                        - (wedge_g1_xyz[2] * other[e125])
                        - (wedge_g2[0] * other[e415])
                        - (wedge_g2[1] * other[e425])
                        - (wedge_g2[2] * other[e435]),
                ),
            // e1, e2, e3, e4
            (Simd32x3::from([
                (wedge_g0[2] * other[e315]) - (wedge_g0[1] * other[e125]),
                (wedge_g0[0] * other[e125]) - (wedge_g0[2] * other[e235]),
                (wedge_g0[1] * other[e235]) - (wedge_g0[0] * other[e315]),
            ]) + (Simd32x3::from(wedge_g1_w) * other.group0().xyz())
                + (Simd32x3::from(other[e12345]) * wedge_g3.xyz()))
            .with_w(wedge_g3[3] * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       76       96        0        0
    //    simd2        0        3        0      N/A
    //    simd3       50       64        0      N/A
    //    simd4       14       17        0      N/A
    // Totals...
    // yes simd      140      180        0      N/A
    //  no simd      282      362        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1 = (other.group9().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let right_dual_g3_w = other[e321] * -1.0;
        let right_dual_g5 = other.group6().xyz();
        let right_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_dual_g9_xyz = other.group1().xyz();
        let wedge_g0_y = (right_dual_g0[0] * self[e12345]) + (right_dual_g9_xyz[0] * self[e1]) + (right_dual_g9_xyz[1] * self[e2]) + (right_dual_g9_xyz[2] * self[e3])
            - (right_dual_g3_w * self[e321])
            - (right_dual_g5[0] * self[e415])
            - (right_dual_g5[1] * self[e425])
            - (right_dual_g5[2] * self[e435])
            - (other[e423] * self[e235])
            - (other[e431] * self[e315])
            - (other[e412] * self[e125])
            - (other[e235] * self[e423])
            - (other[e315] * self[e431])
            - (other[e125] * self[e412])
            - (other[e4] * self[e5])
            - (self[e4] * other[e5]);
        let wedge_g1 = Simd32x4::from(right_dual_g0[0]) * self.group3();
        let wedge_g2 = right_dual_g0[0] * self[e5];
        let wedge_g3 = (Simd32x4::from(other[e3215]) * self.group3()) - (right_dual_g1 * Simd32x4::from(self[e5]));
        let wedge_g4 = (Simd32x3::from(self[e4]) * right_dual_g1.xyz()) - (Simd32x3::from(right_dual_g1[3]) * self.group3().xyz());
        let wedge_g5 = Simd32x3::from([
            (right_dual_g1[2] * self[e2]) - (right_dual_g1[1] * self[e3]),
            (right_dual_g1[0] * self[e3]) - (right_dual_g1[2] * self[e1]),
            (right_dual_g1[1] * self[e1]) - (right_dual_g1[0] * self[e2]),
        ]);
        let wedge_g6 = ((Simd32x3::from(right_dual_g0[0]) * self.group1().xyz()) + (Simd32x3::from(self[e5]) * other.group7()) + (Simd32x3::from(self[e4]) * other.group8())
            - (Simd32x3::from(right_dual_g3_w) * self.group3().xyz()))
        .with_w(right_dual_g0[0] * self[e321]);
        let wedge_g7 = (right_dual_g5 * Simd32x3::from(self[e4])) + (Simd32x3::from(right_dual_g0[0]) * self.group0().xyz()) + (other.group7().yzx() * self.group3().zxy())
            - (other.group7().zxy() * self.group3().yzx());
        let wedge_g8 = (right_dual_g5 * Simd32x3::from(self[e5])) + (Simd32x3::from(right_dual_g0[0]) * self.group2().xyz()) + (other.group8().zxy() * self.group3().yzx())
            - (other.group8().yzx() * self.group3().zxy());
        let wedge_g9 = (Simd32x4::from(other[e3215]) * self.group0().xyz().with_w(self[e321]))
            + (right_dual_g1.yzxz() * self.group1().zxy().with_w(self[e125]))
            + ((right_dual_g8 * Simd32x3::from(self[e4]))
                + (right_dual_g6_xyz.yzx() * self.group3().zxy())
                + -(right_dual_g1.zx() * self.group1().yz()).with_z(right_dual_g1[1] * self[e415] * -1.0)
                - (right_dual_g7 * Simd32x3::from(self[e5])))
            .with_w((right_dual_g1[0] * self[e235]) + (right_dual_g1[1] * self[e315]) - (right_dual_g8[1] * self[e2]) - (right_dual_g8[2] * self[e3]))
            - (self.group2() * Simd32x3::from(right_dual_g1[3]).with_w(other[e45]))
            - (self.group3().yzxx() * right_dual_g6_xyz.zxy().with_w(right_dual_g8[0]));
        let wedge_g10 = (right_dual_g7[0] * self[e1]) + (right_dual_g7[1] * self[e2]) + (right_dual_g7[2] * self[e3]) + (other[e45] * self[e4])
            - (right_dual_g1[0] * self[e423])
            - (right_dual_g1[1] * self[e431])
            - (right_dual_g1[2] * self[e412])
            - (right_dual_g1[3] * self[e321]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar])
                    + (wedge_g10 * other[e5])
                    + (wedge_g2 * other[e1234])
                    + (wedge_g1[0] * other[e4235])
                    + (wedge_g1[1] * other[e4315])
                    + (wedge_g1[2] * other[e4125])
                    + (wedge_g1[3] * other[e3215])
                    + (wedge_g9[0] * other[e1])
                    + (wedge_g9[1] * other[e2])
                    + (wedge_g9[2] * other[e3])
                    + (wedge_g9[3] * other[e4])
                    - (wedge_g4[0] * other[e235])
                    - (wedge_g4[1] * other[e315])
                    - (wedge_g4[2] * other[e125])
                    - (wedge_g5[0] * other[e415])
                    - (wedge_g5[1] * other[e425])
                    - (wedge_g5[2] * other[e435])
                    - (wedge_g7[0] * other[e15])
                    - (wedge_g7[1] * other[e25])
                    - (wedge_g7[2] * other[e35])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43])
                    - (other[e23] * wedge_g6[0])
                    - (other[e31] * wedge_g6[1])
                    - (other[e12] * wedge_g6[2])
                    - (other[e423] * wedge_g3[0])
                    - (other[e431] * wedge_g3[1])
                    - (other[e412] * wedge_g3[2])
                    - (wedge_g3[3] * other[e321])
                    - (wedge_g6[3] * other[e45]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            (wedge_g1 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group1())
                + (other.group9().yzxx() * wedge_g5.zxy().with_w(wedge_g4[0]))
                + ((Simd32x3::from(wedge_g6[3]) * other.group6().xyz())
                    + (Simd32x3::from(wedge_g9[3]) * other.group4())
                    + (Simd32x3::from(other[e321]) * wedge_g6.xyz())
                    + (Simd32x3::from(other[e1234]) * wedge_g3.xyz())
                    + (wedge_g7.zxy() * other.group8().yzx())
                    + (wedge_g8.yzx() * other.group7().zxy())
                    + (other.group5().yzx() * wedge_g9.zxy())
                    - (wedge_g4 * Simd32x3::from(other[e3215]))
                    - (Simd32x3::from(wedge_g10) * other.group3().xyz())
                    - (wedge_g5.yzx() * other.group9().zxy())
                    - (wedge_g7.yzx() * other.group8().zxy())
                    - (wedge_g8.zxy() * other.group7().yzx())
                    - (other.group5().zxy() * wedge_g9.yzx()))
                .with_w((wedge_g4[1] * other[e4315]) + (wedge_g4[2] * other[e4125]) + (wedge_g3[3] * other[e1234])),
            // e5
            (wedge_g0_y * other[e5])
                + (wedge_g2 * other[e12345])
                + (wedge_g9[0] * other[e15])
                + (wedge_g9[1] * other[e25])
                + (wedge_g9[2] * other[e35])
                + (wedge_g9[3] * other[e45])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435])
                - (other[e235] * wedge_g6[0])
                - (other[e315] * wedge_g6[1])
                - (other[e125] * wedge_g6[2])
                - (wedge_g3[0] * other[e4235])
                - (wedge_g3[1] * other[e4315])
                - (wedge_g3[2] * other[e4125])
                - (wedge_g3[3] * other[e3215]),
            // e15, e25, e35, e45
            (wedge_g3 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group3())
                + ((Simd32x3::from(wedge_g9[3]) * other.group6().xyz())
                    + (Simd32x3::from(other[e3215]) * wedge_g6.xyz())
                    + (wedge_g8.yzx() * other.group9().zxy())
                    + (other.group8().yzx() * wedge_g9.zxy())
                    - (wedge_g8.zxy() * other.group9().yzx())
                    - (other.group8().zxy() * wedge_g9.yzx()))
                .with_w(0.0),
            // e41, e42, e43
            (wedge_g4 * Simd32x3::from(other[e12345]))
                + (Simd32x3::from(wedge_g0_y) * other.group4())
                + (Simd32x3::from(wedge_g10) * other.group6().xyz())
                + (Simd32x3::from(other[e1234]) * wedge_g6.xyz())
                + (wedge_g7.zxy() * other.group9().yzx())
                + (other.group7().zxy() * wedge_g9.yzx())
                - (wedge_g7.yzx() * other.group9().zxy())
                - (other.group7().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (wedge_g5 * Simd32x3::from(other[e12345]))
                + (wedge_g7 * Simd32x3::from(other[e3215]))
                + (wedge_g8 * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(wedge_g0_y) * other.group5())
                + (Simd32x3::from(wedge_g10) * other.group8())
                + (Simd32x3::from(wedge_g9[3]) * other.group7())
                - (Simd32x3::from(wedge_g6[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            (wedge_g6 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group6())
                + (wedge_g9.yzxw() * other.group9().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group9().yz()).with_zw(wedge_g9[1] * other[e4235] * -1.0, wedge_g10 * other[e3215] * -1.0),
            // e423, e431, e412
            (wedge_g7 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group7()) + (Simd32x3::from(other[e1234]) * wedge_g9.xyz())
                - (Simd32x3::from(wedge_g10) * other.group9().xyz()),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (wedge_g9 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group9()),
            // e1234
            (wedge_g0_y * other[e1234]) + (wedge_g10 * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for VersorEven {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       10        0        0
    //    simd2        4        8        0      N/A
    //    simd3        3        5        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd       14       23        0      N/A
    //  no simd       33       41        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[e4] * -1.0) * other.group0().xyz();
        let wedge_g1_xy = (other.group0().yz() * self.group3().zx()) - (other.group0().zx() * self.group3().yz());
        let wedge_g1_z = (other[e4235] * self[e2]) - (other[e4315] * self[e1]);
        let wedge_g3 = Simd32x4::from([0.0, 0.0, (other[e4315] * self[e415]) - (other[e4235] * self[e425]), 0.0])
            + ((Simd32x3::from(other[e3215]) * self.group0().xyz()) + ((other.group0().zx() * self.group1().yz()) - (other.group0().yz() * self.group1().zx())).with_z(0.0))
                .with_w(0.0);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, (wedge_g3[0] * other[e4315]) - (wedge_g3[1] * other[e4235]), 0.0])
                + ((wedge_g3.yz() * other.group0().zx()) - (wedge_g3.zx() * other.group0().yz())).with_zw(0.0, 0.0),
            // e235, e315, e125, e4
            ((Simd32x3::from(wedge_g3[3]) * other.group0().xyz()) - (Simd32x3::from(other[e3215]) * wedge_g3.xyz())).with_w(wedge_g0[0] * other[e4235]),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, (wedge_g1_xy[1] * other[e4235]) - (wedge_g1_xy[0] * other[e4315]), 0.0])
                + (((Simd32x2::from([wedge_g1_z, wedge_g1_xy[0]]) * other.group0().yz()) - (Simd32x2::from([wedge_g1_xy[1], wedge_g1_z]) * other.group0().zx())).with_z(0.0)
                    - (wedge_g0 * Simd32x3::from(other[e3215])))
                .with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<RoundPoint> for VersorEven {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd        4       10        0        0
    fn project_via_origin_onto(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let wedge_g0 = (right_dual_g0_xyz[0] * self[e1]) + (right_dual_g0_xyz[1] * self[e2]) + (right_dual_g0_xyz[2] * self[e3]) - (other[e4] * self[e5]) - (self[e4] * other[e5]);
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(wedge_g0) * other.group0(), /* e5 */ wedge_g0 * other[e5])
    }
}
impl ProjectViaOriginOnto<Sphere> for VersorEven {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        8        0        0
    //    simd3       11       19        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       13       28        0      N/A
    //  no simd       38       69        0        0
    fn project_via_origin_onto(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = (right_dual_g0_xyz * Simd32x3::from(self[e4])) - (Simd32x3::from(other[e1234]) * self.group3().xyz());
        let wedge_g1_xyz = (right_dual_g0_xyz.zxy() * self.group3().yzx()) - (right_dual_g0_xyz.yzx() * self.group3().zxy());
        let wedge_g2_w = self[e321] * other[e1234] * -1.0;
        let wedge_g3_xyz = (Simd32x3::from(other[e3215]) * self.group0().xyz()) + (right_dual_g0_xyz.yzx() * self.group1().zxy())
            - (Simd32x3::from(other[e1234]) * self.group2().xyz())
            - (right_dual_g0_xyz.zxy() * self.group1().yzx());
        let wedge_g3_w = (right_dual_g0_xyz[0] * self[e235]) + (right_dual_g0_xyz[1] * self[e315]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (wedge_g3_xyz * Simd32x3::from(other[e1234])) - (Simd32x3::from(wedge_g2_w) * other.group0().xyz()),
            // e415, e425, e435, e321
            (wedge_g3_xyz.yzx() * other.group0().zxy()).with_w(wedge_g3_w * other[e1234]) - (other.group0().yzxw() * wedge_g3_xyz.zxy().with_w(wedge_g2_w)),
            // e235, e315, e125, e4
            ((Simd32x3::from(wedge_g3_w) * other.group0().xyz()) - (wedge_g3_xyz * Simd32x3::from(other[e3215]))).with_w(wedge_g0[0] * other[e4235]),
            // e1, e2, e3, e5
            ((Simd32x3::from(other[e3215] * other[e1234]) * self.group3().xyz()) + (wedge_g1_xyz.zxy() * other.group0().yzx())
                - (right_dual_g0_xyz * Simd32x3::from(self[e5] * other[e1234]))
                - (wedge_g0 * Simd32x3::from(other[e3215]))
                - (wedge_g1_xyz.yzx() * other.group0().zxy()))
            .with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       18       27        0        0
    //    simd2        2        3        0      N/A
    //    simd3       13       17        0      N/A
    //    simd4        5        7        0      N/A
    // Totals...
    // yes simd       38       54        0      N/A
    //  no simd       81      112        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let right_dual_g1_xyz = other.group1().xyz();
        let right_dual_g1_w = other[e321] * -1.0;
        let right_dual_g2_xyz = other.group2().xyz();
        let wedge_g0 = (Simd32x4::from(right_dual_g0_w) * self.group0())
            + ((right_dual_g1_xyz * Simd32x3::from(self[e4])) + (right_dual_g0_xyz.yzx() * self.group3().zxy()) - (right_dual_g0_xyz.zxy() * self.group3().yzx())).with_w(
                (other[e1] * self[e1])
                    - (right_dual_g1_w * self[e321])
                    - (right_dual_g0_xyz[0] * self[e235])
                    - (right_dual_g0_xyz[1] * self[e315])
                    - (right_dual_g0_xyz[2] * self[e125])
                    - (right_dual_g1_xyz[0] * self[e415])
                    - (right_dual_g1_xyz[1] * self[e425])
                    - (right_dual_g1_xyz[2] * self[e435])
                    - (right_dual_g2_xyz[0] * self[e423])
                    - (right_dual_g2_xyz[1] * self[e431])
                    - (right_dual_g2_xyz[2] * self[e412])
                    - (other[e4] * self[e5]),
            );
        let wedge_g1 = ((right_dual_g0_xyz * Simd32x3::from(self[e5])) + (right_dual_g2_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_dual_g0_w) * self.group1().xyz())
            - (Simd32x3::from(right_dual_g1_w) * self.group3().xyz()))
        .with_w(right_dual_g0_w * self[e321]);
        let wedge_g2 = ((right_dual_g1_xyz * Simd32x3::from(self[e5])) + (Simd32x3::from(right_dual_g0_w) * self.group2().xyz()) + (right_dual_g2_xyz.zxy() * self.group3().yzx())
            - (right_dual_g2_xyz.yzx() * self.group3().zxy()))
        .with_w(right_dual_g0_w * self[e5]);
        let wedge_g3 = Simd32x4::from(right_dual_g0_w) * self.group3();
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x3::from(wedge_g0[3]) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g0.xyz())).with_w(wedge_g0[3] * other[e12345]),
            // e415, e425, e435, e321
            (wedge_g1 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0[3]) * other.group1()),
            // e235, e315, e125, e5
            (wedge_g2 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0[3]) * other.group2())
                + Simd32x3::from(0.0).with_w(
                    -(wedge_g1[0] * other[e235])
                        - (wedge_g1[1] * other[e315])
                        - (wedge_g1[2] * other[e125])
                        - (wedge_g2[0] * other[e415])
                        - (wedge_g2[1] * other[e425])
                        - (wedge_g2[2] * other[e435]),
                ),
            // e1, e2, e3, e4
            (wedge_g0.zxyw() * other.group2().yzx().with_w(other[e4]))
                + ((Simd32x3::from(wedge_g0[3]) * other.group3().xyz())
                    + (Simd32x3::from(wedge_g1[3]) * other.group1().xyz())
                    + (Simd32x3::from(other[e12345]) * wedge_g3.xyz())
                    + (Simd32x3::from(other[e321]) * wedge_g1.xyz())
                    + ((wedge_g2.yz() * other.group0().zx()) - (wedge_g0.yz() * other.group2().zx()) - (wedge_g2.zx() * other.group0().yz()))
                        .with_z((wedge_g2[0] * other[e431]) - (wedge_g0[0] * other[e315]) - (wedge_g2[1] * other[e423])))
                .with_w(wedge_g3[3] * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<VersorOdd> for VersorEven {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       22        0        0
    //    simd2        4        7        0      N/A
    //    simd3        8       14        0      N/A
    //    simd4        6        6        0      N/A
    // Totals...
    // yes simd       31       49        0      N/A
    //  no simd       69      102        0        0
    fn project_via_origin_onto(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = (right_dual_g3_xyz * Simd32x3::from(self[e4])) - (Simd32x3::from(other[e1234]) * self.group3().xyz());
        let wedge_g1 =
            (self.group3().yzxw() * right_dual_g3_xyz.zxy().with_w(other[e3215])) + -(right_dual_g3_xyz.yzx() * self.group3().zxy()).with_w(self[e5] * other[e1234] * -1.0);
        let wedge_g2_xyz = (Simd32x3::from(other[e3215]) * self.group3().xyz()) - (right_dual_g3_xyz * Simd32x3::from(self[e5]));
        let wedge_g3 = (self.group2().wwwz() * other.group0().xyz().with_w(right_dual_g3_xyz[2]))
            + ((Simd32x3::from(other[e3215]) * self.group0().xyz())
                + (right_dual_g3_xyz.yzx() * self.group1().zxy())
                + ((self.group3().yz() * other.group1().zx()) - (self.group3().zx() * other.group1().yz())).with_z((self[e1] * other[e31]) - (self[e2] * other[e23]))
                - (Simd32x3::from(self[e4]) * other.group2().xyz())
                - (Simd32x3::from(other[e1234]) * self.group2().xyz())
                - (right_dual_g3_xyz.zxy() * self.group1().yzx()))
            .with_w((right_dual_g3_xyz[0] * self[e235]) + (right_dual_g3_xyz[1] * self[e315]) + (self[e321] * other[e3215]) - (self[e5] * other[e45]));
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e1234]) * wedge_g3.xyz(),
            // e415, e425, e435, e321
            ((wedge_g3.yz() * other.group3().zx()) - (wedge_g3.zx() * other.group3().yz()))
                .with_zw((wedge_g3[0] * other[e4315]) - (wedge_g3[1] * other[e4235]), wedge_g3[3] * other[e1234]),
            // e235, e315, e125, e4
            (Simd32x3::from(wedge_g3[3]) * other.group3().xyz()).with_w((wedge_g0[0] * other[e4235]) - (wedge_g3[1] * other[e42]) - (wedge_g3[2] * other[e43]))
                - (wedge_g3.xyzx() * Simd32x3::from(other[e3215]).with_w(other[e41])),
            // e1, e2, e3, e5
            (Simd32x4::from(wedge_g3[3]) * other.group0().xyz().with_w(other[e45]))
                + (wedge_g3.zxyz() * other.group1().yzx().with_w(other[e35]))
                + ((wedge_g2_xyz * Simd32x3::from(other[e1234]))
                    + ((wedge_g1.zx() * other.group3().yz()) - (wedge_g1.yz() * other.group3().zx()) - (wedge_g3.yz() * other.group1().zx()))
                        .with_z((wedge_g1[1] * other[e4235]) - (wedge_g1[0] * other[e4315]) - (wedge_g3[0] * other[e31])))
                .with_w(
                    (wedge_g3[0] * other[e15]) + (wedge_g3[1] * other[e25]) - (wedge_g2_xyz[1] * other[e4315]) - (wedge_g2_xyz[2] * other[e4125]) - (wedge_g1[3] * other[e3215]),
                )
                - (other.group3().wwwx() * wedge_g0.with_w(wedge_g2_xyz[0])),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for VersorOdd {
    type Output = ProjectViaOriginOntoInfixPartial<VersorOdd>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<AntiCircleRotor> for VersorOdd {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       20       23        0        0
    //    simd3        0        5        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd       20       31        0      N/A
    //  no simd       20       50        0        0
    fn project_via_origin_onto(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g0 = right_dual_g0 * Simd32x3::from(self[scalar]);
        let wedge_g1 = right_dual_g1 * Simd32x4::from(self[scalar]);
        let wedge_g2_xyz = Simd32x3::from(self[scalar] * -1.0) * other.group2().xyz();
        let wedge_g2_w = (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43]) + (other[scalar] * self[scalar])
            - (right_dual_g0[0] * self[e15])
            - (right_dual_g0[1] * self[e25])
            - (right_dual_g0[2] * self[e35])
            - (right_dual_g1[0] * self[e23])
            - (right_dual_g1[1] * self[e31])
            - (right_dual_g1[2] * self[e12])
            - (right_dual_g1[3] * self[e45]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g2_w) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(wedge_g2_w) * other.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(wedge_g2_w) * other.group2().xyz()).with_w(
                (wedge_g2_w * other[scalar])
                    - (wedge_g0[0] * other[e15])
                    - (wedge_g0[1] * other[e25])
                    - (wedge_g0[2] * other[e35])
                    - (wedge_g2_xyz[0] * other[e41])
                    - (wedge_g2_xyz[1] * other[e42])
                    - (wedge_g2_xyz[2] * other[e43])
                    - (wedge_g1[0] * other[e23])
                    - (wedge_g1[1] * other[e31])
                    - (wedge_g1[2] * other[e12])
                    - (wedge_g1[3] * other[e45]),
            ),
        )
    }
}
impl ProjectViaOriginOnto<AntiDipoleInversion> for VersorOdd {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       15       20        0        0
    //    simd3       10       16        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd       27       39        0      N/A
    //  no simd       53       80        0        0
    fn project_via_origin_onto(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        let right_dual_g2_xyz = other.group2().xyz();
        let wedge_g0 = Simd32x3::from(self[scalar]) * other.group0();
        let wedge_g1 = right_dual_g1 * Simd32x4::from(self[scalar]);
        let wedge_g2_xyz = right_dual_g2_xyz * Simd32x3::from(self[scalar]);
        let wedge_g2_w = -(other[e423] * self[e23])
            - (other[e431] * self[e31])
            - (other[e412] * self[e12])
            - (right_dual_g1[0] * self[e41])
            - (right_dual_g1[1] * self[e42])
            - (right_dual_g1[2] * self[e43])
            - (other[e4] * self[scalar]);
        let wedge_g3_xyz = (Simd32x3::from(right_dual_g1[3]) * self.group1().xyz())
            + (Simd32x3::from(self[scalar]) * other.group3().xyz())
            + (Simd32x3::from(self[e45]) * right_dual_g1.xyz())
            + (right_dual_g2_xyz.zxy() * self.group0().yzx())
            + (other.group0().yzx() * self.group2().zxy())
            - (right_dual_g2_xyz.yzx() * self.group0().zxy())
            - (other.group0().zxy() * self.group2().yzx());
        let wedge_g3_w = other[e5] * self[scalar] * -1.0;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(wedge_g2_w) * other.group1().xyz()) + (wedge_g3_xyz.yzx() * other.group0().zxy()) - (wedge_g3_xyz.zxy() * other.group0().yzx()),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g2_w) * other.group2().xyz()) + (Simd32x3::from(wedge_g3_w) * other.group0()) - (wedge_g3_xyz * Simd32x3::from(other[e321]))).with_w(0.0),
            // e15, e25, e35, scalar
            (Simd32x4::from(wedge_g3_w) * other.group1().xyz().with_w(other[e4]))
                + (wedge_g3_xyz.zxy() * other.group2().yzx()).with_w(
                    (wedge_g3_xyz[0] * other[e1])
                        - (wedge_g0[1] * other[e315])
                        - (wedge_g0[2] * other[e125])
                        - (wedge_g2_xyz[0] * other[e423])
                        - (wedge_g2_xyz[1] * other[e431])
                        - (wedge_g2_xyz[2] * other[e412])
                        - (wedge_g1[0] * other[e415])
                        - (wedge_g1[1] * other[e425])
                        - (wedge_g1[2] * other[e435])
                        - (wedge_g1[3] * other[e321]),
                )
                - (other.group2().zxyx() * wedge_g3_xyz.yzx().with_w(wedge_g0[0])),
        )
    }
}
impl ProjectViaOriginOnto<AntiDualNum> for VersorOdd {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        5        0      N/A
    //  no simd        1       10        0        0
    fn project_via_origin_onto(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (self.group0().xyz() * Simd32x2::from(other[e3215]).with_z(other[e3215])).with_w((other[e3215] * self[e1234]) + (other[scalar] * self[scalar]));
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(other[scalar] * wedge_g0[3]),
            // e15, e25, e35, e3215
            wedge_g0 * Simd32x4::from(other[e3215]),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlatPoint> for VersorOdd {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        3        5        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        9        0      N/A
    //  no simd        9       22        0        0
    fn project_via_origin_onto(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e321] * -1.0;
        let wedge_g1_xyz =
            (Simd32x3::from(right_dual_g0_w) * self.group1().xyz()) + (right_dual_g0_xyz.zxy() * self.group0().yzx()) - (right_dual_g0_xyz.yzx() * self.group0().zxy());
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other[e321] * -1.0) * wedge_g1_xyz.with_w(right_dual_g0_w * self[scalar]),
            // e15, e25, e35, e3215
            ((wedge_g1_xyz.zxy() * other.group0().yzx()) - (wedge_g1_xyz.yzx() * other.group0().zxy())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<AntiFlector> for VersorOdd {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        4        8        0      N/A
    // Totals...
    // yes simd        7       14        0      N/A
    //  no simd       15       30        0        0
    fn project_via_origin_onto(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e321] * -1.0;
        let wedge_g1_xyz =
            (Simd32x3::from(right_dual_g0_w) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * other.group1().xyz()) + (right_dual_g0_xyz.zxy() * self.group0().yzx())
                - (right_dual_g0_xyz.yzx() * self.group0().zxy());
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (wedge_g1_xyz * Simd32x4::from(other[e321]).xyz() * Simd32x3::from(-1.0))
                .with_w((wedge_g1_xyz[0] * other[e1]) + (wedge_g1_xyz[1] * other[e2]) + (wedge_g1_xyz[2] * other[e3]) - (right_dual_g0_w * other[e321] * self[scalar])),
            // e15, e25, e35, e3215
            ((wedge_g1_xyz.zxy() * other.group0().yzx()) - (wedge_g1_xyz.yzx() * other.group0().zxy())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<AntiLine> for VersorOdd {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7        9        0        0
    //    simd3        0        4        0      N/A
    // Totals...
    // yes simd        7       13        0      N/A
    //  no simd        7       21        0        0
    fn project_via_origin_onto(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = right_dual_g0 * Simd32x3::from(self[scalar]);
        let wedge_g0_w = (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43])
            - (right_dual_g0[0] * self[e23])
            - (right_dual_g0[1] * self[e31])
            - (right_dual_g0[2] * self[e12]);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(wedge_g0_w) * other.group0()).with_w(-(wedge_g0_xyz[0] * other[e23]) - (wedge_g0_xyz[1] * other[e31]) - (wedge_g0_xyz[2] * other[e12])),
            // e15, e25, e35, e3215
            (Simd32x3::from(wedge_g0_w) * other.group1()).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<AntiMotor> for VersorOdd {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       11        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd       11       17        0      N/A
    //  no simd       19       32        0        0
    fn project_via_origin_onto(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g0 = (right_dual_g0 * Simd32x4::from(self[scalar]))
            + (other.group1().wwwx() * self.group0().xyzx())
            + Simd32x3::from(0.0).with_w(
                (other[e25] * self[e42]) + (other[e35] * self[e43]) + (other[e3215] * self[e1234])
                    - (right_dual_g0[0] * self[e23])
                    - (right_dual_g0[1] * self[e31])
                    - (right_dual_g0[2] * self[e12]),
            );
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x4::from(wedge_g0[3]).xyz() * other.group0().xyz())
                .with_w((wedge_g0[3] * other[scalar]) - (wedge_g0[0] * other[e23]) - (wedge_g0[1] * other[e31]) - (wedge_g0[2] * other[e12])),
            // e15, e25, e35, e3215
            ((Simd32x3::from(wedge_g0[3]) * other.group1().xyz()) + (Simd32x3::from(other[e3215]) * wedge_g0.xyz())).with_w(wedge_g0[3] * other[e3215]),
        )
    }
}
impl ProjectViaOriginOnto<AntiPlane> for VersorOdd {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2       11        0        0
    fn project_via_origin_onto(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        Scalar::from_groups(/* scalar */ (wedge_g0[0] * other[e1]) + (wedge_g0[1] * other[e2]) + (wedge_g0[2] * other[e3]))
    }
}
impl ProjectViaOriginOnto<AntiScalar> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0        9        0      N/A
    //  no simd        0       21        0        0
    fn project_via_origin_onto(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e12345] * -1.0;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual_g0 * other[e12345]) * self.group3(),
        )
    }
}
impl ProjectViaOriginOnto<Circle> for VersorOdd {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        9       13        0      N/A
    // Totals...
    // yes simd       14       19        0      N/A
    //  no simd       32       45        0        0
    fn project_via_origin_onto(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g2_w = -(right_dual_g1_xyz[0] * self[e41])
            - (right_dual_g1_xyz[1] * self[e42])
            - (right_dual_g1_xyz[2] * self[e43])
            - (other[e423] * self[e23])
            - (other[e431] * self[e31])
            - (other[e412] * self[e12]);
        let wedge_g3_xyz = (right_dual_g1_xyz * Simd32x3::from(self[e45])) + (other.group0().yzx() * self.group2().zxy()) + (other.group2().zxy() * self.group0().yzx())
            - (Simd32x3::from(other[e321]) * self.group1().xyz())
            - (other.group0().zxy() * self.group2().yzx())
            - (other.group2().yzx() * self.group0().zxy());
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(wedge_g2_w) * other.group1().xyz()) + (wedge_g3_xyz.yzx() * other.group0().zxy()) - (wedge_g3_xyz.zxy() * other.group0().yzx()),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g2_w) * other.group2()) - (wedge_g3_xyz * Simd32x3::from(other[e321]))).with_w(0.0),
            // e15, e25, e35, scalar
            ((wedge_g3_xyz.zxy() * other.group2().yzx()) - (wedge_g3_xyz.yzx() * other.group2().zxy())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<CircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       18        0        0
    //    simd3       16       19        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd       20       40        0      N/A
    //  no simd       55       87        0        0
    fn project_via_origin_onto(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        let right_dual_g2_xyz = other.group2().xyz();
        let right_dual_g2_w = other[e12345] * -1.0;
        let wedge_g1 = (right_dual_g1 * Simd32x4::from(self[scalar])) + (Simd32x4::from(right_dual_g2_w) * self.group1());
        let wedge_g2_w = right_dual_g2_w * self[e1234];
        let wedge_g3 = ((Simd32x3::from(right_dual_g2_w) * self.group3().xyz())
            + (Simd32x3::from(right_dual_g1[3]) * self.group1().xyz())
            + (Simd32x3::from(self[e45]) * right_dual_g1.xyz())
            + (right_dual_g2_xyz.zxy() * self.group0().yzx())
            + (other.group0().yzx() * self.group2().zxy())
            - (right_dual_g2_xyz.yzx() * self.group0().zxy())
            - (other.group0().zxy() * self.group2().yzx()))
        .with_w(right_dual_g2_w * self[e3215]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x3::from(wedge_g2_w) * other.group1().xyz())
                + (Simd32x3::from(right_dual_g2_w * other[e12345]) * self.group0().xyz())
                + (Simd32x3::from(other[e12345] * self[scalar]) * other.group0())
                + (other.group0().zxy() * wedge_g3.yzx())
                - (other.group0().yzx() * wedge_g3.zxy()))
            .with_w(right_dual_g2_w * other[e12345] * self[scalar]),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g2_w) * other.group2().xyz()) + (Simd32x3::from(wedge_g3[3]) * other.group0()) + (Simd32x3::from(other[e12345]) * wedge_g1.xyz())
                - (Simd32x3::from(other[e321]) * wedge_g3.xyz()))
            .with_w(wedge_g1[3] * other[e12345]),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g3[2] * other[e315]) - (wedge_g3[1] * other[e125]),
                (wedge_g3[0] * other[e125]) - (wedge_g3[2] * other[e235]),
                (wedge_g3[1] * other[e235]) - (wedge_g3[0] * other[e315]),
            ]) + (right_dual_g2_xyz * Simd32x3::from(other[e12345] * self[scalar]))
                + (Simd32x3::from(wedge_g3[3]) * other.group1().xyz())
                + (Simd32x3::from(right_dual_g2_w * other[e12345]) * self.group2().xyz()))
            .with_w(wedge_g2_w * other[e12345]),
            // e4235, e4315, e4125, e3215
            wedge_g3 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Dipole> for VersorOdd {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       18       20        0        0
    //    simd3        0        6        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd       18       29        0      N/A
    //  no simd       18       50        0        0
    fn project_via_origin_onto(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        let wedge_g0 = right_dual_g0 * Simd32x3::from(self[scalar]);
        let wedge_g1 = right_dual_g1 * Simd32x4::from(self[scalar]);
        let wedge_g2_xyz = right_dual_g2 * Simd32x3::from(self[scalar]);
        let wedge_g2_w = -(right_dual_g0[0] * self[e15])
            - (right_dual_g0[1] * self[e25])
            - (right_dual_g0[2] * self[e35])
            - (right_dual_g2[0] * self[e41])
            - (right_dual_g2[1] * self[e42])
            - (right_dual_g2[2] * self[e43])
            - (right_dual_g1[0] * self[e23])
            - (right_dual_g1[1] * self[e31])
            - (right_dual_g1[2] * self[e12])
            - (right_dual_g1[3] * self[e45]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g2_w) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(wedge_g2_w) * other.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(wedge_g2_w) * other.group2()).with_w(
                -(wedge_g0[0] * other[e15])
                    - (wedge_g0[1] * other[e25])
                    - (wedge_g0[2] * other[e35])
                    - (wedge_g2_xyz[0] * other[e41])
                    - (wedge_g2_xyz[1] * other[e42])
                    - (wedge_g2_xyz[2] * other[e43])
                    - (wedge_g1[0] * other[e23])
                    - (wedge_g1[1] * other[e31])
                    - (wedge_g1[2] * other[e12])
                    - (wedge_g1[3] * other[e45]),
            ),
        )
    }
}
impl ProjectViaOriginOnto<DipoleInversion> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       22       34        0        0
    //    simd2        1        3        0      N/A
    //    simd3       14       19        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd       39       60        0      N/A
    //  no simd       74      113        0        0
    fn project_via_origin_onto(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let wedge_g0 = (self.group1().xyzy() * Simd32x3::from(other[e1234]).with_w(other[e31]))
            + ((right_dual_g0 * Simd32x3::from(self[scalar]))
                + ((other.group3().yz() * self.group0().zx()) - (other.group3().zx() * self.group0().yz())).with_z((other[e4235] * self[e42]) - (other[e4315] * self[e41])))
            .with_w(
                (other[e23] * self[e23]) + (other[e12] * self[e12])
                    - (right_dual_g0[0] * self[e15])
                    - (right_dual_g0[1] * self[e25])
                    - (right_dual_g0[2] * self[e35])
                    - (other[e45] * self[e45])
                    - (other[e4235] * self[e4235])
                    - (other[e4315] * self[e4315]),
            );
        let wedge_g1_xyz =
            (Simd32x3::from(other[e1234]) * self.group2().xyz()) + (Simd32x3::from(other[e3215]) * self.group0().xyz()) + (Simd32x3::from(self[e45]) * other.group3().xyz())
                - (Simd32x3::from(self[scalar]) * other.group1().xyz());
        let wedge_g1_w = other[e45] * self[scalar];
        let wedge_g2_xyz = Simd32x3::from([
            (other[e4125] * self[e25]) - (other[e4315] * self[e35]),
            (other[e4235] * self[e35]) - (other[e4125] * self[e15]),
            (other[e4315] * self[e15]) - (other[e4235] * self[e25]),
        ]) + (Simd32x3::from(other[e3215]) * self.group1().xyz())
            - (Simd32x3::from(self[scalar]) * other.group2().xyz());
        let wedge_g3 = Simd32x4::from(self[scalar]) * (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group3().yzxw() * wedge_g0.zxy().with_w(other[e1234] * self[scalar]))
                + ((wedge_g1_xyz * Simd32x3::from(other[e1234]))
                    + (Simd32x3::from(wedge_g0[3]) * other.group0())
                    + -(wedge_g0.yz() * other.group3().zx()).with_z(wedge_g0[0] * other[e4315] * -1.0))
                .with_w(
                    (wedge_g3[0] * other[e4235]) + (wedge_g3[1] * other[e4315])
                        - (wedge_g1_w * other[e45])
                        - (wedge_g1_xyz[0] * other[e23])
                        - (wedge_g1_xyz[1] * other[e31])
                        - (wedge_g1_xyz[2] * other[e12])
                        - (wedge_g2_xyz[0] * other[e41])
                        - (wedge_g2_xyz[1] * other[e42])
                        - (wedge_g2_xyz[2] * other[e43])
                        - (wedge_g0[0] * other[e15])
                        - (wedge_g0[1] * other[e25])
                        - (wedge_g0[2] * other[e35]),
                ),
            // e23, e31, e12, e45
            ((wedge_g2_xyz * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0[3]) * other.group1().xyz()) + (Simd32x3::from(other[e3215]) * wedge_g0.xyz())
                - (Simd32x3::from(wedge_g1_w) * other.group3().xyz()))
            .with_w(wedge_g0[3] * other[e45]),
            // e15, e25, e35, e1234
            ((wedge_g1_xyz * Simd32x3::from(other[e3215])) + (Simd32x3::from(wedge_g0[3]) * other.group2().xyz()) + (wedge_g2_xyz.yzx() * other.group3().zxy())
                - (wedge_g2_xyz.zxy() * other.group3().yzx()))
            .with_w(wedge_g0[3] * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0[3]) * other.group3(),
        )
    }
}
impl ProjectViaOriginOnto<DualNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2       13        0        0
    //    simd3        0        3        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        2       20        0      N/A
    //  no simd        2       38        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(other[e12345] * -1.0) * self.group0();
        let wedge_g2 = Simd32x4::from(other[e12345] * -1.0) * self.group2();
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (wedge_g0.xyz() * Simd32x2::from(other[e12345]).with_z(other[e12345])).with_w((other[e5] * wedge_g2[3]) + (other[e12345] * wedge_g0[3])),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345] * other[e12345] * -1.0) * self.group1(),
            // e15, e25, e35, e1234
            wedge_g2 * Simd32x4::from(other[e12345]),
            // e4235, e4315, e4125, e3215
            (self.group3().xyz() * Simd32x2::from(other[e12345] * other[e12345] * -1.0).with_z(other[e12345] * other[e12345]) * Simd32x3::from([1.0, 1.0, -1.0]))
                .with_w(-(other[e12345] * other[e12345] * self[e3215]) - (other[e5] * other[e12345] * self[scalar])),
        )
    }
}
impl ProjectViaOriginOnto<FlatPoint> for VersorOdd {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       14        0        0
    fn project_via_origin_onto(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let wedge_g2_w = (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43]) - (other[e45] * self[e45]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(wedge_g2_w * other[e45]),
            // e15, e25, e35, scalar
            other.group0() * Simd32x3::from(wedge_g2_w).with_w(other[e45] * self[scalar]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl ProjectViaOriginOnto<Flector> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       20        0        0
    //    simd2        1        3        0      N/A
    //    simd3        5        7        0      N/A
    //    simd4        4        5        0      N/A
    // Totals...
    // yes simd       19       35        0      N/A
    //  no simd       42       67        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (self.group0().zxyx() * other.group1().yzx().with_w(other[e15]))
            + -(other.group1().zx() * self.group0().yz()).with_zw(other[e4315] * self[e41] * -1.0, -(other[e45] * self[e45]) - (other[e4235] * self[e4235]));
        let wedge_g1 = (other.group1().wwwx() * self.group0().xyz().with_w(self[e23]))
            + (Simd32x3::from(self[e45]) * other.group1().xyz()).with_w((other[e45] * self[scalar]) + (other[e4315] * self[e31]) + (other[e4125] * self[e12]));
        let wedge_g2_xyz = Simd32x3::from([
            (other[e4125] * self[e25]) - (other[e4315] * self[e35]),
            (other[e4235] * self[e35]) - (other[e4125] * self[e15]),
            (other[e4315] * self[e15]) - (other[e4235] * self[e25]),
        ]) + (Simd32x3::from(other[e3215]) * self.group1().xyz())
            - (Simd32x3::from(self[scalar]) * other.group0().xyz());
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((wedge_g0.zx() * other.group1().yz()) - (wedge_g0.yz() * other.group1().zx())).with_zw(
                (wedge_g0[1] * other[e4235]) - (wedge_g0[0] * other[e4315]),
                -(wedge_g1[3] * other[e45]) - (other[e4235] * other[e4235] * self[scalar]),
            ),
            // e23, e31, e12, e45
            (wedge_g0 * Simd32x3::from(other[e3215]).with_w(other[e45])) + Simd32x3::from(0.0).with_w(-(wedge_g1[1] * other[e4315]) - (wedge_g1[2] * other[e4125]))
                - (wedge_g1.wwwx() * other.group1().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(wedge_g0[3]) * other.group0().xyz()) + (Simd32x3::from(other[e3215]) * wedge_g1.xyz()) + (wedge_g2_xyz.yzx() * other.group1().zxy())
                - (wedge_g2_xyz.zxy() * other.group1().yzx()))
            .with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0[3]) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Line> for VersorOdd {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        3        7        0      N/A
    // Totals...
    // yes simd        7       13        0      N/A
    //  no simd       13       27        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g2_w = -(other[e415] * self[e41]) - (other[e425] * self[e42]) - (other[e435] * self[e43]);
        let wedge_g3_xyz = (Simd32x3::from(self[e45]) * other.group0()) + (other.group1().zxy() * self.group0().yzx()) - (other.group1().yzx() * self.group0().zxy());
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g2_w) * other.group0(),
            // e23, e31, e12, e45
            (Simd32x3::from(wedge_g2_w) * other.group1()).with_w(-(wedge_g3_xyz[0] * other[e415]) - (wedge_g3_xyz[1] * other[e425]) - (wedge_g3_xyz[2] * other[e435])),
            // e15, e25, e35, scalar
            ((wedge_g3_xyz.zxy() * other.group1().yzx()) - (wedge_g3_xyz.yzx() * other.group1().zxy())).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       20        0        0
    //    simd3        8       11        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd       18       35        0      N/A
    //  no simd       40       69        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let right_dual_g1_xyz = other.group1().xyz();
        let wedge_g0 = Simd32x4::from(right_dual_g0_w) * self.group0();
        let wedge_g1_xyz = (right_dual_g0_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_dual_g0_w) * self.group1().xyz());
        let wedge_g2_w = right_dual_g0_w * self[e1234];
        let wedge_g3 =
            ((right_dual_g0_xyz * Simd32x3::from(self[e45])) + (Simd32x3::from(right_dual_g0_w) * self.group3().xyz()) + (right_dual_g1_xyz.zxy() * self.group0().yzx())
                - (right_dual_g1_xyz.yzx() * self.group0().zxy()))
            .with_w(right_dual_g0_w * self[e3215]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (wedge_g0 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g2_w) * other.group0().xyz().with_w(other[e5]))
                + Simd32x3::from(0.0).with_w(
                    -(wedge_g1_xyz[0] * other[e415])
                        - (wedge_g1_xyz[1] * other[e425])
                        - (wedge_g1_xyz[2] * other[e435])
                        - (wedge_g0[0] * other[e235])
                        - (wedge_g0[1] * other[e315])
                        - (wedge_g0[2] * other[e125]),
                ),
            // e23, e31, e12, e45
            ((wedge_g1_xyz * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g2_w) * other.group1().xyz())).with_w(right_dual_g0_w * other[e12345] * self[e45]),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g3[2] * other[e315]) - (wedge_g3[1] * other[e125]),
                (wedge_g3[0] * other[e125]) - (wedge_g3[2] * other[e235]),
                (wedge_g3[1] * other[e235]) - (wedge_g3[0] * other[e315]),
            ]) + (right_dual_g1_xyz * Simd32x3::from(other[e12345] * self[scalar]))
                + (Simd32x3::from(wedge_g3[3]) * other.group0().xyz())
                + (Simd32x3::from(right_dual_g0_w * other[e12345]) * self.group2().xyz()))
            .with_w(wedge_g2_w * other[e12345]),
            // e4235, e4315, e4125, e3215
            wedge_g3 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       80      105        0        0
    //    simd2        0        2        0      N/A
    //    simd3       52       65        0      N/A
    //    simd4       10       13        0      N/A
    // Totals...
    // yes simd      142      185        0      N/A
    //  no simd      276      356        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_dual_g1 = (other.group9().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let right_dual_g3 = other.group8().with_w(other[e321] * -1.0);
        let right_dual_g5 = other.group6().xyz();
        let right_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let wedge_g0_y = (right_dual_g0[1] * self[scalar])
            + (right_dual_g1[0] * self[e4235])
            + (right_dual_g1[1] * self[e4315])
            + (right_dual_g1[2] * self[e4125])
            + (right_dual_g1[3] * self[e3215])
            + (other[e3215] * self[e1234])
            - (right_dual_g6_xyz[0] * self[e23])
            - (right_dual_g6_xyz[1] * self[e31])
            - (right_dual_g6_xyz[2] * self[e12])
            - (right_dual_g7[0] * self[e15])
            - (right_dual_g7[1] * self[e25])
            - (right_dual_g7[2] * self[e35])
            - (right_dual_g8[0] * self[e41])
            - (right_dual_g8[1] * self[e42])
            - (right_dual_g8[2] * self[e43])
            - (other[e45] * self[e45]);
        let wedge_g1 = right_dual_g1 * Simd32x4::from(self[scalar]);
        let wedge_g2 = other[e3215] * self[scalar];
        let wedge_g3 = (right_dual_g3 * Simd32x4::from(self[scalar])) + (Simd32x4::from(right_dual_g0[0]) * self.group2().xyz().with_w(self[e45]));
        let wedge_g4 = (Simd32x3::from(right_dual_g0[0]) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * other.group7());
        let wedge_g5 = (right_dual_g5 * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_dual_g0[0]) * self.group1().xyz());
        let wedge_g6 =
            ((right_dual_g6_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_dual_g1[3]) * self.group2().xyz()) + (Simd32x3::from(other[e3215]) * self.group0().xyz())
                - (Simd32x3::from(self[e45]) * right_dual_g1.xyz()))
            .with_w(other[e45] * self[scalar]);
        let wedge_g7 = Simd32x3::from([
            (right_dual_g1[2] * self[e42]) - (right_dual_g1[1] * self[e43]),
            (right_dual_g1[0] * self[e43]) - (right_dual_g1[2] * self[e41]),
            (right_dual_g1[1] * self[e41]) - (right_dual_g1[0] * self[e42]),
        ]) + (right_dual_g7 * Simd32x3::from(self[scalar]))
            + (Simd32x3::from(right_dual_g1[3]) * self.group1().xyz());
        let wedge_g8 = Simd32x3::from([
            (right_dual_g1[1] * self[e35]) - (right_dual_g1[2] * self[e25]),
            (right_dual_g1[2] * self[e15]) - (right_dual_g1[0] * self[e35]),
            (right_dual_g1[0] * self[e25]) - (right_dual_g1[1] * self[e15]),
        ]) + (right_dual_g8 * Simd32x3::from(self[scalar]))
            + (Simd32x3::from(other[e3215]) * self.group1().xyz());
        let wedge_g9 = (Simd32x3::from([
            (right_dual_g3[2] * self[e42]) - (right_dual_g3[1] * self[e43]),
            (right_dual_g3[0] * self[e43]) - (right_dual_g3[2] * self[e41]),
            (right_dual_g3[1] * self[e41]) - (right_dual_g3[0] * self[e42]),
        ]) + (right_dual_g5 * Simd32x3::from(self[e45]))
            + (Simd32x3::from(right_dual_g0[0]) * self.group3().xyz())
            + (Simd32x3::from(right_dual_g3[3]) * self.group1().xyz())
            + (Simd32x3::from(self[scalar]) * other.group1().xyz())
            + (other.group7().yzx() * self.group2().zxy())
            - (other.group7().zxy() * self.group2().yzx()))
        .with_w(right_dual_g0[0] * self[e3215]);
        let wedge_g10 = (right_dual_g0[0] * self[e1234])
            - (right_dual_g5[0] * self[e41])
            - (right_dual_g5[1] * self[e42])
            - (right_dual_g5[2] * self[e43])
            - (other[e423] * self[e23])
            - (other[e431] * self[e31])
            - (other[e412] * self[e12])
            - (other[e4] * self[scalar]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (wedge_g0_y * other[scalar])
                    + (wedge_g10 * other[e5])
                    + (wedge_g2 * other[e1234])
                    + (wedge_g1[0] * other[e4235])
                    + (wedge_g1[1] * other[e4315])
                    + (wedge_g1[2] * other[e4125])
                    + (wedge_g1[3] * other[e3215])
                    + (wedge_g9[0] * other[e1])
                    + (wedge_g9[1] * other[e2])
                    + (wedge_g9[2] * other[e3])
                    + (wedge_g9[3] * other[e4])
                    + (right_dual_g0[0] * other[e12345] * self[scalar])
                    - (wedge_g4[0] * other[e235])
                    - (wedge_g4[1] * other[e315])
                    - (wedge_g4[2] * other[e125])
                    - (wedge_g5[0] * other[e415])
                    - (wedge_g5[1] * other[e425])
                    - (wedge_g5[2] * other[e435])
                    - (wedge_g7[0] * other[e15])
                    - (wedge_g7[1] * other[e25])
                    - (wedge_g7[2] * other[e35])
                    - (wedge_g8[0] * other[e41])
                    - (wedge_g8[1] * other[e42])
                    - (wedge_g8[2] * other[e43])
                    - (other[e23] * wedge_g6[0])
                    - (other[e31] * wedge_g6[1])
                    - (other[e12] * wedge_g6[2])
                    - (other[e423] * wedge_g3[0])
                    - (other[e431] * wedge_g3[1])
                    - (other[e412] * wedge_g3[2])
                    - (wedge_g3[3] * other[e321])
                    - (wedge_g6[3] * other[e45]),
                wedge_g0_y * other[e12345],
            ]),
            // e1, e2, e3, e4
            (wedge_g1 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group1())
                + (other.group9().yzxx() * wedge_g5.zxy().with_w(wedge_g4[0]))
                + ((Simd32x3::from(wedge_g6[3]) * other.group6().xyz())
                    + (Simd32x3::from(wedge_g9[3]) * other.group4())
                    + (Simd32x3::from(other[e321]) * wedge_g6.xyz())
                    + (Simd32x3::from(other[e1234]) * wedge_g3.xyz())
                    + (wedge_g7.zxy() * other.group8().yzx())
                    + (wedge_g8.yzx() * other.group7().zxy())
                    + (other.group5().yzx() * wedge_g9.zxy())
                    - (wedge_g4 * Simd32x3::from(other[e3215]))
                    - (Simd32x3::from(wedge_g10) * other.group3().xyz())
                    - (wedge_g5.yzx() * other.group9().zxy())
                    - (wedge_g7.yzx() * other.group8().zxy())
                    - (wedge_g8.zxy() * other.group7().yzx())
                    - (other.group5().zxy() * wedge_g9.yzx()))
                .with_w((wedge_g4[1] * other[e4315]) + (wedge_g4[2] * other[e4125]) + (wedge_g3[3] * other[e1234])),
            // e5
            (wedge_g0_y * other[e5])
                + (wedge_g2 * other[e12345])
                + (wedge_g9[0] * other[e15])
                + (wedge_g9[1] * other[e25])
                + (wedge_g9[2] * other[e35])
                + (wedge_g9[3] * other[e45])
                - (wedge_g8[0] * other[e415])
                - (wedge_g8[1] * other[e425])
                - (wedge_g8[2] * other[e435])
                - (other[e235] * wedge_g6[0])
                - (other[e315] * wedge_g6[1])
                - (other[e125] * wedge_g6[2])
                - (wedge_g3[0] * other[e4235])
                - (wedge_g3[1] * other[e4315])
                - (wedge_g3[2] * other[e4125])
                - (wedge_g3[3] * other[e3215]),
            // e15, e25, e35, e45
            (wedge_g3 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group3())
                + ((Simd32x3::from(wedge_g9[3]) * other.group6().xyz())
                    + (Simd32x3::from(other[e3215]) * wedge_g6.xyz())
                    + (wedge_g8.yzx() * other.group9().zxy())
                    + (other.group8().yzx() * wedge_g9.zxy())
                    - (wedge_g8.zxy() * other.group9().yzx())
                    - (other.group8().zxy() * wedge_g9.yzx()))
                .with_w(0.0),
            // e41, e42, e43
            (wedge_g4 * Simd32x3::from(other[e12345]))
                + (Simd32x3::from(wedge_g0_y) * other.group4())
                + (Simd32x3::from(wedge_g10) * other.group6().xyz())
                + (Simd32x3::from(other[e1234]) * wedge_g6.xyz())
                + (wedge_g7.zxy() * other.group9().yzx())
                + (other.group7().zxy() * wedge_g9.yzx())
                - (wedge_g7.yzx() * other.group9().zxy())
                - (other.group7().yzx() * wedge_g9.zxy()),
            // e23, e31, e12
            (wedge_g5 * Simd32x3::from(other[e12345]))
                + (wedge_g7 * Simd32x3::from(other[e3215]))
                + (wedge_g8 * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(wedge_g0_y) * other.group5())
                + (Simd32x3::from(wedge_g10) * other.group8())
                + (Simd32x3::from(wedge_g9[3]) * other.group7())
                - (Simd32x3::from(wedge_g6[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e321]) * wedge_g9.xyz()),
            // e415, e425, e435, e321
            (wedge_g6 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g0_y) * other.group6())
                + (wedge_g9.yzxw() * other.group9().zxy().with_w(other[e1234]))
                + -(wedge_g9.zx() * other.group9().yz()).with_zw(wedge_g9[1] * other[e4235] * -1.0, wedge_g10 * other[e3215] * -1.0),
            // e423, e431, e412
            (wedge_g7 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group7()) + (Simd32x3::from(other[e1234]) * wedge_g9.xyz())
                - (Simd32x3::from(wedge_g10) * other.group9().xyz()),
            // e235, e315, e125
            (wedge_g8 * Simd32x3::from(other[e12345])) + (Simd32x3::from(wedge_g0_y) * other.group8()) + (Simd32x3::from(wedge_g9[3]) * other.group9().xyz())
                - (Simd32x3::from(other[e3215]) * wedge_g9.xyz()),
            // e4235, e4315, e4125, e3215
            (wedge_g9 * Simd32x4::from(other[e12345])) + (Simd32x4::from(wedge_g0_y) * other.group9()),
            // e1234
            (wedge_g0_y * other[e1234]) + (wedge_g10 * other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       17        0        0
    //    simd2        2        4        0      N/A
    //    simd3        4        6        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       11       29        0      N/A
    //  no simd       21       51        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xy = (other.group0().yz() * self.group0().zx()) - (other.group0().zx() * self.group0().yz());
        let wedge_g0_z = (other[e4235] * self[e42]) - (other[e4315] * self[e41]);
        let wedge_g2_xyz = Simd32x3::from([
            (other[e4125] * self[e25]) - (other[e4315] * self[e35]),
            (other[e4235] * self[e35]) - (other[e4125] * self[e15]),
            (other[e4315] * self[e15]) - (other[e4235] * self[e25]),
        ]) + (Simd32x3::from(other[e3215]) * self.group1().xyz());
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x2::from([wedge_g0_z, wedge_g0_xy[0]]) * other.group0().yz()) - (Simd32x2::from([wedge_g0_xy[1], wedge_g0_z]) * other.group0().zx()))
                .with_zw((wedge_g0_xy[1] * other[e4235]) - (wedge_g0_xy[0] * other[e4315]), other[e4235] * other[e4235] * self[scalar] * -1.0),
            // e23, e31, e12, e45
            Simd32x4::from([1.0, 1.0, wedge_g0_z, 0.0]) * (Simd32x4::from(other[e3215]).xyz() * wedge_g0_xy.with_z(1.0)).with_w(0.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e3215] * other[e3215]) * self.group0().xyz())
                + (Simd32x3::from(other[e3215] * self[e45]) * other.group0().xyz())
                + (wedge_g2_xyz.yzx() * other.group0().zxy())
                - (wedge_g2_xyz.zxy() * other.group0().yzx()))
            .with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e4235] * self[e4235] * -1.0) * other.group0(),
        )
    }
}
impl ProjectViaOriginOnto<RoundPoint> for VersorOdd {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3        9        0        0
    fn project_via_origin_onto(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(self[scalar]) * other.group0().xyz();
        Scalar::from_groups(
            // scalar
            (wedge_g0_xyz[0] * other[e1]) + (wedge_g0_xyz[1] * other[e2]) + (wedge_g0_xyz[2] * other[e3]) - 2.0 * (other[e4] * self[scalar] * other[e5]),
        )
    }
}
impl ProjectViaOriginOnto<Scalar> for VersorOdd {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn project_via_origin_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * other[scalar] * other[scalar])
    }
}
impl ProjectViaOriginOnto<Sphere> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       12        0        0
    //    simd2        0        2        0      N/A
    //    simd3        7       11        0      N/A
    //    simd4        4        5        0      N/A
    // Totals...
    // yes simd       14       30        0      N/A
    //  no simd       40       69        0        0
    fn project_via_origin_onto(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let wedge_g0 = Simd32x4::from([0.0, 0.0, right_dual_g0[0] * self[e42] * -1.0, 0.0])
            + (right_dual_g0.zxyx() * self.group0().yzx().with_w(self[e4235]))
            + (right_dual_g0.wwwy() * self.group1().xyz().with_w(self[e4315]))
            + -(right_dual_g0.yz() * self.group0().zx()).with_zw(0.0, 0.0);
        let wedge_g1_xyz =
            (Simd32x3::from(right_dual_g0[3]) * self.group2().xyz()) + (Simd32x3::from(other[e3215]) * self.group0().xyz()) - (Simd32x3::from(self[e45]) * right_dual_g0.xyz());
        let wedge_g2_xyz = Simd32x3::from([
            (right_dual_g0[1] * self[e35]) - (right_dual_g0[2] * self[e25]),
            (right_dual_g0[2] * self[e15]) - (right_dual_g0[0] * self[e35]),
            (right_dual_g0[0] * self[e25]) - (right_dual_g0[1] * self[e15]),
        ]) + (Simd32x3::from(other[e3215]) * self.group1().xyz());
        let wedge_g3 = right_dual_g0 * Simd32x4::from(self[scalar]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0().yzxy() * wedge_g0.zxy().with_w(wedge_g3[1]))
                + ((wedge_g1_xyz * Simd32x3::from(other[e1234])) + -(wedge_g0.yz() * other.group0().zx()).with_z(wedge_g0[0] * other[e4315] * -1.0))
                    .with_w(wedge_g3[0] * other[e4235]),
            // e23, e31, e12, e45
            ((wedge_g2_xyz * Simd32x3::from(other[e1234])) + (Simd32x3::from(other[e3215]) * wedge_g0.xyz())).with_w(0.0),
            // e15, e25, e35, e1234
            ((wedge_g1_xyz * Simd32x3::from(other[e3215])) + (wedge_g2_xyz.yzx() * other.group0().zxy()) - (wedge_g2_xyz.zxy() * other.group0().yzx()))
                .with_w(wedge_g0[3] * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0[3]) * other.group0(),
        )
    }
}
impl ProjectViaOriginOnto<VersorEven> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       20       37        0        0
    //    simd2        0        1        0      N/A
    //    simd3       12       14        0      N/A
    //    simd4        6        8        0      N/A
    // Totals...
    // yes simd       38       60        0      N/A
    //  no simd       80      113        0        0
    fn project_via_origin_onto(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let right_dual_g0_w = other[e12345] * -1.0;
        let right_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        let right_dual_g2 = other.group2().xyz().with_w(other[e4] * -1.0);
        let wedge_g0 = ((right_dual_g0_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_dual_g0_w) * self.group0().xyz())).with_w(right_dual_g0_w * self[scalar]);
        let wedge_g1 = (right_dual_g1 * Simd32x4::from(self[scalar])) + (Simd32x4::from(right_dual_g0_w) * self.group1());
        let wedge_g2 = (right_dual_g2 * Simd32x4::from(self[scalar]))
            + (Simd32x4::from(right_dual_g0_w) * self.group2())
            + Simd32x3::from(0.0).with_w(
                -(right_dual_g0_xyz[0] * self[e23])
                    - (right_dual_g0_xyz[1] * self[e31])
                    - (right_dual_g0_xyz[2] * self[e12])
                    - (right_dual_g1[0] * self[e41])
                    - (right_dual_g1[1] * self[e42])
                    - (right_dual_g1[2] * self[e43]),
            );
        let wedge_g3 = (Simd32x3::from([
            (right_dual_g2[2] * self[e42]) - (right_dual_g2[1] * self[e43]),
            (right_dual_g2[0] * self[e43]) - (right_dual_g2[2] * self[e41]),
            (right_dual_g2[1] * self[e41]) - (right_dual_g2[0] * self[e42]),
        ]) + (Simd32x3::from(right_dual_g0_w) * self.group3().xyz())
            + (Simd32x3::from(right_dual_g1[3]) * self.group1().xyz())
            + (Simd32x3::from(self[scalar]) * other.group3().xyz())
            + (Simd32x3::from(self[e45]) * right_dual_g1.xyz())
            + (right_dual_g0_xyz.yzx() * self.group2().zxy())
            - (right_dual_g0_xyz.zxy() * self.group2().yzx()))
        .with_w(right_dual_g0_w * self[e3215]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (wedge_g0 * Simd32x4::from(other[e12345]))
                + (Simd32x4::from(wedge_g2[3]) * other.group1().xyz().with_w(other[e5]))
                + (wedge_g3.yzxx() * other.group0().zxy().with_w(other[e1]))
                + -(wedge_g3.zx() * other.group0().yz()).with_zw(
                    wedge_g3[1] * other[e423] * -1.0,
                    -(wedge_g0[0] * other[e235])
                        - (wedge_g0[1] * other[e315])
                        - (wedge_g0[2] * other[e125])
                        - (wedge_g1[0] * other[e415])
                        - (wedge_g1[1] * other[e425])
                        - (wedge_g1[2] * other[e435])
                        - (wedge_g1[3] * other[e321])
                        - (wedge_g2[0] * other[e423])
                        - (wedge_g2[1] * other[e431])
                        - (wedge_g2[2] * other[e412]),
                ),
            // e23, e31, e12, e45
            ((Simd32x3::from(wedge_g2[3]) * other.group2().xyz()) + (Simd32x3::from(wedge_g3[3]) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * wedge_g1.xyz())
                - (Simd32x3::from(other[e321]) * wedge_g3.xyz()))
            .with_w(wedge_g1[3] * other[e12345]),
            // e15, e25, e35, e1234
            (Simd32x3::from([
                (wedge_g3[2] * other[e315]) - (wedge_g3[1] * other[e125]),
                (wedge_g3[0] * other[e125]) - (wedge_g3[2] * other[e235]),
                (wedge_g3[1] * other[e235]) - (wedge_g3[0] * other[e315]),
            ]) + (Simd32x3::from(wedge_g3[3]) * other.group1().xyz())
                + (Simd32x3::from(other[e12345]) * wedge_g2.xyz()))
            .with_w(wedge_g2[3] * other[e12345]),
            // e4235, e4315, e4125, e3215
            wedge_g3 * Simd32x4::from(other[e12345]),
        )
    }
}
impl ProjectViaOriginOnto<VersorOdd> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       19       27        0        0
    //    simd2        1        2        0      N/A
    //    simd3       14       21        0      N/A
    //    simd4        3        5        0      N/A
    // Totals...
    // yes simd       37       55        0      N/A
    //  no simd       75      114        0        0
    fn project_via_origin_onto(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_dual_g2_xyz = other.group2().xyz() * Simd32x3::from(-1.0);
        let right_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        let wedge_g0 = (right_dual_g0 * Simd32x4::from(self[scalar]))
            + (self.group1().xyzx() * Simd32x3::from(other[e1234]).with_w(other[e23]))
            + (right_dual_g3_xyz.zxy() * self.group0().yzx()).with_w(
                (right_dual_g3_xyz[0] * self[e4235])
                    - (right_dual_g2_xyz[1] * self[e42])
                    - (right_dual_g2_xyz[2] * self[e43])
                    - (right_dual_g0[0] * self[e15])
                    - (right_dual_g0[1] * self[e25])
                    - (right_dual_g0[2] * self[e35])
                    - (other[e45] * self[e45]),
            )
            - (self.group0().zxyx() * right_dual_g3_xyz.yzx().with_w(right_dual_g2_xyz[0]));
        let wedge_g1_xyz = (Simd32x3::from(other[e1234]) * self.group2().xyz()) + (Simd32x3::from(other[e3215]) * self.group0().xyz())
            - (right_dual_g3_xyz * Simd32x3::from(self[e45]))
            - (Simd32x3::from(self[scalar]) * other.group1().xyz());
        let wedge_g1_w = other[e45] * self[scalar];
        let wedge_g2_xyz =
            (right_dual_g2_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(other[e3215]) * self.group1().xyz()) + (right_dual_g3_xyz.yzx() * self.group2().zxy())
                - (right_dual_g3_xyz.zxy() * self.group2().yzx());
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((wedge_g1_xyz * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(wedge_g0[3]) * other.group0().xyz())
                + ((wedge_g0.zx() * other.group3().yz()) - (wedge_g0.yz() * other.group3().zx())).with_z((wedge_g0[1] * other[e4235]) - (wedge_g0[0] * other[e4315])))
            .with_w(
                (wedge_g0[3] * other[scalar]) + (right_dual_g3_xyz[0] * other[e4235] * self[scalar]) + (other[e1234] * other[e3215] * self[scalar])
                    - (wedge_g1_w * other[e45])
                    - (wedge_g1_xyz[0] * other[e23])
                    - (wedge_g1_xyz[1] * other[e31])
                    - (wedge_g1_xyz[2] * other[e12])
                    - (wedge_g2_xyz[0] * other[e41])
                    - (wedge_g2_xyz[1] * other[e42])
                    - (wedge_g2_xyz[2] * other[e43])
                    - (wedge_g0[0] * other[e15])
                    - (wedge_g0[1] * other[e25])
                    - (wedge_g0[2] * other[e35]),
            ),
            // e23, e31, e12, e45
            ((wedge_g2_xyz * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0[3]) * other.group1().xyz()) + (Simd32x3::from(other[e3215]) * wedge_g0.xyz())
                - (Simd32x3::from(wedge_g1_w) * other.group3().xyz()))
            .with_w(wedge_g0[3] * other[e45]),
            // e15, e25, e35, e1234
            ((wedge_g1_xyz * Simd32x3::from(other[e3215])) + (Simd32x3::from(wedge_g0[3]) * other.group2().xyz()) + (wedge_g2_xyz.yzx() * other.group3().zxy())
                - (wedge_g2_xyz.zxy() * other.group3().yzx()))
            .with_w(wedge_g0[3] * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(wedge_g0[3]) * other.group3(),
        )
    }
}
