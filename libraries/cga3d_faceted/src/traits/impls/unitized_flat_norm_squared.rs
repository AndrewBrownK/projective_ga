// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 30
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       0       0
//  Maximum:         2       1       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       0       0
//  Maximum:         2       1       0
impl UnitizedFlatNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        -(self[e15] * self[e15] / (self[e45] * self[e45])) - (self[e25] * self[e25] / (self[e45] * self[e45])) - (self[e35] * self[e35] / (self[e45] * self[e45]))
    }
}
impl UnitizedFlatNormSquared for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        -(self[e15] * self[e15] / (self[e45] * self[e45])) - (self[e25] * self[e25] / (self[e45] * self[e45])) - (self[e35] * self[e35] / (self[e45] * self[e45]))
    }
}
impl UnitizedFlatNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e5] * self[e5] * -1.0
    }
}
impl UnitizedFlatNormSquared for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e5] * self[e5] * -1.0
    }
}
impl UnitizedFlatNormSquared for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e5] * self[e5] * self[e415] * self[e415] * -1.0
    }
}
impl UnitizedFlatNormSquared for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235] * -1.0
    }
}
impl UnitizedFlatNormSquared for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235] * -1.0
    }
}
impl UnitizedFlatNormSquared for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235] * -1.0
    }
}
impl UnitizedFlatNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235] * -1.0
    }
}
impl UnitizedFlatNormSquared for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235] * -1.0
    }
}
impl UnitizedFlatNormSquared for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235] * -1.0
    }
}
impl UnitizedFlatNormSquared for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235] * -1.0
    }
}
impl UnitizedFlatNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        -(self[e15] * self[e15] / (self[e45] * self[e45])) - (self[e25] * self[e25] / (self[e45] * self[e45])) - (self[e35] * self[e35] / (self[e45] * self[e45]))
    }
}
impl UnitizedFlatNormSquared for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        -(self[e15] * self[e15] / (self[e45] * self[e45])) - (self[e25] * self[e25] / (self[e45] * self[e45])) - (self[e35] * self[e35] / (self[e45] * self[e45]))
    }
}
impl UnitizedFlatNormSquared for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        -(self[e15] * self[e15] / (self[e45] * self[e45])) - (self[e25] * self[e25] / (self[e45] * self[e45])) - (self[e35] * self[e35] / (self[e45] * self[e45]))
    }
}
impl UnitizedFlatNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e45] * self[e45] * self[e15] * self[e15] * -1.0
    }
}
impl UnitizedFlatNormSquared for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e45] * self[e45] * self[e15] * self[e15] * -1.0
    }
}
impl UnitizedFlatNormSquared for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e45] * self[e45] * self[e15] * self[e15] * -1.0
    }
}
impl UnitizedFlatNormSquared for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        -(self[e15] * self[e15] / (self[e45] * self[e45])) - (self[e25] * self[e25] / (self[e45] * self[e45])) - (self[e35] * self[e35] / (self[e45] * self[e45]))
    }
}
impl UnitizedFlatNormSquared for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e15] * self[e15] * self[e45] * self[e45] * -1.0
    }
}
impl UnitizedFlatNormSquared for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235] * -1.0
    }
}
impl UnitizedFlatNormSquared for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e5] * self[e5] * -1.0
    }
}
impl UnitizedFlatNormSquared for MultiVector {
    fn unitized_flat_norm_squared(self) -> f32 {
        0.0
    }
}
impl UnitizedFlatNormSquared for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        -(self[e4235] * self[e4235] * self[e3215] * self[e3215]) - (self[e4315] * self[e4315] * self[e3215] * self[e3215]) - (self[e4125] * self[e4125] * self[e3215] * self[e3215])
    }
}
impl UnitizedFlatNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        -(self[e4235] * self[e4235] * self[e3215] * self[e3215]) - (self[e4315] * self[e4315] * self[e3215] * self[e3215]) - (self[e4125] * self[e4125] * self[e3215] * self[e3215])
    }
}
impl UnitizedFlatNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e5] * self[e5] * -1.0
    }
}
impl UnitizedFlatNormSquared for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e5] * self[e5] * -1.0
    }
}
impl UnitizedFlatNormSquared for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e5] * self[e5] * -1.0
    }
}
impl UnitizedFlatNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e45] * self[e45] * self[e15] * self[e15] * -1.0
    }
}
impl UnitizedFlatNormSquared for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e15] * self[e15] * self[e45] * self[e45] * -1.0
    }
}
