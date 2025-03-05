// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 49
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       2       0
//  Average:         2       2       0
//  Maximum:        23      36       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       8       0
//  Average:         2       8       0
//  Maximum:        23      65       1
impl std::ops::Div<UnitizePrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = AntiCircleOnOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for AntiCircleOnOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        2        6        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2));
        return AntiCircleOnOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product[e12345]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product[e12345]) * self.group1(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for AntiCircleRotor {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        2       11        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2));
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(geometric_anti_product[e12345]) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(geometric_anti_product[e12345]) * self.group2(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        2       10        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2));
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product[e12345]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product[e12345]) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(geometric_anti_product[e12345]) * self.group2(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for AntiCircleRotorOnOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        2        7        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz());
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2));
        return AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(geometric_anti_product[e12345]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product[e12345]) * self.group1(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for AntiDipoleInversion {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       15        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0().with_w(self[e4]).wxyz());
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e45], 2) + f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(geometric_anti_product[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(geometric_anti_product[e12345]) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(geometric_anti_product[e12345]) * self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(geometric_anti_product[e12345]) * self.group3(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        8        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e45], 2) + f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2),
        );
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(geometric_anti_product[e12345]) * self.group0(),
            // e4, e1, e2, e3
            Simd32x4::from(geometric_anti_product[e12345]) * self.group1(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3       11        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e45], 2) + f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2),
        );
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from(geometric_anti_product[e12345]) * self.group0(),
            // e415, e425, e435
            Simd32x3::from(geometric_anti_product[e12345]) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(geometric_anti_product[e12345]) * self.group2(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for AntiDipoleOnOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        return AntiDipoleOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2)) * self.group0(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for AntiDualNum {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn unitize(self) -> Self {
        use crate::elements::*;
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(1.0 / self[e1234]) * self.group0());
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for AntiSphereOnOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn unitize(self) -> Self {
        use crate::elements::*;
        return AntiSphereOnOrigin::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(1.0 / self[e4]) * self.group0());
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for AntiVersorEvenOnOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        8        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2),
        );
        return AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(geometric_anti_product[e12345]) * self.group0(),
            // e23, e31, e12, e1234
            Simd32x4::from(geometric_anti_product[e12345]) * self.group1(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for Circle {
    type Output = Circle;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for Circle {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        2       10        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2));
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(geometric_anti_product[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(geometric_anti_product[e12345]) * self.group1(),
            // e235, e315, e125
            Simd32x3::from(geometric_anti_product[e12345]) * self.group2(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for CircleAligningOrigin {
    type Output = CircleAligningOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for CircleAligningOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        2        9        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2));
        return CircleAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(geometric_anti_product[e12345]) * self.group0(),
            // e415, e425, e435
            Simd32x3::from(geometric_anti_product[e12345]) * self.group1(),
            // e235, e315, e125
            Simd32x3::from(geometric_anti_product[e12345]) * self.group2(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for CircleAtOrigin {
    type Output = CircleAtOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for CircleAtOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        2        6        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2));
        return CircleAtOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(geometric_anti_product[e12345]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(geometric_anti_product[e12345]) * self.group1(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for CircleOnOrigin {
    type Output = CircleOnOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for CircleOnOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        2        6        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2));
        return CircleOnOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(geometric_anti_product[e12345]) * self.group0(),
            // e415, e425, e435
            Simd32x3::from(geometric_anti_product[e12345]) * self.group1(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for CircleOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        2        7        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2));
        return CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(geometric_anti_product[e12345]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(geometric_anti_product[e12345]) * self.group1(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for CircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for CircleRotor {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        2       11        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2));
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(geometric_anti_product[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(geometric_anti_product[e12345]) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(geometric_anti_product[e12345]) * self.group2(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for CircleRotorAligningOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        2       10        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2));
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(geometric_anti_product[e12345]) * self.group0(),
            // e415, e425, e435
            Simd32x3::from(geometric_anti_product[e12345]) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(geometric_anti_product[e12345]) * self.group2(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for CircleRotorOnOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        2        7        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2));
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(geometric_anti_product[e12345]) * self.group0(),
            // e415, e425, e435
            Simd32x3::from(geometric_anti_product[e12345]) * self.group1(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for Dipole {
    type Output = Dipole;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for Dipole {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        2       10        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2));
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(geometric_anti_product[e12345]) * self.group1(),
            // e15, e25, e35
            Simd32x3::from(geometric_anti_product[e12345]) * self.group2(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for DipoleAligningOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        2        7        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz());
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2));
        return DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(geometric_anti_product[e12345]) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(geometric_anti_product[e12345]) * self.group1(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for DipoleAtOrigin {
    type Output = DipoleAtOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for DipoleAtOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        2        6        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2));
        return DipoleAtOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product[e12345]) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(geometric_anti_product[e12345]) * self.group1(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for DipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for DipoleInversion {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       15        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0().with_w(self[e1234]));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2),
        );
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(geometric_anti_product[e12345]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(geometric_anti_product[e12345]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(geometric_anti_product[e12345]) * self.group3(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = DipoleInversionAligningOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for DipoleInversionAligningOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3       12        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2),
        );
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(geometric_anti_product[e12345]) * self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from(geometric_anti_product[e12345]) * self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(geometric_anti_product[e12345]) * self.group2(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for DipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        8        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2),
        );
        return DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from(geometric_anti_product[e12345]) * self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from(geometric_anti_product[e12345]) * self.group1(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for DipoleInversionOnOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        8        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2),
        );
        return DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(geometric_anti_product[e12345]) * self.group0(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(geometric_anti_product[e12345]) * self.group1(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3       11        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2),
        );
        return DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from(geometric_anti_product[e12345]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product[e12345]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(geometric_anti_product[e12345]) * self.group2(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for DipoleOnOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz());
        return DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2)) * self.group0(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for DipoleOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        2        9        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2));
        return DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product[e12345]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product[e12345]) * self.group1(),
            // e15, e25, e35
            Simd32x3::from(geometric_anti_product[e12345]) * self.group2(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for DualNum {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn unitize(self) -> Self {
        use crate::elements::*;
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(1.0 / self[e4]) * self.group0());
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       23        0
    //    simd2        0        2        0
    //    simd3        0        6        0
    //    simd4        0        5        0
    // Totals...
    // yes simd       23       36        0
    //  no simd       23       65        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let sub_type = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group3().xyz().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        let other = Infinity::from_groups(/* e5 */ 1.0);
        let wedge = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e5] * sub_type[e1234]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e5] * sub_type[e4]),
            // e15, e25, e35
            Simd32x3::from(other[e5]) * sub_type.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (Simd32x3::from(other[e5]) * sub_type.group3().xyz()).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e5] * sub_type[e423], other[e5] * sub_type[e431], other[e5] * sub_type[e412]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]),
            // e3215
            0.0,
        );
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            2.0 * (wedge[e4] * wedge[e5])
                + 2.0 * (wedge[e423] * wedge[e235])
                + 2.0 * (wedge[e431] * wedge[e315])
                + 2.0 * (wedge[e412] * wedge[e125])
                + f32::powi(wedge[e12345], 2)
                + f32::powi(wedge[e45], 2)
                + f32::powi(wedge[e415], 2)
                + f32::powi(wedge[e425], 2)
                + f32::powi(wedge[e435], 2)
                + f32::powi(wedge[e4235], 2)
                + f32::powi(wedge[e4315], 2)
                + f32::powi(wedge[e4125], 2)
                - f32::powi(wedge[scalar], 2)
                - f32::powi(wedge[e1], 2)
                - f32::powi(wedge[e2], 2)
                - f32::powi(wedge[e3], 2)
                - f32::powi(wedge[e23], 2)
                - f32::powi(wedge[e31], 2)
                - f32::powi(wedge[e12], 2)
                - f32::powi(wedge[e321], 2)
                - 2.0 * (wedge[e41] * wedge[e15])
                - 2.0 * (wedge[e42] * wedge[e25])
                - 2.0 * (wedge[e43] * wedge[e35])
                - 2.0 * (wedge[e1234] * wedge[e3215]),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(geometric_anti_product[e12345]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product[e12345]) * self.group1(),
            // e5
            geometric_anti_product[e12345] * self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(geometric_anti_product[e12345]) * self.group3(),
            // e15, e25, e35
            Simd32x3::from(geometric_anti_product[e12345]) * self.group4(),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product[e12345]) * self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(geometric_anti_product[e12345]) * self.group6(),
            // e423, e431, e412
            Simd32x3::from(geometric_anti_product[e12345]) * self.group7(),
            // e235, e315, e125
            Simd32x3::from(geometric_anti_product[e12345]) * self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(geometric_anti_product[e12345]) * self.group9(),
            // e3215
            geometric_anti_product[e12345] * self[e3215],
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for NullCircleAtOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for NullCircleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        return NullCircleAtOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2)) * self.group0(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for NullDipoleAtOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for NullDipoleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return NullDipoleAtOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2)) * self.group0(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for NullDipoleInversionAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for NullDipoleInversionAtOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for NullDipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0());
        return NullDipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2)) * self.group0(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for NullSphereAtOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for NullSphereAtOrigin {
    fn unitize(self) -> Self {
        return NullSphereAtOrigin::from_groups(/* e1234 */ 1.0);
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for NullVersorEvenAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for NullVersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for NullVersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0().wxyz());
        return NullVersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            Simd32x4::from(f32::powi(wedge[e45], 2) + f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2)) * self.group0(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for Origin {
    type Output = Origin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for Origin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for Origin {
    fn unitize(self) -> Self {
        return Origin::from_groups(/* e4 */ 1.0);
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for RoundPoint {
    type Output = RoundPoint;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for RoundPoint {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ 1.0 / self[e4]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product[e12345]) * self.group0(),
            // e5
            geometric_anti_product[e12345] * self[e5],
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for RoundPointAtOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn unitize(self) -> Self {
        use crate::elements::*;
        return RoundPointAtOrigin::from_groups(/* e4, e5 */ Simd32x2::from(1.0 / self[e4]) * self.group0());
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for Sphere {
    type Output = Sphere;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for Sphere {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ 1.0 / self[e1234]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(geometric_anti_product[e12345]) * self.group0(),
            // e1234
            geometric_anti_product[e12345] * self[e1234],
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for SphereAtOrigin {
    type Output = SphereAtOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for SphereAtOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn unitize(self) -> Self {
        use crate::elements::*;
        return SphereAtOrigin::from_groups(/* e3215, e1234 */ Simd32x2::from(1.0 / self[e1234]) * self.group0());
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for SphereOnOrigin {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for SphereOnOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn unitize(self) -> Self {
        use crate::elements::*;
        return SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ Simd32x4::from(1.0 / self[e1234]) * self.group0());
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for VersorEven {
    type Output = VersorEven;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for VersorEven {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       16        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e45], 2) + f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(geometric_anti_product[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(geometric_anti_product[e12345]) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(geometric_anti_product[e12345]) * self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product[e12345]) * self.group3(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for VersorEvenAligningOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3       12        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e45], 2) + f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2),
        );
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(geometric_anti_product[e12345]) * self.group0(),
            // e415, e425, e435, e4
            Simd32x4::from(geometric_anti_product[e12345]) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(geometric_anti_product[e12345]) * self.group2(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = VersorEvenAtOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for VersorEvenAtOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        8        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0().wxyz());
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e45], 2) + f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2),
        );
        return VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            Simd32x4::from(geometric_anti_product[e12345]) * self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(geometric_anti_product[e12345]) * self.group1(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = VersorEvenOnOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for VersorEvenOnOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        8        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e45], 2) + f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2),
        );
        return VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(geometric_anti_product[e12345]) * self.group0(),
            // e415, e425, e435, e4
            Simd32x4::from(geometric_anti_product[e12345]) * self.group1(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3       12        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e45], 2) + f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2),
        );
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(geometric_anti_product[e12345]) * self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(geometric_anti_product[e12345]) * self.group1(),
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product[e12345]) * self.group2(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for VersorOdd {
    type Output = VersorOdd;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for VersorOdd {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       16        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(geometric_anti_product[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(geometric_anti_product[e12345]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(geometric_anti_product[e12345]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(geometric_anti_product[e12345]) * self.group3(),
        );
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for VersorOddOrthogonalOrigin {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3       12        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2),
        );
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(geometric_anti_product[e12345]) * self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from(geometric_anti_product[e12345]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(geometric_anti_product[e12345]) * self.group2(),
        );
    }
}
