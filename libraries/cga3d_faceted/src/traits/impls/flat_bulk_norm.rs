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
//  Maximum:        23      25       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       1       0
//  Average:         2       2       0
//  Maximum:        23      31       0
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group2().xyz());
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2));
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group2().xyz());
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2));
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1().xyz());
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2));
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1().xyz());
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2));
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from([self[e5], self[e235], self[e315], self[e125]]) * Simd32x4::from(-1.0),
        );
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e45], 2) - f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group1().with_w(self[e5]).wxyz() * Simd32x4::from(-1.0));
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e45], 2) - f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from([self[e5], self[e235], self[e315], self[e125]]) * Simd32x4::from(-1.0),
        );
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e45], 2) - f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for AntiFlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz() * Simd32x3::from(-1.0));
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2));
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for AntiFlector {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from([self[e5], self[e235], self[e315], self[e125]]) * Simd32x4::from(-1.0),
        );
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e45], 2) - f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for AntiLine {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1());
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2));
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for AntiMotor {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group1());
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2) - f32::powi(wedge[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for AntiPlane {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiPlane {
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e5]);
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for Circle {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group2() * Simd32x3::from(-1.0));
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2));
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for CircleAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group2() * Simd32x3::from(-1.0));
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2));
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for CircleAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group1() * Simd32x3::from(-1.0));
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2));
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for CircleAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group1() * Simd32x3::from(-1.0));
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2));
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group1() * Simd32x3::from(-1.0));
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2));
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for CircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group2().xyz() * Simd32x3::from(-1.0));
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2));
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group2().xyz() * Simd32x3::from(-1.0));
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2));
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group1().xyz() * Simd32x3::from(-1.0));
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2));
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group1().xyz() * Simd32x3::from(-1.0));
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2));
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for Dipole {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group2());
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2));
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1());
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2));
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for DipoleAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1());
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2));
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for DipoleAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1());
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2));
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for DipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]));
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2) - f32::powi(wedge[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]));
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2) - f32::powi(wedge[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group1().with_w(self[e3215]));
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2) - f32::powi(wedge[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]));
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2) - f32::powi(wedge[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]));
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2) - f32::powi(wedge[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group2());
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2));
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for FlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz());
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2));
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for FlatPointAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for FlatPointAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2));
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for Flector {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]));
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2) - f32::powi(wedge[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for FlectorAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for FlectorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0());
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2) - f32::powi(wedge[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for Horizon {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e3215] * -1.0);
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for Infinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for Infinity {
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e5]);
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for Line {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group1() * Simd32x3::from(-1.0));
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2));
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for LineAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for LineAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        3        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0() * Simd32x3::from(-1.0));
        return Scalar::from_groups(/* scalar */ -f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2));
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for Motor {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group1().wxyz() * Simd32x4::from(-1.0));
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e45], 2) - f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for MotorAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for MotorAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0().wxyz() * Simd32x4::from(-1.0));
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e45], 2) - f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       22        0
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       23       25        0
    //  no simd       23       31        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let sub_type = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            self[e3215],
        );
        let other = Origin::from_groups(/* e4 */ 1.0);
        let wedge = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, sub_type[e3215] * other[e4]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(sub_type[e5] * other[e4] * -1.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (Simd32x3::from(other[e4]) * sub_type.group4()).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, sub_type[e235] * other[e4], sub_type[e315] * other[e4], sub_type[e125] * other[e4]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
        return Scalar::from_groups(
            // scalar
            2.0 * (wedge[e41] * wedge[e15])
                + 2.0 * (wedge[e42] * wedge[e25])
                + 2.0 * (wedge[e43] * wedge[e35])
                + 2.0 * (wedge[e1234] * wedge[e3215])
                + f32::powi(wedge[scalar], 2)
                + f32::powi(wedge[e1], 2)
                + f32::powi(wedge[e2], 2)
                + f32::powi(wedge[e3], 2)
                + f32::powi(wedge[e23], 2)
                + f32::powi(wedge[e31], 2)
                + f32::powi(wedge[e12], 2)
                + f32::powi(wedge[e321], 2)
                - f32::powi(wedge[e12345], 2)
                - f32::powi(wedge[e45], 2)
                - f32::powi(wedge[e415], 2)
                - f32::powi(wedge[e425], 2)
                - f32::powi(wedge[e435], 2)
                - f32::powi(wedge[e4235], 2)
                - f32::powi(wedge[e4315], 2)
                - f32::powi(wedge[e4125], 2)
                - 2.0 * (wedge[e4] * wedge[e5])
                - 2.0 * (wedge[e423] * wedge[e235])
                - 2.0 * (wedge[e431] * wedge[e315])
                - 2.0 * (wedge[e412] * wedge[e125]),
        );
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for Plane {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e3215] * -1.0);
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for RoundPoint {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for RoundPoint {
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e5]);
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for RoundPointAtOrigin {
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e5]);
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for Sphere {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e3215] * -1.0);
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for SphereAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e3215] * -1.0);
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for VersorEven {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group2().wxyz() * Simd32x4::from(-1.0));
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e45], 2) - f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group2().wxyz() * Simd32x4::from(-1.0));
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e45], 2) - f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group2().wxyz() * Simd32x4::from(-1.0));
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e45], 2) - f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group1().wxyz() * Simd32x4::from(-1.0));
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e45], 2) - f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group1().wxyz() * Simd32x4::from(-1.0));
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e45], 2) - f32::powi(wedge[e4235], 2) - f32::powi(wedge[e4315], 2) - f32::powi(wedge[e4125], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for VersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]));
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2) - f32::powi(wedge[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]));
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2) - f32::powi(wedge[e12345], 2),
        );
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]));
        return Scalar::from_groups(
            // scalar
            -f32::powi(wedge[e415], 2) - f32::powi(wedge[e425], 2) - f32::powi(wedge[e435], 2) - f32::powi(wedge[e12345], 2),
        );
    }
}
