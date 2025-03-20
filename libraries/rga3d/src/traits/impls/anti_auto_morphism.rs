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
            // e1, e2, e3, e4
            self.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412, e321
            self.group1() * Simd32x4::from(-1.0),
        )
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
        Horizon::from_groups(/* e321 */ self[e321] * -1.0)
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
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_auto_morphism(self) -> Self {
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
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_auto_morphism(self) -> Self {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4] * -1.0)
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
        Plane::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Div<AntiAutoMorphismPrefixOrPostfix> for Point {
    type Output = Point;
    fn div(self, _rhs: AntiAutoMorphismPrefixOrPostfix) -> Self::Output {
        self.anti_auto_morphism()
    }
}
impl std::ops::DivAssign<AntiAutoMorphismPrefixOrPostfix> for Point {
    fn div_assign(&mut self, _rhs: AntiAutoMorphismPrefixOrPostfix) {
        *self = self.anti_auto_morphism()
    }
}
impl AntiAutoMorphism for Point {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_auto_morphism(self) -> Self {
        Point::from_groups(/* e1, e2, e3, e4 */ self.group0() * Simd32x4::from(-1.0))
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
    fn anti_auto_morphism(self) -> Self {
        self
    }
}
