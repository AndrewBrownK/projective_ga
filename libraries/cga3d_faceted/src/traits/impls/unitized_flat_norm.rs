// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 30
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         0       3       0     N/A
//  Average:         0       3       0     N/A
//  Maximum:         2       6       3     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         0       3       0       0
//  Average:         0       3       0       0
//  Maximum:         2       6       3       0
impl UnitizedFlatNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        3        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        let wedge_g0 = self.group2().xyz();
        -(wedge_g0[0] * wedge_g0[0] / self[e45]) - (wedge_g0[1] * wedge_g0[1] / self[e45]) - (wedge_g0[2] * wedge_g0[2] / self[e45])
    }
}
impl UnitizedFlatNorm for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        3        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        let wedge_g0 = self.group1().xyz();
        -(wedge_g0[0] * wedge_g0[0] / self[e45]) - (wedge_g0[1] * wedge_g0[1] / self[e45]) - (wedge_g0[2] * wedge_g0[2] / self[e45])
    }
}
impl UnitizedFlatNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e5] * self[e5]
    }
}
impl UnitizedFlatNorm for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e5] * self[e5]
    }
}
impl UnitizedFlatNorm for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e5] * self[e5] * self[e415] * self[e415]
    }
}
impl UnitizedFlatNorm for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235]
    }
}
impl UnitizedFlatNorm for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235]
    }
}
impl UnitizedFlatNorm for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235]
    }
}
impl UnitizedFlatNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235]
    }
}
impl UnitizedFlatNorm for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235]
    }
}
impl UnitizedFlatNorm for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235]
    }
}
impl UnitizedFlatNorm for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235]
    }
}
impl UnitizedFlatNorm for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        3        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        -(self[e15] * self[e15] / self[e45]) - (self[e25] * self[e25] / self[e45]) - (self[e35] * self[e35] / self[e45])
    }
}
impl UnitizedFlatNorm for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        3        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        -(self[e15] * self[e15] / self[e45]) - (self[e25] * self[e25] / self[e45]) - (self[e35] * self[e35] / self[e45])
    }
}
impl UnitizedFlatNorm for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        3        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        -(self[e15] * self[e15] / self[e45]) - (self[e25] * self[e25] / self[e45]) - (self[e35] * self[e35] / self[e45])
    }
}
impl UnitizedFlatNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        4        0        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        (self[e45] * self[e45] * self[e15] * self[e15]) * -1.0
    }
}
impl UnitizedFlatNorm for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        4        0        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        (self[e45] * self[e45] * self[e15] * self[e15]) * -1.0
    }
}
impl UnitizedFlatNorm for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        4        0        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        (self[e45] * self[e45] * self[e15] * self[e15]) * -1.0
    }
}
impl UnitizedFlatNorm for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        3        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        let wedge_g0 = self.group0().xyz();
        -(wedge_g0[0] * wedge_g0[0] / self[e45]) - (wedge_g0[1] * wedge_g0[1] / self[e45]) - (wedge_g0[2] * wedge_g0[2] / self[e45])
    }
}
impl UnitizedFlatNorm for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        4        0        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        (self[e15] * self[e15] * self[e45] * self[e45]) * -1.0
    }
}
impl UnitizedFlatNorm for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e235] * self[e235]
    }
}
impl UnitizedFlatNorm for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
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
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        let sub_type_g0 = self.group0().xyz();
        -(sub_type_g0[0] * sub_type_g0[0] * self[e3215]) - (sub_type_g0[1] * sub_type_g0[1] * self[e3215]) - (sub_type_g0[2] * sub_type_g0[2] * self[e3215])
    }
}
impl UnitizedFlatNorm for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        let sub_type_g0 = self.group0().xyz();
        -(sub_type_g0[0] * sub_type_g0[0] * self[e3215]) - (sub_type_g0[1] * sub_type_g0[1] * self[e3215]) - (sub_type_g0[2] * sub_type_g0[2] * self[e3215])
    }
}
impl UnitizedFlatNorm for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e5] * self[e5]
    }
}
impl UnitizedFlatNorm for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e5] * self[e5]
    }
}
impl UnitizedFlatNorm for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        self[e415] * self[e415] * self[e5] * self[e5]
    }
}
impl UnitizedFlatNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        4        0        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        (self[e45] * self[e45] * self[e15] * self[e15]) * -1.0
    }
}
impl UnitizedFlatNorm for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        4        0        0
    fn unitized_flat_norm(self) -> f32 {
        use crate::elements::*;
        (self[e15] * self[e15] * self[e45] * self[e45]) * -1.0
    }
}
