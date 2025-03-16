// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 24
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       3       0
//  Average:         1       3       0
//  Maximum:         6       7       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         6       7       0
//  Average:         4       6       0
//  Maximum:        12      18       3
impl Fix for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        0        1
    // no simd        0        0        3
    fn fix(self) -> Self {
        AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ (self.group0().xyz() / self.group0().www()).with_w(1.0))
    }
}
impl Fix for AntiFlatOrigin {
    fn fix(self) -> Self {
        AntiFlatOrigin::from_groups(/* e321 */ 1.0)
    }
}
impl Fix for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        0        1
    // no simd        0        0        3
    fn fix(self) -> Self {
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ (self.group0().xyz() / self.group0().www()).with_w(1.0))
    }
}
impl Fix for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6        6        0
    //  no simd       12       12        0
    fn fix(self) -> Self {
        use crate::elements::*;
        AntiFlectorOnOrigin::from_groups(
            // e321, e1, e2, e3
            Simd32x4::from([
                (self[e3] * self[e3] * self[e321]) - f32::powi(self[e321], 3),
                f32::powi(self[e1], 3) - (self[e321] * self[e321] * self[e1]),
                f32::powi(self[e2], 3) - (self[e321] * self[e321] * self[e2]),
                f32::powi(self[e3], 3) - (self[e321] * self[e321] * self[e3]),
            ]) + (Simd32x4::powi(self.group0().yzwy(), 2) * self.group0())
                + (Simd32x4::powi(self.group0().zwyz(), 2) * self.group0()),
        )
    }
}
impl Fix for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        6        0
    // no simd        6       18        0
    fn fix(self) -> Self {
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        AntiLineOnOrigin::from_groups(
            // e23, e31, e12
            -(self.group0() * self.group0() * reverse_g0) - (self.group0() * reverse_g0.yxx() * self.group0().yxx()) - (self.group0() * reverse_g0.zzy() * self.group0().zzy()),
        )
    }
}
impl Fix for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6        6        0
    //  no simd       12       12        0
    fn fix(self) -> Self {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self[scalar] * self[scalar] * self[e23]) - f32::powi(self[e23], 3),
                (self[scalar] * self[scalar] * self[e31]) - f32::powi(self[e31], 3),
                (self[scalar] * self[scalar] * self[e12]) - f32::powi(self[e12], 3),
                f32::powi(self[scalar], 3) - (self[e12] * self[e12] * self[scalar]),
            ]) - (Simd32x4::powi(self.group0().yzxx(), 2) * self.group0())
                - (Simd32x4::powi(self.group0().zxyy(), 2) * self.group0()),
        )
    }
}
impl Fix for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        8        9        0
    fn fix(self) -> Self {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([f32::powi(self[e1], 3), f32::powi(self[e2], 3), f32::powi(self[e3], 3), self[e3] * self[e3] * self[e5]])
                + (Simd32x4::powi(self.group0().yxxx(), 2) * self.group0())
                + (Simd32x4::powi(self.group0().zzyy(), 2) * self.group0()),
        )
    }
}
impl Fix for AntiPlaneOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        2        0
    // no simd        6        6        0
    fn fix(self) -> Self {
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::powi(self.group0(), 3) + (Simd32x3::powi(self.group0().yxx(), 2) * self.group0()) + (Simd32x3::powi(self.group0().zzy(), 2) * self.group0()),
        )
    }
}
impl Fix for AntiScalar {
    fn fix(self) -> Self {
        AntiScalar::from_groups(/* e12345 */ -1.0)
    }
}
impl Fix for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        8        9        0
    fn fix(self) -> Self {
        use crate::elements::*;
        AntiSphereOnOrigin::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([f32::powi(self[e1], 3), f32::powi(self[e2], 3), f32::powi(self[e3], 3), self[e3] * self[e3] * self[e4]])
                + (Simd32x4::powi(self.group0().yxxx(), 2) * self.group0())
                + (Simd32x4::powi(self.group0().zzyy(), 2) * self.group0()),
        )
    }
}
impl Fix for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        1
    // no simd        0        3        3
    fn fix(self) -> Self {
        DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ (self.group0().xyz() * Simd32x3::from(-1.0) / self.group0().www()).with_w(-1.0))
    }
}
impl Fix for FlatOrigin {
    fn fix(self) -> Self {
        FlatOrigin::from_groups(/* e45 */ -1.0)
    }
}
impl Fix for FlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        1
    // no simd        0        3        3
    fn fix(self) -> Self {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ (self.group0().xyz() * Simd32x3::from(-1.0) / self.group0().www()).with_w(-1.0))
    }
}
impl Fix for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6        6        0
    //  no simd       12       12        0
    fn fix(self) -> Self {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from([
                f32::powi(self[e45], 3) - (self[e4125] * self[e4125] * self[e45]),
                (self[e45] * self[e45] * self[e4235]) - f32::powi(self[e4235], 3),
                (self[e45] * self[e45] * self[e4315]) - f32::powi(self[e4315], 3),
                (self[e45] * self[e45] * self[e4125]) - f32::powi(self[e4125], 3),
            ]) - (Simd32x4::powi(self.group0().yzwy(), 2) * self.group0())
                - (Simd32x4::powi(self.group0().zwyz(), 2) * self.group0()),
        )
    }
}
impl Fix for LineOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        6        0
    // no simd        6       18        0
    fn fix(self) -> Self {
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        LineOnOrigin::from_groups(
            // e415, e425, e435
            (self.group0() * self.group0() * reverse_g0) + (self.group0() * reverse_g0.yxx() * self.group0().yxx()) + (self.group0() * reverse_g0.zzy() * self.group0().zzy()),
        )
    }
}
impl Fix for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6        6        0
    //  no simd       12       12        0
    fn fix(self) -> Self {
        use crate::elements::*;
        MotorOnOrigin::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                f32::powi(self[e415], 3) - (self[e12345] * self[e12345] * self[e415]),
                f32::powi(self[e425], 3) - (self[e12345] * self[e12345] * self[e425]),
                f32::powi(self[e435], 3) - (self[e12345] * self[e12345] * self[e435]),
                (self[e435] * self[e435] * self[e12345]) - f32::powi(self[e12345], 3),
            ]) + (Simd32x4::powi(self.group0().yzxx(), 2) * self.group0())
                + (Simd32x4::powi(self.group0().zxyy(), 2) * self.group0()),
        )
    }
}
impl Fix for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        8       13        0
    fn fix(self) -> Self {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                f32::powi(self[e4235], 3) * -1.0,
                f32::powi(self[e4315], 3) * -1.0,
                f32::powi(self[e4125], 3) * -1.0,
                self[e4125] * self[e4125] * self[e3215] * -1.0,
            ]) - (Simd32x4::powi(self.group0().yxxx(), 2) * self.group0())
                - (Simd32x4::powi(self.group0().zzyy(), 2) * self.group0()),
        )
    }
}
impl Fix for PlaneOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        2        0
    // no simd        6        6        0
    fn fix(self) -> Self {
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            -Simd32x3::powi(self.group0(), 3) - (Simd32x3::powi(self.group0().yxx(), 2) * self.group0()) - (Simd32x3::powi(self.group0().zzy(), 2) * self.group0()),
        )
    }
}
impl Fix for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        7        0
    fn fix(self) -> Self {
        use crate::elements::*;
        let geometric_product_g0 = self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] - 2.0 * (self[e4] * self[e5]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_product_g0) * self.group0(),
            // e5
            geometric_product_g0 * self[e5],
        )
    }
}
impl Fix for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn fix(self) -> Self {
        RoundPointAtOrigin::from_groups(/* e4, e5 */ Simd32x2::powf(self.group0(), 0.5) * Simd32x2::from(-2.0) / Simd32x2::powf(self.group0().yx(), 0.5))
    }
}
impl Fix for Scalar {
    fn fix(self) -> Self {
        Scalar::from_groups(/* scalar */ 1.0)
    }
}
impl Fix for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        7        0
    fn fix(self) -> Self {
        use crate::elements::*;
        let geometric_product_g0 = 2.0 * (self[e3215] * self[e1234]) - self[e4235] * self[e4235] - self[e4315] * self[e4315] - self[e4125] * self[e4125];
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(geometric_product_g0) * self.group0(),
            // e1234
            geometric_product_g0 * self[e1234],
        )
    }
}
impl Fix for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn fix(self) -> Self {
        SphereAtOrigin::from_groups(
            // e3215, e1234
            Simd32x2::powf(self.group0(), 0.5) * Simd32x2::from(2.0) / Simd32x2::powf(self.group0().yx(), 0.5),
        )
    }
}
impl Fix for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        8       13        0
    fn fix(self) -> Self {
        use crate::elements::*;
        SphereOnOrigin::from_groups(
            // e4235, e4315, e4125, e1234
            Simd32x4::from([
                f32::powi(self[e4235], 3) * -1.0,
                f32::powi(self[e4315], 3) * -1.0,
                f32::powi(self[e4125], 3) * -1.0,
                self[e4125] * self[e4125] * self[e1234] * -1.0,
            ]) - (Simd32x4::powi(self.group0().yxxx(), 2) * self.group0())
                - (Simd32x4::powi(self.group0().zzyy(), 2) * self.group0()),
        )
    }
}
