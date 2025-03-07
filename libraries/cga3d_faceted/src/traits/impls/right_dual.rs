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
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = CircleOnOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
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
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiCircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = CircleRotorAtInfinity;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiDipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn right_dual(self) -> Self::Output {
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
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiDipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_dual(self) -> Self::Output {
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
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiDipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_dual(self) -> Self::Output {
        DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234, e4235, e4315, e4125
            self.group1() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
        )
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiDipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_dual(self) -> Self::Output {
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
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = DipoleOnOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiDipoleOnOrigin {
    type Output = DipoleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiDualNum {
    type Output = DualNum;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiDualNum {
    type Output = DualNum;
    fn right_dual(self) -> Self::Output {
        DualNum::from_groups(/* e4, e12345 */ self.group0())
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiFlatOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiFlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiFlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiFlector {
    type Output = Flector;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiFlector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_dual(self) -> Self::Output {
        Flector::from_groups(
            // e15, e25, e35, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiFlectorOnOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiLine {
    type Output = Line;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiLineOnOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiMotor {
    type Output = Motor;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiMotorOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = MysteryCircleRotor;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = MysteryDipoleInversion;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiMysteryDipoleInversion {
    type Output = MysteryDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        MysteryDipoleInversion::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125
            self.group1(),
        )
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiPlane {
    type Output = Plane;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiPlane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiPlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiPlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn right_dual(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0())
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiScalar {
    type Output = Scalar;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for AntiSphereOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = VersorEvenOnOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for Circle {
    type Output = Dipole;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for Circle {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
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
impl std::ops::Div<RightDualPrefixOrPostfix> for CircleAligningOrigin {
    type Output = DipoleOrthogonalOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for CircleAligningOrigin {
    type Output = DipoleOrthogonalOrigin;
    fn right_dual(self) -> Self::Output {
        DipoleOrthogonalOrigin::from_groups(/* e41, e42, e43 */ self.group0(), /* e23, e31, e12 */ self.group1(), /* e15, e25, e35 */ self.group2())
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for CircleAtInfinity {
    type Output = DipoleAtInfinity;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for CircleAtInfinity {
    type Output = DipoleAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group1(),
        )
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for CircleAtOrigin {
    type Output = DipoleAtOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for CircleAtOrigin {
    type Output = DipoleAtOrigin;
    fn right_dual(self) -> Self::Output {
        DipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0(), /* e15, e25, e35 */ self.group1())
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for CircleOnOrigin {
    type Output = AntiCircleOnOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for CircleOnOrigin {
    type Output = AntiCircleOnOrigin;
    fn right_dual(self) -> Self::Output {
        AntiCircleOnOrigin::from_groups(/* e41, e42, e43 */ self.group0(), /* e23, e31, e12 */ self.group1())
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = DipoleAligningOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for CircleOrthogonalOrigin {
    type Output = DipoleAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group1(),
        )
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for CircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_dual(self) -> Self::Output {
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
impl std::ops::Div<RightDualPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for CircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
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
impl std::ops::Div<RightDualPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for CircleRotorAligningOriginAtInfinity {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = AntiCircleRotorAtInfinity;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for CircleRotorAtInfinity {
    type Output = AntiCircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_dual(self) -> Self::Output {
        AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for CircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12
            self.group1(),
        )
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for Dipole {
    type Output = Circle;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = CircleOrthogonalOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for DipoleAtInfinity {
    type Output = CircleAtInfinity;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for DipoleAtOrigin {
    type Output = CircleAtOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for DipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for DipoleInversionAligningOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn right_dual(self) -> Self::Output {
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
impl std::ops::Div<RightDualPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = VersorEvenAtOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for DipoleInversionAtOrigin {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            self.group0().xyz().with_w(self[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(self[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for DipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = CircleAligningOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for DualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for FlatOrigin {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for FlatOrigin {
    type Output = AntiFlatOrigin;
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e45])
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for FlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for FlatPointAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for Flector {
    type Output = AntiFlector;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for FlectorAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for FlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for Horizon {
    type Output = Infinity;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for Horizon {
    type Output = Infinity;
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e3215])
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for Infinity {
    type Output = Horizon;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for Line {
    type Output = AntiLine;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for Line {
    type Output = AntiLine;
    fn right_dual(self) -> Self::Output {
        AntiLine::from_groups(/* e23, e31, e12 */ self.group0(), /* e15, e25, e35 */ self.group1())
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for LineAtInfinity {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for LineAtInfinity {
    type Output = FlatPointAtInfinity;
    fn right_dual(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0())
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for LineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for LineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn right_dual(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0())
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for Motor {
    type Output = AntiMotor;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_dual(self) -> Self::Output {
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for MotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for MotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl std::ops::DivAssign<RightDualPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: RightDualPrefixOrPostfix) {
        *self = self.right_dual()
    }
}
impl RightDual for MultiVector {
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
impl std::ops::Div<RightDualPrefixOrPostfix> for MysteryCircle {
    type Output = MysteryDipole;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for MysteryCircle {
    type Output = MysteryDipole;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        MysteryDipole::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for MysteryCircleRotor {
    type Output = AntiMysteryCircleRotor;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for MysteryCircleRotor {
    type Output = AntiMysteryCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        AntiMysteryCircleRotor::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // scalar
            self[e12345] * -1.0,
        )
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for MysteryDipole {
    type Output = MysteryCircle;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for MysteryDipoleInversion {
    type Output = AntiMysteryDipoleInversion;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for MysteryVersorEven {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for MysteryVersorEven {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_dual(self) -> Self::Output {
        MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for MysteryVersorOdd {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for NullCircleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for NullCircleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn right_dual(self) -> Self::Output {
        NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0())
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for NullDipoleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for NullDipoleInversionAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for NullSphereAtOrigin {
    type Output = Origin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for NullSphereAtOrigin {
    type Output = Origin;
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e1234])
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for NullVersorEvenAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for NullVersorEvenAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for Origin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for Plane {
    type Output = AntiPlane;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for PlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for RoundPoint {
    type Output = Sphere;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for RoundPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            self.group0().xyz().with_w(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            self[e4] * -1.0,
        )
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = SphereAtOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for Scalar {
    type Output = AntiScalar;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for Scalar {
    type Output = AntiScalar;
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[scalar])
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for Sphere {
    type Output = RoundPoint;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            self.group0().xyz().with_w(self[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            self[e3215],
        )
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for SphereAtOrigin {
    type Output = RoundPointAtOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for SphereAtOrigin {
    type Output = RoundPointAtOrigin;
    fn right_dual(self) -> Self::Output {
        RoundPointAtOrigin::from_groups(/* e4, e5 */ self.group0().yx())
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for SphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
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
impl std::ops::Div<RightDualPrefixOrPostfix> for VersorEven {
    type Output = VersorOdd;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn right_dual(self) -> Self::Output {
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
impl std::ops::Div<RightDualPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for VersorEvenAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn right_dual(self) -> Self::Output {
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
impl std::ops::Div<RightDualPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn right_dual(self) -> Self::Output {
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
impl std::ops::Div<RightDualPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = DipoleInversionAtOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for VersorEvenAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_dual(self) -> Self::Output {
        use crate::elements::*;
        DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0().xyz().with_w(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group1().xyz().with_w(self[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for VersorEvenOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_dual(self) -> Self::Output {
        AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e1234
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<RightDualPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for VersorEvenOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn right_dual(self) -> Self::Output {
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
impl std::ops::Div<RightDualPrefixOrPostfix> for VersorOdd {
    type Output = VersorEven;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn right_dual(self) -> Self::Output {
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
impl std::ops::Div<RightDualPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn right_dual(self) -> Self::Output {
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
impl std::ops::Div<RightDualPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    fn div(self, _rhs: RightDualPrefixOrPostfix) -> Self::Output {
        self.right_dual()
    }
}
impl RightDual for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn right_dual(self) -> Self::Output {
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
