// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 49
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         2       3       0     N/A
//  Average:         2       2       0     N/A
//  Maximum:         7       8       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         2       3       0       0
//  Average:         2       2       0       0
//  Maximum:         7       8       0       0
impl RoundWeightNorm for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43])
    }
}
impl RoundWeightNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43])
    }
}
impl RoundWeightNorm for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43])
    }
}
impl RoundWeightNorm for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_weight_norm(self) -> AntiScalar {
        let wedge_g0 = self.group0().xyz();
        AntiScalar::from_groups(/* e12345 */ wedge_g0[0] * wedge_g0[0] + wedge_g0[1] * wedge_g0[1] + wedge_g0[2] * wedge_g0[2])
    }
}
impl RoundWeightNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4])
    }
}
impl RoundWeightNorm for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4])
    }
}
impl RoundWeightNorm for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4])
    }
}
impl RoundWeightNorm for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_weight_norm(self) -> AntiScalar {
        let wedge_g0 = self.group0().xyz();
        AntiScalar::from_groups(/* e12345 */ wedge_g0[0] * wedge_g0[0] + wedge_g0[1] * wedge_g0[1] + wedge_g0[2] * wedge_g0[2])
    }
}
impl RoundWeightNorm for AntiDualNum {
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e1234])
    }
}
impl RoundWeightNorm for AntiSphereOnOrigin {
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e4])
    }
}
impl RoundWeightNorm for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let wedge_g0_xyz = self.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            wedge_g0_xyz[0] * wedge_g0_xyz[0] + wedge_g0_xyz[1] * wedge_g0_xyz[1] + wedge_g0_xyz[2] * wedge_g0_xyz[2] + self[e1234] * self[e1234],
        )
    }
}
impl RoundWeightNorm for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412])
    }
}
impl RoundWeightNorm for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412])
    }
}
impl RoundWeightNorm for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412])
    }
}
impl RoundWeightNorm for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412])
    }
}
impl RoundWeightNorm for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_weight_norm(self) -> AntiScalar {
        let wedge_g0 = self.group0().xyz();
        AntiScalar::from_groups(/* e12345 */ wedge_g0[0] * wedge_g0[0] + wedge_g0[1] * wedge_g0[1] + wedge_g0[2] * wedge_g0[2])
    }
}
impl RoundWeightNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412])
    }
}
impl RoundWeightNorm for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412])
    }
}
impl RoundWeightNorm for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_weight_norm(self) -> AntiScalar {
        let wedge_g0 = self.group0().xyz();
        AntiScalar::from_groups(/* e12345 */ wedge_g0[0] * wedge_g0[0] + wedge_g0[1] * wedge_g0[1] + wedge_g0[2] * wedge_g0[2])
    }
}
impl RoundWeightNorm for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43])
    }
}
impl RoundWeightNorm for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_weight_norm(self) -> AntiScalar {
        let wedge_g0 = self.group0().xyz();
        AntiScalar::from_groups(/* e12345 */ wedge_g0[0] * wedge_g0[0] + wedge_g0[1] * wedge_g0[1] + wedge_g0[2] * wedge_g0[2])
    }
}
impl RoundWeightNorm for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43])
    }
}
impl RoundWeightNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234])
    }
}
impl RoundWeightNorm for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let wedge_g0_xyz = self.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            wedge_g0_xyz[0] * wedge_g0_xyz[0] + wedge_g0_xyz[1] * wedge_g0_xyz[1] + wedge_g0_xyz[2] * wedge_g0_xyz[2] + self[e1234] * self[e1234],
        )
    }
}
impl RoundWeightNorm for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let wedge_g0_xyz = self.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            wedge_g0_xyz[0] * wedge_g0_xyz[0] + wedge_g0_xyz[1] * wedge_g0_xyz[1] + wedge_g0_xyz[2] * wedge_g0_xyz[2] + self[e1234] * self[e1234],
        )
    }
}
impl RoundWeightNorm for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let wedge_g0_xyz = self.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            wedge_g0_xyz[0] * wedge_g0_xyz[0] + wedge_g0_xyz[1] * wedge_g0_xyz[1] + wedge_g0_xyz[2] * wedge_g0_xyz[2] + self[e1234] * self[e1234],
        )
    }
}
impl RoundWeightNorm for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let wedge_g0_xyz = self.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            wedge_g0_xyz[0] * wedge_g0_xyz[0] + wedge_g0_xyz[1] * wedge_g0_xyz[1] + wedge_g0_xyz[2] * wedge_g0_xyz[2] + self[e1234] * self[e1234],
        )
    }
}
impl RoundWeightNorm for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_weight_norm(self) -> AntiScalar {
        let wedge_g0 = self.group0().xyz();
        AntiScalar::from_groups(/* e12345 */ wedge_g0[0] * wedge_g0[0] + wedge_g0[1] * wedge_g0[1] + wedge_g0[2] * wedge_g0[2])
    }
}
impl RoundWeightNorm for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43])
    }
}
impl RoundWeightNorm for DualNum {
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e4])
    }
}
impl RoundWeightNorm for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        7        8        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type_g3_xyz = self.group3().xyz();
        AntiScalar::from_groups(
            // e12345
            sub_type_g3_xyz[0] * sub_type_g3_xyz[0]
                + sub_type_g3_xyz[1] * sub_type_g3_xyz[1]
                + sub_type_g3_xyz[2] * sub_type_g3_xyz[2]
                + self[e4] * self[e4]
                + self[e423] * self[e423]
                + self[e431] * self[e431]
                + self[e412] * self[e412]
                + self[e1234] * self[e1234],
        )
    }
}
impl RoundWeightNorm for NullCircleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412])
    }
}
impl RoundWeightNorm for NullDipoleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43])
    }
}
impl RoundWeightNorm for NullDipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234])
    }
}
impl RoundWeightNorm for NullSphereAtOrigin {
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e1234])
    }
}
impl RoundWeightNorm for NullVersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn round_weight_norm(self) -> AntiScalar {
        let wedge_g0 = self.group0().wxyz();
        AntiScalar::from_groups(
            // e12345
            wedge_g0[0] * wedge_g0[0] + wedge_g0[1] * wedge_g0[1] + wedge_g0[2] * wedge_g0[2] + wedge_g0[3] * wedge_g0[3],
        )
    }
}
impl RoundWeightNorm for Origin {
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e4])
    }
}
impl RoundWeightNorm for RoundPoint {
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e4])
    }
}
impl RoundWeightNorm for RoundPointAtOrigin {
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e4])
    }
}
impl RoundWeightNorm for Sphere {
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e1234])
    }
}
impl RoundWeightNorm for SphereAtOrigin {
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e1234])
    }
}
impl RoundWeightNorm for SphereOnOrigin {
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e1234])
    }
}
impl RoundWeightNorm for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4])
    }
}
impl RoundWeightNorm for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4])
    }
}
impl RoundWeightNorm for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn round_weight_norm(self) -> AntiScalar {
        let wedge_g0 = self.group0().wxyz();
        AntiScalar::from_groups(
            // e12345
            wedge_g0[0] * wedge_g0[0] + wedge_g0[1] * wedge_g0[1] + wedge_g0[2] * wedge_g0[2] + wedge_g0[3] * wedge_g0[3],
        )
    }
}
impl RoundWeightNorm for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4])
    }
}
impl RoundWeightNorm for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4])
    }
}
impl RoundWeightNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let wedge_g0_xyz = self.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            wedge_g0_xyz[0] * wedge_g0_xyz[0] + wedge_g0_xyz[1] * wedge_g0_xyz[1] + wedge_g0_xyz[2] * wedge_g0_xyz[2] + self[e1234] * self[e1234],
        )
    }
}
impl RoundWeightNorm for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn round_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        let wedge_g0_xyz = self.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            wedge_g0_xyz[0] * wedge_g0_xyz[0] + wedge_g0_xyz[1] * wedge_g0_xyz[1] + wedge_g0_xyz[2] * wedge_g0_xyz[2] + self[e1234] * self[e1234],
        )
    }
}
