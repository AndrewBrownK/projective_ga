// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 23
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         2       3       3
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         2       3       3
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiCircleOnOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * f32::powi(self[e23], 2)
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiCircleRotor {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * f32::powi(self[e23], 2)
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiCircleRotorAligningOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * f32::powi(self[e23], 2)
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiCircleRotorOnOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * f32::powi(self[e23], 2)
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiDipoleInversion {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * f32::powi(self[e4], 2)
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiDipoleInversionOnOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * f32::powi(self[e4], 2)
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        (self[e423] * self[e423] * self[e321]) + (self[e431] * self[e431] * self[e321]) + (self[e412] * self[e412] * self[e321])
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiDualNum {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[scalar] / (self[e1234])
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        3
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        (self[e1] * self[e1] / (self[e4])) + (self[e2] * self[e2] / (self[e4])) + (self[e3] * self[e3] / (self[e4]))
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiVersorEvenOnOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * f32::powi(self[e23], 2)
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        (self[e423] * self[e423] * self[e321]) + (self[e431] * self[e431] * self[e321]) + (self[e412] * self[e412] * self[e321])
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        (self[e423] * self[e423] * self[e321]) + (self[e431] * self[e431] * self[e321]) + (self[e412] * self[e412] * self[e321])
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        (self[e423] * self[e423] * self[e321]) + (self[e431] * self[e431] * self[e321]) + (self[e412] * self[e412] * self[e321])
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for Dipole {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * f32::powi(self[e23], 2)
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for DipoleInversion {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * f32::powi(self[e23], 2)
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for DipoleInversionOrthogonalOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * f32::powi(self[e23], 2)
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for DipoleOrthogonalOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * f32::powi(self[e23], 2)
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for MultiVector {
    fn unitized_round_norm(self) -> f32 {
        0.0
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for RoundPoint {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        3
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        (self[e1] * self[e1] / (self[e4])) + (self[e2] * self[e2] / (self[e4])) + (self[e3] * self[e3] / (self[e4]))
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for VersorEven {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * f32::powi(self[e4], 2)
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for VersorEvenOrthogonalOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * f32::powi(self[e4], 2)
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for VersorOdd {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * f32::powi(self[e23], 2)
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for VersorOddOrthogonalOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * f32::powi(self[e23], 2)
    }
}
