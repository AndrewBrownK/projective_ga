// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 11
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       0       0
//  Maximum:         0       3       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       3       0
//  Maximum:         0      10       0
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
    fn conjugation(self) -> Self {
        self
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
    fn conjugation(self) -> Self {
        self
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
        Flector::from_groups(/* e1, e2, e3, e4 */ self.group0() * Simd32x4::from(-1.0), /* e423, e431, e412, e321 */ self.group1())
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
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn conjugation(self) -> Self {
        Line::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        )
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
            // e41, e42, e43, e1234
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
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
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn conjugation(self) -> Self {
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
impl std::ops::Div<ConjugationPrefixOrPostfix> for Point {
    type Output = Point;
    fn div(self, _rhs: ConjugationPrefixOrPostfix) -> Self::Output {
        self.conjugation()
    }
}
impl std::ops::DivAssign<ConjugationPrefixOrPostfix> for Point {
    fn div_assign(&mut self, _rhs: ConjugationPrefixOrPostfix) {
        *self = self.conjugation()
    }
}
impl Conjugation for Point {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn conjugation(self) -> Self {
        Point::from_groups(/* e1, e2, e3, e4 */ self.group0() * Simd32x4::from(-1.0))
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
