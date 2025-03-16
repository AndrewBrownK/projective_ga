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
impl FlatWeightNorm for AntiCircleRotor {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e45])
    }
}
impl FlatWeightNorm for AntiCircleRotorAtInfinity {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e45])
    }
}
impl FlatWeightNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435])
    }
}
impl FlatWeightNorm for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435])
    }
}
impl FlatWeightNorm for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435])
    }
}
impl FlatWeightNorm for AntiMysteryCircleRotor {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e45])
    }
}
impl FlatWeightNorm for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435])
    }
}
impl FlatWeightNorm for AntiScalar {
    fn flat_weight_norm(self) -> AntiScalar {
        self
    }
}
impl FlatWeightNorm for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435])
    }
}
impl FlatWeightNorm for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435])
    }
}
impl FlatWeightNorm for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435])
    }
}
impl FlatWeightNorm for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435])
    }
}
impl FlatWeightNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
        )
    }
}
impl FlatWeightNorm for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
        )
    }
}
impl FlatWeightNorm for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
        )
    }
}
impl FlatWeightNorm for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
        )
    }
}
impl FlatWeightNorm for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435],
        )
    }
}
impl FlatWeightNorm for Dipole {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e45])
    }
}
impl FlatWeightNorm for DipoleAligningOrigin {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e45])
    }
}
impl FlatWeightNorm for DipoleAtInfinity {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e45])
    }
}
impl FlatWeightNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        )
    }
}
impl FlatWeightNorm for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        )
    }
}
impl FlatWeightNorm for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        )
    }
}
impl FlatWeightNorm for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        )
    }
}
impl FlatWeightNorm for DipoleOnOrigin {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e45])
    }
}
impl FlatWeightNorm for DualNum {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e12345])
    }
}
impl FlatWeightNorm for FlatOrigin {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e45])
    }
}
impl FlatWeightNorm for FlatPoint {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e45])
    }
}
impl FlatWeightNorm for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        )
    }
}
impl FlatWeightNorm for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        )
    }
}
impl FlatWeightNorm for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435])
    }
}
impl FlatWeightNorm for LineOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435])
    }
}
impl FlatWeightNorm for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
        )
    }
}
impl FlatWeightNorm for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
        )
    }
}
impl FlatWeightNorm for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e12345] * self[e12345]
                + self[e45] * self[e45]
                + self[e415] * self[e415]
                + self[e425] * self[e425]
                + self[e435] * self[e435]
                + self[e4235] * self[e4235]
                + self[e4315] * self[e4315]
                + self[e4125] * self[e4125],
        )
    }
}
impl FlatWeightNorm for MysteryCircle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435])
    }
}
impl FlatWeightNorm for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
        )
    }
}
impl FlatWeightNorm for MysteryDipole {
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e45])
    }
}
impl FlatWeightNorm for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        )
    }
}
impl FlatWeightNorm for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435],
        )
    }
}
impl FlatWeightNorm for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125] + self[e45] * self[e45],
        )
    }
}
impl FlatWeightNorm for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125])
    }
}
impl FlatWeightNorm for PlaneOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125])
    }
}
impl FlatWeightNorm for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125])
    }
}
impl FlatWeightNorm for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125])
    }
}
impl FlatWeightNorm for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435],
        )
    }
}
impl FlatWeightNorm for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435],
        )
    }
}
impl FlatWeightNorm for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435],
        )
    }
}
impl FlatWeightNorm for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435],
        )
    }
}
impl FlatWeightNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        )
    }
}
impl FlatWeightNorm for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        )
    }
}
