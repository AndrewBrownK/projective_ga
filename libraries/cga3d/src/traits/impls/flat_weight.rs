// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 17
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         0       0       0     N/A
//  Average:         0       0       0     N/A
//  Maximum:         0       0       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         0       0       0       0
//  Average:         0       0       0       0
//  Maximum:         0       0       0       0
impl std::ops::Div<FlatWeightPrefixOrPostfix> for AntiCircleRotor {
    type Output = FlatPoint;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for AntiCircleRotor {
    type Output = FlatPoint;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x3::from(0.0).with_w(self[e45]))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for AntiDipoleInversion {
    type Output = Line;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for AntiDipoleInversion {
    type Output = Line;
    fn flat_weight(self) -> Self::Output {
        Line::from_groups(/* e415, e425, e435 */ self.group1().xyz(), /* e235, e315, e125 */ Simd32x3::from(0.0))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<FlatWeightPrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: FlatWeightPrefixOrPostfix) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for AntiScalar {
    type Output = AntiScalar;
    fn flat_weight(self) -> Self::Output {
        self
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for Circle {
    type Output = Line;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for Circle {
    type Output = Line;
    fn flat_weight(self) -> Self::Output {
        Line::from_groups(/* e415, e425, e435 */ self.group1().xyz(), /* e235, e315, e125 */ Simd32x3::from(0.0))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for CircleRotor {
    type Output = Motor;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for CircleRotor {
    type Output = Motor;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group1().xyz().with_w(self[e12345]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        )
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for Dipole {
    type Output = FlatPoint;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for Dipole {
    type Output = FlatPoint;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x3::from(0.0).with_w(self[e45]))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for DipoleInversion {
    type Output = Flector;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for DipoleInversion {
    type Output = Flector;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(self[e45]),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(0.0),
        )
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for DualNum {
    type Output = AntiScalar;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e12345])
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for FlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<FlatWeightPrefixOrPostfix> for FlatPoint {
    fn div_assign(&mut self, _rhs: FlatWeightPrefixOrPostfix) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for FlatPoint {
    type Output = FlatPoint;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x3::from(0.0).with_w(self[e45]))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for Flector {
    type Output = Flector;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<FlatWeightPrefixOrPostfix> for Flector {
    fn div_assign(&mut self, _rhs: FlatWeightPrefixOrPostfix) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for Flector {
    type Output = Flector;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(self[e45]),
            // e4235, e4315, e4125, e3215
            self.group1().xyz().with_w(0.0),
        )
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for Line {
    type Output = Line;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<FlatWeightPrefixOrPostfix> for Line {
    fn div_assign(&mut self, _rhs: FlatWeightPrefixOrPostfix) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for Line {
    type Output = Line;
    fn flat_weight(self) -> Self::Output {
        Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ Simd32x3::from(0.0))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for Motor {
    type Output = Motor;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<FlatWeightPrefixOrPostfix> for Motor {
    fn div_assign(&mut self, _rhs: FlatWeightPrefixOrPostfix) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for Motor {
    type Output = Motor;
    fn flat_weight(self) -> Self::Output {
        Motor::from_groups(/* e415, e425, e435, e12345 */ self.group0(), /* e235, e315, e125, e5 */ Simd32x4::from(0.0))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<FlatWeightPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: FlatWeightPrefixOrPostfix) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for MultiVector {
    type Output = MultiVector;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(self[e45]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            self.group9().xyz().with_w(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for Plane {
    type Output = Plane;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<FlatWeightPrefixOrPostfix> for Plane {
    fn div_assign(&mut self, _rhs: FlatWeightPrefixOrPostfix) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for Plane {
    type Output = Plane;
    fn flat_weight(self) -> Self::Output {
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0().xyz().with_w(0.0))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for Sphere {
    type Output = Plane;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for Sphere {
    type Output = Plane;
    fn flat_weight(self) -> Self::Output {
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0().xyz().with_w(0.0))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for VersorEven {
    type Output = Motor;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for VersorEven {
    type Output = Motor;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group1().xyz().with_w(self[e12345]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        )
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for VersorOdd {
    type Output = Flector;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for VersorOdd {
    type Output = Flector;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(0.0).with_w(self[e45]),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(0.0),
        )
    }
}
