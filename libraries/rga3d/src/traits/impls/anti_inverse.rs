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
//   Median:         2       3       0
//  Average:         1       3       0
//  Maximum:         7       8       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       8       0
//  Average:         1       9       0
//  Maximum:         7      26       1
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
    //      add/sub      mul      div
    // f32        0        0        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ 1.0 / self[e1234]);
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
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(f32::powi(self[e1234], -2)) * self.group0());
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
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3       12        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e4] * self[e4] + self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412];
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from(other_g0) * self.group1(),
        );
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
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2       12        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43];
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_g0) * self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(other_g0) * self.group1() * Simd32x3::from(-1.0),
        );
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
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       16        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234];
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
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
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        7        8        0
    //  no simd        7       26        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e1234] * self[e1234]
            + self[e4] * self[e4]
            + self[e41] * self[e41]
            + self[e42] * self[e42]
            + self[e43] * self[e43]
            + self[e423] * self[e423]
            + self[e431] * self[e431]
            + self[e412] * self[e412];
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other_g0) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from(-1.0),
            // e41, e42, e43
            Simd32x3::from(other_g0) * self.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(other_g0) * self.group3() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from(other_g0) * self.group4(),
        );
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
    //      add/sub      mul      div
    // f32        0        1        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ 1.0 / self[e4] * -1.0);
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
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412]) * self.group0(),
        );
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
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(f32::powi(self[e4], -2)) * Simd32x4::from([self[e1] * -1.0, self[e2] * -1.0, self[e3] * -1.0, self[e4] * -1.0]),
        );
    }
}
