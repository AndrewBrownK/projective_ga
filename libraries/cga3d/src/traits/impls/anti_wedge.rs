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
//   Median:         4       6       0     N/A
//  Average:         6       9       0     N/A
//  Maximum:       101     119       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       1       0       0
//   Median:         6      12       0       0
//  Average:        14      20       0       0
//  Maximum:       225     243       0       0
impl std::ops::Div<AntiWedgeInfix> for AntiCircleRotor {
    type Output = AntiWedgeInfixPartial<AntiCircleRotor>;
    fn div(self, _rhs: AntiWedgeInfix) -> Self::Output {
        AntiWedgeInfixPartial(self)
    }
}
impl AntiWedge<AntiDipoleInversion> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e3215]) * self.group0().with_w(self[e45]))
    }
}
impl AntiWedge<AntiFlatPoint> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e3215]) * self.group0().with_w(self[e45]))
    }
}
impl AntiWedge<AntiScalar> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       11        0        0
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
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
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
    //           add/sub      mul      div      pow
    //      f32       10       11        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       10       14        0      N/A
    //  no simd       10       21        0        0
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
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       20       20        0        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(self[e42] * other[e4315]) - (self[e43] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                + (self.group1().yzx() * other.group3().zxy()).with_w(0.0)
                - (Simd32x4::from(other[e1234]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                - (other.group3().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e5
            (self[e45] * other[e3215]) + (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
        )
    }
}
impl AntiWedge<DualNum> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       11        0        0
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
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        7        9        0      N/A
    //  no simd       16       16        0        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(self[e42] * other[e4315]) - (self[e43] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                + (self.group1().yzx() * other.group1().zxy()).with_w(0.0)
                - (other.group1().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e5
            (self[e45] * other[e3215]) + (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
        )
    }
}
impl AntiWedge<Line> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
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
    //           add/sub      mul      div      pow
    //      f32        6        7        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6       10        0      N/A
    //  no simd        6       17        0        0
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
    //           add/sub      mul      div      pow
    //      f32       14       17        0        0
    //    simd3        0        4        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       18       24        0      N/A
    //  no simd       30       41        0        0
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
            Simd32x3::from(0.0).with_w(-(self[e42] * other[e4315]) - (self[e43] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                + (self.group1().yzx() * other.group9().zxy()).with_w(0.0)
                - (Simd32x4::from(other[e1234]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                - (other.group9().yzxx() * self.group1().zxy().with_w(self[e41])),
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
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        7        9        0      N/A
    //  no simd       16       16        0        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(self[e42] * other[e4315]) - (self[e43] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                + (self.group1().yzx() * other.group0().zxy()).with_w(0.0)
                - (other.group0().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e5
            (self[e45] * other[e3215]) + (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
        )
    }
}
impl AntiWedge<Sphere> for AntiCircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       20       20        0        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(self[e42] * other[e4315]) - (self[e43] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                + (self.group1().yzx() * other.group0().zxy()).with_w(0.0)
                - (Simd32x4::from(other[e1234]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                - (other.group0().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e5
            (self[e45] * other[e3215]) + (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
        )
    }
}
impl AntiWedge<VersorEven> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       11        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       10       14        0      N/A
    //  no simd       10       21        0        0
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
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       20       20        0        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(self[e42] * other[e4315]) - (self[e43] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                + (self.group1().yzx() * other.group3().zxy()).with_w(0.0)
                - (Simd32x4::from(other[e1234]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                - (other.group3().yzxx() * self.group1().zxy().with_w(self[e41])),
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
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
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
    //           add/sub      mul      div      pow
    //      f32        8       12        0        0
    //    simd3        0        6        0      N/A
    //    simd4        6        0        0      N/A
    // Totals...
    // yes simd       14       18        0      N/A
    //  no simd       32       30        0        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(other[e412] * self[e435]) - (other[e415] * self[e423]) - (other[e425] * self[e431]) - (other[e435] * self[e412]))
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group2().yzx()).with_w(0.0)
                + (self.group0().zxy() * other.group2().yzx()).with_w(0.0)
                - (other.group0().yzx() * self.group2().zxy()).with_w(other[e423] * self[e415])
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
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        7        0        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other[e3215]) * self.group0().with_w(self[e4]),
            // e15, e25, e35, e3215
            (Simd32x3::from(other[e3215]) * self.group1().xyz()).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiFlatPoint> for AntiDipoleInversion {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e315]) - (self[e435] * other[e125]))
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (self.group0().zxy() * other.group0().yzx()).with_w(0.0)
                - (Simd32x4::from([self[e431], self[e412], self[e423], self[e415]]) * other.group0().zxyx()),
        )
    }
}
impl AntiWedge<AntiFlector> for AntiDipoleInversion {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e315]) - (self[e435] * other[e125]))
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (self.group0().zxy() * other.group0().yzx()).with_w(0.0)
                - (Simd32x4::from([self[e431], self[e412], self[e423], self[e415]]) * other.group0().zxyx()),
        )
    }
}
impl AntiWedge<AntiLine> for AntiDipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
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
    //           add/sub      mul      div      pow
    //      f32        6        7        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        6        9        0      N/A
    //  no simd        6       13        0        0
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
            (Simd32x3::from(other[e3215]) * self.group1().xyz()).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiScalar> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       15        0        0
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
    //           add/sub      mul      div      pow
    //      f32        8       12        0        0
    //    simd3        0        6        0      N/A
    //    simd4        6        0        0      N/A
    // Totals...
    // yes simd       14       18        0      N/A
    //  no simd       32       30        0        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(self[e412] * other[e435]) - (self[e415] * other[e423]) - (self[e425] * other[e431]) - (self[e435] * other[e412]))
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (self.group0().zxy() * other.group2().yzx()).with_w(0.0)
                + (other.group0().zxy() * self.group2().yzx()).with_w(0.0)
                - (self.group0().yzx() * other.group2().zxy()).with_w(self[e423] * other[e415])
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
    //           add/sub      mul      div      pow
    //      f32        9       11        0        0
    //    simd3        0        6        0      N/A
    //    simd4        7        4        0      N/A
    // Totals...
    // yes simd       16       21        0      N/A
    //  no simd       37       45        0        0
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
            (Simd32x4::from(other[e12345]) * self.group3())
                + Simd32x3::from(0.0).with_w(-(self[e425] * other[e315]) - (self[e435] * other[e125]) - (self[e315] * other[e425]) - (self[e125] * other[e435]))
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (self.group0().zxy() * other.group2().yzx()).with_w(0.0)
                + (other.group0().zxy() * self.group2().yzx()).with_w(0.0)
                - (Simd32x4::from([self[e431], self[e412], self[e423], self[e415]]) * other.group2().zxyx())
                - (Simd32x4::from([other[e431], other[e412], other[e423], other[e415]]) * self.group2().zxyx()),
        )
    }
}
impl AntiWedge<Dipole> for AntiDipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
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
    //           add/sub      mul      div      pow
    //      f32       12       14        0        0
    //    simd3        2        5        0      N/A
    //    simd4        6        4        0      N/A
    // Totals...
    // yes simd       20       23        0      N/A
    //  no simd       42       45        0        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (self.group0().zxy() * other.group3().yzx()) - (self.group0().yzx() * other.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125]))
                + (Simd32x3::from(other[e1234]) * self.group2().xyz()).with_w(0.0)
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                - (self.group1().wwwx() * other.group3().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(other[e3215]) * self.group1().xyz().with_w(self[e4]))
                + (other.group3().zxyx() * self.group2().yzx().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w(
                    (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e5] * other[e1234])
                        - (self[e423] * other[e15])
                        - (self[e431] * other[e25])
                        - (self[e412] * other[e35])
                        - (self[e415] * other[e23])
                        - (self[e425] * other[e31])
                        - (self[e435] * other[e12])
                        - (self[e321] * other[e45])
                        - (self[e315] * other[e42])
                        - (self[e125] * other[e43]),
                )
                - (self.group2().zxyx() * other.group3().yzx().with_w(other[e41])),
        )
    }
}
impl AntiWedge<DualNum> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       15        0        0
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
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        1        4        0      N/A
    //    simd4        5        3        0      N/A
    // Totals...
    // yes simd       12       15        0      N/A
    //  no simd       29       32        0        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (self.group0().zxy() * other.group1().yzx()) - (self.group0().yzx() * other.group1().zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125])) + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                - (self.group1().wwwx() * other.group1().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(other[e3215]) * self.group1().xyz().with_w(self[e4]))
                + (other.group1().zxyx() * self.group2().yzx().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w(
                    (self[e2] * other[e4315]) + (self[e3] * other[e4125])
                        - (self[e423] * other[e15])
                        - (self[e431] * other[e25])
                        - (self[e412] * other[e35])
                        - (self[e321] * other[e45]),
                )
                - (self.group2().zxy() * other.group1().yzx()).with_w(0.0),
        )
    }
}
impl AntiWedge<Line> for AntiDipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        0        3        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        9       12        0      N/A
    //  no simd       18       18        0        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(self[e431] * other[e425]) - (self[e412] * other[e435]))
                + (Simd32x3::from(self[e321]) * other.group0()).with_w(0.0)
                + (self.group0().zxy() * other.group1().yzx()).with_w(0.0)
                - (self.group0().yzx() * other.group1().zxy()).with_w(self[e423] * other[e415]),
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
    //           add/sub      mul      div      pow
    //      f32        7        9        0        0
    //    simd3        0        4        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       11       16        0      N/A
    //  no simd       23       33        0        0
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
            (other.group0() * Simd32x3::from(self[e321]).with_w(self[e5]))
                + Simd32x3::from(0.0)
                    .with_w(-(self[e425] * other[e315]) - (self[e435] * other[e125]) - (self[e235] * other[e415]) - (self[e315] * other[e425]) - (self[e125] * other[e435]))
                + (Simd32x3::from(other[e12345]) * self.group3().xyz()).with_w(0.0)
                + (self.group0().zxy() * other.group1().yzx()).with_w(0.0)
                - (Simd32x4::from([self[e431], self[e412], self[e423], self[e415]]) * other.group1().zxyx()),
        )
    }
}
impl AntiWedge<MultiVector> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       24       30        0        0
    //    simd3        4       16        0      N/A
    //    simd4       10        3        0      N/A
    // Totals...
    // yes simd       38       49        0      N/A
    //  no simd       76       90        0        0
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
            (Simd32x4::from(other[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]))
                + Simd32x3::from(0.0).with_w(-(self[e412] * other[e435]) - (self[e415] * other[e423]) - (self[e425] * other[e431]) - (self[e435] * other[e412]))
                + (Simd32x3::from(self[e321]) * other.group6().xyz()).with_w(0.0)
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (self.group0().zxy() * other.group8().yzx()).with_w(0.0)
                + (other.group7().zxy() * self.group2().yzx()).with_w(0.0)
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
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group1().xyz()).with_w(0.0)
                + (self.group2().yzx() * other.group9().zxy()).with_w(0.0)
                - (other.group9().yzxx() * self.group2().zxy().with_w(self[e415])),
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
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        1        4        0      N/A
    //    simd4        5        3        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       25       28        0        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (self.group0().zxy() * other.group0().yzx()) - (self.group0().yzx() * other.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125])) + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                - (self.group1().wwwx() * other.group0().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(other[e3215]) * self.group1().xyz().with_w(self[e4]))
                + (other.group0().zxyx() * self.group2().yzx().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w((self[e2] * other[e4315]) + (self[e3] * other[e4125]))
                - (self.group2().zxy() * other.group0().yzx()).with_w(0.0),
        )
    }
}
impl AntiWedge<Sphere> for AntiDipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        2        6        0      N/A
    //    simd4        6        3        0      N/A
    // Totals...
    // yes simd       11       14        0      N/A
    //  no simd       33       35        0        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (self.group0().zxy() * other.group0().yzx()) - (self.group0().yzx() * other.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                + (Simd32x3::from(other[e1234]) * self.group2().xyz()).with_w(0.0)
                - (self.group1().wwwx() * other.group0().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(other[e3215]) * self.group1().xyz().with_w(self[e4]))
                + (other.group0().zxyx() * self.group2().yzx().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w((self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e5] * other[e1234]))
                - (self.group2().zxy() * other.group0().yzx()).with_w(0.0),
        )
    }
}
impl AntiWedge<VersorEven> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       11        0        0
    //    simd3        0        6        0      N/A
    //    simd4        7        4        0      N/A
    // Totals...
    // yes simd       16       21        0      N/A
    //  no simd       37       45        0        0
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
            (Simd32x4::from(other[e12345]) * self.group3())
                + Simd32x3::from(0.0).with_w(-(self[e425] * other[e315]) - (self[e435] * other[e125]) - (self[e315] * other[e425]) - (self[e125] * other[e435]))
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (self.group0().zxy() * other.group2().yzx()).with_w(0.0)
                + (self.group2().yzx() * other.group0().zxy()).with_w(0.0)
                - (Simd32x4::from([self[e431], self[e412], self[e423], self[e415]]) * other.group2().zxyx())
                - (self.group2().zxyx() * other.group0().yzx().with_w(other[e415])),
        )
    }
}
impl AntiWedge<VersorOdd> for AntiDipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       14        0        0
    //    simd3        2        5        0      N/A
    //    simd4        6        4        0      N/A
    // Totals...
    // yes simd       20       23        0      N/A
    //  no simd       42       45        0        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (self.group0().zxy() * other.group3().yzx()) - (self.group0().yzx() * other.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125]))
                + (Simd32x3::from(other[e1234]) * self.group2().xyz()).with_w(0.0)
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                - (self.group1().wwwx() * other.group3().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(other[e3215]) * self.group1().xyz().with_w(self[e4]))
                + (other.group3().zxyx() * self.group2().yzx().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w(
                    (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e5] * other[e1234])
                        - (self[e423] * other[e15])
                        - (self[e431] * other[e25])
                        - (self[e412] * other[e35])
                        - (self[e415] * other[e23])
                        - (self[e425] * other[e31])
                        - (self[e435] * other[e12])
                        - (self[e321] * other[e45])
                        - (self[e315] * other[e42])
                        - (self[e125] * other[e43]),
                )
                - (self.group2().zxyx() * other.group3().yzx().with_w(other[e41])),
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
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e3215] * -1.0) * other.group0().with_w(other[e45]))
    }
}
impl AntiWedge<AntiDipoleInversion> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        7        0        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(self[e3215]) * other.group0().with_w(other[e4]),
            // e15, e25, e35, e3215
            (Simd32x3::from(self[e3215]) * other.group1().xyz()).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiScalar> for AntiDualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        0        1        0      N/A
    // no simd        0        2        0        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<Circle> for AntiDualNum {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0().xxxy() * other.group0().with_w(other[e12345]),
            // e15, e25, e35, e3215
            Simd32x4::from(self[e3215]) * other.group1().xyz().with_w(other[e12345]),
        )
    }
}
impl AntiWedge<Dipole> for AntiDualNum {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e3215] * -1.0) * other.group0().with_w(other[e45]))
    }
}
impl AntiWedge<DipoleInversion> for AntiDualNum {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215] * -1.0) * other.group3().xyz().with_w(other[e1234]),
            // e1, e2, e3, e5
            Simd32x4::from(self[e3215] * -1.0) * other.group0().with_w(other[e45]),
        )
    }
}
impl AntiWedge<DualNum> for AntiDualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        0        1        0      N/A
    // no simd        0        2        0        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<FlatPoint> for AntiDualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e3215] * other[e45] * -1.0, 0.0]))
    }
}
impl AntiWedge<Flector> for AntiDualNum {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        6        0        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (Simd32x3::from(self[e3215] * -1.0) * other.group1().xyz()).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e3215] * other[e45] * -1.0),
        )
    }
}
impl AntiWedge<Line> for AntiDualNum {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0))
    }
}
impl AntiWedge<Motor> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
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
    //           add/sub      mul      div      pow
    //      f32        1        9        0        0
    //    simd3        0        4        0      N/A
    // Totals...
    // yes simd        1       13        0      N/A
    //  no simd        1       21        0        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self[e3215] * other[e4]) + (self[scalar] * other[e12345]), 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(self[e3215] * -1.0) * other.group4()).with_w(0.0),
            // e5
            self[e3215] * other[e45] * -1.0,
            // e15, e25, e35, e45
            (Simd32x3::from(self[e3215]) * other.group6().xyz()).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e3215]) * other.group7(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e3215] * other[e1234] * -1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[e3215] * -1.0) * other.group9().xyz(),
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
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        4        0        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ (Simd32x3::from(self[e3215] * -1.0) * other.group0().xyz()).with_w(0.0))
    }
}
impl AntiWedge<RoundPoint> for AntiDualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e3215] * other[e4])
    }
}
impl AntiWedge<Sphere> for AntiDualNum {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(self[e3215] * -1.0) * other.group0().xyz().with_w(other[e1234]))
    }
}
impl AntiWedge<VersorEven> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        1        9        0        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e3215]) * other.group0().xyz()).with_w((self[e3215] * other[e4]) + (self[scalar] * other[e12345])),
            // e15, e25, e35, e3215
            Simd32x4::from(self[e3215]) * other.group1().xyz().with_w(other[e12345]),
        )
    }
}
impl AntiWedge<VersorOdd> for AntiDualNum {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215] * -1.0) * other.group3().xyz().with_w(other[e1234]),
            // e1, e2, e3, e5
            Simd32x4::from(self[e3215] * -1.0) * other.group0().xyz().with_w(other[e45]),
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
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e315]) - (other[e435] * self[e125]))
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
                - (Simd32x4::from([other[e431], other[e412], other[e423], other[e415]]) * self.group0().zxyx()),
        )
    }
}
impl AntiWedge<AntiScalar> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<Circle> for AntiFlatPoint {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(self[e315] * other[e425]) - (self[e125] * other[e435]))
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
                - (Simd32x4::from([other[e431], other[e412], other[e423], other[e415]]) * self.group0().zxyx()),
        )
    }
}
impl AntiWedge<CircleRotor> for AntiFlatPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       13       16        0        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e12345]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(self[e315] * other[e425]) - (self[e125] * other[e435]))
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
                - (Simd32x4::from([other[e431], other[e412], other[e423], other[e415]]) * self.group0().zxyx()),
        )
    }
}
impl AntiWedge<Dipole> for AntiFlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       13       16        0        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(self[e235] * other[e41]) - (self[e315] * other[e42]) - (self[e125] * other[e43]))
                + (Simd32x3::from(other[e1234]) * self.group0().xyz()).with_w(0.0)
                - (Simd32x4::from(self[e321]) * other.group3().xyz().with_w(other[e45])),
            // e15, e25, e35, e3215
            ((self.group0().yzx() * other.group3().zxy()) - (self.group0().zxy() * other.group3().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<DualNum> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<FlatPoint> for AntiFlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * other[e45] * -1.0)
    }
}
impl AntiWedge<Flector> for AntiFlatPoint {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        1        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        3       11        0        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321] * -1.0) * other.group1().xyz().with_w(other[e45]),
            // e15, e25, e35, e3215
            ((self.group0().yzx() * other.group1().zxy()) - (self.group0().zxy() * other.group1().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<Line> for AntiFlatPoint {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
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
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2       10        0        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e12345]) * self.group0(),
            // e1, e2, e3, e5
            (Simd32x3::from(self[e321]) * other.group0().xyz()).with_w(-(self[e235] * other[e415]) - (self[e315] * other[e425]) - (self[e125] * other[e435])),
        )
    }
}
impl AntiWedge<MultiVector> for AntiFlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        8        0        0
    //    simd3        2        8        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        9       16        0      N/A
    //  no simd       19       32        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        2        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        6       11        0        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(self[e321] * -1.0) * other.group0().xyz(),
            // e15, e25, e35
            (self.group0().yzx() * other.group0().zxy()) + Simd32x2::from(0.0).with_z((self[e315] * other[e4235]) * -1.0) - (self.group0().zx() * other.group0().yz()).with_z(0.0),
        )
    }
}
impl AntiWedge<Sphere> for AntiFlatPoint {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        3        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        9       13        0        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            (Simd32x3::from(other[e1234]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * other.group0().xyz()),
            // e15, e25, e35
            (self.group0().yzx() * other.group0().zxy()) + Simd32x2::from(0.0).with_z((self[e315] * other[e4235]) * -1.0) - (self.group0().zx() * other.group0().yz()).with_z(0.0),
        )
    }
}
impl AntiWedge<VersorEven> for AntiFlatPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       13       16        0        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e12345]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(self[e315] * other[e425]) - (self[e125] * other[e435]))
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (self.group0().yzx() * other.group0().zxy()).with_w(0.0)
                - (self.group0().zxyx() * other.group0().yzx().with_w(other[e415])),
        )
    }
}
impl AntiWedge<VersorOdd> for AntiFlatPoint {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       13       16        0        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(self[e235] * other[e41]) - (self[e315] * other[e42]) - (self[e125] * other[e43]))
                + (Simd32x3::from(other[e1234]) * self.group0().xyz()).with_w(0.0)
                - (Simd32x4::from(self[e321]) * other.group3().xyz().with_w(other[e45])),
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
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e315]) - (other[e435] * self[e125]))
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
                - (Simd32x4::from([other[e431], other[e412], other[e423], other[e415]]) * self.group0().zxyx()),
        )
    }
}
impl AntiWedge<AntiScalar> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
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
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(self[e315] * other[e425]) - (self[e125] * other[e435]))
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
                - (Simd32x4::from([other[e431], other[e412], other[e423], other[e415]]) * self.group0().zxyx()),
        )
    }
}
impl AntiWedge<CircleRotor> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       17       20        0        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e12345]) * self.group0(),
            // e1, e2, e3, e5
            (Simd32x4::from(other[e12345]) * self.group1())
                + Simd32x3::from(0.0).with_w(-(self[e315] * other[e425]) - (self[e125] * other[e435]))
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
                - (Simd32x4::from([other[e431], other[e412], other[e423], other[e415]]) * self.group0().zxyx()),
        )
    }
}
impl AntiWedge<Dipole> for AntiFlector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       16       20        0        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x4::from(other[e1234]) * self.group0().xyz().with_w(self[e5]))
                + Simd32x3::from(0.0).with_w(
                    (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125])
                        - (self[e235] * other[e41])
                        - (self[e315] * other[e42])
                        - (self[e125] * other[e43]),
                )
                - (Simd32x4::from(self[e321]) * other.group3().xyz().with_w(other[e45])),
            // e15, e25, e35, e3215
            ((self.group0().yzx() * other.group3().zxy()) - (self.group0().zxy() * other.group3().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<DualNum> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
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
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * other[e45] * -1.0)
    }
}
impl AntiWedge<Flector> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        6       14        0        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e321] * -1.0) * other.group1().xyz())
                .with_w((self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]) - (self[e321] * other[e45])),
            // e15, e25, e35, e3215
            ((self.group0().yzx() * other.group1().zxy()) - (self.group0().zxy() * other.group1().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<Line> for AntiFlector {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
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
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       10       14        0        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e12345]) * self.group0(),
            // e1, e2, e3, e5
            (other.group0() * Simd32x3::from(self[e321]).with_w(self[e5]))
                + Simd32x3::from(0.0).with_w(-(self[e235] * other[e415]) - (self[e315] * other[e425]) - (self[e125] * other[e435]))
                + (Simd32x3::from(other[e12345]) * self.group1().xyz()).with_w(0.0),
        )
    }
}
impl AntiWedge<MultiVector> for AntiFlector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       13        0        0
    //    simd3        2        9        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd       15       22        0      N/A
    //  no simd       28       40        0        0
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
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        5       13        0        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e321] * -1.0) * other.group0().xyz()).with_w((self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125])),
            // e15, e25, e35, e3215
            ((self.group0().yzx() * other.group0().zxy()) - (self.group0().zxy() * other.group0().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<Sphere> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       13       16        0        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x4::from(other[e1234]) * self.group0().xyz().with_w(self[e5]))
                + Simd32x3::from(0.0).with_w((self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]))
                - (Simd32x3::from(self[e321]) * other.group0().xyz()).with_w(0.0),
            // e15, e25, e35, e3215
            ((self.group0().yzx() * other.group0().zxy()) - (self.group0().zxy() * other.group0().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<VersorEven> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       17       20        0        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e12345]) * self.group0(),
            // e1, e2, e3, e5
            (Simd32x4::from(other[e12345]) * self.group1())
                + Simd32x3::from(0.0).with_w(-(self[e315] * other[e425]) - (self[e125] * other[e435]))
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (self.group0().yzx() * other.group0().zxy()).with_w(0.0)
                - (self.group0().zxyx() * other.group0().yzx().with_w(other[e415])),
        )
    }
}
impl AntiWedge<VersorOdd> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       16       20        0        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x4::from(other[e1234]) * self.group0().xyz().with_w(self[e5]))
                + Simd32x3::from(0.0).with_w(
                    (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125])
                        - (self[e235] * other[e41])
                        - (self[e315] * other[e42])
                        - (self[e125] * other[e43]),
                )
                - (Simd32x4::from(self[e321]) * other.group3().xyz().with_w(other[e45])),
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
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
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
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
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
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
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
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       12        0        0
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
            (Simd32x3::from(other[e12345]) * self.group1()).with_w(0.0),
        )
    }
}
impl AntiWedge<DipoleInversion> for AntiLine {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x4::from([self[e31], self[e12], self[e23], self[e15]]) * other.group3().zxyx())
                + Simd32x3::from(0.0).with_w((self[e25] * other[e4315]) + (self[e35] * other[e4125]))
                - (Simd32x3::from(other[e1234]) * self.group1()).with_w(0.0)
                - (self.group0().zxy() * other.group3().yzx()).with_w(0.0),
        )
    }
}
impl AntiWedge<DualNum> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
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
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        3        4        0      N/A
    //  no simd        9        9        0        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x4::from([self[e31], self[e12], self[e23], self[e15]]) * other.group1().zxyx())
                + Simd32x3::from(0.0).with_w((self[e25] * other[e4315]) + (self[e35] * other[e4125]))
                - (self.group0().zxy() * other.group1().yzx()).with_w(0.0),
        )
    }
}
impl AntiWedge<Line> for AntiLine {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -(self[e23] * other[e415]) - (self[e31] * other[e425]) - (self[e12] * other[e435]))
    }
}
impl AntiWedge<Motor> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        9        0        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(other[e12345]) * self.group0()).with_w(-(self[e23] * other[e415]) - (self[e31] * other[e425]) - (self[e12] * other[e435])),
            // e15, e25, e35, e3215
            (Simd32x3::from(other[e12345]) * self.group1()).with_w(0.0),
        )
    }
}
impl AntiWedge<MultiVector> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7        9        0        0
    //    simd3        0        5        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd       15       24        0        0
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
            (Simd32x3::from(other[e12345]) * self.group1()).with_w(0.0),
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
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        3        4        0      N/A
    //  no simd        9        9        0        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x4::from([self[e31], self[e12], self[e23], self[e15]]) * other.group0().zxyx())
                + Simd32x3::from(0.0).with_w((self[e25] * other[e4315]) + (self[e35] * other[e4125]))
                - (self.group0().zxy() * other.group0().yzx()).with_w(0.0),
        )
    }
}
impl AntiWedge<Sphere> for AntiLine {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x4::from([self[e31], self[e12], self[e23], self[e15]]) * other.group0().zxyx())
                + Simd32x3::from(0.0).with_w((self[e25] * other[e4315]) + (self[e35] * other[e4125]))
                - (Simd32x3::from(other[e1234]) * self.group1()).with_w(0.0)
                - (self.group0().zxy() * other.group0().yzx()).with_w(0.0),
        )
    }
}
impl AntiWedge<VersorEven> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       12        0        0
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
            (Simd32x3::from(other[e12345]) * self.group1()).with_w(0.0),
        )
    }
}
impl AntiWedge<VersorOdd> for AntiLine {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x4::from([self[e31], self[e12], self[e23], self[e15]]) * other.group3().zxyx())
                + Simd32x3::from(0.0).with_w((self[e25] * other[e4315]) + (self[e35] * other[e4125]))
                - (Simd32x3::from(other[e1234]) * self.group1()).with_w(0.0)
                - (self.group0().zxy() * other.group3().yzx()).with_w(0.0),
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
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e3215] * -1.0) * other.group0().with_w(other[e45]))
    }
}
impl AntiWedge<AntiDipoleInversion> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        7        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        6        9        0      N/A
    //  no simd        6       13        0        0
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
            (Simd32x3::from(self[e3215]) * other.group1().xyz()).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiScalar> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
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
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       12        0        0
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
            (Simd32x3::from(self[e3215]) * other.group1().xyz()).with_w(0.0),
        )
    }
}
impl AntiWedge<CircleRotor> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       16       20        0        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x4::from(other[e12345]) * self.group0())
                + Simd32x3::from(0.0).with_w(
                    -(self[e23] * other[e415])
                        - (self[e31] * other[e425])
                        - (self[e12] * other[e435])
                        - (self[e15] * other[e423])
                        - (self[e25] * other[e431])
                        - (self[e35] * other[e412]),
                )
                + (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0),
            // e15, e25, e35, e3215
            ((Simd32x3::from(self[e3215]) * other.group1().xyz()) + (Simd32x3::from(other[e12345]) * self.group1().xyz())).with_w(self[e3215] * other[e12345]),
        )
    }
}
impl AntiWedge<Dipole> for AntiMotor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e3215] * -1.0) * other.group0().with_w(other[e45]))
    }
}
impl AntiWedge<DipoleInversion> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd       17       21        0        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215] * -1.0) * other.group3().xyz().with_w(other[e1234]),
            // e1, e2, e3, e5
            (other.group3().zxyx() * self.group0().yzx().with_w(self[e15])) + Simd32x3::from(0.0).with_w((self[e25] * other[e4315]) + (self[e35] * other[e4125]))
                - (self.group1() * Simd32x3::from(other[e1234]).with_w(other[e45]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (self.group0().zxy() * other.group3().yzx()).with_w(0.0),
        )
    }
}
impl AntiWedge<DualNum> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
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
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e3215] * other[e45] * -1.0, 0.0]))
    }
}
impl AntiWedge<Flector> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        2        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd       10       14        0        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (Simd32x3::from(self[e3215] * -1.0) * other.group1().xyz()).with_w(0.0),
            // e1, e2, e3, e5
            (other.group1().zxyx() * self.group0().yzx().with_w(self[e15]))
                + Simd32x3::from(0.0).with_w((self[e25] * other[e4315]) + (self[e35] * other[e4125]) - (self[e3215] * other[e45]))
                - (self.group0().zxy() * other.group1().yzx()).with_w(0.0),
        )
    }
}
impl AntiWedge<Line> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(self[e23] * other[e415]) - (self[e31] * other[e425]) - (self[e12] * other[e435])),
            // e15, e25, e35, e3215
            (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0),
        )
    }
}
impl AntiWedge<Motor> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        6       14        0        0
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
    //           add/sub      mul      div      pow
    //      f32       10       16        0        0
    //    simd3        2        9        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd       15       25        0      N/A
    //  no simd       28       43        0        0
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
            Simd32x3::from(0.0).with_w(self[e3215] * other[e1234] * -1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[e3215] * -1.0) * other.group9().xyz(),
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
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        9       13        0        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (Simd32x3::from(self[e3215] * -1.0) * other.group0().xyz()).with_w(0.0),
            // e1, e2, e3, e5
            (other.group0().zxyx() * self.group0().yzx().with_w(self[e15])) + Simd32x3::from(0.0).with_w((self[e25] * other[e4315]) + (self[e35] * other[e4125]))
                - (self.group0().zxy() * other.group0().yzx()).with_w(0.0),
        )
    }
}
impl AntiWedge<RoundPoint> for AntiMotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e3215] * other[e4])
    }
}
impl AntiWedge<Sphere> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd       13       17        0        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215] * -1.0) * other.group0().xyz().with_w(other[e1234]),
            // e1, e2, e3, e5
            (other.group0().zxyx() * self.group0().yzx().with_w(self[e15])) + Simd32x3::from(0.0).with_w((self[e25] * other[e4315]) + (self[e35] * other[e4125]))
                - (Simd32x3::from(other[e1234]) * self.group1().xyz()).with_w(0.0)
                - (self.group0().zxy() * other.group0().yzx()).with_w(0.0),
        )
    }
}
impl AntiWedge<VersorEven> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       16       21        0        0
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
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd       17       21        0        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215] * -1.0) * other.group3().xyz().with_w(other[e1234]),
            // e1, e2, e3, e5
            (other.group3().zxyx() * self.group0().yzx().with_w(self[e15])) + Simd32x3::from(0.0).with_w((self[e25] * other[e4315]) + (self[e35] * other[e4125]))
                - (self.group1() * Simd32x3::from(other[e1234]).with_w(other[e45]))
                - (Simd32x3::from(self[e3215]) * other.group0().xyz()).with_w(0.0)
                - (self.group0().zxy() * other.group3().yzx()).with_w(0.0),
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
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<CircleRotor> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<DipoleInversion> for AntiPlane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<Flector> for AntiPlane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]))
    }
}
impl AntiWedge<Motor> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<MultiVector> for AntiPlane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3        8        0        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e5] * other[e1234]), 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(other[e12345]) * self.group0().xyz()).with_w(0.0),
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
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]))
    }
}
impl AntiWedge<Sphere> for AntiPlane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<VersorOdd> for AntiPlane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       11        0        0
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
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       15        0        0
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
    //          add/sub      mul      div      pow
    //   simd2        0        1        0      N/A
    // no simd        0        2        0        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<AntiFlatPoint> for AntiScalar {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<AntiFlector> for AntiScalar {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
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
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<AntiScalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[e12345] * self[e12345])
    }
}
impl AntiWedge<Circle> for AntiScalar {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       10        0        0
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
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       11        0        0
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
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       10        0        0
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
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       15        0        0
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
    //          add/sub      mul      div      pow
    //   simd2        0        1        0      N/A
    // no simd        0        2        0        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<FlatPoint> for AntiScalar {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<Flector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
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
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        4        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0       11        0      N/A
    //  no simd        0       32        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<RoundPoint> for AntiScalar {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e12345]) * other.group0(), /* e5 */ self[e12345] * other[e5])
    }
}
impl AntiWedge<Scalar> for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e12345] * other[scalar])
    }
}
impl AntiWedge<Sphere> for AntiScalar {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
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
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
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
    //           add/sub      mul      div      pow
    //      f32        8       12        0        0
    //    simd3        0        6        0      N/A
    //    simd4        6        0        0      N/A
    // Totals...
    // yes simd       14       18        0      N/A
    //  no simd       32       30        0        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(other[e412] * self[e435]) - (other[e415] * self[e423]) - (other[e425] * self[e431]) - (other[e435] * self[e412]))
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group2().yzx()).with_w(0.0)
                + (self.group0().zxy() * other.group2().yzx()).with_w(0.0)
                - (other.group0().yzx() * self.group2().zxy()).with_w(other[e423] * self[e415])
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
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
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
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(other[e315] * self[e425]) - (other[e125] * self[e435]))
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (self.group0().zxy() * other.group0().yzx()).with_w(0.0)
                - (Simd32x4::from([self[e431], self[e412], self[e423], self[e415]]) * other.group0().zxyx()),
        )
    }
}
impl AntiWedge<AntiFlector> for Circle {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(other[e315] * self[e425]) - (other[e125] * self[e435]))
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (self.group0().zxy() * other.group0().yzx()).with_w(0.0)
                - (Simd32x4::from([self[e431], self[e412], self[e423], self[e415]]) * other.group0().zxyx()),
        )
    }
}
impl AntiWedge<AntiLine> for Circle {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
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
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       12        0        0
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
            (Simd32x3::from(other[e3215]) * self.group1().xyz()).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiScalar> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       10        0        0
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
    //           add/sub      mul      div      pow
    //      f32        8       12        0        0
    //    simd3        0        6        0      N/A
    //    simd4        6        0        0      N/A
    // Totals...
    // yes simd       14       18        0      N/A
    //  no simd       32       30        0        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(other[e412] * self[e435]) - (other[e415] * self[e423]) - (other[e425] * self[e431]) - (other[e435] * self[e412]))
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group2().yzx()).with_w(0.0)
                + (other.group2().yzx() * self.group0().zxy()).with_w(0.0)
                - (other.group0().yzx() * self.group2().zxy()).with_w(other[e423] * self[e415])
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
    //           add/sub      mul      div      pow
    //      f32        8       11        0        0
    //    simd3        0        7        0      N/A
    //    simd4        6        2        0      N/A
    // Totals...
    // yes simd       14       20        0      N/A
    //  no simd       32       40        0        0
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
            Simd32x3::from(0.0).with_w(-(self[e435] * other[e125]) - (self[e235] * other[e415]) - (self[e315] * other[e425]) - (self[e125] * other[e435]))
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (self.group0().zxy() * other.group2().yzx()).with_w(0.0)
                + (self.group2().yzx() * other.group0().zxy()).with_w(0.0)
                - (Simd32x4::from([self[e431], self[e412], self[e423], self[e415]]) * other.group2().zxyx())
                - (self.group2().zxy() * other.group0().yzx()).with_w(self[e425] * other[e315]),
        )
    }
}
impl AntiWedge<Dipole> for Circle {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
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
    //           add/sub      mul      div      pow
    //      f32        9       12        0        0
    //    simd3        2        8        0      N/A
    //    simd4        6        1        0      N/A
    // Totals...
    // yes simd       17       21        0      N/A
    //  no simd       39       40        0        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (self.group0().zxy() * other.group3().yzx()) - (self.group0().yzx() * other.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125]))
                + (Simd32x3::from(other[e1234]) * self.group2()).with_w(0.0)
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                - (self.group1().wwwx() * other.group3().xyzx()),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(
                -(self[e431] * other[e25])
                    - (self[e412] * other[e35])
                    - (self[e415] * other[e23])
                    - (self[e425] * other[e31])
                    - (self[e435] * other[e12])
                    - (self[e321] * other[e45])
                    - (self[e235] * other[e41])
                    - (self[e315] * other[e42])
                    - (self[e125] * other[e43]),
            ) + (Simd32x3::from(other[e3215]) * self.group1().xyz()).with_w(0.0)
                + (self.group2().yzx() * other.group3().zxy()).with_w(0.0)
                - (self.group2().zxy() * other.group3().yzx()).with_w(self[e423] * other[e15]),
        )
    }
}
impl AntiWedge<DualNum> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       10        0        0
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
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        1        6        0      N/A
    //    simd4        5        1        0      N/A
    // Totals...
    // yes simd        9       13        0      N/A
    //  no simd       26       28        0        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (self.group0().zxy() * other.group1().yzx()) - (self.group0().yzx() * other.group1().zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125])) + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                - (self.group1().wwwx() * other.group1().xyzx()),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(-(self[e431] * other[e25]) - (self[e412] * other[e35]) - (self[e321] * other[e45]))
                + (Simd32x3::from(other[e3215]) * self.group1().xyz()).with_w(0.0)
                + (self.group2().yzx() * other.group1().zxy()).with_w(0.0)
                - (self.group2().zxy() * other.group1().yzx()).with_w(self[e423] * other[e15]),
        )
    }
}
impl AntiWedge<Line> for Circle {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        0        3        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        9       12        0      N/A
    //  no simd       18       18        0        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(self[e431] * other[e425]) - (self[e412] * other[e435]))
                + (Simd32x3::from(self[e321]) * other.group0()).with_w(0.0)
                + (self.group0().zxy() * other.group1().yzx()).with_w(0.0)
                - (self.group0().yzx() * other.group1().zxy()).with_w(self[e423] * other[e415]),
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
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        0        4        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd       18       28        0        0
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
            Simd32x3::from(0.0)
                .with_w(-(self[e425] * other[e315]) - (self[e435] * other[e125]) - (self[e235] * other[e415]) - (self[e315] * other[e425]) - (self[e125] * other[e435]))
                + (Simd32x3::from(self[e321]) * other.group0().xyz()).with_w(0.0)
                + (self.group0().zxy() * other.group1().yzx()).with_w(0.0)
                - (Simd32x4::from([self[e431], self[e412], self[e423], self[e415]]) * other.group1().zxyx()),
        )
    }
}
impl AntiWedge<MultiVector> for Circle {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       18       24        0        0
    //    simd3        4       16        0      N/A
    //    simd4        9        2        0      N/A
    // Totals...
    // yes simd       31       42        0      N/A
    //  no simd       66       80        0        0
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
            Simd32x3::from(0.0).with_w(-(self[e412] * other[e435]) - (self[e415] * other[e423]) - (self[e425] * other[e431]) - (self[e435] * other[e412]))
                + (Simd32x3::from(self[e321]) * other.group6().xyz()).with_w(0.0)
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (self.group0().zxy() * other.group8().yzx()).with_w(0.0)
                + (self.group2().yzx() * other.group7().zxy()).with_w(0.0)
                - (self.group0().yzx() * other.group8().zxy()).with_w(self[e423] * other[e415])
                - (self.group2().zxy() * other.group7().yzx()).with_w(self[e431] * other[e425]),
            // e5
            -(self[e415] * other[e235])
                - (self[e425] * other[e315])
                - (self[e435] * other[e125])
                - (self[e235] * other[e415])
                - (self[e315] * other[e425])
                - (self[e125] * other[e435]),
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group1().xyz()).with_w(0.0)
                + (self.group2().yzx() * other.group9().zxy()).with_w(0.0)
                - (other.group9().yzxx() * self.group2().zxy().with_w(self[e415])),
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
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        3        6        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        6        9        0      N/A
    //  no simd       18       24        0        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            (self.group0().zxy() * other.group0().yzx()) - (self.group0().yzx() * other.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125])) + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                - (self.group1().wwwx() * other.group0().xyzx()),
            // e15, e25, e35
            (Simd32x3::from(other[e3215]) * self.group1().xyz()) + (self.group2().yzx() * other.group0().zxy()) - (self.group2().zxy() * other.group0().yzx()),
        )
    }
}
impl AntiWedge<Sphere> for Circle {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        4        8        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       25       30        0        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (self.group0().zxy() * other.group0().yzx()) - (self.group0().yzx() * other.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                + (Simd32x3::from(other[e1234]) * self.group2()).with_w(0.0)
                - (self.group1().wwwx() * other.group0().xyzx()),
            // e15, e25, e35
            (Simd32x3::from(other[e3215]) * self.group1().xyz()) + (self.group2().yzx() * other.group0().zxy()) - (self.group2().zxy() * other.group0().yzx()),
        )
    }
}
impl AntiWedge<VersorEven> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       11        0        0
    //    simd3        0        7        0      N/A
    //    simd4        6        2        0      N/A
    // Totals...
    // yes simd       14       20        0      N/A
    //  no simd       32       40        0        0
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
            Simd32x3::from(0.0).with_w(-(self[e435] * other[e125]) - (self[e235] * other[e415]) - (self[e315] * other[e425]) - (self[e125] * other[e435]))
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (self.group0().zxy() * other.group2().yzx()).with_w(0.0)
                + (self.group2().yzx() * other.group0().zxy()).with_w(0.0)
                - (Simd32x4::from([self[e431], self[e412], self[e423], self[e415]]) * other.group2().zxyx())
                - (self.group2().zxy() * other.group0().yzx()).with_w(self[e425] * other[e315]),
        )
    }
}
impl AntiWedge<VersorOdd> for Circle {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       12        0        0
    //    simd3        2        8        0      N/A
    //    simd4        6        1        0      N/A
    // Totals...
    // yes simd       17       21        0      N/A
    //  no simd       39       40        0        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (self.group0().zxy() * other.group3().yzx()) - (self.group0().yzx() * other.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125]))
                + (Simd32x3::from(other[e1234]) * self.group2()).with_w(0.0)
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                - (self.group1().wwwx() * other.group3().xyzx()),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(
                -(self[e431] * other[e25])
                    - (self[e412] * other[e35])
                    - (self[e415] * other[e23])
                    - (self[e425] * other[e31])
                    - (self[e435] * other[e12])
                    - (self[e321] * other[e45])
                    - (self[e235] * other[e41])
                    - (self[e315] * other[e42])
                    - (self[e125] * other[e43]),
            ) + (Simd32x3::from(other[e3215]) * self.group1().xyz()).with_w(0.0)
                + (self.group2().yzx() * other.group3().zxy()).with_w(0.0)
                - (self.group2().zxy() * other.group3().yzx()).with_w(self[e423] * other[e15]),
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
    //           add/sub      mul      div      pow
    //      f32       10       11        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       10       14        0      N/A
    //  no simd       10       21        0        0
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
    //           add/sub      mul      div      pow
    //      f32        9       11        0        0
    //    simd3        0        6        0      N/A
    //    simd4        7        4        0      N/A
    // Totals...
    // yes simd       16       21        0      N/A
    //  no simd       37       45        0        0
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
            (Simd32x4::from(self[e12345]) * other.group3())
                + Simd32x3::from(0.0).with_w(-(other[e425] * self[e315]) - (other[e435] * self[e125]) - (other[e315] * self[e425]) - (other[e125] * self[e435]))
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group2().yzx()).with_w(0.0)
                + (self.group0().zxy() * other.group2().yzx()).with_w(0.0)
                - (Simd32x4::from([other[e431], other[e412], other[e423], other[e415]]) * self.group2().zxyx())
                - (Simd32x4::from([self[e431], self[e412], self[e423], self[e415]]) * other.group2().zxyx()),
        )
    }
}
impl AntiWedge<AntiDualNum> for CircleRotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0().xxxy() * self.group0().with_w(self[e12345]),
            // e15, e25, e35, e3215
            Simd32x4::from(other[e3215]) * self.group1().xyz().with_w(self[e12345]),
        )
    }
}
impl AntiWedge<AntiFlatPoint> for CircleRotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       13       16        0        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * other.group0(),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(other[e315] * self[e425]) - (other[e125] * self[e435]))
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (self.group0().zxy() * other.group0().yzx()).with_w(0.0)
                - (Simd32x4::from([self[e431], self[e412], self[e423], self[e415]]) * other.group0().zxyx()),
        )
    }
}
impl AntiWedge<AntiFlector> for CircleRotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       17       20        0        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * other.group0(),
            // e1, e2, e3, e5
            (Simd32x4::from(self[e12345]) * other.group1())
                + Simd32x3::from(0.0).with_w(-(other[e315] * self[e425]) - (other[e125] * self[e435]))
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (self.group0().zxy() * other.group0().yzx()).with_w(0.0)
                - (Simd32x4::from([self[e431], self[e412], self[e423], self[e415]]) * other.group0().zxyx()),
        )
    }
}
impl AntiWedge<AntiLine> for CircleRotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       12        0        0
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
            (Simd32x3::from(self[e12345]) * other.group1()).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiMotor> for CircleRotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       16       20        0        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x4::from(self[e12345]) * other.group0())
                + Simd32x3::from(0.0).with_w(
                    -(other[e23] * self[e415])
                        - (other[e31] * self[e425])
                        - (other[e12] * self[e435])
                        - (other[e15] * self[e423])
                        - (other[e25] * self[e431])
                        - (other[e35] * self[e412]),
                )
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0),
            // e15, e25, e35, e3215
            ((Simd32x3::from(other[e3215]) * self.group1().xyz()) + (Simd32x3::from(self[e12345]) * other.group1().xyz())).with_w(other[e3215] * self[e12345]),
        )
    }
}
impl AntiWedge<AntiPlane> for CircleRotor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<AntiScalar> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       11        0        0
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
    //           add/sub      mul      div      pow
    //      f32        8       11        0        0
    //    simd3        0        7        0      N/A
    //    simd4        6        2        0      N/A
    // Totals...
    // yes simd       14       20        0      N/A
    //  no simd       32       40        0        0
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
            Simd32x3::from(0.0).with_w(-(other[e435] * self[e125]) - (other[e235] * self[e415]) - (other[e315] * self[e425]) - (other[e125] * self[e435]))
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group2().yzx()).with_w(0.0)
                + (other.group2().yzx() * self.group0().zxy()).with_w(0.0)
                - (Simd32x4::from([other[e431], other[e412], other[e423], other[e415]]) * self.group2().zxyx())
                - (other.group2().zxy() * self.group0().yzx()).with_w(other[e425] * self[e315]),
        )
    }
}
impl AntiWedge<CircleRotor> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       19        0        0
    //    simd3        1        8        0      N/A
    //    simd4        7        2        0      N/A
    // Totals...
    // yes simd       19       29        0      N/A
    //  no simd       42       51        0        0
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
            Simd32x3::from(0.0).with_w(-(other[e412] * self[e435]) - (other[e415] * self[e423]) - (other[e425] * self[e431]) - (other[e435] * self[e412]))
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group2().yzx()).with_w(0.0)
                + (self.group0().zxy() * other.group2().yzx()).with_w(0.0)
                - (other.group0().yzx() * self.group2().zxy()).with_w(other[e423] * self[e415])
                - (self.group0().yzx() * other.group2().zxy()).with_w(other[e431] * self[e425]),
        )
    }
}
impl AntiWedge<Dipole> for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       10        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        9       13        0      N/A
    //  no simd        9       20        0        0
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
    //           add/sub      mul      div      pow
    //      f32       10       15        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        8        0      N/A
    //    simd4        8        3        0      N/A
    // Totals...
    // yes simd       22       28        0      N/A
    //  no simd       54       55        0        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(
                -(self[e431] * other[e25])
                    - (self[e412] * other[e35])
                    - (self[e415] * other[e23])
                    - (self[e425] * other[e31])
                    - (self[e435] * other[e12])
                    - (self[e321] * other[e45])
                    - (self[e235] * other[e41])
                    - (self[e315] * other[e42])
                    - (self[e125] * other[e43]),
            ) + (Simd32x3::from(self[e12345]) * other.group0()).with_w(0.0)
                + (Simd32x3::from(other[e1234]) * self.group1().xyz()).with_w(0.0)
                + (self.group0().zxy() * other.group3().yzx()).with_w(0.0)
                - (self.group0().yzx() * other.group3().zxy()).with_w(self[e423] * other[e15]),
            // e23, e31, e12, e45
            (self.group2() * Simd32x3::from(other[e1234]).with_w(other[e45]))
                + Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125]))
                + (Simd32x3::from(self[e12345]) * other.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                - (self.group1().wwwx() * other.group3().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e12345]) * other.group2().xyz())
                + (Simd32x3::from(other[e3215]) * self.group1().xyz())
                + Simd32x2::from(0.0).with_z((self[e235] * other[e4315]) - (self[e315] * other[e4235]))
                + (self.group2().yz() * other.group3().zx()).with_z(0.0)
                - (self.group2().zx() * other.group3().yz()).with_z(0.0))
            .with_w(self[e12345] * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group3(),
        )
    }
}
impl AntiWedge<DualNum> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        3        0      N/A
    // no simd        0       12        0        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other[e12345]) * self.group0().with_w(self[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group1(),
            // e235, e315, e125, e5
            self.group2() * Simd32x3::from(other[e12345]).with_w(other[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}
impl AntiWedge<FlatPoint> for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3        8        0        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(self[e12345] * other[e45]),
            // e15, e25, e35, scalar
            (Simd32x3::from(self[e12345]) * other.group0().xyz())
                .with_w(-(self[e423] * other[e15]) - (self[e431] * other[e25]) - (self[e412] * other[e35]) - (self[e321] * other[e45])),
        )
    }
}
impl AntiWedge<Flector> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd3        0        7        0      N/A
    //    simd4        7        2        0      N/A
    // Totals...
    // yes simd       10       16        0      N/A
    //  no simd       31       36        0        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(-(self[e431] * other[e25]) - (self[e412] * other[e35]) - (self[e321] * other[e45]))
                + (self.group0().zxy() * other.group1().yzx()).with_w(0.0)
                - (self.group0().yzx() * other.group1().zxy()).with_w(self[e423] * other[e15]),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(self[e12345] * other[e45])
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
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        0        5        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd       18       24        0        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (Simd32x3::from(self[e12345]) * other.group0()).with_w(0.0),
            // e235, e315, e125, e4
            (Simd32x3::from(self[e12345]) * other.group1()).with_w(-(self[e423] * other[e415]) - (self[e431] * other[e425]) - (self[e412] * other[e435])),
            // e1, e2, e3, e5
            Simd32x3::from(0.0)
                .with_w(-(self[e425] * other[e315]) - (self[e435] * other[e125]) - (self[e235] * other[e415]) - (self[e315] * other[e425]) - (self[e125] * other[e435]))
                + (Simd32x3::from(self[e321]) * other.group0()).with_w(0.0)
                + (self.group0().zxy() * other.group1().yzx()).with_w(0.0)
                - (self.group0().yzx() * other.group1().zxy()).with_w(self[e415] * other[e235]),
        )
    }
}
impl AntiWedge<Motor> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       10        0        0
    //    simd3        1        6        0      N/A
    //    simd4        5        2        0      N/A
    // Totals...
    // yes simd       12       18        0      N/A
    //  no simd       29       36        0        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other[e12345]) * self.group0().with_w(self[e12345]),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e12345]) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * self.group1().xyz())).with_w(self[e321] * other[e12345]),
            // e235, e315, e125, e5
            (self.group2() * Simd32x3::from(other[e12345]).with_w(other[e5]))
                + Simd32x3::from(0.0).with_w(
                    -(self[e415] * other[e235])
                        - (self[e425] * other[e315])
                        - (self[e435] * other[e125])
                        - (self[e235] * other[e415])
                        - (self[e315] * other[e425])
                        - (self[e125] * other[e435]),
                )
                + (Simd32x3::from(self[e12345]) * other.group1().xyz()).with_w(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(self[e431] * other[e425]) - (self[e412] * other[e435]))
                + (Simd32x3::from(self[e321]) * other.group0().xyz()).with_w(0.0)
                + (self.group0().zxy() * other.group1().yzx()).with_w(0.0)
                - (self.group0().yzx() * other.group1().zxy()).with_w(self[e423] * other[e415]),
        )
    }
}
impl AntiWedge<MultiVector> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       20       28        0        0
    //    simd3        8       20        0      N/A
    //    simd4       12        6        0      N/A
    // Totals...
    // yes simd       40       54        0      N/A
    //  no simd       92      112        0        0
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
            (Simd32x4::from(self[e12345]) * other.group1())
                + Simd32x3::from(0.0).with_w(-(self[e412] * other[e435]) - (self[e415] * other[e423]) - (self[e425] * other[e431]) - (self[e435] * other[e412]))
                + (Simd32x3::from(self[e321]) * other.group6().xyz()).with_w(0.0)
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (self.group0().zxy() * other.group8().yzx()).with_w(0.0)
                + (other.group7().zxy() * self.group2().yzx()).with_w(0.0)
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
            (Simd32x4::from(self[e12345]) * other.group3())
                + Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group1().xyz()).with_w(0.0)
                + (self.group2().yzx() * other.group9().zxy()).with_w(0.0)
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
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        1        6        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        6       10        0      N/A
    //  no simd       20       28        0        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            (self.group0().zxy() * other.group0().yzx()) - (self.group0().yzx() * other.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125])) + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                - (self.group1().wwwx() * other.group0().xyzx()),
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
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e12345]) * other.group0(), /* e5 */ self[e12345] * other[e5])
    }
}
impl AntiWedge<Scalar> for CircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e12345] * other[scalar])
    }
}
impl AntiWedge<Sphere> for CircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd2        0        2        0      N/A
    //    simd3        5        6        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd       10       15        0      N/A
    //  no simd       29       35        0        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (self.group0().zxy() * other.group0().yzx()) - (self.group0().yzx() * other.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                + (Simd32x3::from(other[e1234]) * self.group2().xyz()).with_w(0.0)
                - (self.group1().wwwx() * other.group0().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e3215]) * self.group1().xyz())
                + Simd32x2::from(0.0).with_z((self[e235] * other[e4315]) - (self[e315] * other[e4235]))
                + (self.group2().yz() * other.group0().zx()).with_z(0.0)
                - (self.group2().zx() * other.group0().yz()).with_z(0.0))
            .with_w(self[e12345] * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group0(),
        )
    }
}
impl AntiWedge<VersorEven> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       12        0        0
    //    simd3        1        8        0      N/A
    //    simd4       10        5        0      N/A
    // Totals...
    // yes simd       19       25        0      N/A
    //  no simd       51       56        0        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x3::from(self[e12345]) * other.group0().xyz()) + (Simd32x3::from(other[e12345]) * self.group0())).with_w(self[e12345] * other[e12345]),
            // e415, e425, e435, e321
            (Simd32x4::from(self[e12345]) * other.group1()) + (Simd32x4::from(other[e12345]) * self.group1()),
            // e235, e315, e125, e5
            (self.group2() * Simd32x3::from(other[e12345]).with_w(other[e5]))
                + Simd32x3::from(0.0).with_w(
                    -(self[e415] * other[e235])
                        - (self[e425] * other[e315])
                        - (self[e435] * other[e125])
                        - (self[e235] * other[e415])
                        - (self[e315] * other[e425])
                        - (self[e125] * other[e435]),
                )
                + (Simd32x3::from(self[e12345]) * other.group2().xyz()).with_w(0.0),
            // e1, e2, e3, e4
            (Simd32x4::from(self[e12345]) * other.group3())
                + Simd32x3::from(0.0).with_w(-(self[e431] * other[e425]) - (self[e412] * other[e435]) - (self[e425] * other[e431]) - (self[e435] * other[e412]))
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (self.group0().zxy() * other.group2().yzx()).with_w(0.0)
                + (self.group2().yzx() * other.group0().zxy()).with_w(0.0)
                - (other.group0().yzxx() * self.group2().zxy().with_w(self[e415]))
                - (self.group0().yzx() * other.group2().zxy()).with_w(self[e423] * other[e415]),
        )
    }
}
impl AntiWedge<VersorOdd> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       15        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        7        0      N/A
    //    simd4        8        4        0      N/A
    // Totals...
    // yes simd       22       28        0      N/A
    //  no simd       54       56        0        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from(self[e12345]) * other.group0())
                + Simd32x3::from(0.0).with_w(
                    -(self[e431] * other[e25])
                        - (self[e412] * other[e35])
                        - (self[e415] * other[e23])
                        - (self[e425] * other[e31])
                        - (self[e435] * other[e12])
                        - (self[e321] * other[e45])
                        - (self[e235] * other[e41])
                        - (self[e315] * other[e42])
                        - (self[e125] * other[e43]),
                )
                + (Simd32x3::from(other[e1234]) * self.group1().xyz()).with_w(0.0)
                + (self.group0().zxy() * other.group3().yzx()).with_w(0.0)
                - (self.group0().yzx() * other.group3().zxy()).with_w(self[e423] * other[e15]),
            // e23, e31, e12, e45
            (self.group2() * Simd32x3::from(other[e1234]).with_w(other[e45]))
                + Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125]))
                + (Simd32x3::from(self[e12345]) * other.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                - (self.group1().wwwx() * other.group3().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e12345]) * other.group2().xyz())
                + (Simd32x3::from(other[e3215]) * self.group1().xyz())
                + Simd32x2::from(0.0).with_z((self[e235] * other[e4315]) - (self[e315] * other[e4235]))
                + (self.group2().yz() * other.group3().zx()).with_z(0.0)
                - (self.group2().zx() * other.group3().yz()).with_z(0.0))
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
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e3215]) * self.group0().with_w(self[e45]))
    }
}
impl AntiWedge<AntiFlatPoint> for Dipole {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e3215]) * self.group0().with_w(self[e45]))
    }
}
impl AntiWedge<AntiScalar> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       10        0        0
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
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
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
    //           add/sub      mul      div      pow
    //      f32        9       10        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        9       13        0      N/A
    //  no simd        9       20        0        0
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
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       20       20        0        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(self[e42] * other[e4315]) - (self[e43] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                + (self.group1().yzx() * other.group3().zxy()).with_w(0.0)
                - (Simd32x4::from(other[e1234]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                - (other.group3().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e5
            (self[e45] * other[e3215]) + (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
        )
    }
}
impl AntiWedge<DualNum> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       10        0        0
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
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        7        9        0      N/A
    //  no simd       16       16        0        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(self[e42] * other[e4315]) - (self[e43] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                + (self.group1().yzx() * other.group1().zxy()).with_w(0.0)
                - (other.group1().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e5
            (self[e45] * other[e3215]) + (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
        )
    }
}
impl AntiWedge<Line> for Dipole {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
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
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        5        9        0      N/A
    //  no simd        5       16        0        0
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
    //           add/sub      mul      div      pow
    //      f32       13       16        0        0
    //    simd3        0        4        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       17       23        0      N/A
    //  no simd       29       40        0        0
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
            Simd32x3::from(0.0).with_w(-(self[e42] * other[e4315]) - (self[e43] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                + (self.group1().yzx() * other.group9().zxy()).with_w(0.0)
                - (Simd32x4::from(other[e1234]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                - (other.group9().yzxx() * self.group1().zxy().with_w(self[e41])),
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
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        7        9        0      N/A
    //  no simd       16       16        0        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(self[e42] * other[e4315]) - (self[e43] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                + (self.group1().yzx() * other.group0().zxy()).with_w(0.0)
                - (other.group0().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e5
            (self[e45] * other[e3215]) + (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
        )
    }
}
impl AntiWedge<Sphere> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       20       20        0        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(self[e42] * other[e4315]) - (self[e43] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                + (self.group1().yzx() * other.group0().zxy()).with_w(0.0)
                - (Simd32x4::from(other[e1234]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                - (other.group0().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e5
            (self[e45] * other[e3215]) + (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]),
        )
    }
}
impl AntiWedge<VersorEven> for Dipole {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       10        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        9       13        0      N/A
    //  no simd        9       20        0        0
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
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       20       20        0        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(self[e42] * other[e4315]) - (self[e43] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                + (self.group1().yzx() * other.group3().zxy()).with_w(0.0)
                - (Simd32x4::from(other[e1234]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                - (other.group3().yzxx() * self.group1().zxy().with_w(self[e41])),
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
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       20       20        0        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]))
                + (self.group3().yzxx() * other.group1().zxy().with_w(other[e41]))
                + Simd32x3::from(0.0).with_w((other[e42] * self[e4315]) + (other[e43] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (other.group1().yzx() * self.group3().zxy()).with_w(0.0),
            // e5
            -(other[e45] * self[e3215]) - (other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]),
        )
    }
}
impl AntiWedge<AntiDipoleInversion> for DipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       14        0        0
    //    simd3        2        5        0      N/A
    //    simd4        6        4        0      N/A
    // Totals...
    // yes simd       20       23        0      N/A
    //  no simd       42       45        0        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group1().xyz()) + (other.group0().zxy() * self.group3().yzx()) - (other.group0().yzx() * self.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125]))
                + (Simd32x3::from(self[e1234]) * other.group2().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (other.group1().wwwx() * self.group3().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(self[e3215]) * other.group1().xyz().with_w(other[e4]))
                + (self.group3().zxyx() * other.group2().yzx().with_w(other[e1]))
                + Simd32x3::from(0.0).with_w(
                    (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e5] * self[e1234])
                        - (other[e423] * self[e15])
                        - (other[e431] * self[e25])
                        - (other[e412] * self[e35])
                        - (other[e415] * self[e23])
                        - (other[e425] * self[e31])
                        - (other[e435] * self[e12])
                        - (other[e321] * self[e45])
                        - (other[e315] * self[e42])
                        - (other[e125] * self[e43]),
                )
                - (other.group2().zxyx() * self.group3().yzx().with_w(self[e41])),
        )
    }
}
impl AntiWedge<AntiDualNum> for DipoleInversion {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
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
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       13       16        0        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(other[e235] * self[e41]) - (other[e315] * self[e42]) - (other[e125] * self[e43]))
                + (Simd32x3::from(self[e1234]) * other.group0().xyz()).with_w(0.0)
                - (Simd32x4::from(other[e321]) * self.group3().xyz().with_w(self[e45])),
            // e15, e25, e35, e3215
            ((other.group0().yzx() * self.group3().zxy()) - (other.group0().zxy() * self.group3().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiFlector> for DipoleInversion {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       16       20        0        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x4::from(self[e1234]) * other.group0().xyz().with_w(other[e5]))
                + Simd32x3::from(0.0).with_w(
                    (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125])
                        - (other[e235] * self[e41])
                        - (other[e315] * self[e42])
                        - (other[e125] * self[e43]),
                )
                - (Simd32x4::from(other[e321]) * self.group3().xyz().with_w(self[e45])),
            // e15, e25, e35, e3215
            ((other.group0().yzx() * self.group3().zxy()) - (other.group0().zxy() * self.group3().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiLine> for DipoleInversion {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn anti_wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(other[e25] * self[e4315]) - (other[e35] * self[e4125]))
                + (Simd32x3::from(self[e1234]) * other.group1()).with_w(0.0)
                + (other.group0().zxy() * self.group3().yzx()).with_w(0.0)
                - (Simd32x4::from([other[e31], other[e12], other[e23], other[e15]]) * self.group3().zxyx()),
        )
    }
}
impl AntiWedge<AntiMotor> for DipoleInversion {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       17       20        0        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e3215]) * self.group3().xyz().with_w(self[e1234]),
            // e1, e2, e3, e5
            (other.group1() * Simd32x3::from(self[e1234]).with_w(self[e45]))
                + Simd32x3::from(0.0).with_w(-(other[e25] * self[e4315]) - (other[e35] * self[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                + (other.group0().zxy() * self.group3().yzx()).with_w(0.0)
                - (self.group3().zxyx() * other.group0().yzx().with_w(other[e15])),
        )
    }
}
impl AntiWedge<AntiPlane> for DipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       15        0        0
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
    //           add/sub      mul      div      pow
    //      f32        9       12        0        0
    //    simd3        2        8        0      N/A
    //    simd4        6        1        0      N/A
    // Totals...
    // yes simd       17       21        0      N/A
    //  no simd       39       40        0        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group1().xyz()) + (other.group0().zxy() * self.group3().yzx()) - (other.group0().yzx() * self.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125]))
                + (Simd32x3::from(self[e1234]) * other.group2()).with_w(0.0)
                + (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (other.group1().wwwx() * self.group3().xyzx()),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(
                -(other[e431] * self[e25])
                    - (other[e412] * self[e35])
                    - (other[e415] * self[e23])
                    - (other[e425] * self[e31])
                    - (other[e435] * self[e12])
                    - (other[e321] * self[e45])
                    - (other[e235] * self[e41])
                    - (other[e315] * self[e42])
                    - (other[e125] * self[e43]),
            ) + (Simd32x3::from(self[e3215]) * other.group1().xyz()).with_w(0.0)
                + (other.group2().yzx() * self.group3().zxy()).with_w(0.0)
                - (other.group2().zxy() * self.group3().yzx()).with_w(other[e423] * self[e15]),
        )
    }
}
impl AntiWedge<CircleRotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       15        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        8        0      N/A
    //    simd4        8        3        0      N/A
    // Totals...
    // yes simd       22       28        0      N/A
    //  no simd       54       55        0        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(
                -(other[e431] * self[e25])
                    - (other[e412] * self[e35])
                    - (other[e415] * self[e23])
                    - (other[e425] * self[e31])
                    - (other[e435] * self[e12])
                    - (other[e321] * self[e45])
                    - (other[e235] * self[e41])
                    - (other[e315] * self[e42])
                    - (other[e125] * self[e43]),
            ) + (Simd32x3::from(other[e12345]) * self.group0()).with_w(0.0)
                + (Simd32x3::from(self[e1234]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group3().yzx()).with_w(0.0)
                - (other.group0().yzx() * self.group3().zxy()).with_w(other[e423] * self[e15]),
            // e23, e31, e12, e45
            (other.group2() * Simd32x3::from(self[e1234]).with_w(self[e45]))
                + Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125]))
                + (Simd32x3::from(other[e12345]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (other.group1().wwwx() * self.group3().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e12345]) * self.group2().xyz())
                + (Simd32x3::from(self[e3215]) * other.group1().xyz())
                + Simd32x2::from(0.0).with_z((other[e235] * self[e4315]) - (other[e315] * self[e4235]))
                + (other.group2().yz() * self.group3().zx()).with_z(0.0)
                - (other.group2().zx() * self.group3().yz()).with_z(0.0))
            .with_w(other[e12345] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group3(),
        )
    }
}
impl AntiWedge<Dipole> for DipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       20       20        0        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]))
                + (self.group3().yzxx() * other.group1().zxy().with_w(other[e41]))
                + Simd32x3::from(0.0).with_w((other[e42] * self[e4315]) + (other[e43] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (other.group1().yzx() * self.group3().zxy()).with_w(0.0),
            // e5
            -(other[e45] * self[e3215]) - (other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]),
        )
    }
}
impl AntiWedge<DipoleInversion> for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        1        4        0      N/A
    //    simd4       11       10        0      N/A
    // Totals...
    // yes simd       18       22        0      N/A
    //  no simd       53       60        0        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * other.group3().xyz()) - (Simd32x3::from(other[e1234]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (Simd32x4::from([self[e4125], self[e4235], self[e4315], self[e1234]]) * other.group3().yzxw())
                - (Simd32x4::from([other[e4125], other[e4235], other[e4315], other[e1234]]) * self.group3().yzxw()),
            // e235, e315, e125, e4
            (self.group3().xyzx() * Simd32x3::from(other[e3215]).with_w(other[e41]))
                + Simd32x3::from(0.0).with_w(
                    (other[e42] * self[e4315]) + (other[e43] * self[e4125]) + (other[e45] * self[e1234])
                        - (other[e1234] * self[e45])
                        - (other[e4315] * self[e42])
                        - (other[e4125] * self[e43]),
                )
                - (other.group3().xyzx() * Simd32x3::from(self[e3215]).with_w(self[e41])),
            // e1, e2, e3, e5
            (Simd32x4::from(other[e3215]) * self.group0().with_w(self[e45]))
                + (other.group3().zxyy() * self.group1().yzx().with_w(self[e25]))
                + (self.group2().wwwx() * other.group2().xyz().with_w(other[e4235]))
                + Simd32x3::from(0.0).with_w((other[e4125] * self[e35]) - (other[e35] * self[e4125]))
                + (other.group1().zxy() * self.group3().yzx()).with_w(0.0)
                - (Simd32x4::from(self[e3215]) * other.group0().with_w(other[e45]))
                - (other.group2().wwwy() * self.group2().xyz().with_w(self[e4315]))
                - (self.group3().zxyx() * other.group1().yzx().with_w(other[e15]))
                - (other.group3().yzx() * self.group1().zxy()).with_w(0.0),
        )
    }
}
impl AntiWedge<DualNum> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       16        0        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x3::from(other[e12345]) * self.group0()).with_w(self[e1234] * other[e5]),
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
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
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
    //           add/sub      mul      div      pow
    //      f32        5       11        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        4        0      N/A
    //    simd4        6        4        0      N/A
    // Totals...
    // yes simd       13       20        0      N/A
    //  no simd       35       41        0        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * other.group1().xyz(),
            // e415, e425, e435, e321
            ((self.group3().zxy() * other.group1().yzx()) + Simd32x2::from(0.0).with_z(self[e4235] * other[e4315] * -1.0) - (self.group3().yz() * other.group1().zx()).with_z(0.0))
                .with_w(self[e1234] * other[e3215]),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(-(self[e42] * other[e4315]) - (self[e43] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group3().xyz()).with_w(self[e1234] * other[e45])
                - (other.group1().xyzx() * Simd32x3::from(self[e3215]).with_w(self[e41])),
            // e1, e2, e3, e5
            (Simd32x4::from(other[e3215]) * self.group0().with_w(self[e45]))
                + (self.group2().wwwy() * other.group0().xyz().with_w(other[e4315]))
                + (other.group1().zxyx() * self.group1().yzx().with_w(self[e15]))
                + Simd32x3::from(0.0)
                    .with_w((self[e35] * other[e4125]) - (self[e4235] * other[e15]) - (self[e4315] * other[e25]) - (self[e4125] * other[e35]) - (self[e3215] * other[e45]))
                - (self.group1().zxy() * other.group1().yzx()).with_w(0.0),
        )
    }
}
impl AntiWedge<Line> for DipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        0        5        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd       18       24        0        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group0(),
            // e23, e31, e12, e45
            (Simd32x3::from(self[e1234]) * other.group1()).with_w(-(self[e4235] * other[e415]) - (self[e4315] * other[e425]) - (self[e4125] * other[e435])),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(-(self[e42] * other[e315]) - (self[e43] * other[e125]) - (self[e23] * other[e415]) - (self[e31] * other[e425]) - (self[e12] * other[e435]))
                + (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                + (other.group1().yzx() * self.group3().zxy()).with_w(0.0)
                - (other.group1().zxy() * self.group3().yzx()).with_w(self[e41] * other[e235]),
        )
    }
}
impl AntiWedge<Motor> for DipoleInversion {
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
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from(self[e1234]) * other.group0().xyz().with_w(other[e5]))
                + Simd32x3::from(0.0).with_w(
                    -(self[e41] * other[e235])
                        - (self[e42] * other[e315])
                        - (self[e43] * other[e125])
                        - (self[e23] * other[e415])
                        - (self[e31] * other[e425])
                        - (self[e12] * other[e435]),
                )
                + (Simd32x3::from(other[e12345]) * self.group0()).with_w(0.0),
            // e23, e31, e12, e45
            (Simd32x4::from(other[e12345]) * self.group1())
                + Simd32x3::from(0.0).with_w(-(self[e4235] * other[e415]) - (self[e4315] * other[e425]) - (self[e4125] * other[e435]))
                + (Simd32x3::from(self[e1234]) * other.group1().xyz()).with_w(0.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e3215]) * other.group0().xyz())
                + (Simd32x3::from(other[e12345]) * self.group2().xyz())
                + Simd32x2::from(0.0).with_z((self[e4315] * other[e235]) - (self[e4235] * other[e315]))
                + (self.group3().zx() * other.group1().yz()).with_z(0.0)
                - (self.group3().yz() * other.group1().zx()).with_z(0.0))
            .with_w(self[e1234] * other[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group3(),
        )
    }
}
impl AntiWedge<MultiVector> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       22       30        0        0
    //    simd3        8       18        0      N/A
    //    simd4       13        9        0      N/A
    // Totals...
    // yes simd       43       57        0      N/A
    //  no simd       98      120        0        0
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
                + (self.group3().yzxx() * other.group5().zxy().with_w(other[e41]))
                + Simd32x3::from(0.0).with_w(self[e4125] * other[e43])
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(self[e4315] * other[e42])
                + (self.group1().yzx() * other.group9().zxy()).with_w(0.0)
                - (Simd32x4::from(other[e1234]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                - (other.group9().yzxx() * self.group1().zxy().with_w(self[e41]))
                - (Simd32x3::from(self[e3215]) * other.group4()).with_w(self[e43] * other[e4125])
                - (other.group5().yzx() * self.group3().zxy()).with_w(self[e42] * other[e4315]),
            // e5
            (self[e45] * other[e3215]) + (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125])
                - (self[e4235] * other[e15])
                - (self[e4315] * other[e25])
                - (self[e4125] * other[e35])
                - (self[e3215] * other[e45]),
            // e15, e25, e35, e45
            (Simd32x4::from(other[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                + Simd32x3::from(0.0).with_w(-(self[e4315] * other[e425]) - (self[e4125] * other[e435]))
                + (Simd32x3::from(self[e3215]) * other.group6().xyz()).with_w(0.0)
                + (other.group8().yzx() * self.group3().zxy()).with_w(0.0)
                - (self.group3().yzxx() * other.group8().zxy().with_w(other[e415])),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group6().xyz()) + (Simd32x3::from(other[e12345]) * self.group0()) + (other.group7().zxy() * self.group3().yzx())
                - (other.group7().yzx() * self.group3().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e1234]) * other.group8()) + (Simd32x3::from(self[e3215]) * other.group7()) + (Simd32x3::from(other[e12345]) * self.group1().xyz())
                - (Simd32x3::from(other[e321]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (Simd32x4::from([self[e4125], self[e4235], self[e4315], self[e1234]]) * other.group9().yzxw()) - (self.group3().yzxw() * other.group9().zxy().with_w(other[e1234])),
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
    //           add/sub      mul      div      pow
    //      f32        2        7        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        4        0      N/A
    //    simd4        5        3        0      N/A
    // Totals...
    // yes simd        9       15        0      N/A
    //  no simd       28       33        0        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * other.group0().xyz(),
            // e415, e425, e435, e321
            ((self.group3().zxy() * other.group0().yzx()) + Simd32x2::from(0.0).with_z(self[e4235] * other[e4315] * -1.0) - (self.group3().yz() * other.group0().zx()).with_z(0.0))
                .with_w(self[e1234] * other[e3215]),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(-(self[e42] * other[e4315]) - (self[e43] * other[e4125])) + (Simd32x3::from(other[e3215]) * self.group3().xyz()).with_w(0.0)
                - (other.group0().xyzx() * Simd32x3::from(self[e3215]).with_w(self[e41])),
            // e1, e2, e3, e5
            (Simd32x4::from(other[e3215]) * self.group0().with_w(self[e45]))
                + (other.group0().zxyx() * self.group1().yzx().with_w(self[e15]))
                + Simd32x3::from(0.0).with_w((self[e25] * other[e4315]) + (self[e35] * other[e4125]))
                - (self.group1().zxy() * other.group0().yzx()).with_w(0.0),
        )
    }
}
impl AntiWedge<RoundPoint> for DipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        5        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        1        5        0      N/A
    //    simd4        7        5        0      N/A
    // Totals...
    // yes simd       11       15        0      N/A
    //  no simd       34       40        0        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * other.group0().xyz()) - (Simd32x3::from(other[e1234]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (Simd32x4::from([self[e4125], self[e4235], self[e4315], self[e1234]]) * other.group0().yzxw()) - (self.group3().yzxw() * other.group0().zxy().with_w(other[e1234])),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(-(self[e42] * other[e4315]) - (self[e43] * other[e4125]) - (self[e45] * other[e1234]))
                + (Simd32x3::from(other[e3215]) * self.group3().xyz()).with_w(0.0)
                - (other.group0().xyzx() * Simd32x3::from(self[e3215]).with_w(self[e41])),
            // e1, e2, e3, e5
            (Simd32x4::from(other[e3215]) * self.group0().with_w(self[e45]))
                + (other.group0().zxyx() * self.group1().yzx().with_w(self[e15]))
                + Simd32x3::from(0.0).with_w((self[e25] * other[e4315]) + (self[e35] * other[e4125]))
                - (Simd32x3::from(other[e1234]) * self.group2().xyz()).with_w(0.0)
                - (self.group1().zxy() * other.group0().yzx()).with_w(0.0),
        )
    }
}
impl AntiWedge<VersorEven> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       17        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        5        0      N/A
    //    simd4        8        6        0      N/A
    // Totals...
    // yes simd       24       30        0      N/A
    //  no simd       56       60        0        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from(self[e1234]) * other.group1().xyz().with_w(other[e5]))
                + (self.group3().yzxx() * other.group0().zxy().with_w(other[e1]))
                + Simd32x3::from(0.0).with_w(
                    (self[e4125] * other[e3]) + (self[e3215] * other[e4])
                        - (self[e41] * other[e235])
                        - (self[e42] * other[e315])
                        - (self[e43] * other[e125])
                        - (self[e23] * other[e415])
                        - (self[e31] * other[e425])
                        - (self[e12] * other[e435])
                        - (self[e45] * other[e321])
                        - (self[e25] * other[e431])
                        - (self[e35] * other[e412]),
                )
                + (Simd32x3::from(other[e12345]) * self.group0()).with_w(self[e4315] * other[e2])
                - (other.group0().yzxx() * self.group3().zxy().with_w(self[e15])),
            // e23, e31, e12, e45
            (Simd32x4::from(other[e12345]) * self.group1())
                + Simd32x3::from(0.0).with_w(-(self[e4315] * other[e425]) - (self[e4125] * other[e435]))
                + (Simd32x3::from(self[e1234]) * other.group2().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e3215]) * other.group0().xyz()).with_w(0.0)
                - (self.group3().xyzx() * other.group1().wwwx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e3215]) * other.group1().xyz())
                + (Simd32x3::from(other[e12345]) * self.group2().xyz())
                + Simd32x2::from(0.0).with_z((self[e4315] * other[e235]) - (self[e4235] * other[e315]))
                + (self.group3().zx() * other.group2().yz()).with_z(0.0)
                - (self.group3().yz() * other.group2().zx()).with_z(0.0))
            .with_w(self[e1234] * other[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group3(),
        )
    }
}
impl AntiWedge<VersorOdd> for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        1        4        0      N/A
    //    simd4       11       10        0      N/A
    // Totals...
    // yes simd       18       22        0      N/A
    //  no simd       53       60        0        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * other.group3().xyz()) - (Simd32x3::from(other[e1234]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (Simd32x4::from([self[e4125], self[e4235], self[e4315], self[e1234]]) * other.group3().yzxw())
                - (Simd32x4::from([other[e4125], other[e4235], other[e4315], other[e1234]]) * self.group3().yzxw()),
            // e235, e315, e125, e4
            (self.group3().xyzx() * Simd32x3::from(other[e3215]).with_w(other[e41]))
                + Simd32x3::from(0.0).with_w(
                    (self[e1234] * other[e45]) + (self[e4315] * other[e42]) + (self[e4125] * other[e43])
                        - (self[e42] * other[e4315])
                        - (self[e43] * other[e4125])
                        - (self[e45] * other[e1234]),
                )
                - (other.group3().xyzx() * Simd32x3::from(self[e3215]).with_w(self[e41])),
            // e1, e2, e3, e5
            (Simd32x4::from(other[e3215]) * self.group0().with_w(self[e45]))
                + (self.group2().wwwy() * other.group2().xyz().with_w(other[e4315]))
                + (other.group3().zxyx() * self.group1().yzx().with_w(self[e15]))
                + Simd32x3::from(0.0).with_w((self[e35] * other[e4125]) - (self[e4125] * other[e35]))
                + (self.group3().yzx() * other.group1().zxy()).with_w(0.0)
                - (Simd32x4::from(self[e3215]) * other.group0().xyz().with_w(other[e45]))
                - (self.group3().zxyy() * other.group1().yzx().with_w(other[e25]))
                - (other.group2().wwwx() * self.group2().xyz().with_w(self[e4235]))
                - (self.group1().zxy() * other.group3().yzx()).with_w(0.0),
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
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       11        0        0
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
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       15        0        0
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
    //          add/sub      mul      div      pow
    //   simd2        0        1        0      N/A
    // no simd        0        2        0        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<AntiFlatPoint> for DualNum {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<AntiFlector> for DualNum {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
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
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<AntiScalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        0        1        0      N/A
    // no simd        0        2        0        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<Circle> for DualNum {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       10        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        0        3        0      N/A
    // no simd        0       12        0        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self[e12345]) * other.group0().with_w(other[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1(),
            // e235, e315, e125, e5
            other.group2() * Simd32x3::from(self[e12345]).with_w(self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}
