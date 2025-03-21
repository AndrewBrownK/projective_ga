// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 18
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
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiCircleRotor {
    type Output = Motor;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiCircleRotor {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().with_w(0.0),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(self[scalar]),
        )
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiDipoleInversion {
    type Output = Flector;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiDipoleInversion {
    type Output = Flector;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e4235, e4315, e4125, e3215
            self.group0().with_w(self[e321]),
        )
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiDualNum {
    type Output = DualNum;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiDualNum {
    type Output = DualNum;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[scalar], 0.0]))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiFlatPoint {
    type Output = AntiDualNum;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiFlatPoint {
    type Output = AntiDualNum;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([self[e321], 0.0]))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiFlector {
    type Output = Flector;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiFlector {
    type Output = Flector;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            self.group1().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e321]),
        )
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiLine {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiLine {
    type Output = AntiFlatPoint;
    fn carrier(self) -> Self::Output {
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0().with_w(0.0))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiMotor {
    type Output = AntiFlector;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiMotor {
    type Output = AntiFlector;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group0().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[scalar]),
        )
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiPlane {
    type Output = FlatPoint;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiPlane {
    type Output = FlatPoint;
    fn carrier(self) -> Self::Output {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0().xyz().with_w(0.0))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for Circle {
    type Output = Plane;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for Circle {
    type Output = Plane;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0().with_w(self[e321]))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for CircleRotor {
    type Output = Plane;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for CircleRotor {
    type Output = Plane;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0().with_w(self[e321]))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for Dipole {
    type Output = Line;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for Dipole {
    type Output = Line;
    fn carrier(self) -> Self::Output {
        Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ self.group1().xyz())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for DipoleInversion {
    type Output = Motor;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DipoleInversion {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().with_w(self[e1234]),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(0.0),
        )
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl std::ops::DivAssign<CarrierPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: CarrierPrefixOrPostfix) {
        *self = self.carrier()
    }
}
impl Carrier for MultiVector {
    type Output = MultiVector;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[scalar],
            // e15, e25, e35, e45
            self.group1(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group4().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group5(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for RoundPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for RoundPoint {
    type Output = FlatPoint;
    fn carrier(self) -> Self::Output {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for Scalar {
    type Output = DualNum;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for Scalar {
    type Output = DualNum;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[scalar], 0.0]))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for Sphere {
    type Output = AntiScalar;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for Sphere {
    type Output = AntiScalar;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e1234])
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for VersorEven {
    type Output = Flector;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for VersorEven {
    type Output = Flector;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            self.group3(),
            // e4235, e4315, e4125, e3215
            self.group0().xyz().with_w(self[e321]),
        )
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for VersorOdd {
    type Output = Motor;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for VersorOdd {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xyz().with_w(self[e1234]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
        )
    }
}
