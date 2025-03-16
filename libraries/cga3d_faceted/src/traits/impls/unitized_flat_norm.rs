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
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         2       3       3
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         2       3       3
impl UnitizedFlatNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        3
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        -(self[e15] * self[e15] / self[e45]) - (self[e25] * self[e25] / self[e45]) - (self[e35] * self[e35] / self[e45])
    }
}
impl UnitizedFlatNorm for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        3
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        -(self[e15] * self[e15] / self[e45]) - (self[e25] * self[e25] / self[e45]) - (self[e35] * self[e35] / self[e45])
    }
}
impl UnitizedFlatNorm for AntiDipoleInversion {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e5] * self[e5]
    }
}
impl UnitizedFlatNorm for AntiDipoleInversionAtInfinity {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e5] * self[e5]
    }
}
impl UnitizedFlatNorm for AntiDipoleInversionOrthogonalOrigin {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e5] * self[e5] * self[e415] * self[e415]
    }
}
impl UnitizedFlatNorm for Circle {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235]
    }
}
impl UnitizedFlatNorm for CircleAligningOrigin {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235]
    }
}
impl UnitizedFlatNorm for CircleAtInfinity {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235]
    }
}
impl UnitizedFlatNorm for CircleRotor {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235]
    }
}
impl UnitizedFlatNorm for CircleRotorAligningOrigin {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235]
    }
}
impl UnitizedFlatNorm for CircleRotorAligningOriginAtInfinity {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235]
    }
}
impl UnitizedFlatNorm for CircleRotorAtInfinity {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235]
    }
}
impl UnitizedFlatNorm for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        3
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        -(self[e15] * self[e15] / self[e45]) - (self[e25] * self[e25] / self[e45]) - (self[e35] * self[e35] / self[e45])
    }
}
impl UnitizedFlatNorm for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        3
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        -(self[e15] * self[e15] / self[e45]) - (self[e25] * self[e25] / self[e45]) - (self[e35] * self[e35] / self[e45])
    }
}
impl UnitizedFlatNorm for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        3
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        -(self[e15] * self[e15] / self[e45]) - (self[e25] * self[e25] / self[e45]) - (self[e35] * self[e35] / self[e45])
    }
}
impl UnitizedFlatNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        (self[e45] * self[e45] * self[e15] * self[e15]) * -1.0
    }
}
impl UnitizedFlatNorm for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        (self[e45] * self[e45] * self[e15] * self[e15]) * -1.0
    }
}
impl UnitizedFlatNorm for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        (self[e45] * self[e45] * self[e15] * self[e15]) * -1.0
    }
}
impl UnitizedFlatNorm for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        3
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        -(self[e15] * self[e15] / self[e45]) - (self[e25] * self[e25] / self[e45]) - (self[e35] * self[e35] / self[e45])
    }
}
impl UnitizedFlatNorm for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        (self[e15] * self[e15] * self[e45] * self[e45]) * -1.0
    }
}
impl UnitizedFlatNorm for Line {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235]
    }
}
impl UnitizedFlatNorm for Motor {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e5] * self[e5]
    }
}
impl UnitizedFlatNorm for MultiVector {
    fn unitized_flat_norm(self) -> f32 {
        0.0
    }
}
impl UnitizedFlatNorm for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        -(self[e4235] * self[e4235] * self[e3215]) - (self[e4315] * self[e4315] * self[e3215]) - (self[e4125] * self[e4125] * self[e3215])
    }
}
impl UnitizedFlatNorm for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        -(self[e4235] * self[e4235] * self[e3215]) - (self[e4315] * self[e4315] * self[e3215]) - (self[e4125] * self[e4125] * self[e3215])
    }
}
impl UnitizedFlatNorm for VersorEven {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e5] * self[e5]
    }
}
impl UnitizedFlatNorm for VersorEvenAligningOrigin {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e5] * self[e5]
    }
}
impl UnitizedFlatNorm for VersorEvenAtInfinity {
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e5] * self[e5]
    }
}
impl UnitizedFlatNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        (self[e45] * self[e45] * self[e15] * self[e15]) * -1.0
    }
}
impl UnitizedFlatNorm for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        (self[e15] * self[e15] * self[e45] * self[e45]) * -1.0
    }
}
