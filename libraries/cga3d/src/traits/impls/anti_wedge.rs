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
//   Median:         4       8       0
//  Average:         6      11       0
//  Maximum:       111     139       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         5      12       0
//  Average:        11      20       0
//  Maximum:       211     243       0
impl std::ops::Div<AntiWedgeInfix> for AntiCircleRotor {
    type Output = AntiWedgeInfixPartial<AntiCircleRotor>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiDipoleInversion> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(self[e41] * other[e235])
                - (self[e42] * other[e315])
                - (self[e43] * other[e125])
                - (self[e23] * other[e415])
                - (self[e31] * other[e425])
                - (self[e12] * other[e435])
                - (self[e45] * other[e321])
                - (self[e15] * other[e423])
                - (self[e25] * other[e431])
                - (self[e35] * other[e412]),
        )
    }
}
impl AntiWedge<AntiDualNum> for AntiCircleRotor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e3215]) * self.group0().with_w(self[e45]))
    }
}
impl AntiWedge<AntiFlatPoint> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(self[e41] * other[e235]) - (self[e42] * other[e315]) - (self[e43] * other[e125]) - (self[e45] * other[e321]),
        )
    }
}
impl AntiWedge<AntiFlector> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(self[e41] * other[e235]) - (self[e42] * other[e315]) - (self[e43] * other[e125]) - (self[e45] * other[e321]),
        )
    }
}
impl AntiWedge<AntiMotor> for AntiCircleRotor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e3215]) * self.group0().with_w(self[e45]))
    }
}
impl AntiWedge<AntiScalar> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345]) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(other[e12345]) * self.group2(),
        )
    }
}
impl AntiWedge<Circle> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(self[e41] * other[e235])
                - (self[e42] * other[e315])
                - (self[e43] * other[e125])
                - (self[e23] * other[e415])
                - (self[e31] * other[e425])
                - (self[e12] * other[e435])
                - (self[e45] * other[e321])
                - (self[e15] * other[e423])
                - (self[e25] * other[e431])
                - (self[e35] * other[e412]),
        )
    }
}
impl AntiWedge<CircleRotor> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       11        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       14        0
    //  no simd       10       21        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345]) * self.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(other[e12345]) * self.group2().xyz()).with_w(
                (self[scalar] * other[e12345])
                    - (self[e41] * other[e235])
                    - (self[e42] * other[e315])
                    - (self[e43] * other[e125])
                    - (self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435])
                    - (self[e45] * other[e321])
                    - (self[e15] * other[e423])
                    - (self[e25] * other[e431])
                    - (self[e35] * other[e412]),
            ),
        )
    }
}
impl AntiWedge<DipoleInversion> for AntiCircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       20        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e3215]) + (self[e31] * other[e4125]),
                (self[e42] * other[e3215]) + (self[e12] * other[e4235]),
                (self[e43] * other[e3215]) + (self[e23] * other[e4315]),
                -(self[e43] * other[e4125]) - (self[e45] * other[e1234]),
            ]) - (other.group3().yzxx() * self.group1().zxy().with_w(self[e41]))
                - (self.group2().xyz() * other.group2().www()).with_w(self[e42] * other[e4315]),
            // e5
            (self[e45] * other[e3215]) + (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
        )
    }
}
impl AntiWedge<DualNum> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345]) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(other[e12345]) * self.group2(),
        )
    }
}
impl AntiWedge<Flector> for AntiCircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       11       16        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e3215]) + (self[e31] * other[e4125]),
                (self[e42] * other[e3215]) + (self[e12] * other[e4235]),
                (self[e43] * other[e3215]) + (self[e23] * other[e4315]),
                -(self[e42] * other[e4315]) - (self[e43] * other[e4125]),
            ]) - (other.group1().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e5
            (self[e45] * other[e3215]) + (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
        )
    }
}
impl AntiWedge<Line> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(self[e41] * other[e235]) - (self[e42] * other[e315]) - (self[e43] * other[e125]) - (self[e23] * other[e415]) - (self[e31] * other[e425]) - (self[e12] * other[e435]),
        )
    }
}
impl AntiWedge<Motor> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       10        0
    //  no simd        6       17        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345]) * self.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(other[e12345]) * self.group2().xyz()).with_w(
                (self[scalar] * other[e12345])
                    - (self[e41] * other[e235])
                    - (self[e42] * other[e315])
                    - (self[e43] * other[e125])
                    - (self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435]),
            ),
        )
    }
}
impl AntiWedge<MultiVector> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       25       41        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[scalar] * other[e12345])
                    - (self[e41] * other[e235])
                    - (self[e42] * other[e315])
                    - (self[e43] * other[e125])
                    - (self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435])
                    - (self[e45] * other[e321])
                    - (self[e15] * other[e423])
                    - (self[e25] * other[e431])
                    - (self[e35] * other[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e3215]) + (self[e31] * other[e4125]),
                (self[e42] * other[e3215]) + (self[e12] * other[e4235]),
                (self[e43] * other[e3215]) + (self[e23] * other[e4315]),
                -(self[e43] * other[e4125]) - (self[e45] * other[e1234]),
            ]) - (other.group9().yzxx() * self.group1().zxy().with_w(self[e41]))
                - (Simd32x3::from(other[e1234]) * self.group2().xyz()).with_w(self[e42] * other[e4315]),
            // e5
            (self[e45] * other[e3215]) + (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
            // e15, e25, e35, e45
            Simd32x4::from(other[e12345]) * self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            Simd32x3::from(other[e12345]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[e12345]) * self.group1().xyz(),
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
impl AntiWedge<Plane> for AntiCircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       11       16        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e3215]) + (self[e31] * other[e4125]),
                (self[e42] * other[e3215]) + (self[e12] * other[e4235]),
                (self[e43] * other[e3215]) + (self[e23] * other[e4315]),
                -(self[e42] * other[e4315]) - (self[e43] * other[e4125]),
            ]) - (other.group0().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e5
            (self[e45] * other[e3215]) + (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
        )
    }
}
impl AntiWedge<Sphere> for AntiCircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       20        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e3215]) + (self[e31] * other[e4125]),
                (self[e42] * other[e3215]) + (self[e12] * other[e4235]),
                (self[e43] * other[e3215]) + (self[e23] * other[e4315]),
                -(self[e43] * other[e4125]) - (self[e45] * other[e1234]),
            ]) - (other.group0().yzxx() * self.group1().zxy().with_w(self[e41]))
                - (Simd32x3::from(other[e1234]) * self.group2().xyz()).with_w(self[e42] * other[e4315]),
            // e5
            (self[e45] * other[e3215]) + (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
        )
    }
}
impl AntiWedge<VersorEven> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       11        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       14        0
    //  no simd       10       21        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345]) * self.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(other[e12345]) * self.group2().xyz()).with_w(
                (self[scalar] * other[e12345])
                    - (self[e41] * other[e235])
                    - (self[e42] * other[e315])
                    - (self[e43] * other[e125])
                    - (self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435])
                    - (self[e45] * other[e321])
                    - (self[e15] * other[e423])
                    - (self[e25] * other[e431])
                    - (self[e35] * other[e412]),
            ),
        )
    }
}
impl AntiWedge<VersorOdd> for AntiCircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       20        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e3215]) + (self[e31] * other[e4125]),
                (self[e42] * other[e3215]) + (self[e12] * other[e4235]),
                (self[e43] * other[e3215]) + (self[e23] * other[e4315]),
                -(self[e43] * other[e4125]) - (self[e45] * other[e1234]),
            ]) - (other.group3().yzxx() * self.group1().zxy().with_w(self[e41]))
                - (self.group2().xyz() * other.group2().www()).with_w(self[e42] * other[e4315]),
            // e5
            (self[e45] * other[e3215]) + (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
        )
    }
}
impl std::ops::Div<AntiWedgeInfix> for AntiDipoleInversion {
    type Output = AntiWedgeInfixPartial<AntiDipoleInversion>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiCircleRotor> for AntiDipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(other[e41] * self[e235])
                - (other[e42] * self[e315])
                - (other[e43] * self[e125])
                - (other[e23] * self[e415])
                - (other[e31] * self[e425])
                - (other[e12] * self[e435])
                - (other[e45] * self[e321])
                - (other[e15] * self[e423])
                - (other[e25] * self[e431])
                - (other[e35] * self[e412]),
        )
    }
}
impl AntiWedge<AntiDipoleInversion> for AntiDipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        2        0
    //    simd4        2        0        0
    // Totals...
    // yes simd       19       26        0
    //  no simd       25       30        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e412] * self[e315]) + (other[e415] * self[e321]) + (other[e321] * self[e415]) + (other[e315] * self[e412]),
                (other[e423] * self[e125]) + (other[e425] * self[e321]) + (other[e321] * self[e425]) + (other[e125] * self[e423]),
                (other[e431] * self[e235]) + (other[e435] * self[e321]) + (other[e321] * self[e435]) + (other[e235] * self[e431]),
                -(other[e412] * self[e435]) - (other[e415] * self[e423]) - (other[e425] * self[e431]) - (other[e435] * self[e412]),
            ]) - (other.group0().yzx() * self.group2().zxy()).with_w(other[e423] * self[e415])
                - (self.group0().yzx() * other.group2().zxy()).with_w(other[e431] * self[e425]),
            // e5
            -(other[e415] * self[e235])
                - (other[e425] * self[e315])
                - (other[e435] * self[e125])
                - (other[e235] * self[e415])
                - (other[e315] * self[e425])
                - (other[e125] * self[e435]),
        )
    }
}
impl AntiWedge<AntiDualNum> for AntiDipoleInversion {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other[e3215]) * self.group0().with_w(self[e4]),
            // e15, e25, e35, e3215
            (self.group1().xyz() * other.group0().xx().with_z(other[e3215])).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiFlatPoint> for AntiDipoleInversion {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e412] * other[e315]) + (self[e415] * other[e321]),
                (self[e423] * other[e125]) + (self[e425] * other[e321]),
                (self[e431] * other[e235]) + (self[e435] * other[e321]),
                -(self[e425] * other[e315]) - (self[e435] * other[e125]),
            ]) - (other.group0().zxyx() * self.group0().yzx().with_w(self[e415])),
        )
    }
}
impl AntiWedge<AntiFlector> for AntiDipoleInversion {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e412] * other[e315]) + (self[e415] * other[e321]),
                (self[e423] * other[e125]) + (self[e425] * other[e321]),
                (self[e431] * other[e235]) + (self[e435] * other[e321]),
                -(self[e425] * other[e315]) - (self[e435] * other[e125]),
            ]) - (other.group0().zxyx() * self.group0().yzx().with_w(self[e415])),
        )
    }
}
impl AntiWedge<AntiLine> for AntiDipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(self[e423] * other[e15]) - (self[e431] * other[e25]) - (self[e412] * other[e35]) - (self[e415] * other[e23]) - (self[e425] * other[e31]) - (self[e435] * other[e12]),
        )
    }
}
impl AntiWedge<AntiMotor> for AntiDipoleInversion {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        6        9        0
    //  no simd        6       13        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(other[e3215]) * self.group0()).with_w(
                (self[e4] * other[e3215])
                    - (self[e423] * other[e15])
                    - (self[e431] * other[e25])
                    - (self[e412] * other[e35])
                    - (self[e415] * other[e23])
                    - (self[e425] * other[e31])
                    - (self[e435] * other[e12]),
            ),
            // e15, e25, e35, e3215
            (self.group1().xyz() * other.group1().www()).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiScalar> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(other[e12345]) * self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(other[e12345]) * self.group3(),
        )
    }
}
impl AntiWedge<Circle> for AntiDipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        2        0
    //    simd4        2        0        0
    // Totals...
    // yes simd       19       26        0
    //  no simd       25       30        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e412] * other[e315]) + (self[e415] * other[e321]) + (self[e321] * other[e415]) + (self[e315] * other[e412]),
                (self[e423] * other[e125]) + (self[e425] * other[e321]) + (self[e321] * other[e425]) + (self[e125] * other[e423]),
                (self[e431] * other[e235]) + (self[e435] * other[e321]) + (self[e321] * other[e435]) + (self[e235] * other[e431]),
                -(self[e412] * other[e435]) - (self[e415] * other[e423]) - (self[e425] * other[e431]) - (self[e435] * other[e412]),
            ]) - (self.group0().yzx() * other.group2().zxy()).with_w(self[e423] * other[e415])
                - (other.group0().yzx() * self.group2().zxy()).with_w(self[e431] * other[e425]),
            // e5
            -(self[e415] * other[e235])
                - (self[e425] * other[e315])
                - (self[e435] * other[e125])
                - (self[e235] * other[e415])
                - (self[e315] * other[e425])
                - (self[e125] * other[e435]),
        )
    }
}
impl AntiWedge<CircleRotor> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       24        0
    //    simd3        0        3        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       21       30        0
    //  no simd       30       45        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(other[e12345]) * self.group2().xyz()).with_w(
                (self[e4] * other[e12345])
                    - (self[e423] * other[e415])
                    - (self[e431] * other[e425])
                    - (self[e412] * other[e435])
                    - (self[e415] * other[e423])
                    - (self[e425] * other[e431])
                    - (self[e435] * other[e412]),
            ),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e415] * other[e321]) + (self[e321] * other[e415]) + (self[e315] * other[e412]) + (self[e1] * other[e12345]),
                (self[e425] * other[e321]) + (self[e321] * other[e425]) + (self[e125] * other[e423]) + (self[e2] * other[e12345]),
                (self[e435] * other[e321]) + (self[e321] * other[e435]) + (self[e235] * other[e431]) + (self[e3] * other[e12345]),
                -(self[e435] * other[e125]) - (self[e235] * other[e415]) - (self[e315] * other[e425]) - (self[e125] * other[e435]),
            ]) + (other.group2().yzxw() * self.group0().zxy().with_w(self[e5]))
                - (other.group2().zxyx() * self.group0().yzx().with_w(self[e415]))
                - (other.group0().yzx() * self.group2().zxy()).with_w(self[e425] * other[e315]),
        )
    }
}
impl AntiWedge<Dipole> for AntiDipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(self[e423] * other[e15])
                - (self[e431] * other[e25])
                - (self[e412] * other[e35])
                - (self[e415] * other[e23])
                - (self[e425] * other[e31])
                - (self[e435] * other[e12])
                - (self[e321] * other[e45])
                - (self[e235] * other[e41])
                - (self[e315] * other[e42])
                - (self[e125] * other[e43]),
        )
    }
}
impl AntiWedge<DipoleInversion> for AntiDipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        2        4        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       21       28        0
    //  no simd       37       45        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (self.group0().zxy() * other.group3().yzx()) - (self.group0().yzx() * other.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e423] * other[e3215]) + (self[e235] * other[e1234]),
                (self[e431] * other[e3215]) + (self[e315] * other[e1234]),
                (self[e412] * other[e3215]) + (self[e125] * other[e1234]),
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) - (self.group1().wwwx() * other.group3().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(other[e3215]) * self.group1().xyz().with_w(self[e4]))
                + (other.group3().zxyx() * self.group2().yzx().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w(
                    (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e5] * other[e1234])
                        - (self[e431] * other[e25])
                        - (self[e412] * other[e35])
                        - (self[e415] * other[e23])
                        - (self[e425] * other[e31])
                        - (self[e435] * other[e12])
                        - (self[e321] * other[e45])
                        - (self[e235] * other[e41])
                        - (self[e315] * other[e42])
                        - (self[e125] * other[e43]),
                )
                - (self.group2().zxy() * other.group3().yzx()).with_w(self[e423] * other[e15]),
        )
    }
}
impl AntiWedge<DualNum> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(other[e12345]) * self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(other[e12345]) * self.group3(),
        )
    }
}
impl AntiWedge<FlatPoint> for AntiDipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(self[e423] * other[e15]) - (self[e431] * other[e25]) - (self[e412] * other[e35]) - (self[e321] * other[e45]),
        )
    }
}
impl AntiWedge<Flector> for AntiDipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        3        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       10       17        0
    //  no simd       24       32        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (self.group0().zxy() * other.group1().yzx()) - (self.group0().yzx() * other.group1().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                self[e423] * other[e3215],
                self[e431] * other[e3215],
                self[e412] * other[e3215],
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) - (self.group1().wwwx() * other.group1().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(other[e3215]) * self.group1().xyz().with_w(self[e4]))
                + (other.group1().zxyx() * self.group2().yzx().with_w(self[e1]))
                + Simd32x3::from(0.0)
                    .with_w((self[e2] * other[e4315]) + (self[e3] * other[e4125]) - (self[e431] * other[e25]) - (self[e412] * other[e35]) - (self[e321] * other[e45]))
                - (self.group2().zxy() * other.group1().yzx()).with_w(self[e423] * other[e15]),
        )
    }
}
impl AntiWedge<Line> for AntiDipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       13       18        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e412] * other[e315]) + (self[e321] * other[e415]),
                (self[e423] * other[e125]) + (self[e321] * other[e425]),
                (self[e431] * other[e235]) + (self[e321] * other[e435]),
                -(self[e431] * other[e425]) - (self[e412] * other[e435]),
            ]) - (self.group0().yzx() * other.group1().zxy()).with_w(self[e423] * other[e415]),
            // e5
            -(self[e415] * other[e235])
                - (self[e425] * other[e315])
                - (self[e435] * other[e125])
                - (self[e235] * other[e415])
                - (self[e315] * other[e425])
                - (self[e125] * other[e435]),
        )
    }
}
impl AntiWedge<Motor> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       16        0
    //    simd3        0        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       12       21        0
    //  no simd       18       33        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(other[e12345]) * self.group2().xyz())
                .with_w((self[e4] * other[e12345]) - (self[e423] * other[e415]) - (self[e431] * other[e425]) - (self[e412] * other[e435])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e321] * other[e415]) + (self[e1] * other[e12345]),
                (self[e321] * other[e425]) + (self[e2] * other[e12345]),
                (self[e321] * other[e435]) + (self[e3] * other[e12345]),
                -(self[e425] * other[e315]) - (self[e435] * other[e125]) - (self[e235] * other[e415]) - (self[e315] * other[e425]) - (self[e125] * other[e435]),
            ]) + (self.group0().zxy() * other.group1().yzx()).with_w(self[e5] * other[e12345])
                - (other.group1().zxyx() * self.group0().yzx().with_w(self[e415])),
        )
    }
}
impl AntiWedge<MultiVector> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       48        0
    //    simd3        4       10        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       44       61        0
    //  no simd       64       90        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e4] * other[e3215]) + (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e5] * other[e1234])
                    - (self[e423] * other[e15])
                    - (self[e431] * other[e25])
                    - (self[e412] * other[e35])
                    - (self[e415] * other[e23])
                    - (self[e425] * other[e31])
                    - (self[e435] * other[e12])
                    - (self[e321] * other[e45])
                    - (self[e235] * other[e41])
                    - (self[e315] * other[e42])
                    - (self[e125] * other[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e412] * other[e315]) + (self[e415] * other[e321]) + (self[e321] * other[e415]) + (self[e315] * other[e412]),
                (self[e423] * other[e125]) + (self[e425] * other[e321]) + (self[e321] * other[e425]) + (self[e125] * other[e423]),
                (self[e431] * other[e235]) + (self[e435] * other[e321]) + (self[e321] * other[e435]) + (self[e235] * other[e431]),
                -(self[e412] * other[e435]) - (self[e415] * other[e423]) - (self[e425] * other[e431]) - (self[e435] * other[e412]),
            ]) + (Simd32x4::from(other[e12345]) * self.group3().xyz().with_w(self[e4]))
                - (self.group0().yzx() * other.group8().zxy()).with_w(self[e423] * other[e415])
                - (other.group7().yzx() * self.group2().zxy()).with_w(self[e431] * other[e425]),
            // e5
            (self[e5] * other[e12345])
                - (self[e415] * other[e235])
                - (self[e425] * other[e315])
                - (self[e435] * other[e125])
                - (self[e235] * other[e415])
                - (self[e315] * other[e425])
                - (self[e125] * other[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e415] * other[e3215]) + (self[e315] * other[e4125]),
                (self[e425] * other[e3215]) + (self[e125] * other[e4235]),
                (self[e435] * other[e3215]) + (self[e235] * other[e4315]),
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) - (other.group9().yzxx() * self.group2().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (self.group0().zxy() * other.group9().yzx()) - (self.group0().yzx() * other.group9().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[e3215]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group2().xyz()) - (Simd32x3::from(self[e321]) * other.group9().xyz()),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group1(),
            // e423, e431, e412
            Simd32x3::from(other[e12345]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other[e12345]) * self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl AntiWedge<Plane> for AntiDipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       13        0
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        6       18        0
    //  no simd       17       31        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (self.group0().zxy() * other.group0().yzx()) - (self.group0().yzx() * other.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                self[e423] * other[e3215],
                self[e431] * other[e3215],
                self[e412] * other[e3215],
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) - (self.group1().wwwx() * other.group0().xyzx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                self[e125] * other[e4315] * -1.0,
                self[e235] * other[e4125] * -1.0,
                self[e315] * other[e4235] * -1.0,
                (self[e2] * other[e4315]) + (self[e3] * other[e4125]),
            ]) + (Simd32x4::from(other[e3215]) * self.group1().xyz().with_w(self[e4]))
                + (other.group0().zxyx() * self.group2().yzx().with_w(self[e1])),
        )
    }
}
impl AntiWedge<Sphere> for AntiDipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        2        3        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       24       38        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (self.group0().zxy() * other.group0().yzx()) - (self.group0().yzx() * other.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e423] * other[e3215]) + (self[e235] * other[e1234]),
                (self[e431] * other[e3215]) + (self[e315] * other[e1234]),
                (self[e412] * other[e3215]) + (self[e125] * other[e1234]),
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) - (self.group1().wwwx() * other.group0().xyzx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                self[e125] * other[e4315] * -1.0,
                self[e235] * other[e4125] * -1.0,
                self[e315] * other[e4235] * -1.0,
                (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e5] * other[e1234]),
            ]) + (Simd32x4::from(other[e3215]) * self.group1().xyz().with_w(self[e4]))
                + (other.group0().zxyx() * self.group2().yzx().with_w(self[e1])),
        )
    }
}
impl AntiWedge<VersorEven> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       25        0
    //    simd3        0        4        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       21       31        0
    //  no simd       30       45        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(other[e12345]) * self.group2().xyz()).with_w(
                (self[e4] * other[e12345])
                    - (self[e423] * other[e415])
                    - (self[e431] * other[e425])
                    - (self[e412] * other[e435])
                    - (self[e415] * other[e423])
                    - (self[e425] * other[e431])
                    - (self[e435] * other[e412]),
            ),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e415] * other[e321]) + (self[e321] * other[e415]) + (self[e315] * other[e412]) + (self[e1] * other[e12345]),
                (self[e425] * other[e321]) + (self[e321] * other[e425]) + (self[e125] * other[e423]) + (self[e2] * other[e12345]),
                (self[e435] * other[e321]) + (self[e321] * other[e435]) + (self[e235] * other[e431]) + (self[e3] * other[e12345]),
                -(self[e435] * other[e125]) - (self[e235] * other[e415]) - (self[e315] * other[e425]) - (self[e125] * other[e435]),
            ]) + (self.group0().zxy() * other.group2().yzx()).with_w(self[e5] * other[e12345])
                - (other.group2().zxyx() * self.group0().yzx().with_w(self[e415]))
                - (self.group2().zxy() * other.group0().yzx()).with_w(self[e425] * other[e315]),
        )
    }
}
impl AntiWedge<VersorOdd> for AntiDipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        2        4        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       21       28        0
    //  no simd       37       45        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (self.group0().zxy() * other.group3().yzx()) - (self.group0().yzx() * other.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e423] * other[e3215]) + (self[e235] * other[e1234]),
                (self[e431] * other[e3215]) + (self[e315] * other[e1234]),
                (self[e412] * other[e3215]) + (self[e125] * other[e1234]),
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) - (self.group1().wwwx() * other.group3().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(other[e3215]) * self.group1().xyz().with_w(self[e4]))
                + (other.group3().zxyx() * self.group2().yzx().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w(
                    (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e5] * other[e1234])
                        - (self[e431] * other[e25])
                        - (self[e412] * other[e35])
                        - (self[e415] * other[e23])
                        - (self[e425] * other[e31])
                        - (self[e435] * other[e12])
                        - (self[e321] * other[e45])
                        - (self[e235] * other[e41])
                        - (self[e315] * other[e42])
                        - (self[e125] * other[e43]),
                )
                - (self.group2().zxy() * other.group3().yzx()).with_w(self[e423] * other[e15]),
        )
    }
}
impl std::ops::Div<AntiWedgeInfix> for AntiDualNum {
    type Output = AntiWedgeInfixPartial<AntiDualNum>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiCircleRotor> for AntiDualNum {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e3215]) * other.group0().with_w(other[e45]) * Simd32x4::from(-1.0))
    }
}
impl AntiWedge<AntiDipoleInversion> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(self[e3215]) * other.group0().with_w(other[e4]),
            // e15, e25, e35, e3215
            (other.group1().xyz() * self.group0().xx().with_z(self[e3215])).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiScalar> for AntiDualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<Circle> for AntiDualNum {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(self[e3215]) * other.group0(),
            // e15, e25, e35
            Simd32x3::from(self[e3215]) * other.group1().xyz(),
        )
    }
}
impl AntiWedge<CircleRotor> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0().xx().with_zw(self[e3215], self[scalar]) * other.group0().with_w(other[e12345]),
            // e15, e25, e35, e3215
            Simd32x4::from(self[e3215]) * other.group1().xyz().with_w(other[e12345]),
        )
    }
}
impl AntiWedge<Dipole> for AntiDualNum {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e3215]) * other.group0().with_w(other[e45]) * Simd32x4::from(-1.0))
    }
}
impl AntiWedge<DipoleInversion> for AntiDualNum {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215]) * other.group3().xyz().with_w(other[e1234]) * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            Simd32x4::from(self[e3215]) * other.group0().with_w(other[e45]) * Simd32x4::from(-1.0),
        )
    }
}
impl AntiWedge<DualNum> for AntiDualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<FlatPoint> for AntiDualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e3215] * other[e45], 1.0]) * Simd32x2::from([-1.0, 0.0]))
    }
}
impl AntiWedge<Flector> for AntiDualNum {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       11        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (other.group1().xyz() * self.group0().xx().with_z(self[e3215]) * Simd32x3::from(-1.0)).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e3215] * other[e45]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        )
    }
}
impl AntiWedge<Line> for AntiDualNum {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ (other.group0() * self.group0().xx().with_z(self[e3215])).with_w(0.0))
    }
}
impl AntiWedge<Motor> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(self[scalar] * other[e12345]),
            // e15, e25, e35, e3215
            Simd32x4::from(self[e3215]) * other.group0(),
        )
    }
}
impl AntiWedge<MultiVector> for AntiDualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd3        0        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1       13        0
    //  no simd        1       28        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self[e3215] * other[e4]) + (self[scalar] * other[e12345]), 0.0]),
            // e1, e2, e3, e4
            (other.group4() * self.group0().xx().with_z(self[e3215]) * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            self[e3215] * other[e45] * -1.0,
            // e15, e25, e35, e45
            (other.group6().xyz() * self.group0().xx().with_z(self[e3215])).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e3215]) * other.group7(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e3215] * other[e1234]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[e3215]) * other.group9().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215] * other[e12345]),
            // e1234
            0.0,
        )
    }
}
impl AntiWedge<Plane> for AntiDualNum {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            (other.group0().xyz() * self.group0().xx().with_z(self[e3215]) * Simd32x3::from(-1.0)).with_w(0.0),
        )
    }
}
impl AntiWedge<RoundPoint> for AntiDualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e3215] * other[e4])
    }
}
impl AntiWedge<Sphere> for AntiDualNum {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215]) * other.group0().xyz().with_w(other[e1234]) * Simd32x4::from(-1.0),
        )
    }
}
impl AntiWedge<VersorEven> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1       10        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0().xx().with_zw(self[e3215], (self[e3215] * other[e4]) + (self[scalar] * other[e12345])) * other.group0().xyz().with_w(1.0),
            // e15, e25, e35, e3215
            Simd32x4::from(self[e3215]) * other.group1().xyz().with_w(other[e12345]),
        )
    }
}
impl AntiWedge<VersorOdd> for AntiDualNum {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215]) * other.group3().xyz().with_w(other[e1234]) * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            Simd32x4::from(self[e3215]) * other.group0().xyz().with_w(other[e45]) * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiWedgeInfix> for AntiFlatPoint {
    type Output = AntiWedgeInfixPartial<AntiFlatPoint>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiCircleRotor> for AntiFlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(other[e41] * self[e235]) - (other[e42] * self[e315]) - (other[e43] * self[e125]) - (other[e45] * self[e321]),
        )
    }
}
impl AntiWedge<AntiDipoleInversion> for AntiFlatPoint {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e412] * self[e315]) + (other[e415] * self[e321]),
                (other[e423] * self[e125]) + (other[e425] * self[e321]),
                (other[e431] * self[e235]) + (other[e435] * self[e321]),
                -(other[e425] * self[e315]) - (other[e435] * self[e125]),
            ]) - (self.group0().zxyx() * other.group0().yzx().with_w(other[e415])),
        )
    }
}
impl AntiWedge<AntiScalar> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<Circle> for AntiFlatPoint {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       12        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e315] * other[e412]) + (self[e321] * other[e415]),
                (self[e125] * other[e423]) + (self[e321] * other[e425]),
                (self[e235] * other[e431]) + (self[e321] * other[e435]),
                -(self[e315] * other[e425]) - (self[e125] * other[e435]),
            ]) - (other.group0().yzx() * self.group0().zxy()).with_w(self[e235] * other[e415]),
        )
    }
}
impl AntiWedge<CircleRotor> for AntiFlatPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       16        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e12345]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e315] * other[e412]) + (self[e321] * other[e415]),
                (self[e125] * other[e423]) + (self[e321] * other[e425]),
                (self[e235] * other[e431]) + (self[e321] * other[e435]),
                -(self[e315] * other[e425]) - (self[e125] * other[e435]),
            ]) - (other.group0().yzx() * self.group0().zxy()).with_w(self[e235] * other[e415]),
        )
    }
}
impl AntiWedge<Dipole> for AntiFlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(self[e235] * other[e41]) - (self[e315] * other[e42]) - (self[e125] * other[e43]) - (self[e321] * other[e45]),
        )
    }
}
impl AntiWedge<DipoleInversion> for AntiFlatPoint {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        7        0
    //    simd3        1        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        9       16        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                self[e235] * other[e1234],
                self[e315] * other[e1234],
                self[e125] * other[e1234],
                -(self[e315] * other[e42]) - (self[e125] * other[e43]) - (self[e321] * other[e45]),
            ]) - (other.group3().xyz() * self.group0().www()).with_w(self[e235] * other[e41]),
            // e15, e25, e35, e3215
            ((self.group0().yzx() * other.group3().zxy()) - (self.group0().zxy() * other.group3().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<DualNum> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<FlatPoint> for AntiFlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * other[e45] * -1.0)
    }
}
impl AntiWedge<Flector> for AntiFlatPoint {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        3       14        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321]) * other.group1().xyz().with_w(other[e45]) * Simd32x4::from(-1.0),
            // e15, e25, e35, e3215
            ((self.group0().yzx() * other.group1().zxy()) - (self.group0().zxy() * other.group1().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<Line> for AntiFlatPoint {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x3::from(self[e321]) * other.group0()).with_w(-(self[e235] * other[e415]) - (self[e315] * other[e425]) - (self[e125] * other[e435])),
        )
    }
}
impl AntiWedge<Motor> for AntiFlatPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2       10        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e12345]) * self.group0(),
            // e1, e2, e3, e5
            (other.group0().xyz() * self.group0().www()).with_w(-(self[e235] * other[e415]) - (self[e315] * other[e425]) - (self[e125] * other[e435])),
        )
    }
}
impl AntiWedge<MultiVector> for AntiFlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        8        0
    //    simd3        2        8        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        9       16        0
    //  no simd       19       32        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([-(self[e235] * other[e41]) - (self[e315] * other[e42]) - (self[e125] * other[e43]) - (self[e321] * other[e45]), 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(self[e321]) * other.group6().xyz()).with_w(0.0) + (other.group7().zxy() * self.group0().yzx()).with_w(0.0)
                - (other.group7().yzx() * self.group0().zxy()).with_w(0.0),
            // e5
            -(self[e235] * other[e415]) - (self[e315] * other[e425]) - (self[e125] * other[e435]),
            // e15, e25, e35, e45
            ((self.group0().yzx() * other.group9().zxy()) - (self.group0().zxy() * other.group9().yzx())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from(other[e1234]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * other.group9().xyz()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321] * other[e12345]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(other[e12345]) * self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl AntiWedge<Plane> for AntiFlatPoint {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        4        0
    // no simd        3       12        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(self[e321]) * other.group0().xyz() * Simd32x3::from(-1.0),
            // e15, e25, e35
            (self.group0().yzx() * other.group0().zxy()) - (self.group0().zxy() * other.group0().yzx()),
        )
    }
}
impl AntiWedge<Sphere> for AntiFlatPoint {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        4        0
    // no simd        6       12        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            (Simd32x3::from(other[e1234]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * other.group0().xyz()),
            // e15, e25, e35
            (self.group0().yzx() * other.group0().zxy()) - (self.group0().zxy() * other.group0().yzx()),
        )
    }
}
impl AntiWedge<VersorEven> for AntiFlatPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e12345]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e315] * other[e412]) + (self[e321] * other[e415]),
                (self[e125] * other[e423]) + (self[e321] * other[e425]),
                (self[e235] * other[e431]) + (self[e321] * other[e435]),
                -(self[e315] * other[e425]) - (self[e125] * other[e435]),
            ]) - (self.group0().zxyx() * other.group0().yzx().with_w(other[e415])),
        )
    }
}
impl AntiWedge<VersorOdd> for AntiFlatPoint {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        9       16        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                self[e235] * other[e1234],
                self[e315] * other[e1234],
                self[e125] * other[e1234],
                -(self[e315] * other[e42]) - (self[e125] * other[e43]) - (self[e321] * other[e45]),
            ]) - (self.group0().wwwx() * other.group3().xyz().with_w(other[e41])),
            // e15, e25, e35, e3215
            ((self.group0().yzx() * other.group3().zxy()) - (self.group0().zxy() * other.group3().yzx())).with_w(0.0),
        )
    }
}
impl std::ops::Div<AntiWedgeInfix> for AntiFlector {
    type Output = AntiWedgeInfixPartial<AntiFlector>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiCircleRotor> for AntiFlector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(other[e41] * self[e235]) - (other[e42] * self[e315]) - (other[e43] * self[e125]) - (other[e45] * self[e321]),
        )
    }
}
impl AntiWedge<AntiDipoleInversion> for AntiFlector {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e412] * self[e315]) + (other[e415] * self[e321]),
                (other[e423] * self[e125]) + (other[e425] * self[e321]),
                (other[e431] * self[e235]) + (other[e435] * self[e321]),
                -(other[e425] * self[e315]) - (other[e435] * self[e125]),
            ]) - (self.group0().zxyx() * other.group0().yzx().with_w(other[e415])),
        )
    }
}
impl AntiWedge<AntiScalar> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e12345]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(other[e12345]) * self.group1(),
        )
    }
}
impl AntiWedge<Circle> for AntiFlector {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       12        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e315] * other[e412]) + (self[e321] * other[e415]),
                (self[e125] * other[e423]) + (self[e321] * other[e425]),
                (self[e235] * other[e431]) + (self[e321] * other[e435]),
                -(self[e315] * other[e425]) - (self[e125] * other[e435]),
            ]) - (other.group0().yzx() * self.group0().zxy()).with_w(self[e235] * other[e415]),
        )
    }
}
impl AntiWedge<CircleRotor> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd3        0        2        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        6       13        0
    //  no simd       12       20        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e12345]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e321] * other[e415]) + (self[e1] * other[e12345]),
                (self[e321] * other[e425]) + (self[e2] * other[e12345]),
                (self[e321] * other[e435]) + (self[e3] * other[e12345]),
                -(self[e315] * other[e425]) - (self[e125] * other[e435]),
            ]) + (other.group0().zxy() * self.group0().yzx()).with_w(self[e5] * other[e12345])
                - (other.group0().yzx() * self.group0().zxy()).with_w(self[e235] * other[e415]),
        )
    }
}
impl AntiWedge<Dipole> for AntiFlector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(self[e235] * other[e41]) - (self[e315] * other[e42]) - (self[e125] * other[e43]) - (self[e321] * other[e45]),
        )
    }
}
impl AntiWedge<DipoleInversion> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        8        0
    //    simd3        1        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        8       12        0
    //  no simd       16       20        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(
                (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e5] * other[e1234])
                    - (self[e315] * other[e42])
                    - (self[e125] * other[e43])
                    - (self[e321] * other[e45]),
            ) + (self.group0().xyz() * other.group2().www()).with_w(self[e1] * other[e4235])
                - (other.group3().xyz() * self.group0().www()).with_w(self[e235] * other[e41]),
            // e15, e25, e35, e3215
            ((self.group0().yzx() * other.group3().zxy()) - (self.group0().zxy() * other.group3().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<DualNum> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e12345]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(other[e12345]) * self.group1(),
        )
    }
}
impl AntiWedge<FlatPoint> for AntiFlector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * other[e45] * -1.0)
    }
}
impl AntiWedge<Flector> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        1        4        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        6       16        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (other.group1().xyz() * self.group0().www() * Simd32x3::from(-1.0))
                .with_w((self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]) - (self[e321] * other[e45])),
            // e15, e25, e35, e3215
            ((self.group0().yzx() * other.group1().zxy()) - (self.group0().zxy() * other.group1().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<Line> for AntiFlector {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x3::from(self[e321]) * other.group0()).with_w(-(self[e235] * other[e415]) - (self[e315] * other[e425]) - (self[e125] * other[e435])),
        )
    }
}
impl AntiWedge<Motor> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        6       14        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e12345]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([
                self[e1] * other[e12345],
                self[e2] * other[e12345],
                self[e3] * other[e12345],
                -(self[e235] * other[e415]) - (self[e315] * other[e425]) - (self[e125] * other[e435]),
            ]) + (other.group0() * self.group0().www().with_w(self[e5])),
        )
    }
}
impl AntiWedge<MultiVector> for AntiFlector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       13        0
    //    simd3        2        9        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       15       22        0
    //  no simd       28       40        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e5] * other[e1234])
                    - (self[e235] * other[e41])
                    - (self[e315] * other[e42])
                    - (self[e125] * other[e43])
                    - (self[e321] * other[e45]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(self[e321]) * other.group6().xyz()).with_w(0.0)
                + (Simd32x3::from(other[e12345]) * self.group1().xyz()).with_w(0.0)
                + (other.group7().zxy() * self.group0().yzx()).with_w(0.0)
                - (other.group7().yzx() * self.group0().zxy()).with_w(0.0),
            // e5
            (self[e5] * other[e12345]) - (self[e235] * other[e415]) - (self[e315] * other[e425]) - (self[e125] * other[e435]),
            // e15, e25, e35, e45
            ((self.group0().yzx() * other.group9().zxy()) - (self.group0().zxy() * other.group9().yzx())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from(other[e1234]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * other.group9().xyz()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321] * other[e12345]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(other[e12345]) * self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl AntiWedge<Plane> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        1        4        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        5       15        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (other.group0().xyz() * self.group0().www() * Simd32x3::from(-1.0)).with_w((self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125])),
            // e15, e25, e35, e3215
            ((self.group0().yzx() * other.group0().zxy()) - (self.group0().zxy() * other.group0().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<Sphere> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       10        0
    //    simd3        1        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4       13        0
    //  no simd        9       19        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                self[e321] * other[e4235] * -1.0,
                self[e321] * other[e4315] * -1.0,
                self[e321] * other[e4125] * -1.0,
                (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e5] * other[e1234]),
            ]) + (Simd32x3::from(other[e1234]) * self.group0().xyz()).with_w(self[e1] * other[e4235]),
            // e15, e25, e35, e3215
            ((self.group0().yzx() * other.group0().zxy()) - (self.group0().zxy() * other.group0().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<VersorEven> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       20        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e12345]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e321] * other[e415]) + (self[e1] * other[e12345]),
                (self[e321] * other[e425]) + (self[e2] * other[e12345]),
                (self[e321] * other[e435]) + (self[e3] * other[e12345]),
                -(self[e315] * other[e425]) - (self[e125] * other[e435]),
            ]) + (other.group0().zxyw() * self.group0().yzx().with_w(self[e5]))
                - (self.group0().zxyx() * other.group0().yzx().with_w(other[e415])),
        )
    }
}
impl AntiWedge<VersorOdd> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        3        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        8       11        0
    //  no simd       16       20        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(
                (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e5] * other[e1234])
                    - (self[e315] * other[e42])
                    - (self[e125] * other[e43])
                    - (self[e321] * other[e45]),
            ) + (self.group0().xyz() * other.group2().www()).with_w(self[e1] * other[e4235])
                - (self.group0().wwwx() * other.group3().xyz().with_w(other[e41])),
            // e15, e25, e35, e3215
            ((self.group0().yzx() * other.group3().zxy()) - (self.group0().zxy() * other.group3().yzx())).with_w(0.0),
        )
    }
}
impl std::ops::Div<AntiWedgeInfix> for AntiLine {
    type Output = AntiWedgeInfixPartial<AntiLine>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiDipoleInversion> for AntiLine {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(other[e423] * self[e15]) - (other[e431] * self[e25]) - (other[e412] * self[e35]) - (other[e415] * self[e23]) - (other[e425] * self[e31]) - (other[e435] * self[e12]),
        )
    }
}
impl AntiWedge<AntiScalar> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(other[e12345]) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(other[e12345]) * self.group1(),
        )
    }
}
impl AntiWedge<Circle> for AntiLine {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(self[e23] * other[e415]) - (self[e31] * other[e425]) - (self[e12] * other[e435]) - (self[e15] * other[e423]) - (self[e25] * other[e431]) - (self[e35] * other[e412]),
        )
    }
}
impl AntiWedge<CircleRotor> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(other[e12345]) * self.group0()).with_w(
                -(self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435])
                    - (self[e15] * other[e423])
                    - (self[e25] * other[e431])
                    - (self[e35] * other[e412]),
            ),
            // e15, e25, e35, e3215
            (self.group1() * other.group2().www()).with_w(0.0),
        )
    }
}
impl AntiWedge<DipoleInversion> for AntiLine {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                -(self[e12] * other[e4315]) - (self[e15] * other[e1234]),
                -(self[e23] * other[e4125]) - (self[e25] * other[e1234]),
                -(self[e31] * other[e4235]) - (self[e35] * other[e1234]),
                (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
            ]) + (other.group3().zxyx() * self.group0().yzx().with_w(self[e15])),
        )
    }
}
impl AntiWedge<DualNum> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(other[e12345]) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(other[e12345]) * self.group1(),
        )
    }
}
impl AntiWedge<Flector> for AntiLine {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        9        0
    //  no simd        5       12        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                self[e12] * other[e4315] * -1.0,
                self[e23] * other[e4125] * -1.0,
                self[e31] * other[e4235] * -1.0,
                (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
            ]) + (other.group1().zxyx() * self.group0().yzx().with_w(self[e15])),
        )
    }
}
impl AntiWedge<Line> for AntiLine {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -(self[e23] * other[e415]) - (self[e31] * other[e425]) - (self[e12] * other[e435]))
    }
}
impl AntiWedge<Motor> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(other[e12345]) * self.group0()).with_w(-(self[e23] * other[e415]) - (self[e31] * other[e425]) - (self[e12] * other[e435])),
            // e15, e25, e35, e3215
            (self.group1() * other.group0().www()).with_w(0.0),
        )
    }
}
impl AntiWedge<MultiVector> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        9        0
    //    simd3        0        5        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       24        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435])
                    - (self[e15] * other[e423])
                    - (self[e25] * other[e431])
                    - (self[e35] * other[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (self.group0().yzx() * other.group9().zxy()).with_w(0.0)
                - (Simd32x3::from(other[e1234]) * self.group1()).with_w(0.0)
                - (self.group0().zxy() * other.group9().yzx()).with_w(0.0),
            // e5
            (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
            // e15, e25, e35, e45
            (self.group1() * other.group0().yy().with_z(other[e12345])).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other[e12345]) * self.group0(),
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
impl AntiWedge<Plane> for AntiLine {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        9        0
    //  no simd        5       12        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                self[e12] * other[e4315] * -1.0,
                self[e23] * other[e4125] * -1.0,
                self[e31] * other[e4235] * -1.0,
                (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
            ]) + (other.group0().zxyx() * self.group0().yzx().with_w(self[e15])),
        )
    }
}
impl AntiWedge<Sphere> for AntiLine {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                -(self[e12] * other[e4315]) - (self[e15] * other[e1234]),
                -(self[e23] * other[e4125]) - (self[e25] * other[e1234]),
                -(self[e31] * other[e4235]) - (self[e35] * other[e1234]),
                (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
            ]) + (other.group0().zxyx() * self.group0().yzx().with_w(self[e15])),
        )
    }
}
impl AntiWedge<VersorEven> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(other[e12345]) * self.group0()).with_w(
                -(self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435])
                    - (self[e15] * other[e423])
                    - (self[e25] * other[e431])
                    - (self[e35] * other[e412]),
            ),
            // e15, e25, e35, e3215
            (self.group1() * other.group0().www()).with_w(0.0),
        )
    }
}
impl AntiWedge<VersorOdd> for AntiLine {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                -(self[e12] * other[e4315]) - (self[e15] * other[e1234]),
                -(self[e23] * other[e4125]) - (self[e25] * other[e1234]),
                -(self[e31] * other[e4235]) - (self[e35] * other[e1234]),
                (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
            ]) + (other.group3().zxyx() * self.group0().yzx().with_w(self[e15])),
        )
    }
}
impl std::ops::Div<AntiWedgeInfix> for AntiMotor {
    type Output = AntiWedgeInfixPartial<AntiMotor>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiCircleRotor> for AntiMotor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e3215]) * other.group0().with_w(other[e45]) * Simd32x4::from(-1.0))
    }
}
impl AntiWedge<AntiDipoleInversion> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        6        9        0
    //  no simd        6       13        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e3215]) * other.group0()).with_w(
                (other[e4] * self[e3215])
                    - (other[e423] * self[e15])
                    - (other[e431] * self[e25])
                    - (other[e412] * self[e35])
                    - (other[e415] * self[e23])
                    - (other[e425] * self[e31])
                    - (other[e435] * self[e12]),
            ),
            // e15, e25, e35, e3215
            (other.group1().xyz() * self.group1().www()).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiScalar> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other[e12345]) * self.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(other[e12345]) * self.group1(),
        )
    }
}
impl AntiWedge<Circle> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e3215]) * other.group0()).with_w(
                -(self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435])
                    - (self[e15] * other[e423])
                    - (self[e25] * other[e431])
                    - (self[e35] * other[e412]),
            ),
            // e15, e25, e35, e3215
            (other.group1().xyz() * self.group1().www()).with_w(0.0),
        )
    }
}
impl AntiWedge<CircleRotor> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7       14        0
    //  no simd       12       20        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                self[e23] * other[e12345],
                self[e31] * other[e12345],
                self[e12] * other[e12345],
                -(self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435])
                    - (self[e15] * other[e423])
                    - (self[e25] * other[e431])
                    - (self[e35] * other[e412]),
            ]) + (other.group0() * self.group1().www()).with_w(self[scalar] * other[e12345]),
            // e15, e25, e35, e3215
            ((Simd32x3::from(self[e3215]) * other.group1().xyz()) + (Simd32x3::from(other[e12345]) * self.group1().xyz())).with_w(self[e3215] * other[e12345]),
        )
    }
}
impl AntiWedge<Dipole> for AntiMotor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        8        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (other.group0() * self.group1().www() * Simd32x3::from(-1.0)).with_w(self[e3215] * other[e45] * -1.0),
        )
    }
}
impl AntiWedge<DipoleInversion> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       13        0
    //  no simd       12       24        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215]) * other.group3().xyz().with_w(other[e1234]) * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            Simd32x4::from([
                -(self[e12] * other[e4315]) - (self[e15] * other[e1234]),
                -(self[e23] * other[e4125]) - (self[e25] * other[e1234]),
                -(self[e31] * other[e4235]) - (self[e35] * other[e1234]),
                (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
            ]) + (other.group3().zxyx() * self.group0().yzx().with_w(self[e15]))
                - (other.group0() * self.group1().www()).with_w(self[e3215] * other[e45]),
        )
    }
}
impl AntiWedge<DualNum> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other[e12345]) * self.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(other[e12345]) * self.group1(),
        )
    }
}
impl AntiWedge<FlatPoint> for AntiMotor {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e3215] * other[e45], 1.0]) * Simd32x2::from([-1.0, 0.0]))
    }
}
impl AntiWedge<Flector> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        0        3        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        9       16        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (other.group1().xyz() * self.group1().www() * Simd32x3::from(-1.0)).with_w(0.0),
            // e1, e2, e3, e5
            (other.group1().zxyx() * self.group0().yzx().with_w(self[e15])) + Simd32x3::from(0.0).with_w((self[e25] * other[e4315]) + (self[e35] * other[e4125]))
                - (self.group0().zxy() * other.group1().yzx()).with_w(self[e3215] * other[e45]),
        )
    }
}
impl AntiWedge<Line> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(self[e23] * other[e415]) - (self[e31] * other[e425]) - (self[e12] * other[e435])),
            // e15, e25, e35, e3215
            (other.group0() * self.group1().www()).with_w(0.0),
        )
    }
}
impl AntiWedge<Motor> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        1        3        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        6       14        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(other[e12345]) * self.group0().xyz())
                .with_w((self[scalar] * other[e12345]) - (self[e23] * other[e415]) - (self[e31] * other[e425]) - (self[e12] * other[e435])),
            // e15, e25, e35, e3215
            ((Simd32x3::from(self[e3215]) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * self.group1().xyz())).with_w(self[e3215] * other[e12345]),
        )
    }
}
impl AntiWedge<MultiVector> for AntiMotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       14        0
    //    simd3        2       10        0
    //    simd4        3        1        0
    // Totals...
    // yes simd       15       25        0
    //  no simd       28       48        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[scalar] * other[e12345]) + (self[e3215] * other[e4])
                    - (self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435])
                    - (self[e15] * other[e423])
                    - (self[e25] * other[e431])
                    - (self[e35] * other[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (self.group0().yzx() * other.group9().zxy()).with_w(0.0)
                - (Simd32x3::from(self[e3215]) * other.group4()).with_w(0.0)
                - (Simd32x3::from(other[e1234]) * self.group1().xyz()).with_w(0.0)
                - (self.group0().zxy() * other.group9().yzx()).with_w(0.0),
            // e5
            (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]) - (self[e3215] * other[e45]),
            // e15, e25, e35, e45
            ((Simd32x3::from(self[e3215]) * other.group6().xyz()) + (Simd32x3::from(other[e12345]) * self.group1().xyz())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from(self[e3215]) * other.group7()) + (Simd32x3::from(other[e12345]) * self.group0().xyz()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e3215] * other[e1234]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[e3215]) * other.group9().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215] * other[e12345]),
            // e1234
            0.0,
        )
    }
}
impl AntiWedge<Plane> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2       11        0
    //  no simd        5       18        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (other.group0().xyz() * self.group1().www() * Simd32x3::from(-1.0)).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([
                self[e12] * other[e4315] * -1.0,
                self[e23] * other[e4125] * -1.0,
                self[e31] * other[e4235] * -1.0,
                (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
            ]) + (other.group0().zxyx() * self.group0().yzx().with_w(self[e15])),
        )
    }
}
impl AntiWedge<RoundPoint> for AntiMotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e3215] * other[e4])
    }
}
impl AntiWedge<Sphere> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       20        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215]) * other.group0().xyz().with_w(other[e1234]) * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            Simd32x4::from([
                -(self[e12] * other[e4315]) - (self[e15] * other[e1234]),
                -(self[e23] * other[e4125]) - (self[e25] * other[e1234]),
                -(self[e31] * other[e4235]) - (self[e35] * other[e1234]),
                (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
            ]) + (other.group0().zxyx() * self.group0().yzx().with_w(self[e15])),
        )
    }
}
impl AntiWedge<VersorEven> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        8       11        0
    //  no simd       16       21        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x4::from(self[e3215]) * other.group0().xyz().with_w(other[e4]))
                + (Simd32x4::from(other[e12345]) * self.group0())
                + Simd32x3::from(0.0).with_w(
                    -(self[e23] * other[e415])
                        - (self[e31] * other[e425])
                        - (self[e12] * other[e435])
                        - (self[e15] * other[e423])
                        - (self[e25] * other[e431])
                        - (self[e35] * other[e412]),
                ),
            // e15, e25, e35, e3215
            ((Simd32x3::from(self[e3215]) * other.group1().xyz()) + (Simd32x3::from(other[e12345]) * self.group1().xyz())).with_w(self[e3215] * other[e12345]),
        )
    }
}
impl AntiWedge<VersorOdd> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       13        0
    //  no simd       12       24        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215]) * other.group3().xyz().with_w(other[e1234]) * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            Simd32x4::from([
                -(self[e15] * other[e1234]) - (self[e3215] * other[e41]),
                -(self[e25] * other[e1234]) - (self[e3215] * other[e42]),
                -(self[e35] * other[e1234]) - (self[e3215] * other[e43]),
                (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
            ]) + (other.group3().zxyx() * self.group0().yzx().with_w(self[e15]))
                - (self.group0().zxy() * other.group3().yzx()).with_w(self[e3215] * other[e45]),
        )
    }
}
impl std::ops::Div<AntiWedgeInfix> for AntiPlane {
    type Output = AntiWedgeInfixPartial<AntiPlane>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiScalar> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<CircleRotor> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<DipoleInversion> for AntiPlane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e5] * other[e1234]),
        )
    }
}
impl AntiWedge<DualNum> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<Flector> for AntiPlane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]))
    }
}
impl AntiWedge<Motor> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<MultiVector> for AntiPlane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3        8        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e5] * other[e1234]), 0.0]),
            // e1, e2, e3, e4
            (self.group0().xyz() * other.group0().yy().with_z(other[e12345])).with_w(0.0),
            // e5
            self[e5] * other[e12345],
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
impl AntiWedge<Plane> for AntiPlane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]))
    }
}
impl AntiWedge<Sphere> for AntiPlane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e5] * other[e1234]),
        )
    }
}
impl AntiWedge<VersorEven> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<VersorOdd> for AntiPlane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e5] * other[e1234]),
        )
    }
}
impl std::ops::Div<AntiWedgeInfix> for AntiScalar {
    type Output = AntiWedgeInfixPartial<AntiScalar>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiCircleRotor> for AntiScalar {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * other.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(self[e12345]) * other.group2(),
        )
    }
}
impl AntiWedge<AntiDipoleInversion> for AntiScalar {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(self[e12345]) * other.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(self[e12345]) * other.group3(),
        )
    }
}
impl AntiWedge<AntiDualNum> for AntiScalar {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<AntiFlatPoint> for AntiScalar {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<AntiFlector> for AntiScalar {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * other.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(self[e12345]) * other.group1(),
        )
    }
}
impl AntiWedge<AntiLine> for AntiScalar {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(self[e12345]) * other.group0(),
            // e15, e25, e35
            Simd32x3::from(self[e12345]) * other.group1(),
        )
    }
}
impl AntiWedge<AntiMotor> for AntiScalar {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(self[e12345]) * other.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(self[e12345]) * other.group1(),
        )
    }
}
impl AntiWedge<AntiPlane> for AntiScalar {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<AntiScalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[e12345] * self[e12345])
    }
}
impl AntiWedge<Circle> for AntiScalar {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1(),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * other.group2(),
        )
    }
}
impl AntiWedge<CircleRotor> for AntiScalar {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(self[e12345]) * other.group2(),
        )
    }
}
impl AntiWedge<Dipole> for AntiScalar {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * other.group1(),
            // e15, e25, e35
            Simd32x3::from(self[e12345]) * other.group2(),
        )
    }
}
impl AntiWedge<DipoleInversion> for AntiScalar {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(self[e12345]) * other.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group3(),
        )
    }
}
impl AntiWedge<DualNum> for AntiScalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<FlatPoint> for AntiScalar {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<Flector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * other.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group1(),
        )
    }
}
impl AntiWedge<Line> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(self[e12345]) * other.group0(),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * other.group1(),
        )
    }
}
impl AntiWedge<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(self[e12345]) * other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(self[e12345]) * other.group1(),
        )
    }
}
impl AntiWedge<MultiVector> for AntiScalar {
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
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(self[e12345]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * other.group1(),
            // e5
            self[e12345] * other[e5],
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * other.group3(),
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group4(),
            // e23, e31, e12
            Simd32x3::from(self[e12345]) * other.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group6(),
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * other.group7(),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * other.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group9(),
            // e1234
            self[e12345] * other[e1234],
        )
    }
}
impl AntiWedge<Plane> for AntiScalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<RoundPoint> for AntiScalar {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e12345]) * other.group0(), /* e5 */ self[e12345] * other[e5])
    }
}
impl AntiWedge<Scalar> for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e12345] * other[scalar])
    }
}
impl AntiWedge<Sphere> for AntiScalar {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group0(),
            // e1234
            self[e12345] * other[e1234],
        )
    }
}
impl AntiWedge<VersorEven> for AntiScalar {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self[e12345]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(self[e12345]) * other.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * other.group3(),
        )
    }
}
impl AntiWedge<VersorOdd> for AntiScalar {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(self[e12345]) * other.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group3(),
        )
    }
}
impl std::ops::Div<AntiWedgeInfix> for Circle {
    type Output = AntiWedgeInfixPartial<Circle>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiCircleRotor> for Circle {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(other[e41] * self[e235])
                - (other[e42] * self[e315])
                - (other[e43] * self[e125])
                - (other[e23] * self[e415])
                - (other[e31] * self[e425])
                - (other[e12] * self[e435])
                - (other[e45] * self[e321])
                - (other[e15] * self[e423])
                - (other[e25] * self[e431])
                - (other[e35] * self[e412]),
        )
    }
}
impl AntiWedge<AntiDipoleInversion> for Circle {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        2        0
    //    simd4        2        0        0
    // Totals...
    // yes simd       19       26        0
    //  no simd       25       30        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e412] * self[e315]) + (other[e415] * self[e321]) + (other[e321] * self[e415]) + (other[e315] * self[e412]),
                (other[e423] * self[e125]) + (other[e425] * self[e321]) + (other[e321] * self[e425]) + (other[e125] * self[e423]),
                (other[e431] * self[e235]) + (other[e435] * self[e321]) + (other[e321] * self[e435]) + (other[e235] * self[e431]),
                -(other[e412] * self[e435]) - (other[e415] * self[e423]) - (other[e425] * self[e431]) - (other[e435] * self[e412]),
            ]) - (other.group0().yzx() * self.group2().zxy()).with_w(other[e423] * self[e415])
                - (self.group0().yzx() * other.group2().zxy()).with_w(other[e431] * self[e425]),
            // e5
            -(other[e415] * self[e235])
                - (other[e425] * self[e315])
                - (other[e435] * self[e125])
                - (other[e235] * self[e415])
                - (other[e315] * self[e425])
                - (other[e125] * self[e435]),
        )
    }
}
impl AntiWedge<AntiDualNum> for Circle {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(other[e3215]) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(other[e3215]) * self.group1().xyz(),
        )
    }
}
impl AntiWedge<AntiFlatPoint> for Circle {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       12        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e315] * self[e412]) + (other[e321] * self[e415]),
                (other[e125] * self[e423]) + (other[e321] * self[e425]),
                (other[e235] * self[e431]) + (other[e321] * self[e435]),
                -(other[e315] * self[e425]) - (other[e125] * self[e435]),
            ]) - (self.group0().yzx() * other.group0().zxy()).with_w(other[e235] * self[e415]),
        )
    }
}
impl AntiWedge<AntiFlector> for Circle {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       12        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e315] * self[e412]) + (other[e321] * self[e415]),
                (other[e125] * self[e423]) + (other[e321] * self[e425]),
                (other[e235] * self[e431]) + (other[e321] * self[e435]),
                -(other[e315] * self[e425]) - (other[e125] * self[e435]),
            ]) - (self.group0().yzx() * other.group0().zxy()).with_w(other[e235] * self[e415]),
        )
    }
}
impl AntiWedge<AntiLine> for Circle {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(other[e23] * self[e415]) - (other[e31] * self[e425]) - (other[e12] * self[e435]) - (other[e15] * self[e423]) - (other[e25] * self[e431]) - (other[e35] * self[e412]),
        )
    }
}
impl AntiWedge<AntiMotor> for Circle {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(other[e3215]) * self.group0()).with_w(
                -(other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435])
                    - (other[e15] * self[e423])
                    - (other[e25] * self[e431])
                    - (other[e35] * self[e412]),
            ),
            // e15, e25, e35, e3215
            (self.group1().xyz() * other.group1().www()).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiScalar> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group1(),
            // e235, e315, e125
            Simd32x3::from(other[e12345]) * self.group2(),
        )
    }
}
impl AntiWedge<Circle> for Circle {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        2        0
    //    simd4        2        0        0
    // Totals...
    // yes simd       19       26        0
    //  no simd       25       30        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e412] * self[e315]) + (other[e415] * self[e321]) + (other[e321] * self[e415]) + (other[e315] * self[e412]),
                (other[e423] * self[e125]) + (other[e425] * self[e321]) + (other[e321] * self[e425]) + (other[e125] * self[e423]),
                (other[e431] * self[e235]) + (other[e435] * self[e321]) + (other[e321] * self[e435]) + (other[e235] * self[e431]),
                -(other[e412] * self[e435]) - (other[e415] * self[e423]) - (other[e425] * self[e431]) - (other[e435] * self[e412]),
            ]) - (other.group0().yzx() * self.group2().zxy()).with_w(other[e423] * self[e415])
                - (other.group2().zxy() * self.group0().yzx()).with_w(other[e431] * self[e425]),
            // e5
            -(other[e415] * self[e235])
                - (other[e425] * self[e315])
                - (other[e435] * self[e125])
                - (other[e235] * self[e415])
                - (other[e315] * self[e425])
                - (other[e125] * self[e435]),
        )
    }
}
impl AntiWedge<CircleRotor> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       25       40        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(other[e12345]) * self.group2()).with_w(
                -(self[e423] * other[e415])
                    - (self[e431] * other[e425])
                    - (self[e412] * other[e435])
                    - (self[e415] * other[e423])
                    - (self[e425] * other[e431])
                    - (self[e435] * other[e412]),
            ),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e412] * other[e315]) + (self[e415] * other[e321]) + (self[e321] * other[e415]) + (self[e315] * other[e412]),
                (self[e423] * other[e125]) + (self[e425] * other[e321]) + (self[e321] * other[e425]) + (self[e125] * other[e423]),
                (self[e431] * other[e235]) + (self[e435] * other[e321]) + (self[e321] * other[e435]) + (self[e235] * other[e431]),
                -(self[e415] * other[e235]) - (self[e425] * other[e315]) - (self[e435] * other[e125]) - (self[e125] * other[e435]),
            ]) - (self.group0().yzx() * other.group2().zxy()).with_w(self[e235] * other[e415])
                - (self.group2().zxy() * other.group0().yzx()).with_w(self[e315] * other[e425]),
        )
    }
}
impl AntiWedge<Dipole> for Circle {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(self[e423] * other[e15])
                - (self[e431] * other[e25])
                - (self[e412] * other[e35])
                - (self[e415] * other[e23])
                - (self[e425] * other[e31])
                - (self[e435] * other[e12])
                - (self[e321] * other[e45])
                - (self[e235] * other[e41])
                - (self[e315] * other[e42])
                - (self[e125] * other[e43]),
        )
    }
}
impl AntiWedge<DipoleInversion> for Circle {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       24        0
    //    simd3        2        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       29       40        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (self.group0().zxy() * other.group3().yzx()) - (self.group0().yzx() * other.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e423] * other[e3215]) + (self[e235] * other[e1234]),
                (self[e431] * other[e3215]) + (self[e315] * other[e1234]),
                (self[e412] * other[e3215]) + (self[e125] * other[e1234]),
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) - (self.group1().wwwx() * other.group3().xyzx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (self[e415] * other[e3215]) + (self[e315] * other[e4125]),
                (self[e425] * other[e3215]) + (self[e125] * other[e4235]),
                (self[e435] * other[e3215]) + (self[e235] * other[e4315]),
                -(self[e431] * other[e25])
                    - (self[e412] * other[e35])
                    - (self[e415] * other[e23])
                    - (self[e425] * other[e31])
                    - (self[e435] * other[e12])
                    - (self[e321] * other[e45])
                    - (self[e235] * other[e41])
                    - (self[e315] * other[e42])
                    - (self[e125] * other[e43]),
            ]) - (self.group2().zxy() * other.group3().yzx()).with_w(self[e423] * other[e15]),
        )
    }
}
impl AntiWedge<DualNum> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group1(),
            // e235, e315, e125
            Simd32x3::from(other[e12345]) * self.group2(),
        )
    }
}
impl AntiWedge<FlatPoint> for Circle {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(self[e423] * other[e15]) - (self[e431] * other[e25]) - (self[e412] * other[e35]) - (self[e321] * other[e45]),
        )
    }
}
impl AntiWedge<Flector> for Circle {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       15        0
    //    simd3        1        3        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       17       28        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (self.group0().zxy() * other.group1().yzx()) - (self.group0().yzx() * other.group1().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                self[e423] * other[e3215],
                self[e431] * other[e3215],
                self[e412] * other[e3215],
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) - (self.group1().wwwx() * other.group1().xyzx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (self[e415] * other[e3215]) + (self[e315] * other[e4125]),
                (self[e425] * other[e3215]) + (self[e125] * other[e4235]),
                (self[e435] * other[e3215]) + (self[e235] * other[e4315]),
                -(self[e431] * other[e25]) - (self[e412] * other[e35]) - (self[e321] * other[e45]),
            ]) - (self.group2().zxy() * other.group1().yzx()).with_w(self[e423] * other[e15]),
        )
    }
}
impl AntiWedge<Line> for Circle {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       13       18        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e412] * other[e315]) + (self[e321] * other[e415]),
                (self[e423] * other[e125]) + (self[e321] * other[e425]),
                (self[e431] * other[e235]) + (self[e321] * other[e435]),
                -(self[e431] * other[e425]) - (self[e412] * other[e435]),
            ]) - (self.group0().yzx() * other.group1().zxy()).with_w(self[e423] * other[e415]),
            // e5
            -(self[e415] * other[e235])
                - (self[e425] * other[e315])
                - (self[e435] * other[e125])
                - (self[e235] * other[e415])
                - (self[e315] * other[e425])
                - (self[e125] * other[e435]),
        )
    }
}
impl AntiWedge<Motor> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       13       28        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(other[e12345]) * self.group2()).with_w(-(self[e423] * other[e415]) - (self[e431] * other[e425]) - (self[e412] * other[e435])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e412] * other[e315]) + (self[e321] * other[e415]),
                (self[e423] * other[e125]) + (self[e321] * other[e425]),
                (self[e431] * other[e235]) + (self[e321] * other[e435]),
                -(self[e415] * other[e235]) - (self[e425] * other[e315]) - (self[e435] * other[e125]) - (self[e315] * other[e425]) - (self[e125] * other[e435]),
            ]) - (self.group0().yzx() * other.group1().zxy()).with_w(self[e235] * other[e415]),
        )
    }
}
impl AntiWedge<MultiVector> for Circle {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       42        0
    //    simd3        4       10        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       37       54        0
    //  no simd       54       80        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(self[e423] * other[e15])
                    - (self[e431] * other[e25])
                    - (self[e412] * other[e35])
                    - (self[e415] * other[e23])
                    - (self[e425] * other[e31])
                    - (self[e435] * other[e12])
                    - (self[e321] * other[e45])
                    - (self[e235] * other[e41])
                    - (self[e315] * other[e42])
                    - (self[e125] * other[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e412] * other[e315]) + (self[e415] * other[e321]) + (self[e321] * other[e415]) + (self[e315] * other[e412]),
                (self[e423] * other[e125]) + (self[e425] * other[e321]) + (self[e321] * other[e425]) + (self[e125] * other[e423]),
                (self[e431] * other[e235]) + (self[e435] * other[e321]) + (self[e321] * other[e435]) + (self[e235] * other[e431]),
                -(self[e412] * other[e435]) - (self[e415] * other[e423]) - (self[e425] * other[e431]) - (self[e435] * other[e412]),
            ]) - (self.group0().yzx() * other.group8().zxy()).with_w(self[e423] * other[e415])
                - (self.group2().zxy() * other.group7().yzx()).with_w(self[e431] * other[e425]),
            // e5
            -(self[e415] * other[e235])
                - (self[e425] * other[e315])
                - (self[e435] * other[e125])
                - (self[e235] * other[e415])
                - (self[e315] * other[e425])
                - (self[e125] * other[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e415] * other[e3215]) + (self[e315] * other[e4125]),
                (self[e425] * other[e3215]) + (self[e125] * other[e4235]),
                (self[e435] * other[e3215]) + (self[e235] * other[e4315]),
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) - (other.group9().yzxx() * self.group2().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (self.group0().zxy() * other.group9().yzx()) - (self.group0().yzx() * other.group9().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[e3215]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group2()) - (Simd32x3::from(self[e321]) * other.group9().xyz()),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group1(),
            // e423, e431, e412
            Simd32x3::from(other[e12345]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other[e12345]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl AntiWedge<Plane> for Circle {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        3        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd       14       24        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            (self.group0().zxy() * other.group0().yzx()) - (self.group0().yzx() * other.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                self[e423] * other[e3215],
                self[e431] * other[e3215],
                self[e412] * other[e3215],
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) - (self.group1().wwwx() * other.group0().xyzx()),
            // e15, e25, e35
            (Simd32x3::from(other[e3215]) * self.group1().xyz()) + (self.group2().yzx() * other.group0().zxy()) - (self.group2().zxy() * other.group0().yzx()),
        )
    }
}
impl AntiWedge<Sphere> for Circle {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        4        6        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       20       30        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (self.group0().zxy() * other.group0().yzx()) - (self.group0().yzx() * other.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e423] * other[e3215]) + (self[e235] * other[e1234]),
                (self[e431] * other[e3215]) + (self[e315] * other[e1234]),
                (self[e412] * other[e3215]) + (self[e125] * other[e1234]),
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) - (self.group1().wwwx() * other.group0().xyzx()),
            // e15, e25, e35
            (Simd32x3::from(other[e3215]) * self.group1().xyz()) + (self.group2().yzx() * other.group0().zxy()) - (self.group2().zxy() * other.group0().yzx()),
        )
    }
}
impl AntiWedge<VersorEven> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       25       40        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(other[e12345]) * self.group2()).with_w(
                -(self[e423] * other[e415])
                    - (self[e431] * other[e425])
                    - (self[e412] * other[e435])
                    - (self[e415] * other[e423])
                    - (self[e425] * other[e431])
                    - (self[e435] * other[e412]),
            ),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e412] * other[e315]) + (self[e415] * other[e321]) + (self[e321] * other[e415]) + (self[e315] * other[e412]),
                (self[e423] * other[e125]) + (self[e425] * other[e321]) + (self[e321] * other[e425]) + (self[e125] * other[e423]),
                (self[e431] * other[e235]) + (self[e435] * other[e321]) + (self[e321] * other[e435]) + (self[e235] * other[e431]),
                -(self[e415] * other[e235]) - (self[e425] * other[e315]) - (self[e435] * other[e125]) - (self[e125] * other[e435]),
            ]) - (self.group0().yzx() * other.group2().zxy()).with_w(self[e235] * other[e415])
                - (self.group2().zxy() * other.group0().yzx()).with_w(self[e315] * other[e425]),
        )
    }
}
impl AntiWedge<VersorOdd> for Circle {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       24        0
    //    simd3        2        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       29       40        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (self.group0().zxy() * other.group3().yzx()) - (self.group0().yzx() * other.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e423] * other[e3215]) + (self[e235] * other[e1234]),
                (self[e431] * other[e3215]) + (self[e315] * other[e1234]),
                (self[e412] * other[e3215]) + (self[e125] * other[e1234]),
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) - (self.group1().wwwx() * other.group3().xyzx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (self[e415] * other[e3215]) + (self[e315] * other[e4125]),
                (self[e425] * other[e3215]) + (self[e125] * other[e4235]),
                (self[e435] * other[e3215]) + (self[e235] * other[e4315]),
                -(self[e431] * other[e25])
                    - (self[e412] * other[e35])
                    - (self[e415] * other[e23])
                    - (self[e425] * other[e31])
                    - (self[e435] * other[e12])
                    - (self[e321] * other[e45])
                    - (self[e235] * other[e41])
                    - (self[e315] * other[e42])
                    - (self[e125] * other[e43]),
            ]) - (self.group2().zxy() * other.group3().yzx()).with_w(self[e423] * other[e15]),
        )
    }
}
impl std::ops::Div<AntiWedgeInfix> for CircleRotor {
    type Output = AntiWedgeInfixPartial<CircleRotor>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiCircleRotor> for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       11        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       14        0
    //  no simd       10       21        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * other.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(self[e12345]) * other.group2().xyz()).with_w(
                (other[scalar] * self[e12345])
                    - (other[e41] * self[e235])
                    - (other[e42] * self[e315])
                    - (other[e43] * self[e125])
                    - (other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435])
                    - (other[e45] * self[e321])
                    - (other[e15] * self[e423])
                    - (other[e25] * self[e431])
                    - (other[e35] * self[e412]),
            ),
        )
    }
}
impl AntiWedge<AntiDipoleInversion> for CircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       24        0
    //    simd3        0        3        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       21       30        0
    //  no simd       30       45        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(self[e12345]) * other.group2().xyz()).with_w(
                (other[e4] * self[e12345])
                    - (other[e423] * self[e415])
                    - (other[e431] * self[e425])
                    - (other[e412] * self[e435])
                    - (other[e415] * self[e423])
                    - (other[e425] * self[e431])
                    - (other[e435] * self[e412]),
            ),
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e415] * self[e321]) + (other[e321] * self[e415]) + (other[e315] * self[e412]) + (other[e1] * self[e12345]),
                (other[e425] * self[e321]) + (other[e321] * self[e425]) + (other[e125] * self[e423]) + (other[e2] * self[e12345]),
                (other[e435] * self[e321]) + (other[e321] * self[e435]) + (other[e235] * self[e431]) + (other[e3] * self[e12345]),
                -(other[e435] * self[e125]) - (other[e235] * self[e415]) - (other[e315] * self[e425]) - (other[e125] * self[e435]),
            ]) + (self.group2().yzxw() * other.group0().zxy().with_w(other[e5]))
                - (self.group2().zxyx() * other.group0().yzx().with_w(other[e415]))
                - (self.group0().yzx() * other.group2().zxy()).with_w(other[e425] * self[e315]),
        )
    }
}
impl AntiWedge<AntiDualNum> for CircleRotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0().xx().with_zw(other[e3215], other[scalar]) * self.group0().with_w(self[e12345]),
            // e15, e25, e35, e3215
            Simd32x4::from(other[e3215]) * self.group1().xyz().with_w(self[e12345]),
        )
    }
}
impl AntiWedge<AntiFlatPoint> for CircleRotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       16        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * other.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e315] * self[e412]) + (other[e321] * self[e415]),
                (other[e125] * self[e423]) + (other[e321] * self[e425]),
                (other[e235] * self[e431]) + (other[e321] * self[e435]),
                -(other[e315] * self[e425]) - (other[e125] * self[e435]),
            ]) - (self.group0().yzx() * other.group0().zxy()).with_w(other[e235] * self[e415]),
        )
    }
}
impl AntiWedge<AntiFlector> for CircleRotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd3        0        2        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        6       13        0
    //  no simd       12       20        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * other.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e321] * self[e415]) + (other[e1] * self[e12345]),
                (other[e321] * self[e425]) + (other[e2] * self[e12345]),
                (other[e321] * self[e435]) + (other[e3] * self[e12345]),
                -(other[e315] * self[e425]) - (other[e125] * self[e435]),
            ]) + (self.group0().zxy() * other.group0().yzx()).with_w(other[e5] * self[e12345])
                - (self.group0().yzx() * other.group0().zxy()).with_w(other[e235] * self[e415]),
        )
    }
}
impl AntiWedge<AntiLine> for CircleRotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn anti_wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e12345]) * other.group0()).with_w(
                -(other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435])
                    - (other[e15] * self[e423])
                    - (other[e25] * self[e431])
                    - (other[e35] * self[e412]),
            ),
            // e15, e25, e35, e3215
            (other.group1() * self.group2().www()).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiMotor> for CircleRotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7       14        0
    //  no simd       12       20        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                other[e23] * self[e12345],
                other[e31] * self[e12345],
                other[e12] * self[e12345],
                -(other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435])
                    - (other[e15] * self[e423])
                    - (other[e25] * self[e431])
                    - (other[e35] * self[e412]),
            ]) + (self.group0() * other.group1().www()).with_w(other[scalar] * self[e12345]),
            // e15, e25, e35, e3215
            ((Simd32x3::from(other[e3215]) * self.group1().xyz()) + (Simd32x3::from(self[e12345]) * other.group1().xyz())).with_w(other[e3215] * self[e12345]),
        )
    }
}
impl AntiWedge<AntiPlane> for CircleRotor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<AntiScalar> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(other[e12345]) * self.group2(),
        )
    }
}
impl AntiWedge<Circle> for CircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       25       40        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(self[e12345]) * other.group2()).with_w(
                -(other[e423] * self[e415])
                    - (other[e431] * self[e425])
                    - (other[e412] * self[e435])
                    - (other[e415] * self[e423])
                    - (other[e425] * self[e431])
                    - (other[e435] * self[e412]),
            ),
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e412] * self[e315]) + (other[e415] * self[e321]) + (other[e321] * self[e415]) + (other[e315] * self[e412]),
                (other[e423] * self[e125]) + (other[e425] * self[e321]) + (other[e321] * self[e425]) + (other[e125] * self[e423]),
                (other[e431] * self[e235]) + (other[e435] * self[e321]) + (other[e321] * self[e435]) + (other[e235] * self[e431]),
                -(other[e415] * self[e235]) - (other[e425] * self[e315]) - (other[e435] * self[e125]) - (other[e125] * self[e435]),
            ]) - (other.group0().yzx() * self.group2().zxy()).with_w(other[e235] * self[e415])
                - (other.group2().zxy() * self.group0().yzx()).with_w(other[e315] * self[e425]),
        )
    }
}
impl AntiWedge<CircleRotor> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       31        0
    //    simd3        1        4        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       24       37        0
    //  no simd       35       51        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x3::from(other[e12345]) * self.group0()) + (Simd32x3::from(self[e12345]) * other.group0())).with_w(other[e12345] * self[e12345]),
            // e415, e425, e435, e321
            (Simd32x4::from(other[e12345]) * self.group1()) + (Simd32x4::from(self[e12345]) * other.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other[e235] * self[e12345]) + (other[e12345] * self[e235]),
                (other[e315] * self[e12345]) + (other[e12345] * self[e315]),
                (other[e125] * self[e12345]) + (other[e12345] * self[e125]),
                -(other[e415] * self[e235])
                    - (other[e425] * self[e315])
                    - (other[e435] * self[e125])
                    - (other[e235] * self[e415])
                    - (other[e315] * self[e425])
                    - (other[e125] * self[e435]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e412] * self[e315]) + (other[e415] * self[e321]) + (other[e321] * self[e415]) + (other[e315] * self[e412]),
                (other[e423] * self[e125]) + (other[e425] * self[e321]) + (other[e321] * self[e425]) + (other[e125] * self[e423]),
                (other[e431] * self[e235]) + (other[e435] * self[e321]) + (other[e321] * self[e435]) + (other[e235] * self[e431]),
                -(other[e412] * self[e435]) - (other[e415] * self[e423]) - (other[e425] * self[e431]) - (other[e435] * self[e412]),
            ]) - (other.group0().yzx() * self.group2().zxy()).with_w(other[e423] * self[e415])
                - (self.group0().yzx() * other.group2().zxy()).with_w(other[e431] * self[e425]),
        )
    }
}
impl AntiWedge<Dipole> for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd        9       20        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * other.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(self[e12345]) * other.group2()).with_w(
                -(self[e423] * other[e15])
                    - (self[e431] * other[e25])
                    - (self[e412] * other[e35])
                    - (self[e415] * other[e23])
                    - (self[e425] * other[e31])
                    - (self[e435] * other[e12])
                    - (self[e321] * other[e45])
                    - (self[e235] * other[e41])
                    - (self[e315] * other[e42])
                    - (self[e125] * other[e43]),
            ),
        )
    }
}
impl AntiWedge<DipoleInversion> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       29        0
    //    simd3        3        6        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       24       37        0
    //  no simd       39       55        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self[e412] * other[e4315]) + (self[e415] * other[e1234]) + (self[e12345] * other[e41]),
                (self[e423] * other[e4125]) + (self[e425] * other[e1234]) + (self[e12345] * other[e42]),
                (self[e431] * other[e4235]) + (self[e435] * other[e1234]) + (self[e12345] * other[e43]),
                -(self[e431] * other[e25])
                    - (self[e412] * other[e35])
                    - (self[e415] * other[e23])
                    - (self[e425] * other[e31])
                    - (self[e435] * other[e12])
                    - (self[e321] * other[e45])
                    - (self[e235] * other[e41])
                    - (self[e315] * other[e42])
                    - (self[e125] * other[e43]),
            ]) - (self.group0().yzx() * other.group3().zxy()).with_w(self[e423] * other[e15]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e235] * other[e1234]) + (self[e12345] * other[e23]),
                (self[e315] * other[e1234]) + (self[e12345] * other[e31]),
                (self[e125] * other[e1234]) + (self[e12345] * other[e12]),
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) + (self.group0() * other.group3().www()).with_w(self[e12345] * other[e45])
                - (self.group1().wwwx() * other.group3().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e12345]) * other.group2().xyz()) + (Simd32x3::from(other[e3215]) * self.group1().xyz()) + (self.group2().yzx() * other.group3().zxy())
                - (self.group2().zxy() * other.group3().yzx()))
            .with_w(self[e12345] * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group3(),
        )
    }
}
impl AntiWedge<DualNum> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other[e12345]) * self.group0().with_w(self[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group1(),
            // e235, e315, e125, e5
            self.group2() * other.group0().yy().with_zw(other[e12345], other[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}
impl AntiWedge<FlatPoint> for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3        8        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(self[e12345] * other[e45]),
            // e15, e25, e35, scalar
            (other.group0().xyz() * self.group2().www()).with_w(-(self[e423] * other[e15]) - (self[e431] * other[e25]) - (self[e412] * other[e35]) - (self[e321] * other[e45])),
        )
    }
}
impl AntiWedge<Flector> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       10        0
    //    simd3        0        6        0
    //    simd4        6        2        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       27       36        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                self[e412] * other[e4315],
                self[e423] * other[e4125],
                self[e431] * other[e4235],
                -(self[e431] * other[e25]) - (self[e412] * other[e35]) - (self[e321] * other[e45]),
            ]) - (self.group0().yzx() * other.group1().zxy()).with_w(self[e423] * other[e15]),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125])) + (self.group0() * other.group1().www()).with_w(self[e12345] * other[e45])
                - (self.group1().wwwx() * other.group1().xyzx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(self[e12345]) * other.group0().xyz()).with_w(0.0)
                + (Simd32x3::from(other[e3215]) * self.group1().xyz()).with_w(0.0)
                + (self.group2().yzx() * other.group1().zxy()).with_w(0.0)
                - (self.group2().zxy() * other.group1().yzx()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group1(),
        )
    }
}
impl AntiWedge<Line> for CircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       24        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (other.group0() * self.group2().www()).with_w(0.0),
            // e235, e315, e125, e4
            (Simd32x3::from(self[e12345]) * other.group1()).with_w(-(self[e423] * other[e415]) - (self[e431] * other[e425]) - (self[e412] * other[e435])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e412] * other[e315]) + (self[e321] * other[e415]),
                (self[e423] * other[e125]) + (self[e321] * other[e425]),
                (self[e431] * other[e235]) + (self[e321] * other[e435]),
                -(self[e415] * other[e235]) - (self[e425] * other[e315]) - (self[e435] * other[e125]) - (self[e315] * other[e425]) - (self[e125] * other[e435]),
            ]) - (self.group0().yzx() * other.group1().zxy()).with_w(self[e235] * other[e415]),
        )
    }
}
impl AntiWedge<Motor> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       19        0
    //    simd3        1        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       20       36        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other[e12345]) * self.group0().with_w(self[e12345]),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e12345]) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * self.group1().xyz())).with_w(self[e321] * other[e12345]),
            // e235, e315, e125, e5
            Simd32x4::from([
                self[e12345] * other[e235],
                self[e12345] * other[e315],
                self[e12345] * other[e125],
                -(self[e415] * other[e235])
                    - (self[e425] * other[e315])
                    - (self[e435] * other[e125])
                    - (self[e235] * other[e415])
                    - (self[e315] * other[e425])
                    - (self[e125] * other[e435]),
            ]) + (self.group2() * other.group0().www().with_w(other[e5])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e412] * other[e315]) + (self[e321] * other[e415]),
                (self[e423] * other[e125]) + (self[e321] * other[e425]),
                (self[e431] * other[e235]) + (self[e321] * other[e435]),
                -(self[e431] * other[e425]) - (self[e412] * other[e435]),
            ]) - (self.group0().yzx() * other.group1().zxy()).with_w(self[e423] * other[e415]),
        )
    }
}
impl AntiWedge<MultiVector> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd3        8       16        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       46       68        0
    //  no simd       80      112        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e12345] * other[scalar])
                    - (self[e423] * other[e15])
                    - (self[e431] * other[e25])
                    - (self[e412] * other[e35])
                    - (self[e415] * other[e23])
                    - (self[e425] * other[e31])
                    - (self[e435] * other[e12])
                    - (self[e321] * other[e45])
                    - (self[e235] * other[e41])
                    - (self[e315] * other[e42])
                    - (self[e125] * other[e43]),
                self[e12345] * other[e12345],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e415] * other[e321]) + (self[e321] * other[e415]) + (self[e315] * other[e412]) + (self[e12345] * other[e1]),
                (self[e425] * other[e321]) + (self[e321] * other[e425]) + (self[e125] * other[e423]) + (self[e12345] * other[e2]),
                (self[e435] * other[e321]) + (self[e321] * other[e435]) + (self[e235] * other[e431]) + (self[e12345] * other[e3]),
                -(self[e412] * other[e435]) - (self[e415] * other[e423]) - (self[e425] * other[e431]) - (self[e435] * other[e412]),
            ]) + (self.group0().zxy() * other.group8().yzx()).with_w(self[e12345] * other[e4])
                - (self.group0().yzx() * other.group8().zxy()).with_w(self[e423] * other[e415])
                - (other.group7().yzx() * self.group2().zxy()).with_w(self[e431] * other[e425]),
            // e5
            (self[e12345] * other[e5])
                - (self[e415] * other[e235])
                - (self[e425] * other[e315])
                - (self[e435] * other[e125])
                - (self[e235] * other[e415])
                - (self[e315] * other[e425])
                - (self[e125] * other[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e315] * other[e4125]) + (self[e12345] * other[e15]),
                (self[e125] * other[e4235]) + (self[e12345] * other[e25]),
                (self[e235] * other[e4315]) + (self[e12345] * other[e35]),
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) + (self.group1().xyz() * other.group9().www()).with_w(self[e12345] * other[e45])
                - (other.group9().yzxx() * self.group2().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(self[e12345]) * other.group4()) + (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (self.group0().zxy() * other.group9().yzx())
                - (self.group0().yzx() * other.group9().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e12345]) * other.group5()) + (Simd32x3::from(other[e3215]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group2().xyz())
                - (Simd32x3::from(self[e321]) * other.group9().xyz()),
            // e415, e425, e435, e321
            (Simd32x4::from(self[e12345]) * other.group6()) + (Simd32x4::from(other[e12345]) * self.group1()),
            // e423, e431, e412
            (Simd32x3::from(self[e12345]) * other.group7()) + (Simd32x3::from(other[e12345]) * self.group0()),
            // e235, e315, e125
            (Simd32x3::from(self[e12345]) * other.group8()) + (Simd32x3::from(other[e12345]) * self.group2().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group9(),
            // e1234
            self[e12345] * other[e1234],
        )
    }
}
impl AntiWedge<Plane> for CircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        1        5        0
    //    simd4        3        2        0
    // Totals...
    // yes simd        5       12        0
    //  no simd       16       28        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            (self.group0().zxy() * other.group0().yzx()) - (self.group0().yzx() * other.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                self[e423] * other[e3215],
                self[e431] * other[e3215],
                self[e412] * other[e3215],
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) - (self.group1().wwwx() * other.group0().xyzx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(other[e3215]) * self.group1().xyz()).with_w(0.0) + (self.group2().yzx() * other.group0().zxy()).with_w(0.0)
                - (self.group2().zxy() * other.group0().yzx()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group0(),
        )
    }
}
impl AntiWedge<RoundPoint> for CircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e12345]) * other.group0(), /* e5 */ self[e12345] * other[e5])
    }
}
impl AntiWedge<Scalar> for CircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e12345] * other[scalar])
    }
}
impl AntiWedge<Sphere> for CircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        4        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       20       35        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (self.group0().zxy() * other.group0().yzx()) - (self.group0().yzx() * other.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e423] * other[e3215]) + (self[e235] * other[e1234]),
                (self[e431] * other[e3215]) + (self[e315] * other[e1234]),
                (self[e412] * other[e3215]) + (self[e125] * other[e1234]),
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) - (self.group1().wwwx() * other.group0().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e3215]) * self.group1().xyz()) + (self.group2().yzx() * other.group0().zxy()) - (self.group2().zxy() * other.group0().yzx()))
                .with_w(self[e12345] * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group0(),
        )
    }
}
impl AntiWedge<VersorEven> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       29        0
    //    simd3        1        5        0
    //    simd4        5        3        0
    // Totals...
    // yes simd       23       37        0
    //  no simd       40       56        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x3::from(self[e12345]) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * self.group0())).with_w(self[e12345] * other[e12345]),
            // e415, e425, e435, e321
            (Simd32x4::from(self[e12345]) * other.group1()) + (Simd32x4::from(other[e12345]) * self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                self[e12345] * other[e235],
                self[e12345] * other[e315],
                self[e12345] * other[e125],
                -(self[e415] * other[e235])
                    - (self[e425] * other[e315])
                    - (self[e435] * other[e125])
                    - (self[e235] * other[e415])
                    - (self[e315] * other[e425])
                    - (self[e125] * other[e435]),
            ]) + (self.group2() * other.group0().www().with_w(other[e5])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e415] * other[e321]) + (self[e321] * other[e415]) + (self[e315] * other[e412]) + (self[e12345] * other[e1]),
                (self[e425] * other[e321]) + (self[e321] * other[e425]) + (self[e125] * other[e423]) + (self[e12345] * other[e2]),
                (self[e435] * other[e321]) + (self[e321] * other[e435]) + (self[e235] * other[e431]) + (self[e12345] * other[e3]),
                -(self[e412] * other[e435]) - (self[e415] * other[e423]) - (self[e425] * other[e431]) - (self[e435] * other[e412]),
            ]) + (self.group0().zxy() * other.group2().yzx()).with_w(self[e12345] * other[e4])
                - (self.group0().yzx() * other.group2().zxy()).with_w(self[e423] * other[e415])
                - (self.group2().zxy() * other.group0().yzx()).with_w(self[e431] * other[e425]),
        )
    }
}
impl AntiWedge<VersorOdd> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       27        0
    //    simd3        3        7        0
    //    simd4        4        2        0
    // Totals...
    // yes simd       22       36        0
    //  no simd       40       56        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self[e415] * other[e1234]) + (self[e12345] * other[e41]),
                (self[e425] * other[e1234]) + (self[e12345] * other[e42]),
                (self[e435] * other[e1234]) + (self[e12345] * other[e43]),
                -(self[e431] * other[e25])
                    - (self[e412] * other[e35])
                    - (self[e415] * other[e23])
                    - (self[e425] * other[e31])
                    - (self[e435] * other[e12])
                    - (self[e321] * other[e45])
                    - (self[e235] * other[e41])
                    - (self[e315] * other[e42])
                    - (self[e125] * other[e43]),
            ]) + (self.group0().zxy() * other.group3().yzx()).with_w(self[e12345] * other[scalar])
                - (self.group0().yzx() * other.group3().zxy()).with_w(self[e423] * other[e15]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e235] * other[e1234]) + (self[e12345] * other[e23]),
                (self[e315] * other[e1234]) + (self[e12345] * other[e31]),
                (self[e125] * other[e1234]) + (self[e12345] * other[e12]),
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) + (self.group0() * other.group3().www()).with_w(self[e12345] * other[e45])
                - (self.group1().wwwx() * other.group3().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e12345]) * other.group2().xyz()) + (Simd32x3::from(other[e3215]) * self.group1().xyz()) + (self.group2().yzx() * other.group3().zxy())
                - (self.group2().zxy() * other.group3().yzx()))
            .with_w(self[e12345] * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group3(),
        )
    }
}
impl std::ops::Div<AntiWedgeInfix> for Dipole {
    type Output = AntiWedgeInfixPartial<Dipole>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiDipoleInversion> for Dipole {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(other[e423] * self[e15])
                - (other[e431] * self[e25])
                - (other[e412] * self[e35])
                - (other[e415] * self[e23])
                - (other[e425] * self[e31])
                - (other[e435] * self[e12])
                - (other[e321] * self[e45])
                - (other[e235] * self[e41])
                - (other[e315] * self[e42])
                - (other[e125] * self[e43]),
        )
    }
}
impl AntiWedge<AntiDualNum> for Dipole {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e3215]) * self.group0().with_w(self[e45]))
    }
}
impl AntiWedge<AntiFlatPoint> for Dipole {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(other[e235] * self[e41]) - (other[e315] * self[e42]) - (other[e125] * self[e43]) - (other[e321] * self[e45]),
        )
    }
}
impl AntiWedge<AntiFlector> for Dipole {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(other[e235] * self[e41]) - (other[e315] * self[e42]) - (other[e125] * self[e43]) - (other[e321] * self[e45]),
        )
    }
}
impl AntiWedge<AntiMotor> for Dipole {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ (self.group0() * other.group1().www()).with_w(other[e3215] * self[e45]))
    }
}
impl AntiWedge<AntiScalar> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345]) * self.group1(),
            // e15, e25, e35
            Simd32x3::from(other[e12345]) * self.group2(),
        )
    }
}
impl AntiWedge<Circle> for Dipole {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(other[e423] * self[e15])
                - (other[e431] * self[e25])
                - (other[e412] * self[e35])
                - (other[e415] * self[e23])
                - (other[e425] * self[e31])
                - (other[e435] * self[e12])
                - (other[e321] * self[e45])
                - (other[e235] * self[e41])
                - (other[e315] * self[e42])
                - (other[e125] * self[e43]),
        )
    }
}
impl AntiWedge<CircleRotor> for Dipole {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd        9       20        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345]) * self.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(other[e12345]) * self.group2()).with_w(
                -(other[e423] * self[e15])
                    - (other[e431] * self[e25])
                    - (other[e412] * self[e35])
                    - (other[e415] * self[e23])
                    - (other[e425] * self[e31])
                    - (other[e435] * self[e12])
                    - (other[e321] * self[e45])
                    - (other[e235] * self[e41])
                    - (other[e315] * self[e42])
                    - (other[e125] * self[e43]),
            ),
        )
    }
}
impl AntiWedge<DipoleInversion> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       20        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e3215]) + (self[e31] * other[e4125]),
                (self[e42] * other[e3215]) + (self[e12] * other[e4235]),
                (self[e43] * other[e3215]) + (self[e23] * other[e4315]),
                -(self[e43] * other[e4125]) - (self[e45] * other[e1234]),
            ]) - (other.group3().yzxy() * self.group1().zxy().with_w(self[e42]))
                - (self.group2() * other.group2().www()).with_w(self[e41] * other[e4235]),
            // e5
            (self[e45] * other[e3215]) + (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
        )
    }
}
impl AntiWedge<DualNum> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345]) * self.group1(),
            // e15, e25, e35
            Simd32x3::from(other[e12345]) * self.group2(),
        )
    }
}
impl AntiWedge<Flector> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       11       16        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e3215]) + (self[e31] * other[e4125]),
                (self[e42] * other[e3215]) + (self[e12] * other[e4235]),
                (self[e43] * other[e3215]) + (self[e23] * other[e4315]),
                -(self[e42] * other[e4315]) - (self[e43] * other[e4125]),
            ]) - (other.group1().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e5
            (self[e45] * other[e3215]) + (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
        )
    }
}
impl AntiWedge<Line> for Dipole {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(self[e41] * other[e235]) - (self[e42] * other[e315]) - (self[e43] * other[e125]) - (self[e23] * other[e415]) - (self[e31] * other[e425]) - (self[e12] * other[e435]),
        )
    }
}
impl AntiWedge<Motor> for Dipole {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       16        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345]) * self.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(other[e12345]) * self.group2()).with_w(
                -(self[e41] * other[e235])
                    - (self[e42] * other[e315])
                    - (self[e43] * other[e125])
                    - (self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435]),
            ),
        )
    }
}
impl AntiWedge<MultiVector> for Dipole {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       23        0
    //    simd3        0        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       18       28        0
    //  no simd       24       40        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(self[e41] * other[e235])
                    - (self[e42] * other[e315])
                    - (self[e43] * other[e125])
                    - (self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435])
                    - (self[e45] * other[e321])
                    - (self[e15] * other[e423])
                    - (self[e25] * other[e431])
                    - (self[e35] * other[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e3215]) + (self[e31] * other[e4125]),
                (self[e42] * other[e3215]) + (self[e12] * other[e4235]),
                (self[e43] * other[e3215]) + (self[e23] * other[e4315]),
                -(self[e43] * other[e4125]) - (self[e45] * other[e1234]),
            ]) - (other.group9().yzxy() * self.group1().zxy().with_w(self[e42]))
                - (Simd32x3::from(other[e1234]) * self.group2()).with_w(self[e41] * other[e4235]),
            // e5
            (self[e45] * other[e3215]) + (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
            // e15, e25, e35, e45
            Simd32x4::from(other[e12345]) * self.group2().with_w(self[e45]),
            // e41, e42, e43
            Simd32x3::from(other[e12345]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[e12345]) * self.group1().xyz(),
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
impl AntiWedge<Plane> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       11       16        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e3215]) + (self[e31] * other[e4125]),
                (self[e42] * other[e3215]) + (self[e12] * other[e4235]),
                (self[e43] * other[e3215]) + (self[e23] * other[e4315]),
                -(self[e42] * other[e4315]) - (self[e43] * other[e4125]),
            ]) - (other.group0().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e5
            (self[e45] * other[e3215]) + (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
        )
    }
}
impl AntiWedge<Sphere> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       20        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e3215]) + (self[e31] * other[e4125]),
                (self[e42] * other[e3215]) + (self[e12] * other[e4235]),
                (self[e43] * other[e3215]) + (self[e23] * other[e4315]),
                -(self[e43] * other[e4125]) - (self[e45] * other[e1234]),
            ]) - (other.group0().yzxy() * self.group1().zxy().with_w(self[e42]))
                - (Simd32x3::from(other[e1234]) * self.group2()).with_w(self[e41] * other[e4235]),
            // e5
            (self[e45] * other[e3215]) + (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
        )
    }
}
impl AntiWedge<VersorEven> for Dipole {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd        9       20        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345]) * self.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(other[e12345]) * self.group2()).with_w(
                -(self[e41] * other[e235])
                    - (self[e42] * other[e315])
                    - (self[e43] * other[e125])
                    - (self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435])
                    - (self[e45] * other[e321])
                    - (self[e15] * other[e423])
                    - (self[e25] * other[e431])
                    - (self[e35] * other[e412]),
            ),
        )
    }
}
impl AntiWedge<VersorOdd> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       20        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e3215]) + (self[e31] * other[e4125]),
                (self[e42] * other[e3215]) + (self[e12] * other[e4235]),
                (self[e43] * other[e3215]) + (self[e23] * other[e4315]),
                -(self[e43] * other[e4125]) - (self[e45] * other[e1234]),
            ]) - (other.group3().yzxy() * self.group1().zxy().with_w(self[e42]))
                - (self.group2() * other.group2().www()).with_w(self[e41] * other[e4235]),
            // e5
            (self[e45] * other[e3215]) + (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
        )
    }
}
impl std::ops::Div<AntiWedgeInfix> for DipoleInversion {
    type Output = AntiWedgeInfixPartial<DipoleInversion>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiCircleRotor> for DipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       20        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(other[e41] * self[e3215]) - (other[e31] * self[e4125]),
                -(other[e42] * self[e3215]) - (other[e12] * self[e4235]),
                -(other[e43] * self[e3215]) - (other[e23] * self[e4315]),
                (other[e43] * self[e4125]) + (other[e45] * self[e1234]),
            ]) + (self.group3().yzxx() * other.group1().zxy().with_w(other[e41]))
                + (other.group2().xyz() * self.group2().www()).with_w(other[e42] * self[e4315]),
            // e5
            -(other[e45] * self[e3215]) - (other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]),
        )
    }
}
impl AntiWedge<AntiDipoleInversion> for DipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        2        4        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       21       28        0
    //  no simd       37       45        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group1().xyz()) + (other.group0().zxy() * self.group3().yzx()) - (other.group0().yzx() * self.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other[e423] * self[e3215]) + (other[e235] * self[e1234]),
                (other[e431] * self[e3215]) + (other[e315] * self[e1234]),
                (other[e412] * self[e3215]) + (other[e125] * self[e1234]),
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) - (other.group1().wwwx() * self.group3().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(self[e3215]) * other.group1().xyz().with_w(other[e4]))
                + (self.group3().zxyx() * other.group2().yzx().with_w(other[e1]))
                + Simd32x3::from(0.0).with_w(
                    (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e5] * self[e1234])
                        - (other[e431] * self[e25])
                        - (other[e412] * self[e35])
                        - (other[e415] * self[e23])
                        - (other[e425] * self[e31])
                        - (other[e435] * self[e12])
                        - (other[e321] * self[e45])
                        - (other[e235] * self[e41])
                        - (other[e315] * self[e42])
                        - (other[e125] * self[e43]),
                )
                - (other.group2().zxy() * self.group3().yzx()).with_w(other[e423] * self[e15]),
        )
    }
}
impl AntiWedge<AntiDualNum> for DipoleInversion {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e3215]) * self.group3().xyz().with_w(self[e1234]),
            // e1, e2, e3, e5
            Simd32x4::from(other[e3215]) * self.group0().with_w(self[e45]),
        )
    }
}
impl AntiWedge<AntiFlatPoint> for DipoleInversion {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        7        0
    //    simd3        1        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        9       16        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                other[e235] * self[e1234],
                other[e315] * self[e1234],
                other[e125] * self[e1234],
                -(other[e315] * self[e42]) - (other[e125] * self[e43]) - (other[e321] * self[e45]),
            ]) - (self.group3().xyz() * other.group0().www()).with_w(other[e235] * self[e41]),
            // e15, e25, e35, e3215
            ((other.group0().yzx() * self.group3().zxy()) - (other.group0().zxy() * self.group3().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiFlector> for DipoleInversion {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        8        0
    //    simd3        1        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        8       12        0
    //  no simd       16       20        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(
                (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e5] * self[e1234])
                    - (other[e315] * self[e42])
                    - (other[e125] * self[e43])
                    - (other[e321] * self[e45]),
            ) + (other.group0().xyz() * self.group2().www()).with_w(other[e1] * self[e4235])
                - (self.group3().xyz() * other.group0().www()).with_w(other[e235] * self[e41]),
            // e15, e25, e35, e3215
            ((other.group0().yzx() * self.group3().zxy()) - (other.group0().zxy() * self.group3().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiLine> for DipoleInversion {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn anti_wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e12] * self[e4315]) + (other[e15] * self[e1234]),
                (other[e23] * self[e4125]) + (other[e25] * self[e1234]),
                (other[e31] * self[e4235]) + (other[e35] * self[e1234]),
                -(other[e25] * self[e4315]) - (other[e35] * self[e4125]),
            ]) - (self.group3().zxyx() * other.group0().yzx().with_w(other[e15])),
        )
    }
}
impl AntiWedge<AntiMotor> for DipoleInversion {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       20        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e3215]) * self.group3().xyz().with_w(self[e1234]),
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e12] * self[e4315]) + (other[e15] * self[e1234]),
                (other[e23] * self[e4125]) + (other[e25] * self[e1234]),
                (other[e31] * self[e4235]) + (other[e35] * self[e1234]),
                -(other[e25] * self[e4315]) - (other[e35] * self[e4125]),
            ]) + (self.group0() * other.group1().www()).with_w(other[e3215] * self[e45])
                - (self.group3().zxyx() * other.group0().yzx().with_w(other[e15])),
        )
    }
}
impl AntiWedge<AntiPlane> for DipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e5] * self[e1234]),
        )
    }
}
impl AntiWedge<AntiScalar> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[e12345]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group3(),
        )
    }
}
impl AntiWedge<Circle> for DipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       24        0
    //    simd3        2        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       29       40        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group1().xyz()) + (other.group0().zxy() * self.group3().yzx()) - (other.group0().yzx() * self.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other[e423] * self[e3215]) + (other[e235] * self[e1234]),
                (other[e431] * self[e3215]) + (other[e315] * self[e1234]),
                (other[e412] * self[e3215]) + (other[e125] * self[e1234]),
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) - (other.group1().wwwx() * self.group3().xyzx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (other[e415] * self[e3215]) + (other[e315] * self[e4125]),
                (other[e425] * self[e3215]) + (other[e125] * self[e4235]),
                (other[e435] * self[e3215]) + (other[e235] * self[e4315]),
                -(other[e431] * self[e25])
                    - (other[e412] * self[e35])
                    - (other[e415] * self[e23])
                    - (other[e425] * self[e31])
                    - (other[e435] * self[e12])
                    - (other[e321] * self[e45])
                    - (other[e235] * self[e41])
                    - (other[e315] * self[e42])
                    - (other[e125] * self[e43]),
            ]) - (other.group2().zxy() * self.group3().yzx()).with_w(other[e423] * self[e15]),
        )
    }
}
impl AntiWedge<CircleRotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       29        0
    //    simd3        3        6        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       24       37        0
    //  no simd       39       55        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other[e412] * self[e4315]) + (other[e415] * self[e1234]) + (other[e12345] * self[e41]),
                (other[e423] * self[e4125]) + (other[e425] * self[e1234]) + (other[e12345] * self[e42]),
                (other[e431] * self[e4235]) + (other[e435] * self[e1234]) + (other[e12345] * self[e43]),
                -(other[e431] * self[e25])
                    - (other[e412] * self[e35])
                    - (other[e415] * self[e23])
                    - (other[e425] * self[e31])
                    - (other[e435] * self[e12])
                    - (other[e321] * self[e45])
                    - (other[e235] * self[e41])
                    - (other[e315] * self[e42])
                    - (other[e125] * self[e43]),
            ]) - (other.group0().yzx() * self.group3().zxy()).with_w(other[e423] * self[e15]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other[e235] * self[e1234]) + (other[e12345] * self[e23]),
                (other[e315] * self[e1234]) + (other[e12345] * self[e31]),
                (other[e125] * self[e1234]) + (other[e12345] * self[e12]),
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) + (other.group0() * self.group3().www()).with_w(other[e12345] * self[e45])
                - (other.group1().wwwx() * self.group3().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e12345]) * self.group2().xyz()) + (Simd32x3::from(self[e3215]) * other.group1().xyz()) + (other.group2().yzx() * self.group3().zxy())
                - (other.group2().zxy() * self.group3().yzx()))
            .with_w(other[e12345] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group3(),
        )
    }
}
impl AntiWedge<Dipole> for DipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       20        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(other[e41] * self[e3215]) - (other[e31] * self[e4125]),
                -(other[e42] * self[e3215]) - (other[e12] * self[e4235]),
                -(other[e43] * self[e3215]) - (other[e23] * self[e4315]),
                (other[e43] * self[e4125]) + (other[e45] * self[e1234]),
            ]) + (self.group3().yzxy() * other.group1().zxy().with_w(other[e42]))
                + (other.group2() * self.group2().www()).with_w(other[e41] * self[e4235]),
            // e5
            -(other[e45] * self[e3215]) - (other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]),
        )
    }
}
impl AntiWedge<DipoleInversion> for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       10        0
    //    simd3        1        6        0
    //    simd4       10        8        0
    // Totals...
    // yes simd       16       24        0
    //  no simd       48       60        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * other.group3().xyz()) - (Simd32x3::from(other[e1234]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (other.group3().yzxw() * self.group3().zxy().with_w(self[e1234])) - (self.group3().yzxw() * other.group3().zxy().with_w(other[e1234])),
            // e235, e315, e125, e4
            (self.group3().xyzx() * other.group3().www().with_w(other[e41]))
                + Simd32x3::from(0.0).with_w(
                    (other[e42] * self[e4315]) + (other[e43] * self[e4125]) + (other[e45] * self[e1234])
                        - (other[e1234] * self[e45])
                        - (other[e4315] * self[e42])
                        - (other[e4125] * self[e43]),
                )
                - (other.group3().xyz() * self.group3().www()).with_w(other[e4235] * self[e41]),
            // e1, e2, e3, e5
            (other.group3().zxyw() * self.group1().yzxw())
                + (self.group2().wwwz() * other.group2().xyz().with_w(other[e4125]))
                + (self.group0() * other.group3().www()).with_w(other[e4235] * self[e15])
                + (other.group1().zxy() * self.group3().yzx()).with_w(other[e4315] * self[e25])
                - (Simd32x4::from(self[e3215]) * other.group0().with_w(other[e45]))
                - (other.group2().wwwy() * self.group2().xyz().with_w(self[e4315]))
                - (self.group3().zxyx() * other.group1().yzx().with_w(other[e15]))
                - (other.group3().yzx() * self.group1().zxy()).with_w(other[e35] * self[e4125]),
        )
    }
}
impl AntiWedge<DualNum> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0().yy().with_zw(other[e12345], other[e5]) * self.group0().with_w(self[e1234]),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[e12345]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group3(),
        )
    }
}
impl AntiWedge<FlatPoint> for DipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * other.group0(),
            // e5
            -(self[e4235] * other[e15]) - (self[e4315] * other[e25]) - (self[e4125] * other[e35]) - (self[e3215] * other[e45]),
        )
    }
}
impl AntiWedge<Flector> for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        1        5        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       11       18        0
    //  no simd       31       40        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * other.group1().xyz(),
            // e415, e425, e435, e321
            ((self.group3().zxy() * other.group1().yzx()) - (self.group3().yzx() * other.group1().zxy())).with_w(self[e1234] * other[e3215]),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(-(self[e42] * other[e4315]) - (self[e43] * other[e4125])) + (self.group3().xyz() * other.group1().www()).with_w(self[e1234] * other[e45])
                - (other.group1().xyzx() * self.group3().www().with_w(self[e41])),
            // e1, e2, e3, e5
            (Simd32x4::from(other[e3215]) * self.group0().with_w(self[e45]))
                + (self.group2().wwwy() * other.group0().xyz().with_w(other[e4315]))
                + (other.group1().zxyx() * self.group1().yzx().with_w(self[e15]))
                + Simd32x3::from(0.0).with_w((self[e35] * other[e4125]) - (self[e4315] * other[e25]) - (self[e4125] * other[e35]) - (self[e3215] * other[e45]))
                - (self.group1().zxy() * other.group1().yzx()).with_w(self[e4235] * other[e15]),
        )
    }
}
impl AntiWedge<Line> for DipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       24        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group0(),
            // e23, e31, e12, e45
            (Simd32x3::from(self[e1234]) * other.group1()).with_w(-(self[e4235] * other[e415]) - (self[e4315] * other[e425]) - (self[e4125] * other[e435])),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (self[e4125] * other[e315]) + (self[e3215] * other[e415]),
                (self[e4235] * other[e125]) + (self[e3215] * other[e425]),
                (self[e4315] * other[e235]) + (self[e3215] * other[e435]),
                -(self[e42] * other[e315]) - (self[e43] * other[e125]) - (self[e23] * other[e415]) - (self[e31] * other[e425]) - (self[e12] * other[e435]),
            ]) - (other.group1().zxy() * self.group3().yzx()).with_w(self[e41] * other[e235]),
        )
    }
}
impl AntiWedge<Motor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       17        0
    //    simd3        3        5        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       24       40        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                self[e1234] * other[e415],
                self[e1234] * other[e425],
                self[e1234] * other[e435],
                -(self[e41] * other[e235])
                    - (self[e42] * other[e315])
                    - (self[e43] * other[e125])
                    - (self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435]),
            ]) + (self.group0() * other.group0().www()).with_w(self[e1234] * other[e5]),
            // e23, e31, e12, e45
            Simd32x4::from([
                self[e1234] * other[e235],
                self[e1234] * other[e315],
                self[e1234] * other[e125],
                -(self[e4235] * other[e415]) - (self[e4315] * other[e425]) - (self[e4125] * other[e435]),
            ]) + (Simd32x4::from(other[e12345]) * self.group1()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e3215]) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * self.group2().xyz()) + (self.group3().zxy() * other.group1().yzx())
                - (self.group3().yzx() * other.group1().zxy()))
            .with_w(self[e1234] * other[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group3(),
        )
    }
}
impl AntiWedge<MultiVector> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       37        0
    //    simd3        8       17        0
    //    simd4       10        8        0
    // Totals...
    // yes simd       43       62        0
    //  no simd       89      120        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e1234] * other[e5]) + (self[e4235] * other[e1]) + (self[e4315] * other[e2]) + (self[e4125] * other[e3]) + (self[e3215] * other[e4])
                    - (self[e41] * other[e235])
                    - (self[e42] * other[e315])
                    - (self[e43] * other[e125])
                    - (self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435])
                    - (self[e45] * other[e321])
                    - (self[e15] * other[e423])
                    - (self[e25] * other[e431])
                    - (self[e35] * other[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * other.group3())
                + (self.group3().yzxy() * other.group5().zxy().with_w(other[e42]))
                + (self.group0() * other.group9().www()).with_w(self[e4235] * other[e41])
                + (self.group1().yzx() * other.group9().zxy()).with_w(self[e4125] * other[e43])
                - (Simd32x4::from(other[e1234]) * self.group2().xyz().with_w(self[e45]))
                - (other.group9().yzxz() * self.group1().zxy().with_w(self[e43]))
                - (other.group4() * self.group3().www()).with_w(self[e41] * other[e4235])
                - (other.group5().yzx() * self.group3().zxy()).with_w(self[e42] * other[e4315]),
            // e5
            (self[e45] * other[e3215]) + (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125])
                - (self[e4235] * other[e15])
                - (self[e4315] * other[e25])
                - (self[e4125] * other[e35])
                - (self[e3215] * other[e45]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e4125] * other[e315]) + (self[e3215] * other[e415]),
                (self[e4235] * other[e125]) + (self[e3215] * other[e425]),
                (self[e4315] * other[e235]) + (self[e3215] * other[e435]),
                -(self[e4315] * other[e425]) - (self[e4125] * other[e435]),
            ]) + (Simd32x4::from(other[e12345]) * self.group2().xyz().with_w(self[e45]))
                - (other.group8().zxy() * self.group3().yzx()).with_w(self[e4235] * other[e415]),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group6().xyz()) + (Simd32x3::from(other[e12345]) * self.group0()) + (other.group7().zxy() * self.group3().yzx())
                - (other.group7().yzx() * self.group3().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e1234]) * other.group8()) + (Simd32x3::from(self[e3215]) * other.group7()) + (Simd32x3::from(other[e12345]) * self.group1().xyz())
                - (Simd32x3::from(other[e321]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (other.group9().yzxw() * self.group3().zxy().with_w(self[e1234])) - (self.group3().yzxw() * other.group9().zxy().with_w(other[e1234])),
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * other.group9().xyz()) - (Simd32x3::from(other[e1234]) * self.group3().xyz()),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group3().xyz()) - (Simd32x3::from(self[e3215]) * other.group9().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group3(),
            // e1234
            self[e1234] * other[e12345],
        )
    }
}
impl AntiWedge<Plane> for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       14        0
    //    simd3        1        3        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        6       20        0
    //  no simd       17       35        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * other.group0().xyz(),
            // e415, e425, e435, e321
            ((self.group3().zxy() * other.group0().yzx()) - (self.group3().yzx() * other.group0().zxy())).with_w(self[e1234] * other[e3215]),
            // e235, e315, e125, e4
            Simd32x4::from([
                self[e4235] * other[e3215],
                self[e4315] * other[e3215],
                self[e4125] * other[e3215],
                -(self[e42] * other[e4315]) - (self[e43] * other[e4125]),
            ]) - (other.group0().xyzx() * self.group3().www().with_w(self[e41])),
            // e1, e2, e3, e5
            Simd32x4::from([
                self[e12] * other[e4315] * -1.0,
                self[e23] * other[e4125] * -1.0,
                self[e31] * other[e4235] * -1.0,
                (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
            ]) + (Simd32x4::from(other[e3215]) * self.group0().with_w(self[e45]))
                + (other.group0().zxyx() * self.group1().yzx().with_w(self[e15])),
        )
    }
}
impl AntiWedge<RoundPoint> for DipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e1234] * other[e5]) + (self[e4235] * other[e1]) + (self[e4315] * other[e2]) + (self[e4125] * other[e3]) + (self[e3215] * other[e4]),
        )
    }
}
impl AntiWedge<Sphere> for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       14        0
    //    simd3        1        2        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       11       21        0
    //  no simd       25       40        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * other.group0().xyz()) - (Simd32x3::from(other[e1234]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (other.group0().yzxw() * self.group3().zxy().with_w(self[e1234])) - (self.group3().yzxw() * other.group0().zxy().with_w(other[e1234])),
            // e235, e315, e125, e4
            Simd32x4::from([
                self[e4235] * other[e3215],
                self[e4315] * other[e3215],
                self[e4125] * other[e3215],
                -(self[e42] * other[e4315]) - (self[e43] * other[e4125]) - (self[e45] * other[e1234]),
            ]) - (other.group0().xyzx() * self.group3().www().with_w(self[e41])),
            // e1, e2, e3, e5
            Simd32x4::from([
                -(self[e12] * other[e4315]) - (self[e15] * other[e1234]),
                -(self[e23] * other[e4125]) - (self[e25] * other[e1234]),
                -(self[e31] * other[e4235]) - (self[e35] * other[e1234]),
                (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
            ]) + (Simd32x4::from(other[e3215]) * self.group0().with_w(self[e45]))
                + (other.group0().zxyx() * self.group1().yzx().with_w(self[e15])),
        )
    }
}
impl AntiWedge<VersorEven> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        0
    //    simd3        3        7        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       23       34        0
    //  no simd       47       60        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group3().yzxy() * other.group0().zxy().with_w(other[e2]))
                + Simd32x3::from(0.0).with_w(
                    (self[e4125] * other[e3]) + (self[e3215] * other[e4])
                        - (self[e42] * other[e315])
                        - (self[e43] * other[e125])
                        - (self[e23] * other[e415])
                        - (self[e31] * other[e425])
                        - (self[e12] * other[e435])
                        - (self[e45] * other[e321])
                        - (self[e15] * other[e423])
                        - (self[e25] * other[e431])
                        - (self[e35] * other[e412]),
                )
                + (self.group0() * other.group0().www()).with_w(self[e1234] * other[e5])
                + (other.group1().xyz() * self.group2().www()).with_w(self[e4235] * other[e1])
                - (self.group3().zxy() * other.group0().yzx()).with_w(self[e41] * other[e235]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e1234] * other[e235]) + (self[e3215] * other[e423]),
                (self[e1234] * other[e315]) + (self[e3215] * other[e431]),
                (self[e1234] * other[e125]) + (self[e3215] * other[e412]),
                -(self[e4315] * other[e425]) - (self[e4125] * other[e435]),
            ]) + (Simd32x4::from(other[e12345]) * self.group1())
                - (self.group3().xyzx() * other.group1().wwwx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e3215]) * other.group1().xyz()) + (Simd32x3::from(other[e12345]) * self.group2().xyz()) + (self.group3().zxy() * other.group2().yzx())
                - (self.group3().yzx() * other.group2().zxy()))
            .with_w(self[e1234] * other[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group3(),
        )
    }
}
impl AntiWedge<VersorOdd> for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        1        5        0
    //    simd4       10        9        0
    // Totals...
    // yes simd       16       23        0
    //  no simd       48       60        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * other.group3().xyz()) - (Simd32x3::from(other[e1234]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (other.group3().yzxw() * self.group3().zxy().with_w(self[e1234])) - (self.group3().yzxw() * other.group3().zxy().with_w(other[e1234])),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(
                (self[e4235] * other[e41]) + (self[e4315] * other[e42]) + (self[e4125] * other[e43])
                    - (self[e42] * other[e4315])
                    - (self[e43] * other[e4125])
                    - (self[e45] * other[e1234]),
            ) + (self.group3().xyz() * other.group3().www()).with_w(self[e1234] * other[e45])
                - (other.group3().xyzx() * self.group3().www().with_w(self[e41])),
            // e1, e2, e3, e5
            (Simd32x4::from(other[e3215]) * self.group0().with_w(self[e45]))
                + (self.group2().wwwy() * other.group2().xyz().with_w(other[e4315]))
                + (other.group3().zxyx() * self.group1().yzx().with_w(self[e15]))
                + (self.group3().yzx() * other.group1().zxy()).with_w(self[e35] * other[e4125])
                - (Simd32x4::from(self[e3215]) * other.group0().xyz().with_w(other[e45]))
                - (self.group3().zxyz() * other.group1().yzx().with_w(other[e35]))
                - (other.group2().wwwy() * self.group2().xyz().with_w(self[e4315]))
                - (self.group1().zxy() * other.group3().yzx()).with_w(self[e4235] * other[e15]),
        )
    }
}
impl std::ops::Div<AntiWedgeInfix> for DualNum {
    type Output = AntiWedgeInfixPartial<DualNum>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiCircleRotor> for DualNum {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * other.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(self[e12345]) * other.group2(),
        )
    }
}
impl AntiWedge<AntiDipoleInversion> for DualNum {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(self[e12345]) * other.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(self[e12345]) * other.group3(),
        )
    }
}
impl AntiWedge<AntiDualNum> for DualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<AntiFlatPoint> for DualNum {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<AntiFlector> for DualNum {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * other.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(self[e12345]) * other.group1(),
        )
    }
}
impl AntiWedge<AntiLine> for DualNum {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(self[e12345]) * other.group0(),
            // e15, e25, e35
            Simd32x3::from(self[e12345]) * other.group1(),
        )
    }
}
impl AntiWedge<AntiMotor> for DualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(self[e12345]) * other.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(self[e12345]) * other.group1(),
        )
    }
}
impl AntiWedge<AntiPlane> for DualNum {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<AntiScalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<Circle> for DualNum {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1(),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * other.group2(),
        )
    }
}
impl AntiWedge<CircleRotor> for DualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self[e12345]) * other.group0().with_w(other[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1(),
            // e235, e315, e125, e5
            other.group2() * self.group0().yy().with_zw(self[e12345], self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}
impl AntiWedge<Dipole> for DualNum {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * other.group1(),
            // e15, e25, e35
            Simd32x3::from(self[e12345]) * other.group2(),
        )
    }
}
impl AntiWedge<DipoleInversion> for DualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().yy().with_zw(self[e12345], self[e5]) * other.group0().with_w(other[e1234]),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(self[e12345]) * other.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group3(),
        )
    }
}
impl AntiWedge<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(
            // e5, e12345
            Simd32x2::from([(other[e5] * self[e12345]) + (other[e12345] * self[e5]), other[e12345] * self[e12345]]),
        )
    }
}
impl AntiWedge<FlatPoint> for DualNum {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * other.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group1(),
        )
    }
}
impl AntiWedge<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(self[e12345]) * other.group0(),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * other.group1(),
        )
    }
}
impl AntiWedge<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1       10        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(self[e12345]) * other.group0(),
            // e235, e315, e125, e5
            self.group0().yy().with_zw(self[e12345], (self[e5] * other[e12345]) + (self[e12345] * other[e5])) * other.group1().xyz().with_w(1.0),
        )
    }
}
impl AntiWedge<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       14        0
    //  no simd        2       34        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self[e5] * other[e1234]) + (self[e12345] * other[scalar]), self[e12345] * other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * other.group1(),
            // e5
            (self[e5] * other[e12345]) + (self[e12345] * other[e5]),
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * other.group3(),
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group4(),
            // e23, e31, e12
            Simd32x3::from(self[e12345]) * other.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group6(),
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * other.group7(),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * other.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group9(),
            // e1234
            self[e12345] * other[e1234],
        )
    }
}
impl AntiWedge<Plane> for DualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<RoundPoint> for DualNum {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e12345]) * other.group0(), /* e5 */ self[e12345] * other[e5])
    }
}
impl AntiWedge<Scalar> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e12345] * other[scalar])
    }
}
impl AntiWedge<Sphere> for DualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(self[e5] * other[e1234]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(self[e12345] * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group0(),
        )
    }
}
impl AntiWedge<VersorEven> for DualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1       18        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self[e12345]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1(),
            // e235, e315, e125, e5
            self.group0().yy().with_zw(self[e12345], (self[e5] * other[e12345]) + (self[e12345] * other[e5])) * other.group2().xyz().with_w(1.0),
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * other.group3(),
        )
    }
}
impl AntiWedge<VersorOdd> for DualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1       18        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().yy().with_zw(self[e12345], (self[e5] * other[e1234]) + (self[e12345] * other[scalar])) * other.group0().xyz().with_w(1.0),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(self[e12345]) * other.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group3(),
        )
    }
}
impl std::ops::Div<AntiWedgeInfix> for FlatPoint {
    type Output = AntiWedgeInfixPartial<FlatPoint>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiDipoleInversion> for FlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(other[e423] * self[e15]) - (other[e431] * self[e25]) - (other[e412] * self[e35]) - (other[e321] * self[e45]),
        )
    }
}
impl AntiWedge<AntiDualNum> for FlatPoint {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([other[e3215] * self[e45], 1.0]) * Simd32x2::from([1.0, 0.0]))
    }
}
impl AntiWedge<AntiFlatPoint> for FlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e321] * self[e45] * -1.0)
    }
}
impl AntiWedge<AntiFlector> for FlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e321] * self[e45] * -1.0)
    }
}
impl AntiWedge<AntiMotor> for FlatPoint {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([other[e3215] * self[e45], 1.0]) * Simd32x2::from([1.0, 0.0]))
    }
}
impl AntiWedge<AntiScalar> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<Circle> for FlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(other[e423] * self[e15]) - (other[e431] * self[e25]) - (other[e412] * self[e35]) - (other[e321] * self[e45]),
        )
    }
}
impl AntiWedge<CircleRotor> for FlatPoint {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3        8        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(other[e12345] * self[e45]),
            // e15, e25, e35, scalar
            (self.group0().xyz() * other.group2().www()).with_w(-(other[e423] * self[e15]) - (other[e431] * self[e25]) - (other[e412] * self[e35]) - (other[e321] * self[e45])),
        )
    }
}
impl AntiWedge<DipoleInversion> for FlatPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234]) * self.group0() * Simd32x4::from(-1.0),
            // e5
            (other[e4235] * self[e15]) + (other[e4315] * self[e25]) + (other[e4125] * self[e35]) + (other[e3215] * self[e45]),
        )
    }
}
impl AntiWedge<DualNum> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<Flector> for FlatPoint {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]) + (self[e45] * other[e3215]),
            0.0,
        ]))
    }
}
impl AntiWedge<Motor> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<MultiVector> for FlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       20        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([-(self[e15] * other[e423]) - (self[e25] * other[e431]) - (self[e35] * other[e412]) - (self[e45] * other[e321]), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234]) * self.group0() * Simd32x4::from(-1.0),
            // e5
            (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]) + (self[e45] * other[e3215]),
            // e15, e25, e35, e45
            Simd32x4::from(other[e12345]) * self.group0(),
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
impl AntiWedge<Plane> for FlatPoint {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]) + (self[e45] * other[e3215]),
            0.0,
        ]))
    }
}
impl AntiWedge<Sphere> for FlatPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234]) * self.group0() * Simd32x4::from(-1.0),
            // e5
            (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]) + (self[e45] * other[e3215]),
        )
    }
}
impl AntiWedge<VersorEven> for FlatPoint {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3        8        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(self[e45] * other[e12345]),
            // e15, e25, e35, scalar
            (Simd32x3::from(other[e12345]) * self.group0().xyz())
                .with_w(-(self[e15] * other[e423]) - (self[e25] * other[e431]) - (self[e35] * other[e412]) - (self[e45] * other[e321])),
        )
    }
}
impl AntiWedge<VersorOdd> for FlatPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234]) * self.group0() * Simd32x4::from(-1.0),
            // e5
            (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]) + (self[e45] * other[e3215]),
        )
    }
}
impl std::ops::Div<AntiWedgeInfix> for Flector {
    type Output = AntiWedgeInfixPartial<Flector>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiCircleRotor> for Flector {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       11       16        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(other[e41] * self[e3215]) - (other[e31] * self[e4125]),
                -(other[e42] * self[e3215]) - (other[e12] * self[e4235]),
                -(other[e43] * self[e3215]) - (other[e23] * self[e4315]),
                (other[e42] * self[e4315]) + (other[e43] * self[e4125]),
            ]) + (self.group1().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e5
            -(other[e45] * self[e3215]) - (other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]),
        )
    }
}
impl AntiWedge<AntiDipoleInversion> for Flector {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        3        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       10       17        0
    //  no simd       24       32        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (other.group0().zxy() * self.group1().yzx()) - (other.group0().yzx() * self.group1().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                other[e423] * self[e3215],
                other[e431] * self[e3215],
                other[e412] * self[e3215],
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) - (other.group1().wwwx() * self.group1().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(self[e3215]) * other.group1().xyz().with_w(other[e4]))
                + (self.group1().zxyx() * other.group2().yzx().with_w(other[e1]))
                + Simd32x3::from(0.0)
                    .with_w((other[e2] * self[e4315]) + (other[e3] * self[e4125]) - (other[e431] * self[e25]) - (other[e412] * self[e35]) - (other[e321] * self[e45]))
                - (other.group2().zxy() * self.group1().yzx()).with_w(other[e423] * self[e15]),
        )
    }
}
impl AntiWedge<AntiDualNum> for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (self.group1().xyz() * other.group0().xx().with_z(other[e3215])).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(other[e3215] * self[e45]),
        )
    }
}
impl AntiWedge<AntiFlatPoint> for Flector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        3       14        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other[e321]) * self.group1().xyz().with_w(self[e45]) * Simd32x4::from(-1.0),
            // e15, e25, e35, e3215
            ((other.group0().yzx() * self.group1().zxy()) - (other.group0().zxy() * self.group1().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiFlector> for Flector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        1        4        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        6       16        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (self.group1().xyz() * other.group0().www() * Simd32x3::from(-1.0))
                .with_w((other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]) - (other[e321] * self[e45])),
            // e15, e25, e35, e3215
            ((other.group0().yzx() * self.group1().zxy()) - (other.group0().zxy() * self.group1().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiLine> for Flector {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        5        9        0
    fn anti_wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                other[e12] * self[e4315],
                other[e23] * self[e4125],
                other[e31] * self[e4235],
                -(other[e25] * self[e4315]) - (other[e35] * self[e4125]),
            ]) - (self.group1().zxyx() * other.group0().yzx().with_w(other[e15])),
        )
    }
}
impl AntiWedge<AntiMotor> for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        0        2        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        9       13        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (self.group1().xyz() * other.group1().www()).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(other[e25] * self[e4315]) - (other[e35] * self[e4125])) + (other.group0().zxy() * self.group1().yzx()).with_w(other[e3215] * self[e45])
                - (self.group1().zxyx() * other.group0().yzx().with_w(other[e15])),
        )
    }
}
impl AntiWedge<AntiPlane> for Flector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn anti_wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]))
    }
}
impl AntiWedge<AntiScalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[e12345]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group1(),
        )
    }
}
impl AntiWedge<Circle> for Flector {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       15        0
    //    simd3        1        3        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       17       28        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (other.group0().zxy() * self.group1().yzx()) - (other.group0().yzx() * self.group1().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                other[e423] * self[e3215],
                other[e431] * self[e3215],
                other[e412] * self[e3215],
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) - (other.group1().wwwx() * self.group1().xyzx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (other[e415] * self[e3215]) + (other[e315] * self[e4125]),
                (other[e425] * self[e3215]) + (other[e125] * self[e4235]),
                (other[e435] * self[e3215]) + (other[e235] * self[e4315]),
                -(other[e431] * self[e25]) - (other[e412] * self[e35]) - (other[e321] * self[e45]),
            ]) - (other.group2().zxy() * self.group1().yzx()).with_w(other[e423] * self[e15]),
        )
    }
}
impl AntiWedge<CircleRotor> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       10        0
    //    simd3        0        6        0
    //    simd4        6        2        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       27       36        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                other[e412] * self[e4315],
                other[e423] * self[e4125],
                other[e431] * self[e4235],
                -(other[e431] * self[e25]) - (other[e412] * self[e35]) - (other[e321] * self[e45]),
            ]) - (other.group0().yzx() * self.group1().zxy()).with_w(other[e423] * self[e15]),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125])) + (other.group0() * self.group1().www()).with_w(other[e12345] * self[e45])
                - (other.group1().wwwx() * self.group1().xyzx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(other[e12345]) * self.group0().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e3215]) * other.group1().xyz()).with_w(0.0)
                + (other.group2().yzx() * self.group1().zxy()).with_w(0.0)
                - (other.group2().zxy() * self.group1().yzx()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group1(),
        )
    }
}
impl AntiWedge<Dipole> for Flector {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       11       16        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(other[e41] * self[e3215]) - (other[e31] * self[e4125]),
                -(other[e42] * self[e3215]) - (other[e12] * self[e4235]),
                -(other[e43] * self[e3215]) - (other[e23] * self[e4315]),
                (other[e42] * self[e4315]) + (other[e43] * self[e4125]),
            ]) + (self.group1().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e5
            -(other[e45] * self[e3215]) - (other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]),
        )
    }
}
impl AntiWedge<DipoleInversion> for Flector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd3        1        6        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       11       20        0
    //  no simd       31       44        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e1234]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            ((other.group3().yzx() * self.group1().zxy()) - (other.group3().zxy() * self.group1().yzx())).with_w(other[e1234] * self[e3215] * -1.0),
            // e235, e315, e125, e4
            (self.group1().xyzx() * other.group3().www().with_w(other[e41])) + Simd32x3::from(0.0).with_w((other[e42] * self[e4315]) + (other[e43] * self[e4125]))
                - (other.group3().xyz() * self.group1().www()).with_w(other[e1234] * self[e45]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w((other[e4315] * self[e25]) + (other[e4125] * self[e35]) + (other[e3215] * self[e45]) - (other[e35] * self[e4125]))
                + (other.group1().zxy() * self.group1().yzx()).with_w(other[e4235] * self[e15])
                - (Simd32x4::from(self[e3215]) * other.group0().with_w(other[e45]))
                - (other.group2().wwwy() * self.group0().xyz().with_w(self[e4315]))
                - (self.group1().zxyx() * other.group1().yzx().with_w(other[e15])),
        )
    }
}
impl AntiWedge<DualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[e12345]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group1(),
        )
    }
}
impl AntiWedge<FlatPoint> for Flector {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            -(other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]) - (other[e45] * self[e3215]),
            0.0,
        ]))
    }
}
impl AntiWedge<Flector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        8       10        0
    //  no simd       16       20        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((other.group1().yzx() * self.group1().zxy()) - (other.group1().zxy() * self.group1().yzx())).with_w(0.0),
            // e235, e315, e125, e5
            (other.group1().wwwx() * self.group1().xyz().with_w(self[e15]))
                + Simd32x3::from(0.0).with_w(
                    (other[e4315] * self[e25]) + (other[e4125] * self[e35]) + (other[e3215] * self[e45])
                        - (other[e25] * self[e4315])
                        - (other[e35] * self[e4125])
                        - (other[e45] * self[e3215]),
                )
                - (self.group1().wwwx() * other.group1().xyz().with_w(other[e15])),
        )
    }
}
impl AntiWedge<Line> for Flector {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e4125] * other[e315]) + (self[e3215] * other[e415]),
                (self[e4235] * other[e125]) + (self[e3215] * other[e425]),
                (self[e4315] * other[e235]) + (self[e3215] * other[e435]),
                -(self[e4315] * other[e425]) - (self[e4125] * other[e435]),
            ]) - (self.group1().yzxx() * other.group1().zxy().with_w(other[e415])),
        )
    }
}
impl AntiWedge<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       20        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e4125] * other[e315]) + (self[e3215] * other[e415]),
                (self[e4235] * other[e125]) + (self[e3215] * other[e425]),
                (self[e4315] * other[e235]) + (self[e3215] * other[e435]),
                -(self[e4315] * other[e425]) - (self[e4125] * other[e435]),
            ]) + (Simd32x4::from(other[e12345]) * self.group0())
                - (self.group1().yzxx() * other.group1().zxy().with_w(other[e415])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group1(),
        )
    }
}
impl AntiWedge<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       36        0
    //    simd3        4       12        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       30       51        0
    //  no simd       50       84        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e4235] * other[e1]) + (self[e4315] * other[e2]) + (self[e4125] * other[e3]) + (self[e3215] * other[e4])
                    - (self[e15] * other[e423])
                    - (self[e25] * other[e431])
                    - (self[e35] * other[e412])
                    - (self[e45] * other[e321]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                -(self[e15] * other[e1234]) - (self[e4125] * other[e31]),
                -(self[e25] * other[e1234]) - (self[e4235] * other[e12]),
                -(self[e35] * other[e1234]) - (self[e4315] * other[e23]),
                (self[e4315] * other[e42]) + (self[e4125] * other[e43]),
            ]) + (self.group1().yzxx() * other.group5().zxy().with_w(other[e41]))
                - (other.group4() * self.group1().www()).with_w(self[e45] * other[e1234]),
            // e5
            (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]) + (self[e45] * other[e3215])
                - (self[e4235] * other[e15])
                - (self[e4315] * other[e25])
                - (self[e4125] * other[e35])
                - (self[e3215] * other[e45]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e4125] * other[e315]) + (self[e3215] * other[e415]),
                (self[e4235] * other[e125]) + (self[e3215] * other[e425]),
                (self[e4315] * other[e235]) + (self[e3215] * other[e435]),
                -(self[e4315] * other[e425]) - (self[e4125] * other[e435]),
            ]) + (Simd32x4::from(other[e12345]) * self.group0())
                - (other.group8().zxy() * self.group1().yzx()).with_w(self[e4235] * other[e415]),
            // e41, e42, e43
            (other.group7().zxy() * self.group1().yzx()) - (other.group7().yzx() * self.group1().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e3215]) * other.group7()) - (Simd32x3::from(other[e321]) * self.group1().xyz()),
            // e415, e425, e435, e321
            ((self.group1().zxy() * other.group9().yzx()) - (self.group1().yzx() * other.group9().zxy())).with_w(self[e3215] * other[e1234] * -1.0),
            // e423, e431, e412
            Simd32x3::from(other[e1234]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group1().xyz()) - (Simd32x3::from(self[e3215]) * other.group9().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group1(),
            // e1234
            0.0,
        )
    }
}
impl AntiWedge<Plane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        9       19        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((self.group1().zxy() * other.group0().yzx()) - (self.group1().yzx() * other.group0().zxy())).with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                self[e3215] * other[e4235] * -1.0,
                self[e3215] * other[e4315] * -1.0,
                self[e3215] * other[e4125] * -1.0,
                (self[e25] * other[e4315]) + (self[e35] * other[e4125]) + (self[e45] * other[e3215]),
            ]) + (other.group0().wwwx() * self.group1().xyz().with_w(self[e15])),
        )
    }
}
impl AntiWedge<RoundPoint> for Flector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e4235] * other[e1]) + (self[e4315] * other[e2]) + (self[e4125] * other[e3]) + (self[e3215] * other[e4]),
        )
    }
}
impl AntiWedge<Sphere> for Flector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd3        2        8        0
    // Totals...
    // yes simd        5       16        0
    //  no simd        9       32        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e1234]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            ((self.group1().zxy() * other.group0().yzx()) - (self.group1().yzx() * other.group0().zxy())).with_w(self[e3215] * other[e1234] * -1.0),
            // e235, e315, e125, e4
            ((Simd32x3::from(other[e3215]) * self.group1().xyz()) - (Simd32x3::from(self[e3215]) * other.group0().xyz())).with_w(self[e45] * other[e1234] * -1.0),
            // e1, e2, e3, e5
            (Simd32x3::from(other[e1234]) * self.group0().xyz() * Simd32x3::from(-1.0))
                .with_w((self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]) + (self[e45] * other[e3215])),
        )
    }
}
impl AntiWedge<VersorEven> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd3        0        4        0
    //    simd4        7        5        0
    // Totals...
    // yes simd       13       17        0
    //  no simd       34       40        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group1().yzxx() * other.group0().zxy().with_w(other[e1]))
                + Simd32x3::from(0.0).with_w(
                    (self[e4315] * other[e2]) + (self[e4125] * other[e3]) + (self[e3215] * other[e4])
                        - (self[e25] * other[e431])
                        - (self[e35] * other[e412])
                        - (self[e45] * other[e321]),
                )
                - (other.group0().yzxx() * self.group1().zxy().with_w(self[e15])),
            // e23, e31, e12, e45
            (other.group0() * self.group1().www().with_w(self[e45])) + Simd32x3::from(0.0).with_w(-(self[e4315] * other[e425]) - (self[e4125] * other[e435]))
                - (self.group1().xyzx() * other.group1().wwwx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(self[e3215]) * other.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(other[e12345]) * self.group0().xyz()).with_w(0.0)
                + (self.group1().zxy() * other.group2().yzx()).with_w(0.0)
                - (self.group1().yzx() * other.group2().zxy()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group1(),
        )
    }
}
impl AntiWedge<VersorOdd> for Flector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd3        1        6        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       11       20        0
    //  no simd       31       44        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e1234]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            ((self.group1().zxy() * other.group3().yzx()) - (self.group1().yzx() * other.group3().zxy())).with_w(self[e3215] * other[e1234] * -1.0),
            // e235, e315, e125, e4
            (self.group1().xyzx() * other.group3().www().with_w(other[e41])) + Simd32x3::from(0.0).with_w((self[e4315] * other[e42]) + (self[e4125] * other[e43]))
                - (other.group3().xyz() * self.group1().www()).with_w(self[e45] * other[e1234]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w((self[e25] * other[e4315]) + (self[e35] * other[e4125]) + (self[e45] * other[e3215]) - (self[e3215] * other[e45]))
                + (self.group1().yzx() * other.group1().zxy()).with_w(self[e15] * other[e4235])
                - (self.group1().zxyy() * other.group1().yzx().with_w(other[e25]))
                - (self.group1().wwwz() * other.group0().xyz().with_w(other[e35]))
                - (other.group2().wwwx() * self.group0().xyz().with_w(self[e4235])),
        )
    }
}
impl std::ops::Div<AntiWedgeInfix> for Line {
    type Output = AntiWedgeInfixPartial<Line>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiCircleRotor> for Line {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(other[e41] * self[e235]) - (other[e42] * self[e315]) - (other[e43] * self[e125]) - (other[e23] * self[e415]) - (other[e31] * self[e425]) - (other[e12] * self[e435]),
        )
    }
}
impl AntiWedge<AntiDipoleInversion> for Line {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       13       18        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e412] * self[e315]) + (other[e321] * self[e415]),
                (other[e423] * self[e125]) + (other[e321] * self[e425]),
                (other[e431] * self[e235]) + (other[e321] * self[e435]),
                -(other[e431] * self[e425]) - (other[e412] * self[e435]),
            ]) - (other.group0().yzx() * self.group1().zxy()).with_w(other[e423] * self[e415]),
            // e5
            -(other[e415] * self[e235])
                - (other[e425] * self[e315])
                - (other[e435] * self[e125])
                - (other[e235] * self[e415])
                - (other[e315] * self[e425])
                - (other[e125] * self[e435]),
        )
    }
}
impl AntiWedge<AntiDualNum> for Line {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ (self.group0() * other.group0().xx().with_z(other[e3215])).with_w(0.0))
    }
}
impl AntiWedge<AntiFlatPoint> for Line {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x3::from(other[e321]) * self.group0()).with_w(-(other[e235] * self[e415]) - (other[e315] * self[e425]) - (other[e125] * self[e435])),
        )
    }
}
impl AntiWedge<AntiFlector> for Line {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x3::from(other[e321]) * self.group0()).with_w(-(other[e235] * self[e415]) - (other[e315] * self[e425]) - (other[e125] * self[e435])),
        )
    }
}
impl AntiWedge<AntiLine> for Line {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn anti_wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -(other[e23] * self[e415]) - (other[e31] * self[e425]) - (other[e12] * self[e435]))
    }
}
impl AntiWedge<AntiMotor> for Line {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(other[e23] * self[e415]) - (other[e31] * self[e425]) - (other[e12] * self[e435])),
            // e15, e25, e35, e3215
            (self.group0() * other.group1().www()).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiScalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(other[e12345]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other[e12345]) * self.group1(),
        )
    }
}
impl AntiWedge<Circle> for Line {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       13       18        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e412] * self[e315]) + (other[e321] * self[e415]),
                (other[e423] * self[e125]) + (other[e321] * self[e425]),
                (other[e431] * self[e235]) + (other[e321] * self[e435]),
                -(other[e431] * self[e425]) - (other[e412] * self[e435]),
            ]) - (other.group0().yzx() * self.group1().zxy()).with_w(other[e423] * self[e415]),
            // e5
            -(other[e415] * self[e235])
                - (other[e425] * self[e315])
                - (other[e435] * self[e125])
                - (other[e235] * self[e415])
                - (other[e315] * self[e425])
                - (other[e125] * self[e435]),
        )
    }
}
impl AntiWedge<CircleRotor> for Line {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       24        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (self.group0() * other.group2().www()).with_w(0.0),
            // e235, e315, e125, e4
            (Simd32x3::from(other[e12345]) * self.group1()).with_w(-(other[e423] * self[e415]) - (other[e431] * self[e425]) - (other[e412] * self[e435])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e412] * self[e315]) + (other[e321] * self[e415]),
                (other[e423] * self[e125]) + (other[e321] * self[e425]),
                (other[e431] * self[e235]) + (other[e321] * self[e435]),
                -(other[e415] * self[e235]) - (other[e425] * self[e315]) - (other[e435] * self[e125]) - (other[e315] * self[e425]) - (other[e125] * self[e435]),
            ]) - (other.group0().yzx() * self.group1().zxy()).with_w(other[e235] * self[e415]),
        )
    }
}
impl AntiWedge<Dipole> for Line {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            -(other[e41] * self[e235]) - (other[e42] * self[e315]) - (other[e43] * self[e125]) - (other[e23] * self[e415]) - (other[e31] * self[e425]) - (other[e12] * self[e435]),
        )
    }
}
impl AntiWedge<DipoleInversion> for Line {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       24        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group0(),
            // e23, e31, e12, e45
            (Simd32x3::from(other[e1234]) * self.group1()).with_w(-(other[e4235] * self[e415]) - (other[e4315] * self[e425]) - (other[e4125] * self[e435])),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (other[e4125] * self[e315]) + (other[e3215] * self[e415]),
                (other[e4235] * self[e125]) + (other[e3215] * self[e425]),
                (other[e4315] * self[e235]) + (other[e3215] * self[e435]),
                -(other[e42] * self[e315]) - (other[e43] * self[e125]) - (other[e23] * self[e415]) - (other[e31] * self[e425]) - (other[e12] * self[e435]),
            ]) - (self.group1().zxy() * other.group3().yzx()).with_w(other[e41] * self[e235]),
        )
    }
}
impl AntiWedge<DualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(other[e12345]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other[e12345]) * self.group1(),
        )
    }
}
impl AntiWedge<Flector> for Line {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (other[e4125] * self[e315]) + (other[e3215] * self[e415]),
                (other[e4235] * self[e125]) + (other[e3215] * self[e425]),
                (other[e4315] * self[e235]) + (other[e3215] * self[e435]),
                -(other[e4315] * self[e425]) - (other[e4125] * self[e435]),
            ]) - (other.group1().yzxx() * self.group1().zxy().with_w(self[e415])),
        )
    }
}
impl AntiWedge<Line> for Line {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            -(other[e415] * self[e235])
                - (other[e425] * self[e315])
                - (other[e435] * self[e125])
                - (other[e235] * self[e415])
                - (other[e315] * self[e425])
                - (other[e125] * self[e435]),
            0.0,
        ]))
    }
}
impl AntiWedge<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (self.group0() * other.group0().www()).with_w(0.0),
            // e235, e315, e125, e5
            (Simd32x3::from(other[e12345]) * self.group1()).with_w(
                -(self[e415] * other[e235])
                    - (self[e425] * other[e315])
                    - (self[e435] * other[e125])
                    - (self[e235] * other[e415])
                    - (self[e315] * other[e425])
                    - (self[e125] * other[e435]),
            ),
        )
    }
}
impl AntiWedge<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       29        0
    //    simd3        0        5        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       20       35        0
    //  no simd       26       48        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(self[e415] * other[e23])
                    - (self[e425] * other[e31])
                    - (self[e435] * other[e12])
                    - (self[e235] * other[e41])
                    - (self[e315] * other[e42])
                    - (self[e125] * other[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e415] * other[e321]) + (self[e315] * other[e412]),
                (self[e425] * other[e321]) + (self[e125] * other[e423]),
                (self[e435] * other[e321]) + (self[e235] * other[e431]),
                -(self[e425] * other[e431]) - (self[e435] * other[e412]),
            ]) - (self.group1().zxy() * other.group7().yzx()).with_w(self[e415] * other[e423]),
            // e5
            -(self[e415] * other[e235])
                - (self[e425] * other[e315])
                - (self[e435] * other[e125])
                - (self[e235] * other[e415])
                - (self[e315] * other[e425])
                - (self[e125] * other[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e415] * other[e3215]) + (self[e315] * other[e4125]),
                (self[e425] * other[e3215]) + (self[e125] * other[e4235]),
                (self[e435] * other[e3215]) + (self[e235] * other[e4315]),
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) - (other.group9().yzxx() * self.group1().zxy().with_w(self[e415])),
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[e1234]) * self.group1(),
            // e415, e425, e435, e321
            (self.group0() * other.group0().yy().with_z(other[e12345])).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(other[e12345]) * self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl AntiWedge<Plane> for Line {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e415] * other[e3215]) + (self[e315] * other[e4125]),
                (self[e425] * other[e3215]) + (self[e125] * other[e4235]),
                (self[e435] * other[e3215]) + (self[e235] * other[e4315]),
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) - (other.group0().yzxx() * self.group1().zxy().with_w(self[e415])),
        )
    }
}
impl AntiWedge<Sphere> for Line {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        2        5        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        8       18        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group0(),
            // e23, e31, e12, e45
            (Simd32x3::from(other[e1234]) * self.group1()).with_w(-(self[e415] * other[e4235]) - (self[e425] * other[e4315]) - (self[e435] * other[e4125])),
            // e15, e25, e35
            (Simd32x3::from(other[e3215]) * self.group0()) + (self.group1().yzx() * other.group0().zxy()) - (self.group1().zxy() * other.group0().yzx()),
        )
    }
}
impl AntiWedge<VersorEven> for Line {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       24        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (self.group0() * other.group0().www()).with_w(0.0),
            // e235, e315, e125, e4
            (Simd32x3::from(other[e12345]) * self.group1()).with_w(-(self[e415] * other[e423]) - (self[e425] * other[e431]) - (self[e435] * other[e412])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e415] * other[e321]) + (self[e315] * other[e412]),
                (self[e425] * other[e321]) + (self[e125] * other[e423]),
                (self[e435] * other[e321]) + (self[e235] * other[e431]),
                -(self[e425] * other[e315]) - (self[e435] * other[e125]) - (self[e235] * other[e415]) - (self[e315] * other[e425]) - (self[e125] * other[e435]),
            ]) - (self.group1().zxy() * other.group0().yzx()).with_w(self[e415] * other[e235]),
        )
    }
}
impl AntiWedge<VersorOdd> for Line {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       24        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group0(),
            // e23, e31, e12, e45
            (Simd32x3::from(other[e1234]) * self.group1()).with_w(-(self[e415] * other[e4235]) - (self[e425] * other[e4315]) - (self[e435] * other[e4125])),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (self[e415] * other[e3215]) + (self[e315] * other[e4125]),
                (self[e425] * other[e3215]) + (self[e125] * other[e4235]),
                (self[e435] * other[e3215]) + (self[e235] * other[e4315]),
                -(self[e425] * other[e31]) - (self[e435] * other[e12]) - (self[e235] * other[e41]) - (self[e315] * other[e42]) - (self[e125] * other[e43]),
            ]) - (self.group1().zxy() * other.group3().yzx()).with_w(self[e415] * other[e23]),
        )
    }
}
impl std::ops::Div<AntiWedgeInfix> for Motor {
    type Output = AntiWedgeInfixPartial<Motor>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiCircleRotor> for Motor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       10        0
    //  no simd        6       17        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * other.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(self[e12345]) * other.group2().xyz()).with_w(
                (other[scalar] * self[e12345])
                    - (other[e41] * self[e235])
                    - (other[e42] * self[e315])
                    - (other[e43] * self[e125])
                    - (other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435]),
            ),
        )
    }
}
impl AntiWedge<AntiDipoleInversion> for Motor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       16        0
    //    simd3        0        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       12       21        0
    //  no simd       18       33        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(self[e12345]) * other.group2().xyz())
                .with_w((other[e4] * self[e12345]) - (other[e423] * self[e415]) - (other[e431] * self[e425]) - (other[e412] * self[e435])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e321] * self[e415]) + (other[e1] * self[e12345]),
                (other[e321] * self[e425]) + (other[e2] * self[e12345]),
                (other[e321] * self[e435]) + (other[e3] * self[e12345]),
                -(other[e425] * self[e315]) - (other[e435] * self[e125]) - (other[e235] * self[e415]) - (other[e315] * self[e425]) - (other[e125] * self[e435]),
            ]) + (other.group0().zxy() * self.group1().yzx()).with_w(other[e5] * self[e12345])
                - (self.group1().zxyx() * other.group0().yzx().with_w(other[e415])),
        )
    }
}
impl AntiWedge<AntiDualNum> for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(other[scalar] * self[e12345]),
            // e15, e25, e35, e3215
            Simd32x4::from(other[e3215]) * self.group0(),
        )
    }
}
impl AntiWedge<AntiFlatPoint> for Motor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2       10        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * other.group0(),
            // e1, e2, e3, e5
            (self.group0().xyz() * other.group0().www()).with_w(-(other[e235] * self[e415]) - (other[e315] * self[e425]) - (other[e125] * self[e435])),
        )
    }
}
impl AntiWedge<AntiFlector> for Motor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        6       14        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * other.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([
                other[e1] * self[e12345],
                other[e2] * self[e12345],
                other[e3] * self[e12345],
                -(other[e235] * self[e415]) - (other[e315] * self[e425]) - (other[e125] * self[e435]),
            ]) + (self.group0() * other.group0().www().with_w(other[e5])),
        )
    }
}
impl AntiWedge<AntiLine> for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn anti_wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e12345]) * other.group0()).with_w(-(other[e23] * self[e415]) - (other[e31] * self[e425]) - (other[e12] * self[e435])),
            // e15, e25, e35, e3215
            (other.group1() * self.group0().www()).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiMotor> for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        1        3        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        6       14        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e12345]) * other.group0().xyz())
                .with_w((other[scalar] * self[e12345]) - (other[e23] * self[e415]) - (other[e31] * self[e425]) - (other[e12] * self[e435])),
            // e15, e25, e35, e3215
            ((Simd32x3::from(other[e3215]) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * other.group1().xyz())).with_w(other[e3215] * self[e12345]),
        )
    }
}
impl AntiWedge<AntiPlane> for Motor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(other[e12345]) * self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(other[e12345]) * self.group1(),
        )
    }
}
impl AntiWedge<Circle> for Motor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       13       28        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(self[e12345]) * other.group2()).with_w(-(other[e423] * self[e415]) - (other[e431] * self[e425]) - (other[e412] * self[e435])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e412] * self[e315]) + (other[e321] * self[e415]),
                (other[e423] * self[e125]) + (other[e321] * self[e425]),
                (other[e431] * self[e235]) + (other[e321] * self[e435]),
                -(other[e415] * self[e235]) - (other[e425] * self[e315]) - (other[e435] * self[e125]) - (other[e315] * self[e425]) - (other[e125] * self[e435]),
            ]) - (other.group0().yzx() * self.group1().zxy()).with_w(other[e235] * self[e415]),
        )
    }
}
impl AntiWedge<CircleRotor> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       19        0
    //    simd3        1        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       20       36        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self[e12345]) * other.group0().with_w(other[e12345]),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e12345]) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * other.group1().xyz())).with_w(other[e321] * self[e12345]),
            // e235, e315, e125, e5
            Simd32x4::from([
                other[e12345] * self[e235],
                other[e12345] * self[e315],
                other[e12345] * self[e125],
                -(other[e415] * self[e235])
                    - (other[e425] * self[e315])
                    - (other[e435] * self[e125])
                    - (other[e235] * self[e415])
                    - (other[e315] * self[e425])
                    - (other[e125] * self[e435]),
            ]) + (other.group2() * self.group0().www().with_w(self[e5])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e412] * self[e315]) + (other[e321] * self[e415]),
                (other[e423] * self[e125]) + (other[e321] * self[e425]),
                (other[e431] * self[e235]) + (other[e321] * self[e435]),
                -(other[e431] * self[e425]) - (other[e412] * self[e435]),
            ]) - (other.group0().yzx() * self.group1().zxy()).with_w(other[e423] * self[e415]),
        )
    }
}
impl AntiWedge<Dipole> for Motor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       16        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * other.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(self[e12345]) * other.group2()).with_w(
                -(other[e41] * self[e235])
                    - (other[e42] * self[e315])
                    - (other[e43] * self[e125])
                    - (other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435]),
            ),
        )
    }
}
impl AntiWedge<DipoleInversion> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       17        0
    //    simd3        3        5        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       24       40        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                other[e1234] * self[e415],
                other[e1234] * self[e425],
                other[e1234] * self[e435],
                -(other[e41] * self[e235])
                    - (other[e42] * self[e315])
                    - (other[e43] * self[e125])
                    - (other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435]),
            ]) + (other.group0() * self.group0().www()).with_w(other[e1234] * self[e5]),
            // e23, e31, e12, e45
            Simd32x4::from([
                other[e1234] * self[e235],
                other[e1234] * self[e315],
                other[e1234] * self[e125],
                -(other[e4235] * self[e415]) - (other[e4315] * self[e425]) - (other[e4125] * self[e435]),
            ]) + (Simd32x4::from(self[e12345]) * other.group1()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e3215]) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * other.group2().xyz()) + (other.group3().zxy() * self.group1().yzx())
                - (other.group3().yzx() * self.group1().zxy()))
            .with_w(other[e1234] * self[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group3(),
        )
    }
}
impl AntiWedge<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1       10        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(other[e12345]) * self.group0(),
            // e235, e315, e125, e5
            other.group0().yy().with_zw(other[e12345], (other[e5] * self[e12345]) + (other[e12345] * self[e5])) * self.group1().xyz().with_w(1.0),
        )
    }
}
impl AntiWedge<FlatPoint> for Motor {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       20        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (other[e4125] * self[e315]) + (other[e3215] * self[e415]),
                (other[e4235] * self[e125]) + (other[e3215] * self[e425]),
                (other[e4315] * self[e235]) + (other[e3215] * self[e435]),
                -(other[e4315] * self[e425]) - (other[e4125] * self[e435]),
            ]) + (Simd32x4::from(self[e12345]) * other.group0())
                - (other.group1().yzxx() * self.group1().zxy().with_w(self[e415])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group1(),
        )
    }
}
impl AntiWedge<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (other.group0() * self.group0().www()).with_w(0.0),
            // e235, e315, e125, e5
            (Simd32x3::from(self[e12345]) * other.group1()).with_w(
                -(other[e415] * self[e235])
                    - (other[e425] * self[e315])
                    - (other[e435] * self[e125])
                    - (other[e235] * self[e415])
                    - (other[e315] * self[e425])
                    - (other[e125] * self[e435]),
            ),
        )
    }
}
impl AntiWedge<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        8       11        0
    //  no simd       16       21        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((Simd32x3::from(other[e12345]) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * other.group0().xyz())).with_w(other[e12345] * self[e12345]),
            // e235, e315, e125, e5
            (Simd32x4::from(other[e12345]) * self.group1())
                + (Simd32x4::from(self[e12345]) * other.group1())
                + Simd32x3::from(0.0).with_w(
                    -(other[e415] * self[e235])
                        - (other[e425] * self[e315])
                        - (other[e435] * self[e125])
                        - (other[e235] * self[e415])
                        - (other[e315] * self[e425])
                        - (other[e125] * self[e435]),
                ),
        )
    }
}
impl AntiWedge<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       37        0
    //    simd3        4       11        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       30       51        0
    //  no simd       50       82        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e12345] * other[scalar]) + (self[e5] * other[e1234])
                    - (self[e415] * other[e23])
                    - (self[e425] * other[e31])
                    - (self[e435] * other[e12])
                    - (self[e235] * other[e41])
                    - (self[e315] * other[e42])
                    - (self[e125] * other[e43]),
                self[e12345] * other[e12345],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e415] * other[e321]) + (self[e12345] * other[e1]),
                (self[e425] * other[e321]) + (self[e12345] * other[e2]),
                (self[e435] * other[e321]) + (self[e12345] * other[e3]),
                -(self[e425] * other[e431]) - (self[e435] * other[e412]),
            ]) + (other.group7().zxy() * self.group1().yzx()).with_w(self[e12345] * other[e4])
                - (other.group7().yzx() * self.group1().zxy()).with_w(self[e415] * other[e423]),
            // e5
            (self[e12345] * other[e5]) + (self[e5] * other[e12345])
                - (self[e415] * other[e235])
                - (self[e425] * other[e315])
                - (self[e435] * other[e125])
                - (self[e235] * other[e415])
                - (self[e315] * other[e425])
                - (self[e125] * other[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e12345] * other[e15]) + (self[e315] * other[e4125]),
                (self[e12345] * other[e25]) + (self[e125] * other[e4235]),
                (self[e12345] * other[e35]) + (self[e235] * other[e4315]),
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) + (self.group0() * other.group9().www().with_w(other[e45]))
                - (other.group9().yzxx() * self.group1().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(self[e12345]) * other.group4()) + (Simd32x3::from(other[e1234]) * self.group0().xyz()),
            // e23, e31, e12
            (Simd32x3::from(self[e12345]) * other.group5()) + (Simd32x3::from(other[e1234]) * self.group1().xyz()),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e12345]) * other.group6().xyz()) + (Simd32x3::from(other[e12345]) * self.group0().xyz())).with_w(self[e12345] * other[e321]),
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * other.group7(),
            // e235, e315, e125
            (Simd32x3::from(self[e12345]) * other.group8()) + (Simd32x3::from(other[e12345]) * self.group1().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group9(),
            // e1234
            self[e12345] * other[e1234],
        )
    }
}
impl AntiWedge<Plane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e415] * other[e3215]) + (self[e315] * other[e4125]),
                (self[e425] * other[e3215]) + (self[e125] * other[e4235]),
                (self[e435] * other[e3215]) + (self[e235] * other[e4315]),
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) - (other.group0().yzxx() * self.group1().zxy().with_w(self[e415])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group0(),
        )
    }
}
impl AntiWedge<RoundPoint> for Motor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e12345]) * other.group0(), /* e5 */ self[e12345] * other[e5])
    }
}
impl AntiWedge<Scalar> for Motor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e12345] * other[scalar])
    }
}
impl AntiWedge<Sphere> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        2        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        8       24        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other[e1234]) * self.group0().xyz().with_w(self[e5]),
            // e23, e31, e12, e45
            (Simd32x3::from(other[e1234]) * self.group1().xyz()).with_w(-(self[e415] * other[e4235]) - (self[e425] * other[e4315]) - (self[e435] * other[e4125])),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e3215]) * self.group0().xyz()) + (self.group1().yzx() * other.group0().zxy()) - (self.group1().zxy() * other.group0().yzx()))
                .with_w(self[e12345] * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group0(),
        )
    }
}
impl AntiWedge<VersorEven> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        1        2        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       14       22        0
    //  no simd       28       41        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self[e12345]) * other.group0(),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e12345]) * other.group1().xyz()) + (Simd32x3::from(other[e12345]) * self.group0().xyz())).with_w(self[e12345] * other[e321]),
            // e235, e315, e125, e5
            (Simd32x4::from(self[e12345]) * other.group2())
                + (Simd32x4::from(other[e12345]) * self.group1())
                + Simd32x3::from(0.0).with_w(
                    -(self[e415] * other[e235])
                        - (self[e425] * other[e315])
                        - (self[e435] * other[e125])
                        - (self[e235] * other[e415])
                        - (self[e315] * other[e425])
                        - (self[e125] * other[e435]),
                ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e12345] * other[e1]) + (self[e315] * other[e412]),
                (self[e12345] * other[e2]) + (self[e125] * other[e423]),
                (self[e12345] * other[e3]) + (self[e235] * other[e431]),
                -(self[e425] * other[e431]) - (self[e435] * other[e412]),
            ]) + (self.group0() * other.group1().www().with_w(other[e4]))
                - (other.group0().yzxx() * self.group1().zxy().with_w(self[e415])),
        )
    }
}
impl AntiWedge<VersorOdd> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd3        3        5        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       28       41        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() * other.group2().www().with_w(other[scalar]))
                + Simd32x3::from(0.0).with_w(
                    -(self[e415] * other[e23])
                        - (self[e425] * other[e31])
                        - (self[e435] * other[e12])
                        - (self[e235] * other[e41])
                        - (self[e315] * other[e42])
                        - (self[e125] * other[e43]),
                )
                + (other.group0().xyz() * self.group0().www()).with_w(self[e5] * other[e1234]),
            // e23, e31, e12, e45
            Simd32x4::from([
                self[e235] * other[e1234],
                self[e315] * other[e1234],
                self[e125] * other[e1234],
                -(self[e415] * other[e4235]) - (self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) + (Simd32x4::from(self[e12345]) * other.group1()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e12345]) * other.group2().xyz()) + (Simd32x3::from(other[e3215]) * self.group0().xyz()) + (self.group1().yzx() * other.group3().zxy())
                - (self.group1().zxy() * other.group3().yzx()))
            .with_w(self[e12345] * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group3(),
        )
    }
}
impl std::ops::Div<AntiWedgeInfix> for MultiVector {
    type Output = AntiWedgeInfixPartial<MultiVector>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       25       41        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (other[scalar] * self[e12345])
                    - (other[e41] * self[e235])
                    - (other[e42] * self[e315])
                    - (other[e43] * self[e125])
                    - (other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435])
                    - (other[e45] * self[e321])
                    - (other[e15] * self[e423])
                    - (other[e25] * self[e431])
                    - (other[e35] * self[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                -(other[e41] * self[e3215]) - (other[e31] * self[e4125]),
                -(other[e42] * self[e3215]) - (other[e12] * self[e4235]),
                -(other[e43] * self[e3215]) - (other[e23] * self[e4315]),
                (other[e43] * self[e4125]) + (other[e45] * self[e1234]),
            ]) + (self.group9().yzxx() * other.group1().zxy().with_w(other[e41]))
                + (Simd32x3::from(self[e1234]) * other.group2().xyz()).with_w(other[e42] * self[e4315]),
            // e5
            -(other[e45] * self[e3215]) - (other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]),
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * other.group2().xyz().with_w(other[e45]),
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[e12345]) * other.group1().xyz(),
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
impl AntiWedge<AntiDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       48        0
    //    simd3        4       10        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       44       61        0
    //  no simd       64       90        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (other[e4] * self[e3215]) + (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e5] * self[e1234])
                    - (other[e423] * self[e15])
                    - (other[e431] * self[e25])
                    - (other[e412] * self[e35])
                    - (other[e415] * self[e23])
                    - (other[e425] * self[e31])
                    - (other[e435] * self[e12])
                    - (other[e321] * self[e45])
                    - (other[e235] * self[e41])
                    - (other[e315] * self[e42])
                    - (other[e125] * self[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e412] * self[e315]) + (other[e415] * self[e321]) + (other[e321] * self[e415]) + (other[e315] * self[e412]),
                (other[e423] * self[e125]) + (other[e425] * self[e321]) + (other[e321] * self[e425]) + (other[e125] * self[e423]),
                (other[e431] * self[e235]) + (other[e435] * self[e321]) + (other[e321] * self[e435]) + (other[e235] * self[e431]),
                -(other[e412] * self[e435]) - (other[e415] * self[e423]) - (other[e425] * self[e431]) - (other[e435] * self[e412]),
            ]) + (Simd32x4::from(self[e12345]) * other.group3().xyz().with_w(other[e4]))
                - (other.group0().yzx() * self.group8().zxy()).with_w(other[e423] * self[e415])
                - (self.group7().yzx() * other.group2().zxy()).with_w(other[e431] * self[e425]),
            // e5
            (other[e5] * self[e12345])
                - (other[e415] * self[e235])
                - (other[e425] * self[e315])
                - (other[e435] * self[e125])
                - (other[e235] * self[e415])
                - (other[e315] * self[e425])
                - (other[e125] * self[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (other[e415] * self[e3215]) + (other[e315] * self[e4125]),
                (other[e425] * self[e3215]) + (other[e125] * self[e4235]),
                (other[e435] * self[e3215]) + (other[e235] * self[e4315]),
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) - (self.group9().yzxx() * other.group2().zxy().with_w(other[e415])),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group1().xyz()) + (other.group0().zxy() * self.group9().yzx()) - (other.group0().yzx() * self.group9().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e3215]) * other.group0()) + (Simd32x3::from(self[e1234]) * other.group2().xyz()) - (Simd32x3::from(other[e321]) * self.group9().xyz()),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1(),
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * other.group0(),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * other.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl AntiWedge<AntiDualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        1        9        0
    //  no simd        1       17        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other[e3215] * self[e4]) + (other[scalar] * self[e12345]), 0.0]),
            // e1, e2, e3, e4
            (self.group4() * other.group0().xx().with_z(other[e3215])).with_w(0.0),
            // e5
            other[e3215] * self[e45],
            // e15, e25, e35, e45
            (self.group6().xyz() * other.group0().xx().with_z(other[e3215])).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other[e3215]) * self.group7(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e3215] * self[e1234]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(other[e3215]) * self.group9().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(other[e3215] * self[e12345]),
            // e1234
            0.0,
        )
    }
}
impl AntiWedge<AntiFlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        8        0
    //    simd3        2        8        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        9       16        0
    //  no simd       19       32        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([-(other[e235] * self[e41]) - (other[e315] * self[e42]) - (other[e125] * self[e43]) - (other[e321] * self[e45]), 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(other[e321]) * self.group6().xyz()).with_w(0.0) + (self.group7().zxy() * other.group0().yzx()).with_w(0.0)
                - (self.group7().yzx() * other.group0().zxy()).with_w(0.0),
            // e5
            -(other[e235] * self[e415]) - (other[e315] * self[e425]) - (other[e125] * self[e435]),
            // e15, e25, e35, e45
            ((other.group0().yzx() * self.group9().zxy()) - (other.group0().zxy() * self.group9().yzx())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from(self[e1234]) * other.group0().xyz()) - (Simd32x3::from(other[e321]) * self.group9().xyz()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321] * self[e12345]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * other.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl AntiWedge<AntiFlector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       13        0
    //    simd3        2        9        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       15       22        0
    //  no simd       28       40        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e5] * self[e1234])
                    - (other[e235] * self[e41])
                    - (other[e315] * self[e42])
                    - (other[e125] * self[e43])
                    - (other[e321] * self[e45]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(other[e321]) * self.group6().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e12345]) * other.group1().xyz()).with_w(0.0)
                + (self.group7().zxy() * other.group0().yzx()).with_w(0.0)
                - (self.group7().yzx() * other.group0().zxy()).with_w(0.0),
            // e5
            (other[e5] * self[e12345]) - (other[e235] * self[e415]) - (other[e315] * self[e425]) - (other[e125] * self[e435]),
            // e15, e25, e35, e45
            ((other.group0().yzx() * self.group9().zxy()) - (other.group0().zxy() * self.group9().yzx())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from(self[e1234]) * other.group0().xyz()) - (Simd32x3::from(other[e321]) * self.group9().xyz()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321] * self[e12345]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * other.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl AntiWedge<AntiLine> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        9        0
    //    simd3        0        5        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       24        0
    fn anti_wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435])
                    - (other[e15] * self[e423])
                    - (other[e25] * self[e431])
                    - (other[e35] * self[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(self[e1234]) * other.group1()).with_w(0.0) + (other.group0().zxy() * self.group9().yzx()).with_w(0.0)
                - (other.group0().yzx() * self.group9().zxy()).with_w(0.0),
            // e5
            -(other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]),
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
impl AntiWedge<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       14        0
    //    simd3        2        9        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       15       23        0
    //  no simd       28       41        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (other[scalar] * self[e12345]) + (other[e3215] * self[e4])
                    - (other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435])
                    - (other[e15] * self[e423])
                    - (other[e25] * self[e431])
                    - (other[e35] * self[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(other[e3215]) * self.group4()).with_w(0.0)
                + (Simd32x3::from(self[e1234]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group9().yzx()).with_w(0.0)
                - (other.group0().yzx() * self.group9().zxy()).with_w(0.0),
            // e5
            (other[e3215] * self[e45]) - (other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]),
            // e15, e25, e35, e45
            ((Simd32x3::from(other[e3215]) * self.group6().xyz()) + (Simd32x3::from(self[e12345]) * other.group1().xyz())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from(other[e3215]) * self.group7()) + (Simd32x3::from(self[e12345]) * other.group0().xyz()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e3215] * self[e1234]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(other[e3215]) * self.group9().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(other[e3215] * self[e12345]),
            // e1234
            0.0,
        )
    }
}
impl AntiWedge<AntiPlane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3        8        0
    fn anti_wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e5] * self[e1234]), 0.0]),
            // e1, e2, e3, e4
            (other.group0().xyz() * self.group0().yy().with_z(self[e12345])).with_w(0.0),
            // e5
            other[e5] * self[e12345],
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
impl AntiWedge<AntiScalar> for MultiVector {
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
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(other[e12345]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[e12345]) * self.group1(),
            // e5
            other[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(other[e12345]) * self.group3(),
            // e41, e42, e43
            Simd32x3::from(other[e12345]) * self.group4(),
            // e23, e31, e12
            Simd32x3::from(other[e12345]) * self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group6(),
            // e423, e431, e412
            Simd32x3::from(other[e12345]) * self.group7(),
            // e235, e315, e125
            Simd32x3::from(other[e12345]) * self.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group9(),
            // e1234
            other[e12345] * self[e1234],
        )
    }
}
impl AntiWedge<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       42        0
    //    simd3        4       10        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       37       54        0
    //  no simd       54       80        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(other[e423] * self[e15])
                    - (other[e431] * self[e25])
                    - (other[e412] * self[e35])
                    - (other[e415] * self[e23])
                    - (other[e425] * self[e31])
                    - (other[e435] * self[e12])
                    - (other[e321] * self[e45])
                    - (other[e235] * self[e41])
                    - (other[e315] * self[e42])
                    - (other[e125] * self[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e412] * self[e315]) + (other[e415] * self[e321]) + (other[e321] * self[e415]) + (other[e315] * self[e412]),
                (other[e423] * self[e125]) + (other[e425] * self[e321]) + (other[e321] * self[e425]) + (other[e125] * self[e423]),
                (other[e431] * self[e235]) + (other[e435] * self[e321]) + (other[e321] * self[e435]) + (other[e235] * self[e431]),
                -(other[e412] * self[e435]) - (other[e415] * self[e423]) - (other[e425] * self[e431]) - (other[e435] * self[e412]),
            ]) - (other.group0().yzx() * self.group8().zxy()).with_w(other[e423] * self[e415])
                - (other.group2().zxy() * self.group7().yzx()).with_w(other[e431] * self[e425]),
            // e5
            -(other[e415] * self[e235])
                - (other[e425] * self[e315])
                - (other[e435] * self[e125])
                - (other[e235] * self[e415])
                - (other[e315] * self[e425])
                - (other[e125] * self[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (other[e415] * self[e3215]) + (other[e315] * self[e4125]),
                (other[e425] * self[e3215]) + (other[e125] * self[e4235]),
                (other[e435] * self[e3215]) + (other[e235] * self[e4315]),
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) - (self.group9().yzxx() * other.group2().zxy().with_w(other[e415])),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group1().xyz()) + (other.group0().zxy() * self.group9().yzx()) - (other.group0().yzx() * self.group9().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e3215]) * other.group0()) + (Simd32x3::from(self[e1234]) * other.group2()) - (Simd32x3::from(other[e321]) * self.group9().xyz()),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1(),
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * other.group0(),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * other.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl AntiWedge<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd3        8       16        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       46       68        0
    //  no simd       80      112        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (other[e12345] * self[scalar])
                    - (other[e423] * self[e15])
                    - (other[e431] * self[e25])
                    - (other[e412] * self[e35])
                    - (other[e415] * self[e23])
                    - (other[e425] * self[e31])
                    - (other[e435] * self[e12])
                    - (other[e321] * self[e45])
                    - (other[e235] * self[e41])
                    - (other[e315] * self[e42])
                    - (other[e125] * self[e43]),
                other[e12345] * self[e12345],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e415] * self[e321]) + (other[e321] * self[e415]) + (other[e315] * self[e412]) + (other[e12345] * self[e1]),
                (other[e425] * self[e321]) + (other[e321] * self[e425]) + (other[e125] * self[e423]) + (other[e12345] * self[e2]),
                (other[e435] * self[e321]) + (other[e321] * self[e435]) + (other[e235] * self[e431]) + (other[e12345] * self[e3]),
                -(other[e412] * self[e435]) - (other[e415] * self[e423]) - (other[e425] * self[e431]) - (other[e435] * self[e412]),
            ]) + (other.group0().zxy() * self.group8().yzx()).with_w(other[e12345] * self[e4])
                - (other.group0().yzx() * self.group8().zxy()).with_w(other[e423] * self[e415])
                - (self.group7().yzx() * other.group2().zxy()).with_w(other[e431] * self[e425]),
            // e5
            (other[e12345] * self[e5])
                - (other[e415] * self[e235])
                - (other[e425] * self[e315])
                - (other[e435] * self[e125])
                - (other[e235] * self[e415])
                - (other[e315] * self[e425])
                - (other[e125] * self[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (other[e315] * self[e4125]) + (other[e12345] * self[e15]),
                (other[e125] * self[e4235]) + (other[e12345] * self[e25]),
                (other[e235] * self[e4315]) + (other[e12345] * self[e35]),
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) + (other.group1().xyz() * self.group9().www()).with_w(other[e12345] * self[e45])
                - (self.group9().yzxx() * other.group2().zxy().with_w(other[e415])),
            // e41, e42, e43
            (Simd32x3::from(other[e12345]) * self.group4()) + (Simd32x3::from(self[e1234]) * other.group1().xyz()) + (other.group0().zxy() * self.group9().yzx())
                - (other.group0().yzx() * self.group9().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[e12345]) * self.group5()) + (Simd32x3::from(self[e3215]) * other.group0()) + (Simd32x3::from(self[e1234]) * other.group2().xyz())
                - (Simd32x3::from(other[e321]) * self.group9().xyz()),
            // e415, e425, e435, e321
            (Simd32x4::from(other[e12345]) * self.group6()) + (Simd32x4::from(self[e12345]) * other.group1()),
            // e423, e431, e412
            (Simd32x3::from(other[e12345]) * self.group7()) + (Simd32x3::from(self[e12345]) * other.group0()),
            // e235, e315, e125
            (Simd32x3::from(other[e12345]) * self.group8()) + (Simd32x3::from(self[e12345]) * other.group2().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group9(),
            // e1234
            other[e12345] * self[e1234],
        )
    }
}
impl AntiWedge<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       23        0
    //    simd3        0        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       18       28        0
    //  no simd       24       40        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(other[e41] * self[e235])
                    - (other[e42] * self[e315])
                    - (other[e43] * self[e125])
                    - (other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435])
                    - (other[e45] * self[e321])
                    - (other[e15] * self[e423])
                    - (other[e25] * self[e431])
                    - (other[e35] * self[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                -(other[e41] * self[e3215]) - (other[e31] * self[e4125]),
                -(other[e42] * self[e3215]) - (other[e12] * self[e4235]),
                -(other[e43] * self[e3215]) - (other[e23] * self[e4315]),
                (other[e43] * self[e4125]) + (other[e45] * self[e1234]),
            ]) + (self.group9().yzxy() * other.group1().zxy().with_w(other[e42]))
                + (Simd32x3::from(self[e1234]) * other.group2()).with_w(other[e41] * self[e4235]),
            // e5
            -(other[e45] * self[e3215]) - (other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]),
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * other.group2().with_w(other[e45]),
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[e12345]) * other.group1().xyz(),
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
impl AntiWedge<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       37        0
    //    simd3        8       17        0
    //    simd4       10        8        0
    // Totals...
    // yes simd       43       62        0
    //  no simd       89      120        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (other[e1234] * self[e5]) + (other[e4235] * self[e1]) + (other[e4315] * self[e2]) + (other[e4125] * self[e3]) + (other[e3215] * self[e4])
                    - (other[e41] * self[e235])
                    - (other[e42] * self[e315])
                    - (other[e43] * self[e125])
                    - (other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435])
                    - (other[e45] * self[e321])
                    - (other[e15] * self[e423])
                    - (other[e25] * self[e431])
                    - (other[e35] * self[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * other.group2().xyz().with_w(other[e45]))
                + (self.group9().yzxz() * other.group1().zxy().with_w(other[e43]))
                + (self.group4() * other.group3().www()).with_w(other[e41] * self[e4235])
                + (self.group5().yzx() * other.group3().zxy()).with_w(other[e42] * self[e4315])
                - (Simd32x4::from(other[e1234]) * self.group3())
                - (other.group3().yzxy() * self.group5().zxy().with_w(self[e42]))
                - (other.group0() * self.group9().www()).with_w(other[e4235] * self[e41])
                - (other.group1().yzx() * self.group9().zxy()).with_w(other[e4125] * self[e43]),
            // e5
            (other[e4235] * self[e15]) + (other[e4315] * self[e25]) + (other[e4125] * self[e35]) + (other[e3215] * self[e45])
                - (other[e45] * self[e3215])
                - (other[e15] * self[e4235])
                - (other[e25] * self[e4315])
                - (other[e35] * self[e4125]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (other[e4125] * self[e315]) + (other[e3215] * self[e415]),
                (other[e4235] * self[e125]) + (other[e3215] * self[e425]),
                (other[e4315] * self[e235]) + (other[e3215] * self[e435]),
                -(other[e4315] * self[e425]) - (other[e4125] * self[e435]),
            ]) + (Simd32x4::from(self[e12345]) * other.group2().xyz().with_w(other[e45]))
                - (self.group8().zxy() * other.group3().yzx()).with_w(other[e4235] * self[e415]),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group6().xyz()) + (Simd32x3::from(self[e12345]) * other.group0()) + (self.group7().zxy() * other.group3().yzx())
                - (self.group7().yzx() * other.group3().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[e1234]) * self.group8()) + (Simd32x3::from(other[e3215]) * self.group7()) + (Simd32x3::from(self[e12345]) * other.group1().xyz())
                - (Simd32x3::from(self[e321]) * other.group3().xyz()),
            // e415, e425, e435, e321
            (other.group3().yzxw() * self.group9().zxy().with_w(self[e1234])) - (self.group9().yzxw() * other.group3().zxy().with_w(other[e1234])),
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * other.group3().xyz()) - (Simd32x3::from(other[e1234]) * self.group9().xyz()),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group9().xyz()) - (Simd32x3::from(self[e3215]) * other.group3().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group3(),
            // e1234
            other[e1234] * self[e12345],
        )
    }
}
impl AntiWedge<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       14        0
    //  no simd        2       34        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other[e5] * self[e1234]) + (other[e12345] * self[scalar]), other[e12345] * self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(other[e12345]) * self.group1(),
            // e5
            (other[e5] * self[e12345]) + (other[e12345] * self[e5]),
            // e15, e25, e35, e45
            Simd32x4::from(other[e12345]) * self.group3(),
            // e41, e42, e43
            Simd32x3::from(other[e12345]) * self.group4(),
            // e23, e31, e12
            Simd32x3::from(other[e12345]) * self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group6(),
            // e423, e431, e412
            Simd32x3::from(other[e12345]) * self.group7(),
            // e235, e315, e125
            Simd32x3::from(other[e12345]) * self.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group9(),
            // e1234
            other[e12345] * self[e1234],
        )
    }
}
impl AntiWedge<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       10        0
    //  no simd        6       16        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([-(other[e15] * self[e423]) - (other[e25] * self[e431]) - (other[e35] * self[e412]) - (other[e45] * self[e321]), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * other.group0(),
            // e5
            -(other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]) - (other[e45] * self[e3215]),
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * other.group0(),
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
impl AntiWedge<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       35        0
    //    simd3        4       11        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       30       49        0
    //  no simd       50       80        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (other[e4235] * self[e1]) + (other[e4315] * self[e2]) + (other[e4125] * self[e3]) + (other[e3215] * self[e4])
                    - (other[e15] * self[e423])
                    - (other[e25] * self[e431])
                    - (other[e35] * self[e412])
                    - (other[e45] * self[e321]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e15] * self[e1234]) + (other[e4125] * self[e31]),
                (other[e25] * self[e1234]) + (other[e4235] * self[e12]),
                (other[e35] * self[e1234]) + (other[e4315] * self[e23]),
                -(other[e4315] * self[e42]) - (other[e4125] * self[e43]),
            ]) + (self.group4() * other.group1().www()).with_w(other[e45] * self[e1234])
                - (other.group1().yzxx() * self.group5().zxy().with_w(self[e41])),
            // e5
            (other[e4235] * self[e15]) + (other[e4315] * self[e25]) + (other[e4125] * self[e35]) + (other[e3215] * self[e45])
                - (other[e15] * self[e4235])
                - (other[e25] * self[e4315])
                - (other[e35] * self[e4125])
                - (other[e45] * self[e3215]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (other[e4125] * self[e315]) + (other[e3215] * self[e415]),
                (other[e4235] * self[e125]) + (other[e3215] * self[e425]),
                (other[e4315] * self[e235]) + (other[e3215] * self[e435]),
                -(other[e4315] * self[e425]) - (other[e4125] * self[e435]),
            ]) + (Simd32x4::from(self[e12345]) * other.group0())
                - (self.group8().zxy() * other.group1().yzx()).with_w(other[e4235] * self[e415]),
            // e41, e42, e43
            (self.group7().zxy() * other.group1().yzx()) - (self.group7().yzx() * other.group1().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[e3215]) * self.group7()) - (Simd32x3::from(self[e321]) * other.group1().xyz()),
            // e415, e425, e435, e321
            ((other.group1().yzx() * self.group9().zxy()) - (other.group1().zxy() * self.group9().yzx())).with_w(other[e3215] * self[e1234]),
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * other.group1().xyz(),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group9().xyz()) - (Simd32x3::from(self[e3215]) * other.group1().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group1(),
            // e1234
            0.0,
        )
    }
}
impl AntiWedge<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       29        0
    //    simd3        0        5        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       20       35        0
    //  no simd       26       48        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(other[e415] * self[e23])
                    - (other[e425] * self[e31])
                    - (other[e435] * self[e12])
                    - (other[e235] * self[e41])
                    - (other[e315] * self[e42])
                    - (other[e125] * self[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e415] * self[e321]) + (other[e315] * self[e412]),
                (other[e425] * self[e321]) + (other[e125] * self[e423]),
                (other[e435] * self[e321]) + (other[e235] * self[e431]),
                -(other[e425] * self[e431]) - (other[e435] * self[e412]),
            ]) - (other.group1().zxy() * self.group7().yzx()).with_w(other[e415] * self[e423]),
            // e5
            -(other[e415] * self[e235])
                - (other[e425] * self[e315])
                - (other[e435] * self[e125])
                - (other[e235] * self[e415])
                - (other[e315] * self[e425])
                - (other[e125] * self[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (other[e415] * self[e3215]) + (other[e315] * self[e4125]),
                (other[e425] * self[e3215]) + (other[e125] * self[e4235]),
                (other[e435] * self[e3215]) + (other[e235] * self[e4315]),
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) - (self.group9().yzxx() * other.group1().zxy().with_w(other[e415])),
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[e1234]) * other.group1(),
            // e415, e425, e435, e321
            (other.group0() * self.group0().yy().with_z(self[e12345])).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * other.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl AntiWedge<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       37        0
    //    simd3        4       11        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       30       51        0
    //  no simd       50       82        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (other[e12345] * self[scalar]) + (other[e5] * self[e1234])
                    - (other[e415] * self[e23])
                    - (other[e425] * self[e31])
                    - (other[e435] * self[e12])
                    - (other[e235] * self[e41])
                    - (other[e315] * self[e42])
                    - (other[e125] * self[e43]),
                other[e12345] * self[e12345],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e415] * self[e321]) + (other[e12345] * self[e1]),
                (other[e425] * self[e321]) + (other[e12345] * self[e2]),
                (other[e435] * self[e321]) + (other[e12345] * self[e3]),
                -(other[e425] * self[e431]) - (other[e435] * self[e412]),
            ]) + (self.group7().zxy() * other.group1().yzx()).with_w(other[e12345] * self[e4])
                - (self.group7().yzx() * other.group1().zxy()).with_w(other[e415] * self[e423]),
            // e5
            (other[e12345] * self[e5]) + (other[e5] * self[e12345])
                - (other[e415] * self[e235])
                - (other[e425] * self[e315])
                - (other[e435] * self[e125])
                - (other[e235] * self[e415])
                - (other[e315] * self[e425])
                - (other[e125] * self[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (other[e12345] * self[e15]) + (other[e315] * self[e4125]),
                (other[e12345] * self[e25]) + (other[e125] * self[e4235]),
                (other[e12345] * self[e35]) + (other[e235] * self[e4315]),
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) + (other.group0() * self.group9().www().with_w(self[e45]))
                - (self.group9().yzxx() * other.group1().zxy().with_w(other[e415])),
            // e41, e42, e43
            (Simd32x3::from(other[e12345]) * self.group4()) + (Simd32x3::from(self[e1234]) * other.group0().xyz()),
            // e23, e31, e12
            (Simd32x3::from(other[e12345]) * self.group5()) + (Simd32x3::from(self[e1234]) * other.group1().xyz()),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e12345]) * self.group6().xyz()) + (Simd32x3::from(self[e12345]) * other.group0().xyz())).with_w(other[e12345] * self[e321]),
            // e423, e431, e412
            Simd32x3::from(other[e12345]) * self.group7(),
            // e235, e315, e125
            (Simd32x3::from(other[e12345]) * self.group8()) + (Simd32x3::from(self[e12345]) * other.group1().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group9(),
            // e1234
            other[e12345] * self[e1234],
        )
    }
}
impl AntiWedge<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       71       93        0
    //    simd3       20       34        0
    //    simd4       20       12        0
    // Totals...
    // yes simd      111      139        0
    //  no simd      211      243        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (other[scalar] * self[e12345])
                    + (other[e12345] * self[scalar])
                    + (other[e1] * self[e4235])
                    + (other[e2] * self[e4315])
                    + (other[e3] * self[e4125])
                    + (other[e4] * self[e3215])
                    + (other[e5] * self[e1234])
                    + (other[e4235] * self[e1])
                    + (other[e4315] * self[e2])
                    + (other[e4125] * self[e3])
                    + (other[e3215] * self[e4])
                    + (other[e1234] * self[e5])
                    - (other[e15] * self[e423])
                    - (other[e25] * self[e431])
                    - (other[e35] * self[e412])
                    - (other[e45] * self[e321])
                    - (other[e41] * self[e235])
                    - (other[e42] * self[e315])
                    - (other[e43] * self[e125])
                    - (other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435])
                    - (other[e415] * self[e23])
                    - (other[e425] * self[e31])
                    - (other[e435] * self[e12])
                    - (other[e321] * self[e45])
                    - (other[e423] * self[e15])
                    - (other[e431] * self[e25])
                    - (other[e412] * self[e35])
                    - (other[e235] * self[e41])
                    - (other[e315] * self[e42])
                    - (other[e125] * self[e43]),
                other[e12345] * self[e12345],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e15] * self[e1234]) + (other[e415] * self[e321]) + (other[e321] * self[e415]) + (other[e4125] * self[e31]),
                (other[e25] * self[e1234]) + (other[e425] * self[e321]) + (other[e321] * self[e425]) + (other[e4235] * self[e12]),
                (other[e35] * self[e1234]) + (other[e435] * self[e321]) + (other[e321] * self[e435]) + (other[e4315] * self[e23]),
                -(other[e415] * self[e423]) - (other[e425] * self[e431]) - (other[e435] * self[e412]) - (other[e1234] * self[e45]),
            ]) + (Simd32x4::from(other[e12345]) * self.group1())
                + (Simd32x4::from(self[e12345]) * other.group1())
                + (self.group9().yzxx() * other.group5().zxy().with_w(other[e41]))
                + (self.group4() * other.group9().www()).with_w(other[e45] * self[e1234])
                + (other.group7().zxy() * self.group8().yzx()).with_w(other[e42] * self[e4315])
                + (other.group8().yzx() * self.group7().zxy()).with_w(other[e43] * self[e4125])
                - (other.group9().yzxy() * self.group5().zxy().with_w(self[e42]))
                - (Simd32x3::from(other[e1234]) * self.group3().xyz()).with_w(other[e4125] * self[e43])
                - (other.group4() * self.group9().www()).with_w(other[e423] * self[e415])
                - (other.group5().yzx() * self.group9().zxy()).with_w(other[e431] * self[e425])
                - (other.group7().yzx() * self.group8().zxy()).with_w(other[e412] * self[e435])
                - (other.group8().zxy() * self.group7().yzx()).with_w(other[e4235] * self[e41]),
            // e5
            (other[e12345] * self[e5])
                + (other[e5] * self[e12345])
                + (other[e4235] * self[e15])
                + (other[e4315] * self[e25])
                + (other[e4125] * self[e35])
                + (other[e3215] * self[e45])
                - (other[e15] * self[e4235])
                - (other[e25] * self[e4315])
                - (other[e35] * self[e4125])
                - (other[e45] * self[e3215])
                - (other[e415] * self[e235])
                - (other[e425] * self[e315])
                - (other[e435] * self[e125])
                - (other[e235] * self[e415])
                - (other[e315] * self[e425])
                - (other[e125] * self[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (other[e415] * self[e3215]) + (other[e315] * self[e4125]) + (other[e4125] * self[e315]) + (other[e3215] * self[e415]),
                (other[e425] * self[e3215]) + (other[e125] * self[e4235]) + (other[e4235] * self[e125]) + (other[e3215] * self[e425]),
                (other[e435] * self[e3215]) + (other[e235] * self[e4315]) + (other[e4315] * self[e235]) + (other[e3215] * self[e435]),
                -(other[e435] * self[e4125]) - (other[e4235] * self[e415]) - (other[e4315] * self[e425]) - (other[e4125] * self[e435]),
            ]) + (Simd32x4::from(other[e12345]) * self.group3())
                + (Simd32x4::from(self[e12345]) * other.group3())
                - (self.group9().yzxx() * other.group8().zxy().with_w(other[e415]))
                - (self.group8().zxy() * other.group9().yzx()).with_w(other[e425] * self[e4315]),
            // e41, e42, e43
            (Simd32x3::from(other[e12345]) * self.group4())
                + (Simd32x3::from(other[e1234]) * self.group6().xyz())
                + (Simd32x3::from(self[e12345]) * other.group4())
                + (Simd32x3::from(self[e1234]) * other.group6().xyz())
                + (other.group7().zxy() * self.group9().yzx())
                + (self.group7().zxy() * other.group9().yzx())
                - (other.group7().yzx() * self.group9().zxy())
                - (self.group7().yzx() * other.group9().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[e12345]) * self.group5())
                + (Simd32x3::from(other[e3215]) * self.group7())
                + (Simd32x3::from(other[e1234]) * self.group8())
                + (Simd32x3::from(self[e12345]) * other.group5())
                + (Simd32x3::from(self[e3215]) * other.group7())
                + (Simd32x3::from(self[e1234]) * other.group8())
                - (Simd32x3::from(other[e321]) * self.group9().xyz())
                - (Simd32x3::from(self[e321]) * other.group9().xyz()),
            // e415, e425, e435, e321
            (Simd32x4::from(other[e12345]) * self.group6()) + (Simd32x4::from(self[e12345]) * other.group6()) + (other.group9().yzxw() * self.group9().zxy().with_w(self[e1234]))
                - (other.group9().zxy() * self.group9().yzx()).with_w(other[e1234] * self[e3215]),
            // e423, e431, e412
            (Simd32x3::from(other[e12345]) * self.group7()) + (Simd32x3::from(self[e12345]) * other.group7()) + (Simd32x3::from(self[e1234]) * other.group9().xyz())
                - (Simd32x3::from(other[e1234]) * self.group9().xyz()),
            // e235, e315, e125
            (Simd32x3::from(other[e12345]) * self.group8()) + (Simd32x3::from(other[e3215]) * self.group9().xyz()) + (Simd32x3::from(self[e12345]) * other.group8())
                - (Simd32x3::from(self[e3215]) * other.group9().xyz()),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(other[e12345]) * self.group9()) + (Simd32x4::from(self[e12345]) * other.group9()),
            // e1234
            (other[e12345] * self[e1234]) + (other[e1234] * self[e12345]),
        )
    }
}
impl AntiWedge<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       25        0
    //    simd3        4        9        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       20       37        0
    //  no simd       34       64        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e4] * other[e3215]), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e3215]) + (self[e31] * other[e4125]),
                (self[e42] * other[e3215]) + (self[e12] * other[e4235]),
                (self[e43] * other[e3215]) + (self[e23] * other[e4315]),
                -(self[e42] * other[e4315]) - (self[e43] * other[e4125]),
            ]) - (other.group0().yzxx() * self.group5().zxy().with_w(self[e41])),
            // e5
            (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]) + (self[e45] * other[e3215]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e415] * other[e3215]) + (self[e315] * other[e4125]),
                (self[e425] * other[e3215]) + (self[e125] * other[e4235]),
                (self[e435] * other[e3215]) + (self[e235] * other[e4315]),
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) - (other.group0().yzxx() * self.group8().zxy().with_w(self[e415])),
            // e41, e42, e43
            (self.group7().zxy() * other.group0().yzx()) - (self.group7().yzx() * other.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[e3215]) * self.group7()) - (Simd32x3::from(self[e321]) * other.group0().xyz()),
            // e415, e425, e435, e321
            ((self.group9().zxy() * other.group0().yzx()) - (self.group9().yzx() * other.group0().zxy())).with_w(self[e1234] * other[e3215]),
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * other.group0().xyz(),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group9().xyz()) - (Simd32x3::from(self[e3215]) * other.group0().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group0(),
            // e1234
            0.0,
        )
    }
}
impl AntiWedge<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        4       10        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e4235] * other[e1]) + (self[e4315] * other[e2]) + (self[e4125] * other[e3]) + (self[e3215] * other[e4]) + (self[e1234] * other[e5]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * other.group0(),
            // e5
            self[e12345] * other[e5],
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
impl AntiWedge<Scalar> for MultiVector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e12345] * other[scalar])
    }
}
impl AntiWedge<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       28        0
    //    simd3        6       12        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       25       44        0
    //  no simd       49       80        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e4] * other[e3215]) + (self[e5] * other[e1234]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e3215]) + (self[e31] * other[e4125]),
                (self[e42] * other[e3215]) + (self[e12] * other[e4235]),
                (self[e43] * other[e3215]) + (self[e23] * other[e4315]),
                -(self[e45] * other[e1234]) - (self[e43] * other[e4125]),
            ]) - (other.group0().yzxx() * self.group5().zxy().with_w(self[e41]))
                - (Simd32x3::from(other[e1234]) * self.group3().xyz()).with_w(self[e42] * other[e4315]),
            // e5
            (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]) + (self[e45] * other[e3215]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e415] * other[e3215]) + (self[e315] * other[e4125]),
                (self[e425] * other[e3215]) + (self[e125] * other[e4235]),
                (self[e435] * other[e3215]) + (self[e235] * other[e4315]),
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) - (other.group0().yzxx() * self.group8().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group6().xyz()) + (self.group7().zxy() * other.group0().yzx()) - (self.group7().yzx() * other.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[e3215]) * self.group7()) + (Simd32x3::from(other[e1234]) * self.group8()) - (Simd32x3::from(self[e321]) * other.group0().xyz()),
            // e415, e425, e435, e321
            (self.group9().zxy() * other.group0().yzx()).with_w(self[e1234] * other[e3215]) - (self.group9().yzxw() * other.group0().zxy().with_w(other[e1234])),
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * other.group0().xyz()) - (Simd32x3::from(other[e1234]) * self.group9().xyz()),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group9().xyz()) - (Simd32x3::from(self[e3215]) * other.group0().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group0(),
            // e1234
            self[e12345] * other[e1234],
        )
    }
}
impl AntiWedge<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       53        0
    //    simd3        8       15        0
    //    simd4        7        6        0
    // Totals...
    // yes simd       53       74        0
    //  no simd       90      122        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[scalar] * other[e12345])
                    + (self[e4235] * other[e1])
                    + (self[e4315] * other[e2])
                    + (self[e4125] * other[e3])
                    + (self[e3215] * other[e4])
                    + (self[e1234] * other[e5])
                    - (self[e15] * other[e423])
                    - (self[e25] * other[e431])
                    - (self[e35] * other[e412])
                    - (self[e45] * other[e321])
                    - (self[e41] * other[e235])
                    - (self[e42] * other[e315])
                    - (self[e43] * other[e125])
                    - (self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435]),
                self[e12345] * other[e12345],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e1] * other[e12345]) + (self[e415] * other[e321]) + (self[e321] * other[e415]) + (self[e315] * other[e412]),
                (self[e2] * other[e12345]) + (self[e425] * other[e321]) + (self[e321] * other[e425]) + (self[e125] * other[e423]),
                (self[e3] * other[e12345]) + (self[e435] * other[e321]) + (self[e321] * other[e435]) + (self[e235] * other[e431]),
                -(self[e415] * other[e423]) - (self[e425] * other[e431]) - (self[e435] * other[e412]) - (self[e412] * other[e435]),
            ]) + (Simd32x4::from(self[e12345]) * other.group3())
                + (self.group7().zxy() * other.group2().yzx()).with_w(self[e4] * other[e12345])
                - (self.group7().yzx() * other.group2().zxy()).with_w(self[e423] * other[e415])
                - (self.group8().zxy() * other.group0().yzx()).with_w(self[e431] * other[e425]),
            // e5
            (self[e12345] * other[e5]) + (self[e5] * other[e12345])
                - (self[e415] * other[e235])
                - (self[e425] * other[e315])
                - (self[e435] * other[e125])
                - (self[e235] * other[e415])
                - (self[e315] * other[e425])
                - (self[e125] * other[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e4125] * other[e315]) + (self[e3215] * other[e415]),
                (self[e4235] * other[e125]) + (self[e3215] * other[e425]),
                (self[e4315] * other[e235]) + (self[e3215] * other[e435]),
                -(self[e4315] * other[e425]) - (self[e4125] * other[e435]),
            ]) + (Simd32x4::from(other[e12345]) * self.group3())
                - (self.group9().yzxx() * other.group2().zxy().with_w(other[e415])),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group1().xyz()) + (Simd32x3::from(other[e12345]) * self.group4()) + (self.group9().yzx() * other.group0().zxy())
                - (self.group9().zxy() * other.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(self[e3215]) * other.group0().xyz()) + (Simd32x3::from(self[e1234]) * other.group2().xyz()) + (Simd32x3::from(other[e12345]) * self.group5())
                - (Simd32x3::from(other[e321]) * self.group9().xyz()),
            // e415, e425, e435, e321
            (Simd32x4::from(self[e12345]) * other.group1()) + (Simd32x4::from(other[e12345]) * self.group6()),
            // e423, e431, e412
            (Simd32x3::from(self[e12345]) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * self.group7()),
            // e235, e315, e125
            (Simd32x3::from(self[e12345]) * other.group2().xyz()) + (Simd32x3::from(other[e12345]) * self.group8()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group9(),
            // e1234
            self[e1234] * other[e12345],
        )
    }
}
impl AntiWedge<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       39        0
    //    simd3        8       18        0
    //    simd4       10        7        0
    // Totals...
    // yes simd       44       64        0
    //  no simd       90      121        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e12345] * other[scalar])
                    + (self[e1] * other[e4235])
                    + (self[e2] * other[e4315])
                    + (self[e3] * other[e4125])
                    + (self[e4] * other[e3215])
                    + (self[e5] * other[e1234])
                    - (self[e415] * other[e23])
                    - (self[e425] * other[e31])
                    - (self[e435] * other[e12])
                    - (self[e321] * other[e45])
                    - (self[e423] * other[e15])
                    - (self[e431] * other[e25])
                    - (self[e412] * other[e35])
                    - (self[e235] * other[e41])
                    - (self[e315] * other[e42])
                    - (self[e125] * other[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * other.group2().xyz().with_w(other[e45]))
                + (self.group9().yzxz() * other.group1().zxy().with_w(other[e43]))
                + (self.group4() * other.group3().www()).with_w(self[e4235] * other[e41])
                + (self.group5().yzx() * other.group3().zxy()).with_w(self[e4315] * other[e42])
                - (other.group3().yzxx() * self.group5().zxy().with_w(self[e41]))
                - (self.group9().zxy() * other.group1().yzx()).with_w(self[e43] * other[e4125])
                - (self.group3().xyz() * other.group2().www()).with_w(self[e42] * other[e4315])
                - (other.group0().xyz() * self.group9().www()).with_w(self[e45] * other[e1234]),
            // e5
            (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]) + (self[e45] * other[e3215])
                - (self[e4235] * other[e15])
                - (self[e4315] * other[e25])
                - (self[e4125] * other[e35])
                - (self[e3215] * other[e45]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e415] * other[e3215]) + (self[e315] * other[e4125]),
                (self[e425] * other[e3215]) + (self[e125] * other[e4235]),
                (self[e435] * other[e3215]) + (self[e235] * other[e4315]),
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) + (Simd32x4::from(self[e12345]) * other.group2().xyz().with_w(other[e45]))
                - (other.group3().yzxx() * self.group8().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(self[e12345]) * other.group0().xyz()) + (Simd32x3::from(other[e1234]) * self.group6().xyz()) + (self.group7().zxy() * other.group3().yzx())
                - (self.group7().yzx() * other.group3().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e12345]) * other.group1().xyz()) + (Simd32x3::from(other[e1234]) * self.group8()) + (Simd32x3::from(other[e3215]) * self.group7())
                - (Simd32x3::from(self[e321]) * other.group3().xyz()),
            // e415, e425, e435, e321
            (self.group9().zxy() * other.group3().yzx()).with_w(self[e1234] * other[e3215]) - (self.group9().yzxw() * other.group3().zxy().with_w(other[e1234])),
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * other.group3().xyz()) - (Simd32x3::from(other[e1234]) * self.group9().xyz()),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group9().xyz()) - (Simd32x3::from(self[e3215]) * other.group3().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group3(),
            // e1234
            self[e12345] * other[e1234],
        )
    }
}
impl std::ops::Div<AntiWedgeInfix> for Plane {
    type Output = AntiWedgeInfixPartial<Plane>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiCircleRotor> for Plane {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       11       16        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(other[e41] * self[e3215]) - (other[e31] * self[e4125]),
                -(other[e42] * self[e3215]) - (other[e12] * self[e4235]),
                -(other[e43] * self[e3215]) - (other[e23] * self[e4315]),
                (other[e42] * self[e4315]) + (other[e43] * self[e4125]),
            ]) + (self.group0().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e5
            -(other[e45] * self[e3215]) - (other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]),
        )
    }
}
impl AntiWedge<AntiDipoleInversion> for Plane {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       13        0
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        6       18        0
    //  no simd       17       31        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                other[e423] * self[e3215],
                other[e431] * self[e3215],
                other[e412] * self[e3215],
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) - (other.group1().wwwx() * self.group0().xyzx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                other[e125] * self[e4315] * -1.0,
                other[e235] * self[e4125] * -1.0,
                other[e315] * self[e4235] * -1.0,
                (other[e2] * self[e4315]) + (other[e3] * self[e4125]),
            ]) + (Simd32x4::from(self[e3215]) * other.group1().xyz().with_w(other[e4]))
                + (self.group0().zxyx() * other.group2().yzx().with_w(other[e1])),
        )
    }
}
impl AntiWedge<AntiDualNum> for Plane {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ (self.group0().xyz() * other.group0().xx().with_z(other[e3215])).with_w(0.0))
    }
}
impl AntiWedge<AntiFlatPoint> for Plane {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        4        0
    // no simd        3       12        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(other[e321]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e15, e25, e35
            (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx()),
        )
    }
}
impl AntiWedge<AntiFlector> for Plane {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        1        4        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        5       15        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (self.group0().xyz() * other.group0().www() * Simd32x3::from(-1.0)).with_w((other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125])),
            // e15, e25, e35, e3215
            ((other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiLine> for Plane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        5        9        0
    fn anti_wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                other[e12] * self[e4315],
                other[e23] * self[e4125],
                other[e31] * self[e4235],
                -(other[e25] * self[e4315]) - (other[e35] * self[e4125]),
            ]) - (self.group0().zxyx() * other.group0().yzx().with_w(other[e15])),
        )
    }
}
impl AntiWedge<AntiMotor> for Plane {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        5       12        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (self.group0().xyz() * other.group1().www()).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([
                other[e12] * self[e4315],
                other[e23] * self[e4125],
                other[e31] * self[e4235],
                -(other[e25] * self[e4315]) - (other[e35] * self[e4125]),
            ]) - (self.group0().zxyx() * other.group0().yzx().with_w(other[e15])),
        )
    }
}
impl AntiWedge<AntiPlane> for Plane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn anti_wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]))
    }
}
impl AntiWedge<AntiScalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<Circle> for Plane {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        3        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd       14       24        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            (other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                other[e423] * self[e3215],
                other[e431] * self[e3215],
                other[e412] * self[e3215],
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) - (other.group1().wwwx() * self.group0().xyzx()),
            // e15, e25, e35
            (Simd32x3::from(self[e3215]) * other.group1().xyz()) + (other.group2().yzx() * self.group0().zxy()) - (other.group2().zxy() * self.group0().yzx()),
        )
    }
}
impl AntiWedge<CircleRotor> for Plane {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        1        5        0
    //    simd4        3        2        0
    // Totals...
    // yes simd        5       12        0
    //  no simd       16       28        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            (other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                other[e423] * self[e3215],
                other[e431] * self[e3215],
                other[e412] * self[e3215],
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) - (other.group1().wwwx() * self.group0().xyzx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(self[e3215]) * other.group1().xyz()).with_w(0.0) + (other.group2().yzx() * self.group0().zxy()).with_w(0.0)
                - (other.group2().zxy() * self.group0().yzx()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group0(),
        )
    }
}
impl AntiWedge<Dipole> for Plane {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       11       16        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(other[e41] * self[e3215]) - (other[e31] * self[e4125]),
                -(other[e42] * self[e3215]) - (other[e12] * self[e4235]),
                -(other[e43] * self[e3215]) - (other[e23] * self[e4315]),
                (other[e42] * self[e4315]) + (other[e43] * self[e4125]),
            ]) + (self.group0().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e5
            -(other[e45] * self[e3215]) - (other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]),
        )
    }
}
impl AntiWedge<DipoleInversion> for Plane {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       15        0
    //    simd3        1        4        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        6       22        0
    //  no simd       17       39        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e1234]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            ((other.group3().yzx() * self.group0().zxy()) - (other.group3().zxy() * self.group0().yzx())).with_w(other[e1234] * self[e3215] * -1.0),
            // e235, e315, e125, e4
            Simd32x4::from([
                other[e4235] * self[e3215] * -1.0,
                other[e4315] * self[e3215] * -1.0,
                other[e4125] * self[e3215] * -1.0,
                (other[e42] * self[e4315]) + (other[e43] * self[e4125]),
            ]) + (self.group0().xyzx() * other.group3().www().with_w(other[e41])),
            // e1, e2, e3, e5
            Simd32x4::from([
                other[e12] * self[e4315],
                other[e23] * self[e4125],
                other[e31] * self[e4235],
                -(other[e25] * self[e4315]) - (other[e35] * self[e4125]),
            ]) - (Simd32x4::from(self[e3215]) * other.group0().with_w(other[e45]))
                - (self.group0().zxyx() * other.group1().yzx().with_w(other[e15])),
        )
    }
}
impl AntiWedge<DualNum> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<FlatPoint> for Plane {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            -(other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]) - (other[e45] * self[e3215]),
            0.0,
        ]))
    }
}
impl AntiWedge<Flector> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        9       16        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((other.group1().yzx() * self.group0().zxy()) - (other.group1().zxy() * self.group0().yzx())).with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                other[e3215] * self[e4235],
                other[e3215] * self[e4315],
                other[e3215] * self[e4125],
                -(other[e25] * self[e4315]) - (other[e35] * self[e4125]) - (other[e45] * self[e3215]),
            ]) - (self.group0().wwwx() * other.group1().xyz().with_w(other[e15])),
        )
    }
}
impl AntiWedge<Line> for Plane {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (other[e415] * self[e3215]) + (other[e315] * self[e4125]),
                (other[e425] * self[e3215]) + (other[e125] * self[e4235]),
                (other[e435] * self[e3215]) + (other[e235] * self[e4315]),
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) - (self.group0().yzxx() * other.group1().zxy().with_w(other[e415])),
        )
    }
}
impl AntiWedge<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (other[e415] * self[e3215]) + (other[e315] * self[e4125]),
                (other[e425] * self[e3215]) + (other[e125] * self[e4235]),
                (other[e435] * self[e3215]) + (other[e235] * self[e4315]),
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) - (self.group0().yzxx() * other.group1().zxy().with_w(other[e415])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group0(),
        )
    }
}
impl AntiWedge<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       26        0
    //    simd3        4       10        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       20       39        0
    //  no simd       34       68        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e4] * self[e3215]), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([
                -(other[e41] * self[e3215]) - (other[e31] * self[e4125]),
                -(other[e42] * self[e3215]) - (other[e12] * self[e4235]),
                -(other[e43] * self[e3215]) - (other[e23] * self[e4315]),
                (other[e42] * self[e4315]) + (other[e43] * self[e4125]),
            ]) + (self.group0().yzxx() * other.group5().zxy().with_w(other[e41])),
            // e5
            -(other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]) - (other[e45] * self[e3215]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (other[e415] * self[e3215]) + (other[e315] * self[e4125]),
                (other[e425] * self[e3215]) + (other[e125] * self[e4235]),
                (other[e435] * self[e3215]) + (other[e235] * self[e4315]),
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) - (self.group0().yzxx() * other.group8().zxy().with_w(other[e415])),
            // e41, e42, e43
            (other.group7().zxy() * self.group0().yzx()) - (other.group7().yzx() * self.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e3215]) * other.group7()) - (Simd32x3::from(other[e321]) * self.group0().xyz()),
            // e415, e425, e435, e321
            ((other.group9().yzx() * self.group0().zxy()) - (other.group9().zxy() * self.group0().yzx())).with_w(other[e1234] * self[e3215] * -1.0),
            // e423, e431, e412
            Simd32x3::from(other[e1234]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * other.group9().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group0(),
            // e1234
            0.0,
        )
    }
}
impl AntiWedge<Plane> for Plane {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        4        0
    // no simd        6       12        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx()),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * other.group0().xyz()),
        )
    }
}
impl AntiWedge<RoundPoint> for Plane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e4235] * other[e1]) + (self[e4315] * other[e2]) + (self[e4125] * other[e3]) + (self[e3215] * other[e4]),
        )
    }
}
impl AntiWedge<Sphere> for Plane {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        2        6        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        6       20        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e1234]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            ((self.group0().zxy() * other.group0().yzx()) - (self.group0().yzx() * other.group0().zxy())).with_w(self[e3215] * other[e1234] * -1.0),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * other.group0().xyz()),
        )
    }
}
impl AntiWedge<VersorEven> for Plane {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       14        0
    //    simd3        0        3        0
    //    simd4        4        3        0
    // Totals...
    // yes simd        7       20        0
    //  no simd       19       35        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                self[e4125] * other[e431] * -1.0,
                self[e4235] * other[e412] * -1.0,
                self[e4315] * other[e423] * -1.0,
                (self[e4315] * other[e2]) + (self[e4125] * other[e3]) + (self[e3215] * other[e4]),
            ]) + (self.group0().yzxx() * other.group0().zxy().with_w(other[e1])),
            // e23, e31, e12, e45
            Simd32x4::from([
                self[e3215] * other[e423],
                self[e3215] * other[e431],
                self[e3215] * other[e412],
                -(self[e4315] * other[e425]) - (self[e4125] * other[e435]),
            ]) - (self.group0().xyzx() * other.group1().wwwx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(self[e3215]) * other.group1().xyz()).with_w(0.0) + (self.group0().zxy() * other.group2().yzx()).with_w(0.0)
                - (self.group0().yzx() * other.group2().zxy()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group0(),
        )
    }
}
impl AntiWedge<VersorOdd> for Plane {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       15        0
    //    simd3        1        4        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        6       22        0
    //  no simd       17       39        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e1234]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            ((self.group0().zxy() * other.group3().yzx()) - (self.group0().yzx() * other.group3().zxy())).with_w(self[e3215] * other[e1234] * -1.0),
            // e235, e315, e125, e4
            Simd32x4::from([
                self[e3215] * other[e4235] * -1.0,
                self[e3215] * other[e4315] * -1.0,
                self[e3215] * other[e4125] * -1.0,
                (self[e4315] * other[e42]) + (self[e4125] * other[e43]),
            ]) + (self.group0().xyzx() * other.group3().www().with_w(other[e41])),
            // e1, e2, e3, e5
            Simd32x4::from([
                self[e4315] * other[e12],
                self[e4125] * other[e23],
                self[e4235] * other[e31],
                -(self[e4125] * other[e35]) - (self[e3215] * other[e45]),
            ]) - (self.group0().zxyx() * other.group1().yzx().with_w(other[e15]))
                - (self.group0().wwwy() * other.group0().xyz().with_w(other[e25])),
        )
    }
}
impl std::ops::Div<AntiWedgeInfix> for RoundPoint {
    type Output = AntiWedgeInfixPartial<RoundPoint>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiDualNum> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e3215] * self[e4])
    }
}
impl AntiWedge<AntiMotor> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e3215] * self[e4])
    }
}
impl AntiWedge<AntiScalar> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[e12345]) * self.group0(), /* e5 */ other[e12345] * self[e5])
    }
}
impl AntiWedge<CircleRotor> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[e12345]) * self.group0(), /* e5 */ other[e12345] * self[e5])
    }
}
impl AntiWedge<DipoleInversion> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (other[e1234] * self[e5]) + (other[e4235] * self[e1]) + (other[e4315] * self[e2]) + (other[e4125] * self[e3]) + (other[e3215] * self[e4]),
        )
    }
}
impl AntiWedge<DualNum> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[e12345]) * self.group0(), /* e5 */ other[e12345] * self[e5])
    }
}
impl AntiWedge<Flector> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (other[e4235] * self[e1]) + (other[e4315] * self[e2]) + (other[e4125] * self[e3]) + (other[e3215] * self[e4]),
        )
    }
}
impl AntiWedge<Motor> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[e12345]) * self.group0(), /* e5 */ other[e12345] * self[e5])
    }
}
impl AntiWedge<MultiVector> for RoundPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        4       10        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (other[e4235] * self[e1]) + (other[e4315] * self[e2]) + (other[e4125] * self[e3]) + (other[e3215] * self[e4]) + (other[e1234] * self[e5]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other[e12345]) * self.group0(),
            // e5
            other[e12345] * self[e5],
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
impl AntiWedge<Plane> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (other[e4235] * self[e1]) + (other[e4315] * self[e2]) + (other[e4125] * self[e3]) + (other[e3215] * self[e4]),
        )
    }
}
impl AntiWedge<Sphere> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e4] * other[e3215]) + (self[e5] * other[e1234]),
        )
    }
}
impl AntiWedge<VersorEven> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[e12345]) * self.group0(), /* e5 */ self[e5] * other[e12345])
    }
}
impl AntiWedge<VersorOdd> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e4] * other[e3215]) + (self[e5] * other[e1234]),
        )
    }
}
impl std::ops::Div<AntiWedgeInfix> for Scalar {
    type Output = AntiWedgeInfixPartial<Scalar>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiScalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e12345] * self[scalar])
    }
}
impl AntiWedge<CircleRotor> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e12345] * self[scalar])
    }
}
impl AntiWedge<DualNum> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e12345] * self[scalar])
    }
}
impl AntiWedge<Motor> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e12345] * self[scalar])
    }
}
impl AntiWedge<MultiVector> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e12345] * self[scalar])
    }
}
impl AntiWedge<VersorEven> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * other[e12345])
    }
}
impl std::ops::Div<AntiWedgeInfix> for Sphere {
    type Output = AntiWedgeInfixPartial<Sphere>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiCircleRotor> for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       20        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(other[e41] * self[e3215]) - (other[e31] * self[e4125]),
                -(other[e42] * self[e3215]) - (other[e12] * self[e4235]),
                -(other[e43] * self[e3215]) - (other[e23] * self[e4315]),
                (other[e43] * self[e4125]) + (other[e45] * self[e1234]),
            ]) + (self.group0().yzxx() * other.group1().zxy().with_w(other[e41]))
                + (Simd32x3::from(self[e1234]) * other.group2().xyz()).with_w(other[e42] * self[e4315]),
            // e5
            -(other[e45] * self[e3215]) - (other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]),
        )
    }
}
impl AntiWedge<AntiDipoleInversion> for Sphere {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        2        3        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       24       38        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group1().xyz()) + (other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other[e423] * self[e3215]) + (other[e235] * self[e1234]),
                (other[e431] * self[e3215]) + (other[e315] * self[e1234]),
                (other[e412] * self[e3215]) + (other[e125] * self[e1234]),
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) - (other.group1().wwwx() * self.group0().xyzx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                other[e125] * self[e4315] * -1.0,
                other[e235] * self[e4125] * -1.0,
                other[e315] * self[e4235] * -1.0,
                (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e5] * self[e1234]),
            ]) + (Simd32x4::from(self[e3215]) * other.group1().xyz().with_w(other[e4]))
                + (self.group0().zxyx() * other.group2().yzx().with_w(other[e1])),
        )
    }
}
impl AntiWedge<AntiDualNum> for Sphere {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other[e3215]) * self.group0().xyz().with_w(self[e1234]))
    }
}
impl AntiWedge<AntiFlatPoint> for Sphere {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        4        0
    // no simd        6       12        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            (Simd32x3::from(self[e1234]) * other.group0().xyz()) - (Simd32x3::from(other[e321]) * self.group0().xyz()),
            // e15, e25, e35
            (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx()),
        )
    }
}
impl AntiWedge<AntiFlector> for Sphere {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       10        0
    //    simd3        1        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4       13        0
    //  no simd        9       19        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                other[e321] * self[e4235] * -1.0,
                other[e321] * self[e4315] * -1.0,
                other[e321] * self[e4125] * -1.0,
                (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e5] * self[e1234]),
            ]) + (Simd32x3::from(self[e1234]) * other.group0().xyz()).with_w(other[e1] * self[e4235]),
            // e15, e25, e35, e3215
            ((other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiLine> for Sphere {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn anti_wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e12] * self[e4315]) + (other[e15] * self[e1234]),
                (other[e23] * self[e4125]) + (other[e25] * self[e1234]),
                (other[e31] * self[e4235]) + (other[e35] * self[e1234]),
                -(other[e25] * self[e4315]) - (other[e35] * self[e4125]),
            ]) - (self.group0().zxyx() * other.group0().yzx().with_w(other[e15])),
        )
    }
}
impl AntiWedge<AntiMotor> for Sphere {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e3215]) * self.group0().xyz().with_w(self[e1234]),
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e12] * self[e4315]) + (other[e15] * self[e1234]),
                (other[e23] * self[e4125]) + (other[e25] * self[e1234]),
                (other[e31] * self[e4235]) + (other[e35] * self[e1234]),
                -(other[e25] * self[e4315]) - (other[e35] * self[e4125]),
            ]) - (self.group0().zxyx() * other.group0().yzx().with_w(other[e15])),
        )
    }
}
impl AntiWedge<AntiPlane> for Sphere {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e5] * self[e1234]),
        )
    }
}
impl AntiWedge<AntiScalar> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group0(),
            // e1234
            other[e12345] * self[e1234],
        )
    }
}
impl AntiWedge<Circle> for Sphere {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        4        6        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       20       30        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group1().xyz()) + (other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other[e423] * self[e3215]) + (other[e235] * self[e1234]),
                (other[e431] * self[e3215]) + (other[e315] * self[e1234]),
                (other[e412] * self[e3215]) + (other[e125] * self[e1234]),
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) - (other.group1().wwwx() * self.group0().xyzx()),
            // e15, e25, e35
            (Simd32x3::from(self[e3215]) * other.group1().xyz()) + (other.group2().yzx() * self.group0().zxy()) - (other.group2().zxy() * self.group0().yzx()),
        )
    }
}
impl AntiWedge<CircleRotor> for Sphere {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        4        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       20       35        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group1().xyz()) + (other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other[e423] * self[e3215]) + (other[e235] * self[e1234]),
                (other[e431] * self[e3215]) + (other[e315] * self[e1234]),
                (other[e412] * self[e3215]) + (other[e125] * self[e1234]),
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) - (other.group1().wwwx() * self.group0().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e3215]) * other.group1().xyz()) + (other.group2().yzx() * self.group0().zxy()) - (other.group2().zxy() * self.group0().yzx()))
                .with_w(other[e12345] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group0(),
        )
    }
}
impl AntiWedge<Dipole> for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       20        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(other[e41] * self[e3215]) - (other[e31] * self[e4125]),
                -(other[e42] * self[e3215]) - (other[e12] * self[e4235]),
                -(other[e43] * self[e3215]) - (other[e23] * self[e4315]),
                (other[e43] * self[e4125]) + (other[e45] * self[e1234]),
            ]) + (self.group0().yzxy() * other.group1().zxy().with_w(other[e42]))
                + (Simd32x3::from(self[e1234]) * other.group2()).with_w(other[e41] * self[e4235]),
            // e5
            -(other[e45] * self[e3215]) - (other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]),
        )
    }
}
impl AntiWedge<DipoleInversion> for Sphere {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        1        2        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       11       24        0
    //  no simd       25       43        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * other.group3().xyz()) - (Simd32x3::from(other[e1234]) * self.group0().xyz()),
            // e415, e425, e435, e321
            (other.group3().yzxw() * self.group0().zxy().with_w(self[e1234])) - (self.group0().yzxw() * other.group3().zxy().with_w(other[e1234])),
            // e235, e315, e125, e4
            Simd32x4::from([
                other[e4235] * self[e3215] * -1.0,
                other[e4315] * self[e3215] * -1.0,
                other[e4125] * self[e3215] * -1.0,
                (other[e42] * self[e4315]) + (other[e43] * self[e4125]) + (other[e45] * self[e1234]),
            ]) + (self.group0().xyzx() * other.group3().www().with_w(other[e41])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e12] * self[e4315]) + (other[e15] * self[e1234]),
                (other[e23] * self[e4125]) + (other[e25] * self[e1234]),
                (other[e31] * self[e4235]) + (other[e35] * self[e1234]),
                -(other[e25] * self[e4315]) - (other[e35] * self[e4125]),
            ]) - (Simd32x4::from(self[e3215]) * other.group0().with_w(other[e45]))
                - (self.group0().zxyx() * other.group1().yzx().with_w(other[e15])),
        )
    }
}
impl AntiWedge<DualNum> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(other[e5] * self[e1234]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(other[e12345] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group0(),
        )
    }
}
impl AntiWedge<FlatPoint> for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * other.group0(),
            // e5
            -(other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]) - (other[e45] * self[e3215]),
        )
    }
}
impl AntiWedge<Flector> for Sphere {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        2        6        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        9       24        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * other.group1().xyz(),
            // e415, e425, e435, e321
            ((other.group1().yzx() * self.group0().zxy()) - (other.group1().zxy() * self.group0().yzx())).with_w(other[e3215] * self[e1234]),
            // e235, e315, e125, e4
            ((Simd32x3::from(other[e3215]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * other.group1().xyz())).with_w(other[e45] * self[e1234]),
            // e1, e2, e3, e5
            (Simd32x3::from(self[e1234]) * other.group0().xyz())
                .with_w(-(other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]) - (other[e45] * self[e3215])),
        )
    }
}
impl AntiWedge<Line> for Sphere {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        2        5        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        8       18        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group0(),
            // e23, e31, e12, e45
            (Simd32x3::from(self[e1234]) * other.group1()).with_w(-(other[e415] * self[e4235]) - (other[e425] * self[e4315]) - (other[e435] * self[e4125])),
            // e15, e25, e35
            (Simd32x3::from(self[e3215]) * other.group0()) + (other.group1().yzx() * self.group0().zxy()) - (other.group1().zxy() * self.group0().yzx()),
        )
    }
}
impl AntiWedge<Motor> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        2        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        8       24        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(self[e1234]) * other.group0().xyz().with_w(other[e5]),
            // e23, e31, e12, e45
            (Simd32x3::from(self[e1234]) * other.group1().xyz()).with_w(-(other[e415] * self[e4235]) - (other[e425] * self[e4315]) - (other[e435] * self[e4125])),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e3215]) * other.group0().xyz()) + (other.group1().yzx() * self.group0().zxy()) - (other.group1().zxy() * self.group0().yzx()))
                .with_w(other[e12345] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group0(),
        )
    }
}
impl AntiWedge<MultiVector> for Sphere {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       28        0
    //    simd3        6       12        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       25       44        0
    //  no simd       49       80        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e4] * self[e3215]) + (other[e5] * self[e1234]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                -(other[e41] * self[e3215]) - (other[e31] * self[e4125]),
                -(other[e42] * self[e3215]) - (other[e12] * self[e4235]),
                -(other[e43] * self[e3215]) - (other[e23] * self[e4315]),
                (other[e45] * self[e1234]) + (other[e43] * self[e4125]),
            ]) + (self.group0().yzxx() * other.group5().zxy().with_w(other[e41]))
                + (Simd32x3::from(self[e1234]) * other.group3().xyz()).with_w(other[e42] * self[e4315]),
            // e5
            -(other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]) - (other[e45] * self[e3215]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (other[e415] * self[e3215]) + (other[e315] * self[e4125]),
                (other[e425] * self[e3215]) + (other[e125] * self[e4235]),
                (other[e435] * self[e3215]) + (other[e235] * self[e4315]),
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) - (self.group0().yzxx() * other.group8().zxy().with_w(other[e415])),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group6().xyz()) + (other.group7().zxy() * self.group0().yzx()) - (other.group7().yzx() * self.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e3215]) * other.group7()) + (Simd32x3::from(self[e1234]) * other.group8()) - (Simd32x3::from(other[e321]) * self.group0().xyz()),
            // e415, e425, e435, e321
            (other.group9().yzxw() * self.group0().zxy().with_w(self[e1234])) - (other.group9().zxy() * self.group0().yzx()).with_w(other[e1234] * self[e3215]),
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * other.group9().xyz()) - (Simd32x3::from(other[e1234]) * self.group0().xyz()),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * other.group9().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group0(),
            // e1234
            other[e12345] * self[e1234],
        )
    }
}
impl AntiWedge<Plane> for Sphere {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        2        5        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        6       16        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * other.group0().xyz(),
            // e415, e425, e435, e321
            ((other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx())).with_w(other[e3215] * self[e1234]),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * other.group0().xyz()),
        )
    }
}
impl AntiWedge<RoundPoint> for Sphere {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e4] * self[e3215]) + (other[e5] * self[e1234]),
        )
    }
}
impl AntiWedge<Sphere> for Sphere {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        2        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3        7        0
    //  no simd       10       20        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * other.group0().xyz()) - (Simd32x3::from(other[e1234]) * self.group0().xyz()),
            // e415, e425, e435, e321
            (other.group0().yzxw() * self.group0().zxy().with_w(self[e1234])) - (other.group0().zxy() * self.group0().yzx()).with_w(other[e1234] * self[e3215]),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * other.group0().xyz()),
        )
    }
}
impl AntiWedge<VersorEven> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       19        0
    //    simd3        2        4        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       11       26        0
    //  no simd       24       43        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                self[e4125] * other[e431] * -1.0,
                self[e4235] * other[e412] * -1.0,
                self[e4315] * other[e423] * -1.0,
                (self[e4125] * other[e3]) + (self[e3215] * other[e4]) + (self[e1234] * other[e5]),
            ]) + (self.group0().yzxx() * other.group0().zxy().with_w(other[e1]))
                + (Simd32x3::from(self[e1234]) * other.group1().xyz()).with_w(self[e4315] * other[e2]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e3215] * other[e423]) + (self[e1234] * other[e235]),
                (self[e3215] * other[e431]) + (self[e1234] * other[e315]),
                (self[e3215] * other[e412]) + (self[e1234] * other[e125]),
                -(self[e4315] * other[e425]) - (self[e4125] * other[e435]),
            ]) - (self.group0().xyzx() * other.group1().wwwx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e3215]) * other.group1().xyz()) + (self.group0().zxy() * other.group2().yzx()) - (self.group0().yzx() * other.group2().zxy()))
                .with_w(self[e1234] * other[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group0(),
        )
    }
}
impl AntiWedge<VersorOdd> for Sphere {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       18        0
    //    simd3        1        3        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       11       25        0
    //  no simd       25       43        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * other.group3().xyz()) - (Simd32x3::from(other[e1234]) * self.group0().xyz()),
            // e415, e425, e435, e321
            (self.group0().zxy() * other.group3().yzx()).with_w(self[e1234] * other[e3215]) - (self.group0().yzxw() * other.group3().zxy().with_w(other[e1234])),
            // e235, e315, e125, e4
            Simd32x4::from([
                self[e3215] * other[e4235] * -1.0,
                self[e3215] * other[e4315] * -1.0,
                self[e3215] * other[e4125] * -1.0,
                (self[e4315] * other[e42]) + (self[e4125] * other[e43]) + (self[e1234] * other[e45]),
            ]) + (self.group0().xyzx() * other.group3().www().with_w(other[e41])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e4315] * other[e12]) + (self[e1234] * other[e15]),
                (self[e4125] * other[e23]) + (self[e1234] * other[e25]),
                (self[e4235] * other[e31]) + (self[e1234] * other[e35]),
                -(self[e4125] * other[e35]) - (self[e3215] * other[e45]),
            ]) - (self.group0().zxyx() * other.group1().yzx().with_w(other[e15]))
                - (self.group0().wwwy() * other.group0().xyz().with_w(other[e25])),
        )
    }
}
impl std::ops::Div<AntiWedgeInfix> for VersorEven {
    type Output = AntiWedgeInfixPartial<VersorEven>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiCircleRotor> for VersorEven {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       11        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       14        0
    //  no simd       10       21        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * other.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(self[e12345]) * other.group2().xyz()).with_w(
                (other[scalar] * self[e12345])
                    - (other[e41] * self[e235])
                    - (other[e42] * self[e315])
                    - (other[e43] * self[e125])
                    - (other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435])
                    - (other[e45] * self[e321])
                    - (other[e15] * self[e423])
                    - (other[e25] * self[e431])
                    - (other[e35] * self[e412]),
            ),
        )
    }
}
impl AntiWedge<AntiDipoleInversion> for VersorEven {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       25        0
    //    simd3        0        4        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       21       31        0
    //  no simd       30       45        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(self[e12345]) * other.group2().xyz()).with_w(
                (other[e4] * self[e12345])
                    - (other[e423] * self[e415])
                    - (other[e431] * self[e425])
                    - (other[e412] * self[e435])
                    - (other[e415] * self[e423])
                    - (other[e425] * self[e431])
                    - (other[e435] * self[e412]),
            ),
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e415] * self[e321]) + (other[e321] * self[e415]) + (other[e315] * self[e412]) + (other[e1] * self[e12345]),
                (other[e425] * self[e321]) + (other[e321] * self[e425]) + (other[e125] * self[e423]) + (other[e2] * self[e12345]),
                (other[e435] * self[e321]) + (other[e321] * self[e435]) + (other[e235] * self[e431]) + (other[e3] * self[e12345]),
                -(other[e435] * self[e125]) - (other[e235] * self[e415]) - (other[e315] * self[e425]) - (other[e125] * self[e435]),
            ]) + (other.group0().zxy() * self.group2().yzx()).with_w(other[e5] * self[e12345])
                - (self.group2().zxyx() * other.group0().yzx().with_w(other[e415]))
                - (other.group2().zxy() * self.group0().yzx()).with_w(other[e425] * self[e315]),
        )
    }
}
impl AntiWedge<AntiDualNum> for VersorEven {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1       10        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0().xx().with_zw(other[e3215], (other[e3215] * self[e4]) + (other[scalar] * self[e12345])) * self.group0().xyz().with_w(1.0),
            // e15, e25, e35, e3215
            Simd32x4::from(other[e3215]) * self.group1().xyz().with_w(self[e12345]),
        )
    }
}
impl AntiWedge<AntiFlatPoint> for VersorEven {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * other.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e315] * self[e412]) + (other[e321] * self[e415]),
                (other[e125] * self[e423]) + (other[e321] * self[e425]),
                (other[e235] * self[e431]) + (other[e321] * self[e435]),
                -(other[e315] * self[e425]) - (other[e125] * self[e435]),
            ]) - (other.group0().zxyx() * self.group0().yzx().with_w(self[e415])),
        )
    }
}
impl AntiWedge<AntiFlector> for VersorEven {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       20        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * other.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e321] * self[e415]) + (other[e1] * self[e12345]),
                (other[e321] * self[e425]) + (other[e2] * self[e12345]),
                (other[e321] * self[e435]) + (other[e3] * self[e12345]),
                -(other[e315] * self[e425]) - (other[e125] * self[e435]),
            ]) + (self.group0().zxyw() * other.group0().yzx().with_w(other[e5]))
                - (other.group0().zxyx() * self.group0().yzx().with_w(self[e415])),
        )
    }
}
impl AntiWedge<AntiLine> for VersorEven {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn anti_wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e12345]) * other.group0()).with_w(
                -(other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435])
                    - (other[e15] * self[e423])
                    - (other[e25] * self[e431])
                    - (other[e35] * self[e412]),
            ),
            // e15, e25, e35, e3215
            (other.group1() * self.group0().www()).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiMotor> for VersorEven {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        8       11        0
    //  no simd       16       21        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x4::from(other[e3215]) * self.group0().xyz().with_w(self[e4]))
                + (Simd32x4::from(self[e12345]) * other.group0())
                + Simd32x3::from(0.0).with_w(
                    -(other[e23] * self[e415])
                        - (other[e31] * self[e425])
                        - (other[e12] * self[e435])
                        - (other[e15] * self[e423])
                        - (other[e25] * self[e431])
                        - (other[e35] * self[e412]),
                ),
            // e15, e25, e35, e3215
            ((Simd32x3::from(other[e3215]) * self.group1().xyz()) + (Simd32x3::from(self[e12345]) * other.group1().xyz())).with_w(other[e3215] * self[e12345]),
        )
    }
}
impl AntiWedge<AntiPlane> for VersorEven {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<AntiScalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(other[e12345]) * self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(other[e12345]) * self.group3(),
        )
    }
}
impl AntiWedge<Circle> for VersorEven {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       25       40        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1(),
            // e235, e315, e125, e4
            (Simd32x3::from(self[e12345]) * other.group2()).with_w(
                -(other[e423] * self[e415])
                    - (other[e431] * self[e425])
                    - (other[e412] * self[e435])
                    - (other[e415] * self[e423])
                    - (other[e425] * self[e431])
                    - (other[e435] * self[e412]),
            ),
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e412] * self[e315]) + (other[e415] * self[e321]) + (other[e321] * self[e415]) + (other[e315] * self[e412]),
                (other[e423] * self[e125]) + (other[e425] * self[e321]) + (other[e321] * self[e425]) + (other[e125] * self[e423]),
                (other[e431] * self[e235]) + (other[e435] * self[e321]) + (other[e321] * self[e435]) + (other[e235] * self[e431]),
                -(other[e415] * self[e235]) - (other[e425] * self[e315]) - (other[e435] * self[e125]) - (other[e125] * self[e435]),
            ]) - (other.group0().yzx() * self.group2().zxy()).with_w(other[e235] * self[e415])
                - (other.group2().zxy() * self.group0().yzx()).with_w(other[e315] * self[e425]),
        )
    }
}
impl AntiWedge<CircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       29        0
    //    simd3        1        5        0
    //    simd4        5        3        0
    // Totals...
    // yes simd       23       37        0
    //  no simd       40       56        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x3::from(other[e12345]) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * other.group0())).with_w(other[e12345] * self[e12345]),
            // e415, e425, e435, e321
            (Simd32x4::from(other[e12345]) * self.group1()) + (Simd32x4::from(self[e12345]) * other.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                other[e12345] * self[e235],
                other[e12345] * self[e315],
                other[e12345] * self[e125],
                -(other[e415] * self[e235])
                    - (other[e425] * self[e315])
                    - (other[e435] * self[e125])
                    - (other[e235] * self[e415])
                    - (other[e315] * self[e425])
                    - (other[e125] * self[e435]),
            ]) + (other.group2() * self.group0().www().with_w(self[e5])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e415] * self[e321]) + (other[e321] * self[e415]) + (other[e315] * self[e412]) + (other[e12345] * self[e1]),
                (other[e425] * self[e321]) + (other[e321] * self[e425]) + (other[e125] * self[e423]) + (other[e12345] * self[e2]),
                (other[e435] * self[e321]) + (other[e321] * self[e435]) + (other[e235] * self[e431]) + (other[e12345] * self[e3]),
                -(other[e412] * self[e435]) - (other[e415] * self[e423]) - (other[e425] * self[e431]) - (other[e435] * self[e412]),
            ]) + (other.group0().zxy() * self.group2().yzx()).with_w(other[e12345] * self[e4])
                - (other.group0().yzx() * self.group2().zxy()).with_w(other[e423] * self[e415])
                - (other.group2().zxy() * self.group0().yzx()).with_w(other[e431] * self[e425]),
        )
    }
}
impl AntiWedge<Dipole> for VersorEven {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd        9       20        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * other.group1(),
            // e15, e25, e35, scalar
            (Simd32x3::from(self[e12345]) * other.group2()).with_w(
                -(other[e41] * self[e235])
                    - (other[e42] * self[e315])
                    - (other[e43] * self[e125])
                    - (other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435])
                    - (other[e45] * self[e321])
                    - (other[e15] * self[e423])
                    - (other[e25] * self[e431])
                    - (other[e35] * self[e412]),
            ),
        )
    }
}
impl AntiWedge<DipoleInversion> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        0
    //    simd3        3        7        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       23       34        0
    //  no simd       47       60        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group3().yzxy() * self.group0().zxy().with_w(self[e2]))
                + Simd32x3::from(0.0).with_w(
                    (other[e4125] * self[e3]) + (other[e3215] * self[e4])
                        - (other[e42] * self[e315])
                        - (other[e43] * self[e125])
                        - (other[e23] * self[e415])
                        - (other[e31] * self[e425])
                        - (other[e12] * self[e435])
                        - (other[e45] * self[e321])
                        - (other[e15] * self[e423])
                        - (other[e25] * self[e431])
                        - (other[e35] * self[e412]),
                )
                + (other.group0() * self.group0().www()).with_w(other[e1234] * self[e5])
                + (self.group1().xyz() * other.group2().www()).with_w(other[e4235] * self[e1])
                - (other.group3().zxy() * self.group0().yzx()).with_w(other[e41] * self[e235]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other[e1234] * self[e235]) + (other[e3215] * self[e423]),
                (other[e1234] * self[e315]) + (other[e3215] * self[e431]),
                (other[e1234] * self[e125]) + (other[e3215] * self[e412]),
                -(other[e4315] * self[e425]) - (other[e4125] * self[e435]),
            ]) + (Simd32x4::from(self[e12345]) * other.group1())
                - (other.group3().xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e3215]) * self.group1().xyz()) + (Simd32x3::from(self[e12345]) * other.group2().xyz()) + (other.group3().zxy() * self.group2().yzx())
                - (other.group3().yzx() * self.group2().zxy()))
            .with_w(other[e1234] * self[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group3(),
        )
    }
}
impl AntiWedge<DualNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1       18        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group1(),
            // e235, e315, e125, e5
            other.group0().yy().with_zw(other[e12345], (other[e5] * self[e12345]) + (other[e12345] * self[e5])) * self.group2().xyz().with_w(1.0),
            // e1, e2, e3, e4
            Simd32x4::from(other[e12345]) * self.group3(),
        )
    }
}
impl AntiWedge<FlatPoint> for VersorEven {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3        8        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(other[e45] * self[e12345]),
            // e15, e25, e35, scalar
            (Simd32x3::from(self[e12345]) * other.group0().xyz())
                .with_w(-(other[e15] * self[e423]) - (other[e25] * self[e431]) - (other[e35] * self[e412]) - (other[e45] * self[e321])),
        )
    }
}
impl AntiWedge<Flector> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd3        0        4        0
    //    simd4        7        5        0
    // Totals...
    // yes simd       13       17        0
    //  no simd       34       40        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group1().yzxx() * self.group0().zxy().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w(
                    (other[e4315] * self[e2]) + (other[e4125] * self[e3]) + (other[e3215] * self[e4])
                        - (other[e25] * self[e431])
                        - (other[e35] * self[e412])
                        - (other[e45] * self[e321]),
                )
                - (self.group0().yzxx() * other.group1().zxy().with_w(other[e15])),
            // e23, e31, e12, e45
            (self.group0() * other.group1().www().with_w(other[e45])) + Simd32x3::from(0.0).with_w(-(other[e4315] * self[e425]) - (other[e4125] * self[e435]))
                - (other.group1().xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(other[e3215]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e12345]) * other.group0().xyz()).with_w(0.0)
                + (other.group1().zxy() * self.group2().yzx()).with_w(0.0)
                - (other.group1().yzx() * self.group2().zxy()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group1(),
        )
    }
}
impl AntiWedge<Line> for VersorEven {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       24        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (other.group0() * self.group0().www()).with_w(0.0),
            // e235, e315, e125, e4
            (Simd32x3::from(self[e12345]) * other.group1()).with_w(-(other[e415] * self[e423]) - (other[e425] * self[e431]) - (other[e435] * self[e412])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e415] * self[e321]) + (other[e315] * self[e412]),
                (other[e425] * self[e321]) + (other[e125] * self[e423]),
                (other[e435] * self[e321]) + (other[e235] * self[e431]),
                -(other[e425] * self[e315]) - (other[e435] * self[e125]) - (other[e235] * self[e415]) - (other[e315] * self[e425]) - (other[e125] * self[e435]),
            ]) - (other.group1().zxy() * self.group0().yzx()).with_w(other[e415] * self[e235]),
        )
    }
}
impl AntiWedge<Motor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        1        2        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       14       22        0
    //  no simd       28       41        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other[e12345]) * self.group0(),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e12345]) * self.group1().xyz()) + (Simd32x3::from(self[e12345]) * other.group0().xyz())).with_w(other[e12345] * self[e321]),
            // e235, e315, e125, e5
            (Simd32x4::from(other[e12345]) * self.group2())
                + (Simd32x4::from(self[e12345]) * other.group1())
                + Simd32x3::from(0.0).with_w(
                    -(other[e415] * self[e235])
                        - (other[e425] * self[e315])
                        - (other[e435] * self[e125])
                        - (other[e235] * self[e415])
                        - (other[e315] * self[e425])
                        - (other[e125] * self[e435]),
                ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e12345] * self[e1]) + (other[e315] * self[e412]),
                (other[e12345] * self[e2]) + (other[e125] * self[e423]),
                (other[e12345] * self[e3]) + (other[e235] * self[e431]),
                -(other[e425] * self[e431]) - (other[e435] * self[e412]),
            ]) + (other.group0() * self.group1().www().with_w(self[e4]))
                - (self.group0().yzxx() * other.group1().zxy().with_w(other[e415])),
        )
    }
}
impl AntiWedge<MultiVector> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       53        0
    //    simd3        8       15        0
    //    simd4        7        6        0
    // Totals...
    // yes simd       53       74        0
    //  no simd       90      122        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (other[scalar] * self[e12345])
                    + (other[e4235] * self[e1])
                    + (other[e4315] * self[e2])
                    + (other[e4125] * self[e3])
                    + (other[e3215] * self[e4])
                    + (other[e1234] * self[e5])
                    - (other[e15] * self[e423])
                    - (other[e25] * self[e431])
                    - (other[e35] * self[e412])
                    - (other[e45] * self[e321])
                    - (other[e41] * self[e235])
                    - (other[e42] * self[e315])
                    - (other[e43] * self[e125])
                    - (other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435]),
                other[e12345] * self[e12345],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e1] * self[e12345]) + (other[e415] * self[e321]) + (other[e321] * self[e415]) + (other[e315] * self[e412]),
                (other[e2] * self[e12345]) + (other[e425] * self[e321]) + (other[e321] * self[e425]) + (other[e125] * self[e423]),
                (other[e3] * self[e12345]) + (other[e435] * self[e321]) + (other[e321] * self[e435]) + (other[e235] * self[e431]),
                -(other[e415] * self[e423]) - (other[e425] * self[e431]) - (other[e435] * self[e412]) - (other[e412] * self[e435]),
            ]) + (Simd32x4::from(other[e12345]) * self.group3())
                + (other.group7().zxy() * self.group2().yzx()).with_w(other[e4] * self[e12345])
                - (other.group7().yzx() * self.group2().zxy()).with_w(other[e423] * self[e415])
                - (other.group8().zxy() * self.group0().yzx()).with_w(other[e431] * self[e425]),
            // e5
            (other[e12345] * self[e5]) + (other[e5] * self[e12345])
                - (other[e415] * self[e235])
                - (other[e425] * self[e315])
                - (other[e435] * self[e125])
                - (other[e235] * self[e415])
                - (other[e315] * self[e425])
                - (other[e125] * self[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (other[e4125] * self[e315]) + (other[e3215] * self[e415]),
                (other[e4235] * self[e125]) + (other[e3215] * self[e425]),
                (other[e4315] * self[e235]) + (other[e3215] * self[e435]),
                -(other[e4315] * self[e425]) - (other[e4125] * self[e435]),
            ]) + (Simd32x4::from(self[e12345]) * other.group3())
                - (other.group9().yzxx() * self.group2().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (Simd32x3::from(self[e12345]) * other.group4()) + (other.group9().yzx() * self.group0().zxy())
                - (other.group9().zxy() * self.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(other[e3215]) * self.group0().xyz()) + (Simd32x3::from(other[e1234]) * self.group2().xyz()) + (Simd32x3::from(self[e12345]) * other.group5())
                - (Simd32x3::from(self[e321]) * other.group9().xyz()),
            // e415, e425, e435, e321
            (Simd32x4::from(other[e12345]) * self.group1()) + (Simd32x4::from(self[e12345]) * other.group6()),
            // e423, e431, e412
            (Simd32x3::from(other[e12345]) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * other.group7()),
            // e235, e315, e125
            (Simd32x3::from(other[e12345]) * self.group2().xyz()) + (Simd32x3::from(self[e12345]) * other.group8()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group9(),
            // e1234
            other[e1234] * self[e12345],
        )
    }
}
impl AntiWedge<Plane> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       14        0
    //    simd3        0        3        0
    //    simd4        4        3        0
    // Totals...
    // yes simd        7       20        0
    //  no simd       19       35        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                other[e4125] * self[e431] * -1.0,
                other[e4235] * self[e412] * -1.0,
                other[e4315] * self[e423] * -1.0,
                (other[e4315] * self[e2]) + (other[e4125] * self[e3]) + (other[e3215] * self[e4]),
            ]) + (other.group0().yzxx() * self.group0().zxy().with_w(self[e1])),
            // e23, e31, e12, e45
            Simd32x4::from([
                other[e3215] * self[e423],
                other[e3215] * self[e431],
                other[e3215] * self[e412],
                -(other[e4315] * self[e425]) - (other[e4125] * self[e435]),
            ]) - (other.group0().xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(other[e3215]) * self.group1().xyz()).with_w(0.0) + (other.group0().zxy() * self.group2().yzx()).with_w(0.0)
                - (other.group0().yzx() * self.group2().zxy()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group0(),
        )
    }
}
impl AntiWedge<RoundPoint> for VersorEven {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e12345]) * other.group0(), /* e5 */ other[e5] * self[e12345])
    }
}
impl AntiWedge<Scalar> for VersorEven {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[scalar] * self[e12345])
    }
}
impl AntiWedge<Sphere> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       19        0
    //    simd3        2        4        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       11       26        0
    //  no simd       24       43        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                other[e4125] * self[e431] * -1.0,
                other[e4235] * self[e412] * -1.0,
                other[e4315] * self[e423] * -1.0,
                (other[e4125] * self[e3]) + (other[e3215] * self[e4]) + (other[e1234] * self[e5]),
            ]) + (other.group0().yzxx() * self.group0().zxy().with_w(self[e1]))
                + (Simd32x3::from(other[e1234]) * self.group1().xyz()).with_w(other[e4315] * self[e2]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other[e3215] * self[e423]) + (other[e1234] * self[e235]),
                (other[e3215] * self[e431]) + (other[e1234] * self[e315]),
                (other[e3215] * self[e412]) + (other[e1234] * self[e125]),
                -(other[e4315] * self[e425]) - (other[e4125] * self[e435]),
            ]) - (other.group0().xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e3215]) * self.group1().xyz()) + (other.group0().zxy() * self.group2().yzx()) - (other.group0().yzx() * self.group2().zxy()))
                .with_w(other[e1234] * self[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group0(),
        )
    }
}
impl AntiWedge<VersorEven> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       25        0
    //    simd3        1        4        0
    //    simd4        7        6        0
    // Totals...
    // yes simd       25       35        0
    //  no simd       48       61        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x3::from(other[e12345]) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * other.group0().xyz())).with_w(other[e12345] * self[e12345]),
            // e415, e425, e435, e321
            (Simd32x4::from(other[e12345]) * self.group1()) + (Simd32x4::from(self[e12345]) * other.group1()),
            // e235, e315, e125, e5
            (Simd32x4::from(other[e12345]) * self.group2())
                + (Simd32x4::from(self[e12345]) * other.group2())
                + Simd32x3::from(0.0).with_w(
                    -(other[e415] * self[e235])
                        - (other[e425] * self[e315])
                        - (other[e435] * self[e125])
                        - (other[e235] * self[e415])
                        - (other[e315] * self[e425])
                        - (other[e125] * self[e435]),
                ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e415] * self[e321]) + (other[e321] * self[e415]) + (other[e315] * self[e412]) + (other[e1] * self[e12345]),
                (other[e425] * self[e321]) + (other[e321] * self[e425]) + (other[e125] * self[e423]) + (other[e2] * self[e12345]),
                (other[e435] * self[e321]) + (other[e321] * self[e435]) + (other[e235] * self[e431]) + (other[e3] * self[e12345]),
                -(other[e412] * self[e435]) - (other[e415] * self[e423]) - (other[e425] * self[e431]) - (other[e435] * self[e412]),
            ]) + (other.group0().zxyw() * self.group2().yzx().with_w(self[e4]))
                + (self.group3().xyz() * other.group0().www()).with_w(other[e4] * self[e12345])
                - (other.group0().yzxx() * self.group2().zxy().with_w(self[e415]))
                - (other.group2().zxy() * self.group0().yzx()).with_w(other[e431] * self[e425]),
        )
    }
}
impl AntiWedge<VersorOdd> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       23        0
    //    simd3        3        6        0
    //    simd4        6        5        0
    // Totals...
    // yes simd       24       34        0
    //  no simd       48       61        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0().zxyw() * other.group3().yzx().with_w(other[scalar]))
                + Simd32x3::from(0.0).with_w(
                    (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e4] * other[e3215])
                        - (self[e431] * other[e25])
                        - (self[e412] * other[e35])
                        - (self[e415] * other[e23])
                        - (self[e425] * other[e31])
                        - (self[e435] * other[e12])
                        - (self[e321] * other[e45])
                        - (self[e235] * other[e41])
                        - (self[e315] * other[e42])
                        - (self[e125] * other[e43]),
                )
                + (self.group1().xyz() * other.group2().www()).with_w(self[e1] * other[e4235])
                + (other.group0().xyz() * self.group0().www()).with_w(self[e5] * other[e1234])
                - (self.group0().yzxx() * other.group3().zxy().with_w(other[e15])),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e12345] * other[e23]) + (self[e235] * other[e1234]),
                (self[e12345] * other[e31]) + (self[e315] * other[e1234]),
                (self[e12345] * other[e12]) + (self[e125] * other[e1234]),
                -(self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) + (self.group0() * other.group3().www().with_w(other[e45]))
                - (self.group1().wwwx() * other.group3().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e12345]) * other.group2().xyz()) + (Simd32x3::from(other[e3215]) * self.group1().xyz()) + (self.group2().yzx() * other.group3().zxy())
                - (self.group2().zxy() * other.group3().yzx()))
            .with_w(self[e12345] * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group3(),
        )
    }
}
impl std::ops::Div<AntiWedgeInfix> for VersorOdd {
    type Output = AntiWedgeInfixPartial<VersorOdd>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiCircleRotor> for VersorOdd {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       20        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(other[e41] * self[e3215]) - (other[e31] * self[e4125]),
                -(other[e42] * self[e3215]) - (other[e12] * self[e4235]),
                -(other[e43] * self[e3215]) - (other[e23] * self[e4315]),
                (other[e43] * self[e4125]) + (other[e45] * self[e1234]),
            ]) + (self.group3().yzxx() * other.group1().zxy().with_w(other[e41]))
                + (other.group2().xyz() * self.group2().www()).with_w(other[e42] * self[e4315]),
            // e5
            -(other[e45] * self[e3215]) - (other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]),
        )
    }
}
impl AntiWedge<AntiDipoleInversion> for VersorOdd {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        2        4        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       21       28        0
    //  no simd       37       45        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group1().xyz()) + (other.group0().zxy() * self.group3().yzx()) - (other.group0().yzx() * self.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other[e423] * self[e3215]) + (other[e235] * self[e1234]),
                (other[e431] * self[e3215]) + (other[e315] * self[e1234]),
                (other[e412] * self[e3215]) + (other[e125] * self[e1234]),
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) - (other.group1().wwwx() * self.group3().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(self[e3215]) * other.group1().xyz().with_w(other[e4]))
                + (self.group3().zxyx() * other.group2().yzx().with_w(other[e1]))
                + Simd32x3::from(0.0).with_w(
                    (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e5] * self[e1234])
                        - (other[e431] * self[e25])
                        - (other[e412] * self[e35])
                        - (other[e415] * self[e23])
                        - (other[e425] * self[e31])
                        - (other[e435] * self[e12])
                        - (other[e321] * self[e45])
                        - (other[e235] * self[e41])
                        - (other[e315] * self[e42])
                        - (other[e125] * self[e43]),
                )
                - (other.group2().zxy() * self.group3().yzx()).with_w(other[e423] * self[e15]),
        )
    }
}
impl AntiWedge<AntiDualNum> for VersorOdd {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e3215]) * self.group3().xyz().with_w(self[e1234]),
            // e1, e2, e3, e5
            Simd32x4::from(other[e3215]) * self.group0().xyz().with_w(self[e45]),
        )
    }
}
impl AntiWedge<AntiFlatPoint> for VersorOdd {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        9       16        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                other[e235] * self[e1234],
                other[e315] * self[e1234],
                other[e125] * self[e1234],
                -(other[e315] * self[e42]) - (other[e125] * self[e43]) - (other[e321] * self[e45]),
            ]) - (other.group0().wwwx() * self.group3().xyz().with_w(self[e41])),
            // e15, e25, e35, e3215
            ((other.group0().yzx() * self.group3().zxy()) - (other.group0().zxy() * self.group3().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiFlector> for VersorOdd {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        3        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        8       11        0
    //  no simd       16       20        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(
                (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e5] * self[e1234])
                    - (other[e315] * self[e42])
                    - (other[e125] * self[e43])
                    - (other[e321] * self[e45]),
            ) + (other.group0().xyz() * self.group2().www()).with_w(other[e1] * self[e4235])
                - (other.group0().wwwx() * self.group3().xyz().with_w(self[e41])),
            // e15, e25, e35, e3215
            ((other.group0().yzx() * self.group3().zxy()) - (other.group0().zxy() * self.group3().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiLine> for VersorOdd {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn anti_wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e12] * self[e4315]) + (other[e15] * self[e1234]),
                (other[e23] * self[e4125]) + (other[e25] * self[e1234]),
                (other[e31] * self[e4235]) + (other[e35] * self[e1234]),
                -(other[e25] * self[e4315]) - (other[e35] * self[e4125]),
            ]) - (self.group3().zxyx() * other.group0().yzx().with_w(other[e15])),
        )
    }
}
impl AntiWedge<AntiMotor> for VersorOdd {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       20        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e3215]) * self.group3().xyz().with_w(self[e1234]),
            // e1, e2, e3, e5
            Simd32x4::from([
                (other[e15] * self[e1234]) + (other[e3215] * self[e41]),
                (other[e25] * self[e1234]) + (other[e3215] * self[e42]),
                (other[e35] * self[e1234]) + (other[e3215] * self[e43]),
                -(other[e25] * self[e4315]) - (other[e35] * self[e4125]),
            ]) + (other.group0().zxy() * self.group3().yzx()).with_w(other[e3215] * self[e45])
                - (self.group3().zxyx() * other.group0().yzx().with_w(other[e15])),
        )
    }
}
impl AntiWedge<AntiPlane> for VersorOdd {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e5] * self[e1234]),
        )
    }
}
impl AntiWedge<AntiScalar> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[e12345]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group3(),
        )
    }
}
impl AntiWedge<Circle> for VersorOdd {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       24        0
    //    simd3        2        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       29       40        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group1().xyz()) + (other.group0().zxy() * self.group3().yzx()) - (other.group0().yzx() * self.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other[e423] * self[e3215]) + (other[e235] * self[e1234]),
                (other[e431] * self[e3215]) + (other[e315] * self[e1234]),
                (other[e412] * self[e3215]) + (other[e125] * self[e1234]),
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) - (other.group1().wwwx() * self.group3().xyzx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (other[e415] * self[e3215]) + (other[e315] * self[e4125]),
                (other[e425] * self[e3215]) + (other[e125] * self[e4235]),
                (other[e435] * self[e3215]) + (other[e235] * self[e4315]),
                -(other[e431] * self[e25])
                    - (other[e412] * self[e35])
                    - (other[e415] * self[e23])
                    - (other[e425] * self[e31])
                    - (other[e435] * self[e12])
                    - (other[e321] * self[e45])
                    - (other[e235] * self[e41])
                    - (other[e315] * self[e42])
                    - (other[e125] * self[e43]),
            ]) - (other.group2().zxy() * self.group3().yzx()).with_w(other[e423] * self[e15]),
        )
    }
}
impl AntiWedge<CircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       27        0
    //    simd3        3        7        0
    //    simd4        4        2        0
    // Totals...
    // yes simd       22       36        0
    //  no simd       40       56        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other[e415] * self[e1234]) + (other[e12345] * self[e41]),
                (other[e425] * self[e1234]) + (other[e12345] * self[e42]),
                (other[e435] * self[e1234]) + (other[e12345] * self[e43]),
                -(other[e431] * self[e25])
                    - (other[e412] * self[e35])
                    - (other[e415] * self[e23])
                    - (other[e425] * self[e31])
                    - (other[e435] * self[e12])
                    - (other[e321] * self[e45])
                    - (other[e235] * self[e41])
                    - (other[e315] * self[e42])
                    - (other[e125] * self[e43]),
            ]) + (other.group0().zxy() * self.group3().yzx()).with_w(other[e12345] * self[scalar])
                - (other.group0().yzx() * self.group3().zxy()).with_w(other[e423] * self[e15]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other[e235] * self[e1234]) + (other[e12345] * self[e23]),
                (other[e315] * self[e1234]) + (other[e12345] * self[e31]),
                (other[e125] * self[e1234]) + (other[e12345] * self[e12]),
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) + (other.group0() * self.group3().www()).with_w(other[e12345] * self[e45])
                - (other.group1().wwwx() * self.group3().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e12345]) * self.group2().xyz()) + (Simd32x3::from(self[e3215]) * other.group1().xyz()) + (other.group2().yzx() * self.group3().zxy())
                - (other.group2().zxy() * self.group3().yzx()))
            .with_w(other[e12345] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group3(),
        )
    }
}
impl AntiWedge<Dipole> for VersorOdd {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       20        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(other[e41] * self[e3215]) - (other[e31] * self[e4125]),
                -(other[e42] * self[e3215]) - (other[e12] * self[e4235]),
                -(other[e43] * self[e3215]) - (other[e23] * self[e4315]),
                (other[e43] * self[e4125]) + (other[e45] * self[e1234]),
            ]) + (self.group3().yzxy() * other.group1().zxy().with_w(other[e42]))
                + (other.group2() * self.group2().www()).with_w(other[e41] * self[e4235]),
            // e5
            -(other[e45] * self[e3215]) - (other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]),
        )
    }
}
impl AntiWedge<DipoleInversion> for VersorOdd {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        1        5        0
    //    simd4       10        9        0
    // Totals...
    // yes simd       16       23        0
    //  no simd       48       60        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * other.group3().xyz()) - (Simd32x3::from(other[e1234]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (other.group3().yzxw() * self.group3().zxy().with_w(self[e1234])) - (self.group3().yzxw() * other.group3().zxy().with_w(other[e1234])),
            // e235, e315, e125, e4
            (self.group3().xyzx() * other.group3().www().with_w(other[e41]))
                + Simd32x3::from(0.0).with_w(
                    (other[e42] * self[e4315]) + (other[e43] * self[e4125]) + (other[e45] * self[e1234])
                        - (other[e4235] * self[e41])
                        - (other[e4315] * self[e42])
                        - (other[e4125] * self[e43]),
                )
                - (other.group3().xyz() * self.group3().www()).with_w(other[e1234] * self[e45]),
            // e1, e2, e3, e5
            (Simd32x4::from(other[e3215]) * self.group0().xyz().with_w(self[e45]))
                + (other.group3().zxyz() * self.group1().yzx().with_w(self[e35]))
                + (self.group2().wwwy() * other.group2().xyz().with_w(other[e4315]))
                + (other.group1().zxy() * self.group3().yzx()).with_w(other[e4235] * self[e15])
                - (Simd32x4::from(self[e3215]) * other.group0().with_w(other[e45]))
                - (other.group2().wwwy() * self.group2().xyz().with_w(self[e4315]))
                - (self.group3().zxyx() * other.group1().yzx().with_w(other[e15]))
                - (other.group3().yzx() * self.group1().zxy()).with_w(other[e35] * self[e4125]),
        )
    }
}
impl AntiWedge<DualNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1       18        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0().yy().with_zw(other[e12345], (other[e5] * self[e1234]) + (other[e12345] * self[scalar])) * self.group0().xyz().with_w(1.0),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[e12345]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group3(),
        )
    }
}
impl AntiWedge<FlatPoint> for VersorOdd {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * other.group0(),
            // e5
            -(other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]) - (other[e45] * self[e3215]),
        )
    }
}
impl AntiWedge<Flector> for VersorOdd {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        1        5        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       11       18        0
    //  no simd       31       40        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * other.group1().xyz(),
            // e415, e425, e435, e321
            ((other.group1().yzx() * self.group3().zxy()) - (other.group1().zxy() * self.group3().yzx())).with_w(other[e3215] * self[e1234]),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(-(other[e4315] * self[e42]) - (other[e4125] * self[e43])) + (self.group3().xyz() * other.group1().www()).with_w(other[e45] * self[e1234])
                - (other.group1().xyzx() * self.group3().www().with_w(self[e41])),
            // e1, e2, e3, e5
            (other.group1().zxyy() * self.group1().yzx().with_w(self[e25]))
                + (other.group1().wwwz() * self.group0().xyz().with_w(self[e35]))
                + (self.group2().wwwx() * other.group0().xyz().with_w(other[e4235]))
                + Simd32x3::from(0.0).with_w((other[e3215] * self[e45]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]) - (other[e45] * self[e3215]))
                - (other.group1().yzx() * self.group1().zxy()).with_w(other[e15] * self[e4235]),
        )
    }
}
impl AntiWedge<Line> for VersorOdd {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       24        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group0(),
            // e23, e31, e12, e45
            (Simd32x3::from(self[e1234]) * other.group1()).with_w(-(other[e415] * self[e4235]) - (other[e425] * self[e4315]) - (other[e435] * self[e4125])),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (other[e415] * self[e3215]) + (other[e315] * self[e4125]),
                (other[e425] * self[e3215]) + (other[e125] * self[e4235]),
                (other[e435] * self[e3215]) + (other[e235] * self[e4315]),
                -(other[e425] * self[e31]) - (other[e435] * self[e12]) - (other[e235] * self[e41]) - (other[e315] * self[e42]) - (other[e125] * self[e43]),
            ]) - (other.group1().zxy() * self.group3().yzx()).with_w(other[e415] * self[e23]),
        )
    }
}
impl AntiWedge<Motor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd3        3        5        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       28       41        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0() * self.group2().www().with_w(self[scalar]))
                + Simd32x3::from(0.0).with_w(
                    -(other[e415] * self[e23])
                        - (other[e425] * self[e31])
                        - (other[e435] * self[e12])
                        - (other[e235] * self[e41])
                        - (other[e315] * self[e42])
                        - (other[e125] * self[e43]),
                )
                + (self.group0().xyz() * other.group0().www()).with_w(other[e5] * self[e1234]),
            // e23, e31, e12, e45
            Simd32x4::from([
                other[e235] * self[e1234],
                other[e315] * self[e1234],
                other[e125] * self[e1234],
                -(other[e415] * self[e4235]) - (other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) + (Simd32x4::from(other[e12345]) * self.group1()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e12345]) * self.group2().xyz()) + (Simd32x3::from(self[e3215]) * other.group0().xyz()) + (other.group1().yzx() * self.group3().zxy())
                - (other.group1().zxy() * self.group3().yzx()))
            .with_w(other[e12345] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group3(),
        )
    }
}
impl AntiWedge<MultiVector> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       39        0
    //    simd3        8       18        0
    //    simd4       10        7        0
    // Totals...
    // yes simd       44       64        0
    //  no simd       90      121        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (other[e12345] * self[scalar])
                    + (other[e1] * self[e4235])
                    + (other[e2] * self[e4315])
                    + (other[e3] * self[e4125])
                    + (other[e4] * self[e3215])
                    + (other[e5] * self[e1234])
                    - (other[e415] * self[e23])
                    - (other[e425] * self[e31])
                    - (other[e435] * self[e12])
                    - (other[e321] * self[e45])
                    - (other[e423] * self[e15])
                    - (other[e431] * self[e25])
                    - (other[e412] * self[e35])
                    - (other[e235] * self[e41])
                    - (other[e315] * self[e42])
                    - (other[e125] * self[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (self.group3().yzxx() * other.group5().zxy().with_w(other[e41]))
                + (other.group9().zxy() * self.group1().yzx()).with_w(other[e43] * self[e4125])
                + (other.group3().xyz() * self.group2().www()).with_w(other[e42] * self[e4315])
                + (self.group0().xyz() * other.group9().www()).with_w(other[e45] * self[e1234])
                - (Simd32x4::from(other[e1234]) * self.group2().xyz().with_w(self[e45]))
                - (other.group9().yzxz() * self.group1().zxy().with_w(self[e43]))
                - (other.group4() * self.group3().www()).with_w(other[e4235] * self[e41])
                - (other.group5().yzx() * self.group3().zxy()).with_w(other[e4315] * self[e42]),
            // e5
            (other[e4235] * self[e15]) + (other[e4315] * self[e25]) + (other[e4125] * self[e35]) + (other[e3215] * self[e45])
                - (other[e15] * self[e4235])
                - (other[e25] * self[e4315])
                - (other[e35] * self[e4125])
                - (other[e45] * self[e3215]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (other[e415] * self[e3215]) + (other[e315] * self[e4125]),
                (other[e425] * self[e3215]) + (other[e125] * self[e4235]),
                (other[e435] * self[e3215]) + (other[e235] * self[e4315]),
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) + (Simd32x4::from(other[e12345]) * self.group2().xyz().with_w(self[e45]))
                - (self.group3().yzxx() * other.group8().zxy().with_w(other[e415])),
            // e41, e42, e43
            (Simd32x3::from(other[e12345]) * self.group0().xyz()) + (Simd32x3::from(self[e1234]) * other.group6().xyz()) + (other.group7().zxy() * self.group3().yzx())
                - (other.group7().yzx() * self.group3().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[e12345]) * self.group1().xyz()) + (Simd32x3::from(self[e1234]) * other.group8()) + (Simd32x3::from(self[e3215]) * other.group7())
                - (Simd32x3::from(other[e321]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (other.group9().yzxw() * self.group3().zxy().with_w(self[e1234])) - (other.group9().zxy() * self.group3().yzx()).with_w(other[e1234] * self[e3215]),
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * other.group9().xyz()) - (Simd32x3::from(other[e1234]) * self.group3().xyz()),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group3().xyz()) - (Simd32x3::from(self[e3215]) * other.group9().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group3(),
            // e1234
            other[e12345] * self[e1234],
        )
    }
}
impl AntiWedge<Plane> for VersorOdd {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       14        0
    //    simd3        1        3        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        6       20        0
    //  no simd       17       35        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * other.group0().xyz(),
            // e415, e425, e435, e321
            ((other.group0().yzx() * self.group3().zxy()) - (other.group0().zxy() * self.group3().yzx())).with_w(other[e3215] * self[e1234]),
            // e235, e315, e125, e4
            Simd32x4::from([
                other[e3215] * self[e4235],
                other[e3215] * self[e4315],
                other[e3215] * self[e4125],
                -(other[e4315] * self[e42]) - (other[e4125] * self[e43]),
            ]) - (other.group0().xyzx() * self.group3().www().with_w(self[e41])),
            // e1, e2, e3, e5
            Simd32x4::from([
                other[e4315] * self[e12] * -1.0,
                other[e4125] * self[e23] * -1.0,
                other[e4235] * self[e31] * -1.0,
                (other[e4125] * self[e35]) + (other[e3215] * self[e45]),
            ]) + (other.group0().zxyx() * self.group1().yzx().with_w(self[e15]))
                + (other.group0().wwwy() * self.group0().xyz().with_w(self[e25])),
        )
    }
}
impl AntiWedge<RoundPoint> for VersorOdd {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e4] * self[e3215]) + (other[e5] * self[e1234]),
        )
    }
}
impl AntiWedge<Sphere> for VersorOdd {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       15        0
    //    simd3        1        3        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       11       22        0
    //  no simd       25       40        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * other.group0().xyz()) - (Simd32x3::from(other[e1234]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (other.group0().yzxw() * self.group3().zxy().with_w(self[e1234])) - (other.group0().zxy() * self.group3().yzx()).with_w(other[e1234] * self[e3215]),
            // e235, e315, e125, e4
            Simd32x4::from([
                other[e3215] * self[e4235],
                other[e3215] * self[e4315],
                other[e3215] * self[e4125],
                -(other[e4315] * self[e42]) - (other[e4125] * self[e43]) - (other[e1234] * self[e45]),
            ]) - (other.group0().xyzx() * self.group3().www().with_w(self[e41])),
            // e1, e2, e3, e5
            Simd32x4::from([
                -(other[e4315] * self[e12]) - (other[e1234] * self[e15]),
                -(other[e4125] * self[e23]) - (other[e1234] * self[e25]),
                -(other[e4235] * self[e31]) - (other[e1234] * self[e35]),
                (other[e4125] * self[e35]) + (other[e3215] * self[e45]),
            ]) + (other.group0().zxyx() * self.group1().yzx().with_w(self[e15]))
                + (other.group0().wwwy() * self.group0().xyz().with_w(self[e25])),
        )
    }
}
impl AntiWedge<VersorEven> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       23        0
    //    simd3        3        6        0
    //    simd4        6        5        0
    // Totals...
    // yes simd       24       34        0
    //  no simd       48       61        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0().zxyw() * self.group3().yzx().with_w(self[scalar]))
                + Simd32x3::from(0.0).with_w(
                    (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e4] * self[e3215])
                        - (other[e431] * self[e25])
                        - (other[e412] * self[e35])
                        - (other[e415] * self[e23])
                        - (other[e425] * self[e31])
                        - (other[e435] * self[e12])
                        - (other[e321] * self[e45])
                        - (other[e235] * self[e41])
                        - (other[e315] * self[e42])
                        - (other[e125] * self[e43]),
                )
                + (other.group1().xyz() * self.group2().www()).with_w(other[e1] * self[e4235])
                + (self.group0().xyz() * other.group0().www()).with_w(other[e5] * self[e1234])
                - (other.group0().yzxx() * self.group3().zxy().with_w(self[e15])),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other[e12345] * self[e23]) + (other[e235] * self[e1234]),
                (other[e12345] * self[e31]) + (other[e315] * self[e1234]),
                (other[e12345] * self[e12]) + (other[e125] * self[e1234]),
                -(other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) + (other.group0() * self.group3().www().with_w(self[e45]))
                - (other.group1().wwwx() * self.group3().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e12345]) * self.group2().xyz()) + (Simd32x3::from(self[e3215]) * other.group1().xyz()) + (other.group2().yzx() * self.group3().zxy())
                - (other.group2().zxy() * self.group3().yzx()))
            .with_w(other[e12345] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group3(),
        )
    }
}
impl AntiWedge<VersorOdd> for VersorOdd {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        1        5        0
    //    simd4       10        9        0
    // Totals...
    // yes simd       16       23        0
    //  no simd       48       60        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * other.group3().xyz()) - (Simd32x3::from(other[e1234]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (other.group3().yzxw() * self.group3().zxy().with_w(self[e1234])) - (self.group3().yzxw() * other.group3().zxy().with_w(other[e1234])),
            // e235, e315, e125, e4
            (self.group3().xyzx() * other.group3().www().with_w(other[e41]))
                + Simd32x3::from(0.0).with_w(
                    (other[e42] * self[e4315]) + (other[e43] * self[e4125]) + (other[e45] * self[e1234])
                        - (other[e4235] * self[e41])
                        - (other[e4315] * self[e42])
                        - (other[e4125] * self[e43]),
                )
                - (other.group3().xyz() * self.group3().www()).with_w(other[e1234] * self[e45]),
            // e1, e2, e3, e5
            (Simd32x4::from(other[e3215]) * self.group0().xyz().with_w(self[e45]))
                + (other.group3().zxyz() * self.group1().yzx().with_w(self[e35]))
                + (self.group2().wwwy() * other.group2().xyz().with_w(other[e4315]))
                + (other.group1().zxy() * self.group3().yzx()).with_w(other[e4235] * self[e15])
                - (Simd32x4::from(self[e3215]) * other.group0().xyz().with_w(other[e45]))
                - (other.group2().wwwy() * self.group2().xyz().with_w(self[e4315]))
                - (self.group3().zxyx() * other.group1().yzx().with_w(other[e15]))
                - (other.group3().yzx() * self.group1().zxy()).with_w(other[e35] * self[e4125]),
        )
    }
}
