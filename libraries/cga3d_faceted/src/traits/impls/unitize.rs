// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 49
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         2       6       0     N/A
//  Average:         2       5       0     N/A
//  Maximum:         7      19       1     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         2      12       0       0
//  Average:         2      11       0       0
//  Maximum:        12      40       1       4
impl Unitize for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        9        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43];
        AntiCircleOnOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product_g0) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product_g0) * self.group1(),
        )
    }
}
impl Unitize for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        2       14        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43];
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product_g0) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(geometric_anti_product_g0) * self.group2(),
        )
    }
}
impl Unitize for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        2       13        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43];
        AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product_g0) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product_g0) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(geometric_anti_product_g0) * self.group2(),
        )
    }
}
impl Unitize for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2       10        0        0
    fn unitize(self) -> Self {
        let wedge_g0 = self.group0().xyz();
        let geometric_anti_product_g0 = wedge_g0[0] * wedge_g0[0] + wedge_g0[1] * wedge_g0[1] + wedge_g0[2] * wedge_g0[2];
        AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product_g0) * self.group1(),
        )
    }
}
impl Unitize for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       19        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4];
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(geometric_anti_product_g0) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(geometric_anti_product_g0) * self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(geometric_anti_product_g0) * self.group3(),
        )
    }
}
impl Unitize for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3       12        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4];
        AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e4, e1, e2, e3
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
        )
    }
}
impl Unitize for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       15        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4];
        AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e415, e425, e435
            Simd32x3::from(geometric_anti_product_g0) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(geometric_anti_product_g0) * self.group2(),
        )
    }
}
impl Unitize for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        8       15        0        0
    fn unitize(self) -> Self {
        let wedge_g0 = self.group0().xyz();
        AntiDipoleOnOrigin::from_groups(
            // e423, e431, e412, e321
            (Simd32x4::from(wedge_g0[0] * wedge_g0[0]) * self.group0())
                + (Simd32x4::from(wedge_g0[1] * wedge_g0[1]) * self.group0())
                + (Simd32x4::from(wedge_g0[2] * wedge_g0[2]) * self.group0()),
        )
    }
}
impl Unitize for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from([1.0, self[scalar] / self[e1234]]))
    }
}
impl Unitize for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        0        1        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        1        1      N/A
    //  no simd        0        3        1        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        AntiSphereOnOrigin::from_groups(/* e1, e2, e3, e4 */ (Simd32x3::from(1.0 / self[e4]) * self.group0().xyz()).with_w(1.0))
    }
}
impl Unitize for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3       12        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge_g0_xyz = self.group0().xyz();
        let geometric_anti_product_g0 = wedge_g0_xyz[0] * wedge_g0_xyz[0] + wedge_g0_xyz[1] * wedge_g0_xyz[1] + wedge_g0_xyz[2] * wedge_g0_xyz[2] + self[e1234] * self[e1234];
        AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e23, e31, e12, e1234
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
        )
    }
}
impl Unitize for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        2       13        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412];
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(geometric_anti_product_g0) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
            // e235, e315, e125
            Simd32x3::from(geometric_anti_product_g0) * self.group2(),
        )
    }
}
impl Unitize for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        3        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        2       12        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412];
        CircleAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(geometric_anti_product_g0) * self.group0(),
            // e415, e425, e435
            Simd32x3::from(geometric_anti_product_g0) * self.group1(),
            // e235, e315, e125
            Simd32x3::from(geometric_anti_product_g0) * self.group2(),
        )
    }
}
impl Unitize for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        9        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412];
        CircleAtOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(geometric_anti_product_g0) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(geometric_anti_product_g0) * self.group1(),
        )
    }
}
impl Unitize for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        9        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412];
        CircleOnOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(geometric_anti_product_g0) * self.group0(),
            // e415, e425, e435
            Simd32x3::from(geometric_anti_product_g0) * self.group1(),
        )
    }
}
impl Unitize for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2       10        0        0
    fn unitize(self) -> Self {
        let wedge_g0 = self.group0().xyz();
        let geometric_anti_product_g0 = wedge_g0[0] * wedge_g0[0] + wedge_g0[1] * wedge_g0[1] + wedge_g0[2] * wedge_g0[2];
        CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(geometric_anti_product_g0) * self.group1(),
        )
    }
}
impl Unitize for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        2       14        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412];
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(geometric_anti_product_g0) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(geometric_anti_product_g0) * self.group2(),
        )
    }
}
impl Unitize for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        2       13        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412];
        CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(geometric_anti_product_g0) * self.group0(),
            // e415, e425, e435
            Simd32x3::from(geometric_anti_product_g0) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(geometric_anti_product_g0) * self.group2(),
        )
    }
}
impl Unitize for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2       10        0        0
    fn unitize(self) -> Self {
        let wedge_g0 = self.group0().xyz();
        let geometric_anti_product_g0 = wedge_g0[0] * wedge_g0[0] + wedge_g0[1] * wedge_g0[1] + wedge_g0[2] * wedge_g0[2];
        CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e415, e425, e435
            Simd32x3::from(geometric_anti_product_g0) * self.group1(),
        )
    }
}
impl Unitize for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        2       13        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43];
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product_g0) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
            // e15, e25, e35
            Simd32x3::from(geometric_anti_product_g0) * self.group2(),
        )
    }
}
impl Unitize for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2       10        0        0
    fn unitize(self) -> Self {
        let wedge_g0 = self.group0().xyz();
        let geometric_anti_product_g0 = wedge_g0[0] * wedge_g0[0] + wedge_g0[1] * wedge_g0[1] + wedge_g0[2] * wedge_g0[2];
        DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(geometric_anti_product_g0) * self.group1(),
        )
    }
}
impl Unitize for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        9        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43];
        DipoleAtOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product_g0) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(geometric_anti_product_g0) * self.group1(),
        )
    }
}
impl Unitize for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       19        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234];
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product_g0) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(geometric_anti_product_g0) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(geometric_anti_product_g0) * self.group3(),
        )
    }
}
impl Unitize for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       16        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge_g0_xyz = self.group0().xyz();
        let geometric_anti_product_g0 = wedge_g0_xyz[0] * wedge_g0_xyz[0] + wedge_g0_xyz[1] * wedge_g0_xyz[1] + wedge_g0_xyz[2] * wedge_g0_xyz[2] + self[e1234] * self[e1234];
        DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(geometric_anti_product_g0) * self.group2(),
        )
    }
}
impl Unitize for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3       12        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge_g0_xyz = self.group0().xyz();
        let geometric_anti_product_g0 = wedge_g0_xyz[0] * wedge_g0_xyz[0] + wedge_g0_xyz[1] * wedge_g0_xyz[1] + wedge_g0_xyz[2] * wedge_g0_xyz[2] + self[e1234] * self[e1234];
        DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
        )
    }
}
impl Unitize for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3       12        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge_g0_xyz = self.group0().xyz();
        let geometric_anti_product_g0 = wedge_g0_xyz[0] * wedge_g0_xyz[0] + wedge_g0_xyz[1] * wedge_g0_xyz[1] + wedge_g0_xyz[2] * wedge_g0_xyz[2] + self[e1234] * self[e1234];
        DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
        )
    }
}
impl Unitize for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       15        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge_g0_xyz = self.group0().xyz();
        let geometric_anti_product_g0 = wedge_g0_xyz[0] * wedge_g0_xyz[0] + wedge_g0_xyz[1] * wedge_g0_xyz[1] + wedge_g0_xyz[2] * wedge_g0_xyz[2] + self[e1234] * self[e1234];
        DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product_g0) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(geometric_anti_product_g0) * self.group2(),
        )
    }
}
impl Unitize for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        8       15        0        0
    fn unitize(self) -> Self {
        let wedge_g0 = self.group0().xyz();
        DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            (Simd32x4::from(wedge_g0[0] * wedge_g0[0]) * self.group0())
                + (Simd32x4::from(wedge_g0[1] * wedge_g0[1]) * self.group0())
                + (Simd32x4::from(wedge_g0[2] * wedge_g0[2]) * self.group0()),
        )
    }
}
impl Unitize for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        3        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        2       12        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43];
        DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product_g0) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product_g0) * self.group1(),
            // e15, e25, e35
            Simd32x3::from(geometric_anti_product_g0) * self.group2(),
        )
    }
}
impl Unitize for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([1.0, self[e12345] / self[e4]]))
    }
}
impl Unitize for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       10        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        4        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        7       19        0      N/A
    //  no simd        7       40        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let sub_type_g3_xyz = self.group3().xyz();
        let geometric_anti_product_g0 = sub_type_g3_xyz[0] * sub_type_g3_xyz[0]
            + sub_type_g3_xyz[1] * sub_type_g3_xyz[1]
            + sub_type_g3_xyz[2] * sub_type_g3_xyz[2]
            + self[e4] * self[e4]
            + self[e423] * self[e423]
            + self[e431] * self[e431]
            + self[e412] * self[e412]
            + self[e1234] * self[e1234];
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(geometric_anti_product_g0) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
            // e5
            geometric_anti_product_g0 * self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(geometric_anti_product_g0) * self.group3(),
            // e15, e25, e35
            Simd32x3::from(geometric_anti_product_g0) * self.group4(),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product_g0) * self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(geometric_anti_product_g0) * self.group6(),
            // e423, e431, e412
            Simd32x3::from(geometric_anti_product_g0) * self.group7(),
            // e235, e315, e125
            Simd32x3::from(geometric_anti_product_g0) * self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(geometric_anti_product_g0) * self.group9(),
            // e3215
            geometric_anti_product_g0 * self[e3215],
        )
    }
}
impl Unitize for NullCircleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        3
    //    simd3        2        2        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        6       11        0        3
    fn unitize(self) -> Self {
        use crate::elements::*;
        NullCircleAtOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::powi(self.group0(), 3)
                + (Simd32x3::from([self[e431] * self[e431], self[e423] * self[e423], self[e423] * self[e423]]) * self.group0())
                + (self.group0() * Simd32x2::from(self[e412] * self[e412]).with_z(self[e431] * self[e431])),
        )
    }
}
impl Unitize for NullDipoleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        3
    //    simd3        2        2        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        6       11        0        3
    fn unitize(self) -> Self {
        use crate::elements::*;
        NullDipoleAtOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::powi(self.group0(), 3)
                + (Simd32x3::from([self[e42] * self[e42], self[e41] * self[e41], self[e41] * self[e41]]) * self.group0())
                + (self.group0() * Simd32x2::from(self[e43] * self[e43]).with_z(self[e42] * self[e42])),
        )
    }
}
impl Unitize for NullDipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        9        0        4
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd        3       12        0      N/A
    //  no simd       12       21        0        4
    fn unitize(self) -> Self {
        use crate::elements::*;
        NullDipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::powi(self.group0(), 3)
                + (Simd32x4::from([self[e42] * self[e42], self[e41] * self[e41], self[e41] * self[e41], self[e41] * self[e41]]) * self.group0())
                + (self.group0() * Simd32x2::from(self[e43] * self[e43]).with_zw(self[e42] * self[e42], self[e42] * self[e42]))
                + (self.group0() * Simd32x3::from(self[e1234] * self[e1234]).with_w(self[e43] * self[e43])),
        )
    }
}
impl Unitize for NullSphereAtOrigin {
    fn unitize(self) -> Self {
        NullSphereAtOrigin::from_groups(/* e1234 */ 1.0)
    }
}
impl Unitize for NullVersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd       12       20        0        0
    fn unitize(self) -> Self {
        let wedge_g0 = self.group0().wxyz();
        NullVersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            (Simd32x4::from(wedge_g0[0] * wedge_g0[0]) * self.group0())
                + (Simd32x4::from(wedge_g0[1] * wedge_g0[1]) * self.group0())
                + (Simd32x4::from(wedge_g0[2] * wedge_g0[2]) * self.group0())
                + (Simd32x4::from(wedge_g0[3] * wedge_g0[3]) * self.group0()),
        )
    }
}
impl Unitize for Origin {
    fn unitize(self) -> Self {
        Origin::from_groups(/* e4 */ 1.0)
    }
}
impl Unitize for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        1        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        1      N/A
    //  no simd        0        5        1        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = 1.0 / self[e4];
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e5
            geometric_anti_product_g0 * self[e5],
        )
    }
}
impl Unitize for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        RoundPointAtOrigin::from_groups(/* e4, e5 */ Simd32x2::from([1.0, self[e5] / self[e4]]))
    }
}
impl Unitize for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        1        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        1      N/A
    //  no simd        0        5        1        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = 1.0 / self[e1234];
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e1234
            geometric_anti_product_g0 * self[e1234],
        )
    }
}
impl Unitize for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        SphereAtOrigin::from_groups(/* e3215, e1234 */ Simd32x2::from([self[e3215] / self[e1234], 1.0]))
    }
}
impl Unitize for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        0        1        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        1        1      N/A
    //  no simd        0        3        1        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ (Simd32x3::from(1.0 / self[e1234]) * self.group0().xyz()).with_w(1.0))
    }
}
impl Unitize for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       20        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4];
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(geometric_anti_product_g0) * self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product_g0) * self.group3(),
        )
    }
}
impl Unitize for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       16        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4];
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e415, e425, e435, e4
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(geometric_anti_product_g0) * self.group2(),
        )
    }
}
impl Unitize for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3       12        0        0
    fn unitize(self) -> Self {
        let wedge_g0 = self.group0().wxyz();
        let geometric_anti_product_g0 = wedge_g0[0] * wedge_g0[0] + wedge_g0[1] * wedge_g0[1] + wedge_g0[2] * wedge_g0[2] + wedge_g0[3] * wedge_g0[3];
        VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
        )
    }
}
impl Unitize for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3       12        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4];
        VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e415, e425, e435, e4
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
        )
    }
}
impl Unitize for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       16        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4];
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product_g0) * self.group2(),
        )
    }
}
impl Unitize for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       20        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge_g0_xyz = self.group0().xyz();
        let geometric_anti_product_g0 = wedge_g0_xyz[0] * wedge_g0_xyz[0] + wedge_g0_xyz[1] * wedge_g0_xyz[1] + wedge_g0_xyz[2] * wedge_g0_xyz[2] + self[e1234] * self[e1234];
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(geometric_anti_product_g0) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(geometric_anti_product_g0) * self.group3(),
        )
    }
}
impl Unitize for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       16        0        0
    fn unitize(self) -> Self {
        use crate::elements::*;
        let wedge_g0_xyz = self.group0().xyz();
        let geometric_anti_product_g0 = wedge_g0_xyz[0] * wedge_g0_xyz[0] + wedge_g0_xyz[1] * wedge_g0_xyz[1] + wedge_g0_xyz[2] * wedge_g0_xyz[2] + self[e1234] * self[e1234];
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(geometric_anti_product_g0) * self.group2(),
        )
    }
}
