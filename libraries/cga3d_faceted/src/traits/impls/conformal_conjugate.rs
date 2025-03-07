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
//  Average:         0       4       0
//  Maximum:         0      22       0
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = AntiCircleOnOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiCircleOnOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiCircleOnOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiCircleRotor {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = AntiCircleRotorAtInfinity;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiCircleRotorAtInfinity {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiCircleRotorOnOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiCircleRotorOnOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiDipoleInversion {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conformal_conjugate(self) -> Self {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn conformal_conjugate(self) -> Self {
        AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
            // e1, e2, e3, e5
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiDipoleInversionOnOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn conformal_conjugate(self) -> Self {
        AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiDipoleOnOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiDipoleOnOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiDualNum {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiDualNum {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiFlatOrigin {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiFlatOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiFlatOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiFlector {
    type Output = AntiFlector;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiFlector {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiFlector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiFlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiFlectorOnOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiFlectorOnOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiLine {
    type Output = AntiLine;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiLine {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiLine {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        AntiLine::from_groups(/* e23, e31, e12 */ self.group0(), /* e15, e25, e35 */ self.group1() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiLineOnOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiLineOnOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiMotor {
    type Output = AntiMotor;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiMotor {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiMotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        AntiMotor::from_groups(/* e23, e31, e12, scalar */ self.group0(), /* e15, e25, e35, e3215 */ self.group1() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiMotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiMotorOnOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiMotorOnOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = AntiMysteryCircleRotor;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiMysteryCircleRotor {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]), /* scalar */ self[scalar])
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = AntiMysteryDipoleInversion;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiMysteryDipoleInversion {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        AntiMysteryDipoleInversion::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3
            self.group1(),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiPlane {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiPlane {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiPlaneOnOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiPlaneOnOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e12345] * -1.0)
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiSphereOnOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiSphereOnOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for AntiVersorEvenOnOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for AntiVersorEvenOnOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for Circle {
    type Output = Circle;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for Circle {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conformal_conjugate(self) -> Self {
        Circle::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for CircleAligningOrigin {
    type Output = CircleAligningOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for CircleAligningOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn conformal_conjugate(self) -> Self {
        CircleAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for CircleAtInfinity {
    type Output = CircleAtInfinity;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for CircleAtInfinity {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conformal_conjugate(self) -> Self {
        CircleAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for CircleAtOrigin {
    type Output = CircleAtOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for CircleAtOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        CircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0(), /* e235, e315, e125 */ self.group1() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for CircleOnOrigin {
    type Output = CircleOnOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for CircleOnOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        CircleOnOrigin::from_groups(/* e423, e431, e412 */ self.group0(), /* e415, e425, e435 */ self.group1() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for CircleOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        CircleOrthogonalOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0(), /* e235, e315, e125 */ self.group1() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for CircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for CircleRotor {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for CircleRotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for CircleRotorAligningOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conformal_conjugate(self) -> Self {
        CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conformal_conjugate(self) -> Self {
        CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = CircleRotorAtInfinity;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for CircleRotorAtInfinity {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for CircleRotorOnOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conformal_conjugate(self) -> Self {
        CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for Dipole {
    type Output = Dipole;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for Dipole {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conformal_conjugate(self) -> Self {
        Dipole::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group2() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for DipoleAligningOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conformal_conjugate(self) -> Self {
        DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for DipoleAtInfinity {
    type Output = DipoleAtInfinity;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for DipoleAtInfinity {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conformal_conjugate(self) -> Self {
        DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for DipoleAtOrigin {
    type Output = DipoleAtOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for DipoleAtOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        DipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0(), /* e15, e25, e35 */ self.group1() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for DipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for DipoleInversion {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for DipoleInversion {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conformal_conjugate(self) -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = DipoleInversionAligningOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for DipoleInversionAligningOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conformal_conjugate(self) -> Self {
        DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for DipoleInversionAtInfinity {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn conformal_conjugate(self) -> Self {
        DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for DipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for DipoleInversionOnOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234, e4235, e4315, e4125
            self.group1() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for DipoleOnOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for DipoleOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35
            self.group2() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for DualNum {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for DualNum {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn conformal_conjugate(self) -> Self {
        DualNum::from_groups(/* e4, e12345 */ self.group0() * Simd32x2::from([1.0, -1.0]))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for FlatOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for FlatOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for FlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e45] * -1.0)
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for FlatPoint {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for FlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for FlatPointAtInfinity {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for FlatPointAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for Flector {
    type Output = Flector;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for Flector {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for Flector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        Flector::from_groups(
            // e15, e25, e35, e45
            self.group0() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for FlectorAtInfinity {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for FlectorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for FlectorOnOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for FlectorOnOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for Horizon {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e3215] * -1.0)
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for Infinity {
    type Output = Infinity;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for Infinity {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for Infinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e5] * -1.0)
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for Line {
    type Output = Line;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for Line {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for Line {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn conformal_conjugate(self) -> Self {
        Line::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for LineAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for LineAtInfinity {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for LineAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for LineOnOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for LineOnOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for LineOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for Motor {
    type Output = Motor;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for Motor {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for Motor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for MotorAtInfinity {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for MotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for MotorOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for MotorOnOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       22        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0() * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5] * -1.0,
            // e41, e42, e43, e45
            self.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            self.group9() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e3215
            self[e3215] * -1.0,
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for MysteryCircle {
    type Output = MysteryCircle;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for MysteryCircle {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for MysteryCircle {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        MysteryCircle::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for MysteryCircleRotor {
    type Output = MysteryCircleRotor;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for MysteryCircleRotor {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        MysteryCircleRotor::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e12345
            self[e12345] * -1.0,
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for MysteryDipole {
    type Output = MysteryDipole;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for MysteryDipole {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for MysteryDipole {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        MysteryDipole::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for MysteryDipoleInversion {
    type Output = MysteryDipoleInversion;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for MysteryDipoleInversion {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conformal_conjugate(self) -> Self {
        MysteryDipoleInversion::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for MysteryVersorEven {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for MysteryVersorEven {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for MysteryVersorOdd {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for MysteryVersorOdd {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for NullCircleAtOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for NullCircleAtOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for NullDipoleAtOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for NullDipoleAtOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for NullDipoleInversionAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for NullDipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for NullDipoleInversionAtOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for NullSphereAtOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for NullSphereAtOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for NullVersorEvenAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for NullVersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for NullVersorEvenAtOrigin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for Origin {
    type Output = Origin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for Origin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for Origin {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for Plane {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for PlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for PlaneOnOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for PlaneOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conformal_conjugate(self) -> Self {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for RoundPoint {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ self.group0(), /* e5 */ self[e5] * -1.0)
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for RoundPointAtOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn conformal_conjugate(self) -> Self {
        RoundPointAtOrigin::from_groups(/* e4, e5 */ self.group0() * Simd32x2::from([1.0, -1.0]))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for Scalar {
    fn conformal_conjugate(self) -> Self {
        self
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for Sphere {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for Sphere {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0() * Simd32x4::from(-1.0), /* e1234 */ self[e1234])
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for SphereAtOrigin {
    type Output = SphereAtOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for SphereAtOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn conformal_conjugate(self) -> Self {
        SphereAtOrigin::from_groups(/* e3215, e1234 */ self.group0() * Simd32x2::from([-1.0, 1.0]))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for SphereOnOrigin {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for SphereOnOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for VersorEven {
    type Output = VersorEven;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for VersorEven {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for VersorEven {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conformal_conjugate(self) -> Self {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            self.group3(),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for VersorEvenAligningOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conformal_conjugate(self) -> Self {
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for VersorEvenAtInfinity {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conformal_conjugate(self) -> Self {
        VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = VersorEvenAtOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for VersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        VersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ self.group0(), /* e235, e315, e125, e5 */ self.group1() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = VersorEvenOnOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for VersorEvenOnOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            self.group2(),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for VersorOdd {
    type Output = VersorOdd;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for VersorOdd {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for VersorOdd {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conformal_conjugate(self) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for VersorOddAtInfinity {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conformal_conjugate(self) -> Self {
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<ConformalConjugatePrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    fn div(self, _rhs: ConformalConjugatePrefixOrPostfix) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl std::ops::DivAssign<ConformalConjugatePrefixOrPostfix> for VersorOddOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: ConformalConjugatePrefixOrPostfix) {
        *self = self.conformal_conjugate()
    }
}
impl ConformalConjugate for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conformal_conjugate(self) -> Self {
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
