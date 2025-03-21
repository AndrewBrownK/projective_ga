// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 8
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         2       7       0     N/A
//  Average:         1       5       0     N/A
//  Maximum:         3      14       1     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         3      10       0       0
//  Average:         2       9       0       0
//  Maximum:         8      23       1       0
impl std::ops::Div<FixPrefixOrPostfix> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        0        1        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        1        1      N/A
    //  no simd        0        3        1        0
    fn fix(self) -> Self {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ (Simd32x3::from(1.0 / self[e321]) * self.group0().xyz()).with_w(1.0))
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for AntiPlane {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       11        0        0
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2       14        0      N/A
    //  no simd        8       23        0        0
    fn fix(self) -> Self {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x4::from([self[e1] * self[e1], self[e2] * self[e2], self[e3] * self[e3], self[e1] * self[e1]]) * self.group0())
                + (Simd32x4::from([self[e2] * self[e2], self[e1] * self[e1], self[e1] * self[e1], self[e2] * self[e2]]) * self.group0())
                + (self.group0() * Simd32x2::from(self[e3] * self[e3]).with_zw(self[e2] * self[e2], self[e3] * self[e3])),
        )
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for AntiScalar {
    fn fix(self) -> Self {
        AntiScalar::from_groups(/* e12345 */ -1.0)
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for FlatPoint {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        1        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        1      N/A
    //  no simd        0        4        1        0
    fn fix(self) -> Self {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ (Simd32x3::from(-1.0 / self[e45]) * self.group0().xyz()).with_w(-1.0))
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
    //           add/sub      mul      div      pow
    //      f32        0       11        0        0
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2       14        0      N/A
    //  no simd        8       23        0        0
    fn fix(self) -> Self {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            -(Simd32x4::from([self[e4235] * self[e4235], self[e4315] * self[e4315], self[e4125] * self[e4125], self[e4235] * self[e4235]]) * self.group0())
                - (Simd32x4::from([self[e4315] * self[e4315], self[e4235] * self[e4235], self[e4235] * self[e4235], self[e4315] * self[e4315]]) * self.group0())
                - (self.group0() * Simd32x2::from(self[e4125] * self[e4125]).with_zw(self[e4315] * self[e4315], self[e4125] * self[e4125])),
        )
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for RoundPoint {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       10        0        0
    fn fix(self) -> Self {
        use crate::elements::*;
        let geometric_product_g0 = self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] - 2.0 * (self[e4] * self[e5]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_product_g0) * self.group0(),
            // e5
            geometric_product_g0 * self[e5],
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
impl std::ops::Div<FixPrefixOrPostfix> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for Sphere {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       10        0        0
    fn fix(self) -> Self {
        use crate::elements::*;
        let geometric_product_g0 = 2.0 * (self[e3215] * self[e1234]) - self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125];
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(geometric_product_g0) * self.group0(),
            // e1234
            geometric_product_g0 * self[e1234],
        )
    }
}
