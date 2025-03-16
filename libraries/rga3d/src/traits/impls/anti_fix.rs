// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 4
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         2       3       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         2       2       0
//  Maximum:         8       9       3
impl std::ops::Div<AntiFixPrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiScalar {
    fn anti_fix(self) -> Self {
        AntiScalar::from_groups(/* e1234 */ 1.0)
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for Origin {
    type Output = Origin;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for Origin {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for Origin {
    fn anti_fix(self) -> Self {
        Origin::from_groups(/* e4 */ 1.0)
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        8        9        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([f32::powi(self[e423], 3), f32::powi(self[e431], 3), f32::powi(self[e412], 3), self[e412] * self[e412] * self[e321]])
                + (Simd32x4::powi(self.group0().yxxx(), 2) * self.group0())
                + (Simd32x4::powi(self.group0().zzyy(), 2) * self.group0()),
        )
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for Point {
    type Output = Point;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for Point {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for Point {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        0        1
    // no simd        0        0        3
    fn anti_fix(self) -> Self {
        Point::from_groups(/* e1, e2, e3, e4 */ (self.group0().xyz() / self.group0().www()).with_w(1.0))
    }
}
