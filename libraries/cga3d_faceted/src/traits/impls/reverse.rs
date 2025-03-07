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
impl std::ops::Div<ReversePrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = AntiCircleOnOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiCircleOnOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn reverse(self) -> Self {
        AntiCircleOnOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiCircleRotor {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn reverse(self) -> Self {
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
impl std::ops::Div<ReversePrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn reverse(self) -> Self {
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
impl std::ops::Div<ReversePrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn reverse(self) -> Self {
        AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = AntiCircleRotorAtInfinity;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiCircleRotorAtInfinity {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn reverse(self) -> Self {
        AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiCircleRotorOnOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn reverse(self) -> Self {
        AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiDipoleInversion {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn reverse(self) -> Self {
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
impl std::ops::Div<ReversePrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn reverse(self) -> Self {
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
impl std::ops::Div<ReversePrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn reverse(self) -> Self {
        AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0), /* e4, e1, e2, e3 */ self.group1())
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn reverse(self) -> Self {
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
impl std::ops::Div<ReversePrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiDipoleOnOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn reverse(self) -> Self {
        AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiDualNum {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiDualNum {
    fn reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for AntiFlatOrigin {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiFlatOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiFlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn reverse(self) -> Self {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e321] * -1.0)
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn reverse(self) -> Self {
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for AntiFlector {
    type Output = AntiFlector;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiFlector {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiFlector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn reverse(self) -> Self {
        AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0), /* e1, e2, e3, e5 */ self.group1())
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for AntiFlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiFlectorOnOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn reverse(self) -> Self {
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]))
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for AntiLine {
    type Output = AntiLine;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiLine {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiLine {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn reverse(self) -> Self {
        AntiLine::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiLineOnOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn reverse(self) -> Self {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for AntiMotor {
    type Output = AntiMotor;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiMotor {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiMotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn reverse(self) -> Self {
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e3215
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for AntiMotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiMotorOnOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn reverse(self) -> Self {
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = AntiMysteryCircleRotor;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiMysteryCircleRotor {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn reverse(self) -> Self {
        use crate::elements::*;
        AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* scalar */ self[scalar])
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = AntiMysteryDipoleInversion;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiMysteryDipoleInversion {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn reverse(self) -> Self {
        AntiMysteryDipoleInversion::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0), /* e1, e2, e3 */ self.group1())
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiPlane {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiPlane {
    fn reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiPlaneOnOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiPlaneOnOrigin {
    fn reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiScalar {
    fn reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiSphereOnOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiSphereOnOrigin {
    fn reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for AntiVersorEvenOnOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn reverse(self) -> Self {
        AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for Circle {
    type Output = Circle;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for Circle {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn reverse(self) -> Self {
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
impl std::ops::Div<ReversePrefixOrPostfix> for CircleAligningOrigin {
    type Output = CircleAligningOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for CircleAligningOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn reverse(self) -> Self {
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
impl std::ops::Div<ReversePrefixOrPostfix> for CircleAtInfinity {
    type Output = CircleAtInfinity;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for CircleAtInfinity {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn reverse(self) -> Self {
        CircleAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for CircleAtOrigin {
    type Output = CircleAtOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for CircleAtOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn reverse(self) -> Self {
        CircleAtOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for CircleOnOrigin {
    type Output = CircleOnOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for CircleOnOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn reverse(self) -> Self {
        CircleOnOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for CircleOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn reverse(self) -> Self {
        CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for CircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for CircleRotor {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn reverse(self) -> Self {
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
impl std::ops::Div<ReversePrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for CircleRotorAligningOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn reverse(self) -> Self {
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
impl std::ops::Div<ReversePrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn reverse(self) -> Self {
        CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = CircleRotorAtInfinity;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for CircleRotorAtInfinity {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn reverse(self) -> Self {
        CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for CircleRotorOnOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn reverse(self) -> Self {
        CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for Dipole {
    type Output = Dipole;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for Dipole {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn reverse(self) -> Self {
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
impl std::ops::Div<ReversePrefixOrPostfix> for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for DipoleAligningOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn reverse(self) -> Self {
        DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for DipoleAtInfinity {
    type Output = DipoleAtInfinity;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for DipoleAtInfinity {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn reverse(self) -> Self {
        DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for DipoleAtOrigin {
    type Output = DipoleAtOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for DipoleAtOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn reverse(self) -> Self {
        DipoleAtOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for DipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for DipoleInversion {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn reverse(self) -> Self {
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
impl std::ops::Div<ReversePrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = DipoleInversionAligningOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for DipoleInversionAligningOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn reverse(self) -> Self {
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
impl std::ops::Div<ReversePrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for DipoleInversionAtInfinity {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn reverse(self) -> Self {
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
impl std::ops::Div<ReversePrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for DipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn reverse(self) -> Self {
        DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for DipoleInversionOnOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn reverse(self) -> Self {
        DipoleInversionOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from(-1.0), /* e1234, e4235, e4315, e4125 */ self.group1())
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn reverse(self) -> Self {
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
impl std::ops::Div<ReversePrefixOrPostfix> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for DipoleOnOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn reverse(self) -> Self {
        DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for DipoleOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn reverse(self) -> Self {
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
impl std::ops::Div<ReversePrefixOrPostfix> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for DualNum {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for DualNum {
    fn reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for FlatOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for FlatOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for FlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn reverse(self) -> Self {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e45] * -1.0)
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for FlatPoint {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for FlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn reverse(self) -> Self {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for FlatPointAtInfinity {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for FlatPointAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn reverse(self) -> Self {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for Flector {
    type Output = Flector;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for Flector {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for Flector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn reverse(self) -> Self {
        Flector::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125, e3215 */ self.group1())
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for FlectorAtInfinity {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for FlectorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn reverse(self) -> Self {
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for FlectorOnOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for FlectorOnOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn reverse(self) -> Self {
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]))
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for Horizon {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for Horizon {
    fn reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for Infinity {
    type Output = Infinity;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for Infinity {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for Infinity {
    fn reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for Line {
    type Output = Line;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for Line {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for Line {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn reverse(self) -> Self {
        Line::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for LineAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for LineAtInfinity {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for LineAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn reverse(self) -> Self {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for LineOnOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for LineOnOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for LineOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn reverse(self) -> Self {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for Motor {
    type Output = Motor;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for Motor {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for Motor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn reverse(self) -> Self {
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for MotorAtInfinity {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for MotorAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn reverse(self) -> Self {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for MotorOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for MotorOnOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn reverse(self) -> Self {
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       20        0
    fn reverse(self) -> Self {
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
impl std::ops::Div<ReversePrefixOrPostfix> for MysteryCircle {
    type Output = MysteryCircle;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for MysteryCircle {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for MysteryCircle {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn reverse(self) -> Self {
        MysteryCircle::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for MysteryCircleRotor {
    type Output = MysteryCircleRotor;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for MysteryCircleRotor {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn reverse(self) -> Self {
        use crate::elements::*;
        MysteryCircleRotor::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0), /* e12345 */ self[e12345])
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for MysteryDipole {
    type Output = MysteryDipole;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for MysteryDipole {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for MysteryDipole {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn reverse(self) -> Self {
        MysteryDipole::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for MysteryDipoleInversion {
    type Output = MysteryDipoleInversion;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for MysteryDipoleInversion {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn reverse(self) -> Self {
        MysteryDipoleInversion::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125 */ self.group1())
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for MysteryVersorEven {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for MysteryVersorEven {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn reverse(self) -> Self {
        MysteryVersorEven::from_groups(/* e12345, e1, e2, e3 */ self.group0(), /* e415, e425, e435, e321 */ self.group1() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for MysteryVersorOdd {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for MysteryVersorOdd {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn reverse(self) -> Self {
        MysteryVersorOdd::from_groups(/* scalar, e4235, e4315, e4125 */ self.group0(), /* e23, e31, e12, e45 */ self.group1() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for NullCircleAtOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for NullCircleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn reverse(self) -> Self {
        NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for NullDipoleAtOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for NullDipoleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn reverse(self) -> Self {
        NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0() * Simd32x3::from(-1.0))
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for NullDipoleInversionAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for NullDipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for NullDipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn reverse(self) -> Self {
        NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for NullSphereAtOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for NullSphereAtOrigin {
    fn reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for NullVersorEvenAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for NullVersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for NullVersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn reverse(self) -> Self {
        NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for Origin {
    type Output = Origin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for Origin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for Origin {
    fn reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for Plane {
    fn reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for PlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for PlaneOnOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for PlaneOnOrigin {
    fn reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for RoundPoint {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for RoundPoint {
    fn reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for RoundPointAtOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for RoundPointAtOrigin {
    fn reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for Scalar {
    fn reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for Sphere {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for Sphere {
    fn reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for SphereAtOrigin {
    type Output = SphereAtOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for SphereAtOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for SphereAtOrigin {
    fn reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for SphereOnOrigin {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for SphereOnOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for SphereOnOrigin {
    fn reverse(self) -> Self {
        self
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for VersorEven {
    type Output = VersorEven;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for VersorEven {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for VersorEven {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn reverse(self) -> Self {
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
impl std::ops::Div<ReversePrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for VersorEvenAligningOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn reverse(self) -> Self {
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
impl std::ops::Div<ReversePrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for VersorEvenAtInfinity {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn reverse(self) -> Self {
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
impl std::ops::Div<ReversePrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = VersorEvenAtOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for VersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn reverse(self) -> Self {
        VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = VersorEvenOnOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for VersorEvenOnOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn reverse(self) -> Self {
        VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<ReversePrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn reverse(self) -> Self {
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
impl std::ops::Div<ReversePrefixOrPostfix> for VersorOdd {
    type Output = VersorOdd;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for VersorOdd {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for VersorOdd {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn reverse(self) -> Self {
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
impl std::ops::Div<ReversePrefixOrPostfix> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for VersorOddAtInfinity {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn reverse(self) -> Self {
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
impl std::ops::Div<ReversePrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    fn div(self, _rhs: ReversePrefixOrPostfix) -> Self::Output {
        self.reverse()
    }
}
impl std::ops::DivAssign<ReversePrefixOrPostfix> for VersorOddOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: ReversePrefixOrPostfix) {
        *self = self.reverse()
    }
}
impl Reverse for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn reverse(self) -> Self {
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
