// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
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
impl UnitizedRoundNormSquared for AntiCircleOnOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNormSquared for AntiCircleRotor {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNormSquared for AntiCircleRotorAligningOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNormSquared for AntiCircleRotorOnOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNormSquared for AntiDipoleInversion {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * self[e4] * self[e4]
    }
}
impl UnitizedRoundNormSquared for AntiDipoleInversionOnOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * self[e4] * self[e4]
    }
}
impl UnitizedRoundNormSquared for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        (self[e423] * self[e423] * self[e321] * self[e321]) + (self[e431] * self[e431] * self[e321] * self[e321]) + (self[e412] * self[e412] * self[e321] * self[e321])
    }
}
impl UnitizedRoundNormSquared for AntiDualNum {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[scalar] * self[scalar] / (self[e1234] * self[e1234])
    }
}
impl UnitizedRoundNormSquared for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        (self[e1] * self[e1] / (self[e4] * self[e4])) + (self[e2] * self[e2] / (self[e4] * self[e4])) + (self[e3] * self[e3] / (self[e4] * self[e4]))
    }
}
impl UnitizedRoundNormSquared for AntiVersorEvenOnOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNormSquared for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        (self[e423] * self[e423] * self[e321] * self[e321]) + (self[e431] * self[e431] * self[e321] * self[e321]) + (self[e412] * self[e412] * self[e321] * self[e321])
    }
}
impl UnitizedRoundNormSquared for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        (self[e423] * self[e423] * self[e321] * self[e321]) + (self[e431] * self[e431] * self[e321] * self[e321]) + (self[e412] * self[e412] * self[e321] * self[e321])
    }
}
impl UnitizedRoundNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        (self[e423] * self[e423] * self[e321] * self[e321]) + (self[e431] * self[e431] * self[e321] * self[e321]) + (self[e412] * self[e412] * self[e321] * self[e321])
    }
}
impl UnitizedRoundNormSquared for Dipole {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNormSquared for DipoleInversion {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNormSquared for DipoleInversionOrthogonalOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNormSquared for DipoleOrthogonalOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNormSquared for MultiVector {
    fn unitized_round_norm_squared(self) -> f32 {
        0.0
    }
}
impl UnitizedRoundNormSquared for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        (self[e1] * self[e1] / (self[e4] * self[e4])) + (self[e2] * self[e2] / (self[e4] * self[e4])) + (self[e3] * self[e3] / (self[e4] * self[e4]))
    }
}
impl UnitizedRoundNormSquared for VersorEven {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * self[e4] * self[e4]
    }
}
impl UnitizedRoundNormSquared for VersorEvenOrthogonalOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * self[e4] * self[e4]
    }
}
impl UnitizedRoundNormSquared for VersorOdd {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNormSquared for VersorOddOrthogonalOrigin {
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
