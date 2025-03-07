// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 23
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
impl std::ops::Div<FlatBulkPrefixOrPostfix> for AntiCircleRotor {
    type Output = FlatPoint;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for AntiCircleRotor {
    type Output = FlatPoint;
    fn flat_bulk(self) -> Self::Output {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group2().xyz().with_w(0.0))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for AntiDipoleInversion {
    type Output = AntiFlector;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for AntiDipoleInversion {
    type Output = AntiFlector;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group2().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e5]),
        )
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<FlatBulkPrefixOrPostfix> for AntiDualNum {
    fn div_assign(&mut self, _rhs: FlatBulkPrefixOrPostfix) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for AntiDualNum {
    type Output = AntiDualNum;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([self[e3215], 0.0]))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<FlatBulkPrefixOrPostfix> for AntiFlatPoint {
    fn div_assign(&mut self, _rhs: FlatBulkPrefixOrPostfix) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for AntiFlatPoint {
    type Output = AntiFlatPoint;
    fn flat_bulk(self) -> Self::Output {
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0().xyz().with_w(0.0))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for AntiFlector {
    type Output = AntiFlector;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<FlatBulkPrefixOrPostfix> for AntiFlector {
    fn div_assign(&mut self, _rhs: FlatBulkPrefixOrPostfix) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for AntiFlector {
    type Output = AntiFlector;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group0().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e5]),
        )
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for AntiLine {
    type Output = FlatPoint;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for AntiLine {
    type Output = FlatPoint;
    fn flat_bulk(self) -> Self::Output {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group1().with_w(0.0))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for AntiMotor {
    type Output = Flector;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for AntiMotor {
    type Output = Flector;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            self.group1().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        )
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for AntiPlane {
    type Output = DualNum;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for AntiPlane {
    type Output = DualNum;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e5], 0.0]))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for Circle {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for Circle {
    type Output = AntiFlatPoint;
    fn flat_bulk(self) -> Self::Output {
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group2().with_w(0.0))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for CircleRotor {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for CircleRotor {
    type Output = AntiFlatPoint;
    fn flat_bulk(self) -> Self::Output {
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group2().xyz().with_w(0.0))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for Dipole {
    type Output = FlatPoint;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for Dipole {
    type Output = FlatPoint;
    fn flat_bulk(self) -> Self::Output {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group2().with_w(0.0))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for DipoleInversion {
    type Output = Flector;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for DipoleInversion {
    type Output = Flector;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            self.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        )
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<FlatBulkPrefixOrPostfix> for DualNum {
    fn div_assign(&mut self, _rhs: FlatBulkPrefixOrPostfix) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for DualNum {
    type Output = DualNum;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e5], 0.0]))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<FlatBulkPrefixOrPostfix> for FlatPoint {
    fn div_assign(&mut self, _rhs: FlatBulkPrefixOrPostfix) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for FlatPoint {
    type Output = FlatPoint;
    fn flat_bulk(self) -> Self::Output {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0().xyz().with_w(0.0))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for Flector {
    type Output = Flector;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<FlatBulkPrefixOrPostfix> for Flector {
    fn div_assign(&mut self, _rhs: FlatBulkPrefixOrPostfix) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for Flector {
    type Output = Flector;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            self.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        )
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for Line {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for Line {
    type Output = AntiFlatPoint;
    fn flat_bulk(self) -> Self::Output {
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group1().with_w(0.0))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for Motor {
    type Output = AntiFlector;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for Motor {
    type Output = AntiFlector;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group1().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e5]),
        )
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<FlatBulkPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: FlatBulkPrefixOrPostfix) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for MultiVector {
    type Output = MultiVector;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3().xyz().with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for Plane {
    type Output = AntiDualNum;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for Plane {
    type Output = AntiDualNum;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([self[e3215], 0.0]))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for RoundPoint {
    type Output = DualNum;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for RoundPoint {
    type Output = DualNum;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e5], 0.0]))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for Sphere {
    type Output = AntiDualNum;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for Sphere {
    type Output = AntiDualNum;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([self[e3215], 0.0]))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for VersorEven {
    type Output = AntiFlector;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for VersorEven {
    type Output = AntiFlector;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group2().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e5]),
        )
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for VersorOdd {
    type Output = Flector;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for VersorOdd {
    type Output = Flector;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            self.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215]),
        )
    }
}
