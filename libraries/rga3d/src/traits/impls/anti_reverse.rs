// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 11
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         0       1       0     N/A
//  Average:         0       0       0     N/A
//  Maximum:         0       3       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         0       1       0       0
//  Average:         0       3       0       0
//  Maximum:         0      10       0       0
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
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_reverse(self) -> Self {
        Flector::from_groups(/* e1, e2, e3, e4 */ self.group0() * Simd32x4::from(-1.0), /* e423, e431, e412, e321 */ self.group1())
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
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn anti_reverse(self) -> Self {
        Line::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        )
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
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn anti_reverse(self) -> Self {
        Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
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
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       10        0        0
    fn anti_reverse(self) -> Self {
        MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1() * Simd32x4::from(-1.0),
            // e41, e42, e43
            self.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group3() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            self.group4(),
        )
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
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4] * -1.0)
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
impl std::ops::Div<AntiReversePrefixOrPostfix> for Point {
    type Output = Point;
    fn div(self, _rhs: AntiReversePrefixOrPostfix) -> Self::Output {
        self.anti_reverse()
    }
}
impl std::ops::DivAssign<AntiReversePrefixOrPostfix> for Point {
    fn div_assign(&mut self, _rhs: AntiReversePrefixOrPostfix) {
        *self = self.anti_reverse()
    }
}
impl AntiReverse for Point {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn anti_reverse(self) -> Self {
        Point::from_groups(/* e1, e2, e3, e4 */ self.group0() * Simd32x4::from(-1.0))
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
