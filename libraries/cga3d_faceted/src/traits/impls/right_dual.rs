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
//   Median:         0       3       0
//  Average:         0       4       0
//  Maximum:         0      21       0
impl RightDual for AntiCircleOnOrigin {
    type Output = CircleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn right_dual(self) -> Self::Output {
        CircleOnOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl RightDual for AntiCircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn right_dual(self) -> Self::Output {
        CircleRotor::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl RightDual for AntiCircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn right_dual(self) -> Self::Output {
        CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl RightDual for AntiCircleRotorAligningOriginAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn right_dual(self) -> Self::Output {
        CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl RightDual for AntiCircleRotorAtInfinity {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_dual(self) -> Self::Output {
        CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl RightDual for AntiCircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn right_dual(self) -> Self::Output {
        CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl RightDual for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e321] * -1.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(self[e4] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(self[e5] * -1.0),
        )
    }
}
impl RightDual for AntiDipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0().xyz().with_w(self[e321] * -1.0),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            self.group2().xyz().with_w(self[e5] * -1.0),
        )
    }
}
impl RightDual for AntiDipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0().xyz().with_w(self[e321] * -1.0),
            // e1234, e4235, e4315, e4125
            self.group1() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
        )
    }
}
impl RightDual for AntiDipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0().xyz().with_w(self[e5] * -1.0),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(self[e4] * -1.0),
        )
    }
}
impl RightDual for AntiDipoleOnOrigin {
    type Output = DipoleOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0().xyz().with_w(self[e321] * -1.0))
    }
}
impl RightDual for AntiDualNum {
    type Output = DualNum;
    fn right_dual(self) -> Self::Output {
        DualNum::from_groups(/* e4, e12345 */ self.group0())
    }
}
impl RightDual for AntiFlatOrigin {
    type Output = FlatOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e321] * -1.0)
    }
}
impl RightDual for AntiFlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0().xyz().with_w(self[e321] * -1.0))
    }
}
impl RightDual for AntiFlector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            self.group0().xyz().with_w(self[e321] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group1().xyz().with_w(self[e5] * -1.0),
        )
    }
}
impl RightDual for AntiFlectorOnOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]))
    }
}
impl RightDual for AntiLine {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn right_dual(self) -> Self::Output {
        Line::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl RightDual for AntiLineOnOrigin {
    type Output = LineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn right_dual(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl RightDual for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_dual(self) -> Self::Output {
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl RightDual for AntiMotorOnOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl RightDual for AntiMysteryCircleRotor {
    type Output = MysteryCircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        MysteryCircleRotor::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e12345
            self[scalar],
        )
    }
}
impl RightDual for AntiMysteryDipoleInversion {
    type Output = MysteryDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        MysteryDipoleInversion::from_groups(
            // e23, e31, e12, e45
            self.group0().xyz().with_w(self[e321] * -1.0),
            // e4235, e4315, e4125
            self.group1(),
        )
    }
}
impl RightDual for AntiPlane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0().xyz().with_w(self[e5] * -1.0))
    }
}
impl RightDual for AntiPlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn right_dual(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0())
    }
}
impl RightDual for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e12345] * -1.0)
    }
}
impl RightDual for AntiSphereOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ self.group0().xyz().with_w(self[e4] * -1.0))
    }
}
impl RightDual for AntiVersorEvenOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_dual(self) -> Self::Output {
        VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl RightDual for Circle {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e321] * -1.0),
            // e15, e25, e35
            self.group2(),
        )
    }
}
impl RightDual for CircleAligningOrigin {
    type Output = DipoleOrthogonalOrigin;
    fn right_dual(self) -> Self::Output {
        DipoleOrthogonalOrigin::from_groups(/* e41, e42, e43 */ self.group0(), /* e23, e31, e12 */ self.group1(), /* e15, e25, e35 */ self.group2())
    }
}
impl RightDual for CircleAtInfinity {
    type Output = DipoleAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        DipoleAtInfinity::from_groups(/* e23, e31, e12, e45 */ self.group0().xyz().with_w(self[e321] * -1.0), /* e15, e25, e35 */ self.group1())
    }
}
impl RightDual for CircleAtOrigin {
    type Output = DipoleAtOrigin;
    fn right_dual(self) -> Self::Output {
        DipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0(), /* e15, e25, e35 */ self.group1())
    }
}
impl RightDual for CircleOnOrigin {
    type Output = AntiCircleOnOrigin;
    fn right_dual(self) -> Self::Output {
        AntiCircleOnOrigin::from_groups(/* e41, e42, e43 */ self.group0(), /* e23, e31, e12 */ self.group1())
    }
}
impl RightDual for CircleOrthogonalOrigin {
    type Output = DipoleAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        DipoleAligningOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0().xyz().with_w(self[e321] * -1.0), /* e15, e25, e35 */ self.group1())
    }
}
impl RightDual for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e321] * -1.0),
            // e15, e25, e35, scalar
            self.group2().xyz().with_w(self[e12345] * -1.0),
        )
    }
}
impl RightDual for CircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            self.group2().xyz().with_w(self[e12345] * -1.0),
        )
    }
}
impl RightDual for CircleRotorAligningOriginAtInfinity {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35, scalar
            self.group1().xyz().with_w(self[e12345] * -1.0),
        )
    }
}
impl RightDual for CircleRotorAtInfinity {
    type Output = AntiCircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0().xyz().with_w(self[e321] * -1.0),
            // e15, e25, e35, scalar
            self.group1().xyz().with_w(self[e12345] * -1.0),
        )
    }
}
impl RightDual for CircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotorOnOrigin::from_groups(/* e41, e42, e43, scalar */ self.group0().xyz().with_w(self[e12345] * -1.0), /* e23, e31, e12 */ self.group1())
    }
}
impl RightDual for Dipole {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn right_dual(self) -> Self::Output {
        Circle::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        )
    }
}
impl RightDual for DipoleAligningOrigin {
    type Output = CircleOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn right_dual(self) -> Self::Output {
        CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl RightDual for DipoleAtInfinity {
    type Output = CircleAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn right_dual(self) -> Self::Output {
        CircleAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl RightDual for DipoleAtOrigin {
    type Output = CircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn right_dual(self) -> Self::Output {
        CircleAtOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl RightDual for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn right_dual(self) -> Self::Output {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl RightDual for DipoleInversionAligningOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            (self.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[e3215]),
            // e1, e2, e3, e4
            (self.group2().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]),
        )
    }
}
impl RightDual for DipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn right_dual(self) -> Self::Output {
        AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
            // e1, e2, e3, e5
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl RightDual for DipoleInversionAtOrigin {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]),
            // e235, e315, e125, e5
            (self.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[e3215]),
        )
    }
}
impl RightDual for DipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_dual(self) -> Self::Output {
        AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4, e1, e2, e3
            self.group1() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
        )
    }
}
impl RightDual for DipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn right_dual(self) -> Self::Output {
        AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl RightDual for DipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl RightDual for DipoleOrthogonalOrigin {
    type Output = CircleAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn right_dual(self) -> Self::Output {
        CircleAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        )
    }
}
impl RightDual for DualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn right_dual(self) -> Self::Output {
        AntiDualNum::from_groups(/* e1234, scalar */ self.group0() * Simd32x2::from(-1.0))
    }
}
impl RightDual for FlatOrigin {
    type Output = AntiFlatOrigin;
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e45])
    }
}
impl RightDual for FlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl RightDual for FlatPointAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn right_dual(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl RightDual for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_dual(self) -> Self::Output {
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl RightDual for FlectorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl RightDual for FlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]))
    }
}
impl RightDual for Horizon {
    type Output = Infinity;
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e3215])
    }
}
impl RightDual for Infinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e5] * -1.0)
    }
}
impl RightDual for Line {
    type Output = AntiLine;
    fn right_dual(self) -> Self::Output {
        AntiLine::from_groups(/* e23, e31, e12 */ self.group0(), /* e15, e25, e35 */ self.group1())
    }
}
impl RightDual for LineAtInfinity {
    type Output = FlatPointAtInfinity;
    fn right_dual(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0())
    }
}
impl RightDual for LineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn right_dual(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0())
    }
}
impl RightDual for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0().xyz().with_w(self[e12345] * -1.0),
            // e15, e25, e35, e3215
            self.group1().xyz().with_w(self[e5] * -1.0),
        )
    }
}
impl RightDual for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0().xyz().with_w(self[e5] * -1.0))
    }
}
impl RightDual for MotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group0().xyz().with_w(self[e12345] * -1.0))
    }
}
impl RightDual for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       21        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group9().yzwx() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            self[e3215],
            // e41, e42, e43, e45
            self.group7().with_w(self[e321] * -1.0),
            // e15, e25, e35
            self.group8(),
            // e23, e31, e12
            self.group6().xyz(),
            // e415, e425, e435, e321
            (self.group5() * Simd32x3::from(-1.0)).with_w(self[e45]),
            // e423, e431, e412
            self.group3().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group4() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            self.group1().wxyz() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e3215
            self[e5] * -1.0,
        )
    }
}
impl RightDual for MysteryCircle {
    type Output = MysteryDipole;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        MysteryDipole::from_groups(/* e23, e31, e12, e45 */ self.group0().xyz().with_w(self[e321] * -1.0))
    }
}
impl RightDual for MysteryCircleRotor {
    type Output = AntiMysteryCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ self.group0().xyz().with_w(self[e321] * -1.0), /* scalar */ self[e12345] * -1.0)
    }
}
impl RightDual for MysteryDipole {
    type Output = MysteryCircle;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        MysteryCircle::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl RightDual for MysteryDipoleInversion {
    type Output = AntiMysteryDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn right_dual(self) -> Self::Output {
        AntiMysteryDipoleInversion::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl RightDual for MysteryVersorEven {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e321] * -1.0),
        )
    }
}
impl RightDual for MysteryVersorOdd {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_dual(self) -> Self::Output {
        MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl RightDual for NullCircleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn right_dual(self) -> Self::Output {
        NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0())
    }
}
impl RightDual for NullDipoleAtOrigin {
    type Output = NullCircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn right_dual(self) -> Self::Output {
        NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl RightDual for NullDipoleInversionAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl RightDual for NullSphereAtOrigin {
    type Output = Origin;
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e1234])
    }
}
impl RightDual for NullVersorEvenAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ self.group0().xyz().with_w(self[e4] * -1.0))
    }
}
impl RightDual for Origin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ self[e4] * -1.0)
    }
}
impl RightDual for Plane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl RightDual for PlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn right_dual(self) -> Self::Output {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl RightDual for RoundPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0().xyz().with_w(self[e5] * -1.0), /* e1234 */ self[e4] * -1.0)
    }
}
impl RightDual for RoundPointAtOrigin {
    type Output = SphereAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn right_dual(self) -> Self::Output {
        SphereAtOrigin::from_groups(/* e3215, e1234 */ self.group0().yx() * Simd32x2::from(-1.0))
    }
}
impl RightDual for Scalar {
    type Output = AntiScalar;
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[scalar])
    }
}
impl RightDual for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]), /* e5 */ self[e3215])
    }
}
impl RightDual for SphereAtOrigin {
    type Output = RoundPointAtOrigin;
    fn right_dual(self) -> Self::Output {
        RoundPointAtOrigin::from_groups(/* e4, e5 */ self.group0().yx())
    }
}
impl RightDual for SphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        AntiSphereOnOrigin::from_groups(/* e1, e2, e3, e4 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl RightDual for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(self[e12345] * -1.0),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e321] * -1.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(self[e4] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(self[e5] * -1.0),
        )
    }
}
impl RightDual for VersorEvenAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(self[e12345] * -1.0),
            // e23, e31, e12, e3215
            self.group1().xyz().with_w(self[e5] * -1.0),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(self[e4] * -1.0),
        )
    }
}
impl RightDual for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([self[e12345], self[e235], self[e315], self[e125]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e321] * -1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e1], self[e2], self[e3], self[e5] * -1.0]),
        )
    }
}
impl RightDual for VersorEvenAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0().xyz().with_w(self[e5] * -1.0),
            // e15, e25, e35, e1234
            self.group1().xyz().with_w(self[e4] * -1.0),
        )
    }
}
impl RightDual for VersorEvenOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(self[e12345] * -1.0),
            // e23, e31, e12, e1234
            self.group1().xyz().with_w(self[e4] * -1.0),
        )
    }
}
impl RightDual for VersorEvenOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0().xyz().with_w(self[e321] * -1.0),
            // e15, e25, e35, e1234
            self.group1().xyz().with_w(self[e4] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group2().xyz().with_w(self[e5] * -1.0),
        )
    }
}
impl RightDual for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       14        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            (self.group2().xyz() * Simd32x3::from(-1.0)).with_w(self[e3215]),
            // e1, e2, e3, e4
            (self.group3().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]),
        )
    }
}
impl RightDual for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([self[scalar], self[e4235], self[e4315], self[e4125]]) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            (self.group0().yzw() * Simd32x3::from(-1.0)).with_w(self[e3215]),
        )
    }
}
impl RightDual for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e4
            (self.group1().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]),
            // e235, e315, e125, e5
            (self.group2().xyz() * Simd32x3::from(-1.0)).with_w(self[e3215]),
        )
    }
}
