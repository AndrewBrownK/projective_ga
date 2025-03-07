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
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for AntiCircleOnOrigin {
    type Output = AntiLineOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group1())
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for AntiCircleRotor {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]))
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for AntiCircleRotorAligningOrigin {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group1().with_w(self[scalar]))
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for AntiCircleRotorAligningOriginAtInfinity {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group0().with_w(self[scalar]))
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for AntiCircleRotorAtInfinity {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]))
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for AntiCircleRotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group1().with_w(self[scalar]))
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiDipoleInversion {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for AntiDipoleInversion {
    type Output = AntiFlectorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]))
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for AntiDipoleInversionAtInfinity {
    type Output = AntiFlectorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]))
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for AntiDipoleInversionOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]))
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for AntiDipoleOnOrigin {
    type Output = AntiFlatOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e321])
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiDualNum {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for AntiDualNum {
    type Output = Scalar;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar])
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiFlatOrigin {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl std::ops::DivAssign<RoundBulkPrefixOrPostfix> for AntiFlatOrigin {
    fn div_assign(&mut self, _rhs: RoundBulkPrefixOrPostfix) {
        *self = self.round_bulk()
    }
}
impl RoundBulk for AntiFlatOrigin {
    type Output = AntiFlatOrigin;
    fn round_bulk(self) -> Self::Output {
        self
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiFlatPoint {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for AntiFlatPoint {
    type Output = AntiFlatOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e321])
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiFlector {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for AntiFlector {
    type Output = AntiFlectorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]))
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiFlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl std::ops::DivAssign<RoundBulkPrefixOrPostfix> for AntiFlectorOnOrigin {
    fn div_assign(&mut self, _rhs: RoundBulkPrefixOrPostfix) {
        *self = self.round_bulk()
    }
}
impl RoundBulk for AntiFlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        self
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiLine {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for AntiLine {
    type Output = AntiLineOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0())
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl std::ops::DivAssign<RoundBulkPrefixOrPostfix> for AntiLineOnOrigin {
    fn div_assign(&mut self, _rhs: RoundBulkPrefixOrPostfix) {
        *self = self.round_bulk()
    }
}
impl RoundBulk for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn round_bulk(self) -> Self::Output {
        self
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiMotor {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for AntiMotor {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group0())
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiMotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl std::ops::DivAssign<RoundBulkPrefixOrPostfix> for AntiMotorOnOrigin {
    fn div_assign(&mut self, _rhs: RoundBulkPrefixOrPostfix) {
        *self = self.round_bulk()
    }
}
impl RoundBulk for AntiMotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        self
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for AntiMysteryCircleRotor {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]))
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for AntiMysteryDipoleInversion {
    type Output = AntiFlectorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]))
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiPlane {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for AntiPlane {
    type Output = AntiPlaneOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0().xyz())
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl std::ops::DivAssign<RoundBulkPrefixOrPostfix> for AntiPlaneOnOrigin {
    fn div_assign(&mut self, _rhs: RoundBulkPrefixOrPostfix) {
        *self = self.round_bulk()
    }
}
impl RoundBulk for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn round_bulk(self) -> Self::Output {
        self
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for AntiSphereOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0().xyz())
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for AntiVersorEvenOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]))
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for Circle {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for Circle {
    type Output = AntiFlatOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e321])
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for CircleAtInfinity {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for CircleAtInfinity {
    type Output = AntiFlatOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e321])
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for CircleOrthogonalOrigin {
    type Output = AntiFlatOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e321])
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for CircleRotor {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for CircleRotor {
    type Output = AntiFlatOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e321])
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for CircleRotorAtInfinity {
    type Output = AntiFlatOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e321])
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for Dipole {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for Dipole {
    type Output = AntiLineOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group1().xyz())
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for DipoleAtInfinity {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for DipoleAtInfinity {
    type Output = AntiLineOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0().xyz())
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for DipoleInversion {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for DipoleInversion {
    type Output = AntiLineOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group1().xyz())
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for DipoleInversionAtInfinity {
    type Output = AntiLineOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0().xyz())
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for DipoleInversionOrthogonalOrigin {
    type Output = AntiLineOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group1())
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for DipoleOrthogonalOrigin {
    type Output = AntiLineOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group1())
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl std::ops::DivAssign<RoundBulkPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: RoundBulkPrefixOrPostfix) {
        *self = self.round_bulk()
    }
}
impl RoundBulk for MultiVector {
    type Output = MultiVector;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for MysteryCircle {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for MysteryCircle {
    type Output = AntiFlatOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e321])
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for MysteryCircleRotor {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for MysteryCircleRotor {
    type Output = AntiFlatOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ self[e321])
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for MysteryDipole {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for MysteryDipole {
    type Output = AntiLineOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0().xyz())
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for MysteryDipoleInversion {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for MysteryDipoleInversion {
    type Output = AntiLineOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0().xyz())
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for MysteryVersorEven {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for MysteryVersorEven {
    type Output = AntiFlectorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]))
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for MysteryVersorOdd {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for MysteryVersorOdd {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]))
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for RoundPoint {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for RoundPoint {
    type Output = AntiPlaneOnOrigin;
    fn round_bulk(self) -> Self::Output {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0().xyz())
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl std::ops::DivAssign<RoundBulkPrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: RoundBulkPrefixOrPostfix) {
        *self = self.round_bulk()
    }
}
impl RoundBulk for Scalar {
    type Output = Scalar;
    fn round_bulk(self) -> Self::Output {
        self
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for VersorEven {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for VersorEven {
    type Output = AntiFlectorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]))
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for VersorEvenAtInfinity {
    type Output = AntiFlectorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]))
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for VersorEvenOrthogonalOrigin {
    type Output = AntiFlectorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]))
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for VersorOdd {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for VersorOdd {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]))
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for VersorOddAtInfinity {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]))
    }
}
impl std::ops::Div<RoundBulkPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: RoundBulkPrefixOrPostfix) -> Self::Output {
        self.round_bulk()
    }
}
impl RoundBulk for VersorOddOrthogonalOrigin {
    type Output = AntiMotorOnOrigin;
    fn round_bulk(self) -> Self::Output {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]))
    }
}
