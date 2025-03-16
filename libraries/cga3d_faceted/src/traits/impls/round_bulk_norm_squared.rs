// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 51
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       0       0
//  Average:         2       0       0
//  Maximum:         7       0       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       0       0
//  Average:         2       0       0
//  Maximum:         7       0       0
impl RoundBulkNormSquared for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar])
    }
}
impl RoundBulkNormSquared for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar])
    }
}
impl RoundBulkNormSquared for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar])
    }
}
impl RoundBulkNormSquared for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar])
    }
}
impl RoundBulkNormSquared for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * self[scalar] + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RoundBulkNormSquared for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RoundBulkNormSquared for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RoundBulkNormSquared for AntiDipoleOnOrigin {
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321])
    }
}
impl RoundBulkNormSquared for AntiDualNum {
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * self[scalar])
    }
}
impl RoundBulkNormSquared for AntiFlatOrigin {
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321])
    }
}
impl RoundBulkNormSquared for AntiFlatPoint {
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321])
    }
}
impl RoundBulkNormSquared for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RoundBulkNormSquared for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RoundBulkNormSquared for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNormSquared for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNormSquared for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar])
    }
}
impl RoundBulkNormSquared for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar])
    }
}
impl RoundBulkNormSquared for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar])
    }
}
impl RoundBulkNormSquared for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RoundBulkNormSquared for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RoundBulkNormSquared for AntiPlaneOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RoundBulkNormSquared for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RoundBulkNormSquared for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * self[scalar] + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNormSquared for Circle {
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321])
    }
}
impl RoundBulkNormSquared for CircleAtInfinity {
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321])
    }
}
impl RoundBulkNormSquared for CircleOrthogonalOrigin {
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321])
    }
}
impl RoundBulkNormSquared for CircleRotor {
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321])
    }
}
impl RoundBulkNormSquared for CircleRotorAtInfinity {
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321])
    }
}
impl RoundBulkNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNormSquared for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNormSquared for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNormSquared for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNormSquared for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[scalar] * self[scalar]
                + self[e1] * self[e1]
                + self[e2] * self[e2]
                + self[e3] * self[e3]
                + self[e23] * self[e23]
                + self[e31] * self[e31]
                + self[e12] * self[e12]
                + self[e321] * self[e321],
        )
    }
}
impl RoundBulkNormSquared for MysteryCircle {
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321])
    }
}
impl RoundBulkNormSquared for MysteryCircleRotor {
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321])
    }
}
impl RoundBulkNormSquared for MysteryDipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNormSquared for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNormSquared for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] + self[e321] * self[e321])
    }
}
impl RoundBulkNormSquared for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * self[scalar] + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNormSquared for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RoundBulkNormSquared for Scalar {
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * self[scalar])
    }
}
impl RoundBulkNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RoundBulkNormSquared for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] + self[e321] * self[e321])
    }
}
impl RoundBulkNormSquared for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RoundBulkNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * self[scalar] + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNormSquared for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * self[scalar] + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNormSquared for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * self[scalar] + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
