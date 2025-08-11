// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 11
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       1       0     N/A
//   Median:         2       3       0     N/A
//  Average:         2       3       0     N/A
//  Maximum:         7       8       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       1       0       0
//   Median:         2       3       0       0
//  Average:         2       3       0       0
//  Maximum:         7       8       0       0
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43])
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4])
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for Circle {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412])
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412])
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for Dipole {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43])
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234])
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        7        8        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e41] * self[e41]
                + self[e42] * self[e42]
                + self[e43] * self[e43]
                + self[e423] * self[e423]
                + self[e431] * self[e431]
                + self[e412] * self[e412]
                + self[e4] * self[e4]
                + self[e1234] * self[e1234],
        )
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for RoundPoint {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e4] * self[e4])
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for Sphere {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e1234] * self[e1234])
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        let wedge_g1_xyz = self.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            wedge_g1_xyz[0] * wedge_g1_xyz[0] + wedge_g1_xyz[1] * wedge_g1_xyz[1] + wedge_g1_xyz[2] * wedge_g1_xyz[2] + self[e4] * self[e4],
        )
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type_g0 = self.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2] + self[e1234] * self[e1234],
        )
    }
}
