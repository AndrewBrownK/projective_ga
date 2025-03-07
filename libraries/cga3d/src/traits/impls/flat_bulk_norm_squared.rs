// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 23
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       1       0
//  Average:         2       0       0
//  Maximum:         7       1       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       3       0
//  Average:         2       2       0
//  Maximum:         7       4       0
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
        let wedge_g0 = self.group2().xyz();
        return Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2]);
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
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        3        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let wedge_g1_xyz = self.group2().xyz() * Simd32x3::from(-1.0);
        return Scalar::from_groups(
            // scalar
            self[e5] * self[e5] - wedge_g1_xyz[0] * wedge_g1_xyz[0] - wedge_g1_xyz[1] * wedge_g1_xyz[1] - wedge_g1_xyz[2] * wedge_g1_xyz[2],
        );
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for AntiDualNum {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e3215] * self[e3215] * -1.0);
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
        let wedge_g0_xyz = self.group0().xyz() * Simd32x3::from(-1.0);
        return Scalar::from_groups(
            // scalar
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2],
        );
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
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        3        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let wedge_g1_xyz = self.group0().xyz() * Simd32x3::from(-1.0);
        return Scalar::from_groups(
            // scalar
            self[e5] * self[e5] - wedge_g1_xyz[0] * wedge_g1_xyz[0] - wedge_g1_xyz[1] * wedge_g1_xyz[1] - wedge_g1_xyz[2] * wedge_g1_xyz[2],
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
        let wedge_g0 = self.group1();
        return Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2]);
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
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        7        4        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let wedge_g0 = (Simd32x3::from(0.0).with_w(1.0).wwwx() * self.group1().xyz().with_w(0.0)) + Simd32x3::from(0.0).with_w(self[e3215]);
        return Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        );
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
        let wedge_g0_xyz = self.group2() * Simd32x3::from(-1.0);
        return Scalar::from_groups(
            // scalar
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2],
        );
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
        let wedge_g0_xyz = self.group2().xyz() * Simd32x3::from(-1.0);
        return Scalar::from_groups(
            // scalar
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2],
        );
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
        let wedge_g0 = self.group2();
        return Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2]);
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
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        7        4        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let wedge_g0 = (Simd32x3::from(0.0).with_w(1.0).wwwx() * self.group2().xyz().with_w(0.0)) + Simd32x3::from(0.0).with_w(self[e3215]);
        return Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        );
    }
}
impl std::ops::Div<FlatBulkNormSquaredPrefixOrPostfix> for DualNum {
    type Output = Scalar;
    fn div(self, _rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_bulk_norm_squared()
    }
}
impl FlatBulkNormSquared for DualNum {
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e5] * self[e5]);
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
        let wedge_g0 = self.group0().xyz();
        return Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2]);
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
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        7        4        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let wedge_g0 = (Simd32x3::from(0.0).with_w(1.0).wwwx() * self.group0().xyz().with_w(0.0)) + Simd32x3::from(0.0).with_w(self[e3215]);
        return Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        );
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
        let wedge_g0_xyz = self.group1() * Simd32x3::from(-1.0);
        return Scalar::from_groups(
            // scalar
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2],
        );
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
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        3        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let wedge_g1_xyz = self.group1().xyz() * Simd32x3::from(-1.0);
        return Scalar::from_groups(
            // scalar
            self[e5] * self[e5] - wedge_g1_xyz[0] * wedge_g1_xyz[0] - wedge_g1_xyz[1] * wedge_g1_xyz[1] - wedge_g1_xyz[2] * wedge_g1_xyz[2],
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
    //      f32        7        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        1        0
    //  no simd        7        4        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let wedge_g9 = self.group8().with_w(0.0) * Simd32x4::from(-1.0);
        return Scalar::from_groups(
            // scalar
            self[e5] * self[e5]
                - wedge_g9[0] * wedge_g9[0]
                - wedge_g9[1] * wedge_g9[1]
                - wedge_g9[2] * wedge_g9[2]
                - self[e15] * self[e15]
                - self[e25] * self[e25]
                - self[e35] * self[e35]
                - self[e3215] * self[e3215],
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
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        3        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let wedge_g1_xyz = self.group2().xyz() * Simd32x3::from(-1.0);
        return Scalar::from_groups(
            // scalar
            self[e5] * self[e5] - wedge_g1_xyz[0] * wedge_g1_xyz[0] - wedge_g1_xyz[1] * wedge_g1_xyz[1] - wedge_g1_xyz[2] * wedge_g1_xyz[2],
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
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        7        4        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let wedge_g0 = (Simd32x3::from(0.0).with_w(1.0).wwwx() * self.group2().xyz().with_w(0.0)) + Simd32x3::from(0.0).with_w(self[e3215]);
        return Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        );
    }
}
