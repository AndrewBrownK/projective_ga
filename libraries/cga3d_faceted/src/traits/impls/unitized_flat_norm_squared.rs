// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 30
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         2       1       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         2       1       0
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        -(self[e15] * self[e15] * f32::powi(self[e45], -2)) - (self[e25] * self[e25] * f32::powi(self[e45], -2)) - (self[e35] * self[e35] * f32::powi(self[e45], -2))
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        -(self[e15] * self[e15] * f32::powi(self[e45], -2)) - (self[e25] * self[e25] * f32::powi(self[e45], -2)) - (self[e35] * self[e35] * f32::powi(self[e45], -2))
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for AntiDipoleInversion {
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * f32::powi(self[e5], 2)
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for AntiDipoleInversionAtInfinity {
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        f32::powi(self.group1().with_w(self[e5]).wxyz()[0], 2) * f32::powi(self[e415], 2)
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for AntiDipoleInversionOrthogonalOrigin {
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e5] * self[e5] * f32::powi(self[e415], 2)
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Circle {
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * f32::powi(self[e235], 2)
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for CircleAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for CircleAligningOrigin {
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * f32::powi(self[e235], 2)
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for CircleAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for CircleAtInfinity {
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * f32::powi(self[e235], 2)
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for CircleRotor {
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * f32::powi(self[e235], 2)
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for CircleRotorAligningOrigin {
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * f32::powi(self[e235], 2)
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for CircleRotorAligningOriginAtInfinity {
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * f32::powi(self[e235], 2)
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for CircleRotorAtInfinity {
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * f32::powi(self[e235], 2)
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        -(self[e15] * self[e15] * f32::powi(self[e45], -2)) - (self[e25] * self[e25] * f32::powi(self[e45], -2)) - (self[e35] * self[e35] * f32::powi(self[e45], -2))
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        -(self[e15] * self[e15] * f32::powi(self[e45], -2)) - (self[e25] * self[e25] * f32::powi(self[e45], -2)) - (self[e35] * self[e35] * f32::powi(self[e45], -2))
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for DipoleAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        -(self[e15] * self[e15] * f32::powi(self[e45], -2)) - (self[e25] * self[e25] * f32::powi(self[e45], -2)) - (self[e35] * self[e35] * f32::powi(self[e45], -2))
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e45] * self[e45] * f32::powi(self[e15], 2) * -1.0
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e45] * self[e45] * f32::powi(self[e15], 2) * -1.0
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e45] * self[e45] * f32::powi(self[e15], 2) * -1.0
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for FlatPoint {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        -(self[e15] * self[e15] * f32::powi(self[e45], -2)) - (self[e25] * self[e25] * f32::powi(self[e45], -2)) - (self[e35] * self[e35] * f32::powi(self[e45], -2))
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Flector {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e15] * self[e15] * f32::powi(self[e45], 2) * -1.0
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Line {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Line {
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * f32::powi(self[e235], 2)
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Motor {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Motor {
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        f32::powi(self.group1().wxyz()[0], 2) * f32::powi(self[e415], 2)
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for MultiVector {
    fn unitized_flat_norm_squared(self) -> f32 {
        0.0
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Plane {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        -(self[e4235] * self[e4235] * f32::powi(self[e3215], 2)) - (self[e4315] * self[e4315] * f32::powi(self[e3215], 2)) - (self[e4125] * self[e4125] * f32::powi(self[e3215], 2))
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for Sphere {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        -(self[e4235] * self[e4235] * f32::powi(self[e3215], 2)) - (self[e4315] * self[e4315] * f32::powi(self[e3215], 2)) - (self[e4125] * self[e4125] * f32::powi(self[e3215], 2))
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for VersorEven {
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        f32::powi(self.group2().wxyz()[0], 2) * f32::powi(self[e415], 2)
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for VersorEvenAligningOrigin {
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        f32::powi(self.group2().wxyz()[0], 2) * f32::powi(self[e415], 2)
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for VersorEvenAtInfinity {
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        f32::powi(self.group2().wxyz()[0], 2) * f32::powi(self[e415], 2)
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e45] * self[e45] * f32::powi(self[e15], 2) * -1.0
    }
}
impl std::ops::Div<UnitizedFlatNormSquaredPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm_squared()
    }
}
impl UnitizedFlatNormSquared for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e15] * self[e15] * f32::powi(self[e45], 2) * -1.0
    }
}
