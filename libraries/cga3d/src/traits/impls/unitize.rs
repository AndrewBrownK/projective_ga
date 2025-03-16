// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 11
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:         2       3       0
//  Average:         2       3       0
//  Maximum:         7      11       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       5       0
//   Median:         2      11       0
//  Average:         2      13       0
//  Maximum:         7      32       1
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
        let geometric_anti_product_g0 = self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43];
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product_g0) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(geometric_anti_product_g0) * self.group2(),
        )
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
        let geometric_anti_product_g0 = self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4];
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(geometric_anti_product_g0) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(geometric_anti_product_g0) * self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(geometric_anti_product_g0) * self.group3(),
        )
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
        let geometric_anti_product_g0 = self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412];
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(geometric_anti_product_g0) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
            // e235, e315, e125
            Simd32x3::from(geometric_anti_product_g0) * self.group2(),
        )
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
        let geometric_anti_product_g0 = self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412];
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(geometric_anti_product_g0) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(geometric_anti_product_g0) * self.group2(),
        )
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
        let geometric_anti_product_g0 = self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43];
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product_g0) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
            // e15, e25, e35
            Simd32x3::from(geometric_anti_product_g0) * self.group2(),
        )
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
        let geometric_anti_product_g0 = self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234];
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product_g0) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(geometric_anti_product_g0) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(geometric_anti_product_g0) * self.group3(),
        )
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
    //      f32        7        2        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       11        0
    //  no simd        7       32        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e4] * self[e4]
            + self[e41] * self[e41]
            + self[e42] * self[e42]
            + self[e43] * self[e43]
            + self[e423] * self[e423]
            + self[e431] * self[e431]
            + self[e412] * self[e412]
            + self[e1234] * self[e1234];
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(geometric_anti_product_g0) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
            // e5
            geometric_anti_product_g0 * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(geometric_anti_product_g0) * self.group3(),
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product_g0) * self.group4(),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product_g0) * self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(geometric_anti_product_g0) * self.group6(),
            // e423, e431, e412
            Simd32x3::from(geometric_anti_product_g0) * self.group7(),
            // e235, e315, e125
            Simd32x3::from(geometric_anti_product_g0) * self.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(geometric_anti_product_g0) * self.group9(),
            // e1234
            geometric_anti_product_g0 * self[e1234],
        )
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
        let geometric_anti_product_g0 = 1.0 / self[e4];
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e5
            geometric_anti_product_g0 * self[e5],
        )
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
        let geometric_anti_product_g0 = 1.0 / self[e1234];
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e1234
            geometric_anti_product_g0 * self[e1234],
        )
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
        let geometric_anti_product_g0 = self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4];
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(geometric_anti_product_g0) * self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product_g0) * self.group3(),
        )
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
        let geometric_anti_product_g0 = self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234];
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(geometric_anti_product_g0) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(geometric_anti_product_g0) * self.group3(),
        )
    }
}