impl AntiWedge<Dipole> for DualNum {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       10        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       16        0        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x3::from(self[e12345]) * other.group0()).with_w(other[e1234] * self[e5]),
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
    //      add/sub      mul      div      pow
    // f32        1        3        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
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
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
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
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        1        9        0        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(self[e12345]) * other.group0(),
            // e235, e315, e125, e5
            (Simd32x3::from(self[e12345]) * other.group1().xyz()).with_w((self[e5] * other[e12345]) + (self[e12345] * other[e5])),
        )
    }
}
impl AntiWedge<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        0        4        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        2       14        0      N/A
    //  no simd        2       34        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<RoundPoint> for DualNum {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e12345]) * other.group0(), /* e5 */ self[e12345] * other[e5])
    }
}
impl AntiWedge<Scalar> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e12345] * other[scalar])
    }
}
impl AntiWedge<Sphere> for DualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
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
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        1        6        0      N/A
    //  no simd        1       17        0        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self[e12345]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * other.group1(),
            // e235, e315, e125, e5
            (Simd32x3::from(self[e12345]) * other.group2().xyz()).with_w((self[e5] * other[e12345]) + (self[e12345] * other[e5])),
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * other.group3(),
        )
    }
}
impl AntiWedge<VersorOdd> for DualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        1        6        0      N/A
    //  no simd        1       17        0        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x3::from(self[e12345]) * other.group0().xyz()).with_w((self[e5] * other[e1234]) + (self[e12345] * other[scalar])),
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
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([other[e3215] * self[e45], 0.0]))
    }
}
impl AntiWedge<AntiFlatPoint> for FlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e321] * self[e45] * -1.0)
    }
}
impl AntiWedge<AntiFlector> for FlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e321] * self[e45] * -1.0)
    }
}
impl AntiWedge<AntiMotor> for FlatPoint {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([other[e3215] * self[e45], 0.0]))
    }
}
impl AntiWedge<AntiScalar> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<Circle> for FlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3        8        0        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(other[e12345] * self[e45]),
            // e15, e25, e35, scalar
            (Simd32x3::from(other[e12345]) * self.group0().xyz())
                .with_w(-(other[e423] * self[e15]) - (other[e431] * self[e25]) - (other[e412] * self[e35]) - (other[e321] * self[e45])),
        )
    }
}
impl AntiWedge<DipoleInversion> for FlatPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3        9        0        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234] * -1.0) * self.group0(),
            // e5
            (other[e4235] * self[e15]) + (other[e4315] * self[e25]) + (other[e4125] * self[e35]) + (other[e3215] * self[e45]),
        )
    }
}
impl AntiWedge<DualNum> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<Flector> for FlatPoint {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<MultiVector> for FlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        6       11        0      N/A
    //  no simd        6       17        0        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([-(self[e15] * other[e423]) - (self[e25] * other[e431]) - (self[e35] * other[e412]) - (self[e45] * other[e321]), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234] * -1.0) * self.group0(),
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
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3        9        0        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234] * -1.0) * self.group0(),
            // e5
            (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]) + (self[e45] * other[e3215]),
        )
    }
}
impl AntiWedge<VersorEven> for FlatPoint {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3        8        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3        9        0        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234] * -1.0) * self.group0(),
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
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        7        9        0      N/A
    //  no simd       16       16        0        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (self.group1().yzxx() * other.group1().zxy().with_w(other[e41])) + Simd32x3::from(0.0).with_w((other[e42] * self[e4315]) + (other[e43] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (other.group1().yzx() * self.group1().zxy()).with_w(0.0),
            // e5
            -(other[e45] * self[e3215]) - (other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]),
        )
    }
}
impl AntiWedge<AntiDipoleInversion> for Flector {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        1        4        0      N/A
    //    simd4        5        3        0      N/A
    // Totals...
    // yes simd       12       15        0      N/A
    //  no simd       29       32        0        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (other.group0().zxy() * self.group1().yzx()) - (other.group0().yzx() * self.group1().zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125])) + (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (other.group1().wwwx() * self.group1().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(self[e3215]) * other.group1().xyz().with_w(other[e4]))
                + (self.group1().zxyx() * other.group2().yzx().with_w(other[e1]))
                + Simd32x3::from(0.0).with_w(
                    (other[e2] * self[e4315]) + (other[e3] * self[e4125])
                        - (other[e423] * self[e15])
                        - (other[e431] * self[e25])
                        - (other[e412] * self[e35])
                        - (other[e321] * self[e45]),
                )
                - (other.group2().zxy() * self.group1().yzx()).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiDualNum> for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        4        0        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (Simd32x3::from(other[e3215]) * self.group1().xyz()).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(other[e3215] * self[e45]),
        )
    }
}
impl AntiWedge<AntiFlatPoint> for Flector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        1        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        3       11        0        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other[e321] * -1.0) * self.group1().xyz().with_w(self[e45]),
            // e15, e25, e35, e3215
            ((other.group0().yzx() * self.group1().zxy()) - (other.group0().zxy() * self.group1().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiFlector> for Flector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        6       14        0        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(other[e321] * -1.0) * self.group1().xyz())
                .with_w((other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]) - (other[e321] * self[e45])),
            // e15, e25, e35, e3215
            ((other.group0().yzx() * self.group1().zxy()) - (other.group0().zxy() * self.group1().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiLine> for Flector {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        3        4        0      N/A
    //  no simd        9        9        0        0
    fn anti_wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(other[e25] * self[e4315]) - (other[e35] * self[e4125])) + (other.group0().zxy() * self.group1().yzx()).with_w(0.0)
                - (Simd32x4::from([other[e31], other[e12], other[e23], other[e15]]) * self.group1().zxyx()),
        )
    }
}
impl AntiWedge<AntiMotor> for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        9       13        0        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (Simd32x3::from(other[e3215]) * self.group1().xyz()).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(other[e25] * self[e4315]) - (other[e35] * self[e4125])) + (other.group0().zxy() * self.group1().yzx()).with_w(other[e3215] * self[e45])
                - (self.group1().zxyx() * other.group0().yzx().with_w(other[e15])),
        )
    }
}
impl AntiWedge<AntiPlane> for Flector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn anti_wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]))
    }
}
impl AntiWedge<AntiScalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        1        6        0      N/A
    //    simd4        5        1        0      N/A
    // Totals...
    // yes simd        9       13        0      N/A
    //  no simd       26       28        0        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (other.group0().zxy() * self.group1().yzx()) - (other.group0().yzx() * self.group1().zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125])) + (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (other.group1().wwwx() * self.group1().xyzx()),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(-(other[e431] * self[e25]) - (other[e412] * self[e35]) - (other[e321] * self[e45]))
                + (Simd32x3::from(self[e3215]) * other.group1().xyz()).with_w(0.0)
                + (other.group2().yzx() * self.group1().zxy()).with_w(0.0)
                - (other.group2().zxy() * self.group1().yzx()).with_w(other[e423] * self[e15]),
        )
    }
}
impl AntiWedge<CircleRotor> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd3        0        7        0      N/A
    //    simd4        7        2        0      N/A
    // Totals...
    // yes simd       10       16        0      N/A
    //  no simd       31       36        0        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(-(other[e431] * self[e25]) - (other[e412] * self[e35]) - (other[e321] * self[e45]))
                + (other.group0().zxy() * self.group1().yzx()).with_w(0.0)
                - (other.group0().yzx() * self.group1().zxy()).with_w(other[e423] * self[e15]),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125]))
                + (Simd32x3::from(self[e3215]) * other.group0()).with_w(other[e12345] * self[e45])
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
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        7        9        0      N/A
    //  no simd       16       16        0        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (self.group1().yzxx() * other.group1().zxy().with_w(other[e41])) + Simd32x3::from(0.0).with_w((other[e42] * self[e4315]) + (other[e43] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (other.group1().yzx() * self.group1().zxy()).with_w(0.0),
            // e5
            -(other[e45] * self[e3215]) - (other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]),
        )
    }
}
impl AntiWedge<DipoleInversion> for Flector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       13        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        4        0      N/A
    //    simd4        6        4        0      N/A
    // Totals...
    // yes simd       13       22        0      N/A
    //  no simd       35       43        0        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e1234] * -1.0) * self.group1().xyz(),
            // e415, e425, e435, e321
            ((other.group3().yzx() * self.group1().zxy()) + Simd32x2::from(0.0).with_z(other[e4315] * self[e4235] * -1.0) - (other.group3().zx() * self.group1().yz()).with_z(0.0))
                .with_w(other[e1234] * self[e3215] * -1.0),
            // e235, e315, e125, e4
            (self.group1().xyzx() * Simd32x3::from(other[e3215]).with_w(other[e41]))
                + Simd32x3::from(0.0).with_w((other[e42] * self[e4315]) + (other[e43] * self[e4125]) - (other[e1234] * self[e45]))
                - (Simd32x3::from(self[e3215]) * other.group3().xyz()).with_w(0.0),
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
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
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
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       16       20        0        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((other.group1().yzx() * self.group1().zxy()) - (other.group1().zxy() * self.group1().yzx())).with_w(0.0),
            // e235, e315, e125, e5
            (Simd32x4::from(other[e3215]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e45]]))
                + Simd32x3::from(0.0).with_w(
                    (other[e4235] * self[e15]) + (other[e4315] * self[e25]) + (other[e4125] * self[e35])
                        - (other[e15] * self[e4235])
                        - (other[e25] * self[e4315])
                        - (other[e35] * self[e4125]),
                )
                - (Simd32x4::from(self[e3215]) * Simd32x4::from([other[e4235], other[e4315], other[e4125], other[e45]])),
        )
    }
}
impl AntiWedge<Line> for Flector {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(self[e4315] * other[e425]) - (self[e4125] * other[e435]))
                + (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                + (other.group1().yzx() * self.group1().zxy()).with_w(0.0)
                - (self.group1().yzxx() * other.group1().zxy().with_w(other[e415])),
        )
    }
}
impl AntiWedge<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       17       20        0        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            (Simd32x4::from(other[e12345]) * self.group0())
                + Simd32x3::from(0.0).with_w(-(self[e4315] * other[e425]) - (self[e4125] * other[e435]))
                + (Simd32x3::from(self[e3215]) * other.group0().xyz()).with_w(0.0)
                + (self.group1().zxy() * other.group1().yzx()).with_w(0.0)
                - (self.group1().yzxx() * other.group1().zxy().with_w(other[e415])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group1(),
        )
    }
}
impl AntiWedge<MultiVector> for Flector {
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
            (self.group1().yzxx() * other.group5().zxy().with_w(other[e41])) + Simd32x3::from(0.0).with_w((self[e4315] * other[e42]) + (self[e4125] * other[e43]))
                - (Simd32x4::from(other[e1234]) * self.group0())
                - (Simd32x3::from(self[e3215]) * other.group4()).with_w(0.0)
                - (other.group5().yzx() * self.group1().zxy()).with_w(0.0),
            // e5
            (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]) + (self[e45] * other[e3215])
                - (self[e4235] * other[e15])
                - (self[e4315] * other[e25])
                - (self[e4125] * other[e35])
                - (self[e3215] * other[e45]),
            // e15, e25, e35, e45
            (Simd32x4::from(other[e12345]) * self.group0())
                + Simd32x3::from(0.0).with_w(-(self[e4315] * other[e425]) - (self[e4125] * other[e435]))
                + (Simd32x3::from(self[e3215]) * other.group6().xyz()).with_w(0.0)
                + (other.group8().yzx() * self.group1().zxy()).with_w(0.0)
                - (self.group1().yzxx() * other.group8().zxy().with_w(other[e415])),
            // e41, e42, e43
            (other.group7().zxy() * self.group1().yzx()) - (other.group7().yzx() * self.group1().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e3215]) * other.group7()) - (Simd32x3::from(other[e321]) * self.group1().xyz()),
            // e415, e425, e435, e321
            ((self.group1().zxy() * other.group9().yzx()) + Simd32x2::from(0.0).with_z(self[e4235] * other[e4315] * -1.0) - (self.group1().yz() * other.group9().zx()).with_z(0.0))
                .with_w(self[e3215] * other[e1234] * -1.0),
            // e423, e431, e412
            Simd32x3::from(other[e1234] * -1.0) * self.group1().xyz(),
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
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       13       16        0        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((self.group1().zxy() * other.group0().yzx()) - (self.group1().yzx() * other.group0().zxy())).with_w(0.0),
            // e235, e315, e125, e5
            (Simd32x4::from(other[e3215]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e45]]))
                + Simd32x3::from(0.0).with_w((self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0().xyz()).with_w(0.0),
        )
    }
}
impl AntiWedge<RoundPoint> for Flector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3       11        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        5        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6       18        0      N/A
    //  no simd       12       32        0        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e1234] * -1.0) * self.group1().xyz(),
            // e415, e425, e435, e321
            ((self.group1().zxy() * other.group0().yzx()) + Simd32x2::from(0.0).with_z(self[e4235] * other[e4315] * -1.0) - (self.group1().yz() * other.group0().zx()).with_z(0.0))
                .with_w(self[e3215] * other[e1234] * -1.0),
            // e235, e315, e125, e4
            ((Simd32x3::from(other[e3215]) * self.group1().xyz()) - (Simd32x3::from(self[e3215]) * other.group0().xyz())).with_w(self[e45] * other[e1234] * -1.0),
            // e1, e2, e3, e5
            Simd32x4::from([
                other[e1234],
                other[e1234],
                other[e1234],
                (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]) + (self[e45] * other[e3215]),
            ]) * (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(1.0),
        )
    }
}
impl AntiWedge<VersorEven> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        0        4        0      N/A
    //    simd4        7        5        0      N/A
    // Totals...
    // yes simd       13       17        0      N/A
    //  no simd       34       40        0        0
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
            (other.group0() * Simd32x3::from(self[e3215]).with_w(self[e45])) + Simd32x3::from(0.0).with_w(-(self[e4315] * other[e425]) - (self[e4125] * other[e435]))
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
    //           add/sub      mul      div      pow
    //      f32        5       13        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        4        0      N/A
    //    simd4        6        4        0      N/A
    // Totals...
    // yes simd       13       22        0      N/A
    //  no simd       35       43        0        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e1234] * -1.0) * self.group1().xyz(),
            // e415, e425, e435, e321
            ((self.group1().zxy() * other.group3().yzx()) + Simd32x2::from(0.0).with_z(self[e4235] * other[e4315] * -1.0) - (self.group1().yz() * other.group3().zx()).with_z(0.0))
                .with_w(self[e3215] * other[e1234] * -1.0),
            // e235, e315, e125, e4
            (self.group1().xyzx() * Simd32x3::from(other[e3215]).with_w(other[e41]))
                + Simd32x3::from(0.0).with_w((self[e4315] * other[e42]) + (self[e4125] * other[e43]) - (self[e45] * other[e1234]))
                - (Simd32x3::from(self[e3215]) * other.group3().xyz()).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w((self[e25] * other[e4315]) + (self[e35] * other[e4125]) + (self[e45] * other[e3215]) - (self[e4125] * other[e35]))
                + (self.group1().yzx() * other.group1().zxy()).with_w(self[e15] * other[e4235])
                - (Simd32x4::from(self[e3215]) * other.group0().xyz().with_w(other[e45]))
                - (self.group1().zxyy() * other.group1().yzx().with_w(other[e25]))
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
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
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
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        0        3        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        9       12        0      N/A
    //  no simd       18       18        0        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(other[e431] * self[e425]) - (other[e412] * self[e435]))
                + (Simd32x3::from(other[e321]) * self.group0()).with_w(0.0)
                + (other.group0().zxy() * self.group1().yzx()).with_w(0.0)
                - (other.group0().yzx() * self.group1().zxy()).with_w(other[e423] * self[e415]),
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
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0))
    }
}
impl AntiWedge<AntiFlatPoint> for Line {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
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
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
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
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn anti_wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -(other[e23] * self[e415]) - (other[e31] * self[e425]) - (other[e12] * self[e435]))
    }
}
impl AntiWedge<AntiMotor> for Line {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(other[e23] * self[e415]) - (other[e31] * self[e425]) - (other[e12] * self[e435])),
            // e15, e25, e35, e3215
            (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiScalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
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
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        0        3        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        9       12        0      N/A
    //  no simd       18       18        0        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(other[e431] * self[e425]) - (other[e412] * self[e435]))
                + (Simd32x3::from(other[e321]) * self.group0()).with_w(0.0)
                + (other.group0().zxy() * self.group1().yzx()).with_w(0.0)
                - (other.group0().yzx() * self.group1().zxy()).with_w(other[e423] * self[e415]),
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
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        0        5        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd       18       24        0        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (Simd32x3::from(other[e12345]) * self.group0()).with_w(0.0),
            // e235, e315, e125, e4
            (Simd32x3::from(other[e12345]) * self.group1()).with_w(-(other[e423] * self[e415]) - (other[e431] * self[e425]) - (other[e412] * self[e435])),
            // e1, e2, e3, e5
            Simd32x3::from(0.0)
                .with_w(-(other[e425] * self[e315]) - (other[e435] * self[e125]) - (other[e235] * self[e415]) - (other[e315] * self[e425]) - (other[e125] * self[e435]))
                + (Simd32x3::from(other[e321]) * self.group0()).with_w(0.0)
                + (other.group0().zxy() * self.group1().yzx()).with_w(0.0)
                - (other.group0().yzx() * self.group1().zxy()).with_w(other[e415] * self[e235]),
        )
    }
}
impl AntiWedge<Dipole> for Line {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
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
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        0        5        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd       18       24        0        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group0(),
            // e23, e31, e12, e45
            (Simd32x3::from(other[e1234]) * self.group1()).with_w(-(other[e4235] * self[e415]) - (other[e4315] * self[e425]) - (other[e4125] * self[e435])),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(-(other[e42] * self[e315]) - (other[e43] * self[e125]) - (other[e23] * self[e415]) - (other[e31] * self[e425]) - (other[e12] * self[e435]))
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                + (self.group1().yzx() * other.group3().zxy()).with_w(0.0)
                - (self.group1().zxy() * other.group3().yzx()).with_w(other[e41] * self[e235]),
        )
    }
}
impl AntiWedge<DualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
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
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(other[e4315] * self[e425]) - (other[e4125] * self[e435]))
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                + (self.group1().yzx() * other.group1().zxy()).with_w(0.0)
                - (other.group1().yzxx() * self.group1().zxy().with_w(self[e415])),
        )
    }
}
impl AntiWedge<Line> for Line {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
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
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       12        0        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x3::from(other[e12345]) * self.group0()).with_w(0.0),
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
    //           add/sub      mul      div      pow
    //      f32       12       17        0        0
    //    simd3        0        9        0      N/A
    //    simd4        6        1        0      N/A
    // Totals...
    // yes simd       18       27        0      N/A
    //  no simd       36       48        0        0
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
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e431]) - (self[e435] * other[e412]))
                + (Simd32x3::from(other[e321]) * self.group0()).with_w(0.0)
                + (self.group1().yzx() * other.group7().zxy()).with_w(0.0)
                - (self.group1().zxy() * other.group7().yzx()).with_w(self[e415] * other[e423]),
            // e5
            -(self[e415] * other[e235])
                - (self[e425] * other[e315])
                - (self[e435] * other[e125])
                - (self[e235] * other[e415])
                - (self[e315] * other[e425])
                - (self[e125] * other[e435]),
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                + (self.group1().yzx() * other.group9().zxy()).with_w(0.0)
                - (other.group9().yzxx() * self.group1().zxy().with_w(self[e415])),
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[e1234]) * self.group1(),
            // e415, e425, e435, e321
            (Simd32x3::from(other[e12345]) * self.group0()).with_w(0.0),
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
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                + (self.group1().yzx() * other.group0().zxy()).with_w(0.0)
                - (other.group0().yzxx() * self.group1().zxy().with_w(self[e415])),
        )
    }
}
impl AntiWedge<Sphere> for Line {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        2        4        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        8       19        0        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                other[e1234],
                other[e1234],
                other[e1234],
                -(self[e415] * other[e4235]) - (self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) * self.group1().with_w(1.0),
            // e15, e25, e35
            (Simd32x3::from(other[e3215]) * self.group0()) + (self.group1().yzx() * other.group0().zxy()) - (self.group1().zxy() * other.group0().yzx()),
        )
    }
}
impl AntiWedge<VersorEven> for Line {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        0        5        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd       18       24        0        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (Simd32x3::from(other[e12345]) * self.group0()).with_w(0.0),
            // e235, e315, e125, e4
            (Simd32x3::from(other[e12345]) * self.group1()).with_w(-(self[e415] * other[e423]) - (self[e425] * other[e431]) - (self[e435] * other[e412])),
            // e1, e2, e3, e5
            Simd32x3::from(0.0)
                .with_w(-(self[e425] * other[e315]) - (self[e435] * other[e125]) - (self[e235] * other[e415]) - (self[e315] * other[e425]) - (self[e125] * other[e435]))
                + (Simd32x3::from(other[e321]) * self.group0()).with_w(0.0)
                + (self.group1().yzx() * other.group0().zxy()).with_w(0.0)
                - (self.group1().zxy() * other.group0().yzx()).with_w(self[e415] * other[e235]),
        )
    }
}
impl AntiWedge<VersorOdd> for Line {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        0        5        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd       18       24        0        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group0(),
            // e23, e31, e12, e45
            (Simd32x3::from(other[e1234]) * self.group1()).with_w(-(self[e415] * other[e4235]) - (self[e425] * other[e4315]) - (self[e435] * other[e4125])),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e31]) - (self[e435] * other[e12]) - (self[e235] * other[e41]) - (self[e315] * other[e42]) - (self[e125] * other[e43]))
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w(0.0)
                + (self.group1().yzx() * other.group3().zxy()).with_w(0.0)
                - (self.group1().zxy() * other.group3().yzx()).with_w(self[e415] * other[e23]),
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
    //           add/sub      mul      div      pow
    //      f32        6        7        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6       10        0      N/A
    //  no simd        6       17        0        0
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
    //           add/sub      mul      div      pow
    //      f32        7        9        0        0
    //    simd3        0        4        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       11       16        0      N/A
    //  no simd       23       33        0        0
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
            (self.group0() * Simd32x3::from(other[e321]).with_w(other[e5]))
                + Simd32x3::from(0.0)
                    .with_w(-(other[e425] * self[e315]) - (other[e435] * self[e125]) - (other[e235] * self[e415]) - (other[e315] * self[e425]) - (other[e125] * self[e435]))
                + (Simd32x3::from(self[e12345]) * other.group3().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group1().yzx()).with_w(0.0)
                - (Simd32x4::from([other[e431], other[e412], other[e423], other[e415]]) * self.group1().zxyx()),
        )
    }
}
impl AntiWedge<AntiDualNum> for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
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
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2       10        0        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * other.group0(),
            // e1, e2, e3, e5
            (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(-(other[e235] * self[e415]) - (other[e315] * self[e425]) - (other[e125] * self[e435])),
        )
    }
}
impl AntiWedge<AntiFlector> for Motor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       10       14        0        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * other.group0(),
            // e1, e2, e3, e5
            (self.group0() * Simd32x3::from(other[e321]).with_w(other[e5]))
                + Simd32x3::from(0.0).with_w(-(other[e235] * self[e415]) - (other[e315] * self[e425]) - (other[e125] * self[e435]))
                + (Simd32x3::from(self[e12345]) * other.group1().xyz()).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiLine> for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        9        0        0
    fn anti_wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e12345]) * other.group0()).with_w(-(other[e23] * self[e415]) - (other[e31] * self[e425]) - (other[e12] * self[e435])),
            // e15, e25, e35, e3215
            (Simd32x3::from(self[e12345]) * other.group1()).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiMotor> for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        6       14        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
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
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        0        4        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd       18       28        0        0
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
            Simd32x3::from(0.0)
                .with_w(-(other[e425] * self[e315]) - (other[e435] * self[e125]) - (other[e235] * self[e415]) - (other[e315] * self[e425]) - (other[e125] * self[e435]))
                + (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group1().yzx()).with_w(0.0)
                - (Simd32x4::from([other[e431], other[e412], other[e423], other[e415]]) * self.group1().zxyx()),
        )
    }
}
impl AntiWedge<CircleRotor> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       10        0        0
    //    simd3        1        6        0      N/A
    //    simd4        5        2        0      N/A
    // Totals...
    // yes simd       12       18        0      N/A
    //  no simd       29       36        0        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self[e12345]) * other.group0().with_w(other[e12345]),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e12345]) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * other.group1().xyz())).with_w(other[e321] * self[e12345]),
            // e235, e315, e125, e5
            (other.group2() * Simd32x3::from(self[e12345]).with_w(self[e5]))
                + Simd32x3::from(0.0).with_w(
                    -(other[e415] * self[e235])
                        - (other[e425] * self[e315])
                        - (other[e435] * self[e125])
                        - (other[e235] * self[e415])
                        - (other[e315] * self[e425])
                        - (other[e125] * self[e435]),
                )
                + (Simd32x3::from(other[e12345]) * self.group1().xyz()).with_w(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(other[e431] * self[e425]) - (other[e412] * self[e435]))
                + (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group1().yzx()).with_w(0.0)
                - (other.group0().yzx() * self.group1().zxy()).with_w(other[e423] * self[e415]),
        )
    }
}
impl AntiWedge<Dipole> for Motor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        5        9        0      N/A
    //  no simd        5       16        0        0
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
    //           add/sub      mul      div      pow
    //      f32        8       12        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        4        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       16       21        0      N/A
    //  no simd       36       40        0        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from(other[e1234]) * self.group0().xyz().with_w(self[e5]))
                + Simd32x3::from(0.0).with_w(
                    -(other[e41] * self[e235])
                        - (other[e42] * self[e315])
                        - (other[e43] * self[e125])
                        - (other[e23] * self[e415])
                        - (other[e31] * self[e425])
                        - (other[e12] * self[e435]),
                )
                + (Simd32x3::from(self[e12345]) * other.group0()).with_w(0.0),
            // e23, e31, e12, e45
            (Simd32x4::from(self[e12345]) * other.group1())
                + Simd32x3::from(0.0).with_w(-(other[e4235] * self[e415]) - (other[e4315] * self[e425]) - (other[e4125] * self[e435]))
                + (Simd32x3::from(other[e1234]) * self.group1().xyz()).with_w(0.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e3215]) * self.group0().xyz())
                + (Simd32x3::from(self[e12345]) * other.group2().xyz())
                + Simd32x2::from(0.0).with_z((other[e4315] * self[e235]) - (other[e4235] * self[e315]))
                + (other.group3().zx() * self.group1().yz()).with_z(0.0)
                - (other.group3().yz() * self.group1().zx()).with_z(0.0))
            .with_w(other[e1234] * self[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group3(),
        )
    }
}
impl AntiWedge<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        1        9        0        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(other[e12345]) * self.group0(),
            // e235, e315, e125, e5
            (Simd32x3::from(other[e12345]) * self.group1().xyz()).with_w((other[e5] * self[e12345]) + (other[e12345] * self[e5])),
        )
    }
}
impl AntiWedge<FlatPoint> for Motor {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       17       20        0        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            (Simd32x4::from(self[e12345]) * other.group0())
                + Simd32x3::from(0.0).with_w(-(other[e4315] * self[e425]) - (other[e4125] * self[e435]))
                + (Simd32x3::from(other[e3215]) * self.group0().xyz()).with_w(0.0)
                + (other.group1().zxy() * self.group1().yzx()).with_w(0.0)
                - (other.group1().yzxx() * self.group1().zxy().with_w(self[e415])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group1(),
        )
    }
}
impl AntiWedge<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       12        0        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x3::from(self[e12345]) * other.group0()).with_w(0.0),
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
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       16       21        0        0
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
    //           add/sub      mul      div      pow
    //      f32       16       24        0        0
    //    simd3        4       14        0      N/A
    //    simd4        8        4        0      N/A
    // Totals...
    // yes simd       28       42        0      N/A
    //  no simd       60       82        0        0
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
            (self.group0() * Simd32x3::from(other[e321]).with_w(other[e4]))
                + Simd32x3::from(0.0).with_w(-(self[e425] * other[e431]) - (self[e435] * other[e412]))
                + (Simd32x3::from(self[e12345]) * other.group1().xyz()).with_w(0.0)
                + (other.group7().zxy() * self.group1().yzx()).with_w(0.0)
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
            (self.group0() * Simd32x3::from(other[e3215]).with_w(other[e45]))
                + Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125]))
                + (Simd32x3::from(self[e12345]) * other.group3().xyz()).with_w(0.0)
                + (self.group1().yzx() * other.group9().zxy()).with_w(0.0)
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
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       13       16        0        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group0().xyz()).with_w(0.0)
                + (self.group1().yzx() * other.group0().zxy()).with_w(0.0)
                - (other.group0().yzxx() * self.group1().zxy().with_w(self[e415])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group0(),
        )
    }
}
impl AntiWedge<RoundPoint> for Motor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e12345]) * other.group0(), /* e5 */ self[e12345] * other[e5])
    }
}
impl AntiWedge<Scalar> for Motor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e12345] * other[scalar])
    }
}
impl AntiWedge<Sphere> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd2        0        2        0      N/A
    //    simd3        3        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        6       12        0      N/A
    //  no simd       12       25        0        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other[e1234]) * self.group0().xyz().with_w(self[e5]),
            // e23, e31, e12, e45
            Simd32x4::from([
                other[e1234],
                other[e1234],
                other[e1234],
                -(self[e415] * other[e4235]) - (self[e425] * other[e4315]) - (self[e435] * other[e4125]),
            ]) * self.group1().xyz().with_w(1.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e3215]) * self.group0().xyz())
                + Simd32x2::from(0.0).with_z((self[e235] * other[e4315]) - (self[e315] * other[e4235]))
                + (self.group1().yz() * other.group0().zx()).with_z(0.0)
                - (self.group1().zx() * other.group0().yz()).with_z(0.0))
            .with_w(self[e12345] * other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group0(),
        )
    }
}
impl AntiWedge<VersorEven> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        1        4        0      N/A
    //    simd4        6        5        0      N/A
    // Totals...
    // yes simd       13       18        0      N/A
    //  no simd       33       41        0        0
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
            (self.group0() * Simd32x3::from(other[e321]).with_w(other[e4]))
                + Simd32x3::from(0.0).with_w(-(self[e425] * other[e431]) - (self[e435] * other[e412]))
                + (Simd32x3::from(self[e12345]) * other.group3().xyz()).with_w(0.0)
                + (self.group1().yzx() * other.group0().zxy()).with_w(0.0)
                - (other.group0().yzxx() * self.group1().zxy().with_w(self[e415])),
        )
    }
}
impl AntiWedge<VersorOdd> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       13        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        4        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       17       22        0      N/A
    //  no simd       37       41        0        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() * Simd32x3::from(other[e1234]).with_w(other[scalar]))
                + Simd32x3::from(0.0).with_w(
                    (self[e5] * other[e1234])
                        - (self[e415] * other[e23])
                        - (self[e425] * other[e31])
                        - (self[e435] * other[e12])
                        - (self[e235] * other[e41])
                        - (self[e315] * other[e42])
                        - (self[e125] * other[e43]),
                )
                + (Simd32x3::from(self[e12345]) * other.group0().xyz()).with_w(0.0),
            // e23, e31, e12, e45
            (Simd32x4::from(self[e12345]) * other.group1())
                + Simd32x3::from(0.0).with_w(-(self[e415] * other[e4235]) - (self[e425] * other[e4315]) - (self[e435] * other[e4125]))
                + (Simd32x3::from(other[e1234]) * self.group1().xyz()).with_w(0.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e12345]) * other.group2().xyz())
                + (Simd32x3::from(other[e3215]) * self.group0().xyz())
                + Simd32x2::from(0.0).with_z((self[e235] * other[e4315]) - (self[e315] * other[e4235]))
                + (self.group1().yz() * other.group3().zx()).with_z(0.0)
                - (self.group1().zx() * other.group3().yz()).with_z(0.0))
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
    //           add/sub      mul      div      pow
    //      f32       14       17        0        0
    //    simd3        0        4        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       18       24        0      N/A
    //  no simd       30       41        0        0
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
            (Simd32x4::from(self[e1234]) * Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]))
                + (self.group9().yzxx() * other.group1().zxy().with_w(other[e41]))
                + Simd32x3::from(0.0).with_w((other[e42] * self[e4315]) + (other[e43] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (other.group1().yzx() * self.group9().zxy()).with_w(0.0),
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
    //           add/sub      mul      div      pow
    //      f32       24       30        0        0
    //    simd3        4       16        0      N/A
    //    simd4       10        3        0      N/A
    // Totals...
    // yes simd       38       49        0      N/A
    //  no simd       76       90        0        0
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
            (Simd32x4::from(self[e12345]) * Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]))
                + Simd32x3::from(0.0).with_w(-(other[e412] * self[e435]) - (other[e415] * self[e423]) - (other[e425] * self[e431]) - (other[e435] * self[e412]))
                + (Simd32x3::from(other[e321]) * self.group6().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group8().yzx()).with_w(0.0)
                + (self.group7().zxy() * other.group2().yzx()).with_w(0.0)
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
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125]))
                + (Simd32x3::from(self[e3215]) * other.group1().xyz()).with_w(0.0)
                + (other.group2().yzx() * self.group9().zxy()).with_w(0.0)
                - (self.group9().yzxx() * other.group2().zxy().with_w(other[e415])),
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
    //           add/sub      mul      div      pow
    //      f32        1        5        0        0
    //    simd3        0        4        0      N/A
    // Totals...
    // yes simd        1        9        0      N/A
    //  no simd        1       17        0        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other[e3215] * self[e4]) + (other[scalar] * self[e12345]), 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(other[e3215]) * self.group4()).with_w(0.0),
            // e5
            other[e3215] * self[e45],
            // e15, e25, e35, e45
            (Simd32x3::from(other[e3215]) * self.group6().xyz()).with_w(0.0),
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
    //           add/sub      mul      div      pow
    //      f32        5        8        0        0
    //    simd3        2        8        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        9       16        0      N/A
    //  no simd       19       32        0        0
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
    //           add/sub      mul      div      pow
    //      f32       10       13        0        0
    //    simd3        2        9        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd       15       22        0      N/A
    //  no simd       28       40        0        0
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
    //           add/sub      mul      div      pow
    //      f32        7        9        0        0
    //    simd3        0        5        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd       15       24        0        0
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
impl AntiWedge<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       14        0        0
    //    simd3        2        9        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd       15       23        0      N/A
    //  no simd       28       41        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3        8        0        0
    fn anti_wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e5] * self[e1234]), 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(self[e12345]) * other.group0().xyz()).with_w(0.0),
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
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        4        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0       11        0      N/A
    //  no simd        0       32        0        0
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
    //           add/sub      mul      div      pow
    //      f32       18       24        0        0
    //    simd3        4       16        0      N/A
    //    simd4        9        2        0      N/A
    // Totals...
    // yes simd       31       42        0      N/A
    //  no simd       66       80        0        0
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
            Simd32x3::from(0.0).with_w(-(other[e412] * self[e435]) - (other[e415] * self[e423]) - (other[e425] * self[e431]) - (other[e435] * self[e412]))
                + (Simd32x3::from(other[e321]) * self.group6().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group8().yzx()).with_w(0.0)
                + (other.group2().yzx() * self.group7().zxy()).with_w(0.0)
                - (other.group0().yzx() * self.group8().zxy()).with_w(other[e423] * self[e415])
                - (other.group2().zxy() * self.group7().yzx()).with_w(other[e431] * self[e425]),
            // e5
            -(other[e415] * self[e235])
                - (other[e425] * self[e315])
                - (other[e435] * self[e125])
                - (other[e235] * self[e415])
                - (other[e315] * self[e425])
                - (other[e125] * self[e435]),
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125]))
                + (Simd32x3::from(self[e3215]) * other.group1().xyz()).with_w(0.0)
                + (other.group2().yzx() * self.group9().zxy()).with_w(0.0)
                - (self.group9().yzxx() * other.group2().zxy().with_w(other[e415])),
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
    //           add/sub      mul      div      pow
    //      f32       20       28        0        0
    //    simd3        8       20        0      N/A
    //    simd4       12        6        0      N/A
    // Totals...
    // yes simd       40       54        0      N/A
    //  no simd       92      112        0        0
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
            (Simd32x4::from(other[e12345]) * self.group1())
                + Simd32x3::from(0.0).with_w(-(other[e412] * self[e435]) - (other[e415] * self[e423]) - (other[e425] * self[e431]) - (other[e435] * self[e412]))
                + (Simd32x3::from(other[e321]) * self.group6().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group8().yzx()).with_w(0.0)
                + (self.group7().zxy() * other.group2().yzx()).with_w(0.0)
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
            (Simd32x4::from(other[e12345]) * self.group3())
                + Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125]))
                + (Simd32x3::from(self[e3215]) * other.group1().xyz()).with_w(0.0)
                + (other.group2().yzx() * self.group9().zxy()).with_w(0.0)
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
    //           add/sub      mul      div      pow
    //      f32       13       16        0        0
    //    simd3        0        4        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       17       23        0      N/A
    //  no simd       29       40        0        0
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
            (Simd32x4::from(self[e1234]) * Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]))
                + (self.group9().yzxx() * other.group1().zxy().with_w(other[e41]))
                + Simd32x3::from(0.0).with_w((other[e42] * self[e4315]) + (other[e43] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (other.group1().yzx() * self.group9().zxy()).with_w(0.0),
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
    //           add/sub      mul      div      pow
    //      f32       22       31        0        0
    //    simd3        8       18        0      N/A
    //    simd4       13        9        0      N/A
    // Totals...
    // yes simd       43       58        0      N/A
    //  no simd       98      121        0        0
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
            (Simd32x4::from(self[e1234]) * Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]))
                + (self.group9().yzxx() * other.group1().zxy().with_w(other[e41]))
                + Simd32x3::from(0.0).with_w((other[e4125] * self[e43]) * -1.0)
                + (Simd32x3::from(other[e3215]) * self.group4()).with_w(other[e43] * self[e4125])
                + (self.group5().yzx() * other.group3().zxy()).with_w(other[e42] * self[e4315])
                - (Simd32x4::from(other[e1234]) * self.group3())
                - (other.group3().yzxx() * self.group5().zxy().with_w(self[e41]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(other[e4315] * self[e42])
                - (other.group1().yzx() * self.group9().zxy()).with_w(0.0),
            // e5
            (other[e4235] * self[e15]) + (other[e4315] * self[e25]) + (other[e4125] * self[e35]) + (other[e3215] * self[e45])
                - (other[e45] * self[e3215])
                - (other[e15] * self[e4235])
                - (other[e25] * self[e4315])
                - (other[e35] * self[e4125]),
            // e15, e25, e35, e45
            (Simd32x4::from(self[e12345]) * Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]))
                + Simd32x3::from(0.0).with_w(-(other[e4315] * self[e425]) - (other[e4125] * self[e435]))
                + (Simd32x3::from(other[e3215]) * self.group6().xyz()).with_w(0.0)
                + (self.group8().yzx() * other.group3().zxy()).with_w(0.0)
                - (other.group3().yzxx() * self.group8().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group6().xyz()) + (Simd32x3::from(self[e12345]) * other.group0()) + (self.group7().zxy() * other.group3().yzx())
                - (self.group7().yzx() * other.group3().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[e1234]) * self.group8()) + (Simd32x3::from(other[e3215]) * self.group7()) + (Simd32x3::from(self[e12345]) * other.group1().xyz())
                - (Simd32x3::from(self[e321]) * other.group3().xyz()),
            // e415, e425, e435, e321
            (other.group3().yzxw() * self.group9().zxy().with_w(self[e1234])) - (Simd32x4::from([other[e4125], other[e4235], other[e4315], other[e1234]]) * self.group9().yzxw()),
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
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        0        4        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        2       14        0      N/A
    //  no simd        2       34        0        0
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
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        6       10        0      N/A
    //  no simd        6       16        0        0
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
    //           add/sub      mul      div      pow
    //      f32       16       23        0        0
    //    simd2        0        1        0      N/A
    //    simd3        5       12        0      N/A
    //    simd4        8        5        0      N/A
    // Totals...
    // yes simd       29       41        0      N/A
    //  no simd       63       81        0        0
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
            (Simd32x4::from(self[e1234]) * other.group0())
                + Simd32x3::from(0.0).with_w(-(other[e4315] * self[e42]) - (other[e4125] * self[e43]))
                + (Simd32x3::from(other[e3215]) * self.group4()).with_w(0.0)
                + (self.group5().yzx() * other.group1().zxy()).with_w(0.0)
                - (other.group1().yzxx() * self.group5().zxy().with_w(self[e41])),
            // e5
            (other[e4235] * self[e15]) + (other[e4315] * self[e25]) + (other[e4125] * self[e35]) + (other[e3215] * self[e45])
                - (other[e15] * self[e4235])
                - (other[e25] * self[e4315])
                - (other[e35] * self[e4125])
                - (other[e45] * self[e3215]),
            // e15, e25, e35, e45
            (Simd32x4::from(self[e12345]) * other.group0())
                + Simd32x3::from(0.0).with_w(-(other[e4315] * self[e425]) - (other[e4125] * self[e435]))
                + (Simd32x3::from(other[e3215]) * self.group6().xyz()).with_w(0.0)
                + (self.group8().yzx() * other.group1().zxy()).with_w(0.0)
                - (other.group1().yzxx() * self.group8().zxy().with_w(self[e415])),
            // e41, e42, e43
            (self.group7().zxy() * other.group1().yzx()) - (self.group7().yzx() * other.group1().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[e3215]) * self.group7()) - (Simd32x3::from(self[e321]) * other.group1().xyz()),
            // e415, e425, e435, e321
            ((other.group1().yzx() * self.group9().zxy()) + Simd32x2::from(0.0).with_z(other[e4315] * self[e4235] * -1.0) - (other.group1().zx() * self.group9().yz()).with_z(0.0))
                .with_w(other[e3215] * self[e1234]),
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
    //           add/sub      mul      div      pow
    //      f32       12       17        0        0
    //    simd3        0        9        0      N/A
    //    simd4        6        1        0      N/A
    // Totals...
    // yes simd       18       27        0      N/A
    //  no simd       36       48        0        0
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
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e431]) - (other[e435] * self[e412]))
                + (Simd32x3::from(self[e321]) * other.group0()).with_w(0.0)
                + (other.group1().yzx() * self.group7().zxy()).with_w(0.0)
                - (other.group1().zxy() * self.group7().yzx()).with_w(other[e415] * self[e423]),
            // e5
            -(other[e415] * self[e235])
                - (other[e425] * self[e315])
                - (other[e435] * self[e125])
                - (other[e235] * self[e415])
                - (other[e315] * self[e425])
                - (other[e125] * self[e435]),
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125]))
                + (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                + (other.group1().yzx() * self.group9().zxy()).with_w(0.0)
                - (self.group9().yzxx() * other.group1().zxy().with_w(other[e415])),
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[e1234]) * other.group1(),
            // e415, e425, e435, e321
            (Simd32x3::from(self[e12345]) * other.group0()).with_w(0.0),
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
    //           add/sub      mul      div      pow
    //      f32       16       24        0        0
    //    simd3        4       14        0      N/A
    //    simd4        8        4        0      N/A
    // Totals...
    // yes simd       28       42        0      N/A
    //  no simd       60       82        0        0
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
            (other.group0() * Simd32x3::from(self[e321]).with_w(self[e4]))
                + Simd32x3::from(0.0).with_w(-(other[e425] * self[e431]) - (other[e435] * self[e412]))
                + (Simd32x3::from(other[e12345]) * self.group1().xyz()).with_w(0.0)
                + (self.group7().zxy() * other.group1().yzx()).with_w(0.0)
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
            (other.group0() * Simd32x3::from(self[e3215]).with_w(self[e45]))
                + Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125]))
                + (Simd32x3::from(other[e12345]) * self.group3().xyz()).with_w(0.0)
                + (other.group1().yzx() * self.group9().zxy()).with_w(0.0)
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
    //           add/sub      mul      div      pow
    //      f32       53       65        0        0
    //    simd3       20       38        0      N/A
    //    simd4       28       16        0      N/A
    // Totals...
    // yes simd      101      119        0      N/A
    //  no simd      225      243        0        0
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
            (Simd32x4::from(other[e12345]) * self.group1())
                + (Simd32x4::from(self[e12345]) * other.group1())
                + (Simd32x4::from(self[e1234]) * other.group3())
                + (self.group9().yzxx() * other.group5().zxy().with_w(other[e41]))
                + Simd32x3::from(0.0).with_w(-(other[e431] * self[e425]) - (other[e412] * self[e435]) - (other[e4315] * self[e42]) - (other[e4125] * self[e43]))
                + (Simd32x3::from(other[e321]) * self.group6().xyz()).with_w(0.0)
                + (Simd32x3::from(other[e3215]) * self.group4()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * other.group6().xyz()).with_w(0.0)
                + (other.group7().zxy() * self.group8().yzx()).with_w(other[e42] * self[e4315])
                + (other.group8().yzx() * self.group7().zxy()).with_w(other[e43] * self[e4125])
                + (self.group5().yzx() * other.group9().zxy()).with_w(0.0)
                - (Simd32x4::from(other[e1234]) * self.group3())
                - (other.group9().yzxx() * self.group5().zxy().with_w(self[e41]))
                - (Simd32x3::from(self[e3215]) * other.group4()).with_w(other[e415] * self[e423])
                - (other.group5().yzx() * self.group9().zxy()).with_w(other[e425] * self[e431])
                - (other.group7().yzx() * self.group8().zxy()).with_w(other[e435] * self[e412])
                - (other.group8().zxy() * self.group7().yzx()).with_w(other[e423] * self[e415]),
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
            (Simd32x4::from(other[e12345]) * self.group3())
                + (Simd32x4::from(self[e12345]) * other.group3())
                + Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125]) - (other[e4315] * self[e425]) - (other[e4125] * self[e435]))
                + (Simd32x3::from(other[e3215]) * self.group6().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e3215]) * other.group6().xyz()).with_w(0.0)
                + (other.group8().yzx() * self.group9().zxy()).with_w(0.0)
                + (self.group8().yzx() * other.group9().zxy()).with_w(0.0)
                - (other.group9().yzxx() * self.group8().zxy().with_w(self[e415]))
                - (self.group9().yzxx() * other.group8().zxy().with_w(other[e415])),
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
                - (self.group9().yzxw() * other.group9().zxy().with_w(other[e1234])),
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
    //           add/sub      mul      div      pow
    //      f32        8       15        0        0
    //    simd2        0        1        0      N/A
    //    simd3        5       12        0      N/A
    //    simd4        6        3        0      N/A
    // Totals...
    // yes simd       19       31        0      N/A
    //  no simd       47       65        0        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e4] * other[e3215]), 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(self[e42] * other[e4315]) - (self[e43] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group4()).with_w(0.0)
                + (self.group5().yzx() * other.group0().zxy()).with_w(0.0)
                - (other.group0().yzxx() * self.group5().zxy().with_w(self[e41])),
            // e5
            (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]) + (self[e45] * other[e3215]),
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group6().xyz()).with_w(0.0)
                + (self.group8().yzx() * other.group0().zxy()).with_w(0.0)
                - (other.group0().yzxx() * self.group8().zxy().with_w(self[e415])),
            // e41, e42, e43
            (self.group7().zxy() * other.group0().yzx()) - (self.group7().yzx() * other.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[e3215]) * self.group7()) - (Simd32x3::from(self[e321]) * other.group0().xyz()),
            // e415, e425, e435, e321
            ((self.group9().zxy() * other.group0().yzx()) + Simd32x2::from(0.0).with_z(self[e4235] * other[e4315] * -1.0) - (self.group9().yz() * other.group0().zx()).with_z(0.0))
                .with_w(self[e1234] * other[e3215]),
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
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd        4       10        0        0
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
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e12345] * other[scalar])
    }
}
impl AntiWedge<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       14        0        0
    //    simd3        6       14        0      N/A
    //    simd4        8        6        0      N/A
    // Totals...
    // yes simd       23       34        0      N/A
    //  no simd       59       80        0        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e4] * other[e3215]) + (self[e5] * other[e1234]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(self[e42] * other[e4315]) - (self[e43] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group4()).with_w(0.0)
                + (self.group5().yzx() * other.group0().zxy()).with_w(0.0)
                - (Simd32x4::from(other[e1234]) * self.group3())
                - (other.group0().yzxx() * self.group5().zxy().with_w(self[e41])),
            // e5
            (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]) + (self[e45] * other[e3215]),
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group6().xyz()).with_w(0.0)
                + (self.group8().yzx() * other.group0().zxy()).with_w(0.0)
                - (other.group0().yzxx() * self.group8().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group6().xyz()) + (self.group7().zxy() * other.group0().yzx()) - (self.group7().yzx() * other.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[e3215]) * self.group7()) + (Simd32x3::from(other[e1234]) * self.group8()) - (Simd32x3::from(self[e321]) * other.group0().xyz()),
            // e415, e425, e435, e321
            (other.group0().yzxw() * self.group9().zxy().with_w(self[e1234])) - (self.group9().yzxw() * other.group0().zxy().with_w(other[e1234])),
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
    //           add/sub      mul      div      pow
    //      f32       27       35        0        0
    //    simd2        0        2        0      N/A
    //    simd3        9       17        0      N/A
    //    simd4       13        8        0      N/A
    // Totals...
    // yes simd       49       62        0      N/A
    //  no simd      106      122        0        0
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
            (Simd32x4::from(self[e12345]) * other.group3())
                + (Simd32x4::from(other[e12345]) * self.group1())
                + Simd32x3::from(0.0).with_w(-(self[e435] * other[e412]) - (self[e423] * other[e415]) - (self[e431] * other[e425]) - (self[e412] * other[e435]))
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(other[e321]) * self.group6().xyz()).with_w(0.0)
                + (self.group7().zxy() * other.group2().yzx()).with_w(0.0)
                + (self.group8().yzx() * other.group0().zxy()).with_w(0.0)
                - (other.group0().yzxx() * self.group8().zxy().with_w(self[e415]))
                - (self.group7().yzx() * other.group2().zxy()).with_w(self[e425] * other[e431]),
            // e5
            (self[e12345] * other[e5]) + (self[e5] * other[e12345])
                - (self[e415] * other[e235])
                - (self[e425] * other[e315])
                - (self[e435] * other[e125])
                - (self[e235] * other[e415])
                - (self[e315] * other[e425])
                - (self[e125] * other[e435]),
            // e15, e25, e35, e45
            (Simd32x4::from(other[e12345]) * self.group3())
                + Simd32x3::from(0.0).with_w(-(self[e4315] * other[e425]) - (self[e4125] * other[e435]))
                + (Simd32x3::from(self[e3215]) * other.group1().xyz()).with_w(0.0)
                + (self.group9().zxy() * other.group2().yzx()).with_w(0.0)
                - (self.group9().yzxx() * other.group2().zxy().with_w(other[e415])),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group1().xyz())
                + (Simd32x3::from(other[e12345]) * self.group4())
                + Simd32x2::from(0.0).with_z((self[e4235] * other[e431]) - (self[e4315] * other[e423]))
                + (self.group9().yz() * other.group0().zx()).with_z(0.0)
                - (self.group9().zx() * other.group0().yz()).with_z(0.0),
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
    //           add/sub      mul      div      pow
    //      f32       24       31        0        0
    //    simd3        8       18        0      N/A
    //    simd4       13        9        0      N/A
    // Totals...
    // yes simd       45       58        0      N/A
    //  no simd      100      121        0        0
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
            (Simd32x4::from(self[e1234]) * Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]))
                + (self.group9().yzxx() * other.group1().zxy().with_w(other[e41]))
                + Simd32x3::from(0.0).with_w(-(self[e42] * other[e4315]) - (self[e43] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group4()).with_w(self[e4315] * other[e42])
                + (self.group5().yzx() * other.group3().zxy()).with_w(self[e4125] * other[e43])
                - (Simd32x4::from(other[e1234]) * self.group3())
                - (other.group3().yzxx() * self.group5().zxy().with_w(self[e41]))
                - (Simd32x3::from(self[e3215]) * other.group0().xyz()).with_w(0.0)
                - (self.group9().zxy() * other.group1().yzx()).with_w(0.0),
            // e5
            (self[e15] * other[e4235]) + (self[e25] * other[e4315]) + (self[e35] * other[e4125]) + (self[e45] * other[e3215])
                - (self[e4235] * other[e15])
                - (self[e4315] * other[e25])
                - (self[e4125] * other[e35])
                - (self[e3215] * other[e45]),
            // e15, e25, e35, e45
            (Simd32x4::from(self[e12345]) * Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]))
                + Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group6().xyz()).with_w(0.0)
                + (self.group8().yzx() * other.group3().zxy()).with_w(0.0)
                - (other.group3().yzxx() * self.group8().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(self[e12345]) * other.group0().xyz()) + (Simd32x3::from(other[e1234]) * self.group6().xyz()) + (self.group7().zxy() * other.group3().yzx())
                - (self.group7().yzx() * other.group3().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e12345]) * other.group1().xyz()) + (Simd32x3::from(other[e1234]) * self.group8()) + (Simd32x3::from(other[e3215]) * self.group7())
                - (Simd32x3::from(self[e321]) * other.group3().xyz()),
            // e415, e425, e435, e321
            (other.group3().yzxw() * self.group9().zxy().with_w(self[e1234])) - (Simd32x4::from([other[e4125], other[e4235], other[e4315], other[e1234]]) * self.group9().yzxw()),
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
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        7        9        0      N/A
    //  no simd       16       16        0        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (self.group0().yzxx() * other.group1().zxy().with_w(other[e41])) + Simd32x3::from(0.0).with_w((other[e42] * self[e4315]) + (other[e43] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (other.group1().yzx() * self.group0().zxy()).with_w(0.0),
            // e5
            -(other[e45] * self[e3215]) - (other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]),
        )
    }
}
impl AntiWedge<AntiDipoleInversion> for Plane {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        1        4        0      N/A
    //    simd4        5        3        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       25       28        0        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125])) + (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (other.group1().wwwx() * self.group0().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(self[e3215]) * other.group1().xyz().with_w(other[e4]))
                + (self.group0().zxyx() * other.group2().yzx().with_w(other[e1]))
                + Simd32x3::from(0.0).with_w((other[e2] * self[e4315]) + (other[e3] * self[e4125]))
                - (other.group2().zxy() * self.group0().yzx()).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiDualNum> for Plane {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ (Simd32x3::from(other[e3215]) * self.group0().xyz()).with_w(0.0))
    }
}
impl AntiWedge<AntiFlatPoint> for Plane {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        2        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        6       11        0        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(other[e321] * -1.0) * self.group0().xyz(),
            // e15, e25, e35
            (other.group0().yzx() * self.group0().zxy()) + Simd32x2::from(0.0).with_z((other[e315] * self[e4235]) * -1.0) - (other.group0().zx() * self.group0().yz()).with_z(0.0),
        )
    }
}
impl AntiWedge<AntiFlector> for Plane {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        5       13        0        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(other[e321] * -1.0) * self.group0().xyz()).with_w((other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125])),
            // e15, e25, e35, e3215
            ((other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiLine> for Plane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        3        4        0      N/A
    //  no simd        9        9        0        0
    fn anti_wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(other[e25] * self[e4315]) - (other[e35] * self[e4125])) + (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
                - (Simd32x4::from([other[e31], other[e12], other[e23], other[e15]]) * self.group0().zxyx()),
        )
    }
}
impl AntiWedge<AntiMotor> for Plane {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        9       12        0        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (Simd32x3::from(other[e3215]) * self.group0().xyz()).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(other[e25] * self[e4315]) - (other[e35] * self[e4125])) + (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
                - (self.group0().zxyx() * other.group0().yzx().with_w(other[e15])),
        )
    }
}
impl AntiWedge<AntiPlane> for Plane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn anti_wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]))
    }
}
impl AntiWedge<AntiScalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<Circle> for Plane {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        3        6        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        6        9        0      N/A
    //  no simd       18       24        0        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            (other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125])) + (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (other.group1().wwwx() * self.group0().xyzx()),
            // e15, e25, e35
            (Simd32x3::from(self[e3215]) * other.group1().xyz()) + (other.group2().yzx() * self.group0().zxy()) - (other.group2().zxy() * self.group0().yzx()),
        )
    }
}
impl AntiWedge<CircleRotor> for Plane {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        1        6        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        6       10        0      N/A
    //  no simd       20       28        0        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            (other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125])) + (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (other.group1().wwwx() * self.group0().xyzx()),
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
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        7        9        0      N/A
    //  no simd       16       16        0        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (self.group0().yzxx() * other.group1().zxy().with_w(other[e41])) + Simd32x3::from(0.0).with_w((other[e42] * self[e4315]) + (other[e43] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (other.group1().yzx() * self.group0().zxy()).with_w(0.0),
            // e5
            -(other[e45] * self[e3215]) - (other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]),
        )
    }
}
impl AntiWedge<DipoleInversion> for Plane {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        9        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        4        0      N/A
    //    simd4        5        3        0      N/A
    // Totals...
    // yes simd        9       17        0      N/A
    //  no simd       28       35        0        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e1234] * -1.0) * self.group0().xyz(),
            // e415, e425, e435, e321
            ((other.group3().yzx() * self.group0().zxy()) + Simd32x2::from(0.0).with_z(other[e4315] * self[e4235] * -1.0) - (other.group3().zx() * self.group0().yz()).with_z(0.0))
                .with_w(other[e1234] * self[e3215] * -1.0),
            // e235, e315, e125, e4
            (self.group0().xyzx() * Simd32x3::from(other[e3215]).with_w(other[e41])) + Simd32x3::from(0.0).with_w((other[e42] * self[e4315]) + (other[e43] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group3().xyz()).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(other[e25] * self[e4315]) - (other[e35] * self[e4125])) + (other.group1().zxy() * self.group0().yzx()).with_w(0.0)
                - (Simd32x4::from(self[e3215]) * other.group0().with_w(other[e45]))
                - (self.group0().zxyx() * other.group1().yzx().with_w(other[e15])),
        )
    }
}
impl AntiWedge<DualNum> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[e12345]) * self.group0())
    }
}
impl AntiWedge<FlatPoint> for Plane {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       13       16        0        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((other.group1().yzx() * self.group0().zxy()) - (other.group1().zxy() * self.group0().yzx())).with_w(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(-(other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group0().xyz()).with_w(0.0)
                - (Simd32x4::from(self[e3215]) * Simd32x4::from([other[e4235], other[e4315], other[e4125], other[e45]])),
        )
    }
}
impl AntiWedge<Line> for Plane {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125]))
                + (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                + (other.group1().yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * other.group1().zxy().with_w(other[e415])),
        )
    }
}
impl AntiWedge<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       13       16        0        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125]))
                + (Simd32x3::from(self[e3215]) * other.group0().xyz()).with_w(0.0)
                + (other.group1().yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * other.group1().zxy().with_w(other[e415])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group0(),
        )
    }
}
impl AntiWedge<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       17        0        0
    //    simd2        0        1        0      N/A
    //    simd3        5       12        0      N/A
    //    simd4        6        3        0      N/A
    // Totals...
    // yes simd       19       33        0      N/A
    //  no simd       47       67        0        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e4] * self[e3215]), 0.0]),
            // e1, e2, e3, e4
            (self.group0().yzxx() * other.group5().zxy().with_w(other[e41])) + Simd32x3::from(0.0).with_w((other[e42] * self[e4315]) + (other[e43] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group4()).with_w(0.0)
                - (other.group5().yzx() * self.group0().zxy()).with_w(0.0),
            // e5
            -(other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]) - (other[e45] * self[e3215]),
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125]))
                + (Simd32x3::from(self[e3215]) * other.group6().xyz()).with_w(0.0)
                + (other.group8().yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * other.group8().zxy().with_w(other[e415])),
            // e41, e42, e43
            (other.group7().zxy() * self.group0().yzx()) - (other.group7().yzx() * self.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e3215]) * other.group7()) - (Simd32x3::from(other[e321]) * self.group0().xyz()),
            // e415, e425, e435, e321
            ((other.group9().yzx() * self.group0().zxy()) + Simd32x2::from(0.0).with_z(other[e4315] * self[e4235] * -1.0) - (other.group9().zx() * self.group0().yz()).with_z(0.0))
                .with_w(other[e1234] * self[e3215] * -1.0),
            // e423, e431, e412
            Simd32x3::from(other[e1234] * -1.0) * self.group0().xyz(),
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
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        3        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        9       13        0        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            (other.group0().yzx() * self.group0().zxy()) + Simd32x2::from(0.0).with_z((other[e4315] * self[e4235]) * -1.0) - (other.group0().zx() * self.group0().yz()).with_z(0.0),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * other.group0().xyz()),
        )
    }
}
impl AntiWedge<RoundPoint> for Plane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        4        0      N/A
    // Totals...
    // yes simd        3       10        0      N/A
    //  no simd        9       19        0        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e1234] * -1.0) * self.group0().xyz(),
            // e415, e425, e435, e321
            ((self.group0().zxy() * other.group0().yzx()) + Simd32x2::from(0.0).with_z(self[e4235] * other[e4315] * -1.0) - (self.group0().yz() * other.group0().zx()).with_z(0.0))
                .with_w(self[e3215] * other[e1234] * -1.0),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * other.group0().xyz()),
        )
    }
}
impl AntiWedge<VersorEven> for Plane {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        5        0      N/A
    //    simd4        6        3        0      N/A
    // Totals...
    // yes simd        9       13        0      N/A
    //  no simd       27       32        0        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0().yzxx() * other.group0().zxy().with_w(other[e1]))
                + Simd32x3::from(0.0).with_w((self[e4315] * other[e2]) + (self[e4125] * other[e3]) + (self[e3215] * other[e4]))
                - (self.group0().zxy() * other.group0().yzx()).with_w(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(self[e4315] * other[e425]) - (self[e4125] * other[e435])) + (Simd32x3::from(self[e3215]) * other.group0().xyz()).with_w(0.0)
                - (self.group0().xyzx() * other.group1().wwwx()),
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
    //           add/sub      mul      div      pow
    //      f32        2        9        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        4        0      N/A
    //    simd4        5        3        0      N/A
    // Totals...
    // yes simd        9       17        0      N/A
    //  no simd       28       35        0        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e1234] * -1.0) * self.group0().xyz(),
            // e415, e425, e435, e321
            ((self.group0().zxy() * other.group3().yzx()) + Simd32x2::from(0.0).with_z(self[e4235] * other[e4315] * -1.0) - (self.group0().yz() * other.group3().zx()).with_z(0.0))
                .with_w(self[e3215] * other[e1234] * -1.0),
            // e235, e315, e125, e4
            (self.group0().xyzx() * Simd32x3::from(other[e3215]).with_w(other[e41])) + Simd32x3::from(0.0).with_w((self[e4315] * other[e42]) + (self[e4125] * other[e43]))
                - (Simd32x3::from(self[e3215]) * other.group3().xyz()).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(self[e4315] * other[e25]) - (self[e4125] * other[e35])) + (self.group0().yzx() * other.group1().zxy()).with_w(0.0)
                - (Simd32x4::from(self[e3215]) * other.group0().xyz().with_w(other[e45]))
                - (self.group0().zxyx() * other.group1().yzx().with_w(other[e15])),
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
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e3215] * self[e4])
    }
}
impl AntiWedge<AntiMotor> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e3215] * self[e4])
    }
}
impl AntiWedge<AntiScalar> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[e12345]) * self.group0(), /* e5 */ other[e12345] * self[e5])
    }
}
impl AntiWedge<CircleRotor> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[e12345]) * self.group0(), /* e5 */ other[e12345] * self[e5])
    }
}
impl AntiWedge<DipoleInversion> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        5        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[e12345]) * self.group0(), /* e5 */ other[e12345] * self[e5])
    }
}
impl AntiWedge<Flector> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[e12345]) * self.group0(), /* e5 */ other[e12345] * self[e5])
    }
}
impl AntiWedge<MultiVector> for RoundPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd        4       10        0        0
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
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //      add/sub      mul      div      pow
    // f32        4        5        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[e12345]) * self.group0(), /* e5 */ self[e5] * other[e12345])
    }
}
impl AntiWedge<VersorOdd> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        5        0        0
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
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e12345] * self[scalar])
    }
}
impl AntiWedge<CircleRotor> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e12345] * self[scalar])
    }
}
impl AntiWedge<DualNum> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e12345] * self[scalar])
    }
}
impl AntiWedge<Motor> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e12345] * self[scalar])
    }
}
impl AntiWedge<MultiVector> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e12345] * self[scalar])
    }
}
impl AntiWedge<VersorEven> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
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
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       20       20        0        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]))
                + (self.group0().yzxx() * other.group1().zxy().with_w(other[e41]))
                + Simd32x3::from(0.0).with_w((other[e42] * self[e4315]) + (other[e43] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (other.group1().yzx() * self.group0().zxy()).with_w(0.0),
            // e5
            -(other[e45] * self[e3215]) - (other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]),
        )
    }
}
impl AntiWedge<AntiDipoleInversion> for Sphere {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        2        6        0      N/A
    //    simd4        6        3        0      N/A
    // Totals...
    // yes simd       11       14        0      N/A
    //  no simd       33       35        0        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group1().xyz()) + (other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125]))
                + (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                + (Simd32x3::from(self[e1234]) * other.group2().xyz()).with_w(0.0)
                - (other.group1().wwwx() * self.group0().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(self[e3215]) * other.group1().xyz().with_w(other[e4]))
                + (self.group0().zxyx() * other.group2().yzx().with_w(other[e1]))
                + Simd32x3::from(0.0).with_w((other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e5] * self[e1234]))
                - (other.group2().zxy() * self.group0().yzx()).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiDualNum> for Sphere {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other[e3215]) * self.group0().xyz().with_w(self[e1234]))
    }
}
impl AntiWedge<AntiFlatPoint> for Sphere {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        3        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        9       13        0        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            (Simd32x3::from(self[e1234]) * other.group0().xyz()) - (Simd32x3::from(other[e321]) * self.group0().xyz()),
            // e15, e25, e35
            (other.group0().yzx() * self.group0().zxy()) + Simd32x2::from(0.0).with_z((other[e315] * self[e4235]) * -1.0) - (other.group0().zx() * self.group0().yz()).with_z(0.0),
        )
    }
}
impl AntiWedge<AntiFlector> for Sphere {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       13       16        0        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x4::from(self[e1234]) * other.group0().xyz().with_w(other[e5]))
                + Simd32x3::from(0.0).with_w((other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]))
                - (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(0.0),
            // e15, e25, e35, e3215
            ((other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiLine> for Sphere {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn anti_wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(other[e25] * self[e4315]) - (other[e35] * self[e4125]))
                + (Simd32x3::from(self[e1234]) * other.group1()).with_w(0.0)
                + (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
                - (Simd32x4::from([other[e31], other[e12], other[e23], other[e15]]) * self.group0().zxyx()),
        )
    }
}
impl AntiWedge<AntiMotor> for Sphere {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       13       16        0        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e3215]) * self.group0().xyz().with_w(self[e1234]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(other[e25] * self[e4315]) - (other[e35] * self[e4125]))
                + (Simd32x3::from(self[e1234]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
                - (self.group0().zxyx() * other.group0().yzx().with_w(other[e15])),
        )
    }
}
impl AntiWedge<AntiPlane> for Sphere {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
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
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        4        8        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       25       30        0        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group1().xyz()) + (other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125]))
                + (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                + (Simd32x3::from(self[e1234]) * other.group2()).with_w(0.0)
                - (other.group1().wwwx() * self.group0().xyzx()),
            // e15, e25, e35
            (Simd32x3::from(self[e3215]) * other.group1().xyz()) + (other.group2().yzx() * self.group0().zxy()) - (other.group2().zxy() * self.group0().yzx()),
        )
    }
}
impl AntiWedge<CircleRotor> for Sphere {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd2        0        2        0      N/A
    //    simd3        5        6        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd       10       15        0      N/A
    //  no simd       29       35        0        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group1().xyz()) + (other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125]))
                + (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                + (Simd32x3::from(self[e1234]) * other.group2().xyz()).with_w(0.0)
                - (other.group1().wwwx() * self.group0().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e3215]) * other.group1().xyz())
                + Simd32x2::from(0.0).with_z((other[e235] * self[e4315]) - (other[e315] * self[e4235]))
                + (other.group2().yz() * self.group0().zx()).with_z(0.0)
                - (other.group2().zx() * self.group0().yz()).with_z(0.0))
            .with_w(other[e12345] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group0(),
        )
    }
}
impl AntiWedge<Dipole> for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       20       20        0        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]))
                + (self.group0().yzxx() * other.group1().zxy().with_w(other[e41]))
                + Simd32x3::from(0.0).with_w((other[e42] * self[e4315]) + (other[e43] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (other.group1().yzx() * self.group0().zxy()).with_w(0.0),
            // e5
            -(other[e45] * self[e3215]) - (other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]),
        )
    }
}
impl AntiWedge<DipoleInversion> for Sphere {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        1        5        0      N/A
    //    simd4        7        5        0      N/A
    // Totals...
    // yes simd       11       15        0      N/A
    //  no simd       34       40        0        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * other.group3().xyz()) - (Simd32x3::from(other[e1234]) * self.group0().xyz()),
            // e415, e425, e435, e321
            (other.group3().yzxw() * self.group0().zxy().with_w(self[e1234])) - (Simd32x4::from([other[e4125], other[e4235], other[e4315], other[e1234]]) * self.group0().yzxw()),
            // e235, e315, e125, e4
            (self.group0().xyzx() * Simd32x3::from(other[e3215]).with_w(other[e41]))
                + Simd32x3::from(0.0).with_w((other[e42] * self[e4315]) + (other[e43] * self[e4125]) + (other[e45] * self[e1234]))
                - (Simd32x3::from(self[e3215]) * other.group3().xyz()).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(other[e25] * self[e4315]) - (other[e35] * self[e4125]))
                + (Simd32x3::from(self[e1234]) * other.group2().xyz()).with_w(0.0)
                + (other.group1().zxy() * self.group0().yzx()).with_w(0.0)
                - (Simd32x4::from(self[e3215]) * other.group0().with_w(other[e45]))
                - (self.group0().zxyx() * other.group1().yzx().with_w(other[e15])),
        )
    }
}
impl AntiWedge<DualNum> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        4        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6       14        0      N/A
    //  no simd       12       26        0        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * other.group1().xyz(),
            // e415, e425, e435, e321
            ((other.group1().yzx() * self.group0().zxy()) + Simd32x2::from(0.0).with_z(other[e4315] * self[e4235] * -1.0) - (other.group1().zx() * self.group0().yz()).with_z(0.0))
                .with_w(other[e3215] * self[e1234]),
            // e235, e315, e125, e4
            ((Simd32x3::from(other[e3215]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * other.group1().xyz())).with_w(other[e45] * self[e1234]),
            // e1, e2, e3, e5
            Simd32x4::from([
                self[e1234],
                self[e1234],
                self[e1234],
                -(other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]) - (other[e45] * self[e3215]),
            ]) * other.group0().xyz().with_w(1.0),
        )
    }
}
impl AntiWedge<Line> for Sphere {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        2        4        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        8       19        0        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                self[e1234],
                self[e1234],
                self[e1234],
                -(other[e415] * self[e4235]) - (other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) * other.group1().with_w(1.0),
            // e15, e25, e35
            (Simd32x3::from(self[e3215]) * other.group0()) + (other.group1().yzx() * self.group0().zxy()) - (other.group1().zxy() * self.group0().yzx()),
        )
    }
}
impl AntiWedge<Motor> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd2        0        2        0      N/A
    //    simd3        3        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        6       12        0      N/A
    //  no simd       12       25        0        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(self[e1234]) * other.group0().xyz().with_w(other[e5]),
            // e23, e31, e12, e45
            Simd32x4::from([
                self[e1234],
                self[e1234],
                self[e1234],
                -(other[e415] * self[e4235]) - (other[e425] * self[e4315]) - (other[e435] * self[e4125]),
            ]) * other.group1().xyz().with_w(1.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e3215]) * other.group0().xyz())
                + Simd32x2::from(0.0).with_z((other[e235] * self[e4315]) - (other[e315] * self[e4235]))
                + (other.group1().yz() * self.group0().zx()).with_z(0.0)
                - (other.group1().zx() * self.group0().yz()).with_z(0.0))
            .with_w(other[e12345] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group0(),
        )
    }
}
impl AntiWedge<MultiVector> for Sphere {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       14        0        0
    //    simd3        6       14        0      N/A
    //    simd4        8        6        0      N/A
    // Totals...
    // yes simd       23       34        0      N/A
    //  no simd       59       80        0        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e4] * self[e3215]) + (other[e5] * self[e1234]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * other.group3())
                + (self.group0().yzxx() * other.group5().zxy().with_w(other[e41]))
                + Simd32x3::from(0.0).with_w((other[e42] * self[e4315]) + (other[e43] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group4()).with_w(0.0)
                - (other.group5().yzx() * self.group0().zxy()).with_w(0.0),
            // e5
            -(other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]) - (other[e45] * self[e3215]),
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125]))
                + (Simd32x3::from(self[e3215]) * other.group6().xyz()).with_w(0.0)
                + (other.group8().yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * other.group8().zxy().with_w(other[e415])),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group6().xyz()) + (other.group7().zxy() * self.group0().yzx()) - (other.group7().yzx() * self.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e3215]) * other.group7()) + (Simd32x3::from(self[e1234]) * other.group8()) - (Simd32x3::from(other[e321]) * self.group0().xyz()),
            // e415, e425, e435, e321
            (other.group9().yzxw() * self.group0().zxy().with_w(self[e1234])) - (self.group0().yzxw() * other.group9().zxy().with_w(other[e1234])),
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
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        4        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        9       17        0        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * other.group0().xyz(),
            // e415, e425, e435, e321
            ((other.group0().yzx() * self.group0().zxy()) + Simd32x2::from(0.0).with_z(other[e4315] * self[e4235] * -1.0) - (other.group0().zx() * self.group0().yz()).with_z(0.0))
                .with_w(other[e3215] * self[e1234]),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * other.group0().xyz()),
        )
    }
}
impl AntiWedge<RoundPoint> for Sphere {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        5        0        0
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
    //           add/sub      mul      div      pow
    //    simd3        2        4        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd       10       20        0        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * other.group0().xyz()) - (Simd32x3::from(other[e1234]) * self.group0().xyz()),
            // e415, e425, e435, e321
            (other.group0().yzxw() * self.group0().zxy().with_w(self[e1234])) - (self.group0().yzxw() * other.group0().zxy().with_w(other[e1234])),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * other.group0().xyz()),
        )
    }
}
impl AntiWedge<VersorEven> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        5        0      N/A
    //    simd4        6        4        0      N/A
    // Totals...
    // yes simd       12       18        0      N/A
    //  no simd       36       41        0        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from(self[e1234]) * other.group1().xyz().with_w(other[e5]))
                + (self.group0().yzxx() * other.group0().zxy().with_w(other[e1]))
                + Simd32x3::from(0.0).with_w((self[e4315] * other[e2]) + (self[e4125] * other[e3]) + (self[e3215] * other[e4]))
                - (self.group0().zxy() * other.group0().yzx()).with_w(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(self[e4315] * other[e425]) - (self[e4125] * other[e435]))
                + (Simd32x3::from(self[e3215]) * other.group0().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e1234]) * other.group2().xyz()).with_w(0.0)
                - (self.group0().xyzx() * other.group1().wwwx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e3215]) * other.group1().xyz()) + (self.group0().zxy() * other.group2().yzx()) + Simd32x2::from(0.0).with_z(self[e4235] * other[e315] * -1.0)
                - (self.group0().yz() * other.group2().zx()).with_z(0.0))
            .with_w(self[e1234] * other[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group0(),
        )
    }
}
impl AntiWedge<VersorOdd> for Sphere {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        1        5        0      N/A
    //    simd4        7        5        0      N/A
    // Totals...
    // yes simd       11       15        0      N/A
    //  no simd       34       40        0        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * other.group3().xyz()) - (Simd32x3::from(other[e1234]) * self.group0().xyz()),
            // e415, e425, e435, e321
            (other.group3().yzxw() * self.group0().zxy().with_w(self[e1234])) - (Simd32x4::from([other[e4125], other[e4235], other[e4315], other[e1234]]) * self.group0().yzxw()),
            // e235, e315, e125, e4
            (self.group0().xyzx() * Simd32x3::from(other[e3215]).with_w(other[e41]))
                + Simd32x3::from(0.0).with_w((self[e4315] * other[e42]) + (self[e4125] * other[e43]) + (self[e1234] * other[e45]))
                - (Simd32x3::from(self[e3215]) * other.group3().xyz()).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(self[e4315] * other[e25]) - (self[e4125] * other[e35]))
                + (Simd32x3::from(self[e1234]) * other.group2().xyz()).with_w(0.0)
                + (self.group0().yzx() * other.group1().zxy()).with_w(0.0)
                - (Simd32x4::from(self[e3215]) * other.group0().xyz().with_w(other[e45]))
                - (self.group0().zxyx() * other.group1().yzx().with_w(other[e15])),
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
    //           add/sub      mul      div      pow
    //      f32       10       11        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       10       14        0      N/A
    //  no simd       10       21        0        0
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
    //           add/sub      mul      div      pow
    //      f32        9       11        0        0
    //    simd3        0        6        0      N/A
    //    simd4        7        4        0      N/A
    // Totals...
    // yes simd       16       21        0      N/A
    //  no simd       37       45        0        0
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
            (Simd32x4::from(self[e12345]) * other.group3())
                + Simd32x3::from(0.0).with_w(-(other[e425] * self[e315]) - (other[e435] * self[e125]) - (other[e315] * self[e425]) - (other[e125] * self[e435]))
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group2().yzx()).with_w(0.0)
                + (other.group2().yzx() * self.group0().zxy()).with_w(0.0)
                - (Simd32x4::from([other[e431], other[e412], other[e423], other[e415]]) * self.group2().zxyx())
                - (other.group2().zxyx() * self.group0().yzx().with_w(self[e415])),
        )
    }
}
impl AntiWedge<AntiDualNum> for VersorEven {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        1        9        0        0
    fn anti_wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(other[e3215]) * self.group0().xyz()).with_w((other[e3215] * self[e4]) + (other[scalar] * self[e12345])),
            // e15, e25, e35, e3215
            Simd32x4::from(other[e3215]) * self.group1().xyz().with_w(self[e12345]),
        )
    }
}
impl AntiWedge<AntiFlatPoint> for VersorEven {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       13       16        0        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * other.group0(),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(other[e315] * self[e425]) - (other[e125] * self[e435]))
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (other.group0().yzx() * self.group0().zxy()).with_w(0.0)
                - (other.group0().zxyx() * self.group0().yzx().with_w(self[e415])),
        )
    }
}
impl AntiWedge<AntiFlector> for VersorEven {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       17       20        0        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * other.group0(),
            // e1, e2, e3, e5
            (Simd32x4::from(self[e12345]) * other.group1())
                + Simd32x3::from(0.0).with_w(-(other[e315] * self[e425]) - (other[e125] * self[e435]))
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (other.group0().yzx() * self.group0().zxy()).with_w(0.0)
                - (other.group0().zxyx() * self.group0().yzx().with_w(self[e415])),
        )
    }
}
impl AntiWedge<AntiLine> for VersorEven {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       12        0        0
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
            (Simd32x3::from(self[e12345]) * other.group1()).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiMotor> for VersorEven {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       16       21        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e12345]) * other.group0())
    }
}
impl AntiWedge<AntiScalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
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
    //           add/sub      mul      div      pow
    //      f32        8       11        0        0
    //    simd3        0        7        0      N/A
    //    simd4        6        2        0      N/A
    // Totals...
    // yes simd       14       20        0      N/A
    //  no simd       32       40        0        0
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
            Simd32x3::from(0.0).with_w(-(other[e435] * self[e125]) - (other[e235] * self[e415]) - (other[e315] * self[e425]) - (other[e125] * self[e435]))
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group2().yzx()).with_w(0.0)
                + (other.group2().yzx() * self.group0().zxy()).with_w(0.0)
                - (Simd32x4::from([other[e431], other[e412], other[e423], other[e415]]) * self.group2().zxyx())
                - (other.group2().zxy() * self.group0().yzx()).with_w(other[e425] * self[e315]),
        )
    }
}
impl AntiWedge<CircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       12        0        0
    //    simd3        1        8        0      N/A
    //    simd4       10        5        0      N/A
    // Totals...
    // yes simd       19       25        0      N/A
    //  no simd       51       56        0        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x3::from(other[e12345]) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * other.group0())).with_w(other[e12345] * self[e12345]),
            // e415, e425, e435, e321
            (Simd32x4::from(other[e12345]) * self.group1()) + (Simd32x4::from(self[e12345]) * other.group1()),
            // e235, e315, e125, e5
            (other.group2() * Simd32x3::from(self[e12345]).with_w(self[e5]))
                + Simd32x3::from(0.0).with_w(
                    -(other[e415] * self[e235])
                        - (other[e425] * self[e315])
                        - (other[e435] * self[e125])
                        - (other[e235] * self[e415])
                        - (other[e315] * self[e425])
                        - (other[e125] * self[e435]),
                )
                + (Simd32x3::from(other[e12345]) * self.group2().xyz()).with_w(0.0),
            // e1, e2, e3, e4
            (Simd32x4::from(other[e12345]) * self.group3())
                + Simd32x3::from(0.0).with_w(-(other[e431] * self[e425]) - (other[e412] * self[e435]) - (other[e425] * self[e431]) - (other[e435] * self[e412]))
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group2().yzx()).with_w(0.0)
                + (other.group2().yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * other.group2().zxy().with_w(other[e415]))
                - (other.group0().yzx() * self.group2().zxy()).with_w(other[e423] * self[e415]),
        )
    }
}
impl AntiWedge<Dipole> for VersorEven {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       10        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        9       13        0      N/A
    //  no simd        9       20        0        0
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
    //           add/sub      mul      div      pow
    //      f32       12       17        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        5        0      N/A
    //    simd4        8        6        0      N/A
    // Totals...
    // yes simd       24       30        0      N/A
    //  no simd       56       60        0        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from(other[e1234]) * self.group1().xyz().with_w(self[e5]))
                + (other.group3().yzxx() * self.group0().zxy().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w(
                    (other[e4125] * self[e3]) + (other[e3215] * self[e4])
                        - (other[e41] * self[e235])
                        - (other[e42] * self[e315])
                        - (other[e43] * self[e125])
                        - (other[e23] * self[e415])
                        - (other[e31] * self[e425])
                        - (other[e12] * self[e435])
                        - (other[e45] * self[e321])
                        - (other[e25] * self[e431])
                        - (other[e35] * self[e412]),
                )
                + (Simd32x3::from(self[e12345]) * other.group0()).with_w(other[e4315] * self[e2])
                - (self.group0().yzxx() * other.group3().zxy().with_w(other[e15])),
            // e23, e31, e12, e45
            (Simd32x4::from(self[e12345]) * other.group1())
                + Simd32x3::from(0.0).with_w(-(other[e4315] * self[e425]) - (other[e4125] * self[e435]))
                + (Simd32x3::from(other[e1234]) * self.group2().xyz()).with_w(0.0)
                + (Simd32x3::from(other[e3215]) * self.group0().xyz()).with_w(0.0)
                - (other.group3().xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e3215]) * self.group1().xyz())
                + (Simd32x3::from(self[e12345]) * other.group2().xyz())
                + Simd32x2::from(0.0).with_z((other[e4315] * self[e235]) - (other[e4235] * self[e315]))
                + (other.group3().zx() * self.group2().yz()).with_z(0.0)
                - (other.group3().yz() * self.group2().zx()).with_z(0.0))
            .with_w(other[e1234] * self[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group3(),
        )
    }
}
impl AntiWedge<DualNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        1        6        0      N/A
    //  no simd        1       17        0        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345]) * self.group1(),
            // e235, e315, e125, e5
            (Simd32x3::from(other[e12345]) * self.group2().xyz()).with_w((other[e5] * self[e12345]) + (other[e12345] * self[e5])),
            // e1, e2, e3, e4
            Simd32x4::from(other[e12345]) * self.group3(),
        )
    }
}
impl AntiWedge<FlatPoint> for VersorEven {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3        8        0        0
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
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        0        4        0      N/A
    //    simd4        7        5        0      N/A
    // Totals...
    // yes simd       13       17        0      N/A
    //  no simd       34       40        0        0
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
            (self.group0() * Simd32x3::from(other[e3215]).with_w(other[e45])) + Simd32x3::from(0.0).with_w(-(other[e4315] * self[e425]) - (other[e4125] * self[e435]))
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
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        0        5        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd       18       24        0        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (Simd32x3::from(self[e12345]) * other.group0()).with_w(0.0),
            // e235, e315, e125, e4
            (Simd32x3::from(self[e12345]) * other.group1()).with_w(-(other[e415] * self[e423]) - (other[e425] * self[e431]) - (other[e435] * self[e412])),
            // e1, e2, e3, e5
            Simd32x3::from(0.0)
                .with_w(-(other[e425] * self[e315]) - (other[e435] * self[e125]) - (other[e235] * self[e415]) - (other[e315] * self[e425]) - (other[e125] * self[e435]))
                + (Simd32x3::from(self[e321]) * other.group0()).with_w(0.0)
                + (other.group1().yzx() * self.group0().zxy()).with_w(0.0)
                - (other.group1().zxy() * self.group0().yzx()).with_w(other[e415] * self[e235]),
        )
    }
}
impl AntiWedge<Motor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        1        4        0      N/A
    //    simd4        6        5        0      N/A
    // Totals...
    // yes simd       13       18        0      N/A
    //  no simd       33       41        0        0
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
            (other.group0() * Simd32x3::from(self[e321]).with_w(self[e4]))
                + Simd32x3::from(0.0).with_w(-(other[e425] * self[e431]) - (other[e435] * self[e412]))
                + (Simd32x3::from(other[e12345]) * self.group3().xyz()).with_w(0.0)
                + (other.group1().yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * other.group1().zxy().with_w(other[e415])),
        )
    }
}
impl AntiWedge<MultiVector> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       27       35        0        0
    //    simd2        0        2        0      N/A
    //    simd3        9       17        0      N/A
    //    simd4       13        8        0      N/A
    // Totals...
    // yes simd       49       62        0      N/A
    //  no simd      106      122        0        0
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
            (Simd32x4::from(other[e12345]) * self.group3())
                + (Simd32x4::from(self[e12345]) * other.group1())
                + Simd32x3::from(0.0).with_w(-(other[e435] * self[e412]) - (other[e423] * self[e415]) - (other[e431] * self[e425]) - (other[e412] * self[e435]))
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * other.group6().xyz()).with_w(0.0)
                + (other.group7().zxy() * self.group2().yzx()).with_w(0.0)
                + (other.group8().yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * other.group8().zxy().with_w(other[e415]))
                - (other.group7().yzx() * self.group2().zxy()).with_w(other[e425] * self[e431]),
            // e5
            (other[e12345] * self[e5]) + (other[e5] * self[e12345])
                - (other[e415] * self[e235])
                - (other[e425] * self[e315])
                - (other[e435] * self[e125])
                - (other[e235] * self[e415])
                - (other[e315] * self[e425])
                - (other[e125] * self[e435]),
            // e15, e25, e35, e45
            (Simd32x4::from(self[e12345]) * other.group3())
                + Simd32x3::from(0.0).with_w(-(other[e4315] * self[e425]) - (other[e4125] * self[e435]))
                + (Simd32x3::from(other[e3215]) * self.group1().xyz()).with_w(0.0)
                + (other.group9().zxy() * self.group2().yzx()).with_w(0.0)
                - (other.group9().yzxx() * self.group2().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group1().xyz())
                + (Simd32x3::from(self[e12345]) * other.group4())
                + Simd32x2::from(0.0).with_z((other[e4235] * self[e431]) - (other[e4315] * self[e423]))
                + (other.group9().yz() * self.group0().zx()).with_z(0.0)
                - (other.group9().zx() * self.group0().yz()).with_z(0.0),
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
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        5        0      N/A
    //    simd4        6        3        0      N/A
    // Totals...
    // yes simd        9       13        0      N/A
    //  no simd       27       32        0        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0().yzxx() * self.group0().zxy().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w((other[e4315] * self[e2]) + (other[e4125] * self[e3]) + (other[e3215] * self[e4]))
                - (other.group0().zxy() * self.group0().yzx()).with_w(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(other[e4315] * self[e425]) - (other[e4125] * self[e435])) + (Simd32x3::from(other[e3215]) * self.group0().xyz()).with_w(0.0)
                - (other.group0().xyzx() * self.group1().wwwx()),
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
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e12345]) * other.group0(), /* e5 */ other[e5] * self[e12345])
    }
}
impl AntiWedge<Scalar> for VersorEven {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[scalar] * self[e12345])
    }
}
impl AntiWedge<Sphere> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        5        0      N/A
    //    simd4        6        4        0      N/A
    // Totals...
    // yes simd       12       18        0      N/A
    //  no simd       36       41        0        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from(other[e1234]) * self.group1().xyz().with_w(self[e5]))
                + (other.group0().yzxx() * self.group0().zxy().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w((other[e4315] * self[e2]) + (other[e4125] * self[e3]) + (other[e3215] * self[e4]))
                - (other.group0().zxy() * self.group0().yzx()).with_w(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(other[e4315] * self[e425]) - (other[e4125] * self[e435]))
                + (Simd32x3::from(other[e3215]) * self.group0().xyz()).with_w(0.0)
                + (Simd32x3::from(other[e1234]) * self.group2().xyz()).with_w(0.0)
                - (other.group0().xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e3215]) * self.group1().xyz()) + (other.group0().zxy() * self.group2().yzx()) + Simd32x2::from(0.0).with_z(other[e4235] * self[e315] * -1.0)
                - (other.group0().yz() * self.group2().zx()).with_z(0.0))
            .with_w(other[e1234] * self[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * other.group0(),
        )
    }
}
impl AntiWedge<VersorEven> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       11        0        0
    //    simd3        1        6        0      N/A
    //    simd4       11        8        0      N/A
    // Totals...
    // yes simd       20       25        0      N/A
    //  no simd       55       61        0        0
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
            (Simd32x4::from(other[e12345]) * self.group3())
                + (Simd32x4::from(self[e12345]) * other.group3())
                + Simd32x3::from(0.0).with_w(-(other[e431] * self[e425]) - (other[e412] * self[e435]) - (other[e425] * self[e431]) - (other[e435] * self[e412]))
                + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group2().yzx()).with_w(0.0)
                + (other.group2().yzx() * self.group0().zxy()).with_w(0.0)
                - (other.group0().yzxx() * self.group2().zxy().with_w(self[e415]))
                - (self.group0().yzxx() * other.group2().zxy().with_w(other[e415])),
        )
    }
}
impl AntiWedge<VersorOdd> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       17        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        4        0      N/A
    //    simd4        8        7        0      N/A
    // Totals...
    // yes simd       25       30        0      N/A
    //  no simd       57       61        0        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from(self[e12345]) * other.group0())
                + (Simd32x4::from(other[e1234]) * self.group1().xyz().with_w(self[e5]))
                + (other.group3().yzxx() * self.group0().zxy().with_w(self[e1]))
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
                - (self.group0().yzxx() * other.group3().zxy().with_w(other[e15])),
            // e23, e31, e12, e45
            (self.group0() * Simd32x3::from(other[e3215]).with_w(other[e45]))
                + Simd32x3::from(0.0).with_w(-(self[e425] * other[e4315]) - (self[e435] * other[e4125]))
                + (Simd32x3::from(self[e12345]) * other.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(other[e1234]) * self.group2().xyz()).with_w(0.0)
                - (self.group1().wwwx() * other.group3().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e12345]) * other.group2().xyz())
                + (Simd32x3::from(other[e3215]) * self.group1().xyz())
                + Simd32x2::from(0.0).with_z((self[e235] * other[e4315]) - (self[e315] * other[e4235]))
                + (self.group2().yz() * other.group3().zx()).with_z(0.0)
                - (self.group2().zx() * other.group3().yz()).with_z(0.0))
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
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       20       20        0        0
    fn anti_wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]))
                + (self.group3().yzxx() * other.group1().zxy().with_w(other[e41]))
                + Simd32x3::from(0.0).with_w((other[e42] * self[e4315]) + (other[e43] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (other.group1().yzx() * self.group3().zxy()).with_w(0.0),
            // e5
            -(other[e45] * self[e3215]) - (other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]),
        )
    }
}
impl AntiWedge<AntiDipoleInversion> for VersorOdd {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       14        0        0
    //    simd3        2        5        0      N/A
    //    simd4        6        4        0      N/A
    // Totals...
    // yes simd       20       23        0      N/A
    //  no simd       42       45        0        0
    fn anti_wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group1().xyz()) + (other.group0().zxy() * self.group3().yzx()) - (other.group0().yzx() * self.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125]))
                + (Simd32x3::from(self[e1234]) * other.group2().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (other.group1().wwwx() * self.group3().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(self[e3215]) * other.group1().xyz().with_w(other[e4]))
                + (self.group3().zxyx() * other.group2().yzx().with_w(other[e1]))
                + Simd32x3::from(0.0).with_w(
                    (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e5] * self[e1234])
                        - (other[e423] * self[e15])
                        - (other[e431] * self[e25])
                        - (other[e412] * self[e35])
                        - (other[e415] * self[e23])
                        - (other[e425] * self[e31])
                        - (other[e435] * self[e12])
                        - (other[e321] * self[e45])
                        - (other[e315] * self[e42])
                        - (other[e125] * self[e43]),
                )
                - (other.group2().zxyx() * self.group3().yzx().with_w(self[e41])),
        )
    }
}
impl AntiWedge<AntiDualNum> for VersorOdd {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
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
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       13       16        0        0
    fn anti_wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(other[e235] * self[e41]) - (other[e315] * self[e42]) - (other[e125] * self[e43]))
                + (Simd32x3::from(self[e1234]) * other.group0().xyz()).with_w(0.0)
                - (Simd32x4::from(other[e321]) * self.group3().xyz().with_w(self[e45])),
            // e15, e25, e35, e3215
            ((other.group0().yzx() * self.group3().zxy()) - (other.group0().zxy() * self.group3().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiFlector> for VersorOdd {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       16       20        0        0
    fn anti_wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x4::from(self[e1234]) * other.group0().xyz().with_w(other[e5]))
                + Simd32x3::from(0.0).with_w(
                    (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125])
                        - (other[e235] * self[e41])
                        - (other[e315] * self[e42])
                        - (other[e125] * self[e43]),
                )
                - (Simd32x4::from(other[e321]) * self.group3().xyz().with_w(self[e45])),
            // e15, e25, e35, e3215
            ((other.group0().yzx() * self.group3().zxy()) - (other.group0().zxy() * self.group3().yzx())).with_w(0.0),
        )
    }
}
impl AntiWedge<AntiLine> for VersorOdd {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn anti_wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(other[e25] * self[e4315]) - (other[e35] * self[e4125]))
                + (Simd32x3::from(self[e1234]) * other.group1()).with_w(0.0)
                + (other.group0().zxy() * self.group3().yzx()).with_w(0.0)
                - (Simd32x4::from([other[e31], other[e12], other[e23], other[e15]]) * self.group3().zxyx()),
        )
    }
}
impl AntiWedge<AntiMotor> for VersorOdd {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       17       20        0        0
    fn anti_wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e3215]) * self.group3().xyz().with_w(self[e1234]),
            // e1, e2, e3, e5
            (other.group1() * Simd32x3::from(self[e1234]).with_w(self[e45]))
                + Simd32x3::from(0.0).with_w(-(other[e25] * self[e4315]) - (other[e35] * self[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group0().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group3().yzx()).with_w(0.0)
                - (self.group3().zxyx() * other.group0().yzx().with_w(other[e15])),
        )
    }
}
impl AntiWedge<AntiPlane> for VersorOdd {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
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
    //           add/sub      mul      div      pow
    //      f32        9       12        0        0
    //    simd3        2        8        0      N/A
    //    simd4        6        1        0      N/A
    // Totals...
    // yes simd       17       21        0      N/A
    //  no simd       39       40        0        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group1().xyz()) + (other.group0().zxy() * self.group3().yzx()) - (other.group0().yzx() * self.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125]))
                + (Simd32x3::from(self[e1234]) * other.group2()).with_w(0.0)
                + (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (other.group1().wwwx() * self.group3().xyzx()),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(
                -(other[e431] * self[e25])
                    - (other[e412] * self[e35])
                    - (other[e415] * self[e23])
                    - (other[e425] * self[e31])
                    - (other[e435] * self[e12])
                    - (other[e321] * self[e45])
                    - (other[e235] * self[e41])
                    - (other[e315] * self[e42])
                    - (other[e125] * self[e43]),
            ) + (Simd32x3::from(self[e3215]) * other.group1().xyz()).with_w(0.0)
                + (other.group2().yzx() * self.group3().zxy()).with_w(0.0)
                - (other.group2().zxy() * self.group3().yzx()).with_w(other[e423] * self[e15]),
        )
    }
}
impl AntiWedge<CircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       15        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        7        0      N/A
    //    simd4        8        4        0      N/A
    // Totals...
    // yes simd       22       28        0      N/A
    //  no simd       54       56        0        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from(other[e12345]) * self.group0())
                + Simd32x3::from(0.0).with_w(
                    -(other[e431] * self[e25])
                        - (other[e412] * self[e35])
                        - (other[e415] * self[e23])
                        - (other[e425] * self[e31])
                        - (other[e435] * self[e12])
                        - (other[e321] * self[e45])
                        - (other[e235] * self[e41])
                        - (other[e315] * self[e42])
                        - (other[e125] * self[e43]),
                )
                + (Simd32x3::from(self[e1234]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group3().yzx()).with_w(0.0)
                - (other.group0().yzx() * self.group3().zxy()).with_w(other[e423] * self[e15]),
            // e23, e31, e12, e45
            (other.group2() * Simd32x3::from(self[e1234]).with_w(self[e45]))
                + Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125]))
                + (Simd32x3::from(other[e12345]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (other.group1().wwwx() * self.group3().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e12345]) * self.group2().xyz())
                + (Simd32x3::from(self[e3215]) * other.group1().xyz())
                + Simd32x2::from(0.0).with_z((other[e235] * self[e4315]) - (other[e315] * self[e4235]))
                + (other.group2().yz() * self.group3().zx()).with_z(0.0)
                - (other.group2().zx() * self.group3().yz()).with_z(0.0))
            .with_w(other[e12345] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group3(),
        )
    }
}
impl AntiWedge<Dipole> for VersorOdd {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       20       20        0        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]))
                + (self.group3().yzxx() * other.group1().zxy().with_w(other[e41]))
                + Simd32x3::from(0.0).with_w((other[e42] * self[e4315]) + (other[e43] * self[e4125]))
                - (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                - (other.group1().yzx() * self.group3().zxy()).with_w(0.0),
            // e5
            -(other[e45] * self[e3215]) - (other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]),
        )
    }
}
impl AntiWedge<DipoleInversion> for VersorOdd {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        1        4        0      N/A
    //    simd4       11       10        0      N/A
    // Totals...
    // yes simd       18       22        0      N/A
    //  no simd       53       60        0        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * other.group3().xyz()) - (Simd32x3::from(other[e1234]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (Simd32x4::from([self[e4125], self[e4235], self[e4315], self[e1234]]) * other.group3().yzxw())
                - (Simd32x4::from([other[e4125], other[e4235], other[e4315], other[e1234]]) * self.group3().yzxw()),
            // e235, e315, e125, e4
            (self.group3().xyzx() * Simd32x3::from(other[e3215]).with_w(other[e41]))
                + Simd32x3::from(0.0).with_w(
                    (other[e42] * self[e4315]) + (other[e43] * self[e4125]) + (other[e45] * self[e1234])
                        - (other[e1234] * self[e45])
                        - (other[e4315] * self[e42])
                        - (other[e4125] * self[e43]),
                )
                - (other.group3().xyzx() * Simd32x3::from(self[e3215]).with_w(self[e41])),
            // e1, e2, e3, e5
            (Simd32x4::from(other[e3215]) * self.group0().xyz().with_w(self[e45]))
                + (other.group3().zxyy() * self.group1().yzx().with_w(self[e25]))
                + (self.group2().wwwx() * other.group2().xyz().with_w(other[e4235]))
                + Simd32x3::from(0.0).with_w((other[e4125] * self[e35]) - (other[e35] * self[e4125]))
                + (other.group1().zxy() * self.group3().yzx()).with_w(0.0)
                - (Simd32x4::from(self[e3215]) * other.group0().with_w(other[e45]))
                - (other.group2().wwwy() * self.group2().xyz().with_w(self[e4315]))
                - (self.group3().zxyx() * other.group1().yzx().with_w(other[e15]))
                - (other.group3().yzx() * self.group1().zxy()).with_w(0.0),
        )
    }
}
impl AntiWedge<DualNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        1        6        0      N/A
    //  no simd        1       17        0        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x3::from(other[e12345]) * self.group0().xyz()).with_w((other[e5] * self[e1234]) + (other[e12345] * self[scalar])),
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
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
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
    //           add/sub      mul      div      pow
    //      f32        5       11        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        4        0      N/A
    //    simd4        6        4        0      N/A
    // Totals...
    // yes simd       13       20        0      N/A
    //  no simd       35       41        0        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * other.group1().xyz(),
            // e415, e425, e435, e321
            ((other.group1().yzx() * self.group3().zxy()) + Simd32x2::from(0.0).with_z(other[e4315] * self[e4235] * -1.0) - (other.group1().zx() * self.group3().yz()).with_z(0.0))
                .with_w(other[e3215] * self[e1234]),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(-(other[e4315] * self[e42]) - (other[e4125] * self[e43]))
                + (Simd32x3::from(other[e3215]) * self.group3().xyz()).with_w(other[e45] * self[e1234])
                - (other.group1().xyzx() * Simd32x3::from(self[e3215]).with_w(self[e41])),
            // e1, e2, e3, e5
            (Simd32x4::from(other[e3215]) * self.group0().xyz().with_w(self[e45]))
                + (other.group1().zxyy() * self.group1().yzx().with_w(self[e25]))
                + (self.group2().wwwx() * other.group0().xyz().with_w(other[e4235]))
                + Simd32x3::from(0.0)
                    .with_w((other[e4125] * self[e35]) - (other[e15] * self[e4235]) - (other[e25] * self[e4315]) - (other[e35] * self[e4125]) - (other[e45] * self[e3215]))
                - (other.group1().yzx() * self.group1().zxy()).with_w(0.0),
        )
    }
}
impl AntiWedge<Line> for VersorOdd {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        0        5        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd       18       24        0        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group0(),
            // e23, e31, e12, e45
            (Simd32x3::from(self[e1234]) * other.group1()).with_w(-(other[e415] * self[e4235]) - (other[e425] * self[e4315]) - (other[e435] * self[e4125])),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(-(other[e425] * self[e31]) - (other[e435] * self[e12]) - (other[e235] * self[e41]) - (other[e315] * self[e42]) - (other[e125] * self[e43]))
                + (Simd32x3::from(self[e3215]) * other.group0()).with_w(0.0)
                + (other.group1().yzx() * self.group3().zxy()).with_w(0.0)
                - (other.group1().zxy() * self.group3().yzx()).with_w(other[e415] * self[e23]),
        )
    }
}
impl AntiWedge<Motor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       13        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        4        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       17       22        0      N/A
    //  no simd       37       41        0        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0() * Simd32x3::from(self[e1234]).with_w(self[scalar]))
                + Simd32x3::from(0.0).with_w(
                    (other[e5] * self[e1234])
                        - (other[e415] * self[e23])
                        - (other[e425] * self[e31])
                        - (other[e435] * self[e12])
                        - (other[e235] * self[e41])
                        - (other[e315] * self[e42])
                        - (other[e125] * self[e43]),
                )
                + (Simd32x3::from(other[e12345]) * self.group0().xyz()).with_w(0.0),
            // e23, e31, e12, e45
            (Simd32x4::from(other[e12345]) * self.group1())
                + Simd32x3::from(0.0).with_w(-(other[e415] * self[e4235]) - (other[e425] * self[e4315]) - (other[e435] * self[e4125]))
                + (Simd32x3::from(self[e1234]) * other.group1().xyz()).with_w(0.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e12345]) * self.group2().xyz())
                + (Simd32x3::from(self[e3215]) * other.group0().xyz())
                + Simd32x2::from(0.0).with_z((other[e235] * self[e4315]) - (other[e315] * self[e4235]))
                + (other.group1().yz() * self.group3().zx()).with_z(0.0)
                - (other.group1().zx() * self.group3().yz()).with_z(0.0))
            .with_w(other[e12345] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group3(),
        )
    }
}
impl AntiWedge<MultiVector> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       24       31        0        0
    //    simd3        8       18        0      N/A
    //    simd4       13        9        0      N/A
    // Totals...
    // yes simd       45       58        0      N/A
    //  no simd      100      121        0        0
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
            (Simd32x4::from(self[e1234]) * other.group3())
                + (self.group3().yzxx() * other.group5().zxy().with_w(other[e41]))
                + Simd32x3::from(0.0).with_w((other[e42] * self[e4315]) + (other[e43] * self[e4125]))
                + (Simd32x3::from(other[e3215]) * self.group0().xyz()).with_w(0.0)
                + (other.group9().zxy() * self.group1().yzx()).with_w(0.0)
                - (Simd32x4::from(other[e1234]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                - (other.group9().yzxx() * self.group1().zxy().with_w(self[e41]))
                - (Simd32x3::from(self[e3215]) * other.group4()).with_w(other[e4315] * self[e42])
                - (other.group5().yzx() * self.group3().zxy()).with_w(other[e4125] * self[e43]),
            // e5
            (other[e4235] * self[e15]) + (other[e4315] * self[e25]) + (other[e4125] * self[e35]) + (other[e3215] * self[e45])
                - (other[e15] * self[e4235])
                - (other[e25] * self[e4315])
                - (other[e35] * self[e4125])
                - (other[e45] * self[e3215]),
            // e15, e25, e35, e45
            (Simd32x4::from(other[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                + Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125]))
                + (Simd32x3::from(self[e3215]) * other.group6().xyz()).with_w(0.0)
                + (other.group8().yzx() * self.group3().zxy()).with_w(0.0)
                - (self.group3().yzxx() * other.group8().zxy().with_w(other[e415])),
            // e41, e42, e43
            (Simd32x3::from(other[e12345]) * self.group0().xyz()) + (Simd32x3::from(self[e1234]) * other.group6().xyz()) + (other.group7().zxy() * self.group3().yzx())
                - (other.group7().yzx() * self.group3().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[e12345]) * self.group1().xyz()) + (Simd32x3::from(self[e1234]) * other.group8()) + (Simd32x3::from(self[e3215]) * other.group7())
                - (Simd32x3::from(other[e321]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (Simd32x4::from([self[e4125], self[e4235], self[e4315], self[e1234]]) * other.group9().yzxw()) - (self.group3().yzxw() * other.group9().zxy().with_w(other[e1234])),
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
    //           add/sub      mul      div      pow
    //      f32        2        7        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        4        0      N/A
    //    simd4        5        3        0      N/A
    // Totals...
    // yes simd        9       15        0      N/A
    //  no simd       28       33        0        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * other.group0().xyz(),
            // e415, e425, e435, e321
            ((other.group0().yzx() * self.group3().zxy()) + Simd32x2::from(0.0).with_z(other[e4315] * self[e4235] * -1.0) - (other.group0().zx() * self.group3().yz()).with_z(0.0))
                .with_w(other[e3215] * self[e1234]),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(-(other[e4315] * self[e42]) - (other[e4125] * self[e43])) + (Simd32x3::from(other[e3215]) * self.group3().xyz()).with_w(0.0)
                - (other.group0().xyzx() * Simd32x3::from(self[e3215]).with_w(self[e41])),
            // e1, e2, e3, e5
            (Simd32x4::from(other[e3215]) * self.group0().xyz().with_w(self[e45]))
                + (other.group0().zxyx() * self.group1().yzx().with_w(self[e15]))
                + Simd32x3::from(0.0).with_w((other[e4315] * self[e25]) + (other[e4125] * self[e35]))
                - (other.group0().yzx() * self.group1().zxy()).with_w(0.0),
        )
    }
}
impl AntiWedge<RoundPoint> for VersorOdd {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        5        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        1        5        0      N/A
    //    simd4        7        5        0      N/A
    // Totals...
    // yes simd       11       15        0      N/A
    //  no simd       34       40        0        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * other.group0().xyz()) - (Simd32x3::from(other[e1234]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (Simd32x4::from([self[e4125], self[e4235], self[e4315], self[e1234]]) * other.group0().yzxw()) - (self.group3().yzxw() * other.group0().zxy().with_w(other[e1234])),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(-(other[e4315] * self[e42]) - (other[e4125] * self[e43]) - (other[e1234] * self[e45]))
                + (Simd32x3::from(other[e3215]) * self.group3().xyz()).with_w(0.0)
                - (other.group0().xyzx() * Simd32x3::from(self[e3215]).with_w(self[e41])),
            // e1, e2, e3, e5
            (Simd32x4::from(other[e3215]) * self.group0().xyz().with_w(self[e45]))
                + (other.group0().zxyx() * self.group1().yzx().with_w(self[e15]))
                + Simd32x3::from(0.0).with_w((other[e4315] * self[e25]) + (other[e4125] * self[e35]))
                - (Simd32x3::from(other[e1234]) * self.group2().xyz()).with_w(0.0)
                - (other.group0().yzx() * self.group1().zxy()).with_w(0.0),
        )
    }
}
impl AntiWedge<VersorEven> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       17        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        4        0      N/A
    //    simd4        8        7        0      N/A
    // Totals...
    // yes simd       25       30        0      N/A
    //  no simd       57       61        0        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from(other[e12345]) * self.group0())
                + (Simd32x4::from(self[e1234]) * other.group1().xyz().with_w(other[e5]))
                + (self.group3().yzxx() * other.group0().zxy().with_w(other[e1]))
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
                - (other.group0().yzxx() * self.group3().zxy().with_w(self[e15])),
            // e23, e31, e12, e45
            (other.group0() * Simd32x3::from(self[e3215]).with_w(self[e45]))
                + Simd32x3::from(0.0).with_w(-(other[e425] * self[e4315]) - (other[e435] * self[e4125]))
                + (Simd32x3::from(other[e12345]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e1234]) * other.group2().xyz()).with_w(0.0)
                - (other.group1().wwwx() * self.group3().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e12345]) * self.group2().xyz())
                + (Simd32x3::from(self[e3215]) * other.group1().xyz())
                + Simd32x2::from(0.0).with_z((other[e235] * self[e4315]) - (other[e315] * self[e4235]))
                + (other.group2().yz() * self.group3().zx()).with_z(0.0)
                - (other.group2().zx() * self.group3().yz()).with_z(0.0))
            .with_w(other[e12345] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345]) * self.group3(),
        )
    }
}
impl AntiWedge<VersorOdd> for VersorOdd {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        1        4        0      N/A
    //    simd4       11       10        0      N/A
    // Totals...
    // yes simd       18       22        0      N/A
    //  no simd       53       60        0        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * other.group3().xyz()) - (Simd32x3::from(other[e1234]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (Simd32x4::from([self[e4125], self[e4235], self[e4315], self[e1234]]) * other.group3().yzxw())
                - (Simd32x4::from([other[e4125], other[e4235], other[e4315], other[e1234]]) * self.group3().yzxw()),
            // e235, e315, e125, e4
            (self.group3().xyzx() * Simd32x3::from(other[e3215]).with_w(other[e41]))
                + Simd32x3::from(0.0).with_w(
                    (other[e42] * self[e4315]) + (other[e43] * self[e4125]) + (other[e45] * self[e1234])
                        - (other[e1234] * self[e45])
                        - (other[e4315] * self[e42])
                        - (other[e4125] * self[e43]),
                )
                - (other.group3().xyzx() * Simd32x3::from(self[e3215]).with_w(self[e41])),
            // e1, e2, e3, e5
            (Simd32x4::from(other[e3215]) * self.group0().xyz().with_w(self[e45]))
                + (other.group3().zxyy() * self.group1().yzx().with_w(self[e25]))
                + (self.group2().wwwx() * other.group2().xyz().with_w(other[e4235]))
                + Simd32x3::from(0.0).with_w((other[e4125] * self[e35]) - (other[e35] * self[e4125]))
                + (other.group1().zxy() * self.group3().yzx()).with_w(0.0)
                - (Simd32x4::from(self[e3215]) * other.group0().xyz().with_w(other[e45]))
                - (other.group2().wwwy() * self.group2().xyz().with_w(self[e4315]))
                - (self.group3().zxyx() * other.group1().yzx().with_w(other[e15]))
                - (other.group3().yzx() * self.group1().zxy()).with_w(0.0),
        )
    }
}
