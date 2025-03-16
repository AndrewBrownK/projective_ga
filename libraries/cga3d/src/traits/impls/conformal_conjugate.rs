// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 25
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       2       0
//  Average:         0       1       0
//  Maximum:         0       6       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       4       0
//  Average:         0       5       0
//  Maximum:         0      18       0
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
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e45] * -1.0),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
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
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group3().xyz().with_w(self[e5] * -1.0),
        )
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
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn conformal_conjugate(self) -> Self {
        AntiDualNum::from_groups(/* e3215, scalar */ self.group0() * Simd32x2::from([-1.0, 1.0]))
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
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group1().xyz().with_w(self[e5] * -1.0),
        )
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
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0().xyz().with_w(self[e5] * -1.0))
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
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e45] * -1.0),
            // e15, e25, e35
            self.group2() * Simd32x3::from(-1.0),
        )
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
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e45] * -1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3() * Simd32x4::from(-1.0),
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
        DualNum::from_groups(/* e5, e12345 */ self.group0() * Simd32x2::from(-1.0))
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
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       18        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0() * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5] * -1.0,
            // e15, e25, e35, e45
            self.group3() * Simd32x4::from(-1.0),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group9() * Simd32x4::from(-1.0),
            // e1234
            self[e1234],
        )
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
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().xyz().with_w(self[e12345] * -1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            self.group3(),
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
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e45] * -1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3() * Simd32x4::from(-1.0),
        )
    }
}
