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
//  Maximum:       111     138       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         5      12       0
//  Average:        11      20       0
//  Maximum:       211     243       0
impl std::ops::Div<WedgeInfix> for AntiCircleRotor {
    type Output = WedgeInfixPartial<AntiCircleRotor>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       30        0
    //    simd3        1        3        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       24       36        0
    //  no simd       35       51        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x3::from(other[scalar]) * self.group0()) + (Simd32x3::from(self[scalar]) * other.group0())).with_w(other[scalar] * self[scalar]),
            // e23, e31, e12, e45
            (Simd32x4::from(other[scalar]) * self.group1()) + (Simd32x4::from(self[scalar]) * other.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other[e15] * self[scalar]) + (other[scalar] * self[e15]),
                (other[e25] * self[scalar]) + (other[scalar] * self[e25]),
                (other[e35] * self[scalar]) + (other[scalar] * self[e35]),
                -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e42] * self[e35]) + (other[e23] * self[e45]) + (other[e45] * self[e23]) + (other[e35] * self[e42]),
                (other[e43] * self[e15]) + (other[e31] * self[e45]) + (other[e45] * self[e31]) + (other[e15] * self[e43]),
                (other[e41] * self[e25]) + (other[e12] * self[e45]) + (other[e45] * self[e12]) + (other[e25] * self[e41]),
                -(other[e12] * self[e35]) - (other[e15] * self[e23]) - (other[e25] * self[e31]) - (other[e35] * self[e12]),
            ]) - (self.group2().yzxx() * other.group0().zxy().with_w(other[e23]))
                - (self.group0().zxy() * other.group2().yzx()).with_w(other[e31] * self[e25]),
        )
    }
}
impl Wedge<AntiDipoleInversion> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       29        0
    //    simd3        3        6        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       24       37        0
    //  no simd       39       55        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self[e42] * other[e3]) + (self[e23] * other[e4]) + (self[scalar] * other[e423]),
                (self[e43] * other[e1]) + (self[e31] * other[e4]) + (self[scalar] * other[e431]),
                (self[e41] * other[e2]) + (self[e12] * other[e4]) + (self[scalar] * other[e412]),
                -(self[e42] * other[e315])
                    - (self[e43] * other[e125])
                    - (self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435])
                    - (self[e45] * other[e321])
                    - (self[e15] * other[e423])
                    - (self[e25] * other[e431])
                    - (self[e35] * other[e412]),
            ]) - (self.group0().zxy() * other.group3().yzx()).with_w(self[e41] * other[e235]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e15] * other[e4]) + (self[scalar] * other[e415]),
                (self[e25] * other[e4]) + (self[scalar] * other[e425]),
                (self[e35] * other[e4]) + (self[scalar] * other[e435]),
                -(self[e31] * other[e2]) - (self[e12] * other[e3]),
            ]) + (self.group0() * other.group3().www()).with_w(self[scalar] * other[e321])
                - (self.group1().wwwx() * other.group3().xyzx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[scalar]) * other.group2().xyz()) + (Simd32x3::from(other[e5]) * self.group1().xyz()) + (self.group2().zxy() * other.group3().yzx())
                - (self.group2().yzx() * other.group3().zxy()))
            .with_w(self[scalar] * other[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group3().xyz().with_w(other[e4]),
        )
    }
}
impl Wedge<AntiDualNum> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other[scalar]) * self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            (self.group2().xyz() * other.group0().yy().with_z(other[scalar])).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[scalar] * other[e3215]),
        )
    }
}
impl Wedge<AntiFlatPoint> for AntiCircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3        8        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[scalar] * other[e321]),
            // e235, e315, e125, e12345
            (other.group0().xyz() * self.group2().www()).with_w(-(self[e41] * other[e235]) - (self[e42] * other[e315]) - (self[e43] * other[e125]) - (self[e45] * other[e321])),
        )
    }
}
impl Wedge<AntiFlector> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       11        0
    //    simd3        3        7        0
    //    simd4        3        1        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       24       36        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                self[e42] * other[e3],
                self[e43] * other[e1],
                self[e41] * other[e2],
                -(self[e42] * other[e315]) - (self[e43] * other[e125]) - (self[e45] * other[e321]),
            ]) - (self.group0().zxy() * other.group1().yzx()).with_w(self[e41] * other[e235]),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(-(self[e31] * other[e2]) - (self[e12] * other[e3])) + (self.group0() * other.group1().www()).with_w(self[scalar] * other[e321])
                - (self.group1().wwwx() * other.group1().xyzx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[scalar]) * other.group0().xyz()) + (Simd32x3::from(other[e5]) * self.group1().xyz()) + (self.group2().zxy() * other.group1().yzx())
                - (self.group2().yzx() * other.group1().zxy()))
            .with_w(self[scalar] * other[e5]),
            // e1, e2, e3, e4
            (other.group1().xyz() * self.group2().www()).with_w(0.0),
        )
    }
}
impl Wedge<AntiLine> for AntiCircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       24        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            (other.group0() * self.group2().www()).with_w(0.0),
            // e15, e25, e35, e1234
            (Simd32x3::from(self[scalar]) * other.group1()).with_w(-(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e42] * other[e35]) + (self[e45] * other[e23]),
                (self[e43] * other[e15]) + (self[e45] * other[e31]),
                (self[e41] * other[e25]) + (self[e45] * other[e12]),
                -(self[e23] * other[e15]) - (self[e31] * other[e25]) - (self[e12] * other[e35]) - (self[e25] * other[e31]) - (self[e35] * other[e12]),
            ]) - (self.group0().zxy() * other.group1().yzx()).with_w(self[e15] * other[e23]),
        )
    }
}
impl Wedge<AntiMotor> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       18        0
    //    simd3        1        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       12       23        0
    //  no simd       20       36        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other[scalar]) * self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            ((Simd32x3::from(self[scalar]) * other.group0().xyz()) + (Simd32x3::from(other[scalar]) * self.group1().xyz())).with_w(self[e45] * other[scalar]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self[e15] * other[scalar]) + (self[scalar] * other[e15]),
                (self[e25] * other[scalar]) + (self[scalar] * other[e25]),
                (self[e35] * other[scalar]) + (self[scalar] * other[e35]),
                -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e45] * other[e23],
                self[e45] * other[e31],
                self[e45] * other[e12],
                -(self[e31] * other[e25]) - (self[e12] * other[e35]) - (self[e15] * other[e23]) - (self[e25] * other[e31]) - (self[e35] * other[e12]),
            ]) + (other.group1().zxyw() * self.group0().yzx().with_w(self[scalar]))
                - (other.group1().yzxx() * self.group0().zxy().with_w(self[e23])),
        )
    }
}
impl Wedge<AntiPlane> for AntiCircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        1        5        0
    //    simd4        3        2        0
    // Totals...
    // yes simd        5       12        0
    //  no simd       16       28        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0().yzx() * other.group0().zxy()) - (self.group0().zxy() * other.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([self[e41] * other[e5], self[e42] * other[e5], self[e43] * other[e5], -(self[e31] * other[e2]) - (self[e12] * other[e3])])
                - (self.group1().wwwx() * other.group0().xyzx()),
            // e235, e315, e125, e4
            (Simd32x3::from(other[e5]) * self.group1().xyz()).with_w(0.0) + (self.group2().zxy() * other.group0().yzx()).with_w(0.0)
                - (self.group2().yzx() * other.group0().zxy()).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(self[scalar]) * other.group0(),
        )
    }
}
impl Wedge<AntiScalar> for AntiCircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[scalar] * other[e12345])
    }
}
impl Wedge<Circle> for AntiCircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd        9       20        0
    fn wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[scalar]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1(),
            // e235, e315, e125, e12345
            (Simd32x3::from(self[scalar]) * other.group2()).with_w(
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
impl Wedge<CircleRotor> for AntiCircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       11        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       14        0
    //  no simd       10       21        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[scalar]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1(),
            // e235, e315, e125, e12345
            (other.group2().xyz() * self.group2().www()).with_w(
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
impl Wedge<Dipole> for AntiCircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       25       40        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1(),
            // e15, e25, e35, e1234
            (Simd32x3::from(self[scalar]) * other.group2()).with_w(
                -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e42] * other[e35]) + (self[e23] * other[e45]) + (self[e45] * other[e23]) + (self[e35] * other[e42]),
                (self[e43] * other[e15]) + (self[e31] * other[e45]) + (self[e45] * other[e31]) + (self[e15] * other[e43]),
                (self[e41] * other[e25]) + (self[e12] * other[e45]) + (self[e45] * other[e12]) + (self[e25] * other[e41]),
                -(self[e12] * other[e35]) - (self[e15] * other[e23]) - (self[e25] * other[e31]) - (self[e35] * other[e12]),
            ]) - (self.group0().zxy() * other.group2().yzx()).with_w(self[e23] * other[e15])
                - (other.group0().zxy() * self.group2().yzx()).with_w(self[e31] * other[e25]),
        )
    }
}
impl Wedge<DipoleInversion> for AntiCircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       25        0
    //    simd3        0        4        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       21       31        0
    //  no simd       30       45        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1(),
            // e15, e25, e35, e1234
            (other.group2().xyz() * self.group2().www()).with_w(
                (self[scalar] * other[e1234])
                    - (self[e41] * other[e23])
                    - (self[e42] * other[e31])
                    - (self[e43] * other[e12])
                    - (self[e23] * other[e41])
                    - (self[e31] * other[e42])
                    - (self[e12] * other[e43]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e23] * other[e45]) + (self[e45] * other[e23]) + (self[e35] * other[e42]) + (self[scalar] * other[e4235]),
                (self[e31] * other[e45]) + (self[e45] * other[e31]) + (self[e15] * other[e43]) + (self[scalar] * other[e4315]),
                (self[e12] * other[e45]) + (self[e45] * other[e12]) + (self[e25] * other[e41]) + (self[scalar] * other[e4125]),
                -(self[e12] * other[e35]) - (self[e15] * other[e23]) - (self[e25] * other[e31]) - (self[e35] * other[e12]),
            ]) + (self.group0().yzx() * other.group2().zxy()).with_w(self[scalar] * other[e3215])
                - (other.group2().yzxx() * self.group0().zxy().with_w(self[e23]))
                - (other.group0().zxy() * self.group2().yzx()).with_w(self[e31] * other[e25]),
        )
    }
}
impl Wedge<DualNum> for AntiCircleRotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0().xx().with_zw(other[e5], other[e12345]) * self.group0().with_w(self[scalar]),
            // e235, e315, e125, e5
            Simd32x4::from(other[e5]) * self.group1().xyz().with_w(self[scalar]),
        )
    }
}
impl Wedge<FlatPoint> for AntiCircleRotor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * other.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e42] * other[e35]) + (self[e23] * other[e45]),
                (self[e43] * other[e15]) + (self[e31] * other[e45]),
                (self[e41] * other[e25]) + (self[e12] * other[e45]),
                -(self[e31] * other[e25]) - (self[e12] * other[e35]),
            ]) - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        )
    }
}
impl Wedge<Flector> for AntiCircleRotor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       20        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * other.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e23] * other[e45]) + (self[scalar] * other[e4235]),
                (self[e31] * other[e45]) + (self[scalar] * other[e4315]),
                (self[e12] * other[e45]) + (self[scalar] * other[e4125]),
                -(self[e31] * other[e25]) - (self[e12] * other[e35]),
            ]) + (self.group0().yzx() * other.group0().zxy()).with_w(self[scalar] * other[e3215])
                - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        )
    }
}
impl Wedge<Line> for AntiCircleRotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x3::from(self[scalar]) * other.group0()).with_w(
                -(self[e41] * other[e235])
                    - (self[e42] * other[e315])
                    - (self[e43] * other[e125])
                    - (self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435]),
            ),
            // e235, e315, e125, e5
            (other.group1() * self.group2().www()).with_w(0.0),
        )
    }
}
impl Wedge<Motor> for AntiCircleRotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7       14        0
    //  no simd       12       20        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                self[scalar] * other[e415],
                self[scalar] * other[e425],
                self[scalar] * other[e435],
                -(self[e41] * other[e235])
                    - (self[e42] * other[e315])
                    - (self[e43] * other[e125])
                    - (self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435]),
            ]) + (self.group0() * other.group1().www()).with_w(self[scalar] * other[e12345]),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[scalar]) * other.group1().xyz()) + (Simd32x3::from(other[e5]) * self.group1().xyz())).with_w(self[scalar] * other[e5]),
        )
    }
}
impl Wedge<MultiVector> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       47        0
    //    simd3        8       15        0
    //    simd4        6        5        0
    // Totals...
    // yes simd       46       67        0
    //  no simd       80      112        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[scalar] * other[scalar],
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group1(),
            // e5
            self[scalar] * other[e5],
            // e15, e25, e35, e45
            (Simd32x4::from(self[scalar]) * other.group3()) + (Simd32x4::from(other[scalar]) * self.group2().xyz().with_w(self[e45])),
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * other.group4()) + (Simd32x3::from(other[scalar]) * self.group0()),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * other.group5()) + (Simd32x3::from(other[scalar]) * self.group1().xyz()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e15] * other[e4]) + (self[scalar] * other[e415]),
                (self[e25] * other[e4]) + (self[scalar] * other[e425]),
                (self[e35] * other[e4]) + (self[scalar] * other[e435]),
                -(self[e31] * other[e2]) - (self[e12] * other[e3]),
            ]) + (Simd32x3::from(other[e5]) * self.group0()).with_w(self[scalar] * other[e321])
                - (self.group1().wwwx() * other.group1().xyzx()),
            // e423, e431, e412
            (Simd32x3::from(self[scalar]) * other.group7()) + (Simd32x3::from(other[e4]) * self.group1().xyz()) + (self.group0().yzx() * other.group1().zxy())
                - (self.group0().zxy() * other.group1().yzx()),
            // e235, e315, e125
            (Simd32x3::from(self[scalar]) * other.group8()) + (Simd32x3::from(other[e5]) * self.group1().xyz()) + (self.group2().zxy() * other.group1().yzx())
                - (self.group2().yzx() * other.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e23] * other[e45]) + (self[e45] * other[e23]) + (self[e35] * other[e42]) + (self[scalar] * other[e4235]),
                (self[e31] * other[e45]) + (self[e45] * other[e31]) + (self[e15] * other[e43]) + (self[scalar] * other[e4315]),
                (self[e12] * other[e45]) + (self[e45] * other[e12]) + (self[e25] * other[e41]) + (self[scalar] * other[e4125]),
                -(self[e23] * other[e15]) - (self[e31] * other[e25]) - (self[e12] * other[e35]) - (self[e35] * other[e12]),
            ]) + (self.group0().yzx() * other.group3().zxy()).with_w(self[scalar] * other[e3215])
                - (self.group2().yzxy() * other.group4().zxy().with_w(other[e31]))
                - (self.group0().zxy() * other.group3().yzx()).with_w(self[e15] * other[e23]),
            // e1234
            (self[scalar] * other[e1234])
                - (self[e41] * other[e23])
                - (self[e42] * other[e31])
                - (self[e43] * other[e12])
                - (self[e23] * other[e41])
                - (self[e31] * other[e42])
                - (self[e12] * other[e43]),
        )
    }
}
impl Wedge<Plane> for AntiCircleRotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self[scalar]) * other.group0())
    }
}
impl Wedge<RoundPoint> for AntiCircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        4        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       20       35        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other[e4]) * self.group1().xyz()) + (self.group0().yzx() * other.group0().zxy()) - (self.group0().zxy() * other.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e41] * other[e5]) + (self[e15] * other[e4]),
                (self[e42] * other[e5]) + (self[e25] * other[e4]),
                (self[e43] * other[e5]) + (self[e35] * other[e4]),
                -(self[e31] * other[e2]) - (self[e12] * other[e3]),
            ]) - (self.group1().wwwx() * other.group0().xyzx()),
            // e235, e315, e125, e4
            ((Simd32x3::from(other[e5]) * self.group1().xyz()) + (self.group2().zxy() * other.group0().yzx()) - (self.group2().yzx() * other.group0().zxy()))
                .with_w(self[scalar] * other[e4]),
            // e1, e2, e3, e5
            Simd32x4::from(self[scalar]) * other.group0().xyz().with_w(other[e5]),
        )
    }
}
impl Wedge<Scalar> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn wedge(self, other: Scalar) -> Self::Output {
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
impl Wedge<Sphere> for AntiCircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group0(),
            // e1234
            self[scalar] * other[e1234],
        )
    }
}
impl Wedge<VersorEven> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       27        0
    //    simd3        3        7        0
    //    simd4        4        2        0
    // Totals...
    // yes simd       22       36        0
    //  no simd       40       56        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self[e23] * other[e4]) + (self[scalar] * other[e423]),
                (self[e31] * other[e4]) + (self[scalar] * other[e431]),
                (self[e12] * other[e4]) + (self[scalar] * other[e412]),
                -(self[e42] * other[e315])
                    - (self[e43] * other[e125])
                    - (self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435])
                    - (self[e45] * other[e321])
                    - (self[e15] * other[e423])
                    - (self[e25] * other[e431])
                    - (self[e35] * other[e412]),
            ]) + (self.group0().yzx() * other.group3().zxy()).with_w(self[scalar] * other[e12345])
                - (self.group0().zxy() * other.group3().yzx()).with_w(self[e41] * other[e235]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e15] * other[e4]) + (self[scalar] * other[e415]),
                (self[e25] * other[e4]) + (self[scalar] * other[e425]),
                (self[e35] * other[e4]) + (self[scalar] * other[e435]),
                -(self[e31] * other[e2]) - (self[e12] * other[e3]),
            ]) + (self.group0() * other.group2().www()).with_w(self[scalar] * other[e321])
                - (self.group1().wwwx() * other.group3().xyzx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[scalar]) * other.group2().xyz()) + (Simd32x3::from(other[e5]) * self.group1().xyz()) + (self.group2().zxy() * other.group3().yzx())
                - (self.group2().yzx() * other.group3().zxy()))
            .with_w(self[scalar] * other[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group3(),
        )
    }
}
impl Wedge<VersorOdd> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       28        0
    //    simd3        1        4        0
    //    simd4        5        4        0
    // Totals...
    // yes simd       23       36        0
    //  no simd       40       56        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x3::from(self[scalar]) * other.group0().xyz()) + (Simd32x3::from(other[scalar]) * self.group0())).with_w(self[scalar] * other[scalar]),
            // e23, e31, e12, e45
            (Simd32x4::from(self[scalar]) * other.group1()) + (Simd32x4::from(other[scalar]) * self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self[scalar] * other[e15],
                self[scalar] * other[e25],
                self[scalar] * other[e35],
                -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]),
            ]) + (self.group2() * other.group0().www().with_w(other[e1234])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e23] * other[e45]) + (self[e45] * other[e23]) + (self[e35] * other[e42]) + (self[scalar] * other[e4235]),
                (self[e31] * other[e45]) + (self[e45] * other[e31]) + (self[e15] * other[e43]) + (self[scalar] * other[e4315]),
                (self[e12] * other[e45]) + (self[e45] * other[e12]) + (self[e25] * other[e41]) + (self[scalar] * other[e4125]),
                -(self[e12] * other[e35]) - (self[e15] * other[e23]) - (self[e25] * other[e31]) - (self[e35] * other[e12]),
            ]) + (self.group0().yzx() * other.group2().zxy()).with_w(self[scalar] * other[e3215])
                - (other.group2().yzxx() * self.group0().zxy().with_w(self[e23]))
                - (self.group2().yzx() * other.group0().zxy()).with_w(self[e31] * other[e25]),
        )
    }
}
impl std::ops::Div<WedgeInfix> for AntiDipoleInversion {
    type Output = WedgeInfixPartial<AntiDipoleInversion>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       29        0
    //    simd3        3        6        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       24       37        0
    //  no simd       39       55        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other[e42] * self[e3]) + (other[e23] * self[e4]) + (other[scalar] * self[e423]),
                (other[e43] * self[e1]) + (other[e31] * self[e4]) + (other[scalar] * self[e431]),
                (other[e41] * self[e2]) + (other[e12] * self[e4]) + (other[scalar] * self[e412]),
                -(other[e42] * self[e315])
                    - (other[e43] * self[e125])
                    - (other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435])
                    - (other[e45] * self[e321])
                    - (other[e15] * self[e423])
                    - (other[e25] * self[e431])
                    - (other[e35] * self[e412]),
            ]) - (other.group0().zxy() * self.group3().yzx()).with_w(other[e41] * self[e235]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other[e15] * self[e4]) + (other[scalar] * self[e415]),
                (other[e25] * self[e4]) + (other[scalar] * self[e425]),
                (other[e35] * self[e4]) + (other[scalar] * self[e435]),
                -(other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]) + (other.group0() * self.group3().www()).with_w(other[scalar] * self[e321])
                - (other.group1().wwwx() * self.group3().xyzx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(other[scalar]) * self.group2().xyz()) + (Simd32x3::from(self[e5]) * other.group1().xyz()) + (other.group2().zxy() * self.group3().yzx())
                - (other.group2().yzx() * self.group3().zxy()))
            .with_w(other[scalar] * self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group3().xyz().with_w(self[e4]),
        )
    }
}
impl Wedge<AntiDipoleInversion> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       10        0
    //    simd3        1        6        0
    //    simd4       10        8        0
    // Totals...
    // yes simd       16       24        0
    //  no simd       48       60        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group3().xyz()) - (Simd32x3::from(other[e4]) * self.group3().xyz()),
            // e23, e31, e12, e45
            (other.group3().zxyw() * self.group3().yzx().with_w(self[e4])) - (self.group3().zxyw() * other.group3().yzx().with_w(other[e4])),
            // e15, e25, e35, e1234
            (self.group3().xyzx() * other.group3().www().with_w(other[e423]))
                + Simd32x3::from(0.0).with_w(
                    (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]) - (other[e4] * self[e321]) - (other[e2] * self[e431]) - (other[e3] * self[e412]),
                )
                - (other.group3().xyz() * self.group3().www()).with_w(other[e1] * self[e423]),
            // e4235, e4315, e4125, e3215
            (other.group3().yzxw() * self.group1().zxyw())
                + (self.group2().wwwz() * other.group2().xyz().with_w(other[e3]))
                + (self.group0() * other.group3().www()).with_w(other[e1] * self[e235])
                + (other.group1().yzx() * self.group3().zxy()).with_w(other[e2] * self[e315])
                - (Simd32x4::from(self[e5]) * other.group0().with_w(other[e321]))
                - (other.group2().wwwy() * self.group2().xyz().with_w(self[e2]))
                - (self.group3().yzxx() * other.group1().zxy().with_w(other[e235]))
                - (other.group3().zxy() * self.group1().yzx()).with_w(other[e125] * self[e3]),
        )
    }
}
impl Wedge<AntiDualNum> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0().yy().with_zw(other[scalar], other[e3215]) * self.group0().with_w(self[e4]),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(other[scalar]) * self.group2().xyz().with_w(self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group3().xyz().with_w(self[e4]),
        )
    }
}
impl Wedge<AntiFlatPoint> for AntiDipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3        8        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (other.group0().xyz() * self.group2().www()).with_w(-(self[e1] * other[e235]) - (self[e2] * other[e315]) - (self[e3] * other[e125]) - (self[e5] * other[e321])),
            // e1234
            self[e4] * other[e321],
        )
    }
}
impl Wedge<AntiFlector> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        1        5        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       11       18        0
    //  no simd       31       40        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e4]) * other.group1().xyz(),
            // e23, e31, e12, e45
            ((self.group3().yzx() * other.group1().zxy()) - (self.group3().zxy() * other.group1().yzx())).with_w(self[e4] * other[e5]),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(-(self[e431] * other[e2]) - (self[e412] * other[e3])) + (self.group3().xyz() * other.group1().www()).with_w(self[e4] * other[e321])
                - (other.group1().xyzx() * self.group3().www().with_w(self[e423])),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(other[e5]) * self.group0().with_w(self[e321]))
                + (self.group2().wwwy() * other.group0().xyz().with_w(other[e2]))
                + (other.group1().yzxx() * self.group1().zxy().with_w(self[e235]))
                + Simd32x3::from(0.0).with_w((self[e125] * other[e3]) - (self[e2] * other[e315]) - (self[e3] * other[e125]) - (self[e5] * other[e321]))
                - (self.group1().yzx() * other.group1().zxy()).with_w(self[e1] * other[e235]),
        )
    }
}
impl Wedge<AntiLine> for AntiDipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       24        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e4]) * other.group0(),
            // e415, e425, e435, e321
            (Simd32x3::from(self[e4]) * other.group1()).with_w(-(self[e1] * other[e23]) - (self[e2] * other[e31]) - (self[e3] * other[e12])),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (self[e2] * other[e35]) + (self[e5] * other[e23]),
                (self[e3] * other[e15]) + (self[e5] * other[e31]),
                (self[e1] * other[e25]) + (self[e5] * other[e12]),
                -(self[e431] * other[e25]) - (self[e412] * other[e35]) - (self[e415] * other[e23]) - (self[e425] * other[e31]) - (self[e435] * other[e12]),
            ]) - (other.group1().yzx() * self.group3().zxy()).with_w(self[e423] * other[e15]),
        )
    }
}
impl Wedge<AntiMotor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       17        0
    //    simd3        3        5        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       24       40        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                self[e4] * other[e23],
                self[e4] * other[e31],
                self[e4] * other[e12],
                -(self[e423] * other[e15])
                    - (self[e431] * other[e25])
                    - (self[e412] * other[e35])
                    - (self[e415] * other[e23])
                    - (self[e425] * other[e31])
                    - (self[e435] * other[e12]),
            ]) + (self.group0() * other.group0().www()).with_w(self[e4] * other[e3215]),
            // e415, e425, e435, e321
            Simd32x4::from([
                self[e4] * other[e15],
                self[e4] * other[e25],
                self[e4] * other[e35],
                -(self[e1] * other[e23]) - (self[e2] * other[e31]) - (self[e3] * other[e12]),
            ]) + (Simd32x4::from(other[scalar]) * self.group1()),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[e5]) * other.group0().xyz()) + (Simd32x3::from(other[scalar]) * self.group2().xyz()) + (self.group3().yzx() * other.group1().zxy())
                - (self.group3().zxy() * other.group1().yzx()))
            .with_w(self[e5] * other[scalar]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group3().xyz().with_w(self[e4]),
        )
    }
}
impl Wedge<AntiPlane> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       14        0
    //    simd3        1        3        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        6       20        0
    //  no simd       17       35        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e4]) * other.group0().xyz(),
            // e23, e31, e12, e45
            ((self.group3().yzx() * other.group0().zxy()) - (self.group3().zxy() * other.group0().yzx())).with_w(self[e4] * other[e5]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e1] * other[e5], self[e2] * other[e5], self[e3] * other[e5], -(self[e431] * other[e2]) - (self[e412] * other[e3])])
                - (other.group0().xyzx() * self.group3().www().with_w(self[e423])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e425] * other[e3] * -1.0,
                self[e435] * other[e1] * -1.0,
                self[e415] * other[e2] * -1.0,
                (self[e315] * other[e2]) + (self[e125] * other[e3]),
            ]) + (Simd32x4::from(other[e5]) * self.group0().with_w(self[e321]))
                + (other.group0().yzxx() * self.group1().zxy().with_w(self[e235])),
        )
    }
}
impl Wedge<Circle> for AntiDipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       20        0
    fn wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e4] * other[e235]) + (self[e3] * other[e425]),
                (self[e4] * other[e315]) + (self[e1] * other[e435]),
                (self[e4] * other[e125]) + (self[e2] * other[e415]),
                -(self[e3] * other[e125]) - (self[e5] * other[e321]),
            ]) - (self.group3().wwwx() * other.group0().with_w(other[e235]))
                - (self.group3().yzx() * other.group1().zxy()).with_w(self[e2] * other[e315]),
            // e1234
            (self[e4] * other[e321]) + (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]),
        )
    }
}
impl Wedge<CircleRotor> for AntiDipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       20        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e4] * other[e235]) + (self[e3] * other[e425]),
                (self[e4] * other[e315]) + (self[e1] * other[e435]),
                (self[e4] * other[e125]) + (self[e2] * other[e415]),
                -(self[e3] * other[e125]) - (self[e5] * other[e321]),
            ]) - (self.group3().yzxy() * other.group1().zxy().with_w(other[e315]))
                - (other.group0() * self.group3().www()).with_w(self[e1] * other[e235]),
            // e1234
            (self[e4] * other[e321]) + (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]),
        )
    }
}
impl Wedge<Dipole> for AntiDipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       24        0
    //    simd3        2        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       29       40        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e4]) * other.group1().xyz()) + (other.group0().yzx() * self.group3().zxy()) - (other.group0().zxy() * self.group3().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e4] * other[e15]) + (self[e5] * other[e41]),
                (self[e4] * other[e25]) + (self[e5] * other[e42]),
                (self[e4] * other[e35]) + (self[e5] * other[e43]),
                -(self[e2] * other[e31]) - (self[e3] * other[e12]),
            ]) - (self.group3().xyzx() * other.group1().wwwx()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (self[e2] * other[e35]) + (self[e5] * other[e23]),
                (self[e3] * other[e15]) + (self[e5] * other[e31]),
                (self[e1] * other[e25]) + (self[e5] * other[e12]),
                -(self[e431] * other[e25])
                    - (self[e412] * other[e35])
                    - (self[e415] * other[e23])
                    - (self[e425] * other[e31])
                    - (self[e435] * other[e12])
                    - (self[e321] * other[e45])
                    - (self[e235] * other[e41])
                    - (self[e315] * other[e42])
                    - (self[e125] * other[e43]),
            ]) - (other.group2().yzx() * self.group3().zxy()).with_w(self[e423] * other[e15]),
        )
    }
}
impl Wedge<DipoleInversion> for AntiDipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        2        4        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       21       28        0
    //  no simd       37       45        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e4]) * other.group1().xyz()) + (other.group0().yzx() * self.group3().zxy()) - (other.group0().zxy() * self.group3().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e4] * other[e15]) + (self[e5] * other[e41]),
                (self[e4] * other[e25]) + (self[e5] * other[e42]),
                (self[e4] * other[e35]) + (self[e5] * other[e43]),
                -(self[e2] * other[e31]) - (self[e3] * other[e12]),
            ]) - (self.group3().xyzx() * other.group1().wwwx()),
            // e235, e315, e125, e12345
            (self.group3().wwwx() * other.group1().xyz().with_w(other[e4235]))
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
                + (self.group3().yzx() * other.group2().zxy()).with_w(self[e4] * other[e3215])
                - (other.group2().yzxx() * self.group3().zxy().with_w(self[e423])),
        )
    }
}
impl Wedge<DualNum> for AntiDipoleInversion {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[e5]) * self.group3().xyz().with_w(self[e4]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e5]) * self.group0().with_w(self[e321]),
        )
    }
}
impl Wedge<FlatPoint> for AntiDipoleInversion {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        9       16        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                self[e4] * other[e15],
                self[e4] * other[e25],
                self[e4] * other[e35],
                -(self[e431] * other[e25]) - (self[e412] * other[e35]) - (self[e321] * other[e45]),
            ]) - (other.group0().wwwx() * self.group3().xyz().with_w(self[e423])),
            // e235, e315, e125, e5
            ((self.group3().yzx() * other.group0().zxy()) - (self.group3().zxy() * other.group0().yzx())).with_w(0.0),
        )
    }
}
impl Wedge<Flector> for AntiDipoleInversion {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        8       10        0
    //  no simd       16       20        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x4::from(self[e4]) * other.group0().xyz().with_w(other[e3215]))
                + Simd32x3::from(0.0).with_w(
                    (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125])
                        - (self[e431] * other[e25])
                        - (self[e412] * other[e35])
                        - (self[e321] * other[e45]),
                )
                - (other.group0().wwwx() * self.group3().xyz().with_w(self[e423])),
            // e235, e315, e125, e5
            ((self.group3().yzx() * other.group0().zxy()) - (self.group3().zxy() * other.group0().yzx())).with_w(0.0),
        )
    }
}
impl Wedge<Line> for AntiDipoleInversion {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e4] * other[e235]) + (self[e3] * other[e425]),
                (self[e4] * other[e315]) + (self[e1] * other[e435]),
                (self[e4] * other[e125]) + (self[e2] * other[e415]),
                -(self[e2] * other[e315]) - (self[e3] * other[e125]),
            ]) - (self.group3().yzxx() * other.group0().zxy().with_w(other[e235])),
        )
    }
}
impl Wedge<Motor> for AntiDipoleInversion {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       20        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[e5]) * self.group3().xyz().with_w(self[e4]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e4] * other[e235]) + (self[e3] * other[e425]),
                (self[e4] * other[e315]) + (self[e1] * other[e435]),
                (self[e4] * other[e125]) + (self[e2] * other[e415]),
                -(self[e2] * other[e315]) - (self[e3] * other[e125]),
            ]) + (Simd32x4::from(other[e5]) * self.group0().with_w(self[e321]))
                - (self.group3().yzxx() * other.group0().zxy().with_w(other[e235])),
        )
    }
}
impl Wedge<MultiVector> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       37        0
    //    simd3        8       17        0
    //    simd4       10        8        0
    // Totals...
    // yes simd       43       62        0
    //  no simd       89      120        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group3().xyz().with_w(self[e4]),
            // e5
            self[e5] * other[scalar],
            // e15, e25, e35, e45
            (Simd32x4::from(other[e5]) * self.group3().xyz().with_w(self[e4])) - (Simd32x4::from(self[e5]) * other.group1()),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group1().xyz()) - (Simd32x3::from(other[e4]) * self.group3().xyz()),
            // e23, e31, e12
            (self.group3().yzx() * other.group1().zxy()) - (self.group3().zxy() * other.group1().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e4] * other[e15]) + (self[e5] * other[e41]),
                (self[e4] * other[e25]) + (self[e5] * other[e42]),
                (self[e4] * other[e35]) + (self[e5] * other[e43]),
                -(self[e2] * other[e31]) - (self[e3] * other[e12]),
            ]) + (Simd32x4::from(other[scalar]) * self.group1())
                - (self.group3().xyz() * other.group3().www()).with_w(self[e1] * other[e23]),
            // e423, e431, e412
            (Simd32x3::from(self[e4]) * other.group5()) + (Simd32x3::from(other[scalar]) * self.group0()) + (other.group4().yzx() * self.group3().zxy())
                - (other.group4().zxy() * self.group3().yzx()),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * other.group5()) + (Simd32x3::from(other[scalar]) * self.group2().xyz()) + (self.group3().yzx() * other.group3().zxy())
                - (self.group3().zxy() * other.group3().yzx()),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(other[e5]) * self.group0().with_w(self[e321]))
                + (other.group1().yzxy() * self.group1().zxy().with_w(self[e315]))
                + (other.group8() * self.group2().www()).with_w(self[e235] * other[e1])
                + (self.group3().zxy() * other.group6().yzx()).with_w(self[e125] * other[e3])
                - (self.group3().yzxw() * other.group6().zxyw())
                - (self.group3().wwwx() * other.group7().with_w(other[e235]))
                - (self.group1().yzx() * other.group1().zxy()).with_w(self[e2] * other[e315])
                - (self.group2().xyz() * other.group1().www()).with_w(self[e3] * other[e125]),
            // e1234
            (self[e4] * other[e321]) + (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412])
                - (self[e423] * other[e1])
                - (self[e431] * other[e2])
                - (self[e412] * other[e3])
                - (self[e321] * other[e4]),
        )
    }
}
impl Wedge<Plane> for AntiDipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (self[e4] * other[e3215]) + (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]),
        )
    }
}
impl Wedge<RoundPoint> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       15        0
    //    simd3        1        3        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       11       22        0
    //  no simd       25       40        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group0().xyz()) - (Simd32x3::from(other[e4]) * self.group3().xyz()),
            // e23, e31, e12, e45
            (self.group3().yzx() * other.group0().zxy()).with_w(self[e4] * other[e5]) - (self.group3().zxyw() * other.group0().yzxw()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self[e1] * other[e5],
                self[e2] * other[e5],
                self[e3] * other[e5],
                -(self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
            ]) - (other.group0().xyzx() * self.group3().www().with_w(self[e423])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * other[e3]) - (self[e235] * other[e4]),
                -(self[e435] * other[e1]) - (self[e315] * other[e4]),
                -(self[e415] * other[e2]) - (self[e125] * other[e4]),
                (self[e315] * other[e2]) + (self[e125] * other[e3]),
            ]) + (Simd32x4::from(other[e5]) * self.group0().with_w(self[e321]))
                + (other.group0().yzxx() * self.group1().zxy().with_w(self[e235])),
        )
    }
}
impl Wedge<Scalar> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn wedge(self, other: Scalar) -> Self::Output {
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
impl Wedge<Sphere> for AntiDipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (self[e4] * other[e3215]) + (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e5] * other[e1234]),
        )
    }
}
impl Wedge<VersorEven> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        7        0
    //    simd4       10        7        0
    // Totals...
    // yes simd       16       25        0
    //  no simd       48       60        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group3().xyz()) - (Simd32x3::from(other[e4]) * self.group3().xyz()),
            // e23, e31, e12, e45
            (self.group3().yzx() * other.group3().zxy()).with_w(self[e4] * other[e5]) - (self.group3().zxyw() * other.group3().yzxw()),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
                (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
            ) + (self.group3().xyz() * other.group2().www()).with_w(self[e4] * other[e321])
                - (other.group3().xyzx() * self.group3().www().with_w(self[e423])),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(other[e5]) * self.group0().with_w(self[e321]))
                + (self.group2().wwwy() * other.group2().xyz().with_w(other[e2]))
                + (other.group3().yzxx() * self.group1().zxy().with_w(self[e235]))
                + (self.group3().zxy() * other.group1().yzx()).with_w(self[e125] * other[e3])
                - (Simd32x4::from(self[e5]) * other.group0().xyz().with_w(other[e321]))
                - (self.group3().yzxz() * other.group1().zxy().with_w(other[e125]))
                - (self.group1().yzx() * other.group3().zxy()).with_w(self[e1] * other[e235])
                - (self.group2().xyz() * other.group3().www()).with_w(self[e2] * other[e315]),
        )
    }
}
impl Wedge<VersorOdd> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        0
    //    simd3        3        7        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       23       34        0
    //  no simd       47       60        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group3().zxyy() * other.group0().yzx().with_w(other[e4315]))
                + Simd32x3::from(0.0).with_w(
                    (self[e3] * other[e4125]) + (self[e5] * other[e1234])
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
                + (self.group0() * other.group0().www()).with_w(self[e4] * other[e3215])
                + (other.group1().xyz() * self.group2().www()).with_w(self[e1] * other[e4235])
                - (self.group3().yzx() * other.group0().zxy()).with_w(self[e423] * other[e15]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e4] * other[e15]) + (self[e5] * other[e41]),
                (self[e4] * other[e25]) + (self[e5] * other[e42]),
                (self[e4] * other[e35]) + (self[e5] * other[e43]),
                -(self[e2] * other[e31]) - (self[e3] * other[e12]),
            ]) + (Simd32x4::from(other[scalar]) * self.group1())
                - (self.group3().xyzx() * other.group1().wwwx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[e5]) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * self.group2().xyz()) + (self.group3().yzx() * other.group2().zxy())
                - (self.group3().zxy() * other.group2().yzx()))
            .with_w(self[e5] * other[scalar]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group3().xyz().with_w(self[e4]),
        )
    }
}
impl std::ops::Div<WedgeInfix> for AntiDualNum {
    type Output = WedgeInfixPartial<AntiDualNum>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for AntiDualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(self[scalar]) * other.group0().with_w(other[scalar]),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1(),
            // e15, e25, e35, e1234
            (other.group2().xyz() * self.group0().yy().with_z(self[scalar])).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(other[scalar] * self[e3215]),
        )
    }
}
impl Wedge<AntiDipoleInversion> for AntiDualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().yy().with_zw(self[scalar], self[e3215]) * other.group0().with_w(other[e4]),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(self[scalar]) * other.group2().xyz().with_w(other[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group3().xyz().with_w(other[e4]),
        )
    }
}
impl Wedge<AntiDualNum> for AntiDualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(
            // e3215, scalar
            Simd32x2::from([(other[e3215] * self[scalar]) + (other[scalar] * self[e3215]), other[scalar] * self[scalar]]),
        )
    }
}
impl Wedge<AntiFlatPoint> for AntiDualNum {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(self[scalar]) * other.group0())
    }
}
impl Wedge<AntiFlector> for AntiDualNum {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[scalar]) * other.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(self[scalar]) * other.group1(),
        )
    }
}
impl Wedge<AntiLine> for AntiDualNum {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group0(),
            // e15, e25, e35
            Simd32x3::from(self[scalar]) * other.group1(),
        )
    }
}
impl Wedge<AntiMotor> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1       10        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(self[scalar]) * other.group0(),
            // e15, e25, e35, e3215
            self.group0().yy().with_zw(self[scalar], (self[e3215] * other[scalar]) + (self[scalar] * other[e3215])) * other.group1().xyz().with_w(1.0),
        )
    }
}
impl Wedge<AntiPlane> for AntiDualNum {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[scalar]) * other.group0())
    }
}
impl Wedge<AntiScalar> for AntiDualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[scalar] * other[e12345])
    }
}
impl Wedge<Circle> for AntiDualNum {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[scalar]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1(),
            // e235, e315, e125
            Simd32x3::from(self[scalar]) * other.group2(),
        )
    }
}
impl Wedge<CircleRotor> for AntiDualNum {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[scalar]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(self[scalar]) * other.group2(),
        )
    }
}
impl Wedge<Dipole> for AntiDualNum {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1(),
            // e15, e25, e35
            Simd32x3::from(self[scalar]) * other.group2(),
        )
    }
}
impl Wedge<DipoleInversion> for AntiDualNum {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(self[scalar]) * other.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group3(),
        )
    }
}
impl Wedge<DualNum> for AntiDualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(self[scalar]) * other.group0())
    }
}
impl Wedge<FlatPoint> for AntiDualNum {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self[scalar]) * other.group0())
    }
}
impl Wedge<Flector> for AntiDualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * other.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group1(),
        )
    }
}
impl Wedge<Line> for AntiDualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(self[scalar]) * other.group0(),
            // e235, e315, e125
            Simd32x3::from(self[scalar]) * other.group1(),
        )
    }
}
impl Wedge<Motor> for AntiDualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(self[scalar]) * other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(self[scalar]) * other.group1(),
        )
    }
}
impl Wedge<MultiVector> for AntiDualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        7        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       15        0
    //  no simd        2       35        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] * other[scalar], (self[e3215] * other[e4]) + (self[scalar] * other[e12345])]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group1(),
            // e5
            self[scalar] * other[e5],
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * other.group3(),
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group4(),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group6(),
            // e423, e431, e412
            Simd32x3::from(self[scalar]) * other.group7(),
            // e235, e315, e125
            Simd32x3::from(self[scalar]) * other.group8(),
            // e4235, e4315, e4125, e3215
            self.group0().yy().with_zw(self[scalar], (self[e3215] * other[scalar]) + (self[scalar] * other[e3215])) * other.group9().xyz().with_w(1.0),
            // e1234
            self[scalar] * other[e1234],
        )
    }
}
impl Wedge<Plane> for AntiDualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self[scalar]) * other.group0())
    }
}
impl Wedge<RoundPoint> for AntiDualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(self[e3215] * other[e4]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(self[scalar] * other[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group0(),
        )
    }
}
impl Wedge<Scalar> for AntiDualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(other[scalar]) * self.group0())
    }
}
impl Wedge<Sphere> for AntiDualNum {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group0(),
            // e1234
            self[scalar] * other[e1234],
        )
    }
}
impl Wedge<VersorEven> for AntiDualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1       18        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().yy().with_zw(self[scalar], (self[e3215] * other[e4]) + (self[scalar] * other[e12345])) * other.group0().xyz().with_w(1.0),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(self[scalar]) * other.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group3(),
        )
    }
}
impl Wedge<VersorOdd> for AntiDualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1       18        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(self[scalar]) * other.group2(),
            // e4235, e4315, e4125, e3215
            self.group0().yy().with_zw(self[scalar], (self[e3215] * other[scalar]) + (self[scalar] * other[e3215])) * other.group3().xyz().with_w(1.0),
        )
    }
}
impl std::ops::Div<WedgeInfix> for AntiFlatPoint {
    type Output = WedgeInfixPartial<AntiFlatPoint>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for AntiFlatPoint {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3        8        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[scalar] * self[e321]),
            // e235, e315, e125, e12345
            (self.group0().xyz() * other.group2().www()).with_w(-(other[e41] * self[e235]) - (other[e42] * self[e315]) - (other[e43] * self[e125]) - (other[e45] * self[e321])),
        )
    }
}
impl Wedge<AntiDipoleInversion> for AntiFlatPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       12        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (self.group0().xyz() * other.group2().www() * Simd32x3::from(-1.0))
                .with_w((other[e1] * self[e235]) + (other[e2] * self[e315]) + (other[e3] * self[e125]) + (other[e5] * self[e321])),
            // e1234
            other[e4] * self[e321] * -1.0,
        )
    }
}
impl Wedge<AntiDualNum> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl Wedge<AntiFlector> for AntiFlatPoint {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            (self[e235] * other[e1]) + (self[e315] * other[e2]) + (self[e125] * other[e3]) + (self[e321] * other[e5]),
            0.0,
        ]))
    }
}
impl Wedge<AntiMotor> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl Wedge<AntiPlane> for AntiFlatPoint {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            (self[e235] * other[e1]) + (self[e315] * other[e2]) + (self[e125] * other[e3]) + (self[e321] * other[e5]),
            0.0,
        ]))
    }
}
impl Wedge<Dipole> for AntiFlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            -(self[e235] * other[e41]) - (self[e315] * other[e42]) - (self[e125] * other[e43]) - (self[e321] * other[e45]),
        )
    }
}
impl Wedge<DipoleInversion> for AntiFlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            -(self[e235] * other[e41]) - (self[e315] * other[e42]) - (self[e125] * other[e43]) - (self[e321] * other[e45]),
        )
    }
}
impl Wedge<DualNum> for AntiFlatPoint {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([self[e321] * other[e5], 1.0]) * Simd32x2::from([1.0, 0.0]))
    }
}
impl Wedge<FlatPoint> for AntiFlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e321] * other[e45] * -1.0)
    }
}
impl Wedge<Flector> for AntiFlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e321] * other[e45] * -1.0)
    }
}
impl Wedge<Motor> for AntiFlatPoint {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([self[e321] * other[e5], 1.0]) * Simd32x2::from([1.0, 0.0]))
    }
}
impl Wedge<MultiVector> for AntiFlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       11        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        6       14        0
    //  no simd        6       20        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, -(self[e235] * other[e41]) - (self[e315] * other[e42]) - (self[e125] * other[e43]) - (self[e321] * other[e45])]),
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
            Simd32x3::from(0.0).with_w(self[e321] * other[scalar]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(other[e4]) * self.group0().xyz() * Simd32x3::from(-1.0))
                .with_w((self[e235] * other[e1]) + (self[e315] * other[e2]) + (self[e125] * other[e3]) + (self[e321] * other[e5])),
            // e1234
            self[e321] * other[e4] * -1.0,
        )
    }
}
impl Wedge<RoundPoint> for AntiFlatPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       12        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(other[e4]) * self.group0().xyz() * Simd32x3::from(-1.0))
                .with_w((self[e235] * other[e1]) + (self[e315] * other[e2]) + (self[e125] * other[e3]) + (self[e321] * other[e5])),
            // e1234
            self[e321] * other[e4] * -1.0,
        )
    }
}
impl Wedge<Scalar> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl Wedge<VersorEven> for AntiFlatPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       12        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(other[e4]) * self.group0().xyz() * Simd32x3::from(-1.0))
                .with_w((self[e235] * other[e1]) + (self[e315] * other[e2]) + (self[e125] * other[e3]) + (self[e321] * other[e5])),
            // e1234
            self[e321] * other[e4] * -1.0,
        )
    }
}
impl Wedge<VersorOdd> for AntiFlatPoint {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3        8        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321] * other[scalar]),
            // e235, e315, e125, e12345
            (Simd32x3::from(other[scalar]) * self.group0().xyz())
                .with_w(-(self[e235] * other[e41]) - (self[e315] * other[e42]) - (self[e125] * other[e43]) - (self[e321] * other[e45])),
        )
    }
}
impl std::ops::Div<WedgeInfix> for AntiFlector {
    type Output = WedgeInfixPartial<AntiFlector>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       11        0
    //    simd3        3        7        0
    //    simd4        3        1        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       24       36        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                other[e42] * self[e3],
                other[e43] * self[e1],
                other[e41] * self[e2],
                -(other[e42] * self[e315]) - (other[e43] * self[e125]) - (other[e45] * self[e321]),
            ]) - (other.group0().zxy() * self.group1().yzx()).with_w(other[e41] * self[e235]),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(-(other[e31] * self[e2]) - (other[e12] * self[e3])) + (other.group0() * self.group1().www()).with_w(other[scalar] * self[e321])
                - (other.group1().wwwx() * self.group1().xyzx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(other[scalar]) * self.group0().xyz()) + (Simd32x3::from(self[e5]) * other.group1().xyz()) + (other.group2().zxy() * self.group1().yzx())
                - (other.group2().yzx() * self.group1().zxy()))
            .with_w(other[scalar] * self[e5]),
            // e1, e2, e3, e4
            (self.group1().xyz() * other.group2().www()).with_w(0.0),
        )
    }
}
impl Wedge<AntiDipoleInversion> for AntiFlector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd3        1        6        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       11       20        0
    //  no simd       31       44        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e4]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            ((other.group3().zxy() * self.group1().yzx()) - (other.group3().yzx() * self.group1().zxy())).with_w(other[e4] * self[e5] * -1.0),
            // e15, e25, e35, e1234
            (self.group1().xyzx() * other.group3().www().with_w(other[e423])) + Simd32x3::from(0.0).with_w((other[e431] * self[e2]) + (other[e412] * self[e3]))
                - (other.group3().xyz() * self.group1().www()).with_w(other[e4] * self[e321]),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w((other[e2] * self[e315]) + (other[e3] * self[e125]) + (other[e5] * self[e321]) - (other[e125] * self[e3]))
                + (other.group1().yzx() * self.group1().zxy()).with_w(other[e1] * self[e235])
                - (Simd32x4::from(self[e5]) * other.group0().with_w(other[e321]))
                - (other.group2().wwwy() * self.group0().xyz().with_w(self[e2]))
                - (self.group1().yzxx() * other.group1().zxy().with_w(other[e235])),
        )
    }
}
impl Wedge<AntiDualNum> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl Wedge<AntiFlatPoint> for AntiFlector {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            -(other[e235] * self[e1]) - (other[e315] * self[e2]) - (other[e125] * self[e3]) - (other[e321] * self[e5]),
            0.0,
        ]))
    }
}
impl Wedge<AntiFlector> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        8       10        0
    //  no simd       16       20        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            ((other.group1().zxy() * self.group1().yzx()) - (other.group1().yzx() * self.group1().zxy())).with_w(0.0),
            // e15, e25, e35, e3215
            (other.group1().wwwx() * self.group1().xyz().with_w(self[e235]))
                + Simd32x3::from(0.0).with_w(
                    (other[e2] * self[e315]) + (other[e3] * self[e125]) + (other[e5] * self[e321]) - (other[e315] * self[e2]) - (other[e125] * self[e3]) - (other[e321] * self[e5]),
                )
                - (self.group1().wwwx() * other.group1().xyz().with_w(other[e235])),
        )
    }
}
impl Wedge<AntiLine> for AntiFlector {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([
                (self[e2] * other[e35]) + (self[e5] * other[e23]),
                (self[e3] * other[e15]) + (self[e5] * other[e31]),
                (self[e1] * other[e25]) + (self[e5] * other[e12]),
                -(self[e2] * other[e31]) - (self[e3] * other[e12]),
            ]) - (self.group1().zxyx() * other.group1().yzx().with_w(other[e23])),
        )
    }
}
impl Wedge<AntiMotor> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       20        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([
                (self[e2] * other[e35]) + (self[e5] * other[e23]),
                (self[e3] * other[e15]) + (self[e5] * other[e31]),
                (self[e1] * other[e25]) + (self[e5] * other[e12]),
                -(self[e2] * other[e31]) - (self[e3] * other[e12]),
            ]) + (Simd32x4::from(other[scalar]) * self.group0())
                - (self.group1().zxyx() * other.group1().yzx().with_w(other[e23])),
            // e1, e2, e3, e5
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl Wedge<AntiPlane> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        9       19        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            ((self.group1().yzx() * other.group0().zxy()) - (self.group1().zxy() * other.group0().yzx())).with_w(0.0),
            // e15, e25, e35, e3215
            Simd32x4::from([
                self[e5] * other[e1] * -1.0,
                self[e5] * other[e2] * -1.0,
                self[e5] * other[e3] * -1.0,
                (self[e315] * other[e2]) + (self[e125] * other[e3]) + (self[e321] * other[e5]),
            ]) + (other.group0().wwwx() * self.group1().xyz().with_w(self[e235])),
        )
    }
}
impl Wedge<Circle> for AntiFlector {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd       11       16        0
    fn wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e3] * other[e425],
                self[e1] * other[e435],
                self[e2] * other[e415],
                -(self[e3] * other[e125]) - (self[e5] * other[e321]),
            ]) - (self.group1().wwwx() * other.group0().with_w(other[e235]))
                - (self.group1().yzx() * other.group1().zxy()).with_w(self[e2] * other[e315]),
            // e1234
            (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]),
        )
    }
}
impl Wedge<CircleRotor> for AntiFlector {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd       11       16        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e3] * other[e425],
                self[e1] * other[e435],
                self[e2] * other[e415],
                -(self[e3] * other[e125]) - (self[e5] * other[e321]),
            ]) - (self.group1().yzxy() * other.group1().zxy().with_w(other[e315]))
                - (other.group0() * self.group1().www()).with_w(self[e1] * other[e235]),
            // e1234
            (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]),
        )
    }
}
impl Wedge<Dipole> for AntiFlector {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       15        0
    //    simd3        1        3        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       17       28        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            (other.group0().yzx() * self.group1().zxy()) - (other.group0().zxy() * self.group1().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([self[e5] * other[e41], self[e5] * other[e42], self[e5] * other[e43], -(self[e2] * other[e31]) - (self[e3] * other[e12])])
                - (self.group1().xyzx() * other.group1().wwwx()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (self[e2] * other[e35]) + (self[e5] * other[e23]),
                (self[e3] * other[e15]) + (self[e5] * other[e31]),
                (self[e1] * other[e25]) + (self[e5] * other[e12]),
                -(self[e315] * other[e42]) - (self[e125] * other[e43]) - (self[e321] * other[e45]),
            ]) - (other.group2().yzx() * self.group1().zxy()).with_w(self[e235] * other[e41]),
        )
    }
}
impl Wedge<DipoleInversion> for AntiFlector {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        3        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       10       17        0
    //  no simd       24       32        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            (other.group0().yzx() * self.group1().zxy()) - (other.group0().zxy() * self.group1().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([self[e5] * other[e41], self[e5] * other[e42], self[e5] * other[e43], -(self[e2] * other[e31]) - (self[e3] * other[e12])])
                - (self.group1().xyzx() * other.group1().wwwx()),
            // e235, e315, e125, e12345
            (self.group1().yzxx() * other.group2().zxy().with_w(other[e4235]))
                + (self.group1().wwwy() * other.group1().xyz().with_w(other[e4315]))
                + Simd32x3::from(0.0)
                    .with_w((self[e3] * other[e4125]) + (self[e5] * other[e1234]) - (self[e315] * other[e42]) - (self[e125] * other[e43]) - (self[e321] * other[e45]))
                - (self.group1().zxy() * other.group2().yzx()).with_w(self[e235] * other[e41]),
        )
    }
}
impl Wedge<DualNum> for AntiFlector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            (self.group1().xyz() * other.group0().xx().with_z(other[e5])).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e321] * other[e5]),
        )
    }
}
impl Wedge<FlatPoint> for AntiFlector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        3       14        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(other[e45]) * self.group1().xyz().with_w(self[e321]) * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            ((self.group1().yzx() * other.group0().zxy()) - (self.group1().zxy() * other.group0().yzx())).with_w(0.0),
        )
    }
}
impl Wedge<Flector> for AntiFlector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        1        4        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        6       16        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x3::from(other[e45]) * self.group1().xyz() * Simd32x3::from(-1.0))
                .with_w((self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]) - (self[e321] * other[e45])),
            // e235, e315, e125, e5
            ((self.group1().yzx() * other.group0().zxy()) - (self.group1().zxy() * other.group0().yzx())).with_w(0.0),
        )
    }
}
impl Wedge<Line> for AntiFlector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        5        9        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e3] * other[e425],
                self[e1] * other[e435],
                self[e2] * other[e415],
                -(self[e2] * other[e315]) - (self[e3] * other[e125]),
            ]) - (self.group1().yzxx() * other.group0().zxy().with_w(other[e235])),
        )
    }
}
impl Wedge<Motor> for AntiFlector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        0        2        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        9       13        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            (self.group1().xyz() * other.group1().www()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(-(self[e2] * other[e315]) - (self[e3] * other[e125])) + (self.group1().zxy() * other.group0().yzx()).with_w(self[e321] * other[e5])
                - (self.group1().yzxx() * other.group0().zxy().with_w(other[e235])),
        )
    }
}
impl Wedge<MultiVector> for AntiFlector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       26        0
    //    simd3        6       18        0
    //    simd4        6        1        0
    // Totals...
    // yes simd       26       45        0
    //  no simd       56       84        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e5] * other[e1234])
                    - (self[e235] * other[e41])
                    - (self[e315] * other[e42])
                    - (self[e125] * other[e43])
                    - (self[e321] * other[e45]),
            ]),
            // e1, e2, e3, e4
            (self.group1().xyz() * other.group0().xx().with_z(other[scalar])).with_w(0.0),
            // e5
            self[e5] * other[scalar],
            // e15, e25, e35, e45
            ((Simd32x3::from(other[e5]) * self.group1().xyz()) - (Simd32x3::from(self[e5]) * other.group1().xyz())).with_w(self[e5] * other[e4] * -1.0),
            // e41, e42, e43
            Simd32x3::from(other[e4]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            (self.group1().yzx() * other.group1().zxy()) - (self.group1().zxy() * other.group1().yzx()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(-(self[e2] * other[e31]) - (self[e3] * other[e12])) + (other.group4() * self.group1().www()).with_w(self[e321] * other[scalar])
                - (self.group1().xyz() * other.group3().www()).with_w(self[e1] * other[e23]),
            // e423, e431, e412
            (other.group4().yzx() * self.group1().zxy()) - (other.group4().zxy() * self.group1().yzx()),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * other.group5()) + (Simd32x3::from(other[scalar]) * self.group0().xyz()) + (self.group1().yzx() * other.group3().zxy())
                - (self.group1().zxy() * other.group3().yzx()),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w((self[e315] * other[e2]) + (self[e125] * other[e3]) + (self[e321] * other[e5]) - (self[e5] * other[e321]))
                + (self.group1().zxy() * other.group6().yzx()).with_w(self[e235] * other[e1])
                - (self.group1().wwwx() * other.group7().with_w(other[e235]))
                - (self.group1().yzx() * other.group6().zxy()).with_w(self[e3] * other[e125])
                - (self.group0().xyz() * other.group1().www()).with_w(self[e2] * other[e315]),
            // e1234
            (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) - (self[e321] * other[e4]),
        )
    }
}
impl Wedge<Plane> for AntiFlector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]))
    }
}
impl Wedge<RoundPoint> for AntiFlector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd3        2        8        0
    // Totals...
    // yes simd        5       16        0
    //  no simd        9       32        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e4]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            ((self.group1().yzx() * other.group0().zxy()) - (self.group1().zxy() * other.group0().yzx())).with_w(self[e5] * other[e4] * -1.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e5]) * self.group1().xyz()) - (Simd32x3::from(self[e5]) * other.group0().xyz())).with_w(self[e321] * other[e4] * -1.0),
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(other[e4]) * self.group0().xyz() * Simd32x3::from(-1.0))
                .with_w((self[e235] * other[e1]) + (self[e315] * other[e2]) + (self[e125] * other[e3]) + (self[e321] * other[e5])),
        )
    }
}
impl Wedge<Scalar> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl Wedge<Sphere> for AntiFlector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e5] * other[e1234]),
        )
    }
}
impl Wedge<VersorEven> for AntiFlector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd3        1        6        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       11       20        0
    //  no simd       31       44        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e4]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            ((self.group1().yzx() * other.group3().zxy()) - (self.group1().zxy() * other.group3().yzx())).with_w(self[e5] * other[e4] * -1.0),
            // e15, e25, e35, e1234
            (self.group1().xyzx() * other.group2().www().with_w(other[e423])) + Simd32x3::from(0.0).with_w((self[e2] * other[e431]) + (self[e3] * other[e412]))
                - (other.group3() * self.group1().www().with_w(self[e321])),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w((self[e315] * other[e2]) + (self[e125] * other[e3]) + (self[e321] * other[e5]) - (self[e5] * other[e321]))
                + (self.group1().zxy() * other.group1().yzx()).with_w(self[e235] * other[e1])
                - (self.group1().yzxy() * other.group1().zxy().with_w(other[e315]))
                - (self.group1().wwwz() * other.group0().xyz().with_w(other[e125]))
                - (self.group0().xyz() * other.group3().www()).with_w(self[e1] * other[e235]),
        )
    }
}
impl Wedge<VersorOdd> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        0
    //    simd3        3        5        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       13       18        0
    //  no simd       31       40        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group1().zxyx() * other.group0().yzx().with_w(other[e4235]))
                + Simd32x3::from(0.0).with_w(
                    (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e5] * other[e1234])
                        - (self[e315] * other[e42])
                        - (self[e125] * other[e43])
                        - (self[e321] * other[e45]),
                )
                - (other.group0().zxyx() * self.group1().yzx().with_w(self[e235])),
            // e415, e425, e435, e321
            (other.group0() * self.group1().www().with_w(self[e321])) + Simd32x3::from(0.0).with_w(-(self[e2] * other[e31]) - (self[e3] * other[e12]))
                - (self.group1().xyzx() * other.group1().wwwx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[e5]) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * self.group0().xyz()) + (self.group1().yzx() * other.group2().zxy())
                - (self.group1().zxy() * other.group2().yzx()))
            .with_w(self[e5] * other[scalar]),
            // e1, e2, e3, e4
            (self.group1().xyz() * other.group0().www()).with_w(0.0),
        )
    }
}
impl std::ops::Div<WedgeInfix> for AntiLine {
    type Output = WedgeInfixPartial<AntiLine>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       24        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            (self.group0() * other.group2().www()).with_w(0.0),
            // e15, e25, e35, e1234
            (Simd32x3::from(other[scalar]) * self.group1()).with_w(-(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e42] * self[e35]) + (other[e45] * self[e23]),
                (other[e43] * self[e15]) + (other[e45] * self[e31]),
                (other[e41] * self[e25]) + (other[e45] * self[e12]),
                -(other[e23] * self[e15]) - (other[e31] * self[e25]) - (other[e12] * self[e35]) - (other[e25] * self[e31]) - (other[e35] * self[e12]),
            ]) - (other.group0().zxy() * self.group1().yzx()).with_w(other[e15] * self[e23]),
        )
    }
}
impl Wedge<AntiDipoleInversion> for AntiLine {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       24        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e4]) * self.group0(),
            // e415, e425, e435, e321
            (Simd32x3::from(other[e4]) * self.group1()).with_w(-(other[e1] * self[e23]) - (other[e2] * self[e31]) - (other[e3] * self[e12])),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other[e2] * self[e35]) + (other[e5] * self[e23]),
                (other[e3] * self[e15]) + (other[e5] * self[e31]),
                (other[e1] * self[e25]) + (other[e5] * self[e12]),
                -(other[e431] * self[e25]) - (other[e412] * self[e35]) - (other[e415] * self[e23]) - (other[e425] * self[e31]) - (other[e435] * self[e12]),
            ]) - (self.group1().yzx() * other.group3().zxy()).with_w(other[e423] * self[e15]),
        )
    }
}
impl Wedge<AntiDualNum> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(other[scalar]) * self.group1(),
        )
    }
}
impl Wedge<AntiFlector> for AntiLine {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([
                (other[e2] * self[e35]) + (other[e5] * self[e23]),
                (other[e3] * self[e15]) + (other[e5] * self[e31]),
                (other[e1] * self[e25]) + (other[e5] * self[e12]),
                -(other[e2] * self[e31]) - (other[e3] * self[e12]),
            ]) - (other.group1().zxyx() * self.group1().yzx().with_w(self[e23])),
        )
    }
}
impl Wedge<AntiLine> for AntiLine {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            -(other[e23] * self[e15]) - (other[e31] * self[e25]) - (other[e12] * self[e35]) - (other[e15] * self[e23]) - (other[e25] * self[e31]) - (other[e35] * self[e12]),
            0.0,
        ]))
    }
}
impl Wedge<AntiMotor> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (self.group0() * other.group0().www()).with_w(0.0),
            // e15, e25, e35, e3215
            (Simd32x3::from(other[scalar]) * self.group1()).with_w(
                -(self[e23] * other[e15]) - (self[e31] * other[e25]) - (self[e12] * other[e35]) - (self[e15] * other[e23]) - (self[e25] * other[e31]) - (self[e35] * other[e12]),
            ),
        )
    }
}
impl Wedge<AntiPlane> for AntiLine {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([
                (self[e23] * other[e5]) + (self[e35] * other[e2]),
                (self[e31] * other[e5]) + (self[e15] * other[e3]),
                (self[e12] * other[e5]) + (self[e25] * other[e1]),
                -(self[e31] * other[e2]) - (self[e12] * other[e3]),
            ]) - (other.group0().zxyx() * self.group1().yzx().with_w(self[e23])),
        )
    }
}
impl Wedge<Circle> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            -(self[e23] * other[e415]) - (self[e31] * other[e425]) - (self[e12] * other[e435]) - (self[e15] * other[e423]) - (self[e25] * other[e431]) - (self[e35] * other[e412]),
        )
    }
}
impl Wedge<CircleRotor> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            -(self[e23] * other[e415]) - (self[e31] * other[e425]) - (self[e12] * other[e435]) - (self[e15] * other[e423]) - (self[e25] * other[e431]) - (self[e35] * other[e412]),
        )
    }
}
impl Wedge<Dipole> for AntiLine {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       13       18        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e23] * other[e45]) + (self[e35] * other[e42]),
                (self[e31] * other[e45]) + (self[e15] * other[e43]),
                (self[e12] * other[e45]) + (self[e25] * other[e41]),
                -(self[e31] * other[e25]) - (self[e12] * other[e35]) - (self[e15] * other[e23]) - (self[e25] * other[e31]) - (self[e35] * other[e12]),
            ]) - (self.group1().yzx() * other.group0().zxy()).with_w(self[e23] * other[e15]),
            // e1234
            -(self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]),
        )
    }
}
impl Wedge<DipoleInversion> for AntiLine {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       13       18        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e23] * other[e45]) + (self[e35] * other[e42]),
                (self[e31] * other[e45]) + (self[e15] * other[e43]),
                (self[e12] * other[e45]) + (self[e25] * other[e41]),
                -(self[e31] * other[e25]) - (self[e12] * other[e35]) - (self[e15] * other[e23]) - (self[e25] * other[e31]) - (self[e35] * other[e12]),
            ]) - (self.group1().yzx() * other.group0().zxy()).with_w(self[e23] * other[e15]),
            // e1234
            -(self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]),
        )
    }
}
impl Wedge<DualNum> for AntiLine {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ (self.group0() * other.group0().xx().with_z(other[e5])).with_w(0.0))
    }
}
impl Wedge<FlatPoint> for AntiLine {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(other[e45]) * self.group0()).with_w(-(self[e23] * other[e15]) - (self[e31] * other[e25]) - (self[e12] * other[e35])),
        )
    }
}
impl Wedge<Flector> for AntiLine {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(other[e45]) * self.group0()).with_w(-(self[e23] * other[e15]) - (self[e31] * other[e25]) - (self[e12] * other[e35])),
        )
    }
}
impl Wedge<Line> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ -(self[e23] * other[e415]) - (self[e31] * other[e425]) - (self[e12] * other[e435]))
    }
}
impl Wedge<Motor> for AntiLine {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).with_w(-(self[e23] * other[e415]) - (self[e31] * other[e425]) - (self[e12] * other[e435])),
            // e235, e315, e125, e5
            (self.group0() * other.group1().www()).with_w(0.0),
        )
    }
}
impl Wedge<MultiVector> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd3        2        8        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       19       32        0
    //  no simd       26       48        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                -(self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435])
                    - (self[e15] * other[e423])
                    - (self[e25] * other[e431])
                    - (self[e35] * other[e412]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            (self.group1() * other.group0().xx().with_z(other[scalar])).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group0(),
            // e415, e425, e435, e321
            (Simd32x3::from(other[e4]) * self.group1()).with_w(-(self[e23] * other[e1]) - (self[e31] * other[e2]) - (self[e12] * other[e3])),
            // e423, e431, e412
            Simd32x3::from(other[e4]) * self.group0(),
            // e235, e315, e125
            (Simd32x3::from(other[e5]) * self.group0()) + (self.group1().zxy() * other.group1().yzx()) - (self.group1().yzx() * other.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e23] * other[e45]) + (self[e35] * other[e42]),
                (self[e31] * other[e45]) + (self[e15] * other[e43]),
                (self[e12] * other[e45]) + (self[e25] * other[e41]),
                -(self[e31] * other[e25]) - (self[e12] * other[e35]) - (self[e15] * other[e23]) - (self[e25] * other[e31]) - (self[e35] * other[e12]),
            ]) - (self.group1().yzx() * other.group4().zxy()).with_w(self[e23] * other[e15]),
            // e1234
            -(self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]),
        )
    }
}
impl Wedge<RoundPoint> for AntiLine {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        2        5        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        8       18        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e4]) * self.group0(),
            // e415, e425, e435, e321
            (Simd32x3::from(other[e4]) * self.group1()).with_w(-(self[e23] * other[e1]) - (self[e31] * other[e2]) - (self[e12] * other[e3])),
            // e235, e315, e125
            (Simd32x3::from(other[e5]) * self.group0()) + (self.group1().zxy() * other.group0().yzx()) - (self.group1().yzx() * other.group0().zxy()),
        )
    }
}
impl Wedge<Scalar> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(other[scalar]) * self.group1(),
        )
    }
}
impl Wedge<VersorEven> for AntiLine {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       24        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e4]) * self.group0(),
            // e415, e425, e435, e321
            (Simd32x3::from(other[e4]) * self.group1()).with_w(-(self[e23] * other[e1]) - (self[e31] * other[e2]) - (self[e12] * other[e3])),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (self[e23] * other[e5]) + (self[e35] * other[e2]),
                (self[e31] * other[e5]) + (self[e15] * other[e3]),
                (self[e12] * other[e5]) + (self[e25] * other[e1]),
                -(self[e31] * other[e425]) - (self[e12] * other[e435]) - (self[e15] * other[e423]) - (self[e25] * other[e431]) - (self[e35] * other[e412]),
            ]) - (self.group1().yzx() * other.group3().zxy()).with_w(self[e23] * other[e415]),
        )
    }
}
impl Wedge<VersorOdd> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       24        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            (self.group0() * other.group0().www()).with_w(0.0),
            // e15, e25, e35, e1234
            (Simd32x3::from(other[scalar]) * self.group1()).with_w(-(self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e23] * other[e45]) + (self[e35] * other[e42]),
                (self[e31] * other[e45]) + (self[e15] * other[e43]),
                (self[e12] * other[e45]) + (self[e25] * other[e41]),
                -(self[e31] * other[e25]) - (self[e12] * other[e35]) - (self[e15] * other[e23]) - (self[e25] * other[e31]) - (self[e35] * other[e12]),
            ]) - (self.group1().yzx() * other.group0().zxy()).with_w(self[e23] * other[e15]),
        )
    }
}
impl std::ops::Div<WedgeInfix> for AntiMotor {
    type Output = WedgeInfixPartial<AntiMotor>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       18        0
    //    simd3        1        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       12       23        0
    //  no simd       20       36        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(self[scalar]) * other.group0().with_w(other[scalar]),
            // e23, e31, e12, e45
            ((Simd32x3::from(other[scalar]) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * other.group1().xyz())).with_w(other[e45] * self[scalar]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other[e15] * self[scalar]) + (other[scalar] * self[e15]),
                (other[e25] * self[scalar]) + (other[scalar] * self[e25]),
                (other[e35] * self[scalar]) + (other[scalar] * self[e35]),
                -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other[e45] * self[e23],
                other[e45] * self[e31],
                other[e45] * self[e12],
                -(other[e31] * self[e25]) - (other[e12] * self[e35]) - (other[e15] * self[e23]) - (other[e25] * self[e31]) - (other[e35] * self[e12]),
            ]) + (self.group1().zxyw() * other.group0().yzx().with_w(other[scalar]))
                - (self.group1().yzxx() * other.group0().zxy().with_w(other[e23])),
        )
    }
}
impl Wedge<AntiDipoleInversion> for AntiMotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       17        0
    //    simd3        3        5        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       24       40        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                other[e4] * self[e23],
                other[e4] * self[e31],
                other[e4] * self[e12],
                -(other[e423] * self[e15])
                    - (other[e431] * self[e25])
                    - (other[e412] * self[e35])
                    - (other[e415] * self[e23])
                    - (other[e425] * self[e31])
                    - (other[e435] * self[e12]),
            ]) + (other.group0() * self.group0().www()).with_w(other[e4] * self[e3215]),
            // e415, e425, e435, e321
            Simd32x4::from([
                other[e4] * self[e15],
                other[e4] * self[e25],
                other[e4] * self[e35],
                -(other[e1] * self[e23]) - (other[e2] * self[e31]) - (other[e3] * self[e12]),
            ]) + (Simd32x4::from(self[scalar]) * other.group1()),
            // e235, e315, e125, e5
            ((Simd32x3::from(other[e5]) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * other.group2().xyz()) + (other.group3().yzx() * self.group1().zxy())
                - (other.group3().zxy() * self.group1().yzx()))
            .with_w(other[e5] * self[scalar]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group3().xyz().with_w(other[e4]),
        )
    }
}
impl Wedge<AntiDualNum> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1       10        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * self.group0(),
            // e15, e25, e35, e3215
            other.group0().yy().with_zw(other[scalar], (other[e3215] * self[scalar]) + (other[scalar] * self[e3215])) * self.group1().xyz().with_w(1.0),
        )
    }
}
impl Wedge<AntiFlatPoint> for AntiMotor {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(self[scalar]) * other.group0())
    }
}
impl Wedge<AntiFlector> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       20        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([
                (other[e2] * self[e35]) + (other[e5] * self[e23]),
                (other[e3] * self[e15]) + (other[e5] * self[e31]),
                (other[e1] * self[e25]) + (other[e5] * self[e12]),
                -(other[e2] * self[e31]) - (other[e3] * self[e12]),
            ]) + (Simd32x4::from(self[scalar]) * other.group0())
                - (other.group1().zxyx() * self.group1().yzx().with_w(self[e23])),
            // e1, e2, e3, e5
            Simd32x4::from(self[scalar]) * other.group1(),
        )
    }
}
impl Wedge<AntiLine> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (other.group0() * self.group0().www()).with_w(0.0),
            // e15, e25, e35, e3215
            (Simd32x3::from(self[scalar]) * other.group1()).with_w(
                -(other[e23] * self[e15]) - (other[e31] * self[e25]) - (other[e12] * self[e35]) - (other[e15] * self[e23]) - (other[e25] * self[e31]) - (other[e35] * self[e12]),
            ),
        )
    }
}
impl Wedge<AntiMotor> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        8       11        0
    //  no simd       16       21        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            ((Simd32x3::from(other[scalar]) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * other.group0().xyz())).with_w(other[scalar] * self[scalar]),
            // e15, e25, e35, e3215
            (Simd32x4::from(other[scalar]) * self.group1())
                + (Simd32x4::from(self[scalar]) * other.group1())
                + Simd32x3::from(0.0).with_w(
                    -(other[e23] * self[e15])
                        - (other[e31] * self[e25])
                        - (other[e12] * self[e35])
                        - (other[e15] * self[e23])
                        - (other[e25] * self[e31])
                        - (other[e35] * self[e12]),
                ),
        )
    }
}
impl Wedge<AntiPlane> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([
                (self[e23] * other[e5]) + (self[e35] * other[e2]),
                (self[e31] * other[e5]) + (self[e15] * other[e3]),
                (self[e12] * other[e5]) + (self[e25] * other[e1]),
                -(self[e31] * other[e2]) - (self[e12] * other[e3]),
            ]) - (other.group0().zxyx() * self.group1().yzx().with_w(self[e23])),
            // e1, e2, e3, e5
            Simd32x4::from(self[scalar]) * other.group0(),
        )
    }
}
impl Wedge<AntiScalar> for AntiMotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[scalar] * other[e12345])
    }
}
impl Wedge<Circle> for AntiMotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       16        0
    fn wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[scalar]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1(),
            // e235, e315, e125, e12345
            (Simd32x3::from(self[scalar]) * other.group2()).with_w(
                -(self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435])
                    - (self[e15] * other[e423])
                    - (self[e25] * other[e431])
                    - (self[e35] * other[e412]),
            ),
        )
    }
}
impl Wedge<CircleRotor> for AntiMotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       10        0
    //  no simd        6       17        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[scalar]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1(),
            // e235, e315, e125, e12345
            (other.group2().xyz() * self.group0().www()).with_w(
                (self[scalar] * other[e12345])
                    - (self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435])
                    - (self[e15] * other[e423])
                    - (self[e25] * other[e431])
                    - (self[e35] * other[e412]),
            ),
        )
    }
}
impl Wedge<Dipole> for AntiMotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       13       28        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1(),
            // e15, e25, e35, e1234
            (Simd32x3::from(self[scalar]) * other.group2()).with_w(-(self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e23] * other[e45]) + (self[e35] * other[e42]),
                (self[e31] * other[e45]) + (self[e15] * other[e43]),
                (self[e12] * other[e45]) + (self[e25] * other[e41]),
                -(self[e31] * other[e25]) - (self[e12] * other[e35]) - (self[e15] * other[e23]) - (self[e25] * other[e31]) - (self[e35] * other[e12]),
            ]) - (other.group0().zxy() * self.group1().yzx()).with_w(self[e23] * other[e15]),
        )
    }
}
impl Wedge<DipoleInversion> for AntiMotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       17        0
    //    simd3        0        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       12       22        0
    //  no simd       18       33        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1(),
            // e15, e25, e35, e1234
            (other.group2().xyz() * self.group0().www()).with_w((self[scalar] * other[e1234]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e23] * other[e45]) + (self[scalar] * other[e4235]),
                (self[e31] * other[e45]) + (self[scalar] * other[e4315]),
                (self[e12] * other[e45]) + (self[scalar] * other[e4125]),
                -(self[e31] * other[e25]) - (self[e12] * other[e35]) - (self[e15] * other[e23]) - (self[e25] * other[e31]) - (self[e35] * other[e12]),
            ]) + (other.group0().yzx() * self.group1().zxy()).with_w(self[scalar] * other[e3215])
                - (other.group0().zxy() * self.group1().yzx()).with_w(self[e23] * other[e15]),
        )
    }
}
impl Wedge<DualNum> for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).with_w(self[scalar] * other[e12345]),
            // e235, e315, e125, e5
            Simd32x4::from(other[e5]) * self.group0(),
        )
    }
}
impl Wedge<FlatPoint> for AntiMotor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2       10        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * other.group0(),
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(other[e45]) * self.group0().xyz()).with_w(-(self[e23] * other[e15]) - (self[e31] * other[e25]) - (self[e12] * other[e35])),
        )
    }
}
impl Wedge<Flector> for AntiMotor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        6       14        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * other.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[scalar] * other[e4235],
                self[scalar] * other[e4315],
                self[scalar] * other[e4125],
                -(self[e23] * other[e15]) - (self[e31] * other[e25]) - (self[e12] * other[e35]),
            ]) + (self.group0() * other.group0().www().with_w(other[e3215])),
        )
    }
}
impl Wedge<Line> for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x3::from(self[scalar]) * other.group0()).with_w(-(self[e23] * other[e415]) - (self[e31] * other[e425]) - (self[e12] * other[e435])),
            // e235, e315, e125, e5
            (other.group1() * self.group0().www()).with_w(0.0),
        )
    }
}
impl Wedge<Motor> for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        1        3        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        6       14        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (other.group0().xyz() * self.group0().www()).with_w((self[scalar] * other[e12345]) - (self[e23] * other[e415]) - (self[e31] * other[e425]) - (self[e12] * other[e435])),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[scalar]) * other.group1().xyz()) + (Simd32x3::from(other[e5]) * self.group0().xyz())).with_w(self[scalar] * other[e5]),
        )
    }
}
impl Wedge<MultiVector> for AntiMotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       29        0
    //    simd3        6       11        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       26       45        0
    //  no simd       50       82        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[scalar] * other[scalar],
                (self[scalar] * other[e12345]) + (self[e3215] * other[e4])
                    - (self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435])
                    - (self[e15] * other[e423])
                    - (self[e25] * other[e431])
                    - (self[e35] * other[e412]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group1(),
            // e5
            self[scalar] * other[e5],
            // e15, e25, e35, e45
            ((Simd32x3::from(self[scalar]) * other.group3().xyz()) + (Simd32x3::from(other[scalar]) * self.group1().xyz())).with_w(self[scalar] * other[e45]),
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group4(),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * other.group5()) + (Simd32x3::from(other[scalar]) * self.group0().xyz()),
            // e415, e425, e435, e321
            Simd32x4::from([
                self[e15] * other[e4],
                self[e25] * other[e4],
                self[e35] * other[e4],
                -(self[e23] * other[e1]) - (self[e31] * other[e2]) - (self[e12] * other[e3]),
            ]) + (Simd32x4::from(self[scalar]) * other.group6()),
            // e423, e431, e412
            (Simd32x3::from(self[scalar]) * other.group7()) + (Simd32x3::from(other[e4]) * self.group0().xyz()),
            // e235, e315, e125
            (Simd32x3::from(self[scalar]) * other.group8()) + (Simd32x3::from(other[e5]) * self.group0().xyz()) + (self.group1().zxy() * other.group1().yzx())
                - (self.group1().yzx() * other.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[scalar] * other[e4235],
                self[scalar] * other[e4315],
                self[scalar] * other[e4125],
                -(self[e23] * other[e15]) - (self[e31] * other[e25]) - (self[e12] * other[e35]) - (self[e25] * other[e31]) - (self[e35] * other[e12]),
            ]) + (self.group0() * other.group3().www().with_w(other[e3215]))
                + (self.group1().zxyw() * other.group4().yzx().with_w(other[scalar]))
                - (self.group1().yzxx() * other.group4().zxy().with_w(other[e23])),
            // e1234
            (self[scalar] * other[e1234]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]),
        )
    }
}
impl Wedge<Plane> for AntiMotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self[scalar]) * other.group0())
    }
}
impl Wedge<RoundPoint> for AntiMotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        2        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        8       24        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[e3215]),
            // e415, e425, e435, e321
            (Simd32x3::from(other[e4]) * self.group1().xyz()).with_w(-(self[e23] * other[e1]) - (self[e31] * other[e2]) - (self[e12] * other[e3])),
            // e235, e315, e125, e5
            ((Simd32x3::from(other[e5]) * self.group0().xyz()) + (self.group1().zxy() * other.group0().yzx()) - (self.group1().yzx() * other.group0().zxy()))
                .with_w(self[scalar] * other[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group0(),
        )
    }
}
impl Wedge<Scalar> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * self.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl Wedge<Sphere> for AntiMotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group0(),
            // e1234
            self[scalar] * other[e1234],
        )
    }
}
impl Wedge<VersorEven> for AntiMotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd3        3        5        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       28       41        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0() * other.group3().www().with_w(other[e12345]))
                + Simd32x3::from(0.0).with_w(
                    -(self[e23] * other[e415])
                        - (self[e31] * other[e425])
                        - (self[e12] * other[e435])
                        - (self[e15] * other[e423])
                        - (self[e25] * other[e431])
                        - (self[e35] * other[e412]),
                )
                + (other.group0().xyz() * self.group0().www()).with_w(self[e3215] * other[e4]),
            // e415, e425, e435, e321
            Simd32x4::from([
                self[e15] * other[e4],
                self[e25] * other[e4],
                self[e35] * other[e4],
                -(self[e23] * other[e1]) - (self[e31] * other[e2]) - (self[e12] * other[e3]),
            ]) + (Simd32x4::from(self[scalar]) * other.group1()),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[scalar]) * other.group2().xyz()) + (Simd32x3::from(other[e5]) * self.group0().xyz()) + (self.group1().zxy() * other.group3().yzx())
                - (self.group1().yzx() * other.group3().zxy()))
            .with_w(self[scalar] * other[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group3(),
        )
    }
}
impl Wedge<VersorOdd> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        1        4        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       11       24        0
    //  no simd       25       41        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            ((Simd32x3::from(self[scalar]) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * self.group0().xyz())).with_w(self[scalar] * other[e45]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self[e15] * other[scalar],
                self[e25] * other[scalar],
                self[e35] * other[scalar],
                -(self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]),
            ]) + (Simd32x4::from(self[scalar]) * other.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e35] * other[e42],
                self[e15] * other[e43],
                self[e25] * other[e41],
                -(self[e31] * other[e25]) - (self[e12] * other[e35]) - (self[e15] * other[e23]) - (self[e25] * other[e31]) - (self[e35] * other[e12]),
            ]) + (self.group0() * other.group1().www().with_w(other[e3215]))
                + (other.group3().xyz() * self.group0().www()).with_w(self[e3215] * other[scalar])
                - (self.group1().yzx() * other.group0().zxy()).with_w(self[e23] * other[e15]),
        )
    }
}
impl std::ops::Div<WedgeInfix> for AntiPlane {
    type Output = WedgeInfixPartial<AntiPlane>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for AntiPlane {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        1        5        0
    //    simd4        3        2        0
    // Totals...
    // yes simd        5       12        0
    //  no simd       16       28        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([other[e41] * self[e5], other[e42] * self[e5], other[e43] * self[e5], -(other[e31] * self[e2]) - (other[e12] * self[e3])])
                - (other.group1().wwwx() * self.group0().xyzx()),
            // e235, e315, e125, e4
            (Simd32x3::from(self[e5]) * other.group1().xyz()).with_w(0.0) + (other.group2().zxy() * self.group0().yzx()).with_w(0.0)
                - (other.group2().yzx() * self.group0().zxy()).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(other[scalar]) * self.group0(),
        )
    }
}
impl Wedge<AntiDipoleInversion> for AntiPlane {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       15        0
    //    simd3        1        4        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        6       22        0
    //  no simd       17       39        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e4]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            ((other.group3().zxy() * self.group0().yzx()) - (other.group3().yzx() * self.group0().zxy())).with_w(other[e4] * self[e5] * -1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                other[e1] * self[e5] * -1.0,
                other[e2] * self[e5] * -1.0,
                other[e3] * self[e5] * -1.0,
                (other[e431] * self[e2]) + (other[e412] * self[e3]),
            ]) + (self.group0().xyzx() * other.group3().www().with_w(other[e423])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other[e425] * self[e3],
                other[e435] * self[e1],
                other[e415] * self[e2],
                -(other[e315] * self[e2]) - (other[e125] * self[e3]),
            ]) - (Simd32x4::from(self[e5]) * other.group0().with_w(other[e321]))
                - (self.group0().yzxx() * other.group1().zxy().with_w(other[e235])),
        )
    }
}
impl Wedge<AntiDualNum> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl Wedge<AntiFlatPoint> for AntiPlane {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            -(other[e235] * self[e1]) - (other[e315] * self[e2]) - (other[e125] * self[e3]) - (other[e321] * self[e5]),
            0.0,
        ]))
    }
}
impl Wedge<AntiFlector> for AntiPlane {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        9       16        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            ((other.group1().zxy() * self.group0().yzx()) - (other.group1().yzx() * self.group0().zxy())).with_w(0.0),
            // e15, e25, e35, e3215
            Simd32x4::from([
                other[e5] * self[e1],
                other[e5] * self[e2],
                other[e5] * self[e3],
                -(other[e315] * self[e2]) - (other[e125] * self[e3]) - (other[e321] * self[e5]),
            ]) - (self.group0().wwwx() * other.group1().xyz().with_w(other[e235])),
        )
    }
}
impl Wedge<AntiLine> for AntiPlane {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([
                (other[e23] * self[e5]) + (other[e35] * self[e2]),
                (other[e31] * self[e5]) + (other[e15] * self[e3]),
                (other[e12] * self[e5]) + (other[e25] * self[e1]),
                -(other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]) - (self.group0().zxyx() * other.group1().yzx().with_w(other[e23])),
        )
    }
}
impl Wedge<AntiMotor> for AntiPlane {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([
                (other[e23] * self[e5]) + (other[e35] * self[e2]),
                (other[e31] * self[e5]) + (other[e15] * self[e3]),
                (other[e12] * self[e5]) + (other[e25] * self[e1]),
                -(other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]) - (self.group0().zxyx() * other.group1().yzx().with_w(other[e23])),
            // e1, e2, e3, e5
            Simd32x4::from(other[scalar]) * self.group0(),
        )
    }
}
impl Wedge<AntiPlane> for AntiPlane {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        4        0
    // no simd        6       12        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            (other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy()),
            // e15, e25, e35
            (Simd32x3::from(other[e5]) * self.group0().xyz()) - (Simd32x3::from(self[e5]) * other.group0().xyz()),
        )
    }
}
impl Wedge<Circle> for AntiPlane {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd       11       16        0
    fn wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e3] * other[e425],
                self[e1] * other[e435],
                self[e2] * other[e415],
                -(self[e3] * other[e125]) - (self[e5] * other[e321]),
            ]) - (self.group0().wwwx() * other.group0().with_w(other[e235]))
                - (self.group0().yzx() * other.group1().zxy()).with_w(self[e2] * other[e315]),
            // e1234
            (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]),
        )
    }
}
impl Wedge<CircleRotor> for AntiPlane {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd       11       16        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e3] * other[e425],
                self[e1] * other[e435],
                self[e2] * other[e415],
                -(self[e3] * other[e125]) - (self[e5] * other[e321]),
            ]) - (self.group0().yzxy() * other.group1().zxy().with_w(other[e315]))
                - (other.group0() * self.group0().www()).with_w(self[e1] * other[e235]),
            // e1234
            (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]),
        )
    }
}
impl Wedge<Dipole> for AntiPlane {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        3        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd       14       24        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([self[e5] * other[e41], self[e5] * other[e42], self[e5] * other[e43], -(self[e2] * other[e31]) - (self[e3] * other[e12])])
                - (self.group0().xyzx() * other.group1().wwwx()),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * other.group1().xyz()) + (other.group2().zxy() * self.group0().yzx()) - (other.group2().yzx() * self.group0().zxy()),
        )
    }
}
impl Wedge<DipoleInversion> for AntiPlane {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       13        0
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        6       18        0
    //  no simd       17       31        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([self[e5] * other[e41], self[e5] * other[e42], self[e5] * other[e43], -(self[e2] * other[e31]) - (self[e3] * other[e12])])
                - (self.group0().xyzx() * other.group1().wwwx()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                self[e3] * other[e25] * -1.0,
                self[e1] * other[e35] * -1.0,
                self[e2] * other[e15] * -1.0,
                (self[e3] * other[e4125]) + (self[e5] * other[e1234]),
            ]) + (self.group0().yzxx() * other.group2().zxy().with_w(other[e4235]))
                + (self.group0().wwwy() * other.group1().xyz().with_w(other[e4315])),
        )
    }
}
impl Wedge<DualNum> for AntiPlane {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ (self.group0().xyz() * other.group0().xx().with_z(other[e5])).with_w(0.0))
    }
}
impl Wedge<FlatPoint> for AntiPlane {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        4        0
    // no simd        3       12        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(other[e45]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            (self.group0().yzx() * other.group0().zxy()) - (self.group0().zxy() * other.group0().yzx()),
        )
    }
}
impl Wedge<Flector> for AntiPlane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        1        4        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        5       15        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x3::from(other[e45]) * self.group0().xyz() * Simd32x3::from(-1.0)).with_w((self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125])),
            // e235, e315, e125, e5
            ((self.group0().yzx() * other.group0().zxy()) - (self.group0().zxy() * other.group0().yzx())).with_w(0.0),
        )
    }
}
impl Wedge<Line> for AntiPlane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        5        9        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e3] * other[e425],
                self[e1] * other[e435],
                self[e2] * other[e415],
                -(self[e2] * other[e315]) - (self[e3] * other[e125]),
            ]) - (self.group0().yzxx() * other.group0().zxy().with_w(other[e235])),
        )
    }
}
impl Wedge<Motor> for AntiPlane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        5       12        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            (self.group0().xyz() * other.group1().www()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e3] * other[e425],
                self[e1] * other[e435],
                self[e2] * other[e415],
                -(self[e2] * other[e315]) - (self[e3] * other[e125]),
            ]) - (self.group0().yzxx() * other.group0().zxy().with_w(other[e235])),
        )
    }
}
impl Wedge<MultiVector> for AntiPlane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       22        0
    //    simd3        5       14        0
    //    simd4        3        1        0
    // Totals...
    // yes simd       15       37        0
    //  no simd       34       68        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e5] * other[e1234])]),
            // e1, e2, e3, e4
            (self.group0().xyz() * other.group0().xx().with_z(other[scalar])).with_w(0.0),
            // e5
            self[e5] * other[scalar],
            // e15, e25, e35, e45
            ((Simd32x3::from(other[e5]) * self.group0().xyz()) - (Simd32x3::from(self[e5]) * other.group1().xyz())).with_w(self[e5] * other[e4] * -1.0),
            // e41, e42, e43
            Simd32x3::from(other[e4]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            (self.group0().yzx() * other.group1().zxy()) - (self.group0().zxy() * other.group1().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([self[e5] * other[e41], self[e5] * other[e42], self[e5] * other[e43], -(self[e2] * other[e31]) - (self[e3] * other[e12])])
                - (self.group0().xyz() * other.group3().www()).with_w(self[e1] * other[e23]),
            // e423, e431, e412
            (other.group4().yzx() * self.group0().zxy()) - (other.group4().zxy() * self.group0().yzx()),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * other.group5()) + (self.group0().yzx() * other.group3().zxy()) - (self.group0().zxy() * other.group3().yzx()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e3] * other[e425],
                self[e1] * other[e435],
                self[e2] * other[e415],
                -(self[e3] * other[e125]) - (self[e5] * other[e321]),
            ]) - (self.group0().wwwx() * other.group7().with_w(other[e235]))
                - (self.group0().yzx() * other.group6().zxy()).with_w(self[e2] * other[e315]),
            // e1234
            (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]),
        )
    }
}
impl Wedge<Plane> for AntiPlane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]))
    }
}
impl Wedge<RoundPoint> for AntiPlane {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        2        6        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        6       20        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e4]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            ((self.group0().yzx() * other.group0().zxy()) - (self.group0().zxy() * other.group0().yzx())).with_w(self[e5] * other[e4] * -1.0),
            // e15, e25, e35
            (Simd32x3::from(other[e5]) * self.group0().xyz()) - (Simd32x3::from(self[e5]) * other.group0().xyz()),
        )
    }
}
impl Wedge<Scalar> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl Wedge<Sphere> for AntiPlane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e5] * other[e1234]),
        )
    }
}
impl Wedge<VersorEven> for AntiPlane {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       15        0
    //    simd3        1        4        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        6       22        0
    //  no simd       17       39        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e4]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            ((self.group0().yzx() * other.group3().zxy()) - (self.group0().zxy() * other.group3().yzx())).with_w(self[e5] * other[e4] * -1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self[e5] * other[e1] * -1.0,
                self[e5] * other[e2] * -1.0,
                self[e5] * other[e3] * -1.0,
                (self[e2] * other[e431]) + (self[e3] * other[e412]),
            ]) + (self.group0().xyzx() * other.group2().www().with_w(other[e423])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e3] * other[e425],
                self[e1] * other[e435],
                self[e2] * other[e415],
                -(self[e3] * other[e125]) - (self[e5] * other[e321]),
            ]) - (self.group0().yzxx() * other.group1().zxy().with_w(other[e235]))
                - (self.group0().wwwy() * other.group0().xyz().with_w(other[e315])),
        )
    }
}
impl Wedge<VersorOdd> for AntiPlane {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       15        0
    //    simd3        2        4        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        7       21        0
    //  no simd       17       35        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                self[e2] * other[e43] * -1.0,
                self[e3] * other[e41] * -1.0,
                self[e1] * other[e42] * -1.0,
                (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e5] * other[e1234]),
            ]) + (self.group0().zxyx() * other.group0().yzx().with_w(other[e4235])),
            // e415, e425, e435, e321
            Simd32x4::from([self[e5] * other[e41], self[e5] * other[e42], self[e5] * other[e43], -(self[e2] * other[e31]) - (self[e3] * other[e12])])
                - (self.group0().xyzx() * other.group1().wwwx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[e5]) * other.group1().xyz()) + (self.group0().yzx() * other.group2().zxy()) - (self.group0().zxy() * other.group2().yzx()))
                .with_w(self[e5] * other[scalar]),
            // e1, e2, e3, e4
            (self.group0().xyz() * other.group0().www()).with_w(0.0),
        )
    }
}
impl std::ops::Div<WedgeInfix> for AntiScalar {
    type Output = WedgeInfixPartial<AntiScalar>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[scalar] * self[e12345])
    }
}
impl Wedge<AntiDualNum> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[scalar] * self[e12345])
    }
}
impl Wedge<AntiMotor> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[scalar] * self[e12345])
    }
}
impl Wedge<MultiVector> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e12345] * other[scalar])
    }
}
impl Wedge<Scalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e12345] * other[scalar])
    }
}
impl Wedge<VersorOdd> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e12345] * other[scalar])
    }
}
impl std::ops::Div<WedgeInfix> for Circle {
    type Output = WedgeInfixPartial<Circle>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for Circle {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd        9       20        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125, e12345
            (Simd32x3::from(other[scalar]) * self.group2()).with_w(
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
impl Wedge<AntiDipoleInversion> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       20        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(other[e4] * self[e235]) - (other[e3] * self[e425]),
                -(other[e4] * self[e315]) - (other[e1] * self[e435]),
                -(other[e4] * self[e125]) - (other[e2] * self[e415]),
                (other[e3] * self[e125]) + (other[e5] * self[e321]),
            ]) + (other.group3().wwwx() * self.group0().with_w(self[e235]))
                + (other.group3().yzx() * self.group1().zxy()).with_w(other[e2] * self[e315]),
            // e1234
            -(other[e4] * self[e321]) - (other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]),
        )
    }
}
impl Wedge<AntiDualNum> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
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
impl Wedge<AntiFlector> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       12        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        5       14        0
    //  no simd       11       19        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other[e3] * self[e425] * -1.0,
                other[e1] * self[e435] * -1.0,
                other[e2] * self[e415] * -1.0,
                (other[e3] * self[e125]) + (other[e5] * self[e321]),
            ]) + (other.group1().wwwx() * self.group0().with_w(self[e235]))
                + (other.group1().yzx() * self.group1().zxy()).with_w(other[e2] * self[e315]),
            // e1234
            -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]),
        )
    }
}
impl Wedge<AntiLine> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            -(other[e23] * self[e415]) - (other[e31] * self[e425]) - (other[e12] * self[e435]) - (other[e15] * self[e423]) - (other[e25] * self[e431]) - (other[e35] * self[e412]),
        )
    }
}
impl Wedge<AntiMotor> for Circle {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       16        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125, e12345
            (Simd32x3::from(other[scalar]) * self.group2()).with_w(
                -(other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435])
                    - (other[e15] * self[e423])
                    - (other[e25] * self[e431])
                    - (other[e35] * self[e412]),
            ),
        )
    }
}
impl Wedge<AntiPlane> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       12        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        5       14        0
    //  no simd       11       19        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other[e3] * self[e425] * -1.0,
                other[e1] * self[e435] * -1.0,
                other[e2] * self[e415] * -1.0,
                (other[e3] * self[e125]) + (other[e5] * self[e321]),
            ]) + (other.group0().wwwx() * self.group0().with_w(self[e235]))
                + (other.group0().yzx() * self.group1().zxy()).with_w(other[e2] * self[e315]),
            // e1234
            -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]),
        )
    }
}
impl Wedge<Dipole> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
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
impl Wedge<DipoleInversion> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
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
impl Wedge<DualNum> for Circle {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[e5]) * self.group0().with_w(self[e321]))
    }
}
impl Wedge<FlatPoint> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            -(self[e423] * other[e15]) - (self[e431] * other[e25]) - (self[e412] * other[e35]) - (self[e321] * other[e45]),
        )
    }
}
impl Wedge<Flector> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            -(self[e423] * other[e15]) - (self[e431] * other[e25]) - (self[e412] * other[e35]) - (self[e321] * other[e45]),
        )
    }
}
impl Wedge<Motor> for Circle {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[e5]) * self.group0().with_w(self[e321]))
    }
}
impl Wedge<MultiVector> for Circle {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       23        0
    //    simd3        0        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       18       28        0
    //  no simd       24       40        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
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
            Simd32x4::from(other[scalar]) * self.group1(),
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * other[e3]) - (self[e235] * other[e4]),
                -(self[e435] * other[e1]) - (self[e315] * other[e4]),
                -(self[e415] * other[e2]) - (self[e125] * other[e4]),
                (self[e321] * other[e5]) + (self[e125] * other[e3]),
            ]) + (other.group1().yzxy() * self.group1().zxy().with_w(self[e315]))
                + (Simd32x3::from(other[e5]) * self.group0()).with_w(self[e235] * other[e1]),
            // e1234
            -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
        )
    }
}
impl Wedge<RoundPoint> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       20        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * other[e3]) - (self[e235] * other[e4]),
                -(self[e435] * other[e1]) - (self[e315] * other[e4]),
                -(self[e415] * other[e2]) - (self[e125] * other[e4]),
                (self[e321] * other[e5]) + (self[e125] * other[e3]),
            ]) + (other.group0().yzxy() * self.group1().zxy().with_w(self[e315]))
                + (Simd32x3::from(other[e5]) * self.group0()).with_w(self[e235] * other[e1]),
            // e1234
            -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
        )
    }
}
impl Wedge<Scalar> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn wedge(self, other: Scalar) -> Self::Output {
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
impl Wedge<VersorEven> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       20        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * other[e3]) - (self[e235] * other[e4]),
                -(self[e435] * other[e1]) - (self[e315] * other[e4]),
                -(self[e415] * other[e2]) - (self[e125] * other[e4]),
                (self[e321] * other[e5]) + (self[e125] * other[e3]),
            ]) + (other.group3().yzxy() * self.group1().zxy().with_w(self[e315]))
                + (self.group0() * other.group2().www()).with_w(self[e235] * other[e1]),
            // e1234
            -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
        )
    }
}
impl Wedge<VersorOdd> for Circle {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd        9       20        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125, e12345
            (Simd32x3::from(other[scalar]) * self.group2()).with_w(
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
impl std::ops::Div<WedgeInfix> for CircleRotor {
    type Output = WedgeInfixPartial<CircleRotor>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       11        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       14        0
    //  no simd       10       21        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125, e12345
            (self.group2().xyz() * other.group2().www()).with_w(
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
impl Wedge<AntiDipoleInversion> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       20        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(other[e4] * self[e235]) - (other[e3] * self[e425]),
                -(other[e4] * self[e315]) - (other[e1] * self[e435]),
                -(other[e4] * self[e125]) - (other[e2] * self[e415]),
                (other[e3] * self[e125]) + (other[e5] * self[e321]),
            ]) + (other.group3().yzxy() * self.group1().zxy().with_w(self[e315]))
                + (self.group0() * other.group3().www()).with_w(other[e1] * self[e235]),
            // e1234
            -(other[e4] * self[e321]) - (other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]),
        )
    }
}
impl Wedge<AntiDualNum> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
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
impl Wedge<AntiFlector> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       12        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        5       14        0
    //  no simd       11       19        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other[e3] * self[e425] * -1.0,
                other[e1] * self[e435] * -1.0,
                other[e2] * self[e415] * -1.0,
                (other[e3] * self[e125]) + (other[e5] * self[e321]),
            ]) + (other.group1().yzxy() * self.group1().zxy().with_w(self[e315]))
                + (self.group0() * other.group1().www()).with_w(other[e1] * self[e235]),
            // e1234
            -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]),
        )
    }
}
impl Wedge<AntiLine> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            -(other[e23] * self[e415]) - (other[e31] * self[e425]) - (other[e12] * self[e435]) - (other[e15] * self[e423]) - (other[e25] * self[e431]) - (other[e35] * self[e412]),
        )
    }
}
impl Wedge<AntiMotor> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       10        0
    //  no simd        6       17        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125, e12345
            (self.group2().xyz() * other.group0().www()).with_w(
                (other[scalar] * self[e12345])
                    - (other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435])
                    - (other[e15] * self[e423])
                    - (other[e25] * self[e431])
                    - (other[e35] * self[e412]),
            ),
        )
    }
}
impl Wedge<AntiPlane> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       12        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        5       14        0
    //  no simd       11       19        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other[e3] * self[e425] * -1.0,
                other[e1] * self[e435] * -1.0,
                other[e2] * self[e415] * -1.0,
                (other[e3] * self[e125]) + (other[e5] * self[e321]),
            ]) + (other.group0().yzxy() * self.group1().zxy().with_w(self[e315]))
                + (self.group0() * other.group0().www()).with_w(other[e1] * self[e235]),
            // e1234
            -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]),
        )
    }
}
impl Wedge<Dipole> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
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
impl Wedge<DipoleInversion> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
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
impl Wedge<DualNum> for CircleRotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[e5]) * self.group0().with_w(self[e321]))
    }
}
impl Wedge<FlatPoint> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            -(self[e423] * other[e15]) - (self[e431] * other[e25]) - (self[e412] * other[e35]) - (self[e321] * other[e45]),
        )
    }
}
impl Wedge<Flector> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            -(self[e423] * other[e15]) - (self[e431] * other[e25]) - (self[e412] * other[e35]) - (self[e321] * other[e45]),
        )
    }
}
impl Wedge<Motor> for CircleRotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[e5]) * self.group0().with_w(self[e321]))
    }
}
impl Wedge<MultiVector> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       23        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       25       41        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
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
            Simd32x4::from(other[scalar]) * self.group1(),
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * other[e3]) - (self[e235] * other[e4]),
                -(self[e435] * other[e1]) - (self[e315] * other[e4]),
                -(self[e415] * other[e2]) - (self[e125] * other[e4]),
                (self[e315] * other[e2]) + (self[e125] * other[e3]),
            ]) + (Simd32x4::from(other[e5]) * self.group0().with_w(self[e321]))
                + (other.group1().yzxx() * self.group1().zxy().with_w(self[e235])),
            // e1234
            -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
        )
    }
}
impl Wedge<RoundPoint> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * other[e3]) - (self[e235] * other[e4]),
                -(self[e435] * other[e1]) - (self[e315] * other[e4]),
                -(self[e415] * other[e2]) - (self[e125] * other[e4]),
                (self[e315] * other[e2]) + (self[e125] * other[e3]),
            ]) + (Simd32x4::from(other[e5]) * self.group0().with_w(self[e321]))
                + (other.group0().yzxx() * self.group1().zxy().with_w(self[e235])),
            // e1234
            -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
        )
    }
}
impl Wedge<Scalar> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn wedge(self, other: Scalar) -> Self::Output {
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
impl Wedge<VersorEven> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * other[e3]) - (self[e235] * other[e4]),
                -(self[e435] * other[e1]) - (self[e315] * other[e4]),
                -(self[e415] * other[e2]) - (self[e125] * other[e4]),
                (self[e315] * other[e2]) + (self[e125] * other[e3]),
            ]) + (Simd32x4::from(other[e5]) * self.group0().with_w(self[e321]))
                + (other.group3().yzxx() * self.group1().zxy().with_w(self[e235])),
            // e1234
            -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
        )
    }
}
impl Wedge<VersorOdd> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       11        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       14        0
    //  no simd       10       21        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125, e12345
            (Simd32x3::from(other[scalar]) * self.group2().xyz()).with_w(
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
            ),
        )
    }
}
impl std::ops::Div<WedgeInfix> for Dipole {
    type Output = WedgeInfixPartial<Dipole>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for Dipole {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       25       40        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            (Simd32x3::from(other[scalar]) * self.group2()).with_w(
                -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e42] * self[e35]) + (other[e23] * self[e45]) + (other[e45] * self[e23]) + (other[e35] * self[e42]),
                (other[e43] * self[e15]) + (other[e31] * self[e45]) + (other[e45] * self[e31]) + (other[e15] * self[e43]),
                (other[e41] * self[e25]) + (other[e12] * self[e45]) + (other[e45] * self[e12]) + (other[e25] * self[e41]),
                -(other[e12] * self[e35]) - (other[e15] * self[e23]) - (other[e25] * self[e31]) - (other[e35] * self[e12]),
            ]) - (other.group0().zxy() * self.group2().yzx()).with_w(other[e23] * self[e15])
                - (self.group0().zxy() * other.group2().yzx()).with_w(other[e31] * self[e25]),
        )
    }
}
impl Wedge<AntiDipoleInversion> for Dipole {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       24        0
    //    simd3        2        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       29       40        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other[e4]) * self.group1().xyz()) + (self.group0().yzx() * other.group3().zxy()) - (self.group0().zxy() * other.group3().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other[e4] * self[e15]) + (other[e5] * self[e41]),
                (other[e4] * self[e25]) + (other[e5] * self[e42]),
                (other[e4] * self[e35]) + (other[e5] * self[e43]),
                -(other[e2] * self[e31]) - (other[e3] * self[e12]),
            ]) - (other.group3().xyzx() * self.group1().wwwx()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other[e2] * self[e35]) + (other[e5] * self[e23]),
                (other[e3] * self[e15]) + (other[e5] * self[e31]),
                (other[e1] * self[e25]) + (other[e5] * self[e12]),
                -(other[e431] * self[e25])
                    - (other[e412] * self[e35])
                    - (other[e415] * self[e23])
                    - (other[e425] * self[e31])
                    - (other[e435] * self[e12])
                    - (other[e321] * self[e45])
                    - (other[e235] * self[e41])
                    - (other[e315] * self[e42])
                    - (other[e125] * self[e43]),
            ]) - (self.group2().yzx() * other.group3().zxy()).with_w(other[e423] * self[e15]),
        )
    }
}
impl Wedge<AntiDualNum> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
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
impl Wedge<AntiFlatPoint> for Dipole {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            -(other[e235] * self[e41]) - (other[e315] * self[e42]) - (other[e125] * self[e43]) - (other[e321] * self[e45]),
        )
    }
}
impl Wedge<AntiFlector> for Dipole {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       15        0
    //    simd3        1        3        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       17       28        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0().yzx() * other.group1().zxy()) - (self.group0().zxy() * other.group1().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([other[e5] * self[e41], other[e5] * self[e42], other[e5] * self[e43], -(other[e2] * self[e31]) - (other[e3] * self[e12])])
                - (other.group1().xyzx() * self.group1().wwwx()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other[e2] * self[e35]) + (other[e5] * self[e23]),
                (other[e3] * self[e15]) + (other[e5] * self[e31]),
                (other[e1] * self[e25]) + (other[e5] * self[e12]),
                -(other[e315] * self[e42]) - (other[e125] * self[e43]) - (other[e321] * self[e45]),
            ]) - (self.group2().yzx() * other.group1().zxy()).with_w(other[e235] * self[e41]),
        )
    }
}
impl Wedge<AntiLine> for Dipole {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       13       18        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e23] * self[e45]) + (other[e35] * self[e42]),
                (other[e31] * self[e45]) + (other[e15] * self[e43]),
                (other[e12] * self[e45]) + (other[e25] * self[e41]),
                -(other[e31] * self[e25]) - (other[e12] * self[e35]) - (other[e15] * self[e23]) - (other[e25] * self[e31]) - (other[e35] * self[e12]),
            ]) - (other.group1().yzx() * self.group0().zxy()).with_w(other[e23] * self[e15]),
            // e1234
            -(other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
        )
    }
}
impl Wedge<AntiMotor> for Dipole {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       13       28        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            (Simd32x3::from(other[scalar]) * self.group2()).with_w(-(other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e23] * self[e45]) + (other[e35] * self[e42]),
                (other[e31] * self[e45]) + (other[e15] * self[e43]),
                (other[e12] * self[e45]) + (other[e25] * self[e41]),
                -(other[e31] * self[e25]) - (other[e12] * self[e35]) - (other[e15] * self[e23]) - (other[e25] * self[e31]) - (other[e35] * self[e12]),
            ]) - (self.group0().zxy() * other.group1().yzx()).with_w(other[e23] * self[e15]),
        )
    }
}
impl Wedge<AntiPlane> for Dipole {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        3        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd       14       24        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            (self.group0().yzx() * other.group0().zxy()) - (self.group0().zxy() * other.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([other[e5] * self[e41], other[e5] * self[e42], other[e5] * self[e43], -(other[e2] * self[e31]) - (other[e3] * self[e12])])
                - (other.group0().xyzx() * self.group1().wwwx()),
            // e235, e315, e125
            (Simd32x3::from(other[e5]) * self.group1().xyz()) + (self.group2().zxy() * other.group0().yzx()) - (self.group2().yzx() * other.group0().zxy()),
        )
    }
}
impl Wedge<Circle> for Dipole {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
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
impl Wedge<CircleRotor> for Dipole {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
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
impl Wedge<Dipole> for Dipole {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        2        0
    //    simd4        2        0        0
    // Totals...
    // yes simd       19       26        0
    //  no simd       25       30        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e42] * self[e35]) + (other[e23] * self[e45]) + (other[e45] * self[e23]) + (other[e35] * self[e42]),
                (other[e43] * self[e15]) + (other[e31] * self[e45]) + (other[e45] * self[e31]) + (other[e15] * self[e43]),
                (other[e41] * self[e25]) + (other[e12] * self[e45]) + (other[e45] * self[e12]) + (other[e25] * self[e41]),
                -(other[e23] * self[e15]) - (other[e31] * self[e25]) - (other[e12] * self[e35]) - (other[e35] * self[e12]),
            ]) - (other.group0().zxy() * self.group2().yzx()).with_w(other[e15] * self[e23])
                - (other.group2().yzx() * self.group0().zxy()).with_w(other[e25] * self[e31]),
            // e1234
            -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
        )
    }
}
impl Wedge<DipoleInversion> for Dipole {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        2        0
    //    simd4        2        0        0
    // Totals...
    // yes simd       19       26        0
    //  no simd       25       30        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e42] * other[e35]) + (self[e23] * other[e45]) + (self[e45] * other[e23]) + (self[e35] * other[e42]),
                (self[e43] * other[e15]) + (self[e31] * other[e45]) + (self[e45] * other[e31]) + (self[e15] * other[e43]),
                (self[e41] * other[e25]) + (self[e12] * other[e45]) + (self[e45] * other[e12]) + (self[e25] * other[e41]),
                -(self[e23] * other[e15]) - (self[e31] * other[e25]) - (self[e12] * other[e35]) - (self[e35] * other[e12]),
            ]) - (self.group0().zxy() * other.group2().yzx()).with_w(self[e15] * other[e23])
                - (self.group2().yzx() * other.group0().zxy()).with_w(self[e25] * other[e31]),
            // e1234
            -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]),
        )
    }
}
impl Wedge<DualNum> for Dipole {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(other[e5]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other[e5]) * self.group1().xyz(),
        )
    }
}
impl Wedge<FlatPoint> for Dipole {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e42] * other[e35]) + (self[e23] * other[e45]),
                (self[e43] * other[e15]) + (self[e31] * other[e45]),
                (self[e41] * other[e25]) + (self[e12] * other[e45]),
                -(self[e31] * other[e25]) - (self[e12] * other[e35]),
            ]) - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        )
    }
}
impl Wedge<Flector> for Dipole {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e42] * other[e35]) + (self[e23] * other[e45]),
                (self[e43] * other[e15]) + (self[e31] * other[e45]),
                (self[e41] * other[e25]) + (self[e12] * other[e45]),
                -(self[e31] * other[e25]) - (self[e12] * other[e35]),
            ]) - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        )
    }
}
impl Wedge<Line> for Dipole {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            -(self[e41] * other[e235]) - (self[e42] * other[e315]) - (self[e43] * other[e125]) - (self[e23] * other[e415]) - (self[e31] * other[e425]) - (self[e12] * other[e435]),
        )
    }
}
impl Wedge<Motor> for Dipole {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x3::from(other[e5]) * self.group0()).with_w(
                -(self[e41] * other[e235])
                    - (self[e42] * other[e315])
                    - (self[e43] * other[e125])
                    - (self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435]),
            ),
            // e235, e315, e125, e5
            (self.group1().xyz() * other.group1().www()).with_w(0.0),
        )
    }
}
impl Wedge<MultiVector> for Dipole {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       42        0
    //    simd3        4       10        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       37       54        0
    //  no simd       54       80        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(other[scalar]) * self.group2().with_w(self[e45]),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e41] * other[e5]) + (self[e15] * other[e4]),
                (self[e42] * other[e5]) + (self[e25] * other[e4]),
                (self[e43] * other[e5]) + (self[e35] * other[e4]),
                -(self[e31] * other[e2]) - (self[e12] * other[e3]),
            ]) - (self.group1().wwwx() * other.group1().xyzx()),
            // e423, e431, e412
            (Simd32x3::from(other[e4]) * self.group1().xyz()) + (self.group0().yzx() * other.group1().zxy()) - (self.group0().zxy() * other.group1().yzx()),
            // e235, e315, e125
            (Simd32x3::from(other[e5]) * self.group1().xyz()) + (self.group2().zxy() * other.group1().yzx()) - (self.group2().yzx() * other.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e42] * other[e35]) + (self[e23] * other[e45]) + (self[e45] * other[e23]) + (self[e35] * other[e42]),
                (self[e43] * other[e15]) + (self[e31] * other[e45]) + (self[e45] * other[e31]) + (self[e15] * other[e43]),
                (self[e41] * other[e25]) + (self[e12] * other[e45]) + (self[e45] * other[e12]) + (self[e25] * other[e41]),
                -(self[e23] * other[e15]) - (self[e31] * other[e25]) - (self[e12] * other[e35]) - (self[e35] * other[e12]),
            ]) - (self.group0().zxy() * other.group3().yzx()).with_w(self[e15] * other[e23])
                - (self.group2().yzx() * other.group4().zxy()).with_w(self[e25] * other[e31]),
            // e1234
            -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]),
        )
    }
}
impl Wedge<RoundPoint> for Dipole {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        4        6        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       20       30        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other[e4]) * self.group1().xyz()) + (self.group0().yzx() * other.group0().zxy()) - (self.group0().zxy() * other.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e41] * other[e5]) + (self[e15] * other[e4]),
                (self[e42] * other[e5]) + (self[e25] * other[e4]),
                (self[e43] * other[e5]) + (self[e35] * other[e4]),
                -(self[e31] * other[e2]) - (self[e12] * other[e3]),
            ]) - (self.group1().wwwx() * other.group0().xyzx()),
            // e235, e315, e125
            (Simd32x3::from(other[e5]) * self.group1().xyz()) + (self.group2().zxy() * other.group0().yzx()) - (self.group2().yzx() * other.group0().zxy()),
        )
    }
}
impl Wedge<Scalar> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn wedge(self, other: Scalar) -> Self::Output {
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
impl Wedge<VersorEven> for Dipole {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       24        0
    //    simd3        2        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       29       40        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other[e4]) * self.group1().xyz()) + (self.group0().yzx() * other.group3().zxy()) - (self.group0().zxy() * other.group3().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e41] * other[e5]) + (self[e15] * other[e4]),
                (self[e42] * other[e5]) + (self[e25] * other[e4]),
                (self[e43] * other[e5]) + (self[e35] * other[e4]),
                -(self[e31] * other[e2]) - (self[e12] * other[e3]),
            ]) - (self.group1().wwwx() * other.group3().xyzx()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (self[e23] * other[e5]) + (self[e35] * other[e2]),
                (self[e31] * other[e5]) + (self[e15] * other[e3]),
                (self[e12] * other[e5]) + (self[e25] * other[e1]),
                -(self[e42] * other[e315])
                    - (self[e43] * other[e125])
                    - (self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435])
                    - (self[e45] * other[e321])
                    - (self[e15] * other[e423])
                    - (self[e25] * other[e431])
                    - (self[e35] * other[e412]),
            ]) - (self.group2().yzx() * other.group3().zxy()).with_w(self[e41] * other[e235]),
        )
    }
}
impl Wedge<VersorOdd> for Dipole {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       25       40        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            (Simd32x3::from(other[scalar]) * self.group2()).with_w(
                -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e42] * other[e35]) + (self[e23] * other[e45]) + (self[e45] * other[e23]) + (self[e35] * other[e42]),
                (self[e43] * other[e15]) + (self[e31] * other[e45]) + (self[e45] * other[e31]) + (self[e15] * other[e43]),
                (self[e41] * other[e25]) + (self[e12] * other[e45]) + (self[e45] * other[e12]) + (self[e25] * other[e41]),
                -(self[e23] * other[e15]) - (self[e31] * other[e25]) - (self[e12] * other[e35]) - (self[e35] * other[e12]),
            ]) - (self.group0().zxy() * other.group2().yzx()).with_w(self[e15] * other[e23])
                - (self.group2().yzx() * other.group0().zxy()).with_w(self[e25] * other[e31]),
        )
    }
}
impl std::ops::Div<WedgeInfix> for DipoleInversion {
    type Output = WedgeInfixPartial<DipoleInversion>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       25        0
    //    simd3        0        4        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       21       31        0
    //  no simd       30       45        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            (self.group2().xyz() * other.group2().www()).with_w(
                (other[scalar] * self[e1234])
                    - (other[e41] * self[e23])
                    - (other[e42] * self[e31])
                    - (other[e43] * self[e12])
                    - (other[e23] * self[e41])
                    - (other[e31] * self[e42])
                    - (other[e12] * self[e43]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e23] * self[e45]) + (other[e45] * self[e23]) + (other[e35] * self[e42]) + (other[scalar] * self[e4235]),
                (other[e31] * self[e45]) + (other[e45] * self[e31]) + (other[e15] * self[e43]) + (other[scalar] * self[e4315]),
                (other[e12] * self[e45]) + (other[e45] * self[e12]) + (other[e25] * self[e41]) + (other[scalar] * self[e4125]),
                -(other[e12] * self[e35]) - (other[e15] * self[e23]) - (other[e25] * self[e31]) - (other[e35] * self[e12]),
            ]) + (other.group0().yzx() * self.group2().zxy()).with_w(other[scalar] * self[e3215])
                - (self.group2().yzxx() * other.group0().zxy().with_w(other[e23]))
                - (self.group0().zxy() * other.group2().yzx()).with_w(other[e31] * self[e25]),
        )
    }
}
impl Wedge<AntiDipoleInversion> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        2        4        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       21       28        0
    //  no simd       37       45        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other[e4]) * self.group1().xyz()) + (self.group0().yzx() * other.group3().zxy()) - (self.group0().zxy() * other.group3().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other[e4] * self[e15]) + (other[e5] * self[e41]),
                (other[e4] * self[e25]) + (other[e5] * self[e42]),
                (other[e4] * self[e35]) + (other[e5] * self[e43]),
                -(other[e2] * self[e31]) - (other[e3] * self[e12]),
            ]) - (other.group3().xyzx() * self.group1().wwwx()),
            // e235, e315, e125, e12345
            (other.group3().wwwx() * self.group1().xyz().with_w(self[e4235]))
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
                + (other.group3().yzx() * self.group2().zxy()).with_w(other[e4] * self[e3215])
                - (self.group2().yzxx() * other.group3().zxy().with_w(other[e423])),
        )
    }
}
impl Wedge<AntiDualNum> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
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
impl Wedge<AntiFlatPoint> for DipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            -(other[e235] * self[e41]) - (other[e315] * self[e42]) - (other[e125] * self[e43]) - (other[e321] * self[e45]),
        )
    }
}
impl Wedge<AntiFlector> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        3        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       10       17        0
    //  no simd       24       32        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0().yzx() * other.group1().zxy()) - (self.group0().zxy() * other.group1().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([other[e5] * self[e41], other[e5] * self[e42], other[e5] * self[e43], -(other[e2] * self[e31]) - (other[e3] * self[e12])])
                - (other.group1().xyzx() * self.group1().wwwx()),
            // e235, e315, e125, e12345
            (other.group1().yzxx() * self.group2().zxy().with_w(self[e4235]))
                + (other.group1().wwwy() * self.group1().xyz().with_w(self[e4315]))
                + Simd32x3::from(0.0)
                    .with_w((other[e3] * self[e4125]) + (other[e5] * self[e1234]) - (other[e315] * self[e42]) - (other[e125] * self[e43]) - (other[e321] * self[e45]))
                - (other.group1().zxy() * self.group2().yzx()).with_w(other[e235] * self[e41]),
        )
    }
}
impl Wedge<AntiLine> for DipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       13       18        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e23] * self[e45]) + (other[e35] * self[e42]),
                (other[e31] * self[e45]) + (other[e15] * self[e43]),
                (other[e12] * self[e45]) + (other[e25] * self[e41]),
                -(other[e31] * self[e25]) - (other[e12] * self[e35]) - (other[e15] * self[e23]) - (other[e25] * self[e31]) - (other[e35] * self[e12]),
            ]) - (other.group1().yzx() * self.group0().zxy()).with_w(other[e23] * self[e15]),
            // e1234
            -(other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
        )
    }
}
impl Wedge<AntiMotor> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       17        0
    //    simd3        0        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       12       22        0
    //  no simd       18       33        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            (self.group2().xyz() * other.group0().www()).with_w((other[scalar] * self[e1234]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e23] * self[e45]) + (other[scalar] * self[e4235]),
                (other[e31] * self[e45]) + (other[scalar] * self[e4315]),
                (other[e12] * self[e45]) + (other[scalar] * self[e4125]),
                -(other[e31] * self[e25]) - (other[e12] * self[e35]) - (other[e15] * self[e23]) - (other[e25] * self[e31]) - (other[e35] * self[e12]),
            ]) + (self.group0().yzx() * other.group1().zxy()).with_w(other[scalar] * self[e3215])
                - (self.group0().zxy() * other.group1().yzx()).with_w(other[e23] * self[e15]),
        )
    }
}
impl Wedge<AntiPlane> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       13        0
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        6       18        0
    //  no simd       17       31        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0().yzx() * other.group0().zxy()) - (self.group0().zxy() * other.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([other[e5] * self[e41], other[e5] * self[e42], other[e5] * self[e43], -(other[e2] * self[e31]) - (other[e3] * self[e12])])
                - (other.group0().xyzx() * self.group1().wwwx()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                other[e3] * self[e25] * -1.0,
                other[e1] * self[e35] * -1.0,
                other[e2] * self[e15] * -1.0,
                (other[e3] * self[e4125]) + (other[e5] * self[e1234]),
            ]) + (other.group0().yzxx() * self.group2().zxy().with_w(self[e4235]))
                + (other.group0().wwwy() * self.group1().xyz().with_w(self[e4315])),
        )
    }
}
impl Wedge<Circle> for DipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
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
impl Wedge<CircleRotor> for DipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
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
impl Wedge<Dipole> for DipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        2        0
    //    simd4        2        0        0
    // Totals...
    // yes simd       19       26        0
    //  no simd       25       30        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e42] * self[e35]) + (other[e23] * self[e45]) + (other[e45] * self[e23]) + (other[e35] * self[e42]),
                (other[e43] * self[e15]) + (other[e31] * self[e45]) + (other[e45] * self[e31]) + (other[e15] * self[e43]),
                (other[e41] * self[e25]) + (other[e12] * self[e45]) + (other[e45] * self[e12]) + (other[e25] * self[e41]),
                -(other[e23] * self[e15]) - (other[e31] * self[e25]) - (other[e12] * self[e35]) - (other[e35] * self[e12]),
            ]) - (other.group0().zxy() * self.group2().yzx()).with_w(other[e15] * self[e23])
                - (other.group2().yzx() * self.group0().zxy()).with_w(other[e25] * self[e31]),
            // e1234
            -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
        )
    }
}
impl Wedge<DipoleInversion> for DipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       23        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       25        0
    //  no simd       25       30        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e42] * self[e35]) + (other[e23] * self[e45]) + (other[e45] * self[e23]) + (other[e35] * self[e42]),
                (other[e43] * self[e15]) + (other[e31] * self[e45]) + (other[e45] * self[e31]) + (other[e15] * self[e43]),
                (other[e41] * self[e25]) + (other[e12] * self[e45]) + (other[e45] * self[e12]) + (other[e25] * self[e41]),
                -(other[e12] * self[e35]) - (other[e15] * self[e23]) - (other[e25] * self[e31]) - (other[e35] * self[e12]),
            ]) - (self.group2().yzxx() * other.group0().zxy().with_w(other[e23]))
                - (self.group0().zxy() * other.group2().yzx()).with_w(other[e31] * self[e25]),
            // e1234
            -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
        )
    }
}
impl Wedge<DualNum> for DipoleInversion {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(other[e5]) * self.group0().with_w(self[e1234]),
            // e235, e315, e125, e5
            (self.group1().xyz() * other.group0().xx().with_z(other[e5])).with_w(0.0),
        )
    }
}
impl Wedge<FlatPoint> for DipoleInversion {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e42] * other[e35]) + (self[e23] * other[e45]),
                (self[e43] * other[e15]) + (self[e31] * other[e45]),
                (self[e41] * other[e25]) + (self[e12] * other[e45]),
                -(self[e31] * other[e25]) - (self[e12] * other[e35]),
            ]) - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        )
    }
}
impl Wedge<Flector> for DipoleInversion {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e42] * other[e35]) + (self[e23] * other[e45]),
                (self[e43] * other[e15]) + (self[e31] * other[e45]),
                (self[e41] * other[e25]) + (self[e12] * other[e45]),
                -(self[e31] * other[e25]) - (self[e12] * other[e35]),
            ]) - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        )
    }
}
impl Wedge<Line> for DipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            -(self[e41] * other[e235]) - (self[e42] * other[e315]) - (self[e43] * other[e125]) - (self[e23] * other[e415]) - (self[e31] * other[e425]) - (self[e12] * other[e435]),
        )
    }
}
impl Wedge<Motor> for DipoleInversion {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        6        9        0
    //  no simd        6       13        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x3::from(other[e5]) * self.group0()).with_w(
                (self[e1234] * other[e5])
                    - (self[e41] * other[e235])
                    - (self[e42] * other[e315])
                    - (self[e43] * other[e125])
                    - (self[e23] * other[e415])
                    - (self[e31] * other[e425])
                    - (self[e12] * other[e435]),
            ),
            // e235, e315, e125, e5
            (self.group1().xyz() * other.group1().www()).with_w(0.0),
        )
    }
}
impl Wedge<MultiVector> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       47        0
    //    simd3        4        9        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       44       60        0
    //  no simd       64       90        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(other[scalar]) * self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e41] * other[e5]) + (self[e15] * other[e4]),
                (self[e42] * other[e5]) + (self[e25] * other[e4]),
                (self[e43] * other[e5]) + (self[e35] * other[e4]),
                -(self[e31] * other[e2]) - (self[e12] * other[e3]),
            ]) - (self.group1().wwwx() * other.group1().xyzx()),
            // e423, e431, e412
            (Simd32x3::from(other[e4]) * self.group1().xyz()) + (self.group0().yzx() * other.group1().zxy()) - (self.group0().zxy() * other.group1().yzx()),
            // e235, e315, e125
            (Simd32x3::from(other[e5]) * self.group1().xyz()) + (self.group2().zxy() * other.group1().yzx()) - (self.group2().yzx() * other.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e42] * other[e35]) + (self[e23] * other[e45]) + (self[e45] * other[e23]) + (self[e35] * other[e42]),
                (self[e43] * other[e15]) + (self[e31] * other[e45]) + (self[e45] * other[e31]) + (self[e15] * other[e43]),
                (self[e41] * other[e25]) + (self[e12] * other[e45]) + (self[e45] * other[e12]) + (self[e25] * other[e41]),
                -(self[e23] * other[e15]) - (self[e31] * other[e25]) - (self[e12] * other[e35]) - (self[e35] * other[e12]),
            ]) + (Simd32x4::from(other[scalar]) * self.group3())
                - (self.group2().yzxy() * other.group4().zxy().with_w(other[e31]))
                - (self.group0().zxy() * other.group3().yzx()).with_w(self[e15] * other[e23]),
            // e1234
            (self[e1234] * other[scalar])
                - (self[e41] * other[e23])
                - (self[e42] * other[e31])
                - (self[e43] * other[e12])
                - (self[e23] * other[e41])
                - (self[e31] * other[e42])
                - (self[e12] * other[e43]),
        )
    }
}
impl Wedge<RoundPoint> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        2        3        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       24       38        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other[e4]) * self.group1().xyz()) + (self.group0().yzx() * other.group0().zxy()) - (self.group0().zxy() * other.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e41] * other[e5]) + (self[e15] * other[e4]),
                (self[e42] * other[e5]) + (self[e25] * other[e4]),
                (self[e43] * other[e5]) + (self[e35] * other[e4]),
                -(self[e31] * other[e2]) - (self[e12] * other[e3]),
            ]) - (self.group1().wwwx() * other.group0().xyzx()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                self[e25] * other[e3] * -1.0,
                self[e35] * other[e1] * -1.0,
                self[e15] * other[e2] * -1.0,
                (self[e4315] * other[e2]) + (self[e4125] * other[e3]) + (self[e3215] * other[e4]),
            ]) + (Simd32x4::from(other[e5]) * self.group1().xyz().with_w(self[e1234]))
                + (other.group0().yzxx() * self.group2().zxy().with_w(self[e4235])),
        )
    }
}
impl Wedge<Scalar> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn wedge(self, other: Scalar) -> Self::Output {
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
impl Wedge<VersorEven> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        2        4        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       21       28        0
    //  no simd       37       45        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other[e4]) * self.group1().xyz()) + (self.group0().yzx() * other.group3().zxy()) - (self.group0().zxy() * other.group3().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e41] * other[e5]) + (self[e15] * other[e4]),
                (self[e42] * other[e5]) + (self[e25] * other[e4]),
                (self[e43] * other[e5]) + (self[e35] * other[e4]),
                -(self[e31] * other[e2]) - (self[e12] * other[e3]),
            ]) - (self.group1().wwwx() * other.group3().xyzx()),
            // e235, e315, e125, e12345
            (Simd32x4::from(other[e5]) * self.group1().xyz().with_w(self[e1234]))
                + (other.group3().yzxx() * self.group2().zxy().with_w(self[e4235]))
                + Simd32x3::from(0.0).with_w(
                    (self[e4315] * other[e2]) + (self[e4125] * other[e3]) + (self[e3215] * other[e4])
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
                - (self.group2().yzx() * other.group3().zxy()).with_w(self[e41] * other[e235]),
        )
    }
}
impl Wedge<VersorOdd> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       25        0
    //    simd3        0        4        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       21       31        0
    //  no simd       30       45        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            (Simd32x3::from(other[scalar]) * self.group2().xyz()).with_w(
                (self[e1234] * other[scalar])
                    - (self[e41] * other[e23])
                    - (self[e42] * other[e31])
                    - (self[e43] * other[e12])
                    - (self[e23] * other[e41])
                    - (self[e31] * other[e42])
                    - (self[e12] * other[e43]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e23] * other[e45]) + (self[e45] * other[e23]) + (self[e35] * other[e42]) + (self[e4235] * other[scalar]),
                (self[e31] * other[e45]) + (self[e45] * other[e31]) + (self[e15] * other[e43]) + (self[e4315] * other[scalar]),
                (self[e12] * other[e45]) + (self[e45] * other[e12]) + (self[e25] * other[e41]) + (self[e4125] * other[scalar]),
                -(self[e12] * other[e35]) - (self[e15] * other[e23]) - (self[e25] * other[e31]) - (self[e35] * other[e12]),
            ]) + (self.group0().yzx() * other.group2().zxy()).with_w(self[e3215] * other[scalar])
                - (other.group2().yzxx() * self.group0().zxy().with_w(self[e23]))
                - (self.group2().yzx() * other.group0().zxy()).with_w(self[e31] * other[e25]),
        )
    }
}
impl std::ops::Div<WedgeInfix> for DualNum {
    type Output = WedgeInfixPartial<DualNum>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xx().with_zw(self[e5], self[e12345]) * other.group0().with_w(other[scalar]),
            // e235, e315, e125, e5
            Simd32x4::from(self[e5]) * other.group1().xyz().with_w(other[scalar]),
        )
    }
}
impl Wedge<AntiDipoleInversion> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[e5]) * other.group3().xyz().with_w(other[e4]) * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e5]) * other.group0().with_w(other[e321]) * Simd32x4::from(-1.0),
        )
    }
}
impl Wedge<AntiDualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(other[scalar]) * self.group0())
    }
}
impl Wedge<AntiFlatPoint> for DualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([other[e321] * self[e5], 1.0]) * Simd32x2::from([-1.0, 0.0]))
    }
}
impl Wedge<AntiFlector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       11        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            (other.group1().xyz() * self.group0().xx().with_z(self[e5]) * Simd32x3::from(-1.0)).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(other[e321] * self[e5]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        )
    }
}
impl Wedge<AntiLine> for DualNum {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ (other.group0() * self.group0().xx().with_z(self[e5])).with_w(0.0))
    }
}
impl Wedge<AntiMotor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).with_w(other[scalar] * self[e12345]),
            // e235, e315, e125, e5
            Simd32x4::from(self[e5]) * other.group0(),
        )
    }
}
impl Wedge<AntiPlane> for DualNum {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            (other.group0().xyz() * self.group0().xx().with_z(self[e5]) * Simd32x3::from(-1.0)).with_w(0.0),
        )
    }
}
impl Wedge<Circle> for DualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e5]) * other.group0().with_w(other[e321]) * Simd32x4::from(-1.0),
        )
    }
}
impl Wedge<CircleRotor> for DualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e5]) * other.group0().with_w(other[e321]) * Simd32x4::from(-1.0),
        )
    }
}
impl Wedge<Dipole> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(self[e5]) * other.group0(),
            // e235, e315, e125
            Simd32x3::from(self[e5]) * other.group1().xyz(),
        )
    }
}
impl Wedge<DipoleInversion> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(self[e5]) * other.group0().with_w(other[e1234]),
            // e235, e315, e125, e5
            (other.group1().xyz() * self.group0().xx().with_z(self[e5])).with_w(0.0),
        )
    }
}
impl Wedge<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        9        0
    //  no simd        1       25        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (self[e5] * other[e1234]) + (self[e12345] * other[scalar])]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5] * other[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(self[e5]) * other.group1() * Simd32x4::from(-1.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (other.group4() * self.group0().xx().with_z(self[e5])).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[e5]) * other.group5(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e5]) * other.group7().with_w(other[e321]) * Simd32x4::from(-1.0),
            // e1234
            0.0,
        )
    }
}
impl Wedge<RoundPoint> for DualNum {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self[e5]) * other.group0() * Simd32x4::from(-1.0))
    }
}
impl Wedge<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(other[scalar]) * self.group0())
    }
}
impl Wedge<Sphere> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e5] * other[e1234])
    }
}
impl Wedge<VersorEven> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[e5]) * other.group3() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e5]) * other.group0().xyz().with_w(other[e321]) * Simd32x4::from(-1.0),
        )
    }
}
impl Wedge<VersorOdd> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1       10        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xx().with_zw(self[e5], (self[e5] * other[e1234]) + (self[e12345] * other[scalar])) * other.group0().xyz().with_w(1.0),
            // e235, e315, e125, e5
            Simd32x4::from(self[e5]) * other.group1().xyz().with_w(other[scalar]),
        )
    }
}
impl std::ops::Div<WedgeInfix> for FlatPoint {
    type Output = WedgeInfixPartial<FlatPoint>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for FlatPoint {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[scalar]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e42] * self[e35]) + (other[e23] * self[e45]),
                (other[e43] * self[e15]) + (other[e31] * self[e45]),
                (other[e41] * self[e25]) + (other[e12] * self[e45]),
                -(other[e31] * self[e25]) - (other[e12] * self[e35]),
            ]) - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23])),
        )
    }
}
impl Wedge<AntiDipoleInversion> for FlatPoint {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        9       16        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                other[e4] * self[e15],
                other[e4] * self[e25],
                other[e4] * self[e35],
                -(other[e431] * self[e25]) - (other[e412] * self[e35]) - (other[e321] * self[e45]),
            ]) - (self.group0().wwwx() * other.group3().xyz().with_w(other[e423])),
            // e235, e315, e125, e5
            ((other.group3().yzx() * self.group0().zxy()) - (other.group3().zxy() * self.group0().yzx())).with_w(0.0),
        )
    }
}
impl Wedge<AntiDualNum> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl Wedge<AntiFlatPoint> for FlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[e321] * self[e45] * -1.0)
    }
}
impl Wedge<AntiFlector> for FlatPoint {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        3       14        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(self[e45]) * other.group1().xyz().with_w(other[e321]) * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            ((other.group1().yzx() * self.group0().zxy()) - (other.group1().zxy() * self.group0().yzx())).with_w(0.0),
        )
    }
}
impl Wedge<AntiLine> for FlatPoint {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(self[e45]) * other.group0()).with_w(-(other[e23] * self[e15]) - (other[e31] * self[e25]) - (other[e12] * self[e35])),
        )
    }
}
impl Wedge<AntiMotor> for FlatPoint {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2       10        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[scalar]) * self.group0(),
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(self[e45]) * other.group0().xyz()).with_w(-(other[e23] * self[e15]) - (other[e31] * self[e25]) - (other[e12] * self[e35])),
        )
    }
}
impl Wedge<AntiPlane> for FlatPoint {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        4        0
    // no simd        3       12        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(self[e45]) * other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx()),
        )
    }
}
impl Wedge<Circle> for FlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            -(other[e423] * self[e15]) - (other[e431] * self[e25]) - (other[e412] * self[e35]) - (other[e321] * self[e45]),
        )
    }
}
impl Wedge<CircleRotor> for FlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            -(other[e423] * self[e15]) - (other[e431] * self[e25]) - (other[e412] * self[e35]) - (other[e321] * self[e45]),
        )
    }
}
impl Wedge<Dipole> for FlatPoint {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e42] * self[e35]) + (other[e23] * self[e45]),
                (other[e43] * self[e15]) + (other[e31] * self[e45]),
                (other[e41] * self[e25]) + (other[e12] * self[e45]),
                -(other[e31] * self[e25]) - (other[e12] * self[e35]),
            ]) - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23])),
        )
    }
}
impl Wedge<DipoleInversion> for FlatPoint {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e42] * self[e35]) + (other[e23] * self[e45]),
                (other[e43] * self[e15]) + (other[e31] * self[e45]),
                (other[e41] * self[e25]) + (other[e12] * self[e45]),
                -(other[e31] * self[e25]) - (other[e12] * self[e35]),
            ]) - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23])),
        )
    }
}
impl Wedge<MultiVector> for FlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd3        2        4        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       17       32        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, -(self[e15] * other[e423]) - (self[e25] * other[e431]) - (self[e35] * other[e412]) - (self[e45] * other[e321])]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(other[scalar]) * self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e4]) * self.group0().xyz()) - (Simd32x3::from(self[e45]) * other.group1().xyz())).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (self.group0().zxy() * other.group1().yzx()) - (self.group0().yzx() * other.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e35] * other[e42]) + (self[e45] * other[e23]),
                (self[e15] * other[e43]) + (self[e45] * other[e31]),
                (self[e25] * other[e41]) + (self[e45] * other[e12]),
                -(self[e25] * other[e31]) - (self[e35] * other[e12]),
            ]) - (self.group0().yzxx() * other.group4().zxy().with_w(other[e23])),
            // e1234
            0.0,
        )
    }
}
impl Wedge<RoundPoint> for FlatPoint {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        4        0
    // no simd        6       12        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            (Simd32x3::from(other[e4]) * self.group0().xyz()) - (Simd32x3::from(self[e45]) * other.group0().xyz()),
            // e235, e315, e125
            (self.group0().zxy() * other.group0().yzx()) - (self.group0().yzx() * other.group0().zxy()),
        )
    }
}
impl Wedge<Scalar> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl Wedge<VersorEven> for FlatPoint {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        9       16        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                self[e15] * other[e4],
                self[e25] * other[e4],
                self[e35] * other[e4],
                -(self[e25] * other[e431]) - (self[e35] * other[e412]) - (self[e45] * other[e321]),
            ]) - (self.group0().wwwx() * other.group3().xyz().with_w(other[e423])),
            // e235, e315, e125, e5
            ((self.group0().zxy() * other.group3().yzx()) - (self.group0().yzx() * other.group3().zxy())).with_w(0.0),
        )
    }
}
impl Wedge<VersorOdd> for FlatPoint {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[scalar]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e35] * other[e42]) + (self[e45] * other[e23]),
                (self[e15] * other[e43]) + (self[e45] * other[e31]),
                (self[e25] * other[e41]) + (self[e45] * other[e12]),
                -(self[e25] * other[e31]) - (self[e35] * other[e12]),
            ]) - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23])),
        )
    }
}
impl std::ops::Div<WedgeInfix> for Flector {
    type Output = WedgeInfixPartial<Flector>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       20        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[scalar]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e23] * self[e45]) + (other[scalar] * self[e4235]),
                (other[e31] * self[e45]) + (other[scalar] * self[e4315]),
                (other[e12] * self[e45]) + (other[scalar] * self[e4125]),
                -(other[e31] * self[e25]) - (other[e12] * self[e35]),
            ]) + (other.group0().yzx() * self.group0().zxy()).with_w(other[scalar] * self[e3215])
                - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23])),
        )
    }
}
impl Wedge<AntiDipoleInversion> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        8       10        0
    //  no simd       16       20        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[e3215]))
                + Simd32x3::from(0.0).with_w(
                    (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125])
                        - (other[e431] * self[e25])
                        - (other[e412] * self[e35])
                        - (other[e321] * self[e45]),
                )
                - (self.group0().wwwx() * other.group3().xyz().with_w(other[e423])),
            // e235, e315, e125, e5
            ((other.group3().yzx() * self.group0().zxy()) - (other.group3().zxy() * self.group0().yzx())).with_w(0.0),
        )
    }
}
impl Wedge<AntiDualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[scalar]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl Wedge<AntiFlatPoint> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[e321] * self[e45] * -1.0)
    }
}
impl Wedge<AntiFlector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        1        4        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        6       16        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x3::from(self[e45]) * other.group1().xyz() * Simd32x3::from(-1.0))
                .with_w((other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]) - (other[e321] * self[e45])),
            // e235, e315, e125, e5
            ((other.group1().yzx() * self.group0().zxy()) - (other.group1().zxy() * self.group0().yzx())).with_w(0.0),
        )
    }
}
impl Wedge<AntiLine> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(self[e45]) * other.group0()).with_w(-(other[e23] * self[e15]) - (other[e31] * self[e25]) - (other[e12] * self[e35])),
        )
    }
}
impl Wedge<AntiMotor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        6       14        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[scalar]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other[scalar] * self[e4235],
                other[scalar] * self[e4315],
                other[scalar] * self[e4125],
                -(other[e23] * self[e15]) - (other[e31] * self[e25]) - (other[e12] * self[e35]),
            ]) + (other.group0() * self.group0().www().with_w(self[e3215])),
        )
    }
}
impl Wedge<AntiPlane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        1        4        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        5       15        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x3::from(self[e45]) * other.group0().xyz() * Simd32x3::from(-1.0)).with_w((other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125])),
            // e235, e315, e125, e5
            ((other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx())).with_w(0.0),
        )
    }
}
impl Wedge<Circle> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            -(other[e423] * self[e15]) - (other[e431] * self[e25]) - (other[e412] * self[e35]) - (other[e321] * self[e45]),
        )
    }
}
impl Wedge<CircleRotor> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            -(other[e423] * self[e15]) - (other[e431] * self[e25]) - (other[e412] * self[e35]) - (other[e321] * self[e45]),
        )
    }
}
impl Wedge<Dipole> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e42] * self[e35]) + (other[e23] * self[e45]),
                (other[e43] * self[e15]) + (other[e31] * self[e45]),
                (other[e41] * self[e25]) + (other[e12] * self[e45]),
                -(other[e31] * self[e25]) - (other[e12] * self[e35]),
            ]) - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23])),
        )
    }
}
impl Wedge<DipoleInversion> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e42] * self[e35]) + (other[e23] * self[e45]),
                (other[e43] * self[e15]) + (other[e31] * self[e45]),
                (other[e41] * self[e25]) + (other[e12] * self[e45]),
                -(other[e31] * self[e25]) - (other[e12] * self[e35]),
            ]) - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23])),
        )
    }
}
impl Wedge<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       16        0
    //    simd3        2        4        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       15       23        0
    //  no simd       25       40        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (self[e4235] * other[e1]) + (self[e4315] * other[e2]) + (self[e4125] * other[e3]) + (self[e3215] * other[e4])
                    - (self[e15] * other[e423])
                    - (self[e25] * other[e431])
                    - (self[e35] * other[e412])
                    - (self[e45] * other[e321]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(other[scalar]) * self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e4]) * self.group0().xyz()) - (Simd32x3::from(self[e45]) * other.group1().xyz())).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (self.group0().zxy() * other.group1().yzx()) - (self.group0().yzx() * other.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e35] * other[e42]) + (self[e45] * other[e23]),
                (self[e15] * other[e43]) + (self[e45] * other[e31]),
                (self[e25] * other[e41]) + (self[e45] * other[e12]),
                -(self[e25] * other[e31]) - (self[e35] * other[e12]),
            ]) + (Simd32x4::from(other[scalar]) * self.group1())
                - (self.group0().yzxx() * other.group4().zxy().with_w(other[e23])),
            // e1234
            0.0,
        )
    }
}
impl Wedge<RoundPoint> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        9       19        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                self[e45] * other[e1] * -1.0,
                self[e45] * other[e2] * -1.0,
                self[e45] * other[e3] * -1.0,
                (self[e4315] * other[e2]) + (self[e4125] * other[e3]) + (self[e3215] * other[e4]),
            ]) + (other.group0().wwwx() * self.group0().xyz().with_w(self[e4235])),
            // e235, e315, e125, e5
            ((self.group0().zxy() * other.group0().yzx()) - (self.group0().yzx() * other.group0().zxy())).with_w(0.0),
        )
    }
}
impl Wedge<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[scalar]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl Wedge<VersorEven> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        8       10        0
    //  no simd       16       20        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (other.group3().wwwx() * self.group0().xyz().with_w(self[e4235]))
                + Simd32x3::from(0.0).with_w(
                    (self[e4315] * other[e2]) + (self[e4125] * other[e3]) + (self[e3215] * other[e4])
                        - (self[e25] * other[e431])
                        - (self[e35] * other[e412])
                        - (self[e45] * other[e321]),
                )
                - (self.group0().wwwx() * other.group3().xyz().with_w(other[e423])),
            // e235, e315, e125, e5
            ((self.group0().zxy() * other.group3().yzx()) - (self.group0().yzx() * other.group3().zxy())).with_w(0.0),
        )
    }
}
impl Wedge<VersorOdd> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       20        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[scalar]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e45] * other[e23]) + (self[e4235] * other[scalar]),
                (self[e45] * other[e31]) + (self[e4315] * other[scalar]),
                (self[e45] * other[e12]) + (self[e4125] * other[scalar]),
                -(self[e25] * other[e31]) - (self[e35] * other[e12]),
            ]) + (other.group0().yzxw() * self.group0().zxy().with_w(self[e3215]))
                - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23])),
        )
    }
}
impl std::ops::Div<WedgeInfix> for Line {
    type Output = WedgeInfixPartial<Line>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x3::from(other[scalar]) * self.group0()).with_w(
                -(other[e41] * self[e235])
                    - (other[e42] * self[e315])
                    - (other[e43] * self[e125])
                    - (other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435]),
            ),
            // e235, e315, e125, e5
            (self.group1() * other.group2().www()).with_w(0.0),
        )
    }
}
impl Wedge<AntiDipoleInversion> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(other[e4] * self[e235]) - (other[e3] * self[e425]),
                -(other[e4] * self[e315]) - (other[e1] * self[e435]),
                -(other[e4] * self[e125]) - (other[e2] * self[e415]),
                (other[e2] * self[e315]) + (other[e3] * self[e125]),
            ]) + (other.group3().yzxx() * self.group0().zxy().with_w(self[e235])),
        )
    }
}
impl Wedge<AntiDualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(other[scalar]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * self.group1(),
        )
    }
}
impl Wedge<AntiFlector> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        9        0
    //  no simd        5       12        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other[e3] * self[e425] * -1.0,
                other[e1] * self[e435] * -1.0,
                other[e2] * self[e415] * -1.0,
                (other[e2] * self[e315]) + (other[e3] * self[e125]),
            ]) + (other.group1().yzxx() * self.group0().zxy().with_w(self[e235])),
        )
    }
}
impl Wedge<AntiLine> for Line {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ -(other[e23] * self[e415]) - (other[e31] * self[e425]) - (other[e12] * self[e435]))
    }
}
impl Wedge<AntiMotor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x3::from(other[scalar]) * self.group0()).with_w(-(other[e23] * self[e415]) - (other[e31] * self[e425]) - (other[e12] * self[e435])),
            // e235, e315, e125, e5
            (self.group1() * other.group0().www()).with_w(0.0),
        )
    }
}
impl Wedge<AntiPlane> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        9        0
    //  no simd        5       12        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other[e3] * self[e425] * -1.0,
                other[e1] * self[e435] * -1.0,
                other[e2] * self[e415] * -1.0,
                (other[e2] * self[e315]) + (other[e3] * self[e125]),
            ]) + (other.group0().yzxx() * self.group0().zxy().with_w(self[e235])),
        )
    }
}
impl Wedge<Dipole> for Line {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            -(other[e41] * self[e235]) - (other[e42] * self[e315]) - (other[e43] * self[e125]) - (other[e23] * self[e415]) - (other[e31] * self[e425]) - (other[e12] * self[e435]),
        )
    }
}
impl Wedge<DipoleInversion> for Line {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            -(other[e41] * self[e235]) - (other[e42] * self[e315]) - (other[e43] * self[e125]) - (other[e23] * self[e415]) - (other[e31] * self[e425]) - (other[e12] * self[e435]),
        )
    }
}
impl Wedge<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       14        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       17        0
    //  no simd       13       24        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                -(self[e415] * other[e23])
                    - (self[e425] * other[e31])
                    - (self[e435] * other[e12])
                    - (self[e235] * other[e41])
                    - (self[e315] * other[e42])
                    - (self[e125] * other[e43]),
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
            (self.group0() * other.group0().xx().with_z(other[scalar])).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * other[e3]) - (self[e235] * other[e4]),
                -(self[e435] * other[e1]) - (self[e315] * other[e4]),
                -(self[e415] * other[e2]) - (self[e125] * other[e4]),
                (self[e315] * other[e2]) + (self[e125] * other[e3]),
            ]) + (other.group1().yzxx() * self.group0().zxy().with_w(self[e235])),
            // e1234
            0.0,
        )
    }
}
impl Wedge<RoundPoint> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * other[e3]) - (self[e235] * other[e4]),
                -(self[e435] * other[e1]) - (self[e315] * other[e4]),
                -(self[e415] * other[e2]) - (self[e125] * other[e4]),
                (self[e315] * other[e2]) + (self[e125] * other[e3]),
            ]) + (other.group0().yzxx() * self.group0().zxy().with_w(self[e235])),
        )
    }
}
impl Wedge<Scalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(other[scalar]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * self.group1(),
        )
    }
}
impl Wedge<VersorEven> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * other[e3]) - (self[e235] * other[e4]),
                -(self[e435] * other[e1]) - (self[e315] * other[e4]),
                -(self[e415] * other[e2]) - (self[e125] * other[e4]),
                (self[e315] * other[e2]) + (self[e125] * other[e3]),
            ]) + (other.group3().yzxx() * self.group0().zxy().with_w(self[e235])),
        )
    }
}
impl Wedge<VersorOdd> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x3::from(other[scalar]) * self.group0()).with_w(
                -(self[e415] * other[e23])
                    - (self[e425] * other[e31])
                    - (self[e435] * other[e12])
                    - (self[e235] * other[e41])
                    - (self[e315] * other[e42])
                    - (self[e125] * other[e43]),
            ),
            // e235, e315, e125, e5
            (self.group1() * other.group0().www()).with_w(0.0),
        )
    }
}
impl std::ops::Div<WedgeInfix> for Motor {
    type Output = WedgeInfixPartial<Motor>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7       14        0
    //  no simd       12       20        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                other[scalar] * self[e415],
                other[scalar] * self[e425],
                other[scalar] * self[e435],
                -(other[e41] * self[e235])
                    - (other[e42] * self[e315])
                    - (other[e43] * self[e125])
                    - (other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435]),
            ]) + (other.group0() * self.group1().www()).with_w(other[scalar] * self[e12345]),
            // e235, e315, e125, e5
            ((Simd32x3::from(other[scalar]) * self.group1().xyz()) + (Simd32x3::from(self[e5]) * other.group1().xyz())).with_w(other[scalar] * self[e5]),
        )
    }
}
impl Wedge<AntiDipoleInversion> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       24        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[e5]) * other.group3().xyz().with_w(other[e4]) * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(other[e4] * self[e235]) - (other[e3] * self[e425]),
                -(other[e4] * self[e315]) - (other[e1] * self[e435]),
                -(other[e4] * self[e125]) - (other[e2] * self[e415]),
                (other[e2] * self[e315]) + (other[e3] * self[e125]),
            ]) + (other.group3().yzxx() * self.group0().zxy().with_w(self[e235]))
                - (Simd32x4::from(self[e5]) * other.group0().with_w(other[e321])),
        )
    }
}
impl Wedge<AntiDualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(other[scalar]) * self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl Wedge<AntiFlatPoint> for Motor {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([other[e321] * self[e5], 1.0]) * Simd32x2::from([-1.0, 0.0]))
    }
}
impl Wedge<AntiFlector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        0        3        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        9       16        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            (other.group1().xyz() * self.group1().www() * Simd32x3::from(-1.0)).with_w(0.0),
            // e4235, e4315, e4125, e3215
            (other.group1().yzxx() * self.group0().zxy().with_w(self[e235])) + Simd32x3::from(0.0).with_w((other[e2] * self[e315]) + (other[e3] * self[e125]))
                - (other.group1().zxy() * self.group0().yzx()).with_w(other[e321] * self[e5]),
        )
    }
}
impl Wedge<AntiLine> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).with_w(-(other[e23] * self[e415]) - (other[e31] * self[e425]) - (other[e12] * self[e435])),
            // e235, e315, e125, e5
            (other.group0() * self.group1().www()).with_w(0.0),
        )
    }
}
impl Wedge<AntiMotor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        1        3        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        6       14        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (self.group0().xyz() * other.group0().www()).with_w((other[scalar] * self[e12345]) - (other[e23] * self[e415]) - (other[e31] * self[e425]) - (other[e12] * self[e435])),
            // e235, e315, e125, e5
            ((Simd32x3::from(other[scalar]) * self.group1().xyz()) + (Simd32x3::from(self[e5]) * other.group0().xyz())).with_w(other[scalar] * self[e5]),
        )
    }
}
impl Wedge<AntiPlane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2       11        0
    //  no simd        5       18        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            (other.group0().xyz() * self.group1().www() * Simd32x3::from(-1.0)).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other[e3] * self[e425] * -1.0,
                other[e1] * self[e435] * -1.0,
                other[e2] * self[e415] * -1.0,
                (other[e2] * self[e315]) + (other[e3] * self[e125]),
            ]) + (other.group0().yzxx() * self.group0().zxy().with_w(self[e235])),
        )
    }
}
impl Wedge<Circle> for Motor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e5]) * other.group0().with_w(other[e321]) * Simd32x4::from(-1.0),
        )
    }
}
impl Wedge<CircleRotor> for Motor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e5]) * other.group0().with_w(other[e321]) * Simd32x4::from(-1.0),
        )
    }
}
impl Wedge<Dipole> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x3::from(self[e5]) * other.group0()).with_w(
                -(other[e41] * self[e235])
                    - (other[e42] * self[e315])
                    - (other[e43] * self[e125])
                    - (other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435]),
            ),
            // e235, e315, e125, e5
            (other.group1().xyz() * self.group1().www()).with_w(0.0),
        )
    }
}
impl Wedge<DipoleInversion> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        6        9        0
    //  no simd        6       13        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x3::from(self[e5]) * other.group0()).with_w(
                (other[e1234] * self[e5])
                    - (other[e41] * self[e235])
                    - (other[e42] * self[e315])
                    - (other[e43] * self[e125])
                    - (other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435]),
            ),
            // e235, e315, e125, e5
            (other.group1().xyz() * self.group1().www()).with_w(0.0),
        )
    }
}
impl Wedge<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       18        0
    //    simd3        2        5        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       15       26        0
    //  no simd       25       45        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (self[e12345] * other[scalar]) + (self[e5] * other[e1234])
                    - (self[e415] * other[e23])
                    - (self[e425] * other[e31])
                    - (self[e435] * other[e12])
                    - (self[e235] * other[e41])
                    - (self[e315] * other[e42])
                    - (self[e125] * other[e43]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5] * other[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(self[e5]) * other.group1() * Simd32x4::from(-1.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e5]) * other.group4()) + (Simd32x3::from(other[scalar]) * self.group0().xyz())).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * other.group5()) + (Simd32x3::from(other[scalar]) * self.group1().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * other[e3]) - (self[e235] * other[e4]),
                -(self[e435] * other[e1]) - (self[e315] * other[e4]),
                -(self[e415] * other[e2]) - (self[e125] * other[e4]),
                (self[e315] * other[e2]) + (self[e125] * other[e3]),
            ]) + (other.group1().yzxx() * self.group0().zxy().with_w(self[e235]))
                - (other.group7() * self.group1().www()).with_w(self[e5] * other[e321]),
            // e1234
            0.0,
        )
    }
}
impl Wedge<RoundPoint> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       20        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[e5]) * other.group0() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * other[e3]) - (self[e235] * other[e4]),
                -(self[e435] * other[e1]) - (self[e315] * other[e4]),
                -(self[e415] * other[e2]) - (self[e125] * other[e4]),
                (self[e315] * other[e2]) + (self[e125] * other[e3]),
            ]) + (other.group0().yzxx() * self.group0().zxy().with_w(self[e235])),
        )
    }
}
impl Wedge<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(other[scalar]) * self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl Wedge<Sphere> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e5] * other[e1234])
    }
}
impl Wedge<VersorEven> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       13        0
    //  no simd       12       24        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[e5]) * other.group3() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e235] * other[e4]) - (self[e5] * other[e423]),
                -(self[e315] * other[e4]) - (self[e5] * other[e431]),
                -(self[e125] * other[e4]) - (self[e5] * other[e412]),
                (self[e315] * other[e2]) + (self[e125] * other[e3]),
            ]) + (other.group3().yzxx() * self.group0().zxy().with_w(self[e235]))
                - (self.group0().yzx() * other.group3().zxy()).with_w(self[e5] * other[e321]),
        )
    }
}
impl Wedge<VersorOdd> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        8       11        0
    //  no simd       16       21        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x4::from(self[e5]) * other.group0().xyz().with_w(other[e1234]))
                + (Simd32x4::from(other[scalar]) * self.group0())
                + Simd32x3::from(0.0).with_w(
                    -(self[e415] * other[e23])
                        - (self[e425] * other[e31])
                        - (self[e435] * other[e12])
                        - (self[e235] * other[e41])
                        - (self[e315] * other[e42])
                        - (self[e125] * other[e43]),
                ),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[e5]) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * self.group1().xyz())).with_w(self[e5] * other[scalar]),
        )
    }
}
impl std::ops::Div<WedgeInfix> for MultiVector {
    type Output = WedgeInfixPartial<MultiVector>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       47        0
    //    simd3        8       15        0
    //    simd4        6        5        0
    // Totals...
    // yes simd       46       67        0
    //  no simd       80      112        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                other[scalar] * self[scalar],
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group1(),
            // e5
            other[scalar] * self[e5],
            // e15, e25, e35, e45
            (Simd32x4::from(other[scalar]) * self.group3()) + (Simd32x4::from(self[scalar]) * other.group2().xyz().with_w(other[e45])),
            // e41, e42, e43
            (Simd32x3::from(other[scalar]) * self.group4()) + (Simd32x3::from(self[scalar]) * other.group0()),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group5()) + (Simd32x3::from(self[scalar]) * other.group1().xyz()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other[e15] * self[e4]) + (other[scalar] * self[e415]),
                (other[e25] * self[e4]) + (other[scalar] * self[e425]),
                (other[e35] * self[e4]) + (other[scalar] * self[e435]),
                -(other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]) + (Simd32x3::from(self[e5]) * other.group0()).with_w(other[scalar] * self[e321])
                - (other.group1().wwwx() * self.group1().xyzx()),
            // e423, e431, e412
            (Simd32x3::from(other[scalar]) * self.group7()) + (Simd32x3::from(self[e4]) * other.group1().xyz()) + (other.group0().yzx() * self.group1().zxy())
                - (other.group0().zxy() * self.group1().yzx()),
            // e235, e315, e125
            (Simd32x3::from(other[scalar]) * self.group8()) + (Simd32x3::from(self[e5]) * other.group1().xyz()) + (other.group2().zxy() * self.group1().yzx())
                - (other.group2().yzx() * self.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e23] * self[e45]) + (other[e45] * self[e23]) + (other[e35] * self[e42]) + (other[scalar] * self[e4235]),
                (other[e31] * self[e45]) + (other[e45] * self[e31]) + (other[e15] * self[e43]) + (other[scalar] * self[e4315]),
                (other[e12] * self[e45]) + (other[e45] * self[e12]) + (other[e25] * self[e41]) + (other[scalar] * self[e4125]),
                -(other[e23] * self[e15]) - (other[e31] * self[e25]) - (other[e12] * self[e35]) - (other[e35] * self[e12]),
            ]) + (other.group0().yzx() * self.group3().zxy()).with_w(other[scalar] * self[e3215])
                - (other.group2().yzxy() * self.group4().zxy().with_w(self[e31]))
                - (other.group0().zxy() * self.group3().yzx()).with_w(other[e15] * self[e23]),
            // e1234
            (other[scalar] * self[e1234])
                - (other[e41] * self[e23])
                - (other[e42] * self[e31])
                - (other[e43] * self[e12])
                - (other[e23] * self[e41])
                - (other[e31] * self[e42])
                - (other[e12] * self[e43]),
        )
    }
}
impl Wedge<AntiDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       37        0
    //    simd3        8       17        0
    //    simd4       10        8        0
    // Totals...
    // yes simd       43       62        0
    //  no simd       89      120        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group3().xyz().with_w(other[e4]),
            // e5
            other[e5] * self[scalar],
            // e15, e25, e35, e45
            (Simd32x4::from(other[e5]) * self.group1()) - (Simd32x4::from(self[e5]) * other.group3().xyz().with_w(other[e4])),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group3().xyz()) - (Simd32x3::from(other[e4]) * self.group1().xyz()),
            // e23, e31, e12
            (other.group3().zxy() * self.group1().yzx()) - (other.group3().yzx() * self.group1().zxy()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other[e4] * self[e15]) + (other[e5] * self[e41]),
                (other[e4] * self[e25]) + (other[e5] * self[e42]),
                (other[e4] * self[e35]) + (other[e5] * self[e43]),
                -(other[e2] * self[e31]) - (other[e3] * self[e12]),
            ]) + (Simd32x4::from(self[scalar]) * other.group1())
                - (other.group3().xyz() * self.group3().www()).with_w(other[e1] * self[e23]),
            // e423, e431, e412
            (Simd32x3::from(other[e4]) * self.group5()) + (Simd32x3::from(self[scalar]) * other.group0()) + (self.group4().yzx() * other.group3().zxy())
                - (self.group4().zxy() * other.group3().yzx()),
            // e235, e315, e125
            (Simd32x3::from(other[e5]) * self.group5()) + (Simd32x3::from(self[scalar]) * other.group2().xyz()) + (other.group3().yzx() * self.group3().zxy())
                - (other.group3().zxy() * self.group3().yzx()),
            // e4235, e4315, e4125, e3215
            (other.group3().yzxw() * self.group6().zxyw())
                + (other.group3().wwwx() * self.group7().with_w(self[e235]))
                + (other.group1().yzx() * self.group1().zxy()).with_w(other[e2] * self[e315])
                + (other.group2().xyz() * self.group1().www()).with_w(other[e3] * self[e125])
                - (Simd32x4::from(self[e5]) * other.group0().with_w(other[e321]))
                - (self.group1().yzxy() * other.group1().zxy().with_w(other[e315]))
                - (self.group8() * other.group2().www()).with_w(other[e235] * self[e1])
                - (other.group3().zxy() * self.group6().yzx()).with_w(other[e125] * self[e3]),
            // e1234
            (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4])
                - (other[e4] * self[e321])
                - (other[e1] * self[e423])
                - (other[e2] * self[e431])
                - (other[e3] * self[e412]),
        )
    }
}
impl Wedge<AntiDualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        7        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       15        0
    //  no simd        2       35        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * self[scalar], (other[e3215] * self[e4]) + (other[scalar] * self[e12345])]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group1(),
            // e5
            other[scalar] * self[e5],
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
            other.group0().yy().with_zw(other[scalar], (other[e3215] * self[scalar]) + (other[scalar] * self[e3215])) * self.group9().xyz().with_w(1.0),
            // e1234
            other[scalar] * self[e1234],
        )
    }
}
impl Wedge<AntiFlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       16        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, -(other[e235] * self[e41]) - (other[e315] * self[e42]) - (other[e125] * self[e43]) - (other[e321] * self[e45])]),
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
            Simd32x3::from(0.0).with_w(other[e321] * self[scalar]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[scalar]) * other.group0().xyz(),
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(self[e4]) * other.group0().xyz()).with_w(-(other[e235] * self[e1]) - (other[e315] * self[e2]) - (other[e125] * self[e3]) - (other[e321] * self[e5])),
            // e1234
            other[e321] * self[e4],
        )
    }
}
impl Wedge<AntiFlector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       25        0
    //    simd3        6       17        0
    //    simd4        6        1        0
    // Totals...
    // yes simd       26       43        0
    //  no simd       56       80        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e5] * self[e1234])
                    - (other[e235] * self[e41])
                    - (other[e315] * self[e42])
                    - (other[e125] * self[e43])
                    - (other[e321] * self[e45]),
            ]),
            // e1, e2, e3, e4
            (other.group1().xyz() * self.group0().xx().with_z(self[scalar])).with_w(0.0),
            // e5
            other[e5] * self[scalar],
            // e15, e25, e35, e45
            ((Simd32x3::from(other[e5]) * self.group1().xyz()) - (Simd32x3::from(self[e5]) * other.group1().xyz())).with_w(other[e5] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(self[e4]) * other.group1().xyz(),
            // e23, e31, e12
            (other.group1().zxy() * self.group1().yzx()) - (other.group1().yzx() * self.group1().zxy()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(-(other[e2] * self[e31]) - (other[e3] * self[e12])) + (self.group4() * other.group1().www()).with_w(other[e321] * self[scalar])
                - (other.group1().xyz() * self.group3().www()).with_w(other[e1] * self[e23]),
            // e423, e431, e412
            (self.group4().yzx() * other.group1().zxy()) - (self.group4().zxy() * other.group1().yzx()),
            // e235, e315, e125
            (Simd32x3::from(other[e5]) * self.group5()) + (Simd32x3::from(self[scalar]) * other.group0().xyz()) + (other.group1().yzx() * self.group3().zxy())
                - (other.group1().zxy() * self.group3().yzx()),
            // e4235, e4315, e4125, e3215
            (other.group1().wwwx() * self.group7().with_w(self[e235]))
                + Simd32x3::from(0.0).with_w((other[e5] * self[e321]) - (other[e315] * self[e2]) - (other[e125] * self[e3]) - (other[e321] * self[e5]))
                + (other.group1().yzx() * self.group6().zxy()).with_w(other[e3] * self[e125])
                + (other.group0().xyz() * self.group1().www()).with_w(other[e2] * self[e315])
                - (other.group1().zxy() * self.group6().yzx()).with_w(other[e235] * self[e1]),
            // e1234
            (other[e321] * self[e4]) - (other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]),
        )
    }
}
impl Wedge<AntiLine> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd3        2        8        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       19       32        0
    //  no simd       26       48        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                -(other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435])
                    - (other[e15] * self[e423])
                    - (other[e25] * self[e431])
                    - (other[e35] * self[e412]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            (other.group1() * self.group0().xx().with_z(self[scalar])).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group0(),
            // e415, e425, e435, e321
            (Simd32x3::from(self[e4]) * other.group1()).with_w(-(other[e23] * self[e1]) - (other[e31] * self[e2]) - (other[e12] * self[e3])),
            // e423, e431, e412
            Simd32x3::from(self[e4]) * other.group0(),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * other.group0()) + (other.group1().zxy() * self.group1().yzx()) - (other.group1().yzx() * self.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e23] * self[e45]) + (other[e35] * self[e42]),
                (other[e31] * self[e45]) + (other[e15] * self[e43]),
                (other[e12] * self[e45]) + (other[e25] * self[e41]),
                -(other[e31] * self[e25]) - (other[e12] * self[e35]) - (other[e15] * self[e23]) - (other[e25] * self[e31]) - (other[e35] * self[e12]),
            ]) - (other.group1().yzx() * self.group4().zxy()).with_w(other[e23] * self[e15]),
            // e1234
            -(other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
        )
    }
}
impl Wedge<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       29        0
    //    simd3        6       11        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       26       45        0
    //  no simd       50       82        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                other[scalar] * self[scalar],
                (other[scalar] * self[e12345]) + (other[e3215] * self[e4])
                    - (other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435])
                    - (other[e15] * self[e423])
                    - (other[e25] * self[e431])
                    - (other[e35] * self[e412]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group1(),
            // e5
            other[scalar] * self[e5],
            // e15, e25, e35, e45
            ((Simd32x3::from(other[scalar]) * self.group3().xyz()) + (Simd32x3::from(self[scalar]) * other.group1().xyz())).with_w(other[scalar] * self[e45]),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group4(),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group5()) + (Simd32x3::from(self[scalar]) * other.group0().xyz()),
            // e415, e425, e435, e321
            Simd32x4::from([
                other[e15] * self[e4],
                other[e25] * self[e4],
                other[e35] * self[e4],
                -(other[e23] * self[e1]) - (other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]) + (Simd32x4::from(other[scalar]) * self.group6()),
            // e423, e431, e412
            (Simd32x3::from(other[scalar]) * self.group7()) + (Simd32x3::from(self[e4]) * other.group0().xyz()),
            // e235, e315, e125
            (Simd32x3::from(other[scalar]) * self.group8()) + (Simd32x3::from(self[e5]) * other.group0().xyz()) + (other.group1().zxy() * self.group1().yzx())
                - (other.group1().yzx() * self.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other[scalar] * self[e4235],
                other[scalar] * self[e4315],
                other[scalar] * self[e4125],
                -(other[e23] * self[e15]) - (other[e31] * self[e25]) - (other[e12] * self[e35]) - (other[e25] * self[e31]) - (other[e35] * self[e12]),
            ]) + (other.group0() * self.group3().www().with_w(self[e3215]))
                + (other.group1().zxyw() * self.group4().yzx().with_w(self[scalar]))
                - (other.group1().yzxx() * self.group4().zxy().with_w(self[e23])),
            // e1234
            (other[scalar] * self[e1234]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
        )
    }
}
impl Wedge<AntiPlane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       24        0
    //    simd3        5       13        0
    //    simd4        3        1        0
    // Totals...
    // yes simd       15       38        0
    //  no simd       34       67        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e5] * self[e1234])]),
            // e1, e2, e3, e4
            (other.group0().xyz() * self.group0().xx().with_z(self[scalar])).with_w(0.0),
            // e5
            other[e5] * self[scalar],
            // e15, e25, e35, e45
            ((Simd32x3::from(other[e5]) * self.group1().xyz()) - (Simd32x3::from(self[e5]) * other.group0().xyz())).with_w(other[e5] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(self[e4]) * other.group0().xyz(),
            // e23, e31, e12
            (other.group0().zxy() * self.group1().yzx()) - (other.group0().yzx() * self.group1().zxy()),
            // e415, e425, e435, e321
            Simd32x4::from([other[e5] * self[e41], other[e5] * self[e42], other[e5] * self[e43], -(other[e2] * self[e31]) - (other[e3] * self[e12])])
                - (other.group0().xyz() * self.group3().www()).with_w(other[e1] * self[e23]),
            // e423, e431, e412
            (self.group4().yzx() * other.group0().zxy()) - (self.group4().zxy() * other.group0().yzx()),
            // e235, e315, e125
            (Simd32x3::from(other[e5]) * self.group5()) + (other.group0().yzx() * self.group3().zxy()) - (other.group0().zxy() * self.group3().yzx()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other[e3] * self[e425] * -1.0,
                other[e1] * self[e435] * -1.0,
                other[e2] * self[e415] * -1.0,
                (other[e3] * self[e125]) + (other[e5] * self[e321]),
            ]) + (other.group0().wwwx() * self.group7().with_w(self[e235]))
                + (other.group0().yzx() * self.group6().zxy()).with_w(other[e2] * self[e315]),
            // e1234
            -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]),
        )
    }
}
impl Wedge<AntiScalar> for MultiVector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[e12345] * self[scalar])
    }
}
impl Wedge<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       23        0
    //    simd3        0        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       18       28        0
    //  no simd       24       40        0
    fn wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
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
            Simd32x4::from(self[scalar]) * other.group1(),
            // e423, e431, e412
            Simd32x3::from(self[scalar]) * other.group0(),
            // e235, e315, e125
            Simd32x3::from(self[scalar]) * other.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e425] * self[e3]) + (other[e235] * self[e4]),
                (other[e435] * self[e1]) + (other[e315] * self[e4]),
                (other[e415] * self[e2]) + (other[e125] * self[e4]),
                -(other[e321] * self[e5]) - (other[e125] * self[e3]),
            ]) - (self.group1().yzxy() * other.group1().zxy().with_w(other[e315]))
                - (Simd32x3::from(self[e5]) * other.group0()).with_w(other[e235] * self[e1]),
            // e1234
            (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]),
        )
    }
}
impl Wedge<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       23        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       25       41        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
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
            Simd32x4::from(self[scalar]) * other.group1(),
            // e423, e431, e412
            Simd32x3::from(self[scalar]) * other.group0(),
            // e235, e315, e125
            Simd32x3::from(self[scalar]) * other.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e425] * self[e3]) + (other[e235] * self[e4]),
                (other[e435] * self[e1]) + (other[e315] * self[e4]),
                (other[e415] * self[e2]) + (other[e125] * self[e4]),
                -(other[e315] * self[e2]) - (other[e125] * self[e3]),
            ]) - (Simd32x4::from(self[e5]) * other.group0().with_w(other[e321]))
                - (self.group1().yzxx() * other.group1().zxy().with_w(other[e235])),
            // e1234
            (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]),
        )
    }
}
impl Wedge<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       42        0
    //    simd3        4       10        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       37       54        0
    //  no simd       54       80        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * other.group2().with_w(other[e45]),
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other[e41] * self[e5]) + (other[e15] * self[e4]),
                (other[e42] * self[e5]) + (other[e25] * self[e4]),
                (other[e43] * self[e5]) + (other[e35] * self[e4]),
                -(other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]) - (other.group1().wwwx() * self.group1().xyzx()),
            // e423, e431, e412
            (Simd32x3::from(self[e4]) * other.group1().xyz()) + (other.group0().yzx() * self.group1().zxy()) - (other.group0().zxy() * self.group1().yzx()),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * other.group1().xyz()) + (other.group2().zxy() * self.group1().yzx()) - (other.group2().yzx() * self.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e42] * self[e35]) + (other[e23] * self[e45]) + (other[e45] * self[e23]) + (other[e35] * self[e42]),
                (other[e43] * self[e15]) + (other[e31] * self[e45]) + (other[e45] * self[e31]) + (other[e15] * self[e43]),
                (other[e41] * self[e25]) + (other[e12] * self[e45]) + (other[e45] * self[e12]) + (other[e25] * self[e41]),
                -(other[e23] * self[e15]) - (other[e31] * self[e25]) - (other[e12] * self[e35]) - (other[e35] * self[e12]),
            ]) - (other.group0().zxy() * self.group3().yzx()).with_w(other[e15] * self[e23])
                - (other.group2().yzx() * self.group4().zxy()).with_w(other[e25] * self[e31]),
            // e1234
            -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
        )
    }
}
impl Wedge<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       47        0
    //    simd3        4        9        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       44       60        0
    //  no simd       64       90        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * other.group2().xyz().with_w(other[e45]),
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other[e41] * self[e5]) + (other[e15] * self[e4]),
                (other[e42] * self[e5]) + (other[e25] * self[e4]),
                (other[e43] * self[e5]) + (other[e35] * self[e4]),
                -(other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]) - (other.group1().wwwx() * self.group1().xyzx()),
            // e423, e431, e412
            (Simd32x3::from(self[e4]) * other.group1().xyz()) + (other.group0().yzx() * self.group1().zxy()) - (other.group0().zxy() * self.group1().yzx()),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * other.group1().xyz()) + (other.group2().zxy() * self.group1().yzx()) - (other.group2().yzx() * self.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e42] * self[e35]) + (other[e23] * self[e45]) + (other[e45] * self[e23]) + (other[e35] * self[e42]),
                (other[e43] * self[e15]) + (other[e31] * self[e45]) + (other[e45] * self[e31]) + (other[e15] * self[e43]),
                (other[e41] * self[e25]) + (other[e12] * self[e45]) + (other[e45] * self[e12]) + (other[e25] * self[e41]),
                -(other[e23] * self[e15]) - (other[e31] * self[e25]) - (other[e12] * self[e35]) - (other[e35] * self[e12]),
            ]) + (Simd32x4::from(self[scalar]) * other.group3())
                - (other.group2().yzxy() * self.group4().zxy().with_w(self[e31]))
                - (other.group0().zxy() * self.group3().yzx()).with_w(other[e15] * self[e23]),
            // e1234
            (other[e1234] * self[scalar])
                - (other[e41] * self[e23])
                - (other[e42] * self[e31])
                - (other[e43] * self[e12])
                - (other[e23] * self[e41])
                - (other[e31] * self[e42])
                - (other[e12] * self[e43]),
        )
    }
}
impl Wedge<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        7        0
    //  no simd        1       17        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other[e5] * self[e1234]) + (other[e12345] * self[scalar])]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5] * self[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(other[e5]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (self.group4() * other.group0().xx().with_z(other[e5])).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(other[e5]) * self.group5(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e5]) * self.group7().with_w(self[e321]),
            // e1234
            0.0,
        )
    }
}
impl Wedge<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd3        2        4        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       17       32        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, -(other[e15] * self[e423]) - (other[e25] * self[e431]) - (other[e35] * self[e412]) - (other[e45] * self[e321])]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * other.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e4]) * other.group0().xyz()) - (Simd32x3::from(other[e45]) * self.group1().xyz())).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group0().zxy() * self.group1().yzx()) - (other.group0().yzx() * self.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e35] * self[e42]) + (other[e45] * self[e23]),
                (other[e15] * self[e43]) + (other[e45] * self[e31]),
                (other[e25] * self[e41]) + (other[e45] * self[e12]),
                -(other[e25] * self[e31]) - (other[e35] * self[e12]),
            ]) - (other.group0().yzxx() * self.group4().zxy().with_w(self[e23])),
            // e1234
            0.0,
        )
    }
}
impl Wedge<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       16        0
    //    simd3        2        4        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       15       23        0
    //  no simd       25       40        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (other[e4235] * self[e1]) + (other[e4315] * self[e2]) + (other[e4125] * self[e3]) + (other[e3215] * self[e4])
                    - (other[e15] * self[e423])
                    - (other[e25] * self[e431])
                    - (other[e35] * self[e412])
                    - (other[e45] * self[e321]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * other.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e4]) * other.group0().xyz()) - (Simd32x3::from(other[e45]) * self.group1().xyz())).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group0().zxy() * self.group1().yzx()) - (other.group0().yzx() * self.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e35] * self[e42]) + (other[e45] * self[e23]),
                (other[e15] * self[e43]) + (other[e45] * self[e31]),
                (other[e25] * self[e41]) + (other[e45] * self[e12]),
                -(other[e25] * self[e31]) - (other[e35] * self[e12]),
            ]) + (Simd32x4::from(self[scalar]) * other.group1())
                - (other.group0().yzxx() * self.group4().zxy().with_w(self[e23])),
            // e1234
            0.0,
        )
    }
}
impl Wedge<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       14        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       17        0
    //  no simd       13       24        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                -(other[e415] * self[e23])
                    - (other[e425] * self[e31])
                    - (other[e435] * self[e12])
                    - (other[e235] * self[e41])
                    - (other[e315] * self[e42])
                    - (other[e125] * self[e43]),
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
            (other.group0() * self.group0().xx().with_z(self[scalar])).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[scalar]) * other.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e425] * self[e3]) + (other[e235] * self[e4]),
                (other[e435] * self[e1]) + (other[e315] * self[e4]),
                (other[e415] * self[e2]) + (other[e125] * self[e4]),
                -(other[e315] * self[e2]) - (other[e125] * self[e3]),
            ]) - (self.group1().yzxx() * other.group0().zxy().with_w(other[e235])),
            // e1234
            0.0,
        )
    }
}
impl Wedge<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       18        0
    //    simd3        2        5        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       15       25        0
    //  no simd       25       41        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (other[e12345] * self[scalar]) + (other[e5] * self[e1234])
                    - (other[e415] * self[e23])
                    - (other[e425] * self[e31])
                    - (other[e435] * self[e12])
                    - (other[e235] * self[e41])
                    - (other[e315] * self[e42])
                    - (other[e125] * self[e43]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5] * self[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(other[e5]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e5]) * self.group4()) + (Simd32x3::from(self[scalar]) * other.group0().xyz())).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from(other[e5]) * self.group5()) + (Simd32x3::from(self[scalar]) * other.group1().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e425] * self[e3]) + (other[e235] * self[e4]),
                (other[e435] * self[e1]) + (other[e315] * self[e4]),
                (other[e415] * self[e2]) + (other[e125] * self[e4]),
                -(other[e315] * self[e2]) - (other[e125] * self[e3]),
            ]) + (self.group7() * other.group1().www()).with_w(other[e5] * self[e321])
                - (self.group1().yzxx() * other.group0().zxy().with_w(other[e235])),
            // e1234
            0.0,
        )
    }
}
impl Wedge<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       71       92        0
    //    simd3       20       33        0
    //    simd4       20       13        0
    // Totals...
    // yes simd      111      138        0
    //  no simd      211      243        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                other[scalar] * self[scalar],
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
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(other[scalar]) * self.group1()) + (Simd32x4::from(self[scalar]) * other.group1()),
            // e5
            (other[scalar] * self[e5]) + (other[e5] * self[scalar]),
            // e15, e25, e35, e45
            (Simd32x4::from(other[scalar]) * self.group3()) + (Simd32x4::from(other[e5]) * self.group1()) + (Simd32x4::from(self[scalar]) * other.group3())
                - (Simd32x4::from(self[e5]) * other.group1()),
            // e41, e42, e43
            (Simd32x3::from(other[scalar]) * self.group4()) + (Simd32x3::from(self[scalar]) * other.group4()) + (Simd32x3::from(self[e4]) * other.group1().xyz())
                - (Simd32x3::from(other[e4]) * self.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group5()) + (Simd32x3::from(self[scalar]) * other.group5()) + (other.group1().zxy() * self.group1().yzx())
                - (other.group1().yzx() * self.group1().zxy()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other[e4] * self[e15]) + (other[e5] * self[e41]) + (other[e15] * self[e4]) + (other[e41] * self[e5]),
                (other[e4] * self[e25]) + (other[e5] * self[e42]) + (other[e25] * self[e4]) + (other[e42] * self[e5]),
                (other[e4] * self[e35]) + (other[e5] * self[e43]) + (other[e35] * self[e4]) + (other[e43] * self[e5]),
                -(other[e1] * self[e23]) - (other[e2] * self[e31]) - (other[e3] * self[e12]) - (other[e12] * self[e3]),
            ]) + (Simd32x4::from(other[scalar]) * self.group6())
                + (Simd32x4::from(self[scalar]) * other.group6())
                - (self.group1().xyzy() * other.group3().www().with_w(other[e31]))
                - (other.group1().xyz() * self.group3().www()).with_w(other[e23] * self[e1]),
            // e423, e431, e412
            (Simd32x3::from(other[scalar]) * self.group7())
                + (Simd32x3::from(other[e4]) * self.group5())
                + (Simd32x3::from(self[scalar]) * other.group7())
                + (Simd32x3::from(self[e4]) * other.group5())
                + (other.group4().yzx() * self.group1().zxy())
                + (self.group4().yzx() * other.group1().zxy())
                - (other.group4().zxy() * self.group1().yzx())
                - (self.group4().zxy() * other.group1().yzx()),
            // e235, e315, e125
            (Simd32x3::from(other[scalar]) * self.group8())
                + (Simd32x3::from(other[e5]) * self.group5())
                + (Simd32x3::from(self[scalar]) * other.group8())
                + (Simd32x3::from(self[e5]) * other.group5())
                + (other.group1().yzx() * self.group3().zxy())
                + (other.group3().zxy() * self.group1().yzx())
                - (other.group1().zxy() * self.group3().yzx())
                - (other.group3().yzx() * self.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e2] * self[e435]) + (other[e5] * self[e423]) + (other[e45] * self[e23]) + (other[e425] * self[e3]),
                (other[e3] * self[e415]) + (other[e5] * self[e431]) + (other[e45] * self[e31]) + (other[e435] * self[e1]),
                (other[e1] * self[e425]) + (other[e5] * self[e412]) + (other[e45] * self[e12]) + (other[e415] * self[e2]),
                -(other[e15] * self[e23]) - (other[e25] * self[e31]) - (other[e35] * self[e12]) - (other[e321] * self[e5]),
            ]) + (Simd32x4::from(other[scalar]) * self.group9())
                + (Simd32x4::from(self[scalar]) * other.group9())
                + (other.group5() * self.group3().www()).with_w(other[e2] * self[e315])
                + (other.group8() * self.group1().www()).with_w(other[e3] * self[e125])
                + (other.group4().yzx() * self.group3().zxy()).with_w(other[e1] * self[e235])
                + (self.group4().yzx() * other.group3().zxy()).with_w(other[e5] * self[e321])
                - (self.group1().yzxz() * other.group6().zxy().with_w(other[e125]))
                - (self.group3().yzxx() * other.group4().zxy().with_w(other[e23]))
                - (Simd32x3::from(self[e5]) * other.group7()).with_w(other[e31] * self[e25])
                - (self.group8() * other.group1().www()).with_w(other[e235] * self[e1])
                - (self.group4().zxy() * other.group3().yzx()).with_w(other[e12] * self[e35])
                - (other.group1().zxy() * self.group6().yzx()).with_w(other[e315] * self[e2]),
            // e1234
            (other[scalar] * self[e1234])
                + (other[e321] * self[e4])
                + (other[e423] * self[e1])
                + (other[e431] * self[e2])
                + (other[e412] * self[e3])
                + (other[e1234] * self[scalar])
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
        )
    }
}
impl Wedge<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e4] * other[e3215])]),
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
            Simd32x4::from(self[scalar]) * other.group0(),
            // e1234
            0.0,
        )
    }
}
impl Wedge<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       27        0
    //    simd3        6       11        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       25       43        0
    //  no simd       49       80        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (self[e4235] * other[e1]) + (self[e4315] * other[e2]) + (self[e4125] * other[e3]) + (self[e3215] * other[e4]) + (self[e1234] * other[e5]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group0(),
            // e5
            self[scalar] * other[e5],
            // e15, e25, e35, e45
            (Simd32x4::from(other[e5]) * self.group1()) - (Simd32x4::from(self[e5]) * other.group0()),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group0().xyz()) - (Simd32x3::from(other[e4]) * self.group1().xyz()),
            // e23, e31, e12
            (self.group1().yzx() * other.group0().zxy()) - (self.group1().zxy() * other.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e15] * other[e4]) + (self[e41] * other[e5]),
                (self[e25] * other[e4]) + (self[e42] * other[e5]),
                (self[e35] * other[e4]) + (self[e43] * other[e5]),
                -(self[e31] * other[e2]) - (self[e12] * other[e3]),
            ]) - (other.group0().xyzx() * self.group3().www().with_w(self[e23])),
            // e423, e431, e412
            (Simd32x3::from(other[e4]) * self.group5()) + (self.group4().yzx() * other.group0().zxy()) - (self.group4().zxy() * other.group0().yzx()),
            // e235, e315, e125
            (Simd32x3::from(other[e5]) * self.group5()) + (self.group3().zxy() * other.group0().yzx()) - (self.group3().yzx() * other.group0().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * other[e3]) - (self[e235] * other[e4]),
                -(self[e435] * other[e1]) - (self[e315] * other[e4]),
                -(self[e415] * other[e2]) - (self[e125] * other[e4]),
                (self[e321] * other[e5]) + (self[e125] * other[e3]),
            ]) + (other.group0().yzxy() * self.group6().zxy().with_w(self[e315]))
                + (Simd32x3::from(other[e5]) * self.group7()).with_w(self[e235] * other[e1]),
            // e1234
            -(self[e321] * other[e4]) - (self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]),
        )
    }
}
impl Wedge<Scalar> for MultiVector {
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
    fn wedge(self, other: Scalar) -> Self::Output {
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
impl Wedge<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        4       10        0
    fn wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e4] * other[e3215]) + (self[e5] * other[e1234]),
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
            Simd32x4::from(self[scalar]) * other.group0(),
            // e1234
            self[scalar] * other[e1234],
        )
    }
}
impl Wedge<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       38        0
    //    simd3        8       17        0
    //    simd4       10        8        0
    // Totals...
    // yes simd       44       63        0
    //  no simd       90      121        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group3(),
            // e5
            self[scalar] * other[e5],
            // e15, e25, e35, e45
            (Simd32x4::from(other[e5]) * self.group1()) - (Simd32x4::from(self[e5]) * other.group3()),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group3().xyz()) - (Simd32x3::from(other[e4]) * self.group1().xyz()),
            // e23, e31, e12
            (self.group1().yzx() * other.group3().zxy()) - (self.group1().zxy() * other.group3().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e15] * other[e4]) + (self[e41] * other[e5]),
                (self[e25] * other[e4]) + (self[e42] * other[e5]),
                (self[e35] * other[e4]) + (self[e43] * other[e5]),
                -(self[e31] * other[e2]) - (self[e12] * other[e3]),
            ]) + (Simd32x4::from(self[scalar]) * other.group1())
                - (other.group3().xyzx() * self.group3().www().with_w(self[e23])),
            // e423, e431, e412
            (Simd32x3::from(self[scalar]) * other.group0().xyz()) + (Simd32x3::from(other[e4]) * self.group5()) + (self.group4().yzx() * other.group3().zxy())
                - (self.group4().zxy() * other.group3().yzx()),
            // e235, e315, e125
            (Simd32x3::from(self[scalar]) * other.group2().xyz()) + (Simd32x3::from(other[e5]) * self.group5()) + (self.group3().zxy() * other.group3().yzx())
                - (self.group3().yzx() * other.group3().zxy()),
            // e4235, e4315, e4125, e3215
            (self.group6().zxyw() * other.group3().yzx().with_w(other[e5]))
                + (self.group7() * other.group2().www()).with_w(self[e235] * other[e1])
                + (self.group1().zxy() * other.group1().yzx()).with_w(self[e315] * other[e2])
                + (other.group2().xyz() * self.group1().www()).with_w(self[e125] * other[e3])
                - (Simd32x4::from(self[e5]) * other.group0().xyz().with_w(other[e321]))
                - (self.group1().yzxy() * other.group1().zxy().with_w(other[e315]))
                - (self.group8() * other.group3().www()).with_w(self[e1] * other[e235])
                - (self.group6().yzx() * other.group3().zxy()).with_w(self[e3] * other[e125]),
            // e1234
            (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321])
                - (self[e321] * other[e4])
                - (self[e423] * other[e1])
                - (self[e431] * other[e2])
                - (self[e412] * other[e3]),
        )
    }
}
impl Wedge<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       53        0
    //    simd3        8       15        0
    //    simd4        7        6        0
    // Totals...
    // yes simd       53       74        0
    //  no simd       90      122        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[scalar] * other[scalar],
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group1(),
            // e5
            self[e5] * other[scalar],
            // e15, e25, e35, e45
            (Simd32x4::from(self[scalar]) * other.group2().xyz().with_w(other[e45])) + (Simd32x4::from(other[scalar]) * self.group3()),
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * other.group0().xyz()) + (Simd32x3::from(other[scalar]) * self.group4()),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * self.group5()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e5] * other[e41]) + (self[e415] * other[scalar]),
                (self[e5] * other[e42]) + (self[e425] * other[scalar]),
                (self[e5] * other[e43]) + (self[e435] * other[scalar]),
                -(self[e2] * other[e31]) - (self[e3] * other[e12]),
            ]) + (other.group2().xyz() * self.group1().www()).with_w(self[e321] * other[scalar])
                - (self.group1().xyzx() * other.group1().wwwx()),
            // e423, e431, e412
            (Simd32x3::from(self[e4]) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * self.group7()) + (self.group1().zxy() * other.group0().yzx())
                - (self.group1().yzx() * other.group0().zxy()),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * self.group8()) + (self.group1().yzx() * other.group2().zxy())
                - (self.group1().zxy() * other.group2().yzx()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e35] * other[e42]) + (self[e45] * other[e23]) + (self[e23] * other[e45]) + (self[e4235] * other[scalar]),
                (self[e15] * other[e43]) + (self[e45] * other[e31]) + (self[e31] * other[e45]) + (self[e4315] * other[scalar]),
                (self[e25] * other[e41]) + (self[e45] * other[e12]) + (self[e12] * other[e45]) + (self[e4125] * other[scalar]),
                -(self[e15] * other[e23]) - (self[e25] * other[e31]) - (self[e35] * other[e12]) - (self[e12] * other[e35]),
            ]) + (Simd32x4::from(self[scalar]) * other.group3())
                + (self.group4().yzx() * other.group2().zxy()).with_w(self[e3215] * other[scalar])
                - (other.group2().yzxx() * self.group4().zxy().with_w(self[e23]))
                - (self.group3().yzx() * other.group0().zxy()).with_w(self[e31] * other[e25]),
            // e1234
            (self[scalar] * other[e1234]) + (self[e1234] * other[scalar])
                - (self[e41] * other[e23])
                - (self[e42] * other[e31])
                - (self[e43] * other[e12])
                - (self[e23] * other[e41])
                - (self[e31] * other[e42])
                - (self[e12] * other[e43]),
        )
    }
}
impl std::ops::Div<WedgeInfix> for Plane {
    type Output = WedgeInfixPartial<Plane>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl Wedge<AntiDipoleInversion> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (other[e4] * self[e3215]) + (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]),
        )
    }
}
impl Wedge<AntiDualNum> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl Wedge<AntiFlector> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]))
    }
}
impl Wedge<AntiMotor> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl Wedge<AntiPlane> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]))
    }
}
impl Wedge<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e4] * self[e3215])]),
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
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1234
            0.0,
        )
    }
}
impl Wedge<RoundPoint> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (self[e4235] * other[e1]) + (self[e4315] * other[e2]) + (self[e4125] * other[e3]) + (self[e3215] * other[e4]),
        )
    }
}
impl Wedge<Scalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl Wedge<VersorEven> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (self[e4235] * other[e1]) + (self[e4315] * other[e2]) + (self[e4125] * other[e3]) + (self[e3215] * other[e4]),
        )
    }
}
impl Wedge<VersorOdd> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl std::ops::Div<WedgeInfix> for RoundPoint {
    type Output = WedgeInfixPartial<RoundPoint>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for RoundPoint {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        4        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       20       35        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e4]) * other.group1().xyz()) + (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other[e41] * self[e5]) + (other[e15] * self[e4]),
                (other[e42] * self[e5]) + (other[e25] * self[e4]),
                (other[e43] * self[e5]) + (other[e35] * self[e4]),
                -(other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]) - (other.group1().wwwx() * self.group0().xyzx()),
            // e235, e315, e125, e4
            ((Simd32x3::from(self[e5]) * other.group1().xyz()) + (other.group2().zxy() * self.group0().yzx()) - (other.group2().yzx() * self.group0().zxy()))
                .with_w(other[scalar] * self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from(other[scalar]) * self.group0().xyz().with_w(self[e5]),
        )
    }
}
impl Wedge<AntiDipoleInversion> for RoundPoint {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       18        0
    //    simd3        1        3        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       11       25        0
    //  no simd       25       43        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group3().xyz()) - (Simd32x3::from(other[e4]) * self.group0().xyz()),
            // e23, e31, e12, e45
            (other.group3().zxyw() * self.group0().yzxw()) - (other.group3().yzx() * self.group0().zxy()).with_w(other[e4] * self[e5]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                other[e1] * self[e5] * -1.0,
                other[e2] * self[e5] * -1.0,
                other[e3] * self[e5] * -1.0,
                (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]),
            ]) + (self.group0().xyzx() * other.group3().www().with_w(other[e423])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e425] * self[e3]) + (other[e235] * self[e4]),
                (other[e435] * self[e1]) + (other[e315] * self[e4]),
                (other[e415] * self[e2]) + (other[e125] * self[e4]),
                -(other[e315] * self[e2]) - (other[e125] * self[e3]),
            ]) - (Simd32x4::from(self[e5]) * other.group0().with_w(other[e321]))
                - (self.group0().yzxx() * other.group1().zxy().with_w(other[e235])),
        )
    }
}
impl Wedge<AntiDualNum> for RoundPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(other[e3215] * self[e4]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(other[scalar] * self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group0(),
        )
    }
}
impl Wedge<AntiFlatPoint> for RoundPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3        8        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(self[e4]) * other.group0().xyz()).with_w(-(other[e235] * self[e1]) - (other[e315] * self[e2]) - (other[e125] * self[e3]) - (other[e321] * self[e5])),
            // e1234
            other[e321] * self[e4],
        )
    }
}
impl Wedge<AntiFlector> for RoundPoint {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        2        6        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        9       24        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e4]) * other.group1().xyz(),
            // e23, e31, e12, e45
            ((other.group1().zxy() * self.group0().yzx()) - (other.group1().yzx() * self.group0().zxy())).with_w(other[e5] * self[e4]),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e5]) * self.group0().xyz()) - (Simd32x3::from(self[e5]) * other.group1().xyz())).with_w(other[e321] * self[e4]),
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(self[e4]) * other.group0().xyz()).with_w(-(other[e235] * self[e1]) - (other[e315] * self[e2]) - (other[e125] * self[e3]) - (other[e321] * self[e5])),
        )
    }
}
impl Wedge<AntiLine> for RoundPoint {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        2        5        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        8       18        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e4]) * other.group0(),
            // e415, e425, e435, e321
            (Simd32x3::from(self[e4]) * other.group1()).with_w(-(other[e23] * self[e1]) - (other[e31] * self[e2]) - (other[e12] * self[e3])),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * other.group0()) + (other.group1().zxy() * self.group0().yzx()) - (other.group1().yzx() * self.group0().zxy()),
        )
    }
}
impl Wedge<AntiMotor> for RoundPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        2        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        8       24        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self[e4]) * other.group0().xyz().with_w(other[e3215]),
            // e415, e425, e435, e321
            (Simd32x3::from(self[e4]) * other.group1().xyz()).with_w(-(other[e23] * self[e1]) - (other[e31] * self[e2]) - (other[e12] * self[e3])),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[e5]) * other.group0().xyz()) + (other.group1().zxy() * self.group0().yzx()) - (other.group1().yzx() * self.group0().zxy()))
                .with_w(other[scalar] * self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group0(),
        )
    }
}
impl Wedge<AntiPlane> for RoundPoint {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        2        5        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        6       16        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e4]) * other.group0().xyz(),
            // e23, e31, e12, e45
            ((other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy())).with_w(other[e5] * self[e4]),
            // e15, e25, e35
            (Simd32x3::from(other[e5]) * self.group0().xyz()) - (Simd32x3::from(self[e5]) * other.group0().xyz()),
        )
    }
}
impl Wedge<Circle> for RoundPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       20        0
    fn wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e425] * self[e3]) + (other[e235] * self[e4]),
                (other[e435] * self[e1]) + (other[e315] * self[e4]),
                (other[e415] * self[e2]) + (other[e125] * self[e4]),
                -(other[e321] * self[e5]) - (other[e125] * self[e3]),
            ]) - (self.group0().yzxy() * other.group1().zxy().with_w(other[e315]))
                - (Simd32x3::from(self[e5]) * other.group0()).with_w(other[e235] * self[e1]),
            // e1234
            (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]),
        )
    }
}
impl Wedge<CircleRotor> for RoundPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e425] * self[e3]) + (other[e235] * self[e4]),
                (other[e435] * self[e1]) + (other[e315] * self[e4]),
                (other[e415] * self[e2]) + (other[e125] * self[e4]),
                -(other[e315] * self[e2]) - (other[e125] * self[e3]),
            ]) - (Simd32x4::from(self[e5]) * other.group0().with_w(other[e321]))
                - (self.group0().yzxx() * other.group1().zxy().with_w(other[e235])),
            // e1234
            (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]),
        )
    }
}
impl Wedge<Dipole> for RoundPoint {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        4        6        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       20       30        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e4]) * other.group1().xyz()) + (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other[e41] * self[e5]) + (other[e15] * self[e4]),
                (other[e42] * self[e5]) + (other[e25] * self[e4]),
                (other[e43] * self[e5]) + (other[e35] * self[e4]),
                -(other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]) - (other.group1().wwwx() * self.group0().xyzx()),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * other.group1().xyz()) + (other.group2().zxy() * self.group0().yzx()) - (other.group2().yzx() * self.group0().zxy()),
        )
    }
}
impl Wedge<DipoleInversion> for RoundPoint {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        2        3        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       24       38        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e4]) * other.group1().xyz()) + (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other[e41] * self[e5]) + (other[e15] * self[e4]),
                (other[e42] * self[e5]) + (other[e25] * self[e4]),
                (other[e43] * self[e5]) + (other[e35] * self[e4]),
                -(other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]) - (other.group1().wwwx() * self.group0().xyzx()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                other[e25] * self[e3] * -1.0,
                other[e35] * self[e1] * -1.0,
                other[e15] * self[e2] * -1.0,
                (other[e4315] * self[e2]) + (other[e4125] * self[e3]) + (other[e3215] * self[e4]),
            ]) + (Simd32x4::from(self[e5]) * other.group1().xyz().with_w(other[e1234]))
                + (self.group0().yzxx() * other.group2().zxy().with_w(other[e4235])),
        )
    }
}
impl Wedge<DualNum> for RoundPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other[e5]) * self.group0())
    }
}
impl Wedge<FlatPoint> for RoundPoint {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        4        0
    // no simd        6       12        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            (Simd32x3::from(self[e4]) * other.group0().xyz()) - (Simd32x3::from(other[e45]) * self.group0().xyz()),
            // e235, e315, e125
            (other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy()),
        )
    }
}
impl Wedge<Flector> for RoundPoint {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        9       19        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                other[e45] * self[e1] * -1.0,
                other[e45] * self[e2] * -1.0,
                other[e45] * self[e3] * -1.0,
                (other[e4315] * self[e2]) + (other[e4125] * self[e3]) + (other[e3215] * self[e4]),
            ]) + (self.group0().wwwx() * other.group0().xyz().with_w(other[e4235])),
            // e235, e315, e125, e5
            ((other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl Wedge<Line> for RoundPoint {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e425] * self[e3]) + (other[e235] * self[e4]),
                (other[e435] * self[e1]) + (other[e315] * self[e4]),
                (other[e415] * self[e2]) + (other[e125] * self[e4]),
                -(other[e315] * self[e2]) - (other[e125] * self[e3]),
            ]) - (self.group0().yzxx() * other.group0().zxy().with_w(other[e235])),
        )
    }
}
impl Wedge<Motor> for RoundPoint {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[e5]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e425] * self[e3]) + (other[e235] * self[e4]),
                (other[e435] * self[e1]) + (other[e315] * self[e4]),
                (other[e415] * self[e2]) + (other[e125] * self[e4]),
                -(other[e315] * self[e2]) - (other[e125] * self[e3]),
            ]) - (self.group0().yzxx() * other.group0().zxy().with_w(other[e235])),
        )
    }
}
impl Wedge<MultiVector> for RoundPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       27        0
    //    simd3        6       11        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       25       43        0
    //  no simd       49       80        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (other[e4235] * self[e1]) + (other[e4315] * self[e2]) + (other[e4125] * self[e3]) + (other[e3215] * self[e4]) + (other[e1234] * self[e5]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group0(),
            // e5
            other[scalar] * self[e5],
            // e15, e25, e35, e45
            (Simd32x4::from(other[e5]) * self.group0()) - (Simd32x4::from(self[e5]) * other.group1()),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group1().xyz()) - (Simd32x3::from(other[e4]) * self.group0().xyz()),
            // e23, e31, e12
            (other.group1().zxy() * self.group0().yzx()) - (other.group1().yzx() * self.group0().zxy()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other[e15] * self[e4]) + (other[e41] * self[e5]),
                (other[e25] * self[e4]) + (other[e42] * self[e5]),
                (other[e35] * self[e4]) + (other[e43] * self[e5]),
                -(other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]) - (self.group0().xyzx() * other.group3().www().with_w(other[e23])),
            // e423, e431, e412
            (Simd32x3::from(self[e4]) * other.group5()) + (other.group4().yzx() * self.group0().zxy()) - (other.group4().zxy() * self.group0().yzx()),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * other.group5()) + (other.group3().zxy() * self.group0().yzx()) - (other.group3().yzx() * self.group0().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e425] * self[e3]) + (other[e235] * self[e4]),
                (other[e435] * self[e1]) + (other[e315] * self[e4]),
                (other[e415] * self[e2]) + (other[e125] * self[e4]),
                -(other[e321] * self[e5]) - (other[e125] * self[e3]),
            ]) - (self.group0().yzxy() * other.group6().zxy().with_w(other[e315]))
                - (Simd32x3::from(self[e5]) * other.group7()).with_w(other[e235] * self[e1]),
            // e1234
            (other[e321] * self[e4]) + (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]),
        )
    }
}
impl Wedge<Plane> for RoundPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (other[e4235] * self[e1]) + (other[e4315] * self[e2]) + (other[e4125] * self[e3]) + (other[e3215] * self[e4]),
        )
    }
}
impl Wedge<RoundPoint> for RoundPoint {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        2        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3        7        0
    //  no simd       10       20        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group0().xyz()) - (Simd32x3::from(other[e4]) * self.group0().xyz()),
            // e23, e31, e12, e45
            (other.group0().zxy() * self.group0().yzx()).with_w(other[e5] * self[e4]) - (other.group0().yzxw() * self.group0().zxy().with_w(self[e5])),
            // e15, e25, e35
            (Simd32x3::from(other[e5]) * self.group0().xyz()) - (Simd32x3::from(self[e5]) * other.group0().xyz()),
        )
    }
}
impl Wedge<Scalar> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar]) * self.group0(), /* e5 */ self[e5] * other[scalar])
    }
}
impl Wedge<Sphere> for RoundPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (self[e1] * other[e4235]) + (self[e2] * other[e4315]) + (self[e3] * other[e4125]) + (self[e4] * other[e3215]) + (self[e5] * other[e1234]),
        )
    }
}
impl Wedge<VersorEven> for RoundPoint {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       19        0
    //    simd3        1        4        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       11       26        0
    //  no simd       25       43        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group3().xyz()) - (Simd32x3::from(other[e4]) * self.group0().xyz()),
            // e23, e31, e12, e45
            (self.group0().yzxw() * other.group3().zxy().with_w(other[e5])) - (self.group0().zxy() * other.group3().yzx()).with_w(self[e5] * other[e4]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self[e5] * other[e1] * -1.0,
                self[e5] * other[e2] * -1.0,
                self[e5] * other[e3] * -1.0,
                (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321]),
            ]) + (self.group0().xyzx() * other.group2().www().with_w(other[e423])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e3] * other[e425]) + (self[e4] * other[e235]),
                (self[e1] * other[e435]) + (self[e4] * other[e315]),
                (self[e2] * other[e415]) + (self[e4] * other[e125]),
                -(self[e3] * other[e125]) - (self[e5] * other[e321]),
            ]) - (self.group0().yzxx() * other.group1().zxy().with_w(other[e235]))
                - (Simd32x3::from(self[e5]) * other.group0().xyz()).with_w(self[e2] * other[e315]),
        )
    }
}
impl Wedge<VersorOdd> for RoundPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       18        0
    //    simd3        2        3        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       11       25        0
    //  no simd       24       43        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                self[e2] * other[e43] * -1.0,
                self[e3] * other[e41] * -1.0,
                self[e1] * other[e42] * -1.0,
                (self[e3] * other[e4125]) + (self[e4] * other[e3215]) + (self[e5] * other[e1234]),
            ]) + (self.group0().zxyx() * other.group0().yzx().with_w(other[e4235]))
                + (self.group0().wwwy() * other.group1().xyz().with_w(other[e4315])),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e4] * other[e15]) + (self[e5] * other[e41]),
                (self[e4] * other[e25]) + (self[e5] * other[e42]),
                (self[e4] * other[e35]) + (self[e5] * other[e43]),
                -(self[e2] * other[e31]) - (self[e3] * other[e12]),
            ]) - (self.group0().xyzx() * other.group1().wwwx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[e5]) * other.group1().xyz()) + (self.group0().yzx() * other.group2().zxy()) - (self.group0().zxy() * other.group2().yzx()))
                .with_w(self[e5] * other[scalar]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group0(),
        )
    }
}
impl std::ops::Div<WedgeInfix> for Scalar {
    type Output = WedgeInfixPartial<Scalar>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for Scalar {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(self[scalar]) * other.group2(),
        )
    }
}
impl Wedge<AntiDipoleInversion> for Scalar {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[scalar]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(self[scalar]) * other.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(self[scalar]) * other.group3(),
        )
    }
}
impl Wedge<AntiDualNum> for Scalar {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(self[scalar]) * other.group0())
    }
}
impl Wedge<AntiFlatPoint> for Scalar {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(self[scalar]) * other.group0())
    }
}
impl Wedge<AntiFlector> for Scalar {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[scalar]) * other.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(self[scalar]) * other.group1(),
        )
    }
}
impl Wedge<AntiLine> for Scalar {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group0(),
            // e15, e25, e35
            Simd32x3::from(self[scalar]) * other.group1(),
        )
    }
}
impl Wedge<AntiMotor> for Scalar {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(self[scalar]) * other.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(self[scalar]) * other.group1(),
        )
    }
}
impl Wedge<AntiPlane> for Scalar {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[scalar]) * other.group0())
    }
}
impl Wedge<AntiScalar> for Scalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[e12345] * self[scalar])
    }
}
impl Wedge<Circle> for Scalar {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[scalar]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1(),
            // e235, e315, e125
            Simd32x3::from(self[scalar]) * other.group2(),
        )
    }
}
impl Wedge<CircleRotor> for Scalar {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[scalar]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(self[scalar]) * other.group2(),
        )
    }
}
impl Wedge<Dipole> for Scalar {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1(),
            // e15, e25, e35
            Simd32x3::from(self[scalar]) * other.group2(),
        )
    }
}
impl Wedge<DipoleInversion> for Scalar {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(self[scalar]) * other.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group3(),
        )
    }
}
impl Wedge<DualNum> for Scalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(self[scalar]) * other.group0())
    }
}
impl Wedge<FlatPoint> for Scalar {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self[scalar]) * other.group0())
    }
}
impl Wedge<Flector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * other.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group1(),
        )
    }
}
impl Wedge<Line> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(self[scalar]) * other.group0(),
            // e235, e315, e125
            Simd32x3::from(self[scalar]) * other.group1(),
        )
    }
}
impl Wedge<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(self[scalar]) * other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(self[scalar]) * other.group1(),
        )
    }
}
impl Wedge<MultiVector> for Scalar {
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
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(self[scalar]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group1(),
            // e5
            other[e5] * self[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * other.group3(),
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group4(),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group6(),
            // e423, e431, e412
            Simd32x3::from(self[scalar]) * other.group7(),
            // e235, e315, e125
            Simd32x3::from(self[scalar]) * other.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group9(),
            // e1234
            other[e1234] * self[scalar],
        )
    }
}
impl Wedge<Plane> for Scalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self[scalar]) * other.group0())
    }
}
impl Wedge<RoundPoint> for Scalar {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[scalar]) * other.group0(), /* e5 */ other[e5] * self[scalar])
    }
}
impl Wedge<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[scalar] * self[scalar])
    }
}
impl Wedge<Sphere> for Scalar {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group0(),
            // e1234
            self[scalar] * other[e1234],
        )
    }
}
impl Wedge<VersorEven> for Scalar {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self[scalar]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(self[scalar]) * other.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group3(),
        )
    }
}
impl Wedge<VersorOdd> for Scalar {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(self[scalar]) * other.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group3(),
        )
    }
}
impl std::ops::Div<WedgeInfix> for Sphere {
    type Output = WedgeInfixPartial<Sphere>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1234
            other[scalar] * self[e1234],
        )
    }
}
impl Wedge<AntiDipoleInversion> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (other[e4] * self[e3215]) + (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e5] * self[e1234]),
        )
    }
}
impl Wedge<AntiDualNum> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1234
            other[scalar] * self[e1234],
        )
    }
}
impl Wedge<AntiFlector> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e5] * self[e1234]),
        )
    }
}
impl Wedge<AntiMotor> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1234
            other[scalar] * self[e1234],
        )
    }
}
impl Wedge<AntiPlane> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e5] * self[e1234]),
        )
    }
}
impl Wedge<DualNum> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[e5] * self[e1234])
    }
}
impl Wedge<Motor> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[e5] * self[e1234])
    }
}
impl Wedge<MultiVector> for Sphere {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        4       10        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e4] * self[e3215]) + (other[e5] * self[e1234]),
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
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1234
            other[scalar] * self[e1234],
        )
    }
}
impl Wedge<RoundPoint> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (other[e1] * self[e4235]) + (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e4] * self[e3215]) + (other[e5] * self[e1234]),
        )
    }
}
impl Wedge<Scalar> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1234
            other[scalar] * self[e1234],
        )
    }
}
impl Wedge<VersorEven> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (self[e4235] * other[e1]) + (self[e4315] * other[e2]) + (self[e4125] * other[e3]) + (self[e3215] * other[e4]) + (self[e1234] * other[e5]),
        )
    }
}
impl Wedge<VersorOdd> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1234
            self[e1234] * other[scalar],
        )
    }
}
impl std::ops::Div<WedgeInfix> for VersorEven {
    type Output = WedgeInfixPartial<VersorEven>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       27        0
    //    simd3        3        7        0
    //    simd4        4        2        0
    // Totals...
    // yes simd       22       36        0
    //  no simd       40       56        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other[e23] * self[e4]) + (other[scalar] * self[e423]),
                (other[e31] * self[e4]) + (other[scalar] * self[e431]),
                (other[e12] * self[e4]) + (other[scalar] * self[e412]),
                -(other[e42] * self[e315])
                    - (other[e43] * self[e125])
                    - (other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435])
                    - (other[e45] * self[e321])
                    - (other[e15] * self[e423])
                    - (other[e25] * self[e431])
                    - (other[e35] * self[e412]),
            ]) + (other.group0().yzx() * self.group3().zxy()).with_w(other[scalar] * self[e12345])
                - (other.group0().zxy() * self.group3().yzx()).with_w(other[e41] * self[e235]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other[e15] * self[e4]) + (other[scalar] * self[e415]),
                (other[e25] * self[e4]) + (other[scalar] * self[e425]),
                (other[e35] * self[e4]) + (other[scalar] * self[e435]),
                -(other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]) + (other.group0() * self.group2().www()).with_w(other[scalar] * self[e321])
                - (other.group1().wwwx() * self.group3().xyzx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(other[scalar]) * self.group2().xyz()) + (Simd32x3::from(self[e5]) * other.group1().xyz()) + (other.group2().zxy() * self.group3().yzx())
                - (other.group2().yzx() * self.group3().zxy()))
            .with_w(other[scalar] * self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group3(),
        )
    }
}
impl Wedge<AntiDipoleInversion> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        7        0
    //    simd4       10        7        0
    // Totals...
    // yes simd       16       25        0
    //  no simd       48       60        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group3().xyz()) - (Simd32x3::from(other[e4]) * self.group3().xyz()),
            // e23, e31, e12, e45
            (other.group3().zxyw() * self.group3().yzxw()) - (other.group3().yzx() * self.group3().zxy()).with_w(other[e4] * self[e5]),
            // e15, e25, e35, e1234
            (self.group3().xyzx() * other.group3().www().with_w(other[e423]))
                + Simd32x3::from(0.0).with_w(
                    (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]) - (other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]),
                )
                - (other.group3().xyz() * self.group2().www()).with_w(other[e4] * self[e321]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(other[e5]) * self.group0().xyz().with_w(self[e321]))
                + (other.group3().yzxz() * self.group1().zxy().with_w(self[e125]))
                + (other.group1().yzx() * self.group3().zxy()).with_w(other[e1] * self[e235])
                + (other.group2().xyz() * self.group3().www()).with_w(other[e2] * self[e315])
                - (Simd32x4::from(self[e5]) * other.group0().with_w(other[e321]))
                - (other.group2().wwwy() * self.group2().xyz().with_w(self[e2]))
                - (self.group3().yzxx() * other.group1().zxy().with_w(other[e235]))
                - (other.group3().zxy() * self.group1().yzx()).with_w(other[e125] * self[e3]),
        )
    }
}
impl Wedge<AntiDualNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1       18        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0().yy().with_zw(other[scalar], (other[e3215] * self[e4]) + (other[scalar] * self[e12345])) * self.group0().xyz().with_w(1.0),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(other[scalar]) * self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group3(),
        )
    }
}
impl Wedge<AntiFlatPoint> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3        8        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(self[e4]) * other.group0().xyz()).with_w(-(other[e235] * self[e1]) - (other[e315] * self[e2]) - (other[e125] * self[e3]) - (other[e321] * self[e5])),
            // e1234
            other[e321] * self[e4],
        )
    }
}
impl Wedge<AntiFlector> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        1        5        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       11       18        0
    //  no simd       31       40        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e4]) * other.group1().xyz(),
            // e23, e31, e12, e45
            ((other.group1().zxy() * self.group3().yzx()) - (other.group1().yzx() * self.group3().zxy())).with_w(other[e5] * self[e4]),
            // e15, e25, e35, e1234
            (self.group3() * other.group1().www().with_w(other[e321])) + Simd32x3::from(0.0).with_w(-(other[e2] * self[e431]) - (other[e3] * self[e412]))
                - (other.group1().xyzx() * self.group2().www().with_w(self[e423])),
            // e4235, e4315, e4125, e3215
            (other.group1().yzxy() * self.group1().zxy().with_w(self[e315]))
                + (other.group1().wwwz() * self.group0().xyz().with_w(self[e125]))
                + Simd32x3::from(0.0).with_w((other[e5] * self[e321]) - (other[e315] * self[e2]) - (other[e125] * self[e3]) - (other[e321] * self[e5]))
                + (other.group0().xyz() * self.group3().www()).with_w(other[e1] * self[e235])
                - (other.group1().zxy() * self.group1().yzx()).with_w(other[e235] * self[e1]),
        )
    }
}
impl Wedge<AntiLine> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       24        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e4]) * other.group0(),
            // e415, e425, e435, e321
            (Simd32x3::from(self[e4]) * other.group1()).with_w(-(other[e23] * self[e1]) - (other[e31] * self[e2]) - (other[e12] * self[e3])),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other[e23] * self[e5]) + (other[e35] * self[e2]),
                (other[e31] * self[e5]) + (other[e15] * self[e3]),
                (other[e12] * self[e5]) + (other[e25] * self[e1]),
                -(other[e31] * self[e425]) - (other[e12] * self[e435]) - (other[e15] * self[e423]) - (other[e25] * self[e431]) - (other[e35] * self[e412]),
            ]) - (other.group1().yzx() * self.group3().zxy()).with_w(other[e23] * self[e415]),
        )
    }
}
impl Wedge<AntiMotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd3        3        5        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       28       41        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() * self.group3().www().with_w(self[e12345]))
                + Simd32x3::from(0.0).with_w(
                    -(other[e23] * self[e415])
                        - (other[e31] * self[e425])
                        - (other[e12] * self[e435])
                        - (other[e15] * self[e423])
                        - (other[e25] * self[e431])
                        - (other[e35] * self[e412]),
                )
                + (self.group0().xyz() * other.group0().www()).with_w(other[e3215] * self[e4]),
            // e415, e425, e435, e321
            Simd32x4::from([
                other[e15] * self[e4],
                other[e25] * self[e4],
                other[e35] * self[e4],
                -(other[e23] * self[e1]) - (other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]) + (Simd32x4::from(other[scalar]) * self.group1()),
            // e235, e315, e125, e5
            ((Simd32x3::from(other[scalar]) * self.group2().xyz()) + (Simd32x3::from(self[e5]) * other.group0().xyz()) + (other.group1().zxy() * self.group3().yzx())
                - (other.group1().yzx() * self.group3().zxy()))
            .with_w(other[scalar] * self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group3(),
        )
    }
}
impl Wedge<AntiPlane> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       14        0
    //    simd3        1        3        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        6       20        0
    //  no simd       17       35        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e4]) * other.group0().xyz(),
            // e23, e31, e12, e45
            ((other.group0().zxy() * self.group3().yzx()) - (other.group0().yzx() * self.group3().zxy())).with_w(other[e5] * self[e4]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e5] * self[e1], other[e5] * self[e2], other[e5] * self[e3], -(other[e2] * self[e431]) - (other[e3] * self[e412])])
                - (other.group0().xyzx() * self.group2().www().with_w(self[e423])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other[e3] * self[e425] * -1.0,
                other[e1] * self[e435] * -1.0,
                other[e2] * self[e415] * -1.0,
                (other[e3] * self[e125]) + (other[e5] * self[e321]),
            ]) + (other.group0().yzxx() * self.group1().zxy().with_w(self[e235]))
                + (other.group0().wwwy() * self.group0().xyz().with_w(self[e315])),
        )
    }
}
impl Wedge<Circle> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       20        0
    fn wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e425] * self[e3]) + (other[e235] * self[e4]),
                (other[e435] * self[e1]) + (other[e315] * self[e4]),
                (other[e415] * self[e2]) + (other[e125] * self[e4]),
                -(other[e321] * self[e5]) - (other[e125] * self[e3]),
            ]) - (self.group3().yzxy() * other.group1().zxy().with_w(other[e315]))
                - (other.group0() * self.group2().www()).with_w(other[e235] * self[e1]),
            // e1234
            (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]),
        )
    }
}
impl Wedge<CircleRotor> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e425] * self[e3]) + (other[e235] * self[e4]),
                (other[e435] * self[e1]) + (other[e315] * self[e4]),
                (other[e415] * self[e2]) + (other[e125] * self[e4]),
                -(other[e315] * self[e2]) - (other[e125] * self[e3]),
            ]) - (Simd32x4::from(self[e5]) * other.group0().with_w(other[e321]))
                - (self.group3().yzxx() * other.group1().zxy().with_w(other[e235])),
            // e1234
            (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]),
        )
    }
}
impl Wedge<Dipole> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       24        0
    //    simd3        2        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       29       40        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e4]) * other.group1().xyz()) + (other.group0().yzx() * self.group3().zxy()) - (other.group0().zxy() * self.group3().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other[e41] * self[e5]) + (other[e15] * self[e4]),
                (other[e42] * self[e5]) + (other[e25] * self[e4]),
                (other[e43] * self[e5]) + (other[e35] * self[e4]),
                -(other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]) - (other.group1().wwwx() * self.group3().xyzx()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other[e23] * self[e5]) + (other[e35] * self[e2]),
                (other[e31] * self[e5]) + (other[e15] * self[e3]),
                (other[e12] * self[e5]) + (other[e25] * self[e1]),
                -(other[e42] * self[e315])
                    - (other[e43] * self[e125])
                    - (other[e23] * self[e415])
                    - (other[e31] * self[e425])
                    - (other[e12] * self[e435])
                    - (other[e45] * self[e321])
                    - (other[e15] * self[e423])
                    - (other[e25] * self[e431])
                    - (other[e35] * self[e412]),
            ]) - (other.group2().yzx() * self.group3().zxy()).with_w(other[e41] * self[e235]),
        )
    }
}
impl Wedge<DipoleInversion> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        2        4        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       21       28        0
    //  no simd       37       45        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e4]) * other.group1().xyz()) + (other.group0().yzx() * self.group3().zxy()) - (other.group0().zxy() * self.group3().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other[e41] * self[e5]) + (other[e15] * self[e4]),
                (other[e42] * self[e5]) + (other[e25] * self[e4]),
                (other[e43] * self[e5]) + (other[e35] * self[e4]),
                -(other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]) - (other.group1().wwwx() * self.group3().xyzx()),
            // e235, e315, e125, e12345
            (Simd32x4::from(self[e5]) * other.group1().xyz().with_w(other[e1234]))
                + (self.group3().yzxx() * other.group2().zxy().with_w(other[e4235]))
                + Simd32x3::from(0.0).with_w(
                    (other[e4315] * self[e2]) + (other[e4125] * self[e3]) + (other[e3215] * self[e4])
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
                - (other.group2().yzx() * self.group3().zxy()).with_w(other[e41] * self[e235]),
        )
    }
}
impl Wedge<DualNum> for VersorEven {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[e5]) * self.group3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e5]) * self.group0().xyz().with_w(self[e321]),
        )
    }
}
impl Wedge<FlatPoint> for VersorEven {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        9       16        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                other[e15] * self[e4],
                other[e25] * self[e4],
                other[e35] * self[e4],
                -(other[e25] * self[e431]) - (other[e35] * self[e412]) - (other[e45] * self[e321]),
            ]) - (other.group0().wwwx() * self.group3().xyz().with_w(self[e423])),
            // e235, e315, e125, e5
            ((other.group0().zxy() * self.group3().yzx()) - (other.group0().yzx() * self.group3().zxy())).with_w(0.0),
        )
    }
}
impl Wedge<Flector> for VersorEven {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        8       10        0
    //  no simd       16       20        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (self.group3().wwwx() * other.group0().xyz().with_w(other[e4235]))
                + Simd32x3::from(0.0).with_w(
                    (other[e4315] * self[e2]) + (other[e4125] * self[e3]) + (other[e3215] * self[e4])
                        - (other[e25] * self[e431])
                        - (other[e35] * self[e412])
                        - (other[e45] * self[e321]),
                )
                - (other.group0().wwwx() * self.group3().xyz().with_w(self[e423])),
            // e235, e315, e125, e5
            ((other.group0().zxy() * self.group3().yzx()) - (other.group0().yzx() * self.group3().zxy())).with_w(0.0),
        )
    }
}
impl Wedge<Line> for VersorEven {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e425] * self[e3]) + (other[e235] * self[e4]),
                (other[e435] * self[e1]) + (other[e315] * self[e4]),
                (other[e415] * self[e2]) + (other[e125] * self[e4]),
                -(other[e315] * self[e2]) - (other[e125] * self[e3]),
            ]) - (self.group3().yzxx() * other.group0().zxy().with_w(other[e235])),
        )
    }
}
impl Wedge<Motor> for VersorEven {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       20        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[e5]) * self.group3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e235] * self[e4]) + (other[e5] * self[e423]),
                (other[e315] * self[e4]) + (other[e5] * self[e431]),
                (other[e125] * self[e4]) + (other[e5] * self[e412]),
                -(other[e315] * self[e2]) - (other[e125] * self[e3]),
            ]) + (other.group0().yzx() * self.group3().zxy()).with_w(other[e5] * self[e321])
                - (self.group3().yzxx() * other.group0().zxy().with_w(other[e235])),
        )
    }
}
impl Wedge<MultiVector> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       38        0
    //    simd3        8       17        0
    //    simd4       10        8        0
    // Totals...
    // yes simd       44       63        0
    //  no simd       90      121        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group3(),
            // e5
            other[scalar] * self[e5],
            // e15, e25, e35, e45
            (Simd32x4::from(other[e5]) * self.group3()) - (Simd32x4::from(self[e5]) * other.group1()),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group1().xyz()) - (Simd32x3::from(other[e4]) * self.group3().xyz()),
            // e23, e31, e12
            (other.group1().zxy() * self.group3().yzx()) - (other.group1().yzx() * self.group3().zxy()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other[e15] * self[e4]) + (other[e41] * self[e5]),
                (other[e25] * self[e4]) + (other[e42] * self[e5]),
                (other[e35] * self[e4]) + (other[e43] * self[e5]),
                -(other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]) + (Simd32x4::from(other[scalar]) * self.group1())
                - (self.group3().xyzx() * other.group3().www().with_w(other[e23])),
            // e423, e431, e412
            (Simd32x3::from(other[scalar]) * self.group0().xyz()) + (Simd32x3::from(self[e4]) * other.group5()) + (other.group4().yzx() * self.group3().zxy())
                - (other.group4().zxy() * self.group3().yzx()),
            // e235, e315, e125
            (Simd32x3::from(other[scalar]) * self.group2().xyz()) + (Simd32x3::from(self[e5]) * other.group5()) + (other.group3().zxy() * self.group3().yzx())
                - (other.group3().yzx() * self.group3().zxy()),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(other[e5]) * self.group0().xyz().with_w(self[e321]))
                + (other.group1().yzxy() * self.group1().zxy().with_w(self[e315]))
                + (other.group8() * self.group3().www()).with_w(other[e1] * self[e235])
                + (other.group6().yzx() * self.group3().zxy()).with_w(other[e3] * self[e125])
                - (other.group6().zxyw() * self.group3().yzx().with_w(self[e5]))
                - (other.group7() * self.group2().www()).with_w(other[e235] * self[e1])
                - (other.group1().zxy() * self.group1().yzx()).with_w(other[e315] * self[e2])
                - (self.group2().xyz() * other.group1().www()).with_w(other[e125] * self[e3]),
            // e1234
            (other[e321] * self[e4]) + (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3])
                - (other[e1] * self[e423])
                - (other[e2] * self[e431])
                - (other[e3] * self[e412])
                - (other[e4] * self[e321]),
        )
    }
}
impl Wedge<Plane> for VersorEven {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (other[e4235] * self[e1]) + (other[e4315] * self[e2]) + (other[e4125] * self[e3]) + (other[e3215] * self[e4]),
        )
    }
}
impl Wedge<RoundPoint> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       16        0
    //    simd3        1        4        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       25       40        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group0().xyz()) - (Simd32x3::from(other[e4]) * self.group3().xyz()),
            // e23, e31, e12, e45
            (other.group0().zxy() * self.group3().yzx()).with_w(other[e5] * self[e4]) - (other.group0().yzxw() * self.group3().zxy().with_w(self[e5])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                other[e5] * self[e1],
                other[e5] * self[e2],
                other[e5] * self[e3],
                -(other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]),
            ]) - (other.group0().xyzx() * self.group2().www().with_w(self[e423])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(other[e3] * self[e425]) - (other[e4] * self[e235]),
                -(other[e1] * self[e435]) - (other[e4] * self[e315]),
                -(other[e2] * self[e415]) - (other[e4] * self[e125]),
                (other[e3] * self[e125]) + (other[e5] * self[e321]),
            ]) + (other.group0().yzxx() * self.group1().zxy().with_w(self[e235]))
                + (Simd32x3::from(other[e5]) * self.group0().xyz()).with_w(other[e2] * self[e315]),
        )
    }
}
impl Wedge<Scalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn wedge(self, other: Scalar) -> Self::Output {
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
impl Wedge<Sphere> for VersorEven {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (other[e4235] * self[e1]) + (other[e4315] * self[e2]) + (other[e4125] * self[e3]) + (other[e3215] * self[e4]) + (other[e1234] * self[e5]),
        )
    }
}
impl Wedge<VersorEven> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        7        0
    //    simd4       10        7        0
    // Totals...
    // yes simd       16       25        0
    //  no simd       48       60        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group3().xyz()) - (Simd32x3::from(other[e4]) * self.group3().xyz()),
            // e23, e31, e12, e45
            (self.group3().yzxw() * other.group3().zxy().with_w(other[e5])) - (other.group3().yzxw() * self.group3().zxy().with_w(self[e5])),
            // e15, e25, e35, e1234
            (self.group3().xyzx() * other.group2().www().with_w(other[e423]))
                + Simd32x3::from(0.0).with_w(
                    (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]),
                )
                - (other.group3().xyzx() * self.group2().www().with_w(self[e423])),
            // e4235, e4315, e4125, e3215
            (other.group3().yzxz() * self.group1().zxy().with_w(self[e125]))
                + (other.group1().yzx() * self.group3().zxy()).with_w(other[e5] * self[e321])
                + (other.group2().xyz() * self.group3().www()).with_w(other[e1] * self[e235])
                + (self.group0().xyz() * other.group2().www()).with_w(other[e2] * self[e315])
                - (Simd32x4::from(self[e5]) * other.group0().xyz().with_w(other[e321]))
                - (self.group3().yzxx() * other.group1().zxy().with_w(other[e235]))
                - (other.group3().zxy() * self.group1().yzx()).with_w(other[e315] * self[e2])
                - (self.group2().xyz() * other.group3().www()).with_w(other[e125] * self[e3]),
        )
    }
}
impl Wedge<VersorOdd> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       23        0
    //    simd3        3        6        0
    //    simd4        6        5        0
    // Totals...
    // yes simd       24       34        0
    //  no simd       48       61        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from(other[scalar]) * self.group0())
                + (self.group3().wwwx() * other.group1().xyz().with_w(other[e4235]))
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
                + (self.group3().zxy() * other.group0().yzx()).with_w(self[e5] * other[e1234])
                - (self.group3().yzx() * other.group0().zxy()).with_w(self[e423] * other[e15]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e5] * other[e41]) + (self[e4] * other[e15]),
                (self[e5] * other[e42]) + (self[e4] * other[e25]),
                (self[e5] * other[e43]) + (self[e4] * other[e35]),
                -(self[e2] * other[e31]) - (self[e3] * other[e12]),
            ]) + (Simd32x4::from(other[scalar]) * self.group1())
                - (self.group3().xyzx() * other.group1().wwwx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[e5]) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * self.group2().xyz()) + (self.group3().yzx() * other.group2().zxy())
                - (self.group3().zxy() * other.group2().yzx()))
            .with_w(self[e5] * other[scalar]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group3(),
        )
    }
}
impl std::ops::Div<WedgeInfix> for VersorOdd {
    type Output = WedgeInfixPartial<VersorOdd>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiCircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       28        0
    //    simd3        1        4        0
    //    simd4        5        4        0
    // Totals...
    // yes simd       23       36        0
    //  no simd       40       56        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x3::from(other[scalar]) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * other.group0())).with_w(other[scalar] * self[scalar]),
            // e23, e31, e12, e45
            (Simd32x4::from(other[scalar]) * self.group1()) + (Simd32x4::from(self[scalar]) * other.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                other[scalar] * self[e15],
                other[scalar] * self[e25],
                other[scalar] * self[e35],
                -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ]) + (other.group2() * self.group0().www().with_w(self[e1234])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e23] * self[e45]) + (other[e45] * self[e23]) + (other[e35] * self[e42]) + (other[scalar] * self[e4235]),
                (other[e31] * self[e45]) + (other[e45] * self[e31]) + (other[e15] * self[e43]) + (other[scalar] * self[e4315]),
                (other[e12] * self[e45]) + (other[e45] * self[e12]) + (other[e25] * self[e41]) + (other[scalar] * self[e4125]),
                -(other[e12] * self[e35]) - (other[e15] * self[e23]) - (other[e25] * self[e31]) - (other[e35] * self[e12]),
            ]) + (other.group0().yzx() * self.group2().zxy()).with_w(other[scalar] * self[e3215])
                - (self.group2().yzxx() * other.group0().zxy().with_w(other[e23]))
                - (other.group2().yzx() * self.group0().zxy()).with_w(other[e31] * self[e25]),
        )
    }
}
impl Wedge<AntiDipoleInversion> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        0
    //    simd3        3        7        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       23       34        0
    //  no simd       47       60        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group3().zxyy() * self.group0().yzx().with_w(self[e4315]))
                + Simd32x3::from(0.0).with_w(
                    (other[e3] * self[e4125]) + (other[e5] * self[e1234])
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
                + (other.group0() * self.group0().www()).with_w(other[e4] * self[e3215])
                + (self.group1().xyz() * other.group2().www()).with_w(other[e1] * self[e4235])
                - (other.group3().yzx() * self.group0().zxy()).with_w(other[e423] * self[e15]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other[e4] * self[e15]) + (other[e5] * self[e41]),
                (other[e4] * self[e25]) + (other[e5] * self[e42]),
                (other[e4] * self[e35]) + (other[e5] * self[e43]),
                -(other[e2] * self[e31]) - (other[e3] * self[e12]),
            ]) + (Simd32x4::from(self[scalar]) * other.group1())
                - (other.group3().xyzx() * self.group1().wwwx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(other[e5]) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * other.group2().xyz()) + (other.group3().yzx() * self.group2().zxy())
                - (other.group3().zxy() * self.group2().yzx()))
            .with_w(other[e5] * self[scalar]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group3().xyz().with_w(other[e4]),
        )
    }
}
impl Wedge<AntiDualNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1       18        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[scalar]) * self.group2(),
            // e4235, e4315, e4125, e3215
            other.group0().yy().with_zw(other[scalar], (other[e3215] * self[scalar]) + (other[scalar] * self[e3215])) * self.group3().xyz().with_w(1.0),
        )
    }
}
impl Wedge<AntiFlatPoint> for VersorOdd {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3        8        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321] * self[scalar]),
            // e235, e315, e125, e12345
            (Simd32x3::from(self[scalar]) * other.group0().xyz())
                .with_w(-(other[e235] * self[e41]) - (other[e315] * self[e42]) - (other[e125] * self[e43]) - (other[e321] * self[e45])),
        )
    }
}
impl Wedge<AntiFlector> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        0
    //    simd3        3        5        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       13       18        0
    //  no simd       31       40        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group1().zxyx() * self.group0().yzx().with_w(self[e4235]))
                + Simd32x3::from(0.0).with_w(
                    (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e5] * self[e1234])
                        - (other[e315] * self[e42])
                        - (other[e125] * self[e43])
                        - (other[e321] * self[e45]),
                )
                - (self.group0().zxyx() * other.group1().yzx().with_w(other[e235])),
            // e415, e425, e435, e321
            (self.group0() * other.group1().www().with_w(other[e321])) + Simd32x3::from(0.0).with_w(-(other[e2] * self[e31]) - (other[e3] * self[e12]))
                - (other.group1().xyzx() * self.group1().wwwx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(other[e5]) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * other.group0().xyz()) + (other.group1().yzx() * self.group2().zxy())
                - (other.group1().zxy() * self.group2().yzx()))
            .with_w(other[e5] * self[scalar]),
            // e1, e2, e3, e4
            (other.group1().xyz() * self.group0().www()).with_w(0.0),
        )
    }
}
impl Wedge<AntiLine> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       24        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            (other.group0() * self.group0().www()).with_w(0.0),
            // e15, e25, e35, e1234
            (Simd32x3::from(self[scalar]) * other.group1()).with_w(-(other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e23] * self[e45]) + (other[e35] * self[e42]),
                (other[e31] * self[e45]) + (other[e15] * self[e43]),
                (other[e12] * self[e45]) + (other[e25] * self[e41]),
                -(other[e31] * self[e25]) - (other[e12] * self[e35]) - (other[e15] * self[e23]) - (other[e25] * self[e31]) - (other[e35] * self[e12]),
            ]) - (other.group1().yzx() * self.group0().zxy()).with_w(other[e23] * self[e15]),
        )
    }
}
impl Wedge<AntiMotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        1        4        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       11       24        0
    //  no simd       25       41        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other[scalar]) * self.group0(),
            // e23, e31, e12, e45
            ((Simd32x3::from(other[scalar]) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * other.group0().xyz())).with_w(other[scalar] * self[e45]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                other[e15] * self[scalar],
                other[e25] * self[scalar],
                other[e35] * self[scalar],
                -(other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ]) + (Simd32x4::from(other[scalar]) * self.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other[e35] * self[e42],
                other[e15] * self[e43],
                other[e25] * self[e41],
                -(other[e31] * self[e25]) - (other[e12] * self[e35]) - (other[e15] * self[e23]) - (other[e25] * self[e31]) - (other[e35] * self[e12]),
            ]) + (other.group0() * self.group1().www().with_w(self[e3215]))
                + (self.group3().xyz() * other.group0().www()).with_w(other[e3215] * self[scalar])
                - (other.group1().yzx() * self.group0().zxy()).with_w(other[e23] * self[e15]),
        )
    }
}
impl Wedge<AntiPlane> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       15        0
    //    simd3        2        4        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        7       21        0
    //  no simd       17       35        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                other[e2] * self[e43] * -1.0,
                other[e3] * self[e41] * -1.0,
                other[e1] * self[e42] * -1.0,
                (other[e2] * self[e4315]) + (other[e3] * self[e4125]) + (other[e5] * self[e1234]),
            ]) + (other.group0().zxyx() * self.group0().yzx().with_w(self[e4235])),
            // e415, e425, e435, e321
            Simd32x4::from([other[e5] * self[e41], other[e5] * self[e42], other[e5] * self[e43], -(other[e2] * self[e31]) - (other[e3] * self[e12])])
                - (other.group0().xyzx() * self.group1().wwwx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(other[e5]) * self.group1().xyz()) + (other.group0().yzx() * self.group2().zxy()) - (other.group0().zxy() * self.group2().yzx()))
                .with_w(other[e5] * self[scalar]),
            // e1, e2, e3, e4
            (other.group0().xyz() * self.group0().www()).with_w(0.0),
        )
    }
}
impl Wedge<AntiScalar> for VersorOdd {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[e12345] * self[scalar])
    }
}
impl Wedge<Circle> for VersorOdd {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd        9       20        0
    fn wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[scalar]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1(),
            // e235, e315, e125, e12345
            (Simd32x3::from(self[scalar]) * other.group2()).with_w(
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
impl Wedge<CircleRotor> for VersorOdd {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       11        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       14        0
    //  no simd       10       21        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[scalar]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1(),
            // e235, e315, e125, e12345
            (Simd32x3::from(self[scalar]) * other.group2().xyz()).with_w(
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
            ),
        )
    }
}
impl Wedge<Dipole> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       25       40        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1(),
            // e15, e25, e35, e1234
            (Simd32x3::from(self[scalar]) * other.group2()).with_w(
                -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e42] * self[e35]) + (other[e23] * self[e45]) + (other[e45] * self[e23]) + (other[e35] * self[e42]),
                (other[e43] * self[e15]) + (other[e31] * self[e45]) + (other[e45] * self[e31]) + (other[e15] * self[e43]),
                (other[e41] * self[e25]) + (other[e12] * self[e45]) + (other[e45] * self[e12]) + (other[e25] * self[e41]),
                -(other[e23] * self[e15]) - (other[e31] * self[e25]) - (other[e12] * self[e35]) - (other[e35] * self[e12]),
            ]) - (other.group0().zxy() * self.group2().yzx()).with_w(other[e15] * self[e23])
                - (other.group2().yzx() * self.group0().zxy()).with_w(other[e25] * self[e31]),
        )
    }
}
impl Wedge<DipoleInversion> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       25        0
    //    simd3        0        4        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       21       31        0
    //  no simd       30       45        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1(),
            // e15, e25, e35, e1234
            (Simd32x3::from(self[scalar]) * other.group2().xyz()).with_w(
                (other[e1234] * self[scalar])
                    - (other[e41] * self[e23])
                    - (other[e42] * self[e31])
                    - (other[e43] * self[e12])
                    - (other[e23] * self[e41])
                    - (other[e31] * self[e42])
                    - (other[e12] * self[e43]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e23] * self[e45]) + (other[e45] * self[e23]) + (other[e35] * self[e42]) + (other[e4235] * self[scalar]),
                (other[e31] * self[e45]) + (other[e45] * self[e31]) + (other[e15] * self[e43]) + (other[e4315] * self[scalar]),
                (other[e12] * self[e45]) + (other[e45] * self[e12]) + (other[e25] * self[e41]) + (other[e4125] * self[scalar]),
                -(other[e12] * self[e35]) - (other[e15] * self[e23]) - (other[e25] * self[e31]) - (other[e35] * self[e12]),
            ]) + (other.group0().yzx() * self.group2().zxy()).with_w(other[e3215] * self[scalar])
                - (self.group2().yzxx() * other.group0().zxy().with_w(other[e23]))
                - (other.group2().yzx() * self.group0().zxy()).with_w(other[e31] * self[e25]),
        )
    }
}
impl Wedge<DualNum> for VersorOdd {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1       10        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0().xx().with_zw(other[e5], (other[e5] * self[e1234]) + (other[e12345] * self[scalar])) * self.group0().xyz().with_w(1.0),
            // e235, e315, e125, e5
            Simd32x4::from(other[e5]) * self.group1().xyz().with_w(self[scalar]),
        )
    }
}
impl Wedge<FlatPoint> for VersorOdd {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * other.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e35] * self[e42]) + (other[e45] * self[e23]),
                (other[e15] * self[e43]) + (other[e45] * self[e31]),
                (other[e25] * self[e41]) + (other[e45] * self[e12]),
                -(other[e25] * self[e31]) - (other[e35] * self[e12]),
            ]) - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        )
    }
}
impl Wedge<Flector> for VersorOdd {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       20        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * other.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e45] * self[e23]) + (other[e4235] * self[scalar]),
                (other[e45] * self[e31]) + (other[e4315] * self[scalar]),
                (other[e45] * self[e12]) + (other[e4125] * self[scalar]),
                -(other[e25] * self[e31]) - (other[e35] * self[e12]),
            ]) + (self.group0().yzxw() * other.group0().zxy().with_w(other[e3215]))
                - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        )
    }
}
impl Wedge<Line> for VersorOdd {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x3::from(self[scalar]) * other.group0()).with_w(
                -(other[e415] * self[e23])
                    - (other[e425] * self[e31])
                    - (other[e435] * self[e12])
                    - (other[e235] * self[e41])
                    - (other[e315] * self[e42])
                    - (other[e125] * self[e43]),
            ),
            // e235, e315, e125, e5
            (other.group1() * self.group0().www()).with_w(0.0),
        )
    }
}
impl Wedge<Motor> for VersorOdd {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        8       11        0
    //  no simd       16       21        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x4::from(other[e5]) * self.group0().xyz().with_w(self[e1234]))
                + (Simd32x4::from(self[scalar]) * other.group0())
                + Simd32x3::from(0.0).with_w(
                    -(other[e415] * self[e23])
                        - (other[e425] * self[e31])
                        - (other[e435] * self[e12])
                        - (other[e235] * self[e41])
                        - (other[e315] * self[e42])
                        - (other[e125] * self[e43]),
                ),
            // e235, e315, e125, e5
            ((Simd32x3::from(other[e5]) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * other.group1().xyz())).with_w(other[e5] * self[scalar]),
        )
    }
}
impl Wedge<MultiVector> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       53        0
    //    simd3        8       15        0
    //    simd4        7        6        0
    // Totals...
    // yes simd       53       74        0
    //  no simd       90      122        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                other[scalar] * self[scalar],
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group1(),
            // e5
            other[e5] * self[scalar],
            // e15, e25, e35, e45
            (Simd32x4::from(other[scalar]) * self.group2().xyz().with_w(self[e45])) + (Simd32x4::from(self[scalar]) * other.group3()),
            // e41, e42, e43
            (Simd32x3::from(other[scalar]) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * other.group4()),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * other.group5()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other[e5] * self[e41]) + (other[e415] * self[scalar]),
                (other[e5] * self[e42]) + (other[e425] * self[scalar]),
                (other[e5] * self[e43]) + (other[e435] * self[scalar]),
                -(other[e2] * self[e31]) - (other[e3] * self[e12]),
            ]) + (self.group2().xyz() * other.group1().www()).with_w(other[e321] * self[scalar])
                - (other.group1().xyzx() * self.group1().wwwx()),
            // e423, e431, e412
            (Simd32x3::from(other[e4]) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * other.group7()) + (other.group1().zxy() * self.group0().yzx())
                - (other.group1().yzx() * self.group0().zxy()),
            // e235, e315, e125
            (Simd32x3::from(other[e5]) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * other.group8()) + (other.group1().yzx() * self.group2().zxy())
                - (other.group1().zxy() * self.group2().yzx()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e35] * self[e42]) + (other[e45] * self[e23]) + (other[e23] * self[e45]) + (other[e4235] * self[scalar]),
                (other[e15] * self[e43]) + (other[e45] * self[e31]) + (other[e31] * self[e45]) + (other[e4315] * self[scalar]),
                (other[e25] * self[e41]) + (other[e45] * self[e12]) + (other[e12] * self[e45]) + (other[e4125] * self[scalar]),
                -(other[e15] * self[e23]) - (other[e25] * self[e31]) - (other[e35] * self[e12]) - (other[e12] * self[e35]),
            ]) + (Simd32x4::from(other[scalar]) * self.group3())
                + (other.group4().yzx() * self.group2().zxy()).with_w(other[e3215] * self[scalar])
                - (self.group2().yzxx() * other.group4().zxy().with_w(other[e23]))
                - (other.group3().yzx() * self.group0().zxy()).with_w(other[e31] * self[e25]),
            // e1234
            (other[scalar] * self[e1234]) + (other[e1234] * self[scalar])
                - (other[e41] * self[e23])
                - (other[e42] * self[e31])
                - (other[e43] * self[e12])
                - (other[e23] * self[e41])
                - (other[e31] * self[e42])
                - (other[e12] * self[e43]),
        )
    }
}
impl Wedge<Plane> for VersorOdd {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self[scalar]) * other.group0())
    }
}
impl Wedge<RoundPoint> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       18        0
    //    simd3        2        3        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       11       25        0
    //  no simd       24       43        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                other[e2] * self[e43] * -1.0,
                other[e3] * self[e41] * -1.0,
                other[e1] * self[e42] * -1.0,
                (other[e3] * self[e4125]) + (other[e4] * self[e3215]) + (other[e5] * self[e1234]),
            ]) + (other.group0().zxyx() * self.group0().yzx().with_w(self[e4235]))
                + (other.group0().wwwy() * self.group1().xyz().with_w(self[e4315])),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other[e4] * self[e15]) + (other[e5] * self[e41]),
                (other[e4] * self[e25]) + (other[e5] * self[e42]),
                (other[e4] * self[e35]) + (other[e5] * self[e43]),
                -(other[e2] * self[e31]) - (other[e3] * self[e12]),
            ]) - (other.group0().xyzx() * self.group1().wwwx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(other[e5]) * self.group1().xyz()) + (other.group0().yzx() * self.group2().zxy()) - (other.group0().zxy() * self.group2().yzx()))
                .with_w(other[e5] * self[scalar]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group0(),
        )
    }
}
impl Wedge<Scalar> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn wedge(self, other: Scalar) -> Self::Output {
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
impl Wedge<Sphere> for VersorOdd {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group0(),
            // e1234
            other[e1234] * self[scalar],
        )
    }
}
impl Wedge<VersorEven> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       23        0
    //    simd3        3        6        0
    //    simd4        6        5        0
    // Totals...
    // yes simd       24       34        0
    //  no simd       48       61        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from(self[scalar]) * other.group0())
                + (other.group3().wwwx() * self.group1().xyz().with_w(self[e4235]))
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
                + (other.group3().zxy() * self.group0().yzx()).with_w(other[e5] * self[e1234])
                - (other.group3().yzx() * self.group0().zxy()).with_w(other[e423] * self[e15]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other[e5] * self[e41]) + (other[e4] * self[e15]),
                (other[e5] * self[e42]) + (other[e4] * self[e25]),
                (other[e5] * self[e43]) + (other[e4] * self[e35]),
                -(other[e2] * self[e31]) - (other[e3] * self[e12]),
            ]) + (Simd32x4::from(self[scalar]) * other.group1())
                - (other.group3().xyzx() * self.group1().wwwx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(other[e5]) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * other.group2().xyz()) + (other.group3().yzx() * self.group2().zxy())
                - (other.group3().zxy() * self.group2().yzx()))
            .with_w(other[e5] * self[scalar]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group3(),
        )
    }
}
impl Wedge<VersorOdd> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       25        0
    //    simd3        1        4        0
    //    simd4        7        6        0
    // Totals...
    // yes simd       25       35        0
    //  no simd       48       61        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x3::from(other[scalar]) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * other.group0().xyz())).with_w(other[scalar] * self[scalar]),
            // e23, e31, e12, e45
            (Simd32x4::from(other[scalar]) * self.group1()) + (Simd32x4::from(self[scalar]) * other.group1()),
            // e15, e25, e35, e1234
            (Simd32x4::from(other[scalar]) * self.group2())
                + (Simd32x4::from(self[scalar]) * other.group2())
                + Simd32x3::from(0.0).with_w(
                    -(other[e41] * self[e23])
                        - (other[e42] * self[e31])
                        - (other[e43] * self[e12])
                        - (other[e23] * self[e41])
                        - (other[e31] * self[e42])
                        - (other[e12] * self[e43]),
                ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other[e23] * self[e45]) + (other[e45] * self[e23]) + (other[e35] * self[e42]) + (other[e4235] * self[scalar]),
                (other[e31] * self[e45]) + (other[e45] * self[e31]) + (other[e15] * self[e43]) + (other[e4315] * self[scalar]),
                (other[e12] * self[e45]) + (other[e45] * self[e12]) + (other[e25] * self[e41]) + (other[e4125] * self[scalar]),
                -(other[e12] * self[e35]) - (other[e15] * self[e23]) - (other[e25] * self[e31]) - (other[e35] * self[e12]),
            ]) + (other.group0().yzxw() * self.group2().zxy().with_w(self[e3215]))
                + (self.group3().xyz() * other.group0().www()).with_w(other[e3215] * self[scalar])
                - (self.group2().yzxx() * other.group0().zxy().with_w(other[e23]))
                - (other.group2().yzx() * self.group0().zxy()).with_w(other[e31] * self[e25]),
        )
    }
}
