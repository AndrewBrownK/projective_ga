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
//  Maximum:         0       2       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         0       1       0       0
//  Average:         0       2       0       0
//  Maximum:         0       8       0       0
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
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn auto_morphism(self) -> Self {
        Flector::from_groups(
            // e1, e2, e3, e4
            self.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412, e321
            self.group1() * Simd32x4::from(-1.0),
        )
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
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn auto_morphism(self) -> Self {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e321] * -1.0)
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
    fn auto_morphism(self) -> Self {
        self
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
    fn auto_morphism(self) -> Self {
        self
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
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn auto_morphism(self) -> Self {
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
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
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
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn auto_morphism(self) -> Self {
        Plane::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AutoMorphismPrefixOrPostfix> for Point {
    type Output = Point;
    fn div(self, _rhs: AutoMorphismPrefixOrPostfix) -> Self::Output {
        self.auto_morphism()
    }
}
impl std::ops::DivAssign<AutoMorphismPrefixOrPostfix> for Point {
    fn div_assign(&mut self, _rhs: AutoMorphismPrefixOrPostfix) {
        *self = self.auto_morphism()
    }
}
impl AutoMorphism for Point {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn auto_morphism(self) -> Self {
        Point::from_groups(/* e1, e2, e3, e4 */ self.group0() * Simd32x4::from(-1.0))
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
