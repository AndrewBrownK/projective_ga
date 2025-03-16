// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 8
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       3       0
//  Average:         1       2       0
//  Maximum:         3       7       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       7       0
//  Average:         2       4       0
//  Maximum:         8      13       3
impl std::ops::Div<AntiFixPrefixOrPostfix> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        1
    // no simd        0        3        3
    fn anti_fix(self) -> Self {
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ (self.group0().xyz() * Simd32x3::from(-1.0) / self.group0().www()).with_w(-1.0))
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for AntiPlane {
    type Output = AntiPlane;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for AntiPlane {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        8       13        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                f32::powi(self[e1], 3) * -1.0,
                f32::powi(self[e2], 3) * -1.0,
                f32::powi(self[e3], 3) * -1.0,
                self[e3] * self[e3] * self[e5] * -1.0,
            ]) - (Simd32x4::powi(self.group0().yxxx(), 2) * self.group0())
                - (Simd32x4::powi(self.group0().zzyy(), 2) * self.group0()),
        )
    }
}
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
        AntiScalar::from_groups(/* e12345 */ 1.0)
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for FlatPoint {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for FlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        0        1
    // no simd        0        0        3
    fn anti_fix(self) -> Self {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ (self.group0().xyz() / self.group0().www()).with_w(1.0))
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
            // e4235, e4315, e4125, e3215
            Simd32x4::from([f32::powi(self[e4235], 3), f32::powi(self[e4315], 3), f32::powi(self[e4125], 3), self[e4125] * self[e4125] * self[e3215]])
                + (Simd32x4::powi(self.group0().yxxx(), 2) * self.group0())
                + (Simd32x4::powi(self.group0().zzyy(), 2) * self.group0()),
        )
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for RoundPoint {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        7        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = 2.0 * (self[e4] * self[e5]) - self[e1] * self[e1] - self[e2] * self[e2] - self[e3] * self[e3];
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e5
            geometric_anti_product_g0 * self[e5],
        )
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for Scalar {
    fn anti_fix(self) -> Self {
        Scalar::from_groups(/* scalar */ -1.0)
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for Sphere {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        7        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125] - 2.0 * (self[e3215] * self[e1234]);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e1234
            geometric_anti_product_g0 * self[e1234],
        )
    }
}
