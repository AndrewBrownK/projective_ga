// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 4
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         0       1       0     N/A
//  Average:         0       3       0     N/A
//  Maximum:         2      11       1     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         0       3       0       0
//  Average:         2       6       0       0
//  Maximum:         8      23       3       0
impl std::ops::Div<FixPrefixOrPostfix> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for Horizon {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for Horizon {
    fn fix(self) -> Self {
        Horizon::from_groups(/* e321 */ 1.0)
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for Plane {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        1      N/A
    // no simd        0        3        3        0
    fn fix(self) -> Self {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ (self.group0().xyz() / Simd32x4::from(self[e321]).xyz()).with_w(1.0))
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for Point {
    type Output = Point;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for Point {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        7        0        0
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd        2       11        0      N/A
    //  no simd        8       23        0        0
    fn fix(self) -> Self {
        use crate::elements::*;
        Point::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([self[e2] * self[e2], self[e1] * self[e1], self[e1] * self[e1], self[e2] * self[e2]]) * self.group0())
                + (self.group0() * Simd32x2::from(self[e3] * self[e3]).with_zw(self[e2] * self[e2], self[e3] * self[e3]))
                + (Simd32x4::powi(self.group0().xyzx(), 2) * self.group0()),
        )
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for Scalar {
    fn fix(self) -> Self {
        Scalar::from_groups(/* scalar */ 1.0)
    }
}
