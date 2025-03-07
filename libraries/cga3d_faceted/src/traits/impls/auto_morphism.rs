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
//  Average:         0       0       0
//  Maximum:         0       6       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       3       0
//  Maximum:         0      17       0
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = AntiCircleOnOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiCircleOnOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiCircleOnOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiCircleRotor {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiCircleRotor {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiCircleRotorAligningOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiCircleRotorAligningOriginAtInfinity {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = AntiCircleRotorAtInfinity;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiCircleRotorAtInfinity {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiCircleRotorOnOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiDipoleInversion {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn auto_morphism(self) -> Self {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            self.group3() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn auto_morphism(self) -> Self {
        AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
            // e1, e2, e3, e5
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn auto_morphism(self) -> Self {
        AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from(-1.0),
            // e4, e1, e2, e3
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn auto_morphism(self) -> Self {
        AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiDipoleOnOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn auto_morphism(self) -> Self {
        AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiDualNum {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiDualNum {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiFlatOrigin {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiFlatOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiFlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e321] * -1.0)
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn auto_morphism(self) -> Self {
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiFlector {
    type Output = AntiFlector;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiFlector {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiFlector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn auto_morphism(self) -> Self {
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group0() * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiFlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiFlectorOnOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn auto_morphism(self) -> Self {
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiLine {
    type Output = AntiLine;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiLine {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiLine {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiLineOnOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiLineOnOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiMotor {
    type Output = AntiMotor;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiMotor {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiMotor {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiMotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiMotorOnOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiMotorOnOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = AntiMysteryCircleRotor;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiMysteryCircleRotor {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiMysteryCircleRotor {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = AntiMysteryDipoleInversion;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiMysteryDipoleInversion {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn auto_morphism(self) -> Self {
        AntiMysteryDipoleInversion::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e1, e2, e3
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiPlane {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiPlane {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn auto_morphism(self) -> Self {
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiPlaneOnOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiPlaneOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn auto_morphism(self) -> Self {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e12345] * -1.0)
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiSphereOnOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn auto_morphism(self) -> Self {
        AntiSphereOnOrigin::from_groups(/* e1, e2, e3, e4 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for AntiVersorEvenOnOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for Circle {
    type Output = Circle;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for Circle {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn auto_morphism(self) -> Self {
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
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for CircleAligningOrigin {
    type Output = CircleAligningOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for CircleAligningOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn auto_morphism(self) -> Self {
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
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for CircleAtInfinity {
    type Output = CircleAtInfinity;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for CircleAtInfinity {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn auto_morphism(self) -> Self {
        CircleAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for CircleAtOrigin {
    type Output = CircleAtOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for CircleAtOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn auto_morphism(self) -> Self {
        CircleAtOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for CircleOnOrigin {
    type Output = CircleOnOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for CircleOnOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn auto_morphism(self) -> Self {
        CircleOnOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for CircleOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn auto_morphism(self) -> Self {
        CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for CircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for CircleRotor {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn auto_morphism(self) -> Self {
        CircleRotor::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for CircleRotorAligningOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn auto_morphism(self) -> Self {
        CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn auto_morphism(self) -> Self {
        CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = CircleRotorAtInfinity;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for CircleRotorAtInfinity {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn auto_morphism(self) -> Self {
        CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for CircleRotorOnOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn auto_morphism(self) -> Self {
        CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for Dipole {
    type Output = Dipole;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for Dipole {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Dipole {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for DipoleAligningOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for DipoleAligningOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for DipoleAtInfinity {
    type Output = DipoleAtInfinity;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for DipoleAtInfinity {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for DipoleAtInfinity {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for DipoleAtOrigin {
    type Output = DipoleAtOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for DipoleAtOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for DipoleAtOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for DipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for DipoleInversion {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for DipoleInversion {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = DipoleInversionAligningOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for DipoleInversionAligningOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for DipoleInversionAligningOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for DipoleInversionAtInfinity {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for DipoleInversionAtInfinity {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for DipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for DipoleInversionAtOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for DipoleInversionOnOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for DipoleInversionOnOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for DipoleInversionOrthogonalOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for DipoleOnOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for DipoleOnOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for DipoleOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for DipoleOrthogonalOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for DualNum {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for DualNum {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn auto_morphism(self) -> Self {
        DualNum::from_groups(/* e4, e12345 */ self.group0() * Simd32x2::from(-1.0))
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for FlatOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for FlatOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for FlatOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for FlatPoint {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for FlatPoint {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for FlatPointAtInfinity {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for FlatPointAtInfinity {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for Flector {
    type Output = Flector;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for Flector {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Flector {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for FlectorAtInfinity {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for FlectorAtInfinity {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for FlectorOnOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for FlectorOnOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for FlectorOnOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for Horizon {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Horizon {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for Infinity {
    type Output = Infinity;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for Infinity {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Infinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e5] * -1.0)
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for Line {
    type Output = Line;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for Line {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Line {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn auto_morphism(self) -> Self {
        Line::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for LineAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for LineAtInfinity {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for LineAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn auto_morphism(self) -> Self {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for LineOnOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for LineOnOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for LineOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn auto_morphism(self) -> Self {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for Motor {
    type Output = Motor;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for Motor {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Motor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn auto_morphism(self) -> Self {
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for MotorAtInfinity {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for MotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn auto_morphism(self) -> Self {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for MotorOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for MotorOnOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn auto_morphism(self) -> Self {
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       17        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0() * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            self.group1() * Simd32x4::from(-1.0),
            // e5
            self[e5] * -1.0,
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
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
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for MysteryCircle {
    type Output = MysteryCircle;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for MysteryCircle {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for MysteryCircle {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn auto_morphism(self) -> Self {
        MysteryCircle::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for MysteryCircleRotor {
    type Output = MysteryCircleRotor;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for MysteryCircleRotor {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        MysteryCircleRotor::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0), /* e12345 */ self[e12345] * -1.0)
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for MysteryDipole {
    type Output = MysteryDipole;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for MysteryDipole {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for MysteryDipole {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for MysteryDipoleInversion {
    type Output = MysteryDipoleInversion;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for MysteryDipoleInversion {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for MysteryDipoleInversion {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for MysteryVersorEven {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for MysteryVersorEven {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn auto_morphism(self) -> Self {
        MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            self.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for MysteryVersorOdd {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for MysteryVersorOdd {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for MysteryVersorOdd {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for NullCircleAtOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for NullCircleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn auto_morphism(self) -> Self {
        NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for NullDipoleAtOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for NullDipoleAtOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for NullDipoleInversionAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for NullDipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for NullDipoleInversionAtOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for NullSphereAtOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for NullSphereAtOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for NullVersorEvenAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for NullVersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for NullVersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn auto_morphism(self) -> Self {
        NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for Origin {
    type Output = Origin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for Origin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4] * -1.0)
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Plane {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for PlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for PlaneOnOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for PlaneOnOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for RoundPoint {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ self.group0() * Simd32x4::from(-1.0), /* e5 */ self[e5] * -1.0)
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for RoundPointAtOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn auto_morphism(self) -> Self {
        RoundPointAtOrigin::from_groups(/* e4, e5 */ self.group0() * Simd32x2::from(-1.0))
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Scalar {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for Sphere {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Sphere {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for SphereAtOrigin {
    type Output = SphereAtOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for SphereAtOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for SphereAtOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for SphereOnOrigin {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for SphereOnOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for SphereOnOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for VersorEven {
    type Output = VersorEven;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for VersorEven {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for VersorEven {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn auto_morphism(self) -> Self {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            self.group3() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for VersorEvenAligningOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn auto_morphism(self) -> Self {
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for VersorEvenAtInfinity {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn auto_morphism(self) -> Self {
        VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = VersorEvenAtOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for VersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn auto_morphism(self) -> Self {
        VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = VersorEvenOnOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for VersorEvenOnOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn auto_morphism(self) -> Self {
        VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn auto_morphism(self) -> Self {
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for VersorOdd {
    type Output = VersorOdd;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for VersorOdd {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for VersorOdd {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for VersorOddAtInfinity {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for VersorOddAtInfinity {
    fn auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for VersorOddOrthogonalOrigin {
    fn auto_morphism(self) -> Self {
        self
    }
}
