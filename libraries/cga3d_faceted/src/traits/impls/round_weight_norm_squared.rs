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
//  Maximum:        23      25       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       0       0
//  Average:         2       0       0
//  Maximum:        23      33       0
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
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2));
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
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2));
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
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2));
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
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2));
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
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0().with_w(self[e4]).wxyz());
        return AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e45], 2) + f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2),
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
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e45], 2) + f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2),
        );
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
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e45], 2) + f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2),
        );
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
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2));
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
        return AntiScalar::from_groups(/* e12345 */ f32::powi(self[e1234], 2));
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
        return AntiScalar::from_groups(/* e12345 */ f32::powi(self[e4], 2));
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
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2),
        );
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
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2));
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
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2));
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
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2));
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
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2));
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
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2));
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
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2));
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
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2));
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
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2));
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
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2));
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
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2));
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
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2));
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
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0().with_w(self[e1234]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2),
        );
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
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2),
        );
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
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2),
        );
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
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2),
        );
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
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2),
        );
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
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2));
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
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2));
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
        return AntiScalar::from_groups(/* e12345 */ f32::powi(self[e4], 2));
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
    //      f32       23       21        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       23       25        0
    //  no simd       23       33        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        let sub_type = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group3().xyz().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        let other = Infinity::from_groups(/* e5 */ 1.0);
        let wedge = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e5] * sub_type[e1234]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(other[e5] * sub_type[e4]),
            // e15, e25, e35
            Simd32x3::from(other[e5]) * sub_type.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (Simd32x3::from(other[e5]) * sub_type.group3().xyz()).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e5] * sub_type[e423], other[e5] * sub_type[e431], other[e5] * sub_type[e412]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]),
            // e3215
            0.0,
        );
        return AntiScalar::from_groups(
            // e12345
            2.0 * (wedge[e4] * wedge[e5])
                + 2.0 * (wedge[e423] * wedge[e235])
                + 2.0 * (wedge[e431] * wedge[e315])
                + 2.0 * (wedge[e412] * wedge[e125])
                + f32::powi(wedge[e12345], 2)
                + f32::powi(wedge[e45], 2)
                + f32::powi(wedge[e415], 2)
                + f32::powi(wedge[e425], 2)
                + f32::powi(wedge[e435], 2)
                + f32::powi(wedge[e4235], 2)
                + f32::powi(wedge[e4315], 2)
                + f32::powi(wedge[e4125], 2)
                - f32::powi(wedge[scalar], 2)
                - f32::powi(wedge[e1], 2)
                - f32::powi(wedge[e2], 2)
                - f32::powi(wedge[e3], 2)
                - f32::powi(wedge[e23], 2)
                - f32::powi(wedge[e31], 2)
                - f32::powi(wedge[e12], 2)
                - f32::powi(wedge[e321], 2)
                - 2.0 * (wedge[e41] * wedge[e15])
                - 2.0 * (wedge[e42] * wedge[e25])
                - 2.0 * (wedge[e43] * wedge[e35])
                - 2.0 * (wedge[e1234] * wedge[e3215]),
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
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2));
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
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return AntiScalar::from_groups(/* e12345 */ f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2));
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
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0());
        return AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2),
        );
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
        return AntiScalar::from_groups(/* e12345 */ f32::powi(self[e1234], 2));
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
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0().wxyz());
        return AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e45], 2) + f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2),
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
        return AntiScalar::from_groups(/* e12345 */ f32::powi(self[e4], 2));
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
        return AntiScalar::from_groups(/* e12345 */ f32::powi(self[e4], 2));
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
        return AntiScalar::from_groups(/* e12345 */ f32::powi(self[e4], 2));
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
        return AntiScalar::from_groups(/* e12345 */ f32::powi(self[e1234], 2));
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
        return AntiScalar::from_groups(/* e12345 */ f32::powi(self[e1234], 2));
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
        return AntiScalar::from_groups(/* e12345 */ f32::powi(self[e1234], 2));
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
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e45], 2) + f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2),
        );
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
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e45], 2) + f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2),
        );
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
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0().wxyz());
        return AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e45], 2) + f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2),
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
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e45], 2) + f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2),
        );
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
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e45], 2) + f32::powi(wedge[e4235], 2) + f32::powi(wedge[e4315], 2) + f32::powi(wedge[e4125], 2),
        );
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
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2),
        );
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
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]));
        return AntiScalar::from_groups(
            // e12345
            f32::powi(wedge[e415], 2) + f32::powi(wedge[e425], 2) + f32::powi(wedge[e435], 2) + f32::powi(wedge[e12345], 2),
        );
    }
}
