// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 24
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       2       0
//  Average:         1       2       0
//  Maximum:         3       6       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       4       0
//  Average:         1       5       0
//  Maximum:         3      12       1
impl std::ops::Div<AntiFixPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for AntiDipoleOnOrigin {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        return AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(1.0 / self[e321] * -1.0) * self.group0());
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for AntiFlatOrigin {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for AntiFlatOrigin {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiFlatOrigin {
    fn anti_fix(self) -> Self {
        return AntiFlatOrigin::from_groups(/* e321 */ -1.0);
    }
}
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
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(1.0 / self[e321] * -1.0) * self.group0());
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for AntiFlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for AntiFlectorOnOrigin {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]);
        return AntiFlectorOnOrigin::from_groups(
            // e321, e1, e2, e3
            Simd32x4::from((anti_reverse_g0[0] * self[e321]) - (anti_reverse_g0[1] * self[e1]) - (anti_reverse_g0[2] * self[e2]) - (anti_reverse_g0[3] * self[e3])) * self.group0(),
        );
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for AntiLineOnOrigin {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        return AntiLineOnOrigin::from_groups(
            // e23, e31, e12
            Simd32x3::from((anti_reverse_g0[0] * self[e23]) + (anti_reverse_g0[1] * self[e31]) + (anti_reverse_g0[2] * self[e12])) * self.group0(),
        );
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for AntiMotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for AntiMotorOnOrigin {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return AntiMotorOnOrigin::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from((anti_reverse_g0[0] * self[e23]) + (anti_reverse_g0[1] * self[e31]) + (anti_reverse_g0[2] * self[e12]) - (anti_reverse_g0[3] * self[scalar]))
                * self.group0(),
        );
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
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(-self[e1] * self[e1] - self[e2] * self[e2] - self[e3] * self[e3]) * self.group0(),
        );
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for AntiPlaneOnOrigin {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiPlaneOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        return AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ Simd32x3::from(-self[e1] * self[e1] - self[e2] * self[e2] - self[e3] * self[e3]) * self.group0());
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
        return AntiScalar::from_groups(/* e12345 */ 1.0);
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for AntiSphereOnOrigin {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        return AntiSphereOnOrigin::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(-self[e1] * self[e1] - self[e2] * self[e2] - self[e3] * self[e3]) * self.group0(),
        );
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for DipoleOnOrigin {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        return DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from(1.0 / self[e45]) * self.group0());
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for FlatOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for FlatOrigin {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for FlatOrigin {
    fn anti_fix(self) -> Self {
        return FlatOrigin::from_groups(/* e45 */ 1.0);
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
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(1.0 / self[e45]) * self.group0());
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for FlectorOnOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for FlectorOnOrigin {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]);
        return FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from((anti_reverse_g0[1] * self[e4235]) + (anti_reverse_g0[2] * self[e4315]) + (anti_reverse_g0[3] * self[e4125]) - (anti_reverse_g0[0] * self[e45]))
                * self.group0(),
        );
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for LineOnOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for LineOnOrigin {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for LineOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        return LineOnOrigin::from_groups(
            // e415, e425, e435
            Simd32x3::from(-(anti_reverse_g0[0] * self[e415]) - (anti_reverse_g0[1] * self[e425]) - (anti_reverse_g0[2] * self[e435])) * self.group0(),
        );
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for MotorOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for MotorOnOrigin {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return MotorOnOrigin::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from((anti_reverse_g0[3] * self[e12345]) - (anti_reverse_g0[0] * self[e415]) - (anti_reverse_g0[1] * self[e425]) - (anti_reverse_g0[2] * self[e435]))
                * self.group0(),
        );
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
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125]) * self.group0(),
        );
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for PlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for PlaneOnOrigin {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for PlaneOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        return PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::from(self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125]) * self.group0(),
        );
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
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e5
            geometric_anti_product_g0 * self[e5],
        );
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for RoundPointAtOrigin {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        return RoundPointAtOrigin::from_groups(/* e4, e5 */ Simd32x2::from(f32::powf(self[e4], -0.5) * f32::powf(self[e5], -0.5) * 2.0) * self.group0());
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
        return Scalar::from_groups(/* scalar */ -1.0);
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
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e1234
            geometric_anti_product_g0 * self[e1234],
        );
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for SphereAtOrigin {
    type Output = SphereAtOrigin;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for SphereAtOrigin {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        return SphereAtOrigin::from_groups(
            // e3215, e1234
            Simd32x2::from(f32::powf(self[e3215], -0.5) * f32::powf(self[e1234], -0.5) * -2.0) * self.group0(),
        );
    }
}
impl std::ops::Div<AntiFixPrefixOrPostfix> for SphereOnOrigin {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: AntiFixPrefixOrPostfix) -> Self::Output {
        self.anti_fix()
    }
}
impl std::ops::DivAssign<AntiFixPrefixOrPostfix> for SphereOnOrigin {
    fn div_assign(&mut self, _rhs: AntiFixPrefixOrPostfix) {
        *self = self.anti_fix()
    }
}
impl AntiFix for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        return SphereOnOrigin::from_groups(
            // e4235, e4315, e4125, e1234
            Simd32x4::from(self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125]) * self.group0(),
        );
    }
}
