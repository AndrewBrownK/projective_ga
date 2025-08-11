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
//   Median:         2       7       0     N/A
//  Average:         1       6       0     N/A
//  Maximum:         7      16       2     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         2      11       0       0
//  Average:         2      11       0       0
//  Maximum:         8      27       4       0
impl std::ops::Div<AntiInversePrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        0        1        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ 1.0 / self[e1234])
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for DualNum {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        2        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([self[scalar] / (self[e1234] * self[e1234]), 1.0 / self[e1234]]))
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for Flector {
    type Output = Flector;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for Flector {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       13        0        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e4] * self[e4] + self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412];
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other_g0 * -1.0) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other_g0) * self.group1(),
        )
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for Line {
    type Output = Line;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for Line {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        2       11        0        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43];
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other_g0 * -1.0) * self.group1(),
        )
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for Motor {
    type Output = Motor;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for Motor {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       20        0        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234];
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       11        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        7       16        0      N/A
    //  no simd        7       27        0        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e1234] * self[e1234]
            + self[e41] * self[e41]
            + self[e42] * self[e42]
            + self[e43] * self[e43]
            + self[e4] * self[e4]
            + self[e423] * self[e423]
            + self[e431] * self[e431]
            + self[e412] * self[e412];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other_g0) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other_g0 * -1.0) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(other_g0 * -1.0) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(other_g0 * -1.0) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other_g0) * self.group4(),
        )
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for Origin {
    type Output = Origin;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for Origin {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ -1.0 / self[e4])
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        7        0        0
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd        2       11        0      N/A
    //  no simd        8       23        0        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x4::from([self[e431] * self[e431], self[e423] * self[e423], self[e423] * self[e423], self[e431] * self[e431]]) * self.group0())
                + (self.group0() * Simd32x2::from(self[e412] * self[e412]).with_zw(self[e431] * self[e431], self[e412] * self[e412]))
                + (Simd32x4::powi(self.group0().xyzx(), 2) * self.group0()),
        )
    }
}
impl std::ops::Div<AntiInversePrefixOrPostfix> for Point {
    type Output = Point;
    fn div(self, _rhs: AntiInversePrefixOrPostfix) -> Self::Output {
        self.anti_inverse()
    }
}
impl std::ops::DivAssign<AntiInversePrefixOrPostfix> for Point {
    fn div_assign(&mut self, _rhs: AntiInversePrefixOrPostfix) {
        *self = self.anti_inverse()
    }
}
impl AntiInverse for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        1        0
    //    simd3        0        3        1      N/A
    // Totals...
    // yes simd        0        4        2      N/A
    //  no simd        0       10        4        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        Point::from_groups(
            // e1, e2, e3, e4
            (self.group0().xyz() * Simd32x3::from(-1.0) / (Simd32x4::from(self[e4]).xyz() * Simd32x4::from(self[e4]).xyz())).with_w(-1.0 / self[e4]),
        )
    }
}
