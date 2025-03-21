// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 55
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       1       0     N/A
//   Median:         2       4       0     N/A
//  Average:         2       3       0     N/A
//  Maximum:         7       8       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       1       0       0
//   Median:         2       4       0       0
//  Average:         2       4       0       0
//  Maximum:         7       8       0       0
impl FlatBulkNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group2().xyz();
        Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2])
    }
}
impl FlatBulkNormSquared for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group2().xyz();
        Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2])
    }
}
impl FlatBulkNormSquared for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group1().xyz();
        Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2])
    }
}
impl FlatBulkNormSquared for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group1().xyz();
        Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2])
    }
}
impl FlatBulkNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125] + self[e5] * self[e5])
    }
}
impl FlatBulkNormSquared for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125] + self[e5] * self[e5])
    }
}
impl FlatBulkNormSquared for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e5] * self[e5] + self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125])
    }
}
impl FlatBulkNormSquared for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group0().xyz() * Simd32x3::from(-1.0);
        Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2])
    }
}
impl FlatBulkNormSquared for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125] + self[e5] * self[e5])
    }
}
impl FlatBulkNormSquared for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35])
    }
}
impl FlatBulkNormSquared for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35] - self[e3215] * self[e3215])
    }
}
impl FlatBulkNormSquared for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e5] * self[e5])
    }
}
impl FlatBulkNormSquared for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125])
    }
}
impl FlatBulkNormSquared for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125])
    }
}
impl FlatBulkNormSquared for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125])
    }
}
impl FlatBulkNormSquared for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125])
    }
}
impl FlatBulkNormSquared for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125])
    }
}
impl FlatBulkNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group2().xyz() * Simd32x3::from(-1.0);
        Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2])
    }
}
impl FlatBulkNormSquared for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group2().xyz() * Simd32x3::from(-1.0);
        Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2])
    }
}
impl FlatBulkNormSquared for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group1().xyz() * Simd32x3::from(-1.0);
        Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2])
    }
}
impl FlatBulkNormSquared for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group1().xyz() * Simd32x3::from(-1.0);
        Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2])
    }
}
impl FlatBulkNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35])
    }
}
impl FlatBulkNormSquared for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35])
    }
}
impl FlatBulkNormSquared for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35])
    }
}
impl FlatBulkNormSquared for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35])
    }
}
impl FlatBulkNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let wedge_g0_xyz = self.group2().xyz();
        Scalar::from_groups(
            // scalar
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2] - self[e3215] * self[e3215],
        )
    }
}
impl FlatBulkNormSquared for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let wedge_g0_xyz = self.group1().xyz();
        Scalar::from_groups(
            // scalar
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2] - self[e3215] * self[e3215],
        )
    }
}
impl FlatBulkNormSquared for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35] - self[e3215] * self[e3215])
    }
}
impl FlatBulkNormSquared for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e3215] * self[e3215] - self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35])
    }
}
impl FlatBulkNormSquared for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let wedge_g0_xyz = self.group2().xyz();
        Scalar::from_groups(
            // scalar
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2] - self[e3215] * self[e3215],
        )
    }
}
impl FlatBulkNormSquared for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35])
    }
}
impl FlatBulkNormSquared for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group0().xyz();
        Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2])
    }
}
impl FlatBulkNormSquared for FlatPointAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35])
    }
}
impl FlatBulkNormSquared for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let wedge_g0_xyz = self.group0().xyz();
        Scalar::from_groups(
            // scalar
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2] - self[e3215] * self[e3215],
        )
    }
}
impl FlatBulkNormSquared for FlectorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35] - self[e3215] * self[e3215])
    }
}
impl FlatBulkNormSquared for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e3215] * self[e3215] * -1.0)
    }
}
impl FlatBulkNormSquared for Infinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e5] * self[e5])
    }
}
impl FlatBulkNormSquared for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125])
    }
}
impl FlatBulkNormSquared for LineAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125])
    }
}
impl FlatBulkNormSquared for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group1().wxyz() * Simd32x4::from(-1.0);
        Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        )
    }
}
impl FlatBulkNormSquared for MotorAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group0().wxyz() * Simd32x4::from(-1.0);
        Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        )
    }
}
impl FlatBulkNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        7        8        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e5] * self[e5] + self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125]
                - self[e15] * self[e15]
                - self[e25] * self[e25]
                - self[e35] * self[e35]
                - self[e3215] * self[e3215],
        )
    }
}
impl FlatBulkNormSquared for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e3215] * self[e3215] * -1.0)
    }
}
impl FlatBulkNormSquared for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e5] * self[e5])
    }
}
impl FlatBulkNormSquared for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e5] * self[e5])
    }
}
impl FlatBulkNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e3215] * self[e3215] * -1.0)
    }
}
impl FlatBulkNormSquared for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e3215] * self[e3215] * -1.0)
    }
}
impl FlatBulkNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group2().wxyz() * Simd32x4::from(-1.0);
        Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        )
    }
}
impl FlatBulkNormSquared for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group2().wxyz() * Simd32x4::from(-1.0);
        Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        )
    }
}
impl FlatBulkNormSquared for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group2().wxyz() * Simd32x4::from(-1.0);
        Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        )
    }
}
impl FlatBulkNormSquared for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group1().wxyz() * Simd32x4::from(-1.0);
        Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        )
    }
}
impl FlatBulkNormSquared for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        let wedge_g0 = self.group1().wxyz() * Simd32x4::from(-1.0);
        Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        )
    }
}
impl FlatBulkNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let wedge_g0_xyz = self.group2().xyz();
        Scalar::from_groups(
            // scalar
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2] - self[e3215] * self[e3215],
        )
    }
}
impl FlatBulkNormSquared for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let wedge_g0_xyz = self.group0().yzw();
        Scalar::from_groups(
            // scalar
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2] - self[e3215] * self[e3215],
        )
    }
}
impl FlatBulkNormSquared for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e3215] * self[e3215] - self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35])
    }
}
