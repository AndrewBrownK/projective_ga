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
impl RoundBulkNorm for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar])
    }
}
impl RoundBulkNorm for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar])
    }
}
impl RoundBulkNorm for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar])
    }
}
impl RoundBulkNorm for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar])
    }
}
impl RoundBulkNorm for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * self[scalar] + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RoundBulkNorm for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RoundBulkNorm for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RoundBulkNorm for AntiDipoleOnOrigin {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321])
    }
}
impl RoundBulkNorm for AntiDualNum {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar])
    }
}
impl RoundBulkNorm for AntiFlatOrigin {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321])
    }
}
impl RoundBulkNorm for AntiFlatPoint {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321])
    }
}
impl RoundBulkNorm for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RoundBulkNorm for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RoundBulkNorm for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNorm for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNorm for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar])
    }
}
impl RoundBulkNorm for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar])
    }
}
impl RoundBulkNorm for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar])
    }
}
impl RoundBulkNorm for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RoundBulkNorm for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RoundBulkNorm for AntiPlaneOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RoundBulkNorm for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RoundBulkNorm for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * self[scalar] + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNorm for Circle {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321])
    }
}
impl RoundBulkNorm for CircleAtInfinity {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321])
    }
}
impl RoundBulkNorm for CircleOrthogonalOrigin {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321])
    }
}
impl RoundBulkNorm for CircleRotor {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321])
    }
}
impl RoundBulkNorm for CircleRotorAtInfinity {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321])
    }
}
impl RoundBulkNorm for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNorm for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNorm for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNorm for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNorm for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNorm for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn round_bulk_norm(self) -> Scalar {
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
impl RoundBulkNorm for MysteryCircle {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321])
    }
}
impl RoundBulkNorm for MysteryCircleRotor {
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321])
    }
}
impl RoundBulkNorm for MysteryDipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNorm for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNorm for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] + self[e321] * self[e321])
    }
}
impl RoundBulkNorm for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * self[scalar] + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RoundBulkNorm for Scalar {
    fn round_bulk_norm(self) -> Scalar {
        self
    }
}
impl RoundBulkNorm for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RoundBulkNorm for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] + self[e321] * self[e321])
    }
}
impl RoundBulkNorm for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3])
    }
}
impl RoundBulkNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * self[scalar] + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNorm for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * self[scalar] + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
impl RoundBulkNorm for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn round_bulk_norm(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * self[scalar] + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12])
    }
}
