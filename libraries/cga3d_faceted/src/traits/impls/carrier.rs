// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 77
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       2       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       6       0
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = Line;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiCircleOnOrigin {
    type Output = Line;
    fn carrier(self) -> Self::Output {
        Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ self.group1())
    }
}
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
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = Motor;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiCircleRotorAligningOrigin {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().with_w(0.0),
            // e235, e315, e125, e5
            self.group1().with_w(self[scalar]),
        )
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiCircleRotorAligningOriginAtInfinity {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0().with_w(self[scalar]))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiCircleRotorAtInfinity {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0().xyz().with_w(self[scalar]))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = Motor;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiCircleRotorOnOrigin {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            self.group1().with_w(self[scalar]),
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
            self.group3().xyz().with_w(self[e4]),
            // e4235, e4315, e4125, e3215
            self.group0().with_w(self[e321]),
        )
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiDipoleInversionAtInfinity {
    type Output = FlectorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group2().xyz().with_w(self[e321]))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = Flector;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiDipoleInversionOnOrigin {
    type Output = Flector;
    fn carrier(self) -> Self::Output {
        Flector::from_groups(/* e15, e25, e35, e45 */ self.group1().yzwx(), /* e4235, e4315, e4125, e3215 */ self.group0())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiDipoleInversionOrthogonalOrigin {
    type Output = FlectorOnOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = Plane;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiDipoleOnOrigin {
    type Output = Plane;
    fn carrier(self) -> Self::Output {
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiDualNum {
    type Output = Motor;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiDualNum {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).with_w(self[e1234]),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(self[scalar]),
        )
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiFlatOrigin {
    type Output = Horizon;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiFlatOrigin {
    type Output = Horizon;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e321])
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiFlatPoint {
    type Output = Horizon;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiFlatPoint {
    type Output = Horizon;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e321])
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiFlector {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiFlector {
    type Output = FlectorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group1().xyz().with_w(self[e321]))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiFlectorOnOrigin {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiFlectorOnOrigin {
    type Output = FlectorAtInfinity;
    fn carrier(self) -> Self::Output {
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0().yzwx())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiLine {
    type Output = LineAtInfinity;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiLine {
    type Output = LineAtInfinity;
    fn carrier(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiLineOnOrigin {
    type Output = LineAtInfinity;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiLineOnOrigin {
    type Output = LineAtInfinity;
    fn carrier(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiMotor {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiMotor {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiMotorOnOrigin {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiMotorOnOrigin {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiMysteryCircleRotor {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0().xyz().with_w(self[scalar]))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiMysteryDipoleInversion {
    type Output = FlectorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group1().with_w(self[e321]))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiPlane {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiPlane {
    type Output = FlatPointAtInfinity;
    fn carrier(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0().xyz())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiPlaneOnOrigin {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiPlaneOnOrigin {
    type Output = FlatPointAtInfinity;
    fn carrier(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = FlatPoint;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiSphereOnOrigin {
    type Output = FlatPoint;
    fn carrier(self) -> Self::Output {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = Motor;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for AntiVersorEvenOnOrigin {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xyz().with_w(self[e1234]),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(self[scalar]),
        )
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
impl std::ops::Div<CarrierPrefixOrPostfix> for CircleAligningOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for CircleAligningOrigin {
    type Output = PlaneOnOrigin;
    fn carrier(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for CircleAtInfinity {
    type Output = Horizon;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for CircleAtInfinity {
    type Output = Horizon;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e321])
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for CircleAtOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for CircleAtOrigin {
    type Output = PlaneOnOrigin;
    fn carrier(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for CircleOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for CircleOnOrigin {
    type Output = PlaneOnOrigin;
    fn carrier(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = Plane;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for CircleOrthogonalOrigin {
    type Output = Plane;
    fn carrier(self) -> Self::Output {
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0())
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
impl std::ops::Div<CarrierPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for CircleRotorAligningOrigin {
    type Output = PlaneOnOrigin;
    fn carrier(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = Horizon;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for CircleRotorAtInfinity {
    type Output = Horizon;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e321])
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for CircleRotorOnOrigin {
    type Output = PlaneOnOrigin;
    fn carrier(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz())
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
impl std::ops::Div<CarrierPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DipoleAligningOrigin {
    type Output = LineOnOrigin;
    fn carrier(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for DipoleAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DipoleAtInfinity {
    type Output = LineAtInfinity;
    fn carrier(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0().xyz())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for DipoleAtOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DipoleAtOrigin {
    type Output = LineOnOrigin;
    fn carrier(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for DipoleInversion {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DipoleInversion {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        CircleRotorAligningOriginAtInfinity::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125, e12345 */ self.group1().xyz().with_w(self[e1234]))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DipoleInversionAligningOrigin {
    type Output = MotorOnOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0().xyz().with_w(self[e1234]))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DipoleInversionAtInfinity {
    type Output = LineAtInfinity;
    fn carrier(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0().xyz())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DipoleInversionAtOrigin {
    type Output = MotorOnOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0().xyz().with_w(self[e1234]))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DipoleInversionOnOrigin {
    type Output = MotorOnOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0().xyz().with_w(self[e1234]))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DipoleInversionOrthogonalOrigin {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        CircleRotorAligningOriginAtInfinity::from_groups(/* e415, e425, e435 */ self.group0().xyz(), /* e235, e315, e125, e12345 */ self.group1().with_w(self[e1234]))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for DipoleOnOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DipoleOnOrigin {
    type Output = LineOnOrigin;
    fn carrier(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = Line;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DipoleOrthogonalOrigin {
    type Output = Line;
    fn carrier(self) -> Self::Output {
        Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ self.group1())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for DualNum {
    type Output = FlatOrigin;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for DualNum {
    type Output = FlatOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e4])
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
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        6        0
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, self[e1234]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[scalar],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(self[e4]),
            // e15, e25, e35
            self.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group3().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group5(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e423], self[e431], self[e412]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]),
            // e3215
            self[e321],
        )
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for MysteryCircle {
    type Output = Horizon;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for MysteryCircle {
    type Output = Horizon;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e321])
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for MysteryCircleRotor {
    type Output = Horizon;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for MysteryCircleRotor {
    type Output = Horizon;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e321])
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for MysteryDipole {
    type Output = LineAtInfinity;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for MysteryDipole {
    type Output = LineAtInfinity;
    fn carrier(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0().xyz())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for MysteryDipoleInversion {
    type Output = LineAtInfinity;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for MysteryDipoleInversion {
    type Output = LineAtInfinity;
    fn carrier(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0().xyz())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for MysteryVersorEven {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for MysteryVersorEven {
    type Output = FlectorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0().yzw().with_w(self[e321]))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for MysteryVersorOdd {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for MysteryVersorOdd {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group1().xyz().with_w(self[scalar]))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for NullCircleAtOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for NullCircleAtOrigin {
    type Output = PlaneOnOrigin;
    fn carrier(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for NullDipoleAtOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for NullDipoleAtOrigin {
    type Output = LineOnOrigin;
    fn carrier(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for NullDipoleInversionAtOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for NullDipoleInversionAtOrigin {
    type Output = MotorOnOrigin;
    fn carrier(self) -> Self::Output {
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for NullSphereAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for NullSphereAtOrigin {
    type Output = AntiScalar;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e1234])
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for NullVersorEvenAtOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for NullVersorEvenAtOrigin {
    type Output = FlectorOnOrigin;
    fn carrier(self) -> Self::Output {
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0().wxyz())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for Origin {
    type Output = FlatOrigin;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for Origin {
    type Output = FlatOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e4])
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
impl std::ops::Div<CarrierPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for RoundPointAtOrigin {
    type Output = FlatOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e4])
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for Scalar {
    type Output = Infinity;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for Scalar {
    type Output = Infinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[scalar])
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
impl std::ops::Div<CarrierPrefixOrPostfix> for SphereAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for SphereAtOrigin {
    type Output = AntiScalar;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e1234])
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for SphereOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for SphereOnOrigin {
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
impl std::ops::Div<CarrierPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for VersorEvenAligningOrigin {
    type Output = FlectorOnOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for VersorEvenAtInfinity {
    type Output = FlectorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0().yzw().with_w(self[e321]))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for VersorEvenAtOrigin {
    type Output = FlectorOnOrigin;
    fn carrier(self) -> Self::Output {
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0().wxyz())
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for VersorEvenOnOrigin {
    type Output = FlectorOnOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = Flector;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for VersorEvenOrthogonalOrigin {
    type Output = Flector;
    fn carrier(self) -> Self::Output {
        Flector::from_groups(/* e15, e25, e35, e45 */ self.group2(), /* e4235, e4315, e4125, e3215 */ self.group0())
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
            self.group1().xyz().with_w(self[scalar]),
        )
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for VersorOddAtInfinity {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group1().xyz().with_w(self[scalar]))
    }
}
impl std::ops::Div<CarrierPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = Motor;
    fn div(self, _rhs: CarrierPrefixOrPostfix) -> Self::Output {
        self.carrier()
    }
}
impl Carrier for VersorOddOrthogonalOrigin {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xyz().with_w(self[e1234]),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(self[scalar]),
        )
    }
}
