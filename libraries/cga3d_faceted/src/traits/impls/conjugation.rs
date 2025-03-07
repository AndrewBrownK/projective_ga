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
//  Average:         0       4       0
//  Maximum:         0      17       0
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = AntiCircleOnOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiCircleOnOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn conjugation(self) -> Self {
        AntiCircleOnOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiCircleRotor {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn conjugation(self) -> Self {
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
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn conjugation(self) -> Self {
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
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conjugation(self) -> Self {
        AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = AntiCircleRotorAtInfinity;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conjugation(self) -> Self {
        AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conjugation(self) -> Self {
        AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiDipoleInversion {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conjugation(self) -> Self {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e5
            self.group3() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0(), /* e4, e1, e2, e3 */ self.group1() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conjugation(self) -> Self {
        AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiDipoleOnOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiDipoleOnOrigin {
    fn conjugation(self) -> Self {
        self
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiDualNum {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiDualNum {
    fn conjugation(self) -> Self {
        self
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiFlatOrigin {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiFlatOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiFlatOrigin {
    fn conjugation(self) -> Self {
        self
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiFlatPoint {
    fn conjugation(self) -> Self {
        self
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiFlector {
    type Output = AntiFlector;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiFlector {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiFlector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0(), /* e1, e2, e3, e5 */ self.group1() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiFlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiFlectorOnOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]))
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiLine {
    type Output = AntiLine;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiLine {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiLine {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn conjugation(self) -> Self {
        AntiLine::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiLineOnOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conjugation(self) -> Self {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiMotor {
    type Output = AntiMotor;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiMotor {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiMotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conjugation(self) -> Self {
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e3215
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiMotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiMotorOnOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = AntiMysteryCircleRotor;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiMysteryCircleRotor {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* scalar */ self[scalar])
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = AntiMysteryDipoleInversion;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiMysteryDipoleInversion {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conjugation(self) -> Self {
        AntiMysteryDipoleInversion::from_groups(/* e415, e425, e435, e321 */ self.group0(), /* e1, e2, e3 */ self.group1() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiPlane {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiPlane {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiPlaneOnOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiPlaneOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conjugation(self) -> Self {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e12345] * -1.0)
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiSphereOnOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        AntiSphereOnOrigin::from_groups(/* e1, e2, e3, e4 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conjugation(self) -> Self {
        AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for Circle {
    type Output = Circle;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for Circle {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for Circle {
    fn conjugation(self) -> Self {
        self
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for CircleAligningOrigin {
    type Output = CircleAligningOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for CircleAligningOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for CircleAligningOrigin {
    fn conjugation(self) -> Self {
        self
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for CircleAtInfinity {
    type Output = CircleAtInfinity;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for CircleAtInfinity {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for CircleAtInfinity {
    fn conjugation(self) -> Self {
        self
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for CircleAtOrigin {
    type Output = CircleAtOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for CircleAtOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for CircleAtOrigin {
    fn conjugation(self) -> Self {
        self
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for CircleOnOrigin {
    type Output = CircleOnOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for CircleOnOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for CircleOnOrigin {
    fn conjugation(self) -> Self {
        self
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for CircleOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for CircleOrthogonalOrigin {
    fn conjugation(self) -> Self {
        self
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for CircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for CircleRotor {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for CircleRotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for CircleRotorAligningOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            self.group0(),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = CircleRotorAtInfinity;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for CircleRotorAtInfinity {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for CircleRotorOnOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435
            self.group1(),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for Dipole {
    type Output = Dipole;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for Dipole {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn conjugation(self) -> Self {
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
impl std::ops::Div<ConjugationPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for DipoleAligningOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conjugation(self) -> Self {
        DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for DipoleAtInfinity {
    type Output = DipoleAtInfinity;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for DipoleAtInfinity {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conjugation(self) -> Self {
        DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for DipoleAtOrigin {
    type Output = DipoleAtOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for DipoleAtOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn conjugation(self) -> Self {
        DipoleAtOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for DipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for DipoleInversion {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn conjugation(self) -> Self {
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
impl std::ops::Div<ConjugationPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = DipoleInversionAligningOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for DipoleInversionAligningOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conjugation(self) -> Self {
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
impl std::ops::Div<ConjugationPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for DipoleInversionAtInfinity {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn conjugation(self) -> Self {
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
impl std::ops::Div<ConjugationPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for DipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conjugation(self) -> Self {
        DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for DipoleInversionOnOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        DipoleInversionOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from(-1.0), /* e1234, e4235, e4315, e4125 */ self.group1())
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn conjugation(self) -> Self {
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
impl std::ops::Div<ConjugationPrefixOrPostfix> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for DipoleOnOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for DipoleOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn conjugation(self) -> Self {
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
impl std::ops::Div<ConjugationPrefixOrPostfix> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for DualNum {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for DualNum {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn conjugation(self) -> Self {
        DualNum::from_groups(/* e4, e12345 */ self.group0() * Simd32x2::from(-1.0))
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for FlatOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for FlatOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for FlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e45] * -1.0)
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for FlatPoint {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for FlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for FlatPointAtInfinity {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for FlatPointAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conjugation(self) -> Self {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for Flector {
    type Output = Flector;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for Flector {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for Flector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        Flector::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125, e3215 */ self.group1())
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for FlectorAtInfinity {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for FlectorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for FlectorOnOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for FlectorOnOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]))
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for Horizon {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for Horizon {
    fn conjugation(self) -> Self {
        self
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for Infinity {
    type Output = Infinity;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for Infinity {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for Infinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e5] * -1.0)
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for Line {
    type Output = Line;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for Line {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for Line {
    fn conjugation(self) -> Self {
        self
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for LineAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for LineAtInfinity {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for LineAtInfinity {
    fn conjugation(self) -> Self {
        self
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for LineOnOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for LineOnOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for LineOnOrigin {
    fn conjugation(self) -> Self {
        self
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for Motor {
    type Output = Motor;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for Motor {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for Motor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conjugation(self) -> Self {
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for MotorAtInfinity {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for MotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for MotorOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for MotorOnOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       17        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0() * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            self.group1() * Simd32x4::from(-1.0),
            // e5
            self[e5] * -1.0,
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
            self.group9(),
            // e3215
            self[e3215],
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for MysteryCircle {
    type Output = MysteryCircle;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for MysteryCircle {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for MysteryCircle {
    fn conjugation(self) -> Self {
        self
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for MysteryCircleRotor {
    type Output = MysteryCircleRotor;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for MysteryCircleRotor {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        MysteryCircleRotor::from_groups(/* e415, e425, e435, e321 */ self.group0(), /* e12345 */ self[e12345] * -1.0)
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for MysteryDipole {
    type Output = MysteryDipole;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for MysteryDipole {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for MysteryDipole {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        MysteryDipole::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for MysteryDipoleInversion {
    type Output = MysteryDipoleInversion;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for MysteryDipoleInversion {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        MysteryDipoleInversion::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125 */ self.group1())
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for MysteryVersorEven {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for MysteryVersorEven {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        MysteryVersorEven::from_groups(/* e12345, e1, e2, e3 */ self.group0() * Simd32x4::from(-1.0), /* e415, e425, e435, e321 */ self.group1())
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for MysteryVersorOdd {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for MysteryVersorOdd {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        MysteryVersorOdd::from_groups(/* scalar, e4235, e4315, e4125 */ self.group0(), /* e23, e31, e12, e45 */ self.group1() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for NullCircleAtOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for NullCircleAtOrigin {
    fn conjugation(self) -> Self {
        self
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for NullDipoleAtOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for NullDipoleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn conjugation(self) -> Self {
        NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for NullDipoleInversionAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for NullDipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for NullDipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for NullSphereAtOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for NullSphereAtOrigin {
    fn conjugation(self) -> Self {
        self
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for NullVersorEvenAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for NullVersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for NullVersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for Origin {
    type Output = Origin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for Origin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4] * -1.0)
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for Plane {
    fn conjugation(self) -> Self {
        self
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for PlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for PlaneOnOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for PlaneOnOrigin {
    fn conjugation(self) -> Self {
        self
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for RoundPoint {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn conjugation(self) -> Self {
        use crate::elements::*;
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ self.group0() * Simd32x4::from(-1.0), /* e5 */ self[e5] * -1.0)
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for RoundPointAtOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn conjugation(self) -> Self {
        RoundPointAtOrigin::from_groups(/* e4, e5 */ self.group0() * Simd32x2::from(-1.0))
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for Scalar {
    fn conjugation(self) -> Self {
        self
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for Sphere {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for Sphere {
    fn conjugation(self) -> Self {
        self
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for SphereAtOrigin {
    type Output = SphereAtOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for SphereAtOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for SphereAtOrigin {
    fn conjugation(self) -> Self {
        self
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for SphereOnOrigin {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for SphereOnOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for SphereOnOrigin {
    fn conjugation(self) -> Self {
        self
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for VersorEven {
    type Output = VersorEven;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for VersorEven {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for VersorEven {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conjugation(self) -> Self {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e4
            self.group3() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for VersorEvenAligningOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conjugation(self) -> Self {
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for VersorEvenAtInfinity {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conjugation(self) -> Self {
        VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = VersorEvenAtOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for VersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conjugation(self) -> Self {
        VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = VersorEvenOnOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for VersorEvenOnOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conjugation(self) -> Self {
        VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conjugation(self) -> Self {
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e4
            self.group2() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<ConjugationPrefixOrPostfix> for VersorOdd {
    type Output = VersorOdd;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for VersorOdd {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for VersorOdd {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conjugation(self) -> Self {
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
impl std::ops::Div<ConjugationPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for VersorOddAtInfinity {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn conjugation(self) -> Self {
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
impl std::ops::Div<ConjugationPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn conjugation(self) -> Self {
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
