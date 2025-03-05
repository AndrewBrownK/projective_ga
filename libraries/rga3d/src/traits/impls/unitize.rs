// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 9
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       1       0
//  Average:         1       1       0
//  Maximum:         7       5       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       4       0
//  Average:         1       5       0
//  Maximum:         7      16       1
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
        return AntiScalar::from_groups(/* e1234 */ 1.0);
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
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(1.0 / self[e1234]) * self.group0());
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
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        8        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let sub_type = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e423, e431, e412, e321
            self.group1().xyz().with_w(0.0),
        );
        let geometric_anti_product = AntiScalar::from_groups(
            // e1234
            f32::powi(sub_type[e4], 2) + f32::powi(sub_type[e423], 2) + f32::powi(sub_type[e431], 2) + f32::powi(sub_type[e412], 2),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product[e1234]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_anti_product[e1234]) * self.group1(),
        );
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
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        2        6        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let sub_type = Line::from_groups(/* e41, e42, e43 */ self.group0(), /* e23, e31, e12 */ Simd32x3::from(0.0));
        let geometric_anti_product = AntiScalar::from_groups(/* e1234 */ f32::powi(sub_type[e41], 2) + f32::powi(sub_type[e42], 2) + f32::powi(sub_type[e43], 2));
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product[e1234]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product[e1234]) * self.group1(),
        );
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
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        8        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let sub_type = Motor::from_groups(/* e41, e42, e43, e1234 */ self.group0(), /* e23, e31, e12, scalar */ Simd32x4::from(0.0));
        let geometric_anti_product = AntiScalar::from_groups(
            // e1234
            f32::powi(sub_type[e41], 2) + f32::powi(sub_type[e42], 2) + f32::powi(sub_type[e43], 2) + f32::powi(sub_type[e1234], 2),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(geometric_anti_product[e1234]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(geometric_anti_product[e1234]) * self.group1(),
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
    //      f32        7        0        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        5        0
    //  no simd        7       16        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let sub_type = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, self[e1234]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            self.group4().xyz().with_w(0.0),
        );
        let geometric_anti_product = AntiScalar::from_groups(
            // e1234
            f32::powi(sub_type[e1234], 2)
                + f32::powi(sub_type[e4], 2)
                + f32::powi(sub_type[e41], 2)
                + f32::powi(sub_type[e42], 2)
                + f32::powi(sub_type[e43], 2)
                + f32::powi(sub_type[e423], 2)
                + f32::powi(sub_type[e431], 2)
                + f32::powi(sub_type[e412], 2),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(geometric_anti_product[e1234]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product[e1234]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product[e1234]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product[e1234]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_anti_product[e1234]) * self.group4(),
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
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let sub_type = Plane::from_groups(/* e423, e431, e412, e321 */ self.group0().xyz().with_w(0.0));
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(f32::powi(sub_type[e423], 2) + f32::powi(sub_type[e431], 2) + f32::powi(sub_type[e412], 2)) * self.group0(),
        );
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
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn unitize(self) -> Self {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(1.0 / self[e4]) * self.group0());
    }
}
