// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 51
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
impl std::ops::Div<FlatWeightPrefixOrPostfix> for AntiCircleRotor {
    type Output = FlatOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for AntiCircleRotor {
    type Output = FlatOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e45])
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = FlatOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for AntiCircleRotorAtInfinity {
    type Output = FlatOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e45])
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for AntiDipoleInversion {
    type Output = LineOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for AntiDipoleInversion {
    type Output = LineOnOrigin;
    fn flat_weight(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1().xyz())
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = LineOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for AntiDipoleInversionAtInfinity {
    type Output = LineOnOrigin;
    fn flat_weight(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz())
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for AntiDipoleInversionOrthogonalOrigin {
    type Output = LineOnOrigin;
    fn flat_weight(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1())
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = FlatOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for AntiMysteryCircleRotor {
    type Output = FlatOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e45])
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = LineOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for AntiMysteryDipoleInversion {
    type Output = LineOnOrigin;
    fn flat_weight(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz())
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
    type Output = LineOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for Circle {
    type Output = LineOnOrigin;
    fn flat_weight(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1().xyz())
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for CircleAligningOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for CircleAligningOrigin {
    type Output = LineOnOrigin;
    fn flat_weight(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1())
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for CircleAtInfinity {
    type Output = LineOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for CircleAtInfinity {
    type Output = LineOnOrigin;
    fn flat_weight(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz())
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for CircleOnOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for CircleOnOrigin {
    type Output = LineOnOrigin;
    fn flat_weight(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1())
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for CircleRotor {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for CircleRotor {
    type Output = MotorOnOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for CircleRotorAligningOrigin {
    type Output = MotorOnOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group1().with_w(self[e12345]))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for CircleRotorAligningOriginAtInfinity {
    type Output = MotorOnOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0().with_w(self[e12345]))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for CircleRotorAtInfinity {
    type Output = MotorOnOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for CircleRotorOnOrigin {
    type Output = MotorOnOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group1().with_w(self[e12345]))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for Dipole {
    type Output = FlatOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for Dipole {
    type Output = FlatOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e45])
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for DipoleAligningOrigin {
    type Output = FlatOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e45])
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for DipoleAtInfinity {
    type Output = FlatOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for DipoleAtInfinity {
    type Output = FlatOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e45])
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for DipoleInversion {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for DipoleInversion {
    type Output = FlectorOnOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for DipoleInversionAligningOrigin {
    type Output = FlectorOnOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for DipoleInversionAtInfinity {
    type Output = FlectorOnOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for DipoleInversionOnOrigin {
    type Output = FlectorOnOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for DipoleOnOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for DipoleOnOrigin {
    type Output = FlatOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e45])
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
impl std::ops::Div<FlatWeightPrefixOrPostfix> for FlatOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<FlatWeightPrefixOrPostfix> for FlatOrigin {
    fn div_assign(&mut self, _rhs: FlatWeightPrefixOrPostfix) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for FlatOrigin {
    type Output = FlatOrigin;
    fn flat_weight(self) -> Self::Output {
        self
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for FlatPoint {
    type Output = FlatOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for FlatPoint {
    type Output = FlatOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e45])
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for Flector {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for Flector {
    type Output = FlectorOnOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for FlectorOnOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<FlatWeightPrefixOrPostfix> for FlectorOnOrigin {
    fn div_assign(&mut self, _rhs: FlatWeightPrefixOrPostfix) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for FlectorOnOrigin {
    type Output = FlectorOnOrigin;
    fn flat_weight(self) -> Self::Output {
        self
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for Line {
    type Output = LineOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for Line {
    type Output = LineOnOrigin;
    fn flat_weight(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0())
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for LineOnOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<FlatWeightPrefixOrPostfix> for LineOnOrigin {
    fn div_assign(&mut self, _rhs: FlatWeightPrefixOrPostfix) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for LineOnOrigin {
    type Output = LineOnOrigin;
    fn flat_weight(self) -> Self::Output {
        self
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for Motor {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for Motor {
    type Output = MotorOnOrigin;
    fn flat_weight(self) -> Self::Output {
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0())
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for MotorOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<FlatWeightPrefixOrPostfix> for MotorOnOrigin {
    fn div_assign(&mut self, _rhs: FlatWeightPrefixOrPostfix) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for MotorOnOrigin {
    type Output = MotorOnOrigin;
    fn flat_weight(self) -> Self::Output {
        self
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
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(self[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            0.0,
        )
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for MysteryCircle {
    type Output = LineOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for MysteryCircle {
    type Output = LineOnOrigin;
    fn flat_weight(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz())
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for MysteryCircleRotor {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for MysteryCircleRotor {
    type Output = MotorOnOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for MysteryDipole {
    type Output = FlatOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for MysteryDipole {
    type Output = FlatOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e45])
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for MysteryDipoleInversion {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for MysteryDipoleInversion {
    type Output = FlectorOnOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for MysteryVersorEven {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for MysteryVersorEven {
    type Output = MotorOnOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for MysteryVersorOdd {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for MysteryVersorOdd {
    type Output = FlectorOnOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for Plane {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for Plane {
    type Output = PlaneOnOrigin;
    fn flat_weight(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz())
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for PlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl std::ops::DivAssign<FlatWeightPrefixOrPostfix> for PlaneOnOrigin {
    fn div_assign(&mut self, _rhs: FlatWeightPrefixOrPostfix) {
        *self = self.flat_weight()
    }
}
impl FlatWeight for PlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn flat_weight(self) -> Self::Output {
        self
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for Sphere {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for Sphere {
    type Output = PlaneOnOrigin;
    fn flat_weight(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz())
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for SphereOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for SphereOnOrigin {
    type Output = PlaneOnOrigin;
    fn flat_weight(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz())
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for VersorEven {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for VersorEven {
    type Output = MotorOnOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for VersorEvenAligningOrigin {
    type Output = MotorOnOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for VersorEvenAtInfinity {
    type Output = MotorOnOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for VersorEvenOnOrigin {
    type Output = MotorOnOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for VersorOdd {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for VersorOdd {
    type Output = FlectorOnOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]))
    }
}
impl std::ops::Div<FlatWeightPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: FlatWeightPrefixOrPostfix) -> Self::Output {
        self.flat_weight()
    }
}
impl FlatWeight for VersorOddAtInfinity {
    type Output = FlectorOnOrigin;
    fn flat_weight(self) -> Self::Output {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]))
    }
}
