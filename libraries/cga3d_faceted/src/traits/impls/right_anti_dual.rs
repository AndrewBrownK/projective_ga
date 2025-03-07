// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
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
//  Maximum:         0      22       0
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = CircleOnOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiCircleOnOrigin {
    type Output = CircleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn right_anti_dual(self) -> Self::Output {
        CircleOnOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiCircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiCircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn right_anti_dual(self) -> Self::Output {
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
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiCircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn right_anti_dual(self) -> Self::Output {
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
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiCircleRotorAligningOriginAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn right_anti_dual(self) -> Self::Output {
        CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = CircleRotorAtInfinity;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiCircleRotorAtInfinity {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiCircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn right_anti_dual(self) -> Self::Output {
        CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiDipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn right_anti_dual(self) -> Self::Output {
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiDipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiDipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234, e4235, e4315, e4125
            self.group1() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiDipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = DipoleOnOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiDipoleOnOrigin {
    type Output = DipoleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiDualNum {
    type Output = DualNum;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiDualNum {
    type Output = DualNum;
    fn right_anti_dual(self) -> Self::Output {
        DualNum::from_groups(/* e4, e12345 */ self.group0())
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiFlatOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiFlatOrigin {
    type Output = FlatOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e321] * -1.0)
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiFlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiFlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiFlector {
    type Output = Flector;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiFlector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        Flector::from_groups(
            // e15, e25, e35, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiFlectorOnOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiFlectorOnOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiLine {
    type Output = Line;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiLine {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn right_anti_dual(self) -> Self::Output {
        Line::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiLineOnOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiLineOnOrigin {
    type Output = LineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn right_anti_dual(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiMotor {
    type Output = Motor;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiMotorOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiMotorOnOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = MysteryCircleRotor;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiMysteryCircleRotor {
    type Output = MysteryCircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        MysteryCircleRotor::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e12345
            self[scalar],
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = MysteryDipoleInversion;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiMysteryDipoleInversion {
    type Output = MysteryDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        MysteryDipoleInversion::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125
            self.group1(),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiPlane {
    type Output = Plane;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiPlane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiPlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiPlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn right_anti_dual(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0())
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiScalar {
    type Output = Scalar;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e12345] * -1.0)
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiSphereOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = VersorEvenOnOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiVersorEvenOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Circle {
    type Output = Dipole;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Circle {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        Dipole::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group2(),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for CircleAligningOrigin {
    type Output = DipoleOrthogonalOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for CircleAligningOrigin {
    type Output = DipoleOrthogonalOrigin;
    fn right_anti_dual(self) -> Self::Output {
        DipoleOrthogonalOrigin::from_groups(/* e41, e42, e43 */ self.group0(), /* e23, e31, e12 */ self.group1(), /* e15, e25, e35 */ self.group2())
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for CircleAtInfinity {
    type Output = DipoleAtInfinity;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for CircleAtInfinity {
    type Output = DipoleAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group1(),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for CircleAtOrigin {
    type Output = DipoleAtOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for CircleAtOrigin {
    type Output = DipoleAtOrigin;
    fn right_anti_dual(self) -> Self::Output {
        DipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0(), /* e15, e25, e35 */ self.group1())
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for CircleOnOrigin {
    type Output = AntiCircleOnOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for CircleOnOrigin {
    type Output = AntiCircleOnOrigin;
    fn right_anti_dual(self) -> Self::Output {
        AntiCircleOnOrigin::from_groups(/* e41, e42, e43 */ self.group0(), /* e23, e31, e12 */ self.group1())
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = DipoleAligningOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for CircleOrthogonalOrigin {
    type Output = DipoleAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group1(),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for CircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for CircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for CircleRotorAligningOriginAtInfinity {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = AntiCircleRotorAtInfinity;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for CircleRotorAtInfinity {
    type Output = AntiCircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for CircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12
            self.group1(),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Dipole {
    type Output = Circle;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Dipole {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn right_anti_dual(self) -> Self::Output {
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
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = CircleOrthogonalOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DipoleAligningOrigin {
    type Output = CircleOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn right_anti_dual(self) -> Self::Output {
        CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for DipoleAtInfinity {
    type Output = CircleAtInfinity;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DipoleAtInfinity {
    type Output = CircleAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn right_anti_dual(self) -> Self::Output {
        CircleAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for DipoleAtOrigin {
    type Output = CircleAtOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DipoleAtOrigin {
    type Output = CircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn right_anti_dual(self) -> Self::Output {
        CircleAtOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for DipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn right_anti_dual(self) -> Self::Output {
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
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DipoleInversionAligningOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(self[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(self[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn right_anti_dual(self) -> Self::Output {
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
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = VersorEvenAtOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DipoleInversionAtOrigin {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            self.group0().xyz().with_w(self[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(self[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4, e1, e2, e3
            self.group1() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn right_anti_dual(self) -> Self::Output {
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
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for DipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = CircleAligningOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DipoleOrthogonalOrigin {
    type Output = CircleAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn right_anti_dual(self) -> Self::Output {
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
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for DualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn right_anti_dual(self) -> Self::Output {
        AntiDualNum::from_groups(/* e1234, scalar */ self.group0() * Simd32x2::from(-1.0))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for FlatOrigin {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for FlatOrigin {
    type Output = AntiFlatOrigin;
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e45])
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for FlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for FlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for FlatPointAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for FlatPointAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn right_anti_dual(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Flector {
    type Output = AntiFlector;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for FlectorAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for FlectorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for FlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for FlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Horizon {
    type Output = Infinity;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Horizon {
    type Output = Infinity;
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e3215])
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Infinity {
    type Output = Horizon;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Infinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e5] * -1.0)
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Line {
    type Output = AntiLine;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Line {
    type Output = AntiLine;
    fn right_anti_dual(self) -> Self::Output {
        AntiLine::from_groups(/* e23, e31, e12 */ self.group0(), /* e15, e25, e35 */ self.group1())
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for LineAtInfinity {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for LineAtInfinity {
    type Output = FlatPointAtInfinity;
    fn right_anti_dual(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0())
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for LineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for LineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn right_anti_dual(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0())
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Motor {
    type Output = AntiMotor;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for MotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for MotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl std::ops::DivAssign<RightAntiDualPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: RightAntiDualPrefixOrPostfix) {
        *self = self.right_anti_dual()
    }
}
impl RightAntiDual for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       22        0
    fn right_anti_dual(self) -> Self::Output {
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
            self.group5().with_w(self[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
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
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for MysteryCircle {
    type Output = MysteryDipole;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for MysteryCircle {
    type Output = MysteryDipole;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        MysteryDipole::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for MysteryCircleRotor {
    type Output = AntiMysteryCircleRotor;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for MysteryCircleRotor {
    type Output = AntiMysteryCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        AntiMysteryCircleRotor::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // scalar
            self[e12345] * -1.0,
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for MysteryDipole {
    type Output = MysteryCircle;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for MysteryDipole {
    type Output = MysteryCircle;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        MysteryCircle::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for MysteryDipoleInversion {
    type Output = AntiMysteryDipoleInversion;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for MysteryDipoleInversion {
    type Output = AntiMysteryDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn right_anti_dual(self) -> Self::Output {
        AntiMysteryDipoleInversion::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for MysteryVersorEven {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for MysteryVersorEven {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for MysteryVersorOdd {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for MysteryVersorOdd {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for NullCircleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for NullCircleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn right_anti_dual(self) -> Self::Output {
        NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0())
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for NullDipoleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for NullDipoleAtOrigin {
    type Output = NullCircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn right_anti_dual(self) -> Self::Output {
        NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for NullDipoleInversionAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for NullDipoleInversionAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for NullSphereAtOrigin {
    type Output = Origin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for NullSphereAtOrigin {
    type Output = Origin;
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e1234])
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for NullVersorEvenAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for NullVersorEvenAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Origin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Origin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ self[e4] * -1.0)
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Plane {
    type Output = AntiPlane;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Plane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for PlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for PlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn right_anti_dual(self) -> Self::Output {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for RoundPoint {
    type Output = Sphere;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for RoundPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            self.group0().xyz().with_w(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            self[e4] * -1.0,
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = SphereAtOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for RoundPointAtOrigin {
    type Output = SphereAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn right_anti_dual(self) -> Self::Output {
        SphereAtOrigin::from_groups(/* e3215, e1234 */ self.group0().yx() * Simd32x2::from(-1.0))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Scalar {
    type Output = AntiScalar;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Scalar {
    type Output = AntiScalar;
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[scalar])
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Sphere {
    type Output = RoundPoint;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            self.group0().xyz().with_w(self[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            self[e3215],
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for SphereAtOrigin {
    type Output = RoundPointAtOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for SphereAtOrigin {
    type Output = RoundPointAtOrigin;
    fn right_anti_dual(self) -> Self::Output {
        RoundPointAtOrigin::from_groups(/* e4, e5 */ self.group0().yx())
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for SphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for SphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        AntiSphereOnOrigin::from_groups(/* e1, e2, e3, e4 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for VersorEven {
    type Output = VersorOdd;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(self[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for VersorEvenAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e3215
            self.group1().xyz().with_w(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(self[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([self[e12345], self[e235], self[e315], self[e125]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group0().yzw().with_w(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = DipoleInversionAtOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for VersorEvenAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0().xyz().with_w(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group1().xyz().with_w(self[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for VersorEvenOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e1234
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for VersorEvenOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group1().xyz().with_w(self[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group2().xyz().with_w(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for VersorOdd {
    type Output = VersorEven;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3().xyz().with_w(self[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([self[scalar], self[e4235], self[e4315], self[e4125]]) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group0().yzw().with_w(self[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e4
            self.group1().xyz().with_w(self[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
