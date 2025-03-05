// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 11
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:         2       4       0
//  Average:         4       7       0
//  Maximum:        24      36       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       5       0
//   Median:         2      16       0
//  Average:         4      22       0
//  Maximum:        24      78       1
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
        let wedge = Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ Simd32x3::from(0.0));
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
    //      f32        3        1        0
    //    simd3        0        1        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       28        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let sub_type = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
        let other = DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0]));
        let wedge = Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(sub_type[e4] * other[e5]),
            // e4235, e4315, e4125, e3215
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * sub_type.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
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
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       22        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let other = DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0]));
        let wedge = Plane::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
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
    //    simd4        0        5        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       23        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let other = DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0]));
        let wedge = Plane::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
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
        let wedge = Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ Simd32x3::from(0.0));
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
        let sub_type = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
        let wedge = Motor::from_groups(
            // e415, e425, e435, e12345
            sub_type.group0().with_w(sub_type[e1234]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
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
    //      f32       24       20        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0       11        0
    // Totals...
    // yes simd       24       36        0
    //  no simd       24       78        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let sub_type = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            self[e1234],
        );
        let other = DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0]));
        let wedge = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other[e5] * sub_type[e1234]) + (other[e12345] * sub_type[scalar])]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(other[e5]) * sub_type.group1(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * sub_type.group4().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * sub_type.group7().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e1234
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
                - 2.0 * (wedge[e15] * wedge[e41])
                - 2.0 * (wedge[e25] * wedge[e42])
                - 2.0 * (wedge[e35] * wedge[e43])
                - 2.0 * (wedge[e3215] * wedge[e1234]),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(geometric_anti_product[e12345]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product[e12345]) * self.group1(),
            // e5
            geometric_anti_product[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(geometric_anti_product[e12345]) * self.group3(),
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product[e12345]) * self.group4(),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product[e12345]) * self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(geometric_anti_product[e12345]) * self.group6(),
            // e423, e431, e412
            Simd32x3::from(geometric_anti_product[e12345]) * self.group7(),
            // e235, e315, e125
            Simd32x3::from(geometric_anti_product[e12345]) * self.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(geometric_anti_product[e12345]) * self.group9(),
            // e1234
            geometric_anti_product[e12345] * self[e1234],
        );
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
    //      f32        3        1        0
    //    simd4        0        7        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       29        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let sub_type = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
        let other = DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0]));
        let wedge = Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(sub_type[e4] * other[e5]),
            // e4235, e4315, e4125, e3215
            other.group0().xx().with_zw(other[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * sub_type.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
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
        let sub_type = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
        let wedge = Motor::from_groups(
            // e415, e425, e435, e12345
            sub_type.group0().with_w(sub_type[e1234]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
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
