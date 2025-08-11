// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 9
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         2       5       0     N/A
//  Average:         1       4       0     N/A
//  Maximum:         7      13       1     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         2       9       0       0
//  Average:         2       8       0       0
//  Maximum:         8      24       3       0
impl std::ops::Div<UnitizePrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for AntiScalar {
    fn unitize(self) -> Self {
        AntiScalar::from_groups(/* e1234 */ 1.0)
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
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([self[scalar] / self[e1234], 1.0]))
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for Flector {
    type Output = Flector;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for Flector {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3       12        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let sub_type_g1_xyz = self.group1().xyz();
        let geometric_anti_product_g0 =
            sub_type_g1_xyz[0] * sub_type_g1_xyz[0] + sub_type_g1_xyz[1] * sub_type_g1_xyz[1] + sub_type_g1_xyz[2] * sub_type_g1_xyz[2] + self[e4] * self[e4];
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
        )
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for Line {
    type Output = Line;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for Line {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        9        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43];
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product_g0) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product_g0) * self.group1(),
        )
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for Motor {
    type Output = Motor;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for Motor {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3       12        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234];
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
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
    //           add/sub      mul      div      pow
    //      f32        7        8        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        7       13        0      N/A
    //  no simd        7       24        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let sub_type_g4_xyz = self.group4().xyz();
        let geometric_anti_product_g0 = self[e1234] * self[e1234]
            + sub_type_g4_xyz[0] * sub_type_g4_xyz[0]
            + sub_type_g4_xyz[1] * sub_type_g4_xyz[1]
            + sub_type_g4_xyz[2] * sub_type_g4_xyz[2]
            + self[e41] * self[e41]
            + self[e42] * self[e42]
            + self[e43] * self[e43]
            + self[e4] * self[e4];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(geometric_anti_product_g0) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product_g0) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product_g0) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_anti_product_g0) * self.group4(),
        )
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
        Origin::from_groups(/* e4 */ 1.0)
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        8       15        0        0
    fn unitize(self) -> Self {
        let sub_type_g0_xyz = self.group0().xyz();
        Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x4::from(sub_type_g0_xyz[0] * sub_type_g0_xyz[0]) * self.group0())
                + (Simd32x4::from(sub_type_g0_xyz[1] * sub_type_g0_xyz[1]) * self.group0())
                + (Simd32x4::from(sub_type_g0_xyz[2] * sub_type_g0_xyz[2]) * self.group0()),
        )
    }
}
impl std::ops::Div<UnitizePrefixOrPostfix> for Point {
    type Output = Point;
    fn div(self, _rhs: UnitizePrefixOrPostfix) -> Self::Output {
        self.unitize()
    }
}
impl std::ops::DivAssign<UnitizePrefixOrPostfix> for Point {
    fn div_assign(&mut self, _rhs: UnitizePrefixOrPostfix) {
        *self = self.unitize()
    }
}
impl Unitize for Point {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        1      N/A
    // no simd        0        3        3        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ (self.group0().xyz() / Simd32x4::from(self[e4]).xyz()).with_w(1.0))
    }
}
