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
//  Maximum:         8      27       2       0
impl std::ops::Div<InversePrefixOrPostfix> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for DualNum {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        2        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        2        2      N/A
    //  no simd        0        3        2        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(1.0 / self[scalar]) * Simd32x2::from([1.0, self[e1234] / self[scalar]]))
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Flector {
    type Output = Flector;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Flector {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       13        0        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] + self[e321] * self[e321];
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other_g0) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other_g0 * -1.0) * self.group1(),
        )
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Horizon {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ -1.0 / self[e321])
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Line {
    type Output = Line;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Line {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        2       11        0        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12];
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other_g0 * -1.0) * self.group1(),
        )
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Motor {
    type Output = Motor;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Motor {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       20        0        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar];
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       11        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        7       16        0      N/A
    //  no simd        7       27        0        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[scalar] * self[scalar]
            + self[e1] * self[e1]
            + self[e2] * self[e2]
            + self[e3] * self[e3]
            + self[e23] * self[e23]
            + self[e31] * self[e31]
            + self[e12] * self[e12]
            + self[e321] * self[e321];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other_g0) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other_g0) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(other_g0 * -1.0) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(other_g0 * -1.0) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other_g0 * -1.0) * self.group4(),
        )
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        2        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        2      N/A
    //  no simd        0        8        2        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(-1.0 / self[e321]) * (Simd32x3::from(1.0 / self[e321]) * self.group0().xyz()).with_w(1.0),
        )
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Point {
    type Output = Point;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Point {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Point {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       11        0        0
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2       14        0      N/A
    //  no simd        8       23        0        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        Point::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([self[e1] * self[e1], self[e2] * self[e2], self[e3] * self[e3], self[e1] * self[e1]]) * self.group0())
                + (Simd32x4::from([self[e2] * self[e2], self[e1] * self[e1], self[e1] * self[e1], self[e2] * self[e2]]) * self.group0())
                + (self.group0() * Simd32x2::from(self[e3] * self[e3]).with_zw(self[e2] * self[e2], self[e3] * self[e3])),
        )
    }
}
impl std::ops::Div<InversePrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: InversePrefixOrPostfix) -> Self::Output {
        self.inverse()
    }
}
impl std::ops::DivAssign<InversePrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: InversePrefixOrPostfix) {
        *self = self.inverse()
    }
}
impl Inverse for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        0        1        0
    fn inverse(self) -> Self {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ 1.0 / self[scalar])
    }
}
