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
impl FlatWeightNormSquared for AntiCircleRotor {
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e45] * self[e45])
    }
}
impl FlatWeightNormSquared for AntiCircleRotorAtInfinity {
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e45] * self[e45])
    }
}
impl FlatWeightNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435])
    }
}
impl FlatWeightNormSquared for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435])
    }
}
impl FlatWeightNormSquared for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435])
    }
}
impl FlatWeightNormSquared for AntiMysteryCircleRotor {
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e45] * self[e45])
    }
}
impl FlatWeightNormSquared for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435])
    }
}
impl FlatWeightNormSquared for AntiScalar {
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e12345] * self[e12345])
    }
}
impl FlatWeightNormSquared for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435])
    }
}
impl FlatWeightNormSquared for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435])
    }
}
impl FlatWeightNormSquared for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435])
    }
}
impl FlatWeightNormSquared for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435])
    }
}
impl FlatWeightNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
        )
    }
}
impl FlatWeightNormSquared for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
        )
    }
}
impl FlatWeightNormSquared for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
        )
    }
}
impl FlatWeightNormSquared for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
        )
    }
}
impl FlatWeightNormSquared for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435],
        )
    }
}
impl FlatWeightNormSquared for Dipole {
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e45] * self[e45])
    }
}
impl FlatWeightNormSquared for DipoleAligningOrigin {
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e45] * self[e45])
    }
}
impl FlatWeightNormSquared for DipoleAtInfinity {
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e45] * self[e45])
    }
}
impl FlatWeightNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        )
    }
}
impl FlatWeightNormSquared for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        )
    }
}
impl FlatWeightNormSquared for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        )
    }
}
impl FlatWeightNormSquared for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        )
    }
}
impl FlatWeightNormSquared for DipoleOnOrigin {
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e45] * self[e45])
    }
}
impl FlatWeightNormSquared for DualNum {
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e12345] * self[e12345])
    }
}
impl FlatWeightNormSquared for FlatOrigin {
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e45] * self[e45])
    }
}
impl FlatWeightNormSquared for FlatPoint {
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e45] * self[e45])
    }
}
impl FlatWeightNormSquared for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        )
    }
}
impl FlatWeightNormSquared for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        )
    }
}
impl FlatWeightNormSquared for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435])
    }
}
impl FlatWeightNormSquared for LineOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435])
    }
}
impl FlatWeightNormSquared for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
        )
    }
}
impl FlatWeightNormSquared for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
        )
    }
}
impl FlatWeightNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
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
impl FlatWeightNormSquared for MysteryCircle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435])
    }
}
impl FlatWeightNormSquared for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
        )
    }
}
impl FlatWeightNormSquared for MysteryDipole {
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e45] * self[e45])
    }
}
impl FlatWeightNormSquared for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        )
    }
}
impl FlatWeightNormSquared for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435],
        )
    }
}
impl FlatWeightNormSquared for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125] + self[e45] * self[e45],
        )
    }
}
impl FlatWeightNormSquared for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125])
    }
}
impl FlatWeightNormSquared for PlaneOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125])
    }
}
impl FlatWeightNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125])
    }
}
impl FlatWeightNormSquared for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125])
    }
}
impl FlatWeightNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435],
        )
    }
}
impl FlatWeightNormSquared for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435],
        )
    }
}
impl FlatWeightNormSquared for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435],
        )
    }
}
impl FlatWeightNormSquared for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435],
        )
    }
}
impl FlatWeightNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        )
    }
}
impl FlatWeightNormSquared for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn flat_weight_norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
        )
    }
}
