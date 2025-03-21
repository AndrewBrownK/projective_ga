// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 59
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0     N/A
//   Median:         3      10       0     N/A
//  Average:         7      15       0     N/A
//  Maximum:       126     207       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0       0
//   Median:         9      18       0       0
//  Average:        16      27       0       0
//  Maximum:       279     308       0       0
impl ConstraintViolation for AntiCircleOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ 2.0 * (self[e41] * self[e23]) + 2.0 * (self[e42] * self[e31]) + 2.0 * (self[e43] * self[e12]))
    }
}
impl ConstraintViolation for AntiCircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       10        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd        6       16        0      N/A
    //  no simd       15       32        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(2.0) * (self.group2().yzxy() * self.group0().zxy().with_w(self[e31]))
                + Simd32x3::from(0.0).with_w(2.0 * (self[e23] * self[e15]) + 2.0 * (self[e12] * self[e35]))
                - Simd32x4::from(2.0) * (Simd32x3::from(self[e45]) * self.group1().xyz()).with_w(0.0)
                - Simd32x4::from(2.0) * (self.group0().yzx() * self.group2().zxy()).with_w(0.0),
            // e1234
            2.0 * (self[e41] * self[e23]) + 2.0 * (self[e42] * self[e31]) + 2.0 * (self[e43] * self[e12]),
        )
    }
}
impl ConstraintViolation for AntiCircleRotorAligningOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       10        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        5       14        0      N/A
    //  no simd       11       25        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(2.0) * (self.group2().yzxy() * self.group0().zxy().with_w(self[e31]))
                + Simd32x3::from(0.0).with_w(2.0 * (self[e23] * self[e15]) + 2.0 * (self[e12] * self[e35]))
                - Simd32x4::from(2.0) * (self.group0().yzx() * self.group2().zxy()).with_w(0.0),
            // e1234
            2.0 * (self[e41] * self[e23]) + 2.0 * (self[e42] * self[e31]) + 2.0 * (self[e43] * self[e12]),
        )
    }
}
impl ConstraintViolation for AntiCircleRotorAligningOriginAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ 2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35]))
    }
}
impl ConstraintViolation for AntiCircleRotorAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2       12        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            self[e23] * self[e45] * -2.0,
            self[e31] * self[e45] * -2.0,
            self[e12] * self[e45] * -2.0,
            2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35]),
        ]))
    }
}
impl ConstraintViolation for AntiCircleRotorOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ 2.0 * (self[e41] * self[e23]) + 2.0 * (self[e42] * self[e31]) + 2.0 * (self[e43] * self[e12]))
    }
}
impl ConstraintViolation for AntiDipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       16        0        0
    //    simd3        0        1        0      N/A
    //    simd4        7       13        0      N/A
    // Totals...
    // yes simd       13       30        0      N/A
    //  no simd       34       71        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(2.0) * (Simd32x4::from(self[e5]) * self.group0().with_w(self[e321]))
                + Simd32x4::from(2.0) * (self.group2().zxyx() * self.group0().yzx().with_w(self[e1]))
                + Simd32x4::from(2.0) * (self.group3().yzxy() * self.group1().zxy().with_w(self[e315]))
                + Simd32x3::from(0.0).with_w((self[e125] * self[e3]) * 2.0)
                - Simd32x4::from(2.0) * (Simd32x4::from([self[e412], self[e423], self[e431], self[e415]]) * self.group2().yzxx())
                - Simd32x4::from(2.0) * (self.group1().xyxy() * Simd32x2::from(self[e321]).with_zw(self[e2], self[e315]))
                - Simd32x4::from(2.0) * (self.group1().yzzz() * self.group3().zx().with_zw(self[e321], self[e125]))
                - Simd32x4::from(2.0) * (Simd32x3::from(self[e4]) * self.group2().xyz()).with_w(0.0),
            // e1234
            -2.0 * (self[e423] * self[e415])
                - 2.0 * (self[e423] * self[e1])
                - 2.0 * (self[e431] * self[e425])
                - 2.0 * (self[e431] * self[e2])
                - 2.0 * (self[e412] * self[e435])
                - 2.0 * (self[e412] * self[e3])
                - 2.0 * (self[e321] * self[e4]),
        )
    }
}
impl ConstraintViolation for AntiDipoleInversionAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd4        3        6        0      N/A
    // Totals...
    // yes simd        6       14        0      N/A
    //  no simd       15       32        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(2.0) * (self.group2().yzxz() * self.group0().zxy().with_w(self[e125]))
                + Simd32x3::from(0.0).with_w(2.0 * (self[e321] * self[e5]) + 2.0 * (self[e235] * self[e1]) + 2.0 * (self[e315] * self[e2]) - 2.0 * (self[e435] * self[e125]))
                - Simd32x4::from(2.0) * (self.group0().xyxx() * Simd32x2::from(self[e321]).with_zw(self[e2], self[e235]))
                - Simd32x4::from(2.0) * (self.group0().yzzy() * self.group2().zx().with_zw(self[e321], self[e315])),
        )
    }
}
impl ConstraintViolation for AntiDipoleInversionOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(
            // e1234
            -2.0 * (self[e423] * self[e1]) - 2.0 * (self[e431] * self[e2]) - 2.0 * (self[e412] * self[e3]) - 2.0 * (self[e321] * self[e4]),
        )
    }
}
impl ConstraintViolation for AntiDipoleInversionOrthogonalOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        8        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        6        0      N/A
    // Totals...
    // yes simd        6       16        0      N/A
    //  no simd       18       38        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(2.0) * (self.group0().xyx() * Simd32x2::from(self[e5]).with_z(self[e315])).with_w(0.0)
                + Simd32x4::from(2.0) * (self.group0().yzz() * self.group2().zx().with_z(self[e5])).with_w(0.0)
                + Simd32x3::from(0.0).with_w((self[e435] * self[e125]) * -2.0)
                - Simd32x4::from(2.0) * (self.group2().xyzy() * Simd32x3::from(self[e4]).with_w(self[e425]))
                - Simd32x4::from(2.0) * (self.group2().yzxx() * self.group0().zxy().with_w(self[e415])),
            // e1234
            -2.0 * (self[e423] * self[e415]) - 2.0 * (self[e431] * self[e425]) - 2.0 * (self[e412] * self[e435]),
        )
    }
}
impl ConstraintViolation for AntiDualNum {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ self[scalar] * self[e1234] * 2.0)
    }
}
impl ConstraintViolation for AntiFlector {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(
            // e3215
            2.0 * (self[e235] * self[e1]) + 2.0 * (self[e315] * self[e2]) + 2.0 * (self[e125] * self[e3]) + 2.0 * (self[e321] * self[e5]),
        )
    }
}
impl ConstraintViolation for AntiLine {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ 2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35]))
    }
}
impl ConstraintViolation for AntiMotor {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(
            // e3215
            2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35]) + 2.0 * (self[scalar] * self[e3215]),
        )
    }
}
impl ConstraintViolation for AntiMysteryCircleRotor {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        6        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::from([self[e23] * self[e45] * -2.0, self[e31] * self[e45] * -2.0, self[e12] * self[e45] * -2.0]),
        )
    }
}
impl ConstraintViolation for AntiMysteryDipoleInversion {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        6        0      N/A
    // no simd        6       18        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::from(2.0) * (self.group1().yzx() * self.group0().zxy())
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e321]) * self.group0().xyz())
                - Simd32x3::from(2.0) * (self.group1().zxy() * self.group0().yzx()),
        )
    }
}
impl ConstraintViolation for AntiVersorEvenOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(
            // e1234
            2.0 * (self[e41] * self[e23]) + 2.0 * (self[e42] * self[e31]) + 2.0 * (self[e43] * self[e12]) + 2.0 * (self[scalar] * self[e1234]),
        )
    }
}
impl ConstraintViolation for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        9        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd        5       15        0      N/A
    //  no simd       14       31        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(2.0) * (self.group0().yzx() * self.group2().zxy()).with_w(0.0) + Simd32x3::from(0.0).with_w((self[e435] * self[e125]) * -2.0)
                - Simd32x4::from(2.0) * (Simd32x4::from([self[e321], self[e321], self[e321], self[e235]]) * self.group1().xyzx())
                - Simd32x4::from(2.0) * (self.group0().zxy() * self.group2().yzx()).with_w(self[e425] * self[e315]),
            // e1234
            -2.0 * (self[e423] * self[e415]) - 2.0 * (self[e431] * self[e425]) - 2.0 * (self[e412] * self[e435]),
        )
    }
}
impl ConstraintViolation for CircleAligningOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       11        0        0
    //    simd3        0        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        5       15        0      N/A
    //  no simd       11       25        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(2.0) * (self.group0().yzx() * self.group2().zxy()).with_w(0.0)
                + Simd32x3::from(0.0).with_w(-2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]))
                - Simd32x4::from(2.0) * (self.group0().zxy() * self.group2().yzx()).with_w(self[e415] * self[e235]),
            // e1234
            -2.0 * (self[e423] * self[e415]) - 2.0 * (self[e431] * self[e425]) - 2.0 * (self[e412] * self[e435]),
        )
    }
}
impl ConstraintViolation for CircleAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2       12        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            self[e415] * self[e321] * -2.0,
            self[e425] * self[e321] * -2.0,
            self[e435] * self[e321] * -2.0,
            -2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]),
        ]))
    }
}
impl ConstraintViolation for CircleAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        3        6        0      N/A
    // no simd        9       18        0        0
    fn constraint_violation(self) -> Self::Output {
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            (reverse_g0.zxy() * self.group1().yzx()) + (reverse_g1.yzx() * self.group0().zxy())
                - (reverse_g0.yzx() * self.group1().zxy())
                - (reverse_g1.zxy() * self.group0().yzx()),
        )
    }
}
impl ConstraintViolation for CircleOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ -2.0 * (self[e423] * self[e415]) - 2.0 * (self[e431] * self[e425]) - 2.0 * (self[e412] * self[e435]))
    }
}
impl ConstraintViolation for CircleOrthogonalOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        3        5        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        9       19        0        0
    fn constraint_violation(self) -> Self::Output {
        let reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            (reverse_g1.yzx() * self.group0().zxy()) + (self.group1().yzx() * reverse_g0.zxy())
                - (reverse_g1.zxy() * self.group0().yzx())
                - (self.group1().zxy() * reverse_g0.yzx()),
        )
    }
}
impl ConstraintViolation for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        8        0        0
    //    simd3        0        1        0      N/A
    //    simd4        3        5        0      N/A
    // Totals...
    // yes simd        5       14        0      N/A
    //  no simd       14       31        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(2.0) * (self.group0().yzx() * self.group2().zxy()).with_w(0.0) + Simd32x3::from(0.0).with_w((self[e435] * self[e125]) * -2.0)
                - Simd32x4::from(2.0) * (Simd32x4::from([self[e412], self[e423], self[e431], self[e415]]) * self.group2().yzxx())
                - Simd32x4::from(2.0) * (Simd32x4::from([self[e321], self[e321], self[e321], self[e315]]) * self.group1().xyzy()),
            // e1234
            -2.0 * (self[e423] * self[e415]) - 2.0 * (self[e431] * self[e425]) - 2.0 * (self[e412] * self[e435]),
        )
    }
}
impl ConstraintViolation for CircleRotorAligningOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       10        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        5       14        0      N/A
    //  no simd       11       25        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(2.0) * (self.group0().yzx() * self.group2().zxy()).with_w(0.0)
                + Simd32x3::from(0.0).with_w(-2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]))
                - Simd32x4::from(2.0) * (Simd32x4::from([self[e412], self[e423], self[e431], self[e415]]) * self.group2().yzxx()),
            // e1234
            -2.0 * (self[e423] * self[e415]) - 2.0 * (self[e431] * self[e425]) - 2.0 * (self[e412] * self[e435]),
        )
    }
}
impl ConstraintViolation for CircleRotorAligningOriginAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ -2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]))
    }
}
impl ConstraintViolation for CircleRotorAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2       12        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            self[e415] * self[e321] * -2.0,
            self[e425] * self[e321] * -2.0,
            self[e435] * self[e321] * -2.0,
            -2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]),
        ]))
    }
}
impl ConstraintViolation for CircleRotorOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ -2.0 * (self[e423] * self[e415]) - 2.0 * (self[e431] * self[e425]) - 2.0 * (self[e412] * self[e435]))
    }
}
impl ConstraintViolation for Dipole {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       11        0        0
    //    simd3        0        3        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd        6       17        0      N/A
    //  no simd       15       32        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(2.0) * (self.group0().zxy() * self.group2().yzx()).with_w(self[e12] * self[e35])
                + Simd32x3::from(0.0).with_w(2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]))
                - Simd32x4::from(2.0) * (Simd32x3::from(self[e45]) * self.group1().xyz()).with_w(0.0)
                - Simd32x4::from(2.0) * (self.group0().yzx() * self.group2().zxy()).with_w(0.0),
            // e1234
            2.0 * (self[e41] * self[e23]) + 2.0 * (self[e42] * self[e31]) + 2.0 * (self[e43] * self[e12]),
        )
    }
}
impl ConstraintViolation for DipoleAligningOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        3        5        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        9       19        0        0
    fn constraint_violation(self) -> Self::Output {
        let reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            (reverse_g1.zxy() * self.group0().yzx()) + (self.group1().zxy() * reverse_g0.yzx())
                - (reverse_g1.yzx() * self.group0().zxy())
                - (self.group1().yzx() * reverse_g0.zxy()),
        )
    }
}
impl ConstraintViolation for DipoleAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2       12        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            self[e23] * self[e45] * -2.0,
            self[e31] * self[e45] * -2.0,
            self[e12] * self[e45] * -2.0,
            2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35]),
        ]))
    }
}
impl ConstraintViolation for DipoleAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        3        6        0      N/A
    // no simd        9       18        0        0
    fn constraint_violation(self) -> Self::Output {
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            (reverse_g0.yzx() * self.group1().zxy()) + (reverse_g1.zxy() * self.group0().yzx())
                - (reverse_g0.zxy() * self.group1().yzx())
                - (reverse_g1.yzx() * self.group0().zxy()),
        )
    }
}
impl ConstraintViolation for DipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       22        0        0
    //    simd3        0        4        0      N/A
    //    simd4        7       10        0      N/A
    // Totals...
    // yes simd       16       36        0      N/A
    //  no simd       37       74        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(2.0) * (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * self.group2().yzxx())
                + Simd32x4::from(2.0) * (self.group1().zxyy() * self.group3().yzx().with_w(self[e25]))
                + Simd32x4::from(2.0) * (self.group2().xyzz() * Simd32x3::from(self[e1234]).with_w(self[e12]))
                + Simd32x3::from(0.0).with_w(2.0 * (self[e45] * self[e3215]) + 2.0 * (self[e15] * self[e4235]) + 2.0 * (self[e25] * self[e4315]) + 2.0 * (self[e35] * self[e4125]))
                - Simd32x4::from(2.0) * (self.group0().xyx() * Simd32x2::from(self[e3215]).with_z(self[e25])).with_w(0.0)
                - Simd32x4::from(2.0) * (self.group0().yzz() * self.group2().zx().with_z(self[e3215])).with_w(0.0)
                - Simd32x4::from(2.0) * (self.group1().xyx() * Simd32x2::from(self[e45]).with_z(self[e4315])).with_w(0.0)
                - Simd32x4::from(2.0) * (self.group1().yzz() * self.group3().zx().with_z(self[e45])).with_w(0.0),
            // e1234
            2.0 * (self[e41] * self[e23]) + 2.0 * (self[e42] * self[e31]) + 2.0 * (self[e43] * self[e12])
                - 2.0 * (self[e41] * self[e4235])
                - 2.0 * (self[e42] * self[e4315])
                - 2.0 * (self[e43] * self[e4125])
                - 2.0 * (self[e45] * self[e1234]),
        )
    }
}
impl ConstraintViolation for DipoleInversionAligningOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       12        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        6        0      N/A
    // Totals...
    // yes simd        8       20        0      N/A
    //  no simd       20       42        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(2.0) * (Simd32x4::from([self[e1234], self[e1234], self[e1234], self[e4235]]) * self.group1().xyzx())
                + Simd32x4::from(2.0) * (self.group0().zxyw() * self.group1().yzx().with_w(self[e3215]))
                + Simd32x3::from(0.0).with_w(2.0 * (self[e25] * self[e4315]) + 2.0 * (self[e35] * self[e4125]))
                - Simd32x4::from(2.0) * (self.group0().xyx() * Simd32x2::from(self[e3215]).with_z(self[e25])).with_w(0.0)
                - Simd32x4::from(2.0) * (self.group0().yzz() * self.group1().zx().with_z(self[e3215])).with_w(0.0),
            // e1234
            -2.0 * (self[e41] * self[e4235]) - 2.0 * (self[e42] * self[e4315]) - 2.0 * (self[e43] * self[e4125]) - 2.0 * (self[e45] * self[e1234]),
        )
    }
}
impl ConstraintViolation for DipoleInversionAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       12        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd        8       18        0      N/A
    //  no simd       17       34        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(2.0) * (self.group0().zxyy() * self.group2().yzx().with_w(self[e25]))
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e23] * self[e15])
                        + 2.0 * (self[e12] * self[e35])
                        + 2.0 * (self[e45] * self[e3215])
                        + 2.0 * (self[e15] * self[e4235])
                        + 2.0 * (self[e25] * self[e4315])
                        + 2.0 * (self[e35] * self[e4125]),
                )
                - Simd32x4::from(2.0) * (self.group0().xyx() * Simd32x2::from(self[e45]).with_z(self[e4315])).with_w(0.0)
                - Simd32x4::from(2.0) * (self.group0().yzz() * self.group2().zx().with_z(self[e45])).with_w(0.0),
        )
    }
}
impl ConstraintViolation for DipoleInversionAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4        7        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd       12       25        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::from(2.0) * (Simd32x3::from(self[e1234]) * self.group1().xyz())
                + Simd32x3::from(2.0) * (self.group0().zxy() * self.group1().yzx())
                + Simd32x2::from(0.0).with_z((self[e41] * self[e25]) * -2.0)
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e3215]) * self.group0().xyz())
                - Simd32x3::from(2.0) * (self.group0().yz() * self.group1().zx()).with_z(0.0),
        )
    }
}
impl ConstraintViolation for DipoleInversionOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(
            // e1234
            -2.0 * (self[e41] * self[e4235]) - 2.0 * (self[e42] * self[e4315]) - 2.0 * (self[e43] * self[e4125]) - 2.0 * (self[e45] * self[e1234]),
        )
    }
}
impl ConstraintViolation for DipoleInversionOrthogonalOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        8        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        6        0      N/A
    // Totals...
    // yes simd        6       16        0      N/A
    //  no simd       18       38        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(2.0) * (self.group2().xyzx() * Simd32x3::from(self[e1234]).with_w(self[e23]))
                + Simd32x4::from(2.0) * (self.group2().yzxz() * self.group0().zxy().with_w(self[e12]))
                + Simd32x3::from(0.0).with_w((self[e31] * self[e25]) * 2.0)
                - Simd32x4::from(2.0) * (self.group0().xyx() * Simd32x2::from(self[e3215]).with_z(self[e25])).with_w(0.0)
                - Simd32x4::from(2.0) * (self.group0().yzz() * self.group2().zx().with_z(self[e3215])).with_w(0.0),
            // e1234
            2.0 * (self[e41] * self[e23]) + 2.0 * (self[e42] * self[e31]) + 2.0 * (self[e43] * self[e12]),
        )
    }
}
impl ConstraintViolation for DipoleOrthogonalOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       11        0        0
    //    simd3        0        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        5       15        0      N/A
    //  no simd       11       25        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(2.0) * (self.group0().zxy() * self.group2().yzx()).with_w(self[e12] * self[e35])
                + Simd32x3::from(0.0).with_w(2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]))
                - Simd32x4::from(2.0) * (self.group0().yzx() * self.group2().zxy()).with_w(0.0),
            // e1234
            2.0 * (self[e41] * self[e23]) + 2.0 * (self[e42] * self[e31]) + 2.0 * (self[e43] * self[e12]),
        )
    }
}
impl ConstraintViolation for DualNum {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ self[e12345] * self[e4] * -2.0)
    }
}
impl ConstraintViolation for Flector {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(
            // e3215
            2.0 * (self[e15] * self[e4235]) + 2.0 * (self[e25] * self[e4315]) + 2.0 * (self[e35] * self[e4125]) + 2.0 * (self[e45] * self[e3215]),
        )
    }
}
impl ConstraintViolation for Line {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ -2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]))
    }
}
impl ConstraintViolation for Motor {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(
            // e3215
            -2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]) - 2.0 * (self[e12345] * self[e5]),
        )
    }
}
impl ConstraintViolation for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       75      172        0        0
    //    simd3        0        4        0      N/A
    //    simd4       51       31        0      N/A
    // Totals...
    // yes simd      126      207        0      N/A
    //  no simd      279      308        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g3 = self.group3() * Simd32x4::from(-1.0);
        let reverse_g6 = self.group6() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                2.0 * (self[e1] * self[e4235])
                    + 2.0 * (self[e3] * self[e4125])
                    + 2.0 * (self[e4] * self[e3215])
                    + 2.0 * (self[e5] * self[e1234])
                    + (self[scalar] * self[e12345])
                    + (self[e2] * self[e4315])
                    + (self[e41] * self[e235])
                    + (self[e42] * self[e315])
                    + (self[e43] * self[e125])
                    + 2.0 * (self[e15] * self[e423])
                    + 2.0 * (self[e25] * self[e431])
                    + 2.0 * (self[e35] * self[e412])
                    + (self[e23] * self[e415])
                    + (self[e31] * self[e425])
                    + (self[e12] * self[e435])
                    - (reverse_g3[0] * self[e235])
                    - (reverse_g3[1] * self[e315])
                    - (reverse_g3[2] * self[e125])
                    - (reverse_g3[3] * self[e321])
                    - (reverse_g6[0] * self[e23])
                    - (reverse_g6[1] * self[e31])
                    - (reverse_g6[2] * self[e12])
                    - (reverse_g6[3] * self[e45]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (Simd32x4::from([self[e3215], self[e3215], self[e3215], self[e12] * self[e412]]) * self.group7().with_w(1.0))
                + Simd32x4::from(2.0) * (self.group4().zxy() * self.group7().yzx()).with_w(self[e31] * self[e431])
                + (Simd32x4::from([reverse_g3[2], reverse_g3[0], reverse_g3[1], self[e23] * self[e423]]) * self.group8().yzx().with_w(1.0))
                + (Simd32x4::from([reverse_g6[2], reverse_g6[0], reverse_g6[1], self[e12345]]) * self.group9().zwyx())
                + (Simd32x4::from([self[e5], self[e5], self[e5], self[e2]]) * self.group3().xyzy())
                + (self.group1() * Simd32x3::from(self[scalar]).with_w(reverse_g3[3]))
                + (self.group1().yzxw() * self.group5().zxy().with_w(self[scalar]))
                + (self.group3().yzxz() * self.group8().zxy().with_w(self[e3]))
                + (self.group6().yzxw() * self.group9().wyzx())
                + Simd32x3::from(0.0).with_w(self[e1] * self[e41])
                + (Simd32x3::from(reverse_g6[3]) * self.group5()).with_w(0.0)
                - Simd32x4::from([0.0, self[e12345] * self[e4315], reverse_g3[0] * self[e315], reverse_g3[0] * self[e1]])
                - Simd32x4::from([0.0, self[e41] * self[e125], self[e42] * self[e235], reverse_g3[1] * self[e425]])
                - Simd32x4::from([self[e12345] * self[e4235], self[e1] * self[e12], self[e2] * self[e23], 0.0])
                - Simd32x4::from([self[e3] * self[e31], 0.0, 0.0, reverse_g3[1] * self[e2]])
                - Simd32x4::from([self[e43] * self[e315], reverse_g3[2] * self[e235], reverse_g3[3] * self[e435], 0.0])
                - Simd32x4::from([self[e435] * self[e4315], 0.0, 0.0, reverse_g6[2] * self[e43]])
                - (reverse_g6 * Simd32x3::from(self[e45]).with_w(self[e1234]))
                - (Simd32x4::from(self[e4]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                - (Simd32x4::from([reverse_g3[1], self[e1234], self[e1234], reverse_g3[2] * self[e3]]) * self.group8().zyz().with_w(1.0))
                - (Simd32x4::from([reverse_g6[1], self[e415], self[e12345], self[e431]]) * self.group9().wwwz())
                - (Simd32x4::from([self[e235], reverse_g6[2], reverse_g6[0], self[e412]]) * self.group9())
                - (self.group6().xywx() * Simd32x2::from(reverse_g3[3]).with_zw(self[e12], reverse_g3[0]))
                - (self.group6().wwyz() * self.group5().xy().with_zw(self[e4235], reverse_g3[2]))
                - Simd32x3::from(0.0).with_w(reverse_g6[1] * self[e42])
                - Simd32x3::from(0.0).with_w(self[e423] * self[e4235])
                - (Simd32x3::from(self[e5]) * reverse_g3.xyz()).with_w(0.0)
                - (self.group4().yzx() * self.group7().zxy()).with_w(reverse_g6[0] * self[e41]),
            // e5
            2.0 * (self[scalar] * self[e5])
                + 2.0 * (self[e12345] * self[e3215])
                + (reverse_g6[3] * self[e3215])
                + (self[e5] * self[e45])
                + (self[e15] * self[e415])
                + (self[e25] * self[e425])
                + (self[e35] * self[e435])
                + 2.0 * (self[e23] * self[e235])
                + 2.0 * (self[e31] * self[e315])
                + 2.0 * (self[e12] * self[e125])
                + 2.0 * (self[e235] * self[e4235])
                + 2.0 * (self[e315] * self[e4315])
                + 2.0 * (self[e125] * self[e4125])
                - (reverse_g3[3] * self[e5])
                - (reverse_g6[0] * self[e15])
                - (reverse_g6[1] * self[e25])
                - (reverse_g6[2] * self[e35])
                - 2.0 * (self[e1] * self[e15])
                - 2.0 * (self[e2] * self[e25])
                - 2.0 * (self[e3] * self[e35])
                - (self[e321] * self[e3215]),
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([reverse_g3[1] * self[e4315], 0.0, reverse_g6[1] * self[e321], reverse_g3[0] * self[e25]])
                + Simd32x4::from([reverse_g6[2] * self[e412], self[e12345] * self[e1], self[e12345] * self[e2], 0.0])
                + Simd32x4::from([self[e41] * self[e23], reverse_g3[0] * self[e3215], reverse_g3[1] * self[e3215], 0.0])
                + Simd32x4::from([self[e42] * self[e31], reverse_g3[3] * self[e23], 0.0, self[e5] * self[e412]])
                + Simd32x4::from([self[e43] * self[e12], reverse_g6[0] * self[e321], reverse_g3[3] * self[e31], 0.0])
                + Simd32x4::from([
                    -(reverse_g3[0] * self[e23])
                        - (reverse_g3[1] * self[e31])
                        - (reverse_g3[2] * self[e12])
                        - (self[e12345] * self[e4])
                        - (self[e4] * self[e321])
                        - (self[e435] * self[e412]),
                    2.0 * (self[e5] * self[e423]) + 2.0 * (self[e431] * self[e125]) + (self[e43] * self[e25]),
                    2.0 * (self[e5] * self[e431]) + 2.0 * (self[e412] * self[e235]) + (self[e41] * self[e35]),
                    2.0 * (self[e423] * self[e315]) + (reverse_g3[2] * self[e3215]) + (reverse_g6[2] * self[e321]) + (self[e1] * self[e425]) + (self[e42] * self[e15]),
                ])
                + (reverse_g3 * Simd32x4::from([self[e4235], self[e35], self[e15], self[e12]]))
                + (reverse_g6 * Simd32x4::from([self[e423], self[e3], self[e1], self[e435]]))
                + (Simd32x4::from(self[e1234]) * Simd32x4::from([self[scalar], self[e15], self[e25], self[e35]]))
                + (Simd32x4::from([reverse_g3[2], self[e12], self[e23], self[e31]]) * self.group9().wzwy())
                + (Simd32x4::from([reverse_g3[3], self[scalar], self[scalar], self[scalar]]) * self.group9())
                + (Simd32x4::from([reverse_g6[3], self[e435], self[e415], self[e12345]]) * self.group1().wyzz())
                + (Simd32x4::from([self[e431], self[e415], self[e425], self[e2]]) * reverse_g6.ywwx())
                - Simd32x4::from([0.0, self[e3] * self[e425], reverse_g3[0] * self[e35], reverse_g3[1] * self[e15]])
                - Simd32x4::from([0.0, self[e4] * self[e235], self[e4] * self[e315], reverse_g6[1] * self[e1]])
                - Simd32x4::from([0.0, self[e41] * self[e3215], self[e42] * self[e3215], self[e41] * self[e25]])
                - Simd32x4::from([self[e2] * self[e431], reverse_g3[2] * self[e25], self[e1] * self[e435], 0.0])
                - Simd32x4::from([self[e3] * self[e412], 0.0, 0.0, self[e2] * self[e415]])
                - Simd32x4::from([self[e45] * self[e1234], 0.0, 0.0, 0.0])
                - Simd32x4::from([self[e425] * self[e431], 0.0, 0.0, 0.0])
                - (Simd32x4::from([self[e43], self[e31], self[e12], self[e23]]) * self.group9().wwyz())
                - (Simd32x4::from([self[e415], self[e315], self[e125], self[e431] * self[e235]]) * self.group7().xzx().with_w(1.0))
                - (Simd32x4::from([self[e423], reverse_g6[2], reverse_g6[0], self[e125]]) * self.group1())
                - (Simd32x4::from([self[e4235], self[e35], self[e15], self[e12]]) * self.group3())
                - (Simd32x4::from([self[e4315], self[e23], self[e31], self[e3215]]) * self.group3().ywwz()),
            // e3215
            2.0 * (self[scalar] * self[e3215])
                + (reverse_g6[0] * self[e235])
                + (reverse_g6[1] * self[e315])
                + (reverse_g6[2] * self[e125])
                + 2.0 * (self[e1] * self[e235])
                + 2.0 * (self[e2] * self[e315])
                + 2.0 * (self[e3] * self[e125])
                + (self[e5] * self[e321])
                + (self[e45] * self[e3215])
                + 2.0 * (self[e15] * self[e23])
                + 2.0 * (self[e15] * self[e4235])
                + 2.0 * (self[e25] * self[e31])
                + 2.0 * (self[e25] * self[e4315])
                + 2.0 * (self[e35] * self[e12])
                + 2.0 * (self[e35] * self[e4125])
                - (reverse_g3[3] * self[e3215])
                - (reverse_g6[3] * self[e5])
                - (self[e415] * self[e235])
                - (self[e425] * self[e315])
                - (self[e435] * self[e125])
                - 2.0 * (self[e12345] * self[e5]),
        )
    }
}
impl ConstraintViolation for MysteryCircle {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        6        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::from([self[e415] * self[e321] * -2.0, self[e425] * self[e321] * -2.0, self[e435] * self[e321] * -2.0]),
        )
    }
}
impl ConstraintViolation for MysteryCircleRotor {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        6        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::from([self[e415] * self[e321] * -2.0, self[e425] * self[e321] * -2.0, self[e435] * self[e321] * -2.0]),
        )
    }
}
impl ConstraintViolation for MysteryDipole {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        6        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::from([self[e23] * self[e45] * -2.0, self[e31] * self[e45] * -2.0, self[e12] * self[e45] * -2.0]),
        )
    }
}
impl ConstraintViolation for MysteryDipoleInversion {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        6        0      N/A
    // no simd        6       18        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::from(2.0) * (self.group1().yzx() * self.group0().zxy())
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e45]) * self.group0().xyz())
                - Simd32x3::from(2.0) * (self.group1().zxy() * self.group0().yzx()),
        )
    }
}
impl ConstraintViolation for MysteryVersorEven {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        6        0      N/A
    // Totals...
    // yes simd        5       12        0      N/A
    //  no simd       13       26        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::from(2.0) * (Simd32x3::from(self[e12345]) * self.group0().yzw())
                + Simd32x3::from(2.0) * (self.group0().zw() * self.group1().zx()).with_z(0.0)
                + Simd32x2::from(0.0).with_z(2.0 * (self[e1] * self[e425]) - 2.0 * (self[e2] * self[e415]))
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e321]) * self.group1().xyz())
                - Simd32x3::from(2.0) * (self.group0().wy() * self.group1().yz()).with_z(0.0),
        )
    }
}
impl ConstraintViolation for MysteryVersorOdd {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        6        0      N/A
    // Totals...
    // yes simd        5       12        0      N/A
    //  no simd       13       26        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::from(2.0) * (Simd32x3::from(self[scalar]) * self.group0().yzw())
                + Simd32x3::from(2.0) * (self.group0().zw() * self.group1().zx()).with_z(0.0)
                + Simd32x2::from(0.0).with_z(2.0 * (self[e4235] * self[e31]) - 2.0 * (self[e4315] * self[e23]))
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e45]) * self.group1().xyz())
                - Simd32x3::from(2.0) * (self.group0().wy() * self.group1().yz()).with_z(0.0),
        )
    }
}
impl ConstraintViolation for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       14       34        0        0
    //    simd3        0        1        0      N/A
    //    simd4       14        7        0      N/A
    // Totals...
    // yes simd       28       42        0      N/A
    //  no simd       70       65        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e431] * self[e125], reverse_g0[0] * self[e125], reverse_g0[1] * self[e235], 0.0])
                + Simd32x4::from([self[e435] * self[e2], self[e415] * self[e3], self[e425] * self[e1], 0.0])
                + (Simd32x4::from(self[e5]) * self.group0().xyz().with_w(self[e321]))
                + (Simd32x4::from([reverse_g0[2], self[e412], self[e423], self[e1]]) * self.group2().yxyx())
                + (self.group3().xyzy() * Simd32x3::from(self[e12345]).with_w(self[e315]))
                + Simd32x3::from(0.0).with_w(self[e125] * self[e3])
                + (Simd32x3::from(reverse_g0[3]) * self.group3().xyz()).with_w(0.0)
                - Simd32x4::from([reverse_g0[1] * self[e125], reverse_g0[2] * self[e235], reverse_g0[0] * self[e315], 0.0])
                - Simd32x4::from([self[e415] * self[e321], self[e425] * self[e321], self[e415] * self[e2], 0.0])
                - Simd32x4::from([self[e425] * self[e3], self[e435] * self[e1], self[e435] * self[e321], 0.0])
                - (reverse_g0 * Simd32x4::from(self[e5]))
                - (self.group2() * Simd32x3::from(self[e4]).with_w(self[e12345]))
                - (self.group2().yzxy() * self.group0().zxy().with_w(self[e425]))
                - Simd32x3::from(0.0).with_w(self[e415] * self[e235])
                - Simd32x3::from(0.0).with_w(self[e435] * self[e125]),
            // e1234
            (reverse_g0[0] * self[e415])
                + (reverse_g0[0] * self[e1])
                + (reverse_g0[1] * self[e425])
                + (reverse_g0[1] * self[e2])
                + (reverse_g0[2] * self[e435])
                + (reverse_g0[2] * self[e3])
                - (reverse_g0[3] * self[e4])
                - (self[e423] * self[e415])
                - (self[e423] * self[e1])
                - (self[e431] * self[e425])
                - (self[e431] * self[e2])
                - (self[e412] * self[e435])
                - (self[e412] * self[e3])
                - (self[e12345] * self[e4])
                - 2.0 * (self[e321] * self[e4]),
        )
    }
}
impl ConstraintViolation for VersorEvenAligningOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       10        0        0
    //    simd3        0        1        0      N/A
    //    simd4        8        9        0      N/A
    // Totals...
    // yes simd       15       20        0      N/A
    //  no simd       39       49        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let reverse_g2 = self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (reverse_g2.xyxx() * Simd32x2::from(self[e4]).with_zw(self[e431], self[e415]))
                + (reverse_g2.yzzy() * self.group0().zx().with_zw(self[e4], self[e425]))
                + (reverse_g2.wwwz() * self.group0().xyz().with_w(self[e435]))
                + Simd32x3::from(0.0).with_w((self[e435] * self[e125]) * -1.0)
                + (reverse_g0.zxy() * self.group2().yzx()).with_w(0.0)
                - (reverse_g0 * Simd32x4::from(self[e5]))
                - (Simd32x4::from([reverse_g0[1], reverse_g0[2], reverse_g0[0], self[e415]]) * self.group2().zxyx())
                - (reverse_g2.zxyw() * self.group0().yzxw())
                - (self.group1().wwwy() * self.group2().xyzy()),
            // e1234
            (reverse_g0[0] * self[e415]) + (reverse_g0[1] * self[e425]) + (reverse_g0[2] * self[e435])
                - (reverse_g0[3] * self[e4])
                - (self[e423] * self[e415])
                - (self[e431] * self[e425])
                - (self[e412] * self[e435])
                - (self[e12345] * self[e4]),
        )
    }
}
impl ConstraintViolation for VersorEvenAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       12        0        0
    //    simd4        5        5        0      N/A
    // Totals...
    // yes simd        8       17        0      N/A
    //  no simd       23       32        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e2] * self[e435], self[e3] * self[e415], self[e1] * self[e425], 0.0])
                + (self.group0().xxxz() * self.group0().yzw().with_w(self[e315]))
                + Simd32x3::from(0.0).with_w(self[e1] * self[e235])
                + Simd32x3::from(0.0).with_w(2.0 * (self[e3] * self[e125]) + 2.0 * (self[e321] * self[e5]) - 2.0 * (self[e12345] * self[e5]) - 2.0 * (self[e435] * self[e125]))
                - Simd32x4::from(2.0) * (Simd32x4::from([self[e321], self[e321], self[e321], self[e315]]) * self.group1().xyzy())
                - Simd32x4::from(2.0) * (self.group1().yzxx() * self.group0().wyz().with_w(self[e235])),
        )
    }
}
impl ConstraintViolation for VersorEvenAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4        7        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd       12       25        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::from(2.0) * (Simd32x3::from(self[e5]) * self.group0().xyz())
                + Simd32x3::from(2.0) * (self.group0().yzx() * self.group1().zxy())
                + Simd32x2::from(0.0).with_z((self[e431] * self[e235]) * -2.0)
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e4]) * self.group1().xyz())
                - Simd32x3::from(2.0) * (self.group0().zx() * self.group1().yz()).with_z(0.0),
        )
    }
}
impl ConstraintViolation for VersorEvenOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(
            // e1234
            -2.0 * (self[e423] * self[e415]) - 2.0 * (self[e431] * self[e425]) - 2.0 * (self[e412] * self[e435]) - 2.0 * (self[e12345] * self[e4]),
        )
    }
}
impl ConstraintViolation for VersorEvenOrthogonalOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       11        0        0
    //    simd3        0        3        0      N/A
    //    simd4        8        7        0      N/A
    // Totals...
    // yes simd       15       21        0      N/A
    //  no simd       39       48        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (reverse_g1 * Simd32x3::from(self[e4]).with_w(self[e321]))
                + (Simd32x4::from([reverse_g0[2], reverse_g0[0], reverse_g0[1], self[e1]]) * self.group1().yzxx())
                + Simd32x3::from(0.0).with_w(self[e125] * self[e3])
                + (Simd32x3::from(reverse_g1[3]) * self.group0().xyz()).with_w(0.0)
                + (reverse_g1.yzx() * self.group0().zxy()).with_w(self[e315] * self[e2])
                - (reverse_g0 * Simd32x4::from(self[e5]))
                - (reverse_g1.zxyx() * self.group0().yzx().with_w(self[e1]))
                - (self.group2().wwwy() * self.group1().xyz().with_w(reverse_g1[1]))
                - (reverse_g0.yzx() * self.group1().zxy()).with_w(reverse_g1[2] * self[e3]),
            // e1234
            (reverse_g0[0] * self[e1]) + (reverse_g0[1] * self[e2]) + (reverse_g0[2] * self[e3]) + (reverse_g0[3] * self[e4])
                - (self[e423] * self[e1])
                - (self[e431] * self[e2])
                - (self[e412] * self[e3])
                - (self[e321] * self[e4]),
        )
    }
}
impl ConstraintViolation for VersorOdd {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       16       37        0        0
    //    simd3        0        5        0      N/A
    //    simd4       14        6        0      N/A
    // Totals...
    // yes simd       30       48        0      N/A
    //  no simd       72       76        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, self[e41] * self[e35], self[e42] * self[e15], self[e12] * self[e35]])
                + Simd32x4::from([reverse_g0[1] * self[e35], self[e25] * self[e1234], self[e35] * self[e1234], 0.0])
                + Simd32x4::from([self[e43] * self[e25], 0.0, 0.0, self[e31] * self[e25]])
                + Simd32x4::from([self[e12] * self[e4315], reverse_g0[2] * self[e15], reverse_g0[0] * self[e25], 0.0])
                + Simd32x4::from([self[e15] * self[e1234], self[e23] * self[e4125], self[e31] * self[e4235], 0.0])
                + (reverse_g0 * Simd32x4::from(self[e3215]))
                + (self.group3() * Simd32x3::from(reverse_g0[3]).with_w(self[scalar]))
                + (self.group3() * Simd32x3::from(self[scalar]).with_w(self[e45]))
                + Simd32x3::from(0.0).with_w(self[e23] * self[e15])
                + Simd32x3::from(0.0).with_w(2.0 * (self[e15] * self[e4235]) + 2.0 * (self[e25] * self[e4315]) + 2.0 * (self[e35] * self[e4125]))
                - (reverse_g0.zxy() * self.group2().yzx()).with_w(0.0)
                - (self.group0().xyx() * Simd32x2::from(self[e3215]).with_z(self[e25])).with_w(0.0)
                - (self.group0().yzz() * self.group2().zx().with_z(self[e3215])).with_w(0.0)
                - Simd32x4::from(2.0) * (self.group1().xyx() * Simd32x2::from(self[e45]).with_z(self[e4315])).with_w(0.0)
                - Simd32x4::from(2.0) * (self.group1().yzz() * self.group3().zx().with_z(self[e45])).with_w(0.0),
            // e1234
            (reverse_g0[0] * self[e4235])
                + (reverse_g0[1] * self[e4315])
                + (reverse_g0[2] * self[e4125])
                + (reverse_g0[3] * self[e1234])
                + (self[e41] * self[e23])
                + (self[e42] * self[e31])
                + (self[e43] * self[e12])
                + (self[scalar] * self[e1234])
                - (reverse_g0[0] * self[e23])
                - (reverse_g0[1] * self[e31])
                - (reverse_g0[2] * self[e12])
                - (self[e41] * self[e4235])
                - (self[e42] * self[e4315])
                - (self[e43] * self[e4125])
                - 2.0 * (self[e45] * self[e1234]),
        )
    }
}
impl ConstraintViolation for VersorOddAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       12        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        6        0      N/A
    // Totals...
    // yes simd        9       20        0      N/A
    //  no simd       21       42        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(2.0) * (Simd32x4::from(self[scalar]) * self.group2())
                + Simd32x4::from(2.0) * (self.group1().zxyy() * self.group2().yzx().with_w(self[e25]))
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e15] * self[e23])
                        + 2.0 * (self[e15] * self[e4235])
                        + 2.0 * (self[e25] * self[e4315])
                        + 2.0 * (self[e35] * self[e12])
                        + 2.0 * (self[e35] * self[e4125])
                        + 2.0 * (self[e45] * self[e3215]),
                )
                - Simd32x4::from(2.0) * (Simd32x3::from([self[e45], self[e45], self[e4315]]) * self.group1().xyx()).with_w(0.0)
                - Simd32x4::from(2.0) * (Simd32x3::from([self[e4125], self[e4235], self[e45]]) * self.group1().yzz()).with_w(0.0),
        )
    }
}
impl ConstraintViolation for VersorOddOrthogonalOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       18        0        0
    //    simd3        0        3        0      N/A
    //    simd4        8        3        0      N/A
    // Totals...
    // yes simd       15       24        0      N/A
    //  no simd       39       39        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e43] * self[e25], reverse_g0[2] * self[e15], reverse_g0[0] * self[e25], 0.0])
                + Simd32x4::from([self[e15] * self[e1234], self[e25] * self[e1234], 0.0, self[scalar] * self[e3215]])
                + (reverse_g0 * Simd32x4::from(self[e3215]))
                + (Simd32x4::from(self[e35]) * Simd32x4::from([reverse_g0[1], self[e41], self[e1234], self[e12]]))
                + Simd32x2::from(0.0).with_zw(self[e42] * self[e15], self[e23] * self[e15])
                + Simd32x3::from(0.0).with_w((self[e31] * self[e25]) * 2.0)
                - (Simd32x3::from([self[e3215], self[e3215], self[e25]]) * self.group0().xyx()).with_w(0.0)
                - (Simd32x3::from([self[e35], self[e15], self[e3215]]) * self.group0().yzz()).with_w(0.0)
                - (reverse_g0.zxy() * self.group2().yzx()).with_w(0.0),
            // e1234
            (reverse_g0[3] * self[e1234]) + (self[e41] * self[e23]) + (self[e42] * self[e31]) + (self[e43] * self[e12]) + (self[scalar] * self[e1234])
                - (reverse_g0[0] * self[e23])
                - (reverse_g0[1] * self[e31])
                - (reverse_g0[2] * self[e12]),
        )
    }
}
