// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 23
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         0       3       0     N/A
//  Average:         0       4       0     N/A
//  Maximum:         2       9       3     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         0       3       0       0
//  Average:         0       4       0       0
//  Maximum:         2       9       3       0
impl UnitizedRoundNormSquared for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNormSquared for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNormSquared for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * self[e4] * self[e4]
    }
}
impl UnitizedRoundNormSquared for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * self[e4] * self[e4]
    }
}
impl UnitizedRoundNormSquared for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        9        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        let wedge_g0 = self.group0().xyz();
        (wedge_g0[0] * wedge_g0[0] * self[e321] * self[e321]) + (wedge_g0[1] * wedge_g0[1] * self[e321] * self[e321]) + (wedge_g0[2] * wedge_g0[2] * self[e321] * self[e321])
    }
}
impl UnitizedRoundNormSquared for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        1        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[scalar] * self[scalar] / (self[e1234] * self[e1234])
    }
}
impl UnitizedRoundNormSquared for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        9        3        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type_g0 = self.group0().xyz();
        (sub_type_g0[0] * sub_type_g0[0] / (self[e4] * self[e4]))
            + (sub_type_g0[1] * sub_type_g0[1] / (self[e4] * self[e4]))
            + (sub_type_g0[2] * sub_type_g0[2] / (self[e4] * self[e4]))
    }
}
impl UnitizedRoundNormSquared for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNormSquared for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        9        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        (self[e423] * self[e423] * self[e321] * self[e321]) + (self[e431] * self[e431] * self[e321] * self[e321]) + (self[e412] * self[e412] * self[e321] * self[e321])
    }
}
impl UnitizedRoundNormSquared for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        9        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        let wedge_g0 = self.group0().xyz();
        (wedge_g0[0] * wedge_g0[0] * self[e321] * self[e321]) + (wedge_g0[1] * wedge_g0[1] * self[e321] * self[e321]) + (wedge_g0[2] * wedge_g0[2] * self[e321] * self[e321])
    }
}
impl UnitizedRoundNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        9        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        (self[e423] * self[e423] * self[e321] * self[e321]) + (self[e431] * self[e431] * self[e321] * self[e321]) + (self[e412] * self[e412] * self[e321] * self[e321])
    }
}
impl UnitizedRoundNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNormSquared for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNormSquared for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
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
    //      add/sub      mul      div      pow
    // f32        2        9        3        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type_g0 = self.group0().xyz();
        (sub_type_g0[0] * sub_type_g0[0] / (self[e4] * self[e4]))
            + (sub_type_g0[1] * sub_type_g0[1] / (self[e4] * self[e4]))
            + (sub_type_g0[2] * sub_type_g0[2] / (self[e4] * self[e4]))
    }
}
impl UnitizedRoundNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * self[e4] * self[e4]
    }
}
impl UnitizedRoundNormSquared for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * self[e4] * self[e4]
    }
}
impl UnitizedRoundNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRoundNormSquared for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
