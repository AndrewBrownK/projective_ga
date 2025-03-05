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
//   Median:         2       0       0
//  Average:         2       0       0
//  Maximum:        23      16       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       0       0
//  Average:         2       0       0
//  Maximum:        23      16       0
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for AntiCircleRotor {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e45]);
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for AntiCircleRotorAtInfinity {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e45]);
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1().xyz());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2));
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2));
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2));
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for AntiMysteryCircleRotor {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e45]);
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2));
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl std::ops::DivAssign<FlatWeightNormPrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: FlatWeightNormPrefixOrPostfix) {
        *self = self.flat_weight_norm()
    }
}
impl FlatWeightNorm for AntiScalar {
    fn flat_weight_norm(self) -> AntiScalar {
        return self;
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for Circle {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1().xyz());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2));
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for CircleAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2));
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for CircleAtInfinity {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2));
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for CircleOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2));
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for CircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2) + f32::powi(sub_type[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group1().with_w(self[e12345]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2) + f32::powi(sub_type[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0().with_w(self[e12345]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2) + f32::powi(sub_type[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2) + f32::powi(sub_type[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group1().with_w(self[e12345]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2) + f32::powi(sub_type[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for Dipole {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for Dipole {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e45]);
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for DipoleAligningOrigin {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e45]);
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for DipoleAtInfinity {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for DipoleAtInfinity {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e45]);
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for DipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e45], 2) + f32::powi(sub_type[e4235], 2) + f32::powi(sub_type[e4315], 2) + f32::powi(sub_type[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e45], 2) + f32::powi(sub_type[e4235], 2) + f32::powi(sub_type[e4315], 2) + f32::powi(sub_type[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e45], 2) + f32::powi(sub_type[e4235], 2) + f32::powi(sub_type[e4315], 2) + f32::powi(sub_type[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e45], 2) + f32::powi(sub_type[e4235], 2) + f32::powi(sub_type[e4315], 2) + f32::powi(sub_type[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for DipoleOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for DipoleOnOrigin {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e45]);
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for DualNum {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e12345]);
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for FlatOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for FlatOrigin {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e45]);
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for FlatPoint {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for FlatPoint {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e45]);
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for Flector {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e45], 2) + f32::powi(sub_type[e4235], 2) + f32::powi(sub_type[e4315], 2) + f32::powi(sub_type[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for FlectorOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            f32::powi(self[e45], 2) + f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for Line {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2));
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for LineOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for LineOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2));
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for Motor {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0());
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2) + f32::powi(sub_type[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for MotorOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2) + f32::powi(self[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for MultiVector {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       23       16        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = MultiVector::from_groups(
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
        );
        return AntiScalar::from_groups(
            // e12345
            2.0 * (sub_type[e4] * sub_type[e5])
                + 2.0 * (sub_type[e423] * sub_type[e235])
                + 2.0 * (sub_type[e431] * sub_type[e315])
                + 2.0 * (sub_type[e412] * sub_type[e125])
                + f32::powi(sub_type[e12345], 2)
                + f32::powi(sub_type[e45], 2)
                + f32::powi(sub_type[e415], 2)
                + f32::powi(sub_type[e425], 2)
                + f32::powi(sub_type[e435], 2)
                + f32::powi(sub_type[e4235], 2)
                + f32::powi(sub_type[e4315], 2)
                + f32::powi(sub_type[e4125], 2)
                - f32::powi(sub_type[scalar], 2)
                - f32::powi(sub_type[e1], 2)
                - f32::powi(sub_type[e2], 2)
                - f32::powi(sub_type[e3], 2)
                - f32::powi(sub_type[e23], 2)
                - f32::powi(sub_type[e31], 2)
                - f32::powi(sub_type[e12], 2)
                - f32::powi(sub_type[e321], 2)
                - 2.0 * (sub_type[e41] * sub_type[e15])
                - 2.0 * (sub_type[e42] * sub_type[e25])
                - 2.0 * (sub_type[e43] * sub_type[e35])
                - 2.0 * (sub_type[e1234] * sub_type[e3215]),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for MysteryCircle {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for MysteryCircle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2));
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for MysteryCircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2) + f32::powi(sub_type[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for MysteryDipole {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for MysteryDipole {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e45]);
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for MysteryDipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e45], 2) + f32::powi(sub_type[e4235], 2) + f32::powi(sub_type[e4315], 2) + f32::powi(sub_type[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for MysteryVersorEven {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2) + f32::powi(sub_type[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for MysteryVersorOdd {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e45], 2) + f32::powi(sub_type[e4235], 2) + f32::powi(sub_type[e4315], 2) + f32::powi(sub_type[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for Plane {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(sub_type[e4235], 2) + f32::powi(sub_type[e4315], 2) + f32::powi(sub_type[e4125], 2));
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for PlaneOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for PlaneOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2));
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for Sphere {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(sub_type[e4235], 2) + f32::powi(sub_type[e4315], 2) + f32::powi(sub_type[e4125], 2));
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for SphereOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(sub_type[e4235], 2) + f32::powi(sub_type[e4315], 2) + f32::powi(sub_type[e4125], 2));
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for VersorEven {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2) + f32::powi(sub_type[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2) + f32::powi(sub_type[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2) + f32::powi(sub_type[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e415], 2) + f32::powi(sub_type[e425], 2) + f32::powi(sub_type[e435], 2) + f32::powi(sub_type[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for VersorOdd {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e45], 2) + f32::powi(sub_type[e4235], 2) + f32::powi(sub_type[e4315], 2) + f32::powi(sub_type[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatWeightNormPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm()
    }
}
impl FlatWeightNorm for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e45], self[e4235], self[e4315], self[e4125]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(sub_type[e45], 2) + f32::powi(sub_type[e4235], 2) + f32::powi(sub_type[e4315], 2) + f32::powi(sub_type[e4125], 2),
        );
    }
}
