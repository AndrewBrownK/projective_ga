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
//   Median:         2       3       0     N/A
//  Average:         1       3       0     N/A
//  Maximum:         7       8       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         2       3       0       0
//  Average:         1       3       0       0
//  Maximum:         7       8       0       0
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        let wedge_g0 = self.group2().xyz();
        Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2])
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
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        7        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge_g1_xyz = self.group2().xyz() * Simd32x3::from(-1.0);
        Scalar::from_groups(
            // scalar
            self[e5] * self[e5] - wedge_g1_xyz[0] * wedge_g1_xyz[0] - wedge_g1_xyz[1] * wedge_g1_xyz[1] - wedge_g1_xyz[2] * wedge_g1_xyz[2],
        )
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for AntiDualNum {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e3215] * -1.0)
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
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn flat_bulk_norm(self) -> Scalar {
        let wedge_g0_xyz = self.group0().xyz() * Simd32x3::from(-1.0);
        Scalar::from_groups(
            // scalar
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2],
        )
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
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        7        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge_g1_xyz = self.group0().xyz() * Simd32x3::from(-1.0);
        Scalar::from_groups(
            // scalar
            self[e5] * self[e5] - wedge_g1_xyz[0] * wedge_g1_xyz[0] - wedge_g1_xyz[1] * wedge_g1_xyz[1] - wedge_g1_xyz[2] * wedge_g1_xyz[2],
        )
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
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35])
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
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        let wedge_g0_xyz = self.group1().xyz();
        Scalar::from_groups(
            // scalar
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2],
        )
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
        Scalar::from_groups(/* scalar */ self[e5])
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
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125])
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
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn flat_bulk_norm(self) -> Scalar {
        let wedge_g0_xyz = self.group2().xyz() * Simd32x3::from(-1.0);
        Scalar::from_groups(
            // scalar
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2],
        )
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
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35])
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
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        let wedge_g0_xyz = self.group2().xyz();
        Scalar::from_groups(
            // scalar
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2],
        )
    }
}
impl std::ops::Div<FlatBulkNormPrefixOrPostfix> for DualNum {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm()
    }
}
impl FlatBulkNorm for DualNum {
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e5])
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
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        let wedge_g0 = self.group0().xyz();
        Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2])
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
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        let wedge_g0_xyz = self.group0().xyz();
        Scalar::from_groups(
            // scalar
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2],
        )
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
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125])
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
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        7        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge_g1_xyz = self.group1().xyz() * Simd32x3::from(-1.0);
        Scalar::from_groups(
            // scalar
            self[e5] * self[e5] - wedge_g1_xyz[0] * wedge_g1_xyz[0] - wedge_g1_xyz[1] * wedge_g1_xyz[1] - wedge_g1_xyz[2] * wedge_g1_xyz[2],
        )
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
    //      add/sub      mul      div      pow
    // f32        7        8        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge_g6_xyz = self.group3().xyz();
        Scalar::from_groups(
            // scalar
            self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125] + self[e5] * self[e5]
                - wedge_g6_xyz[0] * wedge_g6_xyz[0]
                - wedge_g6_xyz[1] * wedge_g6_xyz[1]
                - wedge_g6_xyz[2] * wedge_g6_xyz[2]
                - self[e3215] * self[e3215],
        )
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
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e3215] * -1.0)
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
        Scalar::from_groups(/* scalar */ self[e5])
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
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e3215] * -1.0)
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
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        7        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge_g1_xyz = self.group2().xyz() * Simd32x3::from(-1.0);
        Scalar::from_groups(
            // scalar
            self[e5] * self[e5] - wedge_g1_xyz[0] * wedge_g1_xyz[0] - wedge_g1_xyz[1] * wedge_g1_xyz[1] - wedge_g1_xyz[2] * wedge_g1_xyz[2],
        )
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
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        let wedge_g0_xyz = self.group2().xyz();
        Scalar::from_groups(
            // scalar
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2],
        )
    }
}
