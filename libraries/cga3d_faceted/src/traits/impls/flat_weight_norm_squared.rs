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
//  Maximum:         7       0       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       0       0
//  Average:         2       0       0
//  Maximum:         7       0       0
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for AntiCircleRotor {
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e45] * self[e45]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for AntiCircleRotorAtInfinity {
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e45] * self[e45]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for AntiMysteryCircleRotor {
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e45] * self[e45]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for AntiScalar {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl std::ops::DivAssign<FlatWeightNormSquaredPrefixOrPostfix> for AntiScalar {
    fn div_assign(&mut self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) {
        *self = self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for AntiScalar {
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * self[e12345]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for Circle {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for CircleAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for CircleAtInfinity {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for CircleOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
        );
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
        );
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
        );
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
        );
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435],
        );
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for Dipole {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for Dipole {
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e45] * self[e45]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for DipoleAligningOrigin {
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e45] * self[e45]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for DipoleAtInfinity {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for DipoleAtInfinity {
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e45] * self[e45]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        );
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        );
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        );
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        );
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for DipoleOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for DipoleOnOrigin {
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e45] * self[e45]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for DualNum {
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * self[e12345]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for FlatOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for FlatOrigin {
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e45] * self[e45]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for FlatPoint {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for FlatPoint {
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e45] * self[e45]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for Flector {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        );
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for FlectorOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        );
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for Line {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for LineOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for LineOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for Motor {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
        );
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for MotorOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
        );
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            self[e12345] * self[e12345]
                + self[e45] * self[e45]
                + self[e415] * self[e415]
                + self[e425] * self[e425]
                + self[e435] * self[e435]
                + self[e4235] * self[e4235]
                + self[e4315] * self[e4315]
                + self[e4125] * self[e4125],
        );
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for MysteryCircle {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for MysteryCircle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for MysteryCircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
        );
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for MysteryDipole {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for MysteryDipole {
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e45] * self[e45]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for MysteryDipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        );
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for MysteryVersorEven {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435],
        );
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for MysteryVersorOdd {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125] + self[e45] * self[e45],
        );
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for Plane {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for PlaneOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for PlaneOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for Sphere {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for SphereOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125]);
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435],
        );
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435],
        );
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435],
        );
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435],
        );
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        );
    }
}
impl std::ops::Div<FlatWeightNormSquaredPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = AntiScalar;
    fn div(self, _rhs: FlatWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_weight_norm_squared()
    }
}
impl FlatWeightNormSquared for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        );
    }
}
