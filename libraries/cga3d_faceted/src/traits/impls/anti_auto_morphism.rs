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
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = AntiCircleOnOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiCircleOnOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_auto_morphism(self) -> Self {
        AntiCircleOnOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiCircleRotor {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_auto_morphism(self) -> Self {
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_auto_morphism(self) -> Self {
        AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_auto_morphism(self) -> Self {
        AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = AntiCircleRotorAtInfinity;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_auto_morphism(self) -> Self {
        AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_auto_morphism(self) -> Self {
        AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiDipoleInversion {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiDipoleInversion {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiDipoleInversionAtInfinity {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiDipoleInversionOnOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiDipoleInversionOrthogonalOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiDipoleOnOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiDipoleOnOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiDualNum {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiDualNum {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn anti_auto_morphism(self) -> Self {
        AntiDualNum::from_groups(/* e1234, scalar */ self.group0() * Simd32x2::from(-1.0))
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiFlatOrigin {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiFlatOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiFlatOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiFlatPoint {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiFlector {
    type Output = AntiFlector;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiFlector {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiFlector {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiFlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiFlectorOnOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiFlectorOnOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiLine {
    type Output = AntiLine;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiLine {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiLine {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_auto_morphism(self) -> Self {
        AntiLine::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiLineOnOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_auto_morphism(self) -> Self {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiMotor {
    type Output = AntiMotor;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiMotor {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiMotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_auto_morphism(self) -> Self {
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, e3215
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiMotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiMotorOnOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_auto_morphism(self) -> Self {
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = AntiMysteryCircleRotor;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiMysteryCircleRotor {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_auto_morphism(self) -> Self {
        use crate::elements::*;
        AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* scalar */ self[scalar] * -1.0)
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = AntiMysteryDipoleInversion;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiMysteryDipoleInversion {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiMysteryDipoleInversion {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiPlane {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiPlane {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiPlaneOnOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiPlaneOnOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiScalar {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiSphereOnOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiSphereOnOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_auto_morphism(self) -> Self {
        AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e1234
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for Circle {
    type Output = Circle;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for Circle {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for Circle {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for CircleAligningOrigin {
    type Output = CircleAligningOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for CircleAligningOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for CircleAligningOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for CircleAtInfinity {
    type Output = CircleAtInfinity;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for CircleAtInfinity {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for CircleAtInfinity {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for CircleAtOrigin {
    type Output = CircleAtOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for CircleAtOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for CircleAtOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for CircleOnOrigin {
    type Output = CircleOnOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for CircleOnOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for CircleOnOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for CircleOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for CircleOrthogonalOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for CircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for CircleRotor {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for CircleRotor {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for CircleRotorAligningOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for CircleRotorAligningOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for CircleRotorAligningOriginAtInfinity {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = CircleRotorAtInfinity;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for CircleRotorAtInfinity {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for CircleRotorAtInfinity {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for CircleRotorOnOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for CircleRotorOnOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for Dipole {
    type Output = Dipole;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for Dipole {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_auto_morphism(self) -> Self {
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
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for DipoleAligningOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_auto_morphism(self) -> Self {
        DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for DipoleAtInfinity {
    type Output = DipoleAtInfinity;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for DipoleAtInfinity {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_auto_morphism(self) -> Self {
        DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for DipoleAtOrigin {
    type Output = DipoleAtOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for DipoleAtOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_auto_morphism(self) -> Self {
        DipoleAtOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for DipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for DipoleInversion {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn anti_auto_morphism(self) -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group3() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = DipoleInversionAligningOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for DipoleInversionAligningOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn anti_auto_morphism(self) -> Self {
        DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for DipoleInversionAtInfinity {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_auto_morphism(self) -> Self {
        DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for DipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_auto_morphism(self) -> Self {
        DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for DipoleInversionOnOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_auto_morphism(self) -> Self {
        DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from(-1.0),
            // e1234, e4235, e4315, e4125
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_auto_morphism(self) -> Self {
        DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for DipoleOnOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_auto_morphism(self) -> Self {
        DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for DipoleOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn anti_auto_morphism(self) -> Self {
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
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for DualNum {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for DualNum {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for FlatOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for FlatOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for FlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_auto_morphism(self) -> Self {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e45] * -1.0)
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for FlatPoint {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for FlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_auto_morphism(self) -> Self {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for FlatPointAtInfinity {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for FlatPointAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_auto_morphism(self) -> Self {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for Flector {
    type Output = Flector;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for Flector {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for Flector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_auto_morphism(self) -> Self {
        Flector::from_groups(
            // e15, e25, e35, e45
            self.group0() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for FlectorAtInfinity {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for FlectorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_auto_morphism(self) -> Self {
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for FlectorOnOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for FlectorOnOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_auto_morphism(self) -> Self {
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for Horizon {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_auto_morphism(self) -> Self {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e3215] * -1.0)
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for Infinity {
    type Output = Infinity;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for Infinity {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for Infinity {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for Line {
    type Output = Line;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for Line {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for Line {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for LineAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for LineAtInfinity {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for LineAtInfinity {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for LineOnOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for LineOnOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for LineOnOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for Motor {
    type Output = Motor;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for Motor {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for Motor {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for MotorAtInfinity {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for MotorAtInfinity {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for MotorOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for MotorOnOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for MotorOnOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       17        0
    fn anti_auto_morphism(self) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0() * Simd32x2::from([-1.0, 1.0]),
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
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9() * Simd32x4::from(-1.0),
            // e3215
            self[e3215] * -1.0,
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for MysteryCircle {
    type Output = MysteryCircle;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for MysteryCircle {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for MysteryCircle {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for MysteryCircleRotor {
    type Output = MysteryCircleRotor;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for MysteryCircleRotor {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for MysteryCircleRotor {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for MysteryDipole {
    type Output = MysteryDipole;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for MysteryDipole {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for MysteryDipole {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_auto_morphism(self) -> Self {
        MysteryDipole::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for MysteryDipoleInversion {
    type Output = MysteryDipoleInversion;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for MysteryDipoleInversion {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_auto_morphism(self) -> Self {
        MysteryDipoleInversion::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for MysteryVersorEven {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for MysteryVersorEven {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for MysteryVersorEven {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for MysteryVersorOdd {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for MysteryVersorOdd {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_auto_morphism(self) -> Self {
        MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            self.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for NullCircleAtOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for NullCircleAtOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for NullDipoleAtOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for NullDipoleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_auto_morphism(self) -> Self {
        NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for NullDipoleInversionAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for NullDipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for NullDipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_auto_morphism(self) -> Self {
        NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for NullSphereAtOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for NullSphereAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_auto_morphism(self) -> Self {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ self[e1234] * -1.0)
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for NullVersorEvenAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for NullVersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for NullVersorEvenAtOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for Origin {
    type Output = Origin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for Origin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for Origin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for Plane {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_auto_morphism(self) -> Self {
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for PlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for PlaneOnOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for PlaneOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_auto_morphism(self) -> Self {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for RoundPoint {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for RoundPoint {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for RoundPointAtOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for RoundPointAtOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_auto_morphism(self) -> Self {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * -1.0)
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for Sphere {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_auto_morphism(self) -> Self {
        use crate::elements::*;
        Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0() * Simd32x4::from(-1.0), /* e1234 */ self[e1234] * -1.0)
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for SphereAtOrigin {
    type Output = SphereAtOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for SphereAtOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn anti_auto_morphism(self) -> Self {
        SphereAtOrigin::from_groups(/* e3215, e1234 */ self.group0() * Simd32x2::from(-1.0))
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for SphereOnOrigin {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for SphereOnOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_auto_morphism(self) -> Self {
        SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for VersorEven {
    type Output = VersorEven;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for VersorEven {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for VersorEven {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for VersorEvenAligningOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for VersorEvenAligningOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for VersorEvenAtInfinity {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for VersorEvenAtInfinity {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = VersorEvenAtOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for VersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for VersorEvenAtOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = VersorEvenOnOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for VersorEvenOnOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for VersorEvenOnOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for VersorEvenOrthogonalOrigin {
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for VersorOdd {
    type Output = VersorOdd;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for VersorOdd {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for VersorOdd {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn anti_auto_morphism(self) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group3() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for VersorOddAtInfinity {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn anti_auto_morphism(self) -> Self {
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn anti_auto_morphism(self) -> Self {
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e3215
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
