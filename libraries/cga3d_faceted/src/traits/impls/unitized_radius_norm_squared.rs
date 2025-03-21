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
//  Minimum:         0       2       0     N/A
//   Median:         6      13       0     N/A
//  Average:         6      14       0     N/A
//  Maximum:        30      48       4     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0       0
//   Median:         6      13       0       0
//  Average:         6      14       0       0
//  Maximum:        30      48       4       0
impl UnitizedRadiusNormSquared for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        9        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -self[e23] * self[e23] - self[e31] * self[e31] - self[e12] * self[e12];
        -(self[e41] * self[e41] * anti_dot_product_g0) - (self[e42] * self[e42] * anti_dot_product_g0) - (self[e43] * self[e43] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       17        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e45] * self[e45]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - self[scalar] * self[scalar]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35]);
        -(self[e41] * self[e41] * anti_dot_product_g0) - (self[e42] * self[e42] * anti_dot_product_g0) - (self[e43] * self[e43] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        8       16        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - self[scalar] * self[scalar]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35]);
        -(self[e41] * self[e41] * anti_dot_product_g0) - (self[e42] * self[e42] * anti_dot_product_g0) - (self[e43] * self[e43] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5       10        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -self[scalar] * self[scalar] - self[e23] * self[e23] - self[e31] * self[e31] - self[e12] * self[e12];
        let wedge_g0 = self.group0().xyz();
        -(wedge_g0[0] * wedge_g0[0] * anti_dot_product_g0) - (wedge_g0[1] * wedge_g0[1] * anti_dot_product_g0) - (wedge_g0[2] * wedge_g0[2] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32       13       23        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + 2.0 * (self[e4] * self[e5])
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435]
            - self[e321] * self[e321]
            - self[e1] * self[e1]
            - self[e2] * self[e2]
            - self[e3] * self[e3];
        -(self[e423] * self[e423] * anti_dot_product_g0)
            - (self[e431] * self[e431] * anti_dot_product_g0)
            - (self[e412] * self[e412] * anti_dot_product_g0)
            - (self[e4] * self[e4] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6       12        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -self[e321] * self[e321] - self[e1] * self[e1] - self[e2] * self[e2] - self[e3] * self[e3];
        -(self[e423] * self[e423] * anti_dot_product_g0)
            - (self[e431] * self[e431] * anti_dot_product_g0)
            - (self[e412] * self[e412] * anti_dot_product_g0)
            - (self[e4] * self[e4] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       19        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + 2.0 * (self[e5] * self[e4])
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435];
        -(self[e423] * self[e423] * anti_dot_product_g0)
            - (self[e431] * self[e431] * anti_dot_product_g0)
            - (self[e412] * self[e412] * anti_dot_product_g0)
            - (self[e4] * self[e4] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        8        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e321] * self[e321] * -1.0;
        let wedge_g0 = self.group0().xyz();
        -(wedge_g0[0] * wedge_g0[0] * anti_dot_product_g0) - (wedge_g0[1] * wedge_g0[1] * anti_dot_product_g0) - (wedge_g0[2] * wedge_g0[2] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        1        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[scalar] * self[scalar] / (self[e1234] * self[e1234])
    }
}
impl UnitizedRadiusNormSquared for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        9        3        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        (self[e1] * self[e1] / (self[e4] * self[e4])) + (self[e2] * self[e2] / (self[e4] * self[e4])) + (self[e3] * self[e3] / (self[e4] * self[e4]))
    }
}
impl UnitizedRadiusNormSquared for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6       12        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -self[scalar] * self[scalar] - self[e23] * self[e23] - self[e31] * self[e31] - self[e12] * self[e12];
        let wedge_g0_xyz = self.group0().xyz();
        -(wedge_g0_xyz[0] * wedge_g0_xyz[0] * anti_dot_product_g0)
            - (wedge_g0_xyz[1] * wedge_g0_xyz[1] * anti_dot_product_g0)
            - (wedge_g0_xyz[2] * wedge_g0_xyz[2] * anti_dot_product_g0)
            - (self[e1234] * self[e1234] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        8       16        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435]
            - self[e321] * self[e321];
        -(self[e423] * self[e423] * anti_dot_product_g0) - (self[e431] * self[e431] * anti_dot_product_g0) - (self[e412] * self[e412] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        7       15        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435];
        -(self[e423] * self[e423] * anti_dot_product_g0) - (self[e431] * self[e431] * anti_dot_product_g0) - (self[e412] * self[e412] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4       12        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]);
        -(self[e423] * self[e423] * anti_dot_product_g0) - (self[e431] * self[e431] * anti_dot_product_g0) - (self[e412] * self[e412] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        9        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435];
        -(self[e423] * self[e423] * anti_dot_product_g0) - (self[e431] * self[e431] * anti_dot_product_g0) - (self[e412] * self[e412] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5       13        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]) - self[e321] * self[e321];
        let wedge_g0 = self.group0().xyz();
        -(wedge_g0[0] * wedge_g0[0] * anti_dot_product_g0) - (wedge_g0[1] * wedge_g0[1] * anti_dot_product_g0) - (wedge_g0[2] * wedge_g0[2] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       17        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435]
            + self[e12345] * self[e12345]
            - self[e321] * self[e321];
        -(self[e423] * self[e423] * anti_dot_product_g0) - (self[e431] * self[e431] * anti_dot_product_g0) - (self[e412] * self[e412] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        8       16        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435]
            + self[e12345] * self[e12345];
        -(self[e423] * self[e423] * anti_dot_product_g0) - (self[e431] * self[e431] * anti_dot_product_g0) - (self[e412] * self[e412] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5       10        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435];
        let wedge_g0 = self.group0().xyz();
        -(wedge_g0[0] * wedge_g0[0] * anti_dot_product_g0) - (wedge_g0[1] * wedge_g0[1] * anti_dot_product_g0) - (wedge_g0[2] * wedge_g0[2] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        8       16        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e45] * self[e45]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35]);
        -(self[e41] * self[e41] * anti_dot_product_g0) - (self[e42] * self[e42] * anti_dot_product_g0) - (self[e43] * self[e43] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5       13        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e45] * self[e45] - 2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]);
        let wedge_g0 = self.group0().xyz();
        -(wedge_g0[0] * wedge_g0[0] * anti_dot_product_g0) - (wedge_g0[1] * wedge_g0[1] * anti_dot_product_g0) - (wedge_g0[2] * wedge_g0[2] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4       12        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]);
        -(self[e41] * self[e41] * anti_dot_product_g0) - (self[e42] * self[e42] * anti_dot_product_g0) - (self[e43] * self[e43] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32       13       23        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35])
            - 2.0 * (self[e1234] * self[e3215]);
        -(self[e41] * self[e41] * anti_dot_product_g0)
            - (self[e42] * self[e42] * anti_dot_product_g0)
            - (self[e43] * self[e43] * anti_dot_product_g0)
            - (self[e1234] * self[e1234] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32       10       20        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35])
            - 2.0 * (self[e1234] * self[e3215]);
        let wedge_g0_xyz = self.group0().xyz();
        -(wedge_g0_xyz[0] * wedge_g0_xyz[0] * anti_dot_product_g0)
            - (wedge_g0_xyz[1] * wedge_g0_xyz[1] * anti_dot_product_g0)
            - (wedge_g0_xyz[2] * wedge_g0_xyz[2] * anti_dot_product_g0)
            - (self[e1234] * self[e1234] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6       16        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]) - 2.0 * (self[e3215] * self[e1234]);
        let wedge_g0_xyz = self.group0().xyz();
        -(wedge_g0_xyz[0] * wedge_g0_xyz[0] * anti_dot_product_g0)
            - (wedge_g0_xyz[1] * wedge_g0_xyz[1] * anti_dot_product_g0)
            - (wedge_g0_xyz[2] * wedge_g0_xyz[2] * anti_dot_product_g0)
            - (self[e1234] * self[e1234] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6       12        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125];
        let wedge_g0_xyz = self.group0().xyz();
        -(wedge_g0_xyz[0] * wedge_g0_xyz[0] * anti_dot_product_g0)
            - (wedge_g0_xyz[1] * wedge_g0_xyz[1] * anti_dot_product_g0)
            - (wedge_g0_xyz[2] * wedge_g0_xyz[2] * anti_dot_product_g0)
            - (self[e1234] * self[e1234] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       19        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35])
            - 2.0 * (self[e3215] * self[e1234]);
        let wedge_g0_xyz = self.group0().xyz();
        -(wedge_g0_xyz[0] * wedge_g0_xyz[0] * anti_dot_product_g0)
            - (wedge_g0_xyz[1] * wedge_g0_xyz[1] * anti_dot_product_g0)
            - (wedge_g0_xyz[2] * wedge_g0_xyz[2] * anti_dot_product_g0)
            - (self[e1234] * self[e1234] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        7        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e45] * self[e45];
        let wedge_g0 = self.group0().xyz();
        -(wedge_g0[0] * wedge_g0[0] * anti_dot_product_g0) - (wedge_g0[1] * wedge_g0[1] * anti_dot_product_g0) - (wedge_g0[2] * wedge_g0[2] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        7       15        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 =
            -self[e23] * self[e23] - self[e31] * self[e31] - self[e12] * self[e12] - 2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]);
        -(self[e41] * self[e41] * anti_dot_product_g0) - (self[e42] * self[e42] * anti_dot_product_g0) - (self[e43] * self[e43] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        4        1        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e12345] * self[e12345] * -1.0 / (self[e4] * self[e4])
    }
}
impl UnitizedRadiusNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32       30       48        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e4] * self[e5])
            + 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + self[e12345] * self[e12345]
            + self[e45] * self[e45]
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435]
            + self[e4235] * self[e4235]
            + self[e4315] * self[e4315]
            + self[e4125] * self[e4125]
            - self[scalar] * self[scalar]
            - self[e1] * self[e1]
            - self[e2] * self[e2]
            - self[e3] * self[e3]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - self[e321] * self[e321]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35])
            - 2.0 * (self[e1234] * self[e3215]);
        let sub_type_g3_xyz = self.group3().xyz();
        -(sub_type_g3_xyz[0] * sub_type_g3_xyz[0] * anti_dot_product_g0)
            - (sub_type_g3_xyz[1] * sub_type_g3_xyz[1] * anti_dot_product_g0)
            - (sub_type_g3_xyz[2] * sub_type_g3_xyz[2] * anti_dot_product_g0)
            - (self[e4] * self[e4] * anti_dot_product_g0)
            - (self[e423] * self[e423] * anti_dot_product_g0)
            - (self[e431] * self[e431] * anti_dot_product_g0)
            - (self[e412] * self[e412] * anti_dot_product_g0)
            - (self[e1234] * self[e1234] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3       11        4        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        (self[e1] * self[e1] / (self[e4] * self[e4])) + (self[e2] * self[e2] / (self[e4] * self[e4])) + (self[e3] * self[e3] / (self[e4] * self[e4])) - 2.0 * (self[e5] / self[e4])
    }
}
impl UnitizedRadiusNormSquared for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        1        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e5] * -2.0 / self[e4]
    }
}
impl UnitizedRadiusNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3       11        4        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        2.0 * (self[e3215] / self[e1234])
            - (self[e4235] * self[e4235] / (self[e1234] * self[e1234]))
            - (self[e4315] * self[e4315] / (self[e1234] * self[e1234]))
            - (self[e4125] * self[e4125] / (self[e1234] * self[e1234]))
    }
}
impl UnitizedRadiusNormSquared for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        1        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        self[e3215] * 2.0 / self[e1234]
    }
}
impl UnitizedRadiusNormSquared for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        9        3        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        -(self[e4235] * self[e4235] / (self[e1234] * self[e1234]))
            - (self[e4315] * self[e4315] / (self[e1234] * self[e1234]))
            - (self[e4125] * self[e4125] / (self[e1234] * self[e1234]))
    }
}
impl UnitizedRadiusNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32       14       24        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + 2.0 * (self[e5] * self[e4])
            + self[e12345] * self[e12345]
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435]
            - self[e321] * self[e321]
            - self[e1] * self[e1]
            - self[e2] * self[e2]
            - self[e3] * self[e3];
        -(self[e423] * self[e423] * anti_dot_product_g0)
            - (self[e431] * self[e431] * anti_dot_product_g0)
            - (self[e412] * self[e412] * anti_dot_product_g0)
            - (self[e4] * self[e4] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32       10       20        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + 2.0 * (self[e4] * self[e5])
            + self[e12345] * self[e12345]
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435];
        -(self[e423] * self[e423] * anti_dot_product_g0)
            - (self[e431] * self[e431] * anti_dot_product_g0)
            - (self[e412] * self[e412] * anti_dot_product_g0)
            - (self[e4] * self[e4] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6       16        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]) + 2.0 * (self[e4] * self[e5]);
        let wedge_g0 = self.group0().wxyz();
        -(wedge_g0[0] * wedge_g0[0] * anti_dot_product_g0)
            - (wedge_g0[1] * wedge_g0[1] * anti_dot_product_g0)
            - (wedge_g0[2] * wedge_g0[2] * anti_dot_product_g0)
            - (wedge_g0[3] * wedge_g0[3] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6       12        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435];
        -(self[e423] * self[e423] * anti_dot_product_g0)
            - (self[e431] * self[e431] * anti_dot_product_g0)
            - (self[e412] * self[e412] * anti_dot_product_g0)
            - (self[e4] * self[e4] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32       10       20        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = 2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]) + 2.0 * (self[e5] * self[e4])
            - self[e321] * self[e321]
            - self[e1] * self[e1]
            - self[e2] * self[e2]
            - self[e3] * self[e3];
        -(self[e423] * self[e423] * anti_dot_product_g0)
            - (self[e431] * self[e431] * anti_dot_product_g0)
            - (self[e412] * self[e412] * anti_dot_product_g0)
            - (self[e4] * self[e4] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32       14       24        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125]
            - self[scalar] * self[scalar]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35])
            - 2.0 * (self[e1234] * self[e3215]);
        let wedge_g0_xyz = self.group0().xyz();
        -(wedge_g0_xyz[0] * wedge_g0_xyz[0] * anti_dot_product_g0)
            - (wedge_g0_xyz[1] * wedge_g0_xyz[1] * anti_dot_product_g0)
            - (wedge_g0_xyz[2] * wedge_g0_xyz[2] * anti_dot_product_g0)
            - (self[e1234] * self[e1234] * anti_dot_product_g0)
    }
}
impl UnitizedRadiusNormSquared for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32       10       20        0        0
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        let anti_dot_product_g0 = -self[scalar] * self[scalar]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35])
            - 2.0 * (self[e3215] * self[e1234]);
        let wedge_g0_xyz = self.group0().xyz();
        -(wedge_g0_xyz[0] * wedge_g0_xyz[0] * anti_dot_product_g0)
            - (wedge_g0_xyz[1] * wedge_g0_xyz[1] * anti_dot_product_g0)
            - (wedge_g0_xyz[2] * wedge_g0_xyz[2] * anti_dot_product_g0)
            - (self[e1234] * self[e1234] * anti_dot_product_g0)
    }
}
