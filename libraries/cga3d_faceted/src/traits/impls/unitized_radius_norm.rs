// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 43
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         0       3       0     N/A
//  Average:         0       3       0     N/A
//  Maximum:         3       7       3     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         0       3       0       0
//  Average:         0       3       0       0
//  Maximum:         3       7       3       2
impl UnitizedRadiusNorm for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[e23] * self[e23]
    }
}
impl UnitizedRadiusNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        1
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (f32::powi(self[e41], 3) * self[e15]) * 2.0
    }
}
impl UnitizedRadiusNorm for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        1
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (f32::powi(self[e41], 3) * self[e15]) * 2.0
    }
}
impl UnitizedRadiusNorm for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[scalar] * self[scalar]
    }
}
impl UnitizedRadiusNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * self[e4] * self[e4]
    }
}
impl UnitizedRadiusNorm for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * self[e4] * self[e4]
    }
}
impl UnitizedRadiusNorm for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        4        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (self[e415] * self[e415] * self[e4] * self[e4]) * -1.0
    }
}
impl UnitizedRadiusNorm for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let wedge_g0 = self.group0().xyz();
        (wedge_g0[0] * wedge_g0[0] * self[e321]) + (wedge_g0[1] * wedge_g0[1] * self[e321]) + (wedge_g0[2] * wedge_g0[2] * self[e321])
    }
}
impl UnitizedRadiusNorm for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        self[scalar] / self[e1234]
    }
}
impl UnitizedRadiusNorm for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        3        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (self[e1] * self[e1] / self[e4]) + (self[e2] * self[e2] / self[e4]) + (self[e3] * self[e3] / self[e4])
    }
}
impl UnitizedRadiusNorm for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        self[e41] * self[e41] * self[scalar] * self[scalar]
    }
}
impl UnitizedRadiusNorm for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        self[e423] * self[e423] * self[e321] * self[e321]
    }
}
impl UnitizedRadiusNorm for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        4        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (self[e423] * self[e423] * self[e415] * self[e415]) * -1.0
    }
}
impl UnitizedRadiusNorm for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        1
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (f32::powi(self[e423], 3) * self[e235]) * -2.0
    }
}
impl UnitizedRadiusNorm for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        4        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (self[e423] * self[e423] * self[e415] * self[e415]) * -1.0
    }
}
impl UnitizedRadiusNorm for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        self[e423] * self[e423] * self[e321] * self[e321]
    }
}
impl UnitizedRadiusNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        self[e423] * self[e423] * self[e321] * self[e321]
    }
}
impl UnitizedRadiusNorm for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        4        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (self[e423] * self[e423] * self[e415] * self[e415]) * -1.0
    }
}
impl UnitizedRadiusNorm for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        4        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (self[e423] * self[e423] * self[e12345] * self[e12345]) * -1.0
    }
}
impl UnitizedRadiusNorm for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        1
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (f32::powi(self[e41], 3) * self[e15]) * 2.0
    }
}
impl UnitizedRadiusNorm for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        1
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (f32::powi(self[e41], 3) * self[e15]) * 2.0
    }
}
impl UnitizedRadiusNorm for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        1
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (f32::powi(self[e41], 3) * self[e15]) * 2.0
    }
}
impl UnitizedRadiusNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        1
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (f32::powi(self[e41], 3) * self[e15]) * 2.0
    }
}
impl UnitizedRadiusNorm for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        1
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (f32::powi(self[e41], 3) * self[e15]) * 2.0
    }
}
impl UnitizedRadiusNorm for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        1
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (f32::powi(self[e41], 3) * self[e15]) * 2.0
    }
}
impl UnitizedRadiusNorm for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        4        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (self[e41] * self[e41] * self[e45] * self[e45]) * -1.0
    }
}
impl UnitizedRadiusNorm for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        1
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (f32::powi(self[e41], 3) * self[e15]) * 2.0
    }
}
impl UnitizedRadiusNorm for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        let wedge_g0 = self.group0().xyz();
        -(wedge_g0[0] * wedge_g0[0] * self[e45]) - (wedge_g0[1] * wedge_g0[1] * self[e45]) - (wedge_g0[2] * wedge_g0[2] * self[e45])
    }
}
impl UnitizedRadiusNorm for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        1
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (f32::powi(self[e41], 3) * self[e15]) * 2.0
    }
}
impl UnitizedRadiusNorm for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        1        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        self[e12345] * -1.0 / self[e4]
    }
}
impl UnitizedRadiusNorm for MultiVector {
    fn unitized_radius_norm(self) -> f32 {
        0.0
    }
}
impl UnitizedRadiusNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        7        3        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (self[e1] * self[e1] / self[e4]) + (self[e2] * self[e2] / self[e4]) + (self[e3] * self[e3] / self[e4]) - 2.0 * self[e5]
    }
}
impl UnitizedRadiusNorm for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        2
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        f32::powf(self[e5], 0.5) * -2.0 / f32::powf(self[e4], 0.5)
    }
}
impl UnitizedRadiusNorm for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        7        3        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        2.0 * self[e3215] - (self[e4235] * self[e4235] / self[e1234]) - (self[e4315] * self[e4315] / self[e1234]) - (self[e4125] * self[e4125] / self[e1234])
    }
}
impl UnitizedRadiusNorm for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        2
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        f32::powf(self[e3215], 0.5) * 2.0 / f32::powf(self[e1234], 0.5)
    }
}
impl UnitizedRadiusNorm for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        3        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        -(self[e4235] * self[e4235] / self[e1234]) - (self[e4315] * self[e4315] / self[e1234]) - (self[e4125] * self[e4125] / self[e1234])
    }
}
impl UnitizedRadiusNorm for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * self[e4] * self[e4]
    }
}
impl UnitizedRadiusNorm for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        4        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (self[e12345] * self[e12345] * self[e4] * self[e4]) * -1.0
    }
}
impl UnitizedRadiusNorm for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        4        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (self[e4] * self[e4] * self[e423] * self[e235]) * -2.0
    }
}
impl UnitizedRadiusNorm for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        4        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (self[e12345] * self[e12345] * self[e4] * self[e4]) * -1.0
    }
}
impl UnitizedRadiusNorm for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        self[e321] * self[e321] * self[e4] * self[e4]
    }
}
impl UnitizedRadiusNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        1
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (f32::powi(self[e41], 3) * self[e15]) * 2.0
    }
}
impl UnitizedRadiusNorm for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        1
    fn unitized_radius_norm(self) -> f32 {
        use crate::elements::*;
        (f32::powi(self[e41], 3) * self[e15]) * 2.0
    }
}
