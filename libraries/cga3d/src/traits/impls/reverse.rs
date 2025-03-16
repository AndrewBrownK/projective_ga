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
//   Median:         0       6       0
//  Average:         0       6       0
//  Maximum:         0      20       0
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
            // e15, e25, e35, e45
            self.group3() * Simd32x4::from(-1.0),
            // e41, e42, e43
            self.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group6() * Simd32x4::from(-1.0),
            // e423, e431, e412
            self.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group8() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
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
