// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 49
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
impl RoundWeightNormSquared for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43])
    }
}
impl RoundWeightNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43])
    }
}
impl RoundWeightNormSquared for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43])
    }
}
impl RoundWeightNormSquared for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43])
    }
}
impl RoundWeightNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4])
    }
}
impl RoundWeightNormSquared for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4])
    }
}
impl RoundWeightNormSquared for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4])
    }
}
impl RoundWeightNormSquared for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412])
    }
}
impl RoundWeightNormSquared for AntiDualNum {
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e1234] * self[e1234])
    }
}
impl RoundWeightNormSquared for AntiSphereOnOrigin {
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e4] * self[e4])
    }
}
impl RoundWeightNormSquared for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234])
    }
}
impl RoundWeightNormSquared for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412])
    }
}
impl RoundWeightNormSquared for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412])
    }
}
impl RoundWeightNormSquared for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412])
    }
}
impl RoundWeightNormSquared for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412])
    }
}
impl RoundWeightNormSquared for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412])
    }
}
impl RoundWeightNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412])
    }
}
impl RoundWeightNormSquared for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412])
    }
}
impl RoundWeightNormSquared for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412])
    }
}
impl RoundWeightNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43])
    }
}
impl RoundWeightNormSquared for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43])
    }
}
impl RoundWeightNormSquared for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43])
    }
}
impl RoundWeightNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234])
    }
}
impl RoundWeightNormSquared for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234])
    }
}
impl RoundWeightNormSquared for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234])
    }
}
impl RoundWeightNormSquared for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234])
    }
}
impl RoundWeightNormSquared for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234])
    }
}
impl RoundWeightNormSquared for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43])
    }
}
impl RoundWeightNormSquared for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43])
    }
}
impl RoundWeightNormSquared for DualNum {
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e4] * self[e4])
    }
}
impl RoundWeightNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e4] * self[e4]
                + self[e41] * self[e41]
                + self[e42] * self[e42]
                + self[e43] * self[e43]
                + self[e423] * self[e423]
                + self[e431] * self[e431]
                + self[e412] * self[e412]
                + self[e1234] * self[e1234],
        )
    }
}
impl RoundWeightNormSquared for NullCircleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412])
    }
}
impl RoundWeightNormSquared for NullDipoleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43])
    }
}
impl RoundWeightNormSquared for NullDipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234])
    }
}
impl RoundWeightNormSquared for NullSphereAtOrigin {
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e1234] * self[e1234])
    }
}
impl RoundWeightNormSquared for NullVersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let wedge_g0 = self.group0().wxyz();
        AntiScalar::from_groups(
            // e12345
            wedge_g0[0] * wedge_g0[0] + wedge_g0[1] * wedge_g0[1] + wedge_g0[2] * wedge_g0[2] + wedge_g0[3] * wedge_g0[3],
        )
    }
}
impl RoundWeightNormSquared for Origin {
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e4] * self[e4])
    }
}
impl RoundWeightNormSquared for RoundPoint {
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e4] * self[e4])
    }
}
impl RoundWeightNormSquared for RoundPointAtOrigin {
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e4] * self[e4])
    }
}
impl RoundWeightNormSquared for Sphere {
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e1234] * self[e1234])
    }
}
impl RoundWeightNormSquared for SphereAtOrigin {
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e1234] * self[e1234])
    }
}
impl RoundWeightNormSquared for SphereOnOrigin {
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e1234] * self[e1234])
    }
}
impl RoundWeightNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4])
    }
}
impl RoundWeightNormSquared for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4])
    }
}
impl RoundWeightNormSquared for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let wedge_g0 = self.group0().wxyz();
        AntiScalar::from_groups(
            // e12345
            wedge_g0[0] * wedge_g0[0] + wedge_g0[1] * wedge_g0[1] + wedge_g0[2] * wedge_g0[2] + wedge_g0[3] * wedge_g0[3],
        )
    }
}
impl RoundWeightNormSquared for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4])
    }
}
impl RoundWeightNormSquared for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4])
    }
}
impl RoundWeightNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234])
    }
}
impl RoundWeightNormSquared for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234])
    }
}
