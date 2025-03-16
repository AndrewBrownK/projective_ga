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
//  Maximum:         2       3       3
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         2       3       3
impl UnitizedRoundNorm for AntiCircleOnOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNorm for AntiCircleRotor {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNorm for AntiCircleRotorAligningOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNorm for AntiCircleRotorOnOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNorm for AntiDipoleInversion {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * self[e4] * self[e4]
    }
}
impl UnitizedRoundNorm for AntiDipoleInversionOnOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * self[e4] * self[e4]
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
impl UnitizedRoundNorm for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[scalar] / self[e1234]
    }
}
impl UnitizedRoundNorm for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        3
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        (self[e1] * self[e1] / self[e4]) + (self[e2] * self[e2] / self[e4]) + (self[e3] * self[e3] / self[e4])
    }
}
impl UnitizedRoundNorm for AntiVersorEvenOnOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
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
impl UnitizedRoundNorm for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        (self[e423] * self[e423] * self[e321]) + (self[e431] * self[e431] * self[e321]) + (self[e412] * self[e412] * self[e321])
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
impl UnitizedRoundNorm for Dipole {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNorm for DipoleInversion {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNorm for DipoleInversionOrthogonalOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNorm for DipoleOrthogonalOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNorm for MultiVector {
    fn unitized_round_norm(self) -> f32 {
        0.0
    }
}
impl UnitizedRoundNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        3
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        (self[e1] * self[e1] / self[e4]) + (self[e2] * self[e2] / self[e4]) + (self[e3] * self[e3] / self[e4])
    }
}
impl UnitizedRoundNorm for VersorEven {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * self[e4] * self[e4]
    }
}
impl UnitizedRoundNorm for VersorEvenOrthogonalOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * self[e4] * self[e4]
    }
}
impl UnitizedRoundNorm for VersorOdd {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNorm for VersorOddOrthogonalOrigin {
    fn unitized_round_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
