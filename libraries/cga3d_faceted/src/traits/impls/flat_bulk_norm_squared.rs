// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 55
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       1       0
//  Average:         2       0       0
//  Maximum:         8       2       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       1       0
//  Average:         2       1       0
//  Maximum:         8       6       0
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([self[e5], self[e235], self[e315], self[e125]]) * Simd32x4::from(-1.0);
        return Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        );
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let wedge_g0 = self.group1().with_w(self[e5]).wxyz() * Simd32x4::from(-1.0);
        return Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        );
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([self[e5], self[e235], self[e315], self[e125]]) * Simd32x4::from(-1.0);
        return Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        );
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for AntiFlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group0().xyz() * Simd32x3::from(-1.0);
        return Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for AntiFlector {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([self[e5], self[e235], self[e315], self[e125]]) * Simd32x4::from(-1.0);
        return Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        );
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for AntiLine {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for AntiMotor {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35] - self[e3215] * self[e3215]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for AntiPlane {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiPlane {
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e5] * self[e5]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for Circle {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group2() * Simd32x3::from(-1.0);
        return Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for CircleAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group2() * Simd32x3::from(-1.0);
        return Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for CircleAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group1() * Simd32x3::from(-1.0);
        return Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for CircleAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group1() * Simd32x3::from(-1.0);
        return Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group1() * Simd32x3::from(-1.0);
        return Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group2().xyz() * Simd32x3::from(-1.0);
        return Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group2().xyz() * Simd32x3::from(-1.0);
        return Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group1().xyz() * Simd32x3::from(-1.0);
        return Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group1().xyz() * Simd32x3::from(-1.0);
        return Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for Dipole {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for DipoleAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for DipoleAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35] - self[e3215] * self[e3215]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35] - self[e3215] * self[e3215]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35] - self[e3215] * self[e3215]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -self[e3215] * self[e3215] - self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -self[e3215] * self[e3215] - self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for FlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for FlatPointAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for FlatPointAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for Flector {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35] - self[e3215] * self[e3215]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for FlectorAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for FlectorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35] - self[e3215] * self[e3215]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for Horizon {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e3215] * self[e3215] * -1.0);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for Infinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Infinity {
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e5] * self[e5]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for Line {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group1() * Simd32x3::from(-1.0);
        return Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for LineAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for LineAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group0() * Simd32x3::from(-1.0);
        return Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for Motor {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group1().wxyz() * Simd32x4::from(-1.0);
        return Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        );
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for MotorAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for MotorAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group0().wxyz() * Simd32x4::from(-1.0);
        return Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        );
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        0        0
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8        2        0
    //  no simd        8        6        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let wedge_g0 = Simd32x2::from([1.0, self[e3215]]) * Simd32x2::from([0.0, 1.0]);
        let wedge_g9 = Simd32x4::from([0.0, self[e235], self[e315], self[e125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]);
        return Scalar::from_groups(
            // scalar
            wedge_g0[0] * wedge_g0[0] + self[e5] * self[e5]
                - wedge_g0[1] * wedge_g0[1]
                - wedge_g9[1] * wedge_g9[1]
                - wedge_g9[2] * wedge_g9[2]
                - wedge_g9[3] * wedge_g9[3]
                - self[e15] * self[e15]
                - self[e25] * self[e25]
                - self[e35] * self[e35],
        );
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for Plane {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e3215] * self[e3215] * -1.0);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for RoundPoint {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for RoundPoint {
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e5] * self[e5]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for RoundPointAtOrigin {
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e5] * self[e5]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for Sphere {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e3215] * self[e3215] * -1.0);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for SphereAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e3215] * self[e3215] * -1.0);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group2().wxyz() * Simd32x4::from(-1.0);
        return Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        );
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group2().wxyz() * Simd32x4::from(-1.0);
        return Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        );
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group2().wxyz() * Simd32x4::from(-1.0);
        return Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        );
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group1().wxyz() * Simd32x4::from(-1.0);
        return Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        );
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group1().wxyz() * Simd32x4::from(-1.0);
        return Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        );
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35] - self[e3215] * self[e3215]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35] - self[e3215] * self[e3215]);
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -self[e3215] * self[e3215] - self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35]);
    }
}
