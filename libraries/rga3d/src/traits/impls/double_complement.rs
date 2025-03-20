// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 11
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       0       0
//  Maximum:         0       2       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       2       0
//  Maximum:         0       8       0
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for AntiScalar {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for DualNum {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for DualNum {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for Flector {
    type Output = Flector;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for Flector {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Flector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn double_complement(self) -> Self {
        Flector::from_groups(
            // e1, e2, e3, e4
            self.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412, e321
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for Horizon {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn double_complement(self) -> Self {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e321] * -1.0)
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for Line {
    type Output = Line;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for Line {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Line {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for Motor {
    type Output = Motor;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for Motor {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Motor {
    fn double_complement(self) -> Self {
        self
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for MultiVector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn double_complement(self) -> Self {
        MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1() * Simd32x4::from(-1.0),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for Origin {
    type Output = Origin;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for Origin {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn double_complement(self) -> Self {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4] * -1.0)
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Plane {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn double_complement(self) -> Self {
        Plane::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for Point {
    type Output = Point;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for Point {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Point {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn double_complement(self) -> Self {
        Point::from_groups(/* e1, e2, e3, e4 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<DoubleComplementPrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: DoubleComplementPrefixOrPostfix) -> Self::Output {
        self.double_complement()
    }
}
impl std::ops::DivAssign<DoubleComplementPrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: DoubleComplementPrefixOrPostfix) {
        *self = self.double_complement()
    }
}
impl DoubleComplement for Scalar {
    fn double_complement(self) -> Self {
        self
    }
}
