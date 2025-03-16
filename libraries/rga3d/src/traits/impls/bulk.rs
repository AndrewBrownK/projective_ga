// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 9
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0
impl std::ops::Div<BulkPrefixOrPostfix> for DualNum {
    type Output = Scalar;
    fn div(self, _rhs: BulkPrefixOrPostfix) -> Self::Output {
        self.bulk()
    }
}
impl Bulk for DualNum {
    type Output = Scalar;
    fn bulk(self) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar])
    }
}
impl std::ops::Div<BulkPrefixOrPostfix> for Flector {
    type Output = Flector;
    fn div(self, _rhs: BulkPrefixOrPostfix) -> Self::Output {
        self.bulk()
    }
}
impl std::ops::DivAssign<BulkPrefixOrPostfix> for Flector {
    fn div_assign(&mut self, _rhs: BulkPrefixOrPostfix) {
        *self = self.bulk()
    }
}
impl Bulk for Flector {
    type Output = Flector;
    fn bulk(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            self.group0().xyz().with_w(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[e321]),
        )
    }
}
impl std::ops::Div<BulkPrefixOrPostfix> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: BulkPrefixOrPostfix) -> Self::Output {
        self.bulk()
    }
}
impl std::ops::DivAssign<BulkPrefixOrPostfix> for Horizon {
    fn div_assign(&mut self, _rhs: BulkPrefixOrPostfix) {
        *self = self.bulk()
    }
}
impl Bulk for Horizon {
    type Output = Horizon;
    fn bulk(self) -> Self::Output {
        self
    }
}
impl std::ops::Div<BulkPrefixOrPostfix> for Line {
    type Output = Line;
    fn div(self, _rhs: BulkPrefixOrPostfix) -> Self::Output {
        self.bulk()
    }
}
impl std::ops::DivAssign<BulkPrefixOrPostfix> for Line {
    fn div_assign(&mut self, _rhs: BulkPrefixOrPostfix) {
        *self = self.bulk()
    }
}
impl Bulk for Line {
    type Output = Line;
    fn bulk(self) -> Self::Output {
        Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(0.0), /* e23, e31, e12 */ self.group1())
    }
}
impl std::ops::Div<BulkPrefixOrPostfix> for Motor {
    type Output = Motor;
    fn div(self, _rhs: BulkPrefixOrPostfix) -> Self::Output {
        self.bulk()
    }
}
impl std::ops::DivAssign<BulkPrefixOrPostfix> for Motor {
    fn div_assign(&mut self, _rhs: BulkPrefixOrPostfix) {
        *self = self.bulk()
    }
}
impl Bulk for Motor {
    type Output = Motor;
    fn bulk(self) -> Self::Output {
        Motor::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from(0.0), /* e23, e31, e12, scalar */ self.group1())
    }
}
impl std::ops::Div<BulkPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: BulkPrefixOrPostfix) -> Self::Output {
        self.bulk()
    }
}
impl std::ops::DivAssign<BulkPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: BulkPrefixOrPostfix) {
        *self = self.bulk()
    }
}
impl Bulk for MultiVector {
    type Output = MultiVector;
    fn bulk(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[e321]),
        )
    }
}
impl std::ops::Div<BulkPrefixOrPostfix> for Plane {
    type Output = Horizon;
    fn div(self, _rhs: BulkPrefixOrPostfix) -> Self::Output {
        self.bulk()
    }
}
impl Bulk for Plane {
    type Output = Horizon;
    fn bulk(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e321])
    }
}
impl std::ops::Div<BulkPrefixOrPostfix> for Point {
    type Output = Point;
    fn div(self, _rhs: BulkPrefixOrPostfix) -> Self::Output {
        self.bulk()
    }
}
impl std::ops::DivAssign<BulkPrefixOrPostfix> for Point {
    fn div_assign(&mut self, _rhs: BulkPrefixOrPostfix) {
        *self = self.bulk()
    }
}
impl Bulk for Point {
    type Output = Point;
    fn bulk(self) -> Self::Output {
        Point::from_groups(/* e1, e2, e3, e4 */ self.group0().xyz().with_w(0.0))
    }
}
impl std::ops::Div<BulkPrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: BulkPrefixOrPostfix) -> Self::Output {
        self.bulk()
    }
}
impl std::ops::DivAssign<BulkPrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: BulkPrefixOrPostfix) {
        *self = self.bulk()
    }
}
impl Bulk for Scalar {
    type Output = Scalar;
    fn bulk(self) -> Self::Output {
        self
    }
}
