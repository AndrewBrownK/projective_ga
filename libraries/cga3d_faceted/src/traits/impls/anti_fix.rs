// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 24
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         2       7       0     N/A
//  Average:         1       6       0     N/A
//  Maximum:         3      14       1     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         6      11       0       0
//  Average:         4      11       0       1
//  Maximum:        12      23       2       4
impl AntiFix for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        1        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        1      N/A
    //  no simd        0        4        1        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ (Simd32x3::from(-1.0 / self[e321]) * self.group0().xyz()).with_w(-1.0))
    }
}
impl AntiFix for AntiFlatOrigin {
    fn anti_fix(self) -> Self {
        AntiFlatOrigin::from_groups(/* e321 */ -1.0)
    }
}
impl AntiFix for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        1        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        1      N/A
    //  no simd        0        4        1        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ (Simd32x3::from(-1.0 / self[e321]) * self.group0().xyz()).with_w(-1.0))
    }
}
impl AntiFix for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        9        0        4
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd        3       12        0      N/A
    //  no simd       12       21        0        4
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        AntiFlectorOnOrigin::from_groups(
            // e321, e1, e2, e3
            -Simd32x4::powi(self.group0(), 3)
                - (Simd32x4::from([self[e1] * self[e1], self[e321] * self[e321], self[e321] * self[e321], self[e321] * self[e321]]) * self.group0())
                - (self.group0() * Simd32x2::from(self[e2] * self[e2]).with_zw(self[e1] * self[e1], self[e1] * self[e1]))
                - (self.group0() * Simd32x3::from(self[e3] * self[e3]).with_w(self[e2] * self[e2])),
        )
    }
}
impl AntiFix for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        2       10        0      N/A
    //  no simd        6       20        0        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        AntiLineOnOrigin::from_groups(
            // e23, e31, e12
            (Simd32x3::from([anti_reverse_g0[1] * self[e31], anti_reverse_g0[0] * self[e23], anti_reverse_g0[0] * self[e23]]) * self.group0())
                + (self.group0() * Simd32x2::from(anti_reverse_g0[2] * self[e12]).with_z(anti_reverse_g0[1] * self[e31]))
                + (self.group0() * self.group0() * anti_reverse_g0),
        )
    }
}
impl AntiFix for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        9        0        4
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd        3       12        0      N/A
    //  no simd       12       21        0        4
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(
            // e23, e31, e12, scalar
            -Simd32x4::powi(self.group0(), 3)
                - (Simd32x4::from([self[e31] * self[e31], self[e23] * self[e23], self[e23] * self[e23], self[e23] * self[e23]]) * self.group0())
                - (self.group0() * Simd32x2::from(self[e12] * self[e12]).with_zw(self[e31] * self[e31], self[e31] * self[e31]))
                - (self.group0() * Simd32x3::from(self[scalar] * self[scalar]).with_w(self[e12] * self[e12])),
        )
    }
}
impl AntiFix for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       11        0        0
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2       14        0      N/A
    //  no simd        8       23        0        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            -(Simd32x4::from([self[e1] * self[e1], self[e2] * self[e2], self[e3] * self[e3], self[e1] * self[e1]]) * self.group0())
                - (Simd32x4::from([self[e2] * self[e2], self[e1] * self[e1], self[e1] * self[e1], self[e2] * self[e2]]) * self.group0())
                - (self.group0() * Simd32x2::from(self[e3] * self[e3]).with_zw(self[e2] * self[e2], self[e3] * self[e3])),
        )
    }
}
impl AntiFix for AntiPlaneOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        3
    //    simd3        2        2        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        6       11        0        3
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            -Simd32x3::powi(self.group0(), 3)
                - (Simd32x3::from([self[e2] * self[e2], self[e1] * self[e1], self[e1] * self[e1]]) * self.group0())
                - (self.group0() * Simd32x2::from(self[e3] * self[e3]).with_z(self[e2] * self[e2])),
        )
    }
}
impl AntiFix for AntiScalar {
    fn anti_fix(self) -> Self {
        AntiScalar::from_groups(/* e12345 */ 1.0)
    }
}
impl AntiFix for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       11        0        0
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2       14        0      N/A
    //  no simd        8       23        0        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        AntiSphereOnOrigin::from_groups(
            // e1, e2, e3, e4
            -(Simd32x4::from([self[e1] * self[e1], self[e2] * self[e2], self[e3] * self[e3], self[e1] * self[e1]]) * self.group0())
                - (Simd32x4::from([self[e2] * self[e2], self[e1] * self[e1], self[e1] * self[e1], self[e2] * self[e2]]) * self.group0())
                - (self.group0() * Simd32x2::from(self[e3] * self[e3]).with_zw(self[e2] * self[e2], self[e3] * self[e3])),
        )
    }
}
impl AntiFix for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        0        1        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        1        1      N/A
    //  no simd        0        3        1        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ (Simd32x3::from(1.0 / self[e45]) * self.group0().xyz()).with_w(1.0))
    }
}
impl AntiFix for FlatOrigin {
    fn anti_fix(self) -> Self {
        FlatOrigin::from_groups(/* e45 */ 1.0)
    }
}
impl AntiFix for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        0        1        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        1        1      N/A
    //  no simd        0        3        1        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ (Simd32x3::from(1.0 / self[e45]) * self.group0().xyz()).with_w(1.0))
    }
}
impl AntiFix for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        9        0        4
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd        3       12        0      N/A
    //  no simd       12       21        0        4
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::powi(self.group0(), 3)
                + (Simd32x4::from([self[e4235] * self[e4235], self[e45] * self[e45], self[e45] * self[e45], self[e45] * self[e45]]) * self.group0())
                + (self.group0() * Simd32x2::from(self[e4315] * self[e4315]).with_zw(self[e4235] * self[e4235], self[e4235] * self[e4235]))
                + (self.group0() * Simd32x3::from(self[e4125] * self[e4125]).with_w(self[e4315] * self[e4315])),
        )
    }
}
impl AntiFix for LineOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        2       10        0      N/A
    //  no simd        6       20        0        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        LineOnOrigin::from_groups(
            // e415, e425, e435
            -(Simd32x3::from([anti_reverse_g0[1] * self[e425], anti_reverse_g0[0] * self[e415], anti_reverse_g0[0] * self[e415]]) * self.group0())
                - (self.group0() * Simd32x2::from(anti_reverse_g0[2] * self[e435]).with_z(anti_reverse_g0[1] * self[e425]))
                - (self.group0() * self.group0() * anti_reverse_g0),
        )
    }
}
impl AntiFix for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        9        0        4
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd        3       12        0      N/A
    //  no simd       12       21        0        4
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        MotorOnOrigin::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::powi(self.group0(), 3)
                + (Simd32x4::from([self[e425] * self[e425], self[e415] * self[e415], self[e415] * self[e415], self[e415] * self[e415]]) * self.group0())
                + (self.group0() * Simd32x2::from(self[e435] * self[e435]).with_zw(self[e425] * self[e425], self[e425] * self[e425]))
                + (self.group0() * Simd32x3::from(self[e12345] * self[e12345]).with_w(self[e435] * self[e435])),
        )
    }
}
impl AntiFix for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       11        0        0
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2       14        0      N/A
    //  no simd        8       23        0        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x4::from([self[e4235] * self[e4235], self[e4315] * self[e4315], self[e4125] * self[e4125], self[e4235] * self[e4235]]) * self.group0())
                + (Simd32x4::from([self[e4315] * self[e4315], self[e4235] * self[e4235], self[e4235] * self[e4235], self[e4315] * self[e4315]]) * self.group0())
                + (self.group0() * Simd32x2::from(self[e4125] * self[e4125]).with_zw(self[e4315] * self[e4315], self[e4125] * self[e4125])),
        )
    }
}
impl AntiFix for PlaneOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        3
    //    simd3        2        2        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        6       11        0        3
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::powi(self.group0(), 3)
                + (Simd32x3::from([self[e4315] * self[e4315], self[e4235] * self[e4235], self[e4235] * self[e4235]]) * self.group0())
                + (self.group0() * Simd32x2::from(self[e4125] * self[e4125]).with_z(self[e4315] * self[e4315])),
        )
    }
}
impl AntiFix for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       10        0        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = 2.0 * (self[e4] * self[e5]) - self[e1] * self[e1] - self[e2] * self[e2] - self[e3] * self[e3];
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e5
            geometric_anti_product_g0 * self[e5],
        )
    }
}
impl AntiFix for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        0        0        4
    //    simd2        0        1        1      N/A
    // Totals...
    // yes simd        0        1        1      N/A
    //  no simd        0        2        2        4
    fn anti_fix(self) -> Self {
        RoundPointAtOrigin::from_groups(/* e4, e5 */ Simd32x2::powf(self.group0(), 0.5) * Simd32x2::from(2.0) / Simd32x2::powf(self.group0().yx(), 0.5))
    }
}
impl AntiFix for Scalar {
    fn anti_fix(self) -> Self {
        Scalar::from_groups(/* scalar */ -1.0)
    }
}
impl AntiFix for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       10        0        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        let geometric_anti_product_g0 = self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125] - 2.0 * (self[e3215] * self[e1234]);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e1234
            geometric_anti_product_g0 * self[e1234],
        )
    }
}
impl AntiFix for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        0        0        4
    //    simd2        0        1        1      N/A
    // Totals...
    // yes simd        0        1        1      N/A
    //  no simd        0        2        2        4
    fn anti_fix(self) -> Self {
        SphereAtOrigin::from_groups(
            // e3215, e1234
            Simd32x2::powf(self.group0(), 0.5) * Simd32x2::from(-2.0) / Simd32x2::powf(self.group0().yx(), 0.5),
        )
    }
}
impl AntiFix for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       11        0        0
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2       14        0      N/A
    //  no simd        8       23        0        0
    fn anti_fix(self) -> Self {
        use crate::elements::*;
        SphereOnOrigin::from_groups(
            // e4235, e4315, e4125, e1234
            (Simd32x4::from([self[e4235] * self[e4235], self[e4315] * self[e4315], self[e4125] * self[e4125], self[e4235] * self[e4235]]) * self.group0())
                + (Simd32x4::from([self[e4315] * self[e4315], self[e4235] * self[e4235], self[e4235] * self[e4235], self[e4315] * self[e4315]]) * self.group0())
                + (self.group0() * Simd32x2::from(self[e4125] * self[e4125]).with_zw(self[e4315] * self[e4315], self[e4125] * self[e4125])),
        )
    }
}
