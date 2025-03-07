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
//  Maximum:         2       0       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         2       0       0
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiCircleOnOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self[e41] * self[e41] * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiCircleRotor {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self[e41] * self[e41] * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiCircleRotorAligningOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self[e41] * self[e41] * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiCircleRotorOnOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self[e41] * self[e41] * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiDipoleInversion {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return f32::powi(self.group0().with_w(self[e4]).wxyz()[0], 2) * f32::powi(self[e321], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiDipoleInversionOnOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self[e321] * self[e321] * f32::powi(self[e4], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self[e423] * self[e423] * f32::powi(self[e321], 2)) + (self[e431] * self[e431] * f32::powi(self[e321], 2)) + (self[e412] * self[e412] * f32::powi(self[e321], 2));
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiDualNum {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiDualNum {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self[scalar] * self[scalar] * f32::powi(self[e1234], -2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self[e1] * self[e1] * f32::powi(self[e4], -2)) + (self[e2] * self[e2] * f32::powi(self[e4], -2)) + (self[e3] * self[e3] * f32::powi(self[e4], -2));
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiVersorEvenOnOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self[e41] * self[e41] * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self[e423] * self[e423] * f32::powi(self[e321], 2)) + (self[e431] * self[e431] * f32::powi(self[e321], 2)) + (self[e412] * self[e412] * f32::powi(self[e321], 2));
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self[e423] * self[e423] * f32::powi(self[e321], 2)) + (self[e431] * self[e431] * f32::powi(self[e321], 2)) + (self[e412] * self[e412] * f32::powi(self[e321], 2));
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self[e423] * self[e423] * f32::powi(self[e321], 2)) + (self[e431] * self[e431] * f32::powi(self[e321], 2)) + (self[e412] * self[e412] * f32::powi(self[e321], 2));
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for Dipole {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self[e41] * self[e41] * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for DipoleInversion {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self[e41] * self[e41] * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for DipoleInversionOrthogonalOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self[e41] * self[e41] * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for DipoleOrthogonalOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self[e41] * self[e41] * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for MultiVector {
    fn unitized_round_norm_squared(self) -> f32 {
        return 0.0;
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for RoundPoint {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self[e1] * self[e1] * f32::powi(self[e4], -2)) + (self[e2] * self[e2] * f32::powi(self[e4], -2)) + (self[e3] * self[e3] * f32::powi(self[e4], -2));
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for VersorEven {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self[e321] * self[e321] * f32::powi(self[e4], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for VersorEvenOrthogonalOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self[e321] * self[e321] * f32::powi(self[e4], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for VersorOdd {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self[e41] * self[e41] * f32::powi(self[e23], 2);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for VersorOddOrthogonalOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self[e41] * self[e41] * f32::powi(self[e23], 2);
    }
}
