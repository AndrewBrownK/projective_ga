// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 51
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       0       0
//  Average:         2       0       0
//  Maximum:        23      16       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       0       0
//  Average:         2       0       0
//  Maximum:        23      16       0
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group1());
        return Scalar::from_groups(/* scalar */ f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2));
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]));
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2) + f32::powi(sub_type[scalar], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group1().with_w(self[scalar]));
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2) + f32::powi(sub_type[scalar], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group0().with_w(self[scalar]));
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2) + f32::powi(sub_type[scalar], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]));
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2) + f32::powi(sub_type[scalar], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group1().with_w(self[scalar]));
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2) + f32::powi(sub_type[scalar], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]));
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e321], 2) + f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]));
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e321], 2) + f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]));
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e321], 2) + f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiDipoleOnOrigin {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e321]);
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiDualNum {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiDualNum {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[scalar]);
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiFlatOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiFlatOrigin {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e321]);
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiFlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiFlatPoint {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e321]);
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiFlector {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]));
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e321], 2) + f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiFlectorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(self[e321], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2));
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiLine {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0());
        return Scalar::from_groups(/* scalar */ f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2));
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiLineOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2));
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiMotor {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group0());
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2) + f32::powi(sub_type[scalar], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiMotorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) + f32::powi(self[scalar], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]));
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2) + f32::powi(sub_type[scalar], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]));
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e321], 2) + f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiPlane {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0().xyz());
        return Scalar::from_groups(/* scalar */ f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2));
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiPlaneOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiPlaneOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2));
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0().xyz());
        return Scalar::from_groups(/* scalar */ f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2));
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]));
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2) + f32::powi(sub_type[scalar], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for Circle {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for Circle {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e321]);
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for CircleAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for CircleAtInfinity {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e321]);
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for CircleOrthogonalOrigin {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e321]);
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for CircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for CircleRotor {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e321]);
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for CircleRotorAtInfinity {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e321]);
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for Dipole {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group1().xyz());
        return Scalar::from_groups(/* scalar */ f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2));
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for DipoleAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0().xyz());
        return Scalar::from_groups(/* scalar */ f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2));
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for DipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group1().xyz());
        return Scalar::from_groups(/* scalar */ f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2));
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0().xyz());
        return Scalar::from_groups(/* scalar */ f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2));
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group1());
        return Scalar::from_groups(/* scalar */ f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2));
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group1());
        return Scalar::from_groups(/* scalar */ f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2));
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       23       16        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return Scalar::from_groups(
            // scalar
            2.0 * (sub_type[e41] * sub_type[e15])
                + 2.0 * (sub_type[e42] * sub_type[e25])
                + 2.0 * (sub_type[e43] * sub_type[e35])
                + 2.0 * (sub_type[e1234] * sub_type[e3215])
                + f32::powi(sub_type[scalar], 2)
                + f32::powi(sub_type[e1], 2)
                + f32::powi(sub_type[e2], 2)
                + f32::powi(sub_type[e3], 2)
                + f32::powi(sub_type[e23], 2)
                + f32::powi(sub_type[e31], 2)
                + f32::powi(sub_type[e12], 2)
                + f32::powi(sub_type[e321], 2)
                - f32::powi(sub_type[e12345], 2)
                - f32::powi(sub_type[e45], 2)
                - f32::powi(sub_type[e415], 2)
                - f32::powi(sub_type[e425], 2)
                - f32::powi(sub_type[e435], 2)
                - f32::powi(sub_type[e4235], 2)
                - f32::powi(sub_type[e4315], 2)
                - f32::powi(sub_type[e4125], 2)
                - 2.0 * (sub_type[e4] * sub_type[e5])
                - 2.0 * (sub_type[e423] * sub_type[e235])
                - 2.0 * (sub_type[e431] * sub_type[e315])
                - 2.0 * (sub_type[e412] * sub_type[e125]),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for MysteryCircle {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for MysteryCircle {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e321]);
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for MysteryCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for MysteryCircleRotor {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e321]);
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for MysteryDipole {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for MysteryDipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0().xyz());
        return Scalar::from_groups(/* scalar */ f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2));
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for MysteryDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0().xyz());
        return Scalar::from_groups(/* scalar */ f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2));
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for MysteryVersorEven {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]));
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e321], 2) + f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for MysteryVersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]));
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2) + f32::powi(sub_type[scalar], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for RoundPoint {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0().xyz());
        return Scalar::from_groups(/* scalar */ f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2));
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl std::ops::DivAssign<RoundBulkNormPrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: RoundBulkNormPrefixOrPostfix) {
        *self = self.round_bulk_norm()
    }
}
impl RoundBulkNorm for Scalar {
    fn round_bulk_norm(self) -> Scalar {
        return self;
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for VersorEven {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]));
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e321], 2) + f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]));
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e321], 2) + f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([self[e321], self[e1], self[e2], self[e3]]));
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e321], 2) + f32::powi(sub_type[e1], 2) + f32::powi(sub_type[e2], 2) + f32::powi(sub_type[e3], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for VersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]));
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2) + f32::powi(sub_type[scalar], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]));
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2) + f32::powi(sub_type[scalar], 2),
        );
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]));
        return Scalar::from_groups(
            // scalar
            f32::powi(sub_type[e23], 2) + f32::powi(sub_type[e31], 2) + f32::powi(sub_type[e12], 2) + f32::powi(sub_type[scalar], 2),
        );
    }
}
