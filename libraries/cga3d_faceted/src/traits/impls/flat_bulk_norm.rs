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
//  Minimum:         0       0       0     N/A
//   Median:         2       4       0     N/A
//  Average:         2       3       0     N/A
//  Maximum:         7       8       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         2       4       0       0
//  Average:         2       3       0       0
//  Maximum:         7       8       0       0
impl FlatBulkNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        let wedge_g0 = self.group2().xyz();
        Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2])
    }
}
impl FlatBulkNorm for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        let wedge_g0 = self.group2().xyz();
        Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2])
    }
}
impl FlatBulkNorm for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        let wedge_g0 = self.group1().xyz();
        Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2])
    }
}
impl FlatBulkNorm for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        let wedge_g0 = self.group1().xyz();
        Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2])
    }
}
impl FlatBulkNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125] + self[e5] * self[e5])
    }
}
impl FlatBulkNorm for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125] + self[e5] * self[e5])
    }
}
impl FlatBulkNorm for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e5] * self[e5] + self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125])
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
        let wedge_g0 = self.group0().xyz() * Simd32x3::from(-1.0);
        Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2])
    }
}
impl FlatBulkNorm for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125] + self[e5] * self[e5])
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
impl FlatBulkNorm for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35] - self[e3215] * self[e3215])
    }
}
impl FlatBulkNorm for AntiPlane {
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e5])
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
impl FlatBulkNorm for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125])
    }
}
impl FlatBulkNorm for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125])
    }
}
impl FlatBulkNorm for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125])
    }
}
impl FlatBulkNorm for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125])
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
        let wedge_g0 = self.group2().xyz() * Simd32x3::from(-1.0);
        Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2])
    }
}
impl FlatBulkNorm for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn flat_bulk_norm(self) -> Scalar {
        let wedge_g0 = self.group2().xyz() * Simd32x3::from(-1.0);
        Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2])
    }
}
impl FlatBulkNorm for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn flat_bulk_norm(self) -> Scalar {
        let wedge_g0 = self.group1().xyz() * Simd32x3::from(-1.0);
        Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2])
    }
}
impl FlatBulkNorm for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn flat_bulk_norm(self) -> Scalar {
        let wedge_g0 = self.group1().xyz() * Simd32x3::from(-1.0);
        Scalar::from_groups(/* scalar */ -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2])
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
impl FlatBulkNorm for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35])
    }
}
impl FlatBulkNorm for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35])
    }
}
impl FlatBulkNorm for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35])
    }
}
impl FlatBulkNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge_g0_xyz = self.group2().xyz();
        Scalar::from_groups(
            // scalar
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2] - self[e3215] * self[e3215],
        )
    }
}
impl FlatBulkNorm for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge_g0_xyz = self.group1().xyz();
        Scalar::from_groups(
            // scalar
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2] - self[e3215] * self[e3215],
        )
    }
}
impl FlatBulkNorm for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35] - self[e3215] * self[e3215])
    }
}
impl FlatBulkNorm for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e3215] * self[e3215] - self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35])
    }
}
impl FlatBulkNorm for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge_g0_xyz = self.group2().xyz();
        Scalar::from_groups(
            // scalar
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2] - self[e3215] * self[e3215],
        )
    }
}
impl FlatBulkNorm for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35])
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
impl FlatBulkNorm for FlatPointAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35])
    }
}
impl FlatBulkNorm for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge_g0_xyz = self.group0().xyz();
        Scalar::from_groups(
            // scalar
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2] - self[e3215] * self[e3215],
        )
    }
}
impl FlatBulkNorm for FlectorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35] - self[e3215] * self[e3215])
    }
}
impl FlatBulkNorm for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e3215] * -1.0)
    }
}
impl FlatBulkNorm for Infinity {
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e5])
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
impl FlatBulkNorm for LineAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125])
    }
}
impl FlatBulkNorm for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
    fn flat_bulk_norm(self) -> Scalar {
        let wedge_g0 = self.group1().wxyz() * Simd32x4::from(-1.0);
        Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        )
    }
}
impl FlatBulkNorm for MotorAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
    fn flat_bulk_norm(self) -> Scalar {
        let wedge_g0 = self.group0().wxyz() * Simd32x4::from(-1.0);
        Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        )
    }
}
impl FlatBulkNorm for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        7        8        0        0
    fn flat_bulk_norm(self) -> Scalar {
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
impl FlatBulkNorm for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e3215] * -1.0)
    }
}
impl FlatBulkNorm for RoundPoint {
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e5])
    }
}
impl FlatBulkNorm for RoundPointAtOrigin {
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e5])
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
impl FlatBulkNorm for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e3215] * -1.0)
    }
}
impl FlatBulkNorm for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
    fn flat_bulk_norm(self) -> Scalar {
        let wedge_g0 = self.group2().wxyz() * Simd32x4::from(-1.0);
        Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        )
    }
}
impl FlatBulkNorm for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
    fn flat_bulk_norm(self) -> Scalar {
        let wedge_g0 = self.group2().wxyz() * Simd32x4::from(-1.0);
        Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        )
    }
}
impl FlatBulkNorm for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
    fn flat_bulk_norm(self) -> Scalar {
        let wedge_g0 = self.group2().wxyz() * Simd32x4::from(-1.0);
        Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        )
    }
}
impl FlatBulkNorm for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
    fn flat_bulk_norm(self) -> Scalar {
        let wedge_g0 = self.group1().wxyz() * Simd32x4::from(-1.0);
        Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        )
    }
}
impl FlatBulkNorm for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
    fn flat_bulk_norm(self) -> Scalar {
        let wedge_g0 = self.group1().wxyz() * Simd32x4::from(-1.0);
        Scalar::from_groups(
            // scalar
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
        )
    }
}
impl FlatBulkNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge_g0_xyz = self.group2().xyz();
        Scalar::from_groups(
            // scalar
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2] - self[e3215] * self[e3215],
        )
    }
}
impl FlatBulkNorm for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        let wedge_g0_xyz = self.group0().yzw();
        Scalar::from_groups(
            // scalar
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2] - self[e3215] * self[e3215],
        )
    }
}
impl FlatBulkNorm for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn flat_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -self[e3215] * self[e3215] - self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35])
    }
}
