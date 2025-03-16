// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 18
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       1       0
//  Maximum:         0       4       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       2       0
//  Average:         0       2       0
//  Maximum:         0       9       0
impl std::ops::Div<CoCarrierPrefixOrPostfix> for AntiCircleRotor {
    type Output = Plane;
    fn div(self, _rhs: CoCarrierPrefixOrPostfix) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for AntiCircleRotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ (self.group0() * Simd32x3::from(-1.0)).with_w(self[e45]))
    }
}
impl std::ops::Div<CoCarrierPrefixOrPostfix> for AntiDipoleInversion {
    type Output = Motor;
    fn div(self, _rhs: CoCarrierPrefixOrPostfix) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for AntiDipoleInversion {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().with_w(self[e4] * -1.0),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(0.0),
        )
    }
}
impl std::ops::Div<CoCarrierPrefixOrPostfix> for AntiScalar {
    type Output = DualNum;
    fn div(self, _rhs: CoCarrierPrefixOrPostfix) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for AntiScalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e12345], 0.0]) * Simd32x2::from([-1.0, 0.0]))
    }
}
impl std::ops::Div<CoCarrierPrefixOrPostfix> for Circle {
    type Output = Line;
    fn div(self, _rhs: CoCarrierPrefixOrPostfix) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for Circle {
    type Output = Line;
    fn co_carrier(self) -> Self::Output {
        Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ self.group1().xyz())
    }
}
impl std::ops::Div<CoCarrierPrefixOrPostfix> for CircleRotor {
    type Output = Motor;
    fn div(self, _rhs: CoCarrierPrefixOrPostfix) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for CircleRotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().with_w(0.0),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(self[e12345] * -1.0),
        )
    }
}
impl std::ops::Div<CoCarrierPrefixOrPostfix> for Dipole {
    type Output = Plane;
    fn div(self, _rhs: CoCarrierPrefixOrPostfix) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for Dipole {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ (self.group0() * Simd32x3::from(-1.0)).with_w(self[e45]))
    }
}
impl std::ops::Div<CoCarrierPrefixOrPostfix> for DipoleInversion {
    type Output = Flector;
    fn div(self, _rhs: CoCarrierPrefixOrPostfix) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for DipoleInversion {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e1234]]),
            // e4235, e4315, e4125, e3215
            (self.group0() * Simd32x3::from(-1.0)).with_w(self[e45]),
        )
    }
}
impl std::ops::Div<CoCarrierPrefixOrPostfix> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: CoCarrierPrefixOrPostfix) -> Self::Output {
        self.co_carrier()
    }
}
impl std::ops::DivAssign<CoCarrierPrefixOrPostfix> for DualNum {
    fn div_assign(&mut self, _rhs: CoCarrierPrefixOrPostfix) {
        *self = self.co_carrier()
    }
}
impl CoCarrier for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e12345], 0.0]) * Simd32x2::from([-1.0, 0.0]))
    }
}
impl std::ops::Div<CoCarrierPrefixOrPostfix> for FlatPoint {
    type Output = AntiDualNum;
    fn div(self, _rhs: CoCarrierPrefixOrPostfix) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for FlatPoint {
    type Output = AntiDualNum;
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([self[e45], 0.0]))
    }
}
impl std::ops::Div<CoCarrierPrefixOrPostfix> for Flector {
    type Output = Flector;
    fn div(self, _rhs: CoCarrierPrefixOrPostfix) -> Self::Output {
        self.co_carrier()
    }
}
impl std::ops::DivAssign<CoCarrierPrefixOrPostfix> for Flector {
    fn div_assign(&mut self, _rhs: CoCarrierPrefixOrPostfix) {
        *self = self.co_carrier()
    }
}
impl CoCarrier for Flector {
    type Output = Flector;
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            self.group1().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e45]),
        )
    }
}
impl std::ops::Div<CoCarrierPrefixOrPostfix> for Line {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: CoCarrierPrefixOrPostfix) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for Line {
    type Output = AntiFlatPoint;
    fn co_carrier(self) -> Self::Output {
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0().with_w(0.0))
    }
}
impl std::ops::Div<CoCarrierPrefixOrPostfix> for Motor {
    type Output = AntiFlector;
    fn div(self, _rhs: CoCarrierPrefixOrPostfix) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for Motor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group0().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e12345] * -1.0),
        )
    }
}
impl std::ops::Div<CoCarrierPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: CoCarrierPrefixOrPostfix) -> Self::Output {
        self.co_carrier()
    }
}
impl std::ops::DivAssign<CoCarrierPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: CoCarrierPrefixOrPostfix) {
        *self = self.co_carrier()
    }
}
impl CoCarrier for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        9        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e4]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0().yx()[0] * -1.0,
            // e15, e25, e35, e45
            (self.group9().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group7().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group6().xyz(),
            // e4235, e4315, e4125, e3215
            (self.group4() * Simd32x3::from(-1.0)).with_w(self[e45]),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<CoCarrierPrefixOrPostfix> for Plane {
    type Output = FlatPoint;
    fn div(self, _rhs: CoCarrierPrefixOrPostfix) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for Plane {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn co_carrier(self) -> Self::Output {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0))
    }
}
impl std::ops::Div<CoCarrierPrefixOrPostfix> for RoundPoint {
    type Output = AntiScalar;
    fn div(self, _rhs: CoCarrierPrefixOrPostfix) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for RoundPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e4] * -1.0)
    }
}
impl std::ops::Div<CoCarrierPrefixOrPostfix> for Sphere {
    type Output = FlatPoint;
    fn div(self, _rhs: CoCarrierPrefixOrPostfix) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for Sphere {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]))
    }
}
impl std::ops::Div<CoCarrierPrefixOrPostfix> for VersorEven {
    type Output = Motor;
    fn div(self, _rhs: CoCarrierPrefixOrPostfix) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for VersorEven {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xyz().with_w(self[e4] * -1.0),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(self[e12345] * -1.0),
        )
    }
}
impl std::ops::Div<CoCarrierPrefixOrPostfix> for VersorOdd {
    type Output = Flector;
    fn div(self, _rhs: CoCarrierPrefixOrPostfix) -> Self::Output {
        self.co_carrier()
    }
}
impl CoCarrier for VersorOdd {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn co_carrier(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            (self.group3().xyz() * Simd32x3::from(-1.0)).with_w(self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45]]),
        )
    }
}
