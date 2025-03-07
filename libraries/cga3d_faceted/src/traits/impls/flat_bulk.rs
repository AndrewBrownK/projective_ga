// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 55
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
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for AntiCircleRotor {
    type Output = FlatPointAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group2().xyz())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for AntiCircleRotorAligningOrigin {
    type Output = FlatPointAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group2().xyz())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for AntiCircleRotorAligningOriginAtInfinity {
    type Output = FlatPointAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group1().xyz())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for AntiCircleRotorAtInfinity {
    type Output = FlatPointAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group1().xyz())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for AntiDipoleInversion {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for AntiDipoleInversion {
    type Output = MotorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for AntiDipoleInversionAtInfinity {
    type Output = MotorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group1().with_w(self[e5]))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for AntiDipoleInversionOrthogonalOrigin {
    type Output = MotorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for AntiFlatPoint {
    type Output = LineAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for AntiFlatPoint {
    type Output = LineAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0().xyz())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for AntiFlector {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for AntiFlector {
    type Output = MotorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for AntiLine {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for AntiLine {
    type Output = FlatPointAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group1())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for AntiMotor {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for AntiMotor {
    type Output = FlectorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group1())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for AntiPlane {
    type Output = Infinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for AntiPlane {
    type Output = Infinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e5])
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for Circle {
    type Output = LineAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for Circle {
    type Output = LineAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group2())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for CircleAligningOrigin {
    type Output = LineAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for CircleAligningOrigin {
    type Output = LineAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group2())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for CircleAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for CircleAtInfinity {
    type Output = LineAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group1())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for CircleAtOrigin {
    type Output = LineAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for CircleAtOrigin {
    type Output = LineAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group1())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = LineAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for CircleOrthogonalOrigin {
    type Output = LineAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group1())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for CircleRotor {
    type Output = LineAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for CircleRotor {
    type Output = LineAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group2().xyz())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = LineAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for CircleRotorAligningOrigin {
    type Output = LineAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group2().xyz())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for CircleRotorAligningOriginAtInfinity {
    type Output = LineAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group1().xyz())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for CircleRotorAtInfinity {
    type Output = LineAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group1().xyz())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for Dipole {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for Dipole {
    type Output = FlatPointAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group2())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for DipoleAligningOrigin {
    type Output = FlatPointAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group1())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for DipoleAtInfinity {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for DipoleAtInfinity {
    type Output = FlatPointAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group1())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for DipoleAtOrigin {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for DipoleAtOrigin {
    type Output = FlatPointAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group1())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for DipoleInversion {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for DipoleInversion {
    type Output = FlectorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for DipoleInversionAligningOrigin {
    type Output = FlectorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for DipoleInversionAtInfinity {
    type Output = FlectorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group1().with_w(self[e3215]))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for DipoleInversionAtOrigin {
    type Output = FlectorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for DipoleInversionOrthogonalOrigin {
    type Output = FlectorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for DipoleOrthogonalOrigin {
    type Output = FlatPointAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group2())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for FlatPoint {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for FlatPoint {
    type Output = FlatPointAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0().xyz())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<FlatBulkPrefixOrPostfix> for FlatPointAtInfinity {
    fn div_assign(&mut self, _rhs: FlatBulkPrefixOrPostfix) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        self
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for Flector {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for Flector {
    type Output = FlectorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for FlectorAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<FlatBulkPrefixOrPostfix> for FlectorAtInfinity {
    fn div_assign(&mut self, _rhs: FlatBulkPrefixOrPostfix) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for FlectorAtInfinity {
    type Output = FlectorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        self
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for Horizon {
    type Output = Horizon;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<FlatBulkPrefixOrPostfix> for Horizon {
    fn div_assign(&mut self, _rhs: FlatBulkPrefixOrPostfix) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for Horizon {
    type Output = Horizon;
    fn flat_bulk(self) -> Self::Output {
        self
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for Infinity {
    type Output = Infinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<FlatBulkPrefixOrPostfix> for Infinity {
    fn div_assign(&mut self, _rhs: FlatBulkPrefixOrPostfix) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for Infinity {
    type Output = Infinity;
    fn flat_bulk(self) -> Self::Output {
        self
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for Line {
    type Output = LineAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for Line {
    type Output = LineAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group1())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for LineAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<FlatBulkPrefixOrPostfix> for LineAtInfinity {
    fn div_assign(&mut self, _rhs: FlatBulkPrefixOrPostfix) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for LineAtInfinity {
    type Output = LineAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        self
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for Motor {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for Motor {
    type Output = MotorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group1())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for MotorAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl std::ops::DivAssign<FlatBulkPrefixOrPostfix> for MotorAtInfinity {
    fn div_assign(&mut self, _rhs: FlatBulkPrefixOrPostfix) {
        *self = self.flat_bulk()
    }
}
impl FlatBulk for MotorAtInfinity {
    type Output = MotorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        self
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
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            self[e3215],
        )
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for Plane {
    type Output = Horizon;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for Plane {
    type Output = Horizon;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e3215])
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for RoundPoint {
    type Output = Infinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for RoundPoint {
    type Output = Infinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e5])
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = Infinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for RoundPointAtOrigin {
    type Output = Infinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e5])
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for Sphere {
    type Output = Horizon;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for Sphere {
    type Output = Horizon;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e3215])
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for SphereAtOrigin {
    type Output = Horizon;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for SphereAtOrigin {
    type Output = Horizon;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e3215])
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for VersorEven {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for VersorEven {
    type Output = MotorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group2())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for VersorEvenAligningOrigin {
    type Output = MotorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group2())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for VersorEvenAtInfinity {
    type Output = MotorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group2())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for VersorEvenAtOrigin {
    type Output = MotorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group1())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for VersorEvenOrthogonalOrigin {
    type Output = MotorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group1())
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for VersorOdd {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for VersorOdd {
    type Output = FlectorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for VersorOddAtInfinity {
    type Output = FlectorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]))
    }
}
impl std::ops::Div<FlatBulkPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: FlatBulkPrefixOrPostfix) -> Self::Output {
        self.flat_bulk()
    }
}
impl FlatBulk for VersorOddOrthogonalOrigin {
    type Output = FlectorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]))
    }
}
