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
//  Maximum:         0       6       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       4       0
//  Average:         0       5       0
//  Maximum:         0      20       0
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = AntiCircleOnOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiCircleOnOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_reverse(self) -> Self {
        AntiCircleOnOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiCircleRotor {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_reverse(self) -> Self {
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_reverse(self) -> Self {
        AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = AntiCircleRotorAtInfinity;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiCircleRotorAtInfinity {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reverse(self) -> Self {
        AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiCircleRotorOnOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiDipoleInversion {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_reverse(self) -> Self {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group3(),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
            // e1, e2, e3, e5
            self.group2(),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0), /* e4, e1, e2, e3 */ self.group1())
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_reverse(self) -> Self {
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
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiDipoleOnOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiDualNum {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiDualNum {
    fn anti_reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiFlatOrigin {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiFlatOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiFlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e321] * -1.0)
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiFlector {
    type Output = AntiFlector;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiFlector {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiFlector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0), /* e1, e2, e3, e5 */ self.group1())
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiFlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiFlectorOnOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]))
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiLine {
    type Output = AntiLine;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiLine {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiLine {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_reverse(self) -> Self {
        AntiLine::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiLineOnOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_reverse(self) -> Self {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiMotor {
    type Output = AntiMotor;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiMotor {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiMotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reverse(self) -> Self {
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e3215
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiMotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiMotorOnOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = AntiMysteryCircleRotor;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiMysteryCircleRotor {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* scalar */ self[scalar])
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = AntiMysteryDipoleInversion;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiMysteryDipoleInversion {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        AntiMysteryDipoleInversion::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0), /* e1, e2, e3 */ self.group1())
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiPlane {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiPlane {
    fn anti_reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiPlaneOnOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiPlaneOnOrigin {
    fn anti_reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiScalar {
    fn anti_reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiSphereOnOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiSphereOnOrigin {
    fn anti_reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for AntiVersorEvenOnOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reverse(self) -> Self {
        AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for Circle {
    type Output = Circle;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for Circle {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_reverse(self) -> Self {
        Circle::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for CircleAligningOrigin {
    type Output = CircleAligningOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for CircleAligningOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn anti_reverse(self) -> Self {
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
impl std::ops::Div<AntiReversePrefixOrPostfix> for CircleAtInfinity {
    type Output = CircleAtInfinity;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for CircleAtInfinity {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        CircleAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for CircleAtOrigin {
    type Output = CircleAtOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for CircleAtOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_reverse(self) -> Self {
        CircleAtOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for CircleOnOrigin {
    type Output = CircleOnOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for CircleOnOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_reverse(self) -> Self {
        CircleOnOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for CircleOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for CircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for CircleRotor {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_reverse(self) -> Self {
        CircleRotor::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for CircleRotorAligningOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_reverse(self) -> Self {
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
impl std::ops::Div<AntiReversePrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = CircleRotorAtInfinity;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for CircleRotorAtInfinity {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reverse(self) -> Self {
        CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for CircleRotorOnOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for Dipole {
    type Output = Dipole;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for Dipole {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_reverse(self) -> Self {
        Dipole::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group2() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for DipoleAligningOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for DipoleAtInfinity {
    type Output = DipoleAtInfinity;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for DipoleAtInfinity {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for DipoleAtOrigin {
    type Output = DipoleAtOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for DipoleAtOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_reverse(self) -> Self {
        DipoleAtOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for DipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for DipoleInversion {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_reverse(self) -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = DipoleInversionAligningOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for DipoleInversionAligningOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reverse(self) -> Self {
        DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for DipoleInversionAtInfinity {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group2(),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for DipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reverse(self) -> Self {
        DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for DipoleInversionOnOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        DipoleInversionOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from(-1.0), /* e1234, e4235, e4315, e4125 */ self.group1())
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_reverse(self) -> Self {
        DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for DipoleOnOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for DipoleOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn anti_reverse(self) -> Self {
        DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group2() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for DualNum {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for DualNum {
    fn anti_reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for FlatOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for FlatOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for FlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e45] * -1.0)
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for FlatPoint {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for FlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for FlatPointAtInfinity {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for FlatPointAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_reverse(self) -> Self {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for Flector {
    type Output = Flector;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for Flector {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for Flector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        Flector::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125, e3215 */ self.group1())
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for FlectorAtInfinity {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for FlectorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for FlectorOnOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for FlectorOnOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]))
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for Horizon {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for Horizon {
    fn anti_reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for Infinity {
    type Output = Infinity;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for Infinity {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for Infinity {
    fn anti_reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for Line {
    type Output = Line;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for Line {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for Line {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_reverse(self) -> Self {
        Line::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for LineAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for LineAtInfinity {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for LineAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_reverse(self) -> Self {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for LineOnOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for LineOnOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for LineOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_reverse(self) -> Self {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for Motor {
    type Output = Motor;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for Motor {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for Motor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reverse(self) -> Self {
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for MotorAtInfinity {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for MotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for MotorOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for MotorOnOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       20        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group6() * Simd32x4::from(-1.0),
            // e423, e431, e412
            self.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group8() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for MysteryCircle {
    type Output = MysteryCircle;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for MysteryCircle {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for MysteryCircle {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        MysteryCircle::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for MysteryCircleRotor {
    type Output = MysteryCircleRotor;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for MysteryCircleRotor {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        MysteryCircleRotor::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0), /* e12345 */ self[e12345])
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for MysteryDipole {
    type Output = MysteryDipole;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for MysteryDipole {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for MysteryDipole {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        MysteryDipole::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for MysteryDipoleInversion {
    type Output = MysteryDipoleInversion;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for MysteryDipoleInversion {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        MysteryDipoleInversion::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125 */ self.group1())
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for MysteryVersorEven {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for MysteryVersorEven {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        MysteryVersorEven::from_groups(/* e12345, e1, e2, e3 */ self.group0(), /* e415, e425, e435, e321 */ self.group1() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for MysteryVersorOdd {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for MysteryVersorOdd {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        MysteryVersorOdd::from_groups(/* scalar, e4235, e4315, e4125 */ self.group0(), /* e23, e31, e12, e45 */ self.group1() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for NullCircleAtOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for NullCircleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_reverse(self) -> Self {
        NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for NullDipoleAtOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for NullDipoleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_reverse(self) -> Self {
        NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for NullDipoleInversionAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for NullDipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for NullDipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for NullSphereAtOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for NullSphereAtOrigin {
    fn anti_reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for NullVersorEvenAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for NullVersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for NullVersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for Origin {
    type Output = Origin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for Origin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for Origin {
    fn anti_reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for Plane {
    fn anti_reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for PlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for PlaneOnOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for PlaneOnOrigin {
    fn anti_reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for RoundPoint {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for RoundPoint {
    fn anti_reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for RoundPointAtOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for RoundPointAtOrigin {
    fn anti_reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for Scalar {
    fn anti_reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for Sphere {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for Sphere {
    fn anti_reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for SphereAtOrigin {
    type Output = SphereAtOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for SphereAtOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for SphereAtOrigin {
    fn anti_reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for SphereOnOrigin {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for SphereOnOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for SphereOnOrigin {
    fn anti_reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for VersorEven {
    type Output = VersorEven;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for VersorEven {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for VersorEven {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn anti_reverse(self) -> Self {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for VersorEvenAligningOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn anti_reverse(self) -> Self {
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for VersorEvenAtInfinity {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reverse(self) -> Self {
        VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = VersorEvenAtOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for VersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reverse(self) -> Self {
        VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = VersorEvenOnOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for VersorEvenOnOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reverse(self) -> Self {
        VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reverse(self) -> Self {
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            self.group2(),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for VersorOdd {
    type Output = VersorOdd;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for VersorOdd {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for VersorOdd {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn anti_reverse(self) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for VersorOddAtInfinity {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reverse(self) -> Self {
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group2(),
        )
    }
}
impl std::ops::Div<AntiReversePrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for VersorOddOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn anti_reverse(self) -> Self {
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e3215
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
