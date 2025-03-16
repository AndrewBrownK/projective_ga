// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 95
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       1       0
//  Maximum:         0       8       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       4       0
//  Average:         0       5       0
//  Maximum:         0      20       0
impl Complement for AntiCircleOnOrigin {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn complement(self) -> Self::Output {
        Line::from_groups(
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group0() * Simd32x3::from(-1.0),
        )
    }
}
impl Complement for AntiCircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            self.group2().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            (self.group0() * Simd32x3::from(-1.0)).with_w(self[scalar]),
        )
    }
}
impl Complement for AntiCircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group2().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            (self.group0() * Simd32x3::from(-1.0)).with_w(self[scalar]),
        )
    }
}
impl Complement for AntiCircleRotorAligningOriginAtInfinity {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn complement(self) -> Self::Output {
        CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
        )
    }
}
impl Complement for AntiCircleRotorAtInfinity {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            self.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            Simd32x3::from(0.0).with_w(self[scalar]),
        )
    }
}
impl Complement for AntiCircleRotorOnOrigin {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn complement(self) -> Self::Output {
        CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl Complement for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            (self.group0() * Simd32x3::from(-1.0)).with_w(self[e5]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        )
    }
}
impl Complement for AntiDipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(self[e5]),
            // e4235, e4315, e4125, e3215
            self.group2().xyz().with_w(0.0),
        )
    }
}
impl Complement for AntiDipoleInversionOnOrigin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn complement(self) -> Self::Output {
        Flector::from_groups(
            // e15, e25, e35, e45
            self.group0() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group1().yzwx(),
        )
    }
}
impl Complement for AntiDipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn complement(self) -> Self::Output {
        DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35, e1234
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl Complement for AntiDipoleOnOrigin {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn complement(self) -> Self::Output {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl Complement for AntiDualNum {
    type Output = Motor;
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).with_w(self[scalar]),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(self[e1234]),
        )
    }
}
impl Complement for AntiFlatOrigin {
    type Output = FlatOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e321] * -1.0)
    }
}
impl Complement for AntiFlatPoint {
    type Output = DipoleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn complement(self) -> Self::Output {
        DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl Complement for AntiFlector {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn complement(self) -> Self::Output {
        DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from(-1.0),
            // e1234, e4235, e4315, e4125
            self.group1().wxyz(),
        )
    }
}
impl Complement for AntiFlectorOnOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn complement(self) -> Self::Output {
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]))
    }
}
impl Complement for AntiLine {
    type Output = CircleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn complement(self) -> Self::Output {
        CircleOnOrigin::from_groups(
            // e423, e431, e412
            self.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
        )
    }
}
impl Complement for AntiLineOnOrigin {
    type Output = LineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn complement(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl Complement for AntiMotor {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            (self.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[scalar]),
            // e415, e425, e435, e4
            (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e3215]),
        )
    }
}
impl Complement for AntiMotorOnOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn complement(self) -> Self::Output {
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl Complement for AntiMysteryCircleRotor {
    type Output = MysteryCircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        MysteryCircleRotor::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0), /* e12345 */ self[scalar])
    }
}
impl Complement for AntiMysteryDipoleInversion {
    type Output = MysteryDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn complement(self) -> Self::Output {
        MysteryDipoleInversion::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125 */ self.group1())
    }
}
impl Complement for AntiPlane {
    type Output = SphereOnOrigin;
    fn complement(self) -> Self::Output {
        SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ self.group0())
    }
}
impl Complement for AntiPlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn complement(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0())
    }
}
impl Complement for AntiScalar {
    type Output = Scalar;
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e12345])
    }
}
impl Complement for AntiSphereOnOrigin {
    type Output = Plane;
    fn complement(self) -> Self::Output {
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0())
    }
}
impl Complement for AntiVersorEvenOnOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (self.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[scalar]),
            // e235, e315, e125, e5
            (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]),
        )
    }
}
impl Complement for Circle {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn complement(self) -> Self::Output {
        Dipole::from_groups(
            // e41, e42, e43
            self.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group0() * Simd32x3::from(-1.0),
        )
    }
}
impl Complement for CircleAligningOrigin {
    type Output = DipoleOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn complement(self) -> Self::Output {
        DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            self.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group0() * Simd32x3::from(-1.0),
        )
    }
}
impl Complement for CircleAtInfinity {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn complement(self) -> Self::Output {
        Dipole::from_groups(
            // e41, e42, e43
            self.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
        )
    }
}
impl Complement for CircleAtOrigin {
    type Output = DipoleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn complement(self) -> Self::Output {
        DipoleAtOrigin::from_groups(
            // e41, e42, e43
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group0() * Simd32x3::from(-1.0),
        )
    }
}
impl Complement for CircleOnOrigin {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn complement(self) -> Self::Output {
        AntiLine::from_groups(
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group0() * Simd32x3::from(-1.0),
        )
    }
}
impl Complement for CircleOrthogonalOrigin {
    type Output = DipoleAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            (self.group1() * Simd32x3::from(-1.0)).with_w(self[e321] * -1.0),
            // e15, e25, e35
            self.group0().xyz() * Simd32x3::from(-1.0),
        )
    }
}
impl Complement for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            (self.group0() * Simd32x3::from(-1.0)).with_w(self[e12345]),
        )
    }
}
impl Complement for CircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group2().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35, scalar
            (self.group0() * Simd32x3::from(-1.0)).with_w(self[e12345]),
        )
    }
}
impl Complement for CircleRotorAligningOriginAtInfinity {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn complement(self) -> Self::Output {
        AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
        )
    }
}
impl Complement for CircleRotorAtInfinity {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(self[e12345]),
        )
    }
}
impl Complement for CircleRotorOnOrigin {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn complement(self) -> Self::Output {
        AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl Complement for Dipole {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn complement(self) -> Self::Output {
        Circle::from_groups(
            // e423, e431, e412
            self.group2() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group0() * Simd32x3::from(-1.0),
        )
    }
}
impl Complement for DipoleAligningOrigin {
    type Output = CircleOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            (self.group1() * Simd32x3::from(-1.0)).with_w(self[e45] * -1.0),
            // e235, e315, e125
            self.group0().xyz() * Simd32x3::from(-1.0),
        )
    }
}
impl Complement for DipoleAtInfinity {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn complement(self) -> Self::Output {
        Circle::from_groups(
            // e423, e431, e412
            self.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
        )
    }
}
impl Complement for DipoleAtOrigin {
    type Output = CircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn complement(self) -> Self::Output {
        CircleAtOrigin::from_groups(
            // e423, e431, e412
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group0() * Simd32x3::from(-1.0),
        )
    }
}
impl Complement for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group2().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e4
            (self.group0() * Simd32x3::from(-1.0)).with_w(self[e3215]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e1234]]),
        )
    }
}
impl Complement for DipoleInversionAligningOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            (self.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[e45] * -1.0),
            // e235, e315, e125, e5
            (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]),
            // e1, e2, e3, e4
            self.group2(),
        )
    }
}
impl Complement for DipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(self[e3215]),
            // e1, e2, e3, e5
            self.group2().xyz().with_w(0.0),
        )
    }
}
impl Complement for DipoleInversionAtOrigin {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            (self.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[e3215]),
            // e235, e315, e125, e5
            (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]),
        )
    }
}
impl Complement for DipoleInversionOnOrigin {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn complement(self) -> Self::Output {
        AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0), /* e1, e2, e3, e5 */ self.group1().yzwx())
    }
}
impl Complement for DipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn complement(self) -> Self::Output {
        AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e4
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl Complement for DipoleOnOrigin {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn complement(self) -> Self::Output {
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl Complement for DipoleOrthogonalOrigin {
    type Output = CircleAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn complement(self) -> Self::Output {
        CircleAligningOrigin::from_groups(
            // e423, e431, e412
            self.group2() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group0() * Simd32x3::from(-1.0),
        )
    }
}
impl Complement for DualNum {
    type Output = AntiMotor;
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(self[e12345]),
            // e15, e25, e35, e3215
            Simd32x3::from(0.0).with_w(self[e4]),
        )
    }
}
impl Complement for FlatOrigin {
    type Output = AntiFlatOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e45] * -1.0)
    }
}
impl Complement for FlatPoint {
    type Output = AntiDipoleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn complement(self) -> Self::Output {
        AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl Complement for FlatPointAtInfinity {
    type Output = NullCircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn complement(self) -> Self::Output {
        NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl Complement for Flector {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn complement(self) -> Self::Output {
        AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0), /* e4, e1, e2, e3 */ self.group1().wxyz())
    }
}
impl Complement for FlectorAtInfinity {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn complement(self) -> Self::Output {
        NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl Complement for FlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn complement(self) -> Self::Output {
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]))
    }
}
impl Complement for Horizon {
    type Output = Origin;
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e3215])
    }
}
impl Complement for Infinity {
    type Output = NullSphereAtOrigin;
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ self[e5])
    }
}
impl Complement for Line {
    type Output = AntiCircleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn complement(self) -> Self::Output {
        AntiCircleOnOrigin::from_groups(
            // e41, e42, e43
            self.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
        )
    }
}
impl Complement for LineAtInfinity {
    type Output = NullDipoleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn complement(self) -> Self::Output {
        NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl Complement for LineOnOrigin {
    type Output = AntiLineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn complement(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl Complement for Motor {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            (self.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[e12345]),
            // e23, e31, e12, e1234
            (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e5]),
        )
    }
}
impl Complement for MotorAtInfinity {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn complement(self) -> Self::Output {
        NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl Complement for MotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn complement(self) -> Self::Output {
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl Complement for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        6        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       20        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0().yx(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]),
            // e5
            self[e1234],
            // e41, e42, e43, e45
            (self.group8() * Simd32x3::from(-1.0)).with_w(self[e321] * -1.0),
            // e15, e25, e35
            self.group7() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group6().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            (self.group5() * Simd32x3::from(-1.0)).with_w(self[e45] * -1.0),
            // e423, e431, e412
            self.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group3().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e5], self[e1], self[e2], self[e3]]),
            // e3215
            self[e4],
        )
    }
}
impl Complement for MysteryCircle {
    type Output = MysteryDipole;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn complement(self) -> Self::Output {
        MysteryDipole::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl Complement for MysteryCircleRotor {
    type Output = AntiMysteryCircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* scalar */ self[e12345])
    }
}
impl Complement for MysteryDipole {
    type Output = MysteryCircle;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn complement(self) -> Self::Output {
        MysteryCircle::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl Complement for MysteryDipoleInversion {
    type Output = AntiMysteryDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn complement(self) -> Self::Output {
        AntiMysteryDipoleInversion::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0), /* e1, e2, e3 */ self.group1())
    }
}
impl Complement for MysteryVersorEven {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn complement(self) -> Self::Output {
        MysteryVersorOdd::from_groups(/* scalar, e4235, e4315, e4125 */ self.group0(), /* e23, e31, e12, e45 */ self.group1() * Simd32x4::from(-1.0))
    }
}
impl Complement for MysteryVersorOdd {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn complement(self) -> Self::Output {
        MysteryVersorEven::from_groups(/* e12345, e1, e2, e3 */ self.group0(), /* e415, e425, e435, e321 */ self.group1() * Simd32x4::from(-1.0))
    }
}
impl Complement for NullCircleAtOrigin {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn complement(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl Complement for NullDipoleAtOrigin {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn complement(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl Complement for NullDipoleInversionAtOrigin {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn complement(self) -> Self::Output {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl Complement for NullSphereAtOrigin {
    type Output = Infinity;
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e1234])
    }
}
impl Complement for NullVersorEvenAtOrigin {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn complement(self) -> Self::Output {
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl Complement for Origin {
    type Output = Horizon;
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e4])
    }
}
impl Complement for Plane {
    type Output = AntiSphereOnOrigin;
    fn complement(self) -> Self::Output {
        AntiSphereOnOrigin::from_groups(/* e1, e2, e3, e4 */ self.group0())
    }
}
impl Complement for PlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn complement(self) -> Self::Output {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0())
    }
}
impl Complement for RoundPoint {
    type Output = Sphere;
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0(), /* e1234 */ self[e5])
    }
}
impl Complement for RoundPointAtOrigin {
    type Output = SphereAtOrigin;
    fn complement(self) -> Self::Output {
        SphereAtOrigin::from_groups(/* e3215, e1234 */ self.group0())
    }
}
impl Complement for Scalar {
    type Output = AntiScalar;
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[scalar])
    }
}
impl Complement for Sphere {
    type Output = RoundPoint;
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ self.group0(), /* e5 */ self[e1234])
    }
}
impl Complement for SphereAtOrigin {
    type Output = RoundPointAtOrigin;
    fn complement(self) -> Self::Output {
        RoundPointAtOrigin::from_groups(/* e4, e5 */ self.group0())
    }
}
impl Complement for SphereOnOrigin {
    type Output = AntiPlane;
    fn complement(self) -> Self::Output {
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0())
    }
}
impl Complement for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group2().xyz() * Simd32x3::from(-1.0)).with_w(self[e12345]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e5]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        )
    }
}
impl Complement for VersorEvenAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            (self.group2().xyz() * Simd32x3::from(-1.0)).with_w(self[e12345]),
            // e23, e31, e12, e3215
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e5]),
        )
    }
}
impl Complement for VersorEvenAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group2().xyz() * Simd32x3::from(-1.0)).with_w(self[e12345]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(self[e5]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e1], self[e2], self[e3], 0.0]),
        )
    }
}
impl Complement for VersorEvenAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            (self.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[e4]),
            // e15, e25, e35, e1234
            (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e5]),
        )
    }
}
impl Complement for VersorEvenOnOrigin {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (self.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[e12345]),
            // e15, e25, e35, e3215
            (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e4]),
        )
    }
}
impl Complement for VersorEvenOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            (self.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[e321] * -1.0),
            // e15, e25, e35, e1234
            (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e5]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        )
    }
}
impl Complement for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group2().xyz() * Simd32x3::from(-1.0)).with_w(self[scalar]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]),
            // e1, e2, e3, e4
            self.group3(),
        )
    }
}
impl Complement for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn complement(self) -> Self::Output {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().yzwx() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            self.group2(),
        )
    }
}
impl Complement for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn complement(self) -> Self::Output {
        use crate::elements::*;
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            (self.group2().xyz() * Simd32x3::from(-1.0)).with_w(self[scalar]),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]),
        )
    }
}
