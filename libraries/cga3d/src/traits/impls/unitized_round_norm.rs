// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 10
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         0       3       0     N/A
//  Average:         0       3       0     N/A
//  Maximum:         2       6       3     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         0       3       0       0
//  Average:         0       3       0       0
//  Maximum:         2       6       3       0
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * self[e4] * self[e4]
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
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
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
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
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
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
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
    //      add/sub      mul      div      pow
    // f32        2        6        3        0
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        let sub_type_g0_xyz = self.group0().xyz();
        (sub_type_g0_xyz[0] * sub_type_g0_xyz[0] / self[e4]) + (sub_type_g0_xyz[1] * sub_type_g0_xyz[1] / self[e4]) + (sub_type_g0_xyz[2] * sub_type_g0_xyz[2] / self[e4])
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * self[e4] * self[e4]
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
