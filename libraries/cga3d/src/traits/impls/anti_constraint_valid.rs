// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 8
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for AntiFlatPoint {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for AntiPlane {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for AntiPlane {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for AntiScalar {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for FlatPoint {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for FlatPoint {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for Plane {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for RoundPoint {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for RoundPoint {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for Scalar {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for Sphere {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for Sphere {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
