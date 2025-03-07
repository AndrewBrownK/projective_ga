// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 6
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
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for Horizon {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for Horizon {
    fn anti_constraint_valid(self) -> Self {
        self
    }
}
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for Origin {
    type Output = Origin;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for Origin {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for Origin {
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
impl std::ops::Div<AntiConstraintValidPrefixOrPostfix> for Point {
    type Output = Point;
    fn div(self, _rhs: AntiConstraintValidPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl std::ops::DivAssign<AntiConstraintValidPrefixOrPostfix> for Point {
    fn div_assign(&mut self, _rhs: AntiConstraintValidPrefixOrPostfix) {
        *self = self.anti_constraint_valid()
    }
}
impl AntiConstraintValid for Point {
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
