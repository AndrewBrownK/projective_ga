// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 7
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
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for AntiScalar {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for Flector {
    type Output = Flector;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for Flector {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for Flector {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for Horizon {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for Horizon {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for Origin {
    type Output = Origin;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for Origin {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for Origin {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for Plane {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for Point {
    type Output = Point;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for Point {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for Point {
    fn constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<ConstraintValidPrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: ConstraintValidPrefixOrPostfix) -> Self::Output {
        self.constraint_valid()
    }
}
impl std::ops::DivAssign<ConstraintValidPrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: ConstraintValidPrefixOrPostfix) {
        *self = self.constraint_valid()
    }
}
impl ConstraintValid for Scalar {
    fn constraint_valid(self) -> Self {
        self
    }
}
