// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 49
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       0       0
//  Average:         2       0       0
//  Maximum:         8       2       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       0       0
//  Average:         2       0       0
//  Maximum:         8       6       0
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43]);
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
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        let wedge_g0 = self.group0().with_w(self[e4]).wxyz();
        return AntiScalar::from_groups(
            // e12345
            wedge_g0[0] * wedge_g0[0] + wedge_g0[1] * wedge_g0[1] + wedge_g0[2] * wedge_g0[2] + wedge_g0[3] * wedge_g0[3],
        );
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiDualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiDualNum {
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e1234] * self[e1234]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiSphereOnOrigin {
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e4] * self[e4]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234]);
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
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for CircleAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for CircleAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for CircleOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412]);
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
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412]);
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
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for DipoleAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43]);
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
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for DipoleOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for DualNum {
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e4] * self[e4]);
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
    //           add/sub      mul      div
    //      f32        8        0        0
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8        2        0
    //  no simd        8        6        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        let wedge_g0 = Simd32x2::from([1.0, self[e1234]]) * Simd32x2::from([0.0, 1.0]);
        let wedge_g9 = Simd32x4::from([0.0, self[e423], self[e431], self[e412]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]);
        return AntiScalar::from_groups(
            // e12345
            wedge_g0[1] * wedge_g0[1]
                + wedge_g9[1] * wedge_g9[1]
                + wedge_g9[2] * wedge_g9[2]
                + wedge_g9[3] * wedge_g9[3]
                + self[e4] * self[e4]
                + self[e41] * self[e41]
                + self[e42] * self[e42]
                + self[e43] * self[e43]
                - wedge_g0[0] * wedge_g0[0],
        );
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for NullCircleAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for NullCircleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for NullDipoleAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for NullDipoleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for NullDipoleInversionAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for NullDipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for NullSphereAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for NullSphereAtOrigin {
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e1234] * self[e1234]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for NullVersorEvenAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for NullVersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let wedge_g0 = self.group0().wxyz();
        return AntiScalar::from_groups(
            // e12345
            wedge_g0[0] * wedge_g0[0] + wedge_g0[1] * wedge_g0[1] + wedge_g0[2] * wedge_g0[2] + wedge_g0[3] * wedge_g0[3],
        );
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for Origin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for Origin {
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e4] * self[e4]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for RoundPoint {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for RoundPoint {
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e4] * self[e4]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for RoundPointAtOrigin {
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e4] * self[e4]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for Sphere {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for Sphere {
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e1234] * self[e1234]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for SphereAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for SphereAtOrigin {
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e1234] * self[e1234]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for SphereOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for SphereOnOrigin {
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e1234] * self[e1234]);
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
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let wedge_g0 = self.group0().wxyz();
        return AntiScalar::from_groups(
            // e12345
            wedge_g0[0] * wedge_g0[0] + wedge_g0[1] * wedge_g0[1] + wedge_g0[2] * wedge_g0[2] + wedge_g0[3] * wedge_g0[3],
        );
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4]);
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
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234]);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234]);
    }
}
