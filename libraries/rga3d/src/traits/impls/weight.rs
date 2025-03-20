// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 9
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
impl std::ops::Div<WeightPrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightPrefixOrPostfix) -> Self::Output {
        self.weight()
    }
}
impl std::ops::DivAssign<WeightPrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: WeightPrefixOrPostfix) {
        *self = self.weight()
    }
}
impl Weight for AntiScalar {
    type Output = AntiScalar;
    fn weight(self) -> Self::Output {
        self
    }
}
impl std::ops::Div<WeightPrefixOrPostfix> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: WeightPrefixOrPostfix) -> Self::Output {
        self.weight()
    }
}
impl Weight for DualNum {
    type Output = AntiScalar;
    fn weight(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234])
    }
}
impl std::ops::Div<WeightPrefixOrPostfix> for Flector {
    type Output = Flector;
    fn div(self, _rhs: WeightPrefixOrPostfix) -> Self::Output {
        self.weight()
    }
}
impl std::ops::DivAssign<WeightPrefixOrPostfix> for Flector {
    fn div_assign(&mut self, _rhs: WeightPrefixOrPostfix) {
        *self = self.weight()
    }
}
impl Weight for Flector {
    type Output = Flector;
    fn weight(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e423, e431, e412, e321
            self.group1().xyz().with_w(0.0),
        )
    }
}
impl std::ops::Div<WeightPrefixOrPostfix> for Line {
    type Output = Line;
    fn div(self, _rhs: WeightPrefixOrPostfix) -> Self::Output {
        self.weight()
    }
}
impl std::ops::DivAssign<WeightPrefixOrPostfix> for Line {
    fn div_assign(&mut self, _rhs: WeightPrefixOrPostfix) {
        *self = self.weight()
    }
}
impl Weight for Line {
    type Output = Line;
    fn weight(self) -> Self::Output {
        Line::from_groups(/* e41, e42, e43 */ self.group0(), /* e23, e31, e12 */ Simd32x3::from(0.0))
    }
}
impl std::ops::Div<WeightPrefixOrPostfix> for Motor {
    type Output = Motor;
    fn div(self, _rhs: WeightPrefixOrPostfix) -> Self::Output {
        self.weight()
    }
}
impl std::ops::DivAssign<WeightPrefixOrPostfix> for Motor {
    fn div_assign(&mut self, _rhs: WeightPrefixOrPostfix) {
        *self = self.weight()
    }
}
impl Weight for Motor {
    type Output = Motor;
    fn weight(self) -> Self::Output {
        Motor::from_groups(/* e41, e42, e43, e1234 */ self.group0(), /* e23, e31, e12, scalar */ Simd32x4::from(0.0))
    }
}
impl std::ops::Div<WeightPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: WeightPrefixOrPostfix) -> Self::Output {
        self.weight()
    }
}
impl std::ops::DivAssign<WeightPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: WeightPrefixOrPostfix) {
        *self = self.weight()
    }
}
impl Weight for MultiVector {
    type Output = MultiVector;
    fn weight(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, self[e1234]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            self.group4().xyz().with_w(0.0),
        )
    }
}
impl std::ops::Div<WeightPrefixOrPostfix> for Origin {
    type Output = Origin;
    fn div(self, _rhs: WeightPrefixOrPostfix) -> Self::Output {
        self.weight()
    }
}
impl std::ops::DivAssign<WeightPrefixOrPostfix> for Origin {
    fn div_assign(&mut self, _rhs: WeightPrefixOrPostfix) {
        *self = self.weight()
    }
}
impl Weight for Origin {
    type Output = Origin;
    fn weight(self) -> Self::Output {
        self
    }
}
impl std::ops::Div<WeightPrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: WeightPrefixOrPostfix) -> Self::Output {
        self.weight()
    }
}
impl std::ops::DivAssign<WeightPrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: WeightPrefixOrPostfix) {
        *self = self.weight()
    }
}
impl Weight for Plane {
    type Output = Plane;
    fn weight(self) -> Self::Output {
        Plane::from_groups(/* e423, e431, e412, e321 */ self.group0().xyz().with_w(0.0))
    }
}
impl std::ops::Div<WeightPrefixOrPostfix> for Point {
    type Output = Origin;
    fn div(self, _rhs: WeightPrefixOrPostfix) -> Self::Output {
        self.weight()
    }
}
impl Weight for Point {
    type Output = Origin;
    fn weight(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4])
    }
}
