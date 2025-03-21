// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 9
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         2       9       0     N/A
//  Average:         1       8       0     N/A
//  Maximum:         3      12       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         2       9       0       0
//  Average:         1       8       0       0
//  Maximum:         3      12       0       0
impl UnitizedCenterNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3       12        0        0
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type_g0_xyz = self.group1().xyz();
        (sub_type_g0_xyz[0] * sub_type_g0_xyz[0] * self[e41] * self[e41])
            + (sub_type_g0_xyz[1] * sub_type_g0_xyz[1] * self[e41] * self[e41])
            + (sub_type_g0_xyz[2] * sub_type_g0_xyz[2] * self[e41] * self[e41])
            + (self[e41] * self[e41] * self[scalar] * self[scalar])
    }
}
impl UnitizedCenterNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3       12        0        0
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        (self[e321] * self[e321] * self[e4] * self[e4])
            + (self[e4] * self[e4] * self[e1] * self[e1])
            + (self[e4] * self[e4] * self[e2] * self[e2])
            + (self[e4] * self[e4] * self[e3] * self[e3])
    }
}
impl UnitizedCenterNormSquared for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e423] * self[e423] * self[e321] * self[e321]
    }
}
impl UnitizedCenterNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e423] * self[e423] * self[e321] * self[e321]
    }
}
impl UnitizedCenterNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        9        0        0
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type_g0 = self.group1().xyz();
        (sub_type_g0[0] * sub_type_g0[0] * self[e41] * self[e41])
            + (sub_type_g0[1] * sub_type_g0[1] * self[e41] * self[e41])
            + (sub_type_g0[2] * sub_type_g0[2] * self[e41] * self[e41])
    }
}
impl UnitizedCenterNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        9        0        0
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        let sub_type_g0 = self.group1().xyz();
        (sub_type_g0[0] * sub_type_g0[0] * self[e41] * self[e41])
            + (sub_type_g0[1] * sub_type_g0[1] * self[e41] * self[e41])
            + (sub_type_g0[2] * sub_type_g0[2] * self[e41] * self[e41])
    }
}
impl UnitizedCenterNormSquared for MultiVector {
    fn unitized_center_norm_squared(self) -> f32 {
        0.0
    }
}
impl UnitizedCenterNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3       12        0        0
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        (self[e321] * self[e321] * self[e4] * self[e4])
            + (self[e1] * self[e1] * self[e4] * self[e4])
            + (self[e2] * self[e2] * self[e4] * self[e4])
            + (self[e3] * self[e3] * self[e4] * self[e4])
    }
}
impl UnitizedCenterNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3       12        0        0
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        let wedge_g0_xyz = self.group0().xyz();
        (wedge_g0_xyz[0] * wedge_g0_xyz[0] * self[scalar] * self[scalar])
            + (wedge_g0_xyz[0] * wedge_g0_xyz[0] * self[e23] * self[e23])
            + (wedge_g0_xyz[0] * wedge_g0_xyz[0] * self[e31] * self[e31])
            + (wedge_g0_xyz[0] * wedge_g0_xyz[0] * self[e12] * self[e12])
    }
}
