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
impl std::ops::Div<FixPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for AntiDipoleOnOrigin {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn fix(self) -> Self {
        use crate::elements::*;
        AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(1.0 / self[e321]) * self.group0())
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for AntiFlatOrigin {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for AntiFlatOrigin {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for AntiFlatOrigin {
    fn fix(self) -> Self {
        AntiFlatOrigin::from_groups(/* e321 */ 1.0)
    }
}
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
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn fix(self) -> Self {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(1.0 / self[e321]) * self.group0())
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for AntiFlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for AntiFlectorOnOrigin {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn fix(self) -> Self {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]);
        AntiFlectorOnOrigin::from_groups(
            // e321, e1, e2, e3
            Simd32x4::from((reverse_g0[1] * self[e1]) + (reverse_g0[2] * self[e2]) + (reverse_g0[3] * self[e3]) - (reverse_g0[0] * self[e321])) * self.group0(),
        )
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for AntiLineOnOrigin {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn fix(self) -> Self {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        AntiLineOnOrigin::from_groups(
            // e23, e31, e12
            Simd32x3::from(-(reverse_g0[0] * self[e23]) - (reverse_g0[1] * self[e31]) - (reverse_g0[2] * self[e12])) * self.group0(),
        )
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for AntiMotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for AntiMotorOnOrigin {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn fix(self) -> Self {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiMotorOnOrigin::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from((reverse_g0[3] * self[scalar]) - (reverse_g0[0] * self[e23]) - (reverse_g0[1] * self[e31]) - (reverse_g0[2] * self[e12])) * self.group0(),
        )
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
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn fix(self) -> Self {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3]) * self.group0(),
        )
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for AntiPlaneOnOrigin {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for AntiPlaneOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn fix(self) -> Self {
        use crate::elements::*;
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ Simd32x3::from(self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3]) * self.group0())
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
impl std::ops::Div<FixPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for AntiSphereOnOrigin {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn fix(self) -> Self {
        use crate::elements::*;
        AntiSphereOnOrigin::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3]) * self.group0(),
        )
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for DipoleOnOrigin {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn fix(self) -> Self {
        use crate::elements::*;
        DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from(1.0 / self[e45] * -1.0) * self.group0())
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for FlatOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for FlatOrigin {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for FlatOrigin {
    fn fix(self) -> Self {
        FlatOrigin::from_groups(/* e45 */ -1.0)
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
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn fix(self) -> Self {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(1.0 / self[e45] * -1.0) * self.group0())
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for FlectorOnOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for FlectorOnOrigin {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn fix(self) -> Self {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]);
        FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from((reverse_g0[0] * self[e45]) - (reverse_g0[1] * self[e4235]) - (reverse_g0[2] * self[e4315]) - (reverse_g0[3] * self[e4125])) * self.group0(),
        )
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for LineOnOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for LineOnOrigin {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for LineOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn fix(self) -> Self {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        LineOnOrigin::from_groups(
            // e415, e425, e435
            Simd32x3::from((reverse_g0[0] * self[e415]) + (reverse_g0[1] * self[e425]) + (reverse_g0[2] * self[e435])) * self.group0(),
        )
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for MotorOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for MotorOnOrigin {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn fix(self) -> Self {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MotorOnOrigin::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from((reverse_g0[0] * self[e415]) + (reverse_g0[1] * self[e425]) + (reverse_g0[2] * self[e435]) - (reverse_g0[3] * self[e12345])) * self.group0(),
        )
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
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn fix(self) -> Self {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(-self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125]) * self.group0(),
        )
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for PlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for PlaneOnOrigin {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for PlaneOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn fix(self) -> Self {
        use crate::elements::*;
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::from(-self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125]) * self.group0(),
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
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        7        0
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
impl std::ops::Div<FixPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for RoundPointAtOrigin {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn fix(self) -> Self {
        use crate::elements::*;
        RoundPointAtOrigin::from_groups(/* e4, e5 */ Simd32x2::from(f32::powf(self[e4], -0.5) * f32::powf(self[e5], -0.5) * -2.0) * self.group0())
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
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        7        0
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
impl std::ops::Div<FixPrefixOrPostfix> for SphereAtOrigin {
    type Output = SphereAtOrigin;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for SphereAtOrigin {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn fix(self) -> Self {
        use crate::elements::*;
        SphereAtOrigin::from_groups(
            // e3215, e1234
            Simd32x2::from(f32::powf(self[e3215], -0.5) * f32::powf(self[e1234], -0.5) * 2.0) * self.group0(),
        )
    }
}
impl std::ops::Div<FixPrefixOrPostfix> for SphereOnOrigin {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: FixPrefixOrPostfix) -> Self::Output {
        self.fix()
    }
}
impl std::ops::DivAssign<FixPrefixOrPostfix> for SphereOnOrigin {
    fn div_assign(&mut self, _rhs: FixPrefixOrPostfix) {
        *self = self.fix()
    }
}
impl Fix for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn fix(self) -> Self {
        use crate::elements::*;
        SphereOnOrigin::from_groups(
            // e4235, e4315, e4125, e1234
            Simd32x4::from(-self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125]) * self.group0(),
        )
    }
}
