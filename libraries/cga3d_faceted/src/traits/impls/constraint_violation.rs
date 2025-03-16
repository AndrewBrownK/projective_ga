// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 55
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:         3      10       0
//  Average:         8      19       0
//  Maximum:       115     210       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:         7      14       0
//  Average:        13      25       0
//  Maximum:       163     275       0
impl ConstraintViolation for AntiCircleOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ -2.0 * (self[e41] * self[e23]) - 2.0 * (self[e42] * self[e31]) - 2.0 * (self[e43] * self[e12]))
    }
}
impl ConstraintViolation for AntiCircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       30        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                2.0 * (self[e42] * self[e35]) + 2.0 * (self[e23] * self[e45]) - (self.group2().yzx()[0] * self[e43]) - (self[e43] * self[e25]),
                2.0 * (self[e43] * self[e15]) + 2.0 * (self[e31] * self[e45]) - (self.group2().yzx()[1] * self[e41]) - (self[e41] * self[e35]),
                2.0 * (self[e41] * self[e25]) + 2.0 * (self[e12] * self[e45]) - (self.group2().yzx()[2] * self[e42]) - (self[e42] * self[e15]),
                -2.0 * (self[e23] * self[e15]) - 2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]),
            ]),
            // e1234
            -2.0 * (self[e41] * self[e23]) - 2.0 * (self[e42] * self[e31]) - 2.0 * (self[e43] * self[e12]),
        )
    }
}
impl ConstraintViolation for AntiCircleRotorAligningOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       24        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                2.0 * (self[e42] * self[e35]) - (self.group2().yzx()[0] * self[e43]) - (self[e43] * self[e25]),
                2.0 * (self[e43] * self[e15]) - (self.group2().yzx()[1] * self[e41]) - (self[e41] * self[e35]),
                2.0 * (self[e41] * self[e25]) - (self.group2().yzx()[2] * self[e42]) - (self[e42] * self[e15]),
                -2.0 * (self[e23] * self[e15]) - 2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]),
            ]),
            // e1234
            -2.0 * (self[e41] * self[e23]) - 2.0 * (self[e42] * self[e31]) - 2.0 * (self[e43] * self[e12]),
        )
    }
}
impl ConstraintViolation for AntiCircleRotorAligningOriginAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ -2.0 * (self[e23] * self[e15]) - 2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]))
    }
}
impl ConstraintViolation for AntiCircleRotorAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2       10        0
    //  no simd        2       13        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e23] * self[e45],
                self[e31] * self[e45],
                self[e12] * self[e45],
                -2.0 * (self[e23] * self[e15]) - 2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]),
            ]) * Simd32x4::from([2.0, 2.0, 2.0, 1.0]),
        )
    }
}
impl ConstraintViolation for AntiCircleRotorOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ -2.0 * (self[e41] * self[e23]) - 2.0 * (self[e42] * self[e31]) - 2.0 * (self[e43] * self[e12]))
    }
}
impl ConstraintViolation for AntiDipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       42        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       25       44        0
    //  no simd       31       50        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                2.0 * (self[e412] * self[e315]) + 2.0 * (self[e415] * self[e321]) - 2.0 * (self[e431] * self[e125]),
                2.0 * (self[e423] * self[e125]) + 2.0 * (self[e425] * self[e321]) - 2.0 * (self[e412] * self[e235]),
                (self.group1().xyxy()[2] * self[e2])
                    + (self.group1().xyzx()[2] * self.group1().wwww()[2])
                    + (self.group1().yzzz()[2] * self[e321])
                    + (self.group2().xyxy()[2] * self[e431])
                    + (self.group2().yzzz()[2] * self[e4])
                    + (self[e431] * self[e235])
                    - (self[e415] * self[e2])
                    - (self[e125] * self[e4])
                    - 2.0 * (self[e423] * self[e315]),
                (self.group1().xyzx()[3] * self[e235])
                    + (self.group1().yzzz()[3] * self[e125])
                    + (self.group2().xyxy()[3] * self[e425])
                    + (self.group2().yzzz()[3] * self[e435])
                    + (self.group3().yzxx()[3] * self[e235])
                    + (self[e415] * self[e235])
                    + (self[e315] * self[e2])
                    + (self[e125] * self[e3])
                    - (self.group3().yzxy()[3] * self[e315])
                    - (self[e235] * self[e1]),
            ]) + (Simd32x4::from([
                self.group1().zxy()[0],
                self.group1().zxy()[1],
                self.group1().zxy()[2] * self.group3().yzxx()[2],
                self.group1().xyxy()[3] * self[e315],
            ]) * self.group3().yz().with_zw(1.0, 1.0))
                - (Simd32x4::from([
                    self.group1().zxy()[0],
                    self.group1().zxy()[1],
                    self.group1().zxy()[2] * self.group3().yzxy()[2],
                    self.group2().zxyz()[3] * self[e3],
                ]) * self.group3().yz().with_zw(1.0, 1.0)),
            // e1234
            2.0 * (self[e423] * self[e415]) + 2.0 * (self[e431] * self[e425]) + 2.0 * (self[e412] * self[e435]),
        )
    }
}
impl ConstraintViolation for AntiDipoleInversionAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       16        0
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        5        4        0
    // Totals...
    // yes simd        7       22        0
    //  no simd       22       37        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self.group0().yzx()[0] * self.group2().zxy()[0] * -1.0,
                self.group0().yzx()[1] * self.group2().zxy()[1] * -1.0,
                reverse_g0.zxyw()[2] * self.group2().yzxw()[2] * -1.0,
                (self[e425] * self[e315]) + (self[e435] * self[e125]) + (self[e321] * self[e5]),
            ]) + (Simd32x4::from([
                self.group2().yzx()[0],
                self.group2().yzx()[1],
                self.group2().yzx()[2] * self.group0().zxyx()[2],
                reverse_g0.xyxx()[3] * self[e235],
            ]) * self.group0().zx().with_zw(1.0, 1.0))
                + (self.group0().ww().with_zw(self[e2], reverse_g0.yzzy()[3] * self[e315]) * reverse_g0.xyx().with_w(1.0))
                + (self.group2().zx().with_zw(self[e321], reverse_g0.wwwz()[3] * self[e125]) * reverse_g0.yzz().with_w(1.0))
                + (self.group0().xyz() * reverse_g0.www()).with_w(self.group0().zxyx()[3] * self[e235])
                - (reverse_g0.zx() * self.group2().yz()).with_zw(self.group0().yzx()[2] * self.group2().zxy()[2], reverse_g0.zxyw()[3] * self.group2().yzxw()[3]),
        )
    }
}
impl ConstraintViolation for AntiDipoleInversionOrthogonalOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       24        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       26        0
    //  no simd       15       32        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e412] * self[e315]) - (self[e235] * self[e4]) - 2.0 * (self[e431] * self[e125]),
                (self[e423] * self[e125]) - (self[e315] * self[e4]) - 2.0 * (self[e412] * self[e235]),
                self[e423] * self[e315] * -2.0,
                2.0 * (self[e425] * self[e315]) + 2.0 * (self[e435] * self[e125]),
            ]) + (Simd32x4::from([
                self.group0().zxy()[0],
                self.group0().zxy()[1],
                self.group0().zxy()[2] * self[e235],
                self.group2().yzzx()[3] * self[e415],
            ]) * self.group2().yz().with_zw(1.0, 1.0))
                + (Simd32x4::from([self.group2().xyx()[0], self.group2().xyx()[1], self.group2().xyx()[2] * self[e431], self[e415] * self[e235]])
                    * self.group2().ww().with_zw(1.0, 1.0)),
            // e1234
            2.0 * (self[e423] * self[e415]) + 2.0 * (self[e431] * self[e425]) + 2.0 * (self[e412] * self[e435]),
        )
    }
}
impl ConstraintViolation for AntiDualNum {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ (self[e1234] * self[scalar]) * 2.0)
    }
}
impl ConstraintViolation for AntiLine {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ -2.0 * (self[e23] * self[e15]) - 2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]))
    }
}
impl ConstraintViolation for AntiMotor {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(
            // e3215
            2.0 * (self[scalar] * self[e3215]) - 2.0 * (self[e23] * self[e15]) - 2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]),
        )
    }
}
impl ConstraintViolation for AntiMysteryCircleRotor {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::from([self[e23] * self[e45], self[e31] * self[e45], self[e12] * self[e45]]) * Simd32x3::from(2.0),
        )
    }
}
impl ConstraintViolation for AntiMysteryDipoleInversion {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        5        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        7        0
    //  no simd       15       22        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            (Simd32x3::from([self.group0().xwww()[1], self.group0().xwww()[2], self[e2]]) * reverse_g0.xxyx().yzw())
                + (self.group1().yzx() * self.group0().zxy())
                + (reverse_g0.yyzz().yzw() * self.group1().zx().with_z(self[e321]))
                + (reverse_g0.zwww().yzw() * self.group0().zxyz().yzw())
                - (self.group1().yzx() * reverse_g0.wzxy().yzw())
                - (self.group1().zxy() * self.group0().yzx()),
        )
    }
}
impl ConstraintViolation for AntiVersorEvenOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(
            // e1234
            2.0 * (self[scalar] * self[e1234]) - 2.0 * (self[e41] * self[e23]) - 2.0 * (self[e42] * self[e31]) - 2.0 * (self[e43] * self[e12]),
        )
    }
}
impl ConstraintViolation for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       15        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        5       19        0
    //  no simd       11       30        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(2.0) * (self.group1().xyzz() * self.group1().www().with_w(self[e125]))
                + Simd32x4::from(2.0) * (self.group0().zxy() * self.group2().yzx()).with_w(self[e425] * self[e315])
                + Simd32x4::from([
                    self[e431] * self[e125] * -2.0,
                    self[e412] * self[e235] * -2.0,
                    self[e423] * self[e315] * -2.0,
                    (self.group1().wwwx()[3] * self[e235]) + (self[e415] * self[e235]),
                ]),
            // e1234
            2.0 * (self[e423] * self[e415]) + 2.0 * (self[e431] * self[e425]) + 2.0 * (self[e412] * self[e435]),
        )
    }
}
impl ConstraintViolation for CircleAligningOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       17        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4       19        0
    //  no simd        7       24        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(2.0) * (self.group0().zxy() * self.group2().yzx()).with_w(self[e435] * self[e125])
                + Simd32x4::from([
                    self[e431] * self[e125] * -2.0,
                    self[e412] * self[e235] * -2.0,
                    self[e423] * self[e315] * -2.0,
                    2.0 * (self[e415] * self[e235]) + 2.0 * (self[e425] * self[e315]),
                ]),
            // e1234
            2.0 * (self[e423] * self[e415]) + 2.0 * (self[e431] * self[e425]) + 2.0 * (self[e412] * self[e435]),
        )
    }
}
impl ConstraintViolation for CircleAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2       10        0
    //  no simd        2       13        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e415] * self[e321],
                self[e425] * self[e321],
                self[e435] * self[e321],
                2.0 * (self[e415] * self[e235]) + 2.0 * (self[e425] * self[e315]) + 2.0 * (self[e435] * self[e125]),
            ]) * Simd32x4::from([2.0, 2.0, 2.0, 1.0]),
        )
    }
}
impl ConstraintViolation for CircleAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        4        0
    // no simd        3       12        0
    fn constraint_violation(self) -> Self::Output {
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::from(2.0) * (self.group0().zxy() * self.group1().yzx()) - Simd32x3::from(2.0) * (self.group0().yzx() * self.group1().zxy()),
        )
    }
}
impl ConstraintViolation for CircleOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ 2.0 * (self[e423] * self[e415]) + 2.0 * (self[e431] * self[e425]) + 2.0 * (self[e412] * self[e435]))
    }
}
impl ConstraintViolation for CircleOrthogonalOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        3        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        9       19        0
    fn constraint_violation(self) -> Self::Output {
        let reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            (reverse_g1.yzx() * self.group0().xzxy().yzw()) + (self.group1().yzx() * reverse_g0.xzxy().yzw())
                - (reverse_g1.zxy() * self.group0().wyzx().yzw())
                - (self.group1().zxy() * reverse_g0.yzx()),
        )
    }
}
impl ConstraintViolation for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       30        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                2.0 * (self[e412] * self[e315]) + 2.0 * (self[e415] * self[e321]) - 2.0 * (self[e431] * self[e125]),
                2.0 * (self[e423] * self[e125]) + 2.0 * (self[e425] * self[e321]) - 2.0 * (self[e412] * self[e235]),
                2.0 * (self[e431] * self[e235]) + 2.0 * (self[e435] * self[e321]) - 2.0 * (self[e423] * self[e315]),
                (self.group1().xyzy()[3] * self[e315])
                    + (self.group1().wwwz()[3] * self[e125])
                    + (self.group2().yzxx()[3] * self[e415])
                    + (self[e415] * self[e235])
                    + (self[e425] * self[e315])
                    + (self[e435] * self[e125]),
            ]),
            // e1234
            2.0 * (self[e423] * self[e415]) + 2.0 * (self[e431] * self[e425]) + 2.0 * (self[e412] * self[e435]),
        )
    }
}
impl ConstraintViolation for CircleRotorAligningOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       24        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group2().yzx()[0] * self[e412]) + (self[e412] * self[e315]) - 2.0 * (self[e431] * self[e125]),
                (self.group2().yzx()[1] * self[e423]) + (self[e423] * self[e125]) - 2.0 * (self[e412] * self[e235]),
                (self.group2().yzx()[2] * self[e431]) + (self[e431] * self[e235]) - 2.0 * (self[e423] * self[e315]),
                2.0 * (self[e435] * self[e125]) + 2.0 * (self[e415] * self[e235]) + 2.0 * (self[e425] * self[e315]),
            ]),
            // e1234
            2.0 * (self[e423] * self[e415]) + 2.0 * (self[e431] * self[e425]) + 2.0 * (self[e412] * self[e435]),
        )
    }
}
impl ConstraintViolation for CircleRotorAligningOriginAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ 2.0 * (self[e415] * self[e235]) + 2.0 * (self[e425] * self[e315]) + 2.0 * (self[e435] * self[e125]))
    }
}
impl ConstraintViolation for CircleRotorAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2       10        0
    //  no simd        2       13        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e415] * self[e321],
                self[e425] * self[e321],
                self[e435] * self[e321],
                2.0 * (self[e415] * self[e235]) + 2.0 * (self[e425] * self[e315]) + 2.0 * (self[e435] * self[e125]),
            ]) * Simd32x4::from([2.0, 2.0, 2.0, 1.0]),
        )
    }
}
impl ConstraintViolation for CircleRotorOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ 2.0 * (self[e423] * self[e415]) + 2.0 * (self[e431] * self[e425]) + 2.0 * (self[e412] * self[e435]))
    }
}
impl ConstraintViolation for Dipole {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       23        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       25        0
    //  no simd       10       30        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                2.0 * (self[e42] * self[e35]) + 2.0 * (self[e23] * self[e45]),
                2.0 * (self[e43] * self[e15]) + 2.0 * (self[e31] * self[e45]),
                2.0 * (self[e41] * self[e25]) + 2.0 * (self[e12] * self[e45]),
                -2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]),
            ]) - Simd32x4::from(2.0) * (self.group0().zxy() * self.group2().yzx()).with_w(self[e23] * self[e15]),
            // e1234
            -2.0 * (self[e41] * self[e23]) - 2.0 * (self[e42] * self[e31]) - 2.0 * (self[e43] * self[e12]),
        )
    }
}
impl ConstraintViolation for DipoleAligningOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        3        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        9       19        0
    fn constraint_violation(self) -> Self::Output {
        let reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            (reverse_g1.zxy() * self.group0().wyzx().yzw()) + (self.group1().zxy() * reverse_g0.yzx())
                - (reverse_g1.yzx() * self.group0().xzxy().yzw())
                - (self.group1().yzx() * reverse_g0.xzxy().yzw()),
        )
    }
}
impl ConstraintViolation for DipoleAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2       10        0
    //  no simd        2       13        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e23] * self[e45],
                self[e31] * self[e45],
                self[e12] * self[e45],
                -2.0 * (self[e23] * self[e15]) - 2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]),
            ]) * Simd32x4::from([2.0, 2.0, 2.0, 1.0]),
        )
    }
}
impl ConstraintViolation for DipoleAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        4        0
    // no simd        3       12        0
    fn constraint_violation(self) -> Self::Output {
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::from(2.0) * (self.group0().yzx() * self.group1().zxy()) - Simd32x3::from(2.0) * (self.group0().zxy() * self.group1().yzx()),
        )
    }
}
impl ConstraintViolation for DipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       44        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       29       46        0
    //  no simd       35       52        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                2.0 * (self[e42] * self[e35])
                    + (self.group1().xyx()[0] * self[e45])
                    + (self.group1().yzz()[0] * self[e4125])
                    + (self[e23] * self[e45])
                    + (self[e12] * self[e4315])
                    + (self[e15] * self[e1234])
                    - (self.group2().yzz()[0] * self[e43])
                    - (self.group3().yzx()[0] * self[e12])
                    - (self[e43] * self[e25]),
                2.0 * (self[e43] * self[e15])
                    + (self.group1().xyx()[1] * self[e45])
                    + (self.group1().yzz()[1] * self[e4235])
                    + (self[e23] * self[e4125])
                    + (self[e31] * self[e45])
                    + (self[e25] * self[e1234])
                    - (self.group2().yzz()[1] * self[e41])
                    - (self.group3().yzx()[1] * self[e23])
                    - (self[e41] * self[e35]),
                2.0 * (self[e41] * self[e25])
                    + (self.group1().xyx()[2] * self[e4315])
                    + (self.group1().yzz()[2] * self[e45])
                    + (self[e31] * self[e4235])
                    + (self[e12] * self[e45])
                    + (self[e35] * self[e1234])
                    - (self.group2().yzz()[2] * self[e1234])
                    - (self.group3().yzx()[2] * self.group1().zxyy()[2])
                    - (self[e42] * self[e15]),
                -2.0 * (self[e23] * self[e15]) - 2.0 * (self[e12] * self[e35]),
            ]) - (Simd32x4::from([
                self.group1().yzx()[0],
                self.group1().yzx()[1],
                self.group1().yzx()[2] * self[e4315],
                self.group1().zxyy()[3] * self[e25],
            ]) * self.group3().zx().with_zw(1.0, 1.0))
                - (Simd32x4::from([self.group2().xyx()[0], self.group2().xyx()[1], self.group2().xyx()[2] * self[e42], self[e31] * self[e25]])
                    * self.group2().ww().with_zw(1.0, 1.0)),
            // e1234
            -2.0 * (self[e41] * self[e23]) - 2.0 * (self[e42] * self[e31]) - 2.0 * (self[e43] * self[e12]),
        )
    }
}
impl ConstraintViolation for DipoleInversionAligningOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       16        0
    //    simd4        5        5        0
    // Totals...
    // yes simd        9       21        0
    //  no simd       24       36        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self.group0().yzx()[0] * self.group1().zxy()[0],
                self.group0().yzx()[1] * self.group1().zxy()[1],
                (self.group0().yzz()[2] * self[e3215]) + (self.group1().xyzz()[2] * self.group1().wwww()[2]) - (self[e35] * self[e1234]),
                (self[e15] * self[e4235]) + (self[e25] * self[e4315]) - (self.group2().wwwz()[3] * self[e35]),
            ]) + (Simd32x4::from([
                self.group0().xyx()[0],
                self.group0().xyx()[1],
                self.group0().xyx()[2] * self[e25],
                self.group1().xyzz()[3] * self[e4125],
            ]) * self.group2().ww().with_zw(1.0, 1.0))
                + (Simd32x4::from([self.group0().yzz()[0], self.group0().yzz()[1], self.group0().yzx()[2] * self.group1().zxy()[2], self[e45] * self[e3215]])
                    * self.group1().zx().with_zw(1.0, 1.0))
                - (Simd32x4::from([
                    self.group1().yzx()[0],
                    self.group1().yzx()[1],
                    self.group1().yzx()[2] * self.group0().zxyw()[2],
                    self.group0().zxyw()[3] * self[e3215],
                ]) * self.group0().zx().with_zw(1.0, 1.0))
                - (Simd32x4::from([self[e25], self[e3215], self[e3215], self.group1().yzzy()[3] * self[e4315]]) * self.group0().zyz().with_w(1.0))
                - (Simd32x4::from([self[e3215], self[e35], self[e15], self.group1().xyxx()[3] * self[e4235]]) * self.group0().xxy().with_w(1.0)),
            // e1234
            0.0,
        )
    }
}
impl ConstraintViolation for DipoleInversionAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       22        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       14       24        0
    //  no simd       20       30        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group0().yzz()[0] * self[e4125]) + (self[e23] * self[e45]) + (self[e12] * self[e4315]) - (self.group0().yzx()[0] * self.group2().zxy()[0]),
                (self.group0().yzz()[1] * self[e4235]) + (self[e23] * self[e4125]) + (self[e31] * self[e45]) - (self.group0().yzx()[1] * self.group2().zxy()[1]),
                (self.group0().yzz()[2] * self[e45]) + (self[e31] * self[e4235]) + (self[e12] * self[e45]) - (self.group2().yzx()[2] * self.group0().zxyx()[2]),
                -(self[e23] * self[e15]) - (self[e45] * self[e3215]) - 2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]),
            ]) + (Simd32x4::from([
                self.group0().xyx()[0],
                self.group0().xyx()[1],
                self.group0().xyx()[2] * self[e4315],
                self.group0().zxyw()[3] * self.group2().yzxw()[3],
            ]) * self.group0().ww().with_zw(1.0, 1.0))
                - (Simd32x4::from([
                    self.group2().yzx()[0],
                    self.group2().yzx()[1],
                    self.group0().yzx()[2] * self.group2().zxy()[2],
                    self.group0().zxyx()[3] * self[e15],
                ]) * self.group0().zx().with_zw(1.0, 1.0)),
        )
    }
}
impl ConstraintViolation for DipoleInversionAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       14        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ Simd32x3::from([
            (self.group0().wyzw()[1] * self.group1().wzzw()[1]) + (self[e42] * self[e35]) + (self[e15] * self[e1234])
                - (self.group0().xzx()[1] * self[e25])
                - (self.group1().zwzw()[1] * self[e15])
                - (self[e43] * self[e25]),
            2.0 * (self[e43] * self[e15]) - (self.group0().xzx()[2] * self[e35]) - (self[e41] * self[e35]),
            2.0 * (self[e41] * self[e25]) - 2.0 * (self[e42] * self[e15]),
        ]))
    }
}
impl ConstraintViolation for DipoleInversionOrthogonalOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       24        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       26        0
    //  no simd       15       32        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                2.0 * (self[e42] * self[e35]) + (self[e15] * self[e1234]) - (self[e43] * self[e25]),
                2.0 * (self[e43] * self[e15]) + (self[e25] * self[e1234]) - (self[e41] * self[e35]),
                self[e41] * self[e25] * 2.0,
                -2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]),
            ]) - (Simd32x4::from([self.group0().zxy()[0], self.group0().zxy()[1], self.group0().zxy()[2] * self[e15], self.group2().yzzx()[3] * self[e23]])
                * self.group2().yz().with_zw(1.0, 1.0))
                - (Simd32x4::from([self.group2().xyx()[0], self.group2().xyx()[1], self.group2().xyx()[2] * self[e42], self[e23] * self[e15]])
                    * self.group2().ww().with_zw(1.0, 1.0)),
            // e1234
            -2.0 * (self[e41] * self[e23]) - 2.0 * (self[e42] * self[e31]) - 2.0 * (self[e43] * self[e12]),
        )
    }
}
impl ConstraintViolation for DipoleOrthogonalOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       17        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4       19        0
    //  no simd        7       24        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e42] * self[e35] * 2.0,
                self[e43] * self[e15] * 2.0,
                self[e41] * self[e25] * 2.0,
                -2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]),
            ]) - Simd32x4::from(2.0) * (self.group0().zxy() * self.group2().yzx()).with_w(self[e23] * self[e15]),
            // e1234
            -2.0 * (self[e41] * self[e23]) - 2.0 * (self[e42] * self[e31]) - 2.0 * (self[e43] * self[e12]),
        )
    }
}
impl ConstraintViolation for DualNum {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(/* e1234 */ (self[e4] * self[e12345]) * -2.0)
    }
}
impl ConstraintViolation for Line {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ 2.0 * (self[e415] * self[e235]) + 2.0 * (self[e425] * self[e315]) + 2.0 * (self[e435] * self[e125]))
    }
}
impl ConstraintViolation for Motor {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(
            // e3215
            2.0 * (self[e415] * self[e235]) + 2.0 * (self[e425] * self[e315]) + 2.0 * (self[e435] * self[e125]) - 2.0 * (self[e12345] * self[e5]),
        )
    }
}
impl ConstraintViolation for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       99      188        0
    //    simd3        0        1        0
    //    simd4       16       21        0
    // Totals...
    // yes simd      115      210        0
    //  no simd      163      275        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                2.0 * (self[scalar] * self[e12345])
                    + 2.0 * (self[e1] * self[e4235])
                    + 2.0 * (self[e2] * self[e4315])
                    + 2.0 * (self[e3] * self[e4125])
                    + 2.0 * (self[e4] * self[e3215])
                    + 2.0 * (self[e5] * self[e1234])
                    - 2.0 * (self[e41] * self[e235])
                    - 2.0 * (self[e42] * self[e315])
                    - 2.0 * (self[e43] * self[e125])
                    - 2.0 * (self[e45] * self[e321])
                    - 2.0 * (self[e15] * self[e423])
                    - 2.0 * (self[e25] * self[e431])
                    - 2.0 * (self[e35] * self[e412])
                    - 2.0 * (self[e23] * self[e415])
                    - 2.0 * (self[e31] * self[e425])
                    - 2.0 * (self[e12] * self[e435]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (Simd32x4::from(self[scalar]) * self.group1())
                + Simd32x4::from([
                    2.0 * (self[e25] * self[e412])
                        + 2.0 * (self[e23] * self[e321])
                        + (self[e2] * self[e12])
                        + (self[e5] * self[e41])
                        + (self[e43] * self[e315])
                        + (self[e425] * self[e4125])
                        + (self[e435] * self[e4315])
                        - (self.group3().xyx()[0] * self[e5])
                        - (self.group6().zxy()[0] * self[e4315])
                        - (self.group9().xxx()[0] * self[e235])
                        - (self[e42] * self[e125])
                        - (self[e45] * self[e415]),
                    2.0 * (self[e35] * self[e423])
                        + 2.0 * (self[e31] * self[e321])
                        + (self[e3] * self[e23])
                        + (self[e5] * self[e42])
                        + (self[e41] * self[e125])
                        + (self[e415] * self[e4125])
                        + (self[e435] * self[e4235])
                        - (self.group3().xyx()[1] * self[e5])
                        - (self.group6().zxy()[1] * self[e4125])
                        - (self.group9().xxx()[1] * self[e315])
                        - (self[e43] * self[e235])
                        - (self[e45] * self[e425]),
                    (self.group4().yzz()[2] * self[e4])
                        + (self.group5().xyx()[2] * self[e2])
                        + (self.group5().yzz()[2] * self[e321])
                        + (self.group8().xyx()[2] * self[e42])
                        + (self.group3().zxy()[2] * self[e235])
                        + (self[e1] * self[e31])
                        + (self[e5] * self[e43])
                        + (self[e12] * self[e321])
                        + (self[e415] * self[e4315])
                        + (self[e425] * self[e4235])
                        + (self[e125] * self[e1234])
                        - (self.group6().yzz()[2] * self[e45])
                        - (self.group6().zxy()[2] * self.group9().zwyw()[2])
                        - (self.group9().xxx()[2] * self[e125])
                        - (self.group1().wwww()[2] * self[e35])
                        - (self.group3().yzzx()[2] * self[e5])
                        - (self.group3().wwwx()[2] * self.group6().xyzx()[2])
                        - (self[e2] * self[e23]),
                    2.0 * (self[e12345] * self[e1234]) - 2.0 * (self[e23] * self[e423]) - 2.0 * (self[e31] * self[e431]) - 2.0 * (self[e12] * self[e412]),
                ])
                + (Simd32x4::from([self.group9().xx()[0], self.group9().xx()[1], self.group4().xyx()[2] * self[e431], self[e1] * self[e41]])
                    * self.group8().xy().with_zw(1.0, 1.0))
                + (Simd32x4::from([self.group3().zxy()[0], self.group3().zxy()[1], self.group4().yzx()[2] * self.group7().zxy()[2], self[e3] * self[e43]])
                    * self.group8().yz().with_zw(1.0, 1.0))
                - (Simd32x4::from([self.group1().yzx()[0], self.group1().yzx()[1], self.group1().yzx()[2] * self[e31], self.group1().zxyz()[3] * self[e43]])
                    * self.group5().zx().with_zw(1.0, 1.0))
                - (Simd32x4::from([self.group3().yzx()[0], self.group3().yzx()[1], self.group3().xyx()[2] * self[e315], self.group3().yzzx()[3] * self[e1]])
                    * self.group8().zx().with_zw(1.0, 1.0))
                - (Simd32x4::from([
                    self.group6().xyx()[0],
                    self.group6().xyx()[1],
                    self.group3().yzx()[2] * self[e315],
                    self.group3().wwwx()[3] * self.group6().xyzx()[3],
                ]) * self.group3().ww().with_zw(1.0, 1.0))
                - (Simd32x4::from([self.group6().yzz()[0], self.group6().yzz()[1], self.group6().xyx()[2] * self[e4315], self[e41] * self[e415]])
                    * self.group9().wy().with_zw(1.0, 1.0))
                - Simd32x4::from(2.0) * (self.group0().yy().with_zw(self[e12345], self[e43] * self[e435]) * self.group9().yzw().with_w(1.0))
                - Simd32x4::from(2.0) * (self.group4().zxy() * self.group7().yzx()).with_w(self[e42] * self[e425]),
            // e5
            2.0 * (self[scalar] * self[e5]) + 2.0 * (self[e12345] * self[e3215])
                - 2.0 * (self[e15] * self[e415])
                - 2.0 * (self[e25] * self[e425])
                - 2.0 * (self[e35] * self[e435])
                - 2.0 * (self[e23] * self[e235])
                - 2.0 * (self[e31] * self[e315])
                - 2.0 * (self[e12] * self[e125]),
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
            Simd32x4::from(2.0) * (Simd32x4::from(self[scalar]) * self.group9())
                + Simd32x4::from(2.0) * (Simd32x4::from([self[e415], self[e315], self[e12345] * self[e2], self[e12345] * self[e3]]) * self.group7().xz().with_zw(1.0, 1.0))
                + Simd32x4::from([
                    2.0 * (self[e435] * self[e412]) + (self.group6().yx()[0] * self[e431])
                        - (self.group5().yy()[0] * self[e42])
                        - (self.group9().yx()[0] * self[e41])
                        - (self[e42] * self[e31])
                        - 2.0 * (self[e12345] * self[e4])
                        - 2.0 * (self[e43] * self[e12]),
                    2.0 * (self[e12345] * self[e1])
                        + 2.0 * (self[e42] * self[e35])
                        + 2.0 * (self[e45] * self[e23])
                        + (self.group7().yx()[1] * self[e5])
                        + (self[e31] * self[e4125])
                        + (self[e415] * self[e321])
                        - (self.group5().yy()[1] * self[e4125])
                        - (self.group1().yy()[1] * self[e435])
                        - (self.group9().yx()[1] * self[e15])
                        - 2.0 * (self[e431] * self[e125]),
                    2.0 * (self[e425] * self[e321]) + 2.0 * (self[e423] * self[e125]) + (self[e3] * self[e415]) + (self[e43] * self[e15]) + (self[e45] * self[e31])
                        - (self.group1().wwxy()[2] * self[e435])
                        - 2.0 * (self[e412] * self[e235]),
                    2.0 * (self[e41] * self[e25]) + 2.0 * (self[e435] * self[e321]) + 2.0 * (self[e431] * self[e235]) + (self[e2] * self[e415]) + (self[e45] * self[e12])
                        - (self.group1().wwxy()[3] * self[e415])
                        - (self[e43] * self[e3215])
                        - 2.0 * (self[e423] * self[e315]),
                ])
                + (Simd32x4::from([self.group7().yx()[0], self.group6().yx()[1], self.group3().yxzz()[2] * self[e15], self.group3().yxzz()[3] * self[e3215]])
                    * self.group6().yw().with_zw(1.0, 1.0))
                + (Simd32x4::from([self[e41], self[e15], self[e1] * self[e435], self[e1] * self[e425]]) * self.group9().yx().with_zw(1.0, 1.0))
                + (Simd32x4::from([self[e431], self[e435], self.group3().zyww()[2] * self[e31], self.group3().zyww()[3] * self[e12]]) * self.group1().yy().with_zw(1.0, 1.0))
                - (Simd32x4::from([self.group1().yy()[0], self[e5], self.group1().zzzx()[2] * self[e415], self.group1().zzzx()[3] * self[e425]])
                    * self.group7().yx().with_zw(1.0, 1.0))
                - Simd32x4::from(2.0) * (Simd32x4::from([self[e23], self[e25], self[e35], self[e15]]) * self.group3().xzxy()),
            // e3215
            2.0 * (self[scalar] * self[e3215]) + 2.0 * (self[e415] * self[e235]) + 2.0 * (self[e425] * self[e315]) + 2.0 * (self[e435] * self[e125])
                - 2.0 * (self[e15] * self[e23])
                - 2.0 * (self[e25] * self[e31])
                - 2.0 * (self[e35] * self[e12])
                - 2.0 * (self[e12345] * self[e5]),
        )
    }
}
impl ConstraintViolation for MysteryCircle {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        3       10        0
    fn constraint_violation(self) -> Self::Output {
        let reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            (reverse_g0.xxyz().yzw() * self.group0().xwww().yzw()) + (reverse_g0.ywww().yzw() * self.group0().yxyz().yzw()),
        )
    }
}
impl ConstraintViolation for MysteryCircleRotor {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        3       10        0
    fn constraint_violation(self) -> Self::Output {
        let reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            (reverse_g0.xxyz().yzw() * self.group0().xwww().yzw()) + (reverse_g0.ywww().yzw() * self.group0().yxyz().yzw()),
        )
    }
}
impl ConstraintViolation for MysteryDipole {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::from([self[e23] * self[e45], self[e31] * self[e45], self[e12] * self[e45]]) * Simd32x3::from(2.0),
        )
    }
}
impl ConstraintViolation for MysteryDipoleInversion {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        1        2        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        2        6        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            ((Simd32x2::from(self[e45]) * self.group0().xy()) + (self.group0().xy() * self.group0().wwww().yz())).with_z(self[e12] * self[e45] * 2.0),
        )
    }
}
impl ConstraintViolation for MysteryVersorEven {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        6        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        0
    //  no simd       18       28        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::from(2.0) * (Simd32x3::from(self[e12345]) * self.group0().yzw())
                + (Simd32x3::from([self.group1().xwww()[1], self.group1().xwww()[2], self[e2]]) * reverse_g1.xxyx().yzw())
                + (Simd32x3::from([self[e3], self[e1], self[e321]]) * reverse_g1.yyzz().yzw())
                + (self.group1().zxy() * self.group0().zwy())
                + (reverse_g1.zwww().yzw() * self.group1().zxyz().yzw())
                - (self.group1().yzx() * self.group0().wyz())
                - (self.group0().zwy() * reverse_g1.wzxy().yzw()),
        )
    }
}
impl ConstraintViolation for MysteryVersorOdd {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        5       12        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::from(2.0) * (Simd32x3::from(self[scalar]) * self.group0().yzw())
                + Simd32x3::from([
                    (self.group1().wwww()[1] * self[e23]) + (self[e23] * self[e45]),
                    (self.group1().wwww()[2] * self[e31]) + (self[e31] * self[e45]),
                    self[e12] * self[e45] * 2.0,
                ]),
        )
    }
}
impl ConstraintViolation for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       33        0
    //    simd3        0        1        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       20       42        0
    //  no simd       44       68        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(2.0) * (self.group3().xyz() * self.group0().www()).with_w(self[e415] * self[e235])
                + Simd32x4::from([
                    2.0 * (self[e415] * self[e321]) - (self.group1().zxy()[0] * self[e2]),
                    2.0 * (self[e425] * self[e321]) - (self.group1().zxy()[1] * self[e3]),
                    (self[e412] * self[e5]) + (self[e415] * self[e2]) + (self[e435] * self[e321]) - (self.group1().zxy()[2] * self[e1]) - (self[e423] * self[e315]),
                    (self.group3().yzxz()[3] * self[e125]) + (self[e425] * self[e315]) + (self[e435] * self[e125]) - 2.0 * (self[e12345] * self[e5]),
                ])
                + (Simd32x4::from([self.group0().zxy()[0], self.group0().zxy()[1], self.group0().zxy()[2], self.group1().xyzy()[3]]) * self.group2().yzxy())
                + (Simd32x4::from([
                    self.group1().yzz()[0],
                    self.group1().yzz()[1],
                    self.group1().yzz()[2] * self[e321],
                    self.group2().xyxz()[3] * self[e435],
                ]) * self.group3().zx().with_zw(1.0, 1.0))
                + (Simd32x4::from([
                    self.group1().zxy()[0],
                    self.group1().zxy()[1],
                    self.group1().zxy()[2] * self.group3().yzxz()[2],
                    self.group2().yzzw()[3] * self[e321],
                ]) * self.group3().yz().with_zw(1.0, 1.0))
                + (self.group2().yzxy() * self.group0().zxy().with_w(self.group3().xyzy()[3]))
                - (Simd32x4::from([
                    self.group0().yzx()[0],
                    self.group0().yzx()[1],
                    self.group0().yzx()[2] * self.group2().zxyy()[2],
                    self.group2().zxyy()[3] * self[e2],
                ]) * self.group2().zx().with_zw(1.0, 1.0))
                - (Simd32x4::from([self.group0().yzz()[0], self.group0().yzz()[1], self.group0().yzz()[2], self.group3().zxyz()[3]]) * self.group2().zxwz())
                - (Simd32x4::from([self.group1().yzx()[0], self.group1().yzx()[1], self.group1().yzx()[2] * self.group3().zxyz()[2], self[e321] * self[e5]])
                    * self.group3().zx().with_zw(1.0, 1.0)),
            // e1234
            2.0 * (self[e423] * self[e415]) + 2.0 * (self[e431] * self[e425]) + 2.0 * (self[e412] * self[e435]) - 2.0 * (self[e12345] * self[e4]),
        )
    }
}
impl ConstraintViolation for VersorEvenAligningOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       31        0
    //    simd3        0        1        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       17       35        0
    //  no simd       29       46        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group2().yzz()[0] * self[e412]) + (self[e423] * self[e5]) - (self[e431] * self[e125]) - (self[e4] * self[e235]),
                (self.group2().yzz()[1] * self[e423]) + (self[e431] * self[e5]) - (self[e412] * self[e235]) - (self[e4] * self[e315]),
                (self.group2().yzz()[2] * self[e4]) + (self[e412] * self[e5]) - (self[e423] * self[e315]) - (self[e4] * self[e125]),
                2.0 * (self[e425] * self[e315]) + 2.0 * (self[e435] * self[e125]),
            ]) + (Simd32x4::from([
                self.group0().zxy()[0],
                self.group0().zxy()[1],
                self.group0().zxy()[2] * self[e235],
                self.group2().wwwx()[3] * self[e415],
            ]) * self.group2().yz().with_zw(1.0, 1.0))
                + (Simd32x4::from([self.group2().xyx()[0], self.group2().xyx()[1], self.group2().xyx()[2] * self[e431], self[e415] * self[e235]])
                    * self.group1().ww().with_zw(1.0, 1.0))
                - (Simd32x4::from([
                    self.group0().yzz()[0],
                    self.group0().yzz()[1],
                    self.group0().yzz()[2] * self.group2().zxww()[2],
                    self.group0().xyxw()[3] * self.group2().wwyw()[3],
                ]) * self.group2().zx().with_zw(1.0, 1.0))
                - (self.group0().xyx() * self.group2().wwy()).with_w(self[e12345] * self[e5]),
            // e1234
            2.0 * (self[e423] * self[e415]) + 2.0 * (self[e431] * self[e425]) + 2.0 * (self[e412] * self[e435]) - 2.0 * (self[e12345] * self[e4]),
        )
    }
}
impl ConstraintViolation for VersorEvenAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       16        0
    //    simd2        0        1        0
    //    simd4        4        4        0
    // Totals...
    // yes simd        9       21        0
    //  no simd       21       34        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(2.0) * (self.group1().xyzx() * self.group1().www().with_w(self[e235]))
                + Simd32x4::from([
                    self[e3] * self[e425],
                    self[e1] * self[e435],
                    (self[e12345] * self[e3]) + (self[e2] * self[e415]) - (self[e1] * self[e425]),
                    2.0 * (self[e425] * self[e315]) + 2.0 * (self[e435] * self[e125]) - (self[e2] * self[e315]) - 2.0 * (self[e12345] * self[e5]),
                ])
                + (Simd32x4::from([
                    self.group0().xx()[0],
                    self.group0().xx()[1],
                    self.group0().xxxy()[2] * self.group0().yzww()[2],
                    self.group0().xxxy()[3] * self[e235],
                ]) * self.group0().yz().with_zw(1.0, 1.0))
                + (self.group0().xx() * self.group0().yz()).with_zw(self.group0().yzyz()[2] * self[e425], self.group0().yzyz()[3] * self[e315])
                - (Simd32x4::from([self.group1().yzx()[0], self.group1().yzx()[1], self.group1().yzx()[2], self[e235]]) * self.group0().wyzy()),
        )
    }
}
impl ConstraintViolation for VersorEvenAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        4       12        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::from(2.0) * (self.group0().zxy() * self.group1().yzx())
                + Simd32x3::from([
                    -(self.group0().wyzw()[1] * self.group1().wzzw()[1]) - (self[e431] * self[e125]),
                    self[e412] * self[e235] * -2.0,
                    self[e423] * self[e315] * -2.0,
                ]),
        )
    }
}
impl ConstraintViolation for VersorEvenOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        NullSphereAtOrigin::from_groups(
            // e1234
            2.0 * (self[e423] * self[e415]) + 2.0 * (self[e431] * self[e425]) + 2.0 * (self[e412] * self[e435]) - 2.0 * (self[e12345] * self[e4]),
        )
    }
}
impl ConstraintViolation for VersorEvenOrthogonalOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       16        0
    //    simd4        4        4        0
    // Totals...
    // yes simd        9       20        0
    //  no simd       21       32        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group0().zxy()[0] * self.group1().yzx()[0]) - (self[e235] * self[e4]),
                (self.group0().zxy()[1] * self.group1().yzx()[1]) - (self[e315] * self[e4]),
                (self.group1().yzz()[2] * self[e4]) + (self.group1().wwww()[2] * self[e412]) - (self[e423] * self[e315]) - (self[e125] * self[e4]),
                0.0,
            ]) + (Simd32x4::from([self.group1().xyx()[0], self.group1().xyx()[1], self.group0().zxy()[2] * self.group1().yzx()[2], self[e315] * self[e2]])
                * self.group2().ww().with_zw(1.0, 1.0))
                + (Simd32x4::from([self.group1().yzz()[0], self.group1().yzz()[1], self.group1().xyx()[2] * self[e431], self[e125] * self[e3]])
                    * self.group0().zx().with_zw(1.0, 1.0))
                - (Simd32x4::from([
                    self.group0().yzx()[0],
                    self.group0().yzx()[1],
                    self.group0().yzx()[2] * self.group1().zxyy()[2],
                    self.group1().zxyy()[3] * self[e2],
                ]) * self.group1().zx().with_zw(1.0, 1.0))
                - (Simd32x4::from([
                    self.group0().yzz()[0],
                    self.group0().yzz()[1],
                    self.group0().yzz()[2] * self.group1().zxww()[2],
                    self.group2().wwwz()[3] * self[e125],
                ]) * self.group1().zx().with_zw(1.0, 1.0)),
            // e1234
            0.0,
        )
    }
}
impl ConstraintViolation for VersorOdd {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       50        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       36       54        0
    //  no simd       48       66        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group1().yzz()[0] * self[e4125])
                    + (self.group0().wwww()[0] * self[e4235])
                    + (self.group1().wwww()[0] * self[e23])
                    + (self[e42] * self[e35])
                    + (self[scalar] * self[e4235])
                    + (self[e12] * self[e4315])
                    + (self[e15] * self[e1234])
                    - (self.group2().xyx()[0] * self[e1234])
                    - (self.group2().yzz()[0] * self[e43])
                    - (self.group3().yzx()[0] * self[e12]),
                (self.group1().yzz()[1] * self[e4235])
                    + (self.group0().wwww()[1] * self[e4315])
                    + (self.group1().wwww()[1] * self[e31])
                    + (self[e43] * self[e15])
                    + (self[scalar] * self[e4315])
                    + (self[e23] * self[e4125])
                    + (self[e25] * self[e1234])
                    - (self.group2().xyx()[1] * self[e1234])
                    - (self.group2().yzz()[1] * self[e41])
                    - (self.group3().yzx()[1] * self[e23]),
                2.0 * (self[e41] * self[e25])
                    + (self.group1().yzz()[2] * self[e45])
                    + (self.group0().wwww()[2] * self[e4125])
                    + (self.group1().wwww()[2] * self[e12])
                    + (self[scalar] * self[e4125])
                    + (self[e31] * self[e4235])
                    + (self[e35] * self[e1234])
                    - (self.group2().xyx()[2] * self[e42])
                    - (self.group2().yzz()[2] * self[e1234])
                    - (self.group3().yzx()[2] * self.group1().zxyy()[2])
                    - (self[e43] * self[e3215]),
                -2.0 * (self[e23] * self[e15]) - 2.0 * (self[e12] * self[e35]),
            ]) + (Simd32x4::from([
                self.group0().yzz()[0],
                self.group0().yzz()[1],
                self.group0().yzz()[2] * self[e3215],
                self.group0().xyxw()[3] * self[e3215],
            ]) * self.group2().zx().with_zw(1.0, 1.0))
                + (Simd32x4::from([self.group1().xyx()[0], self.group1().xyx()[1], self.group1().xyx()[2] * self[e4315], self[scalar] * self[e3215]])
                    * self.group1().ww().with_zw(1.0, 1.0))
                - (Simd32x4::from([self.group0().zxy()[0], self.group0().zxy()[1], self.group0().zxy()[2], self.group1().zxyy()[3]]) * self.group2().yzxy())
                - (Simd32x4::from([self.group1().yzx()[0], self.group1().yzx()[1], self.group1().yzx()[2] * self[e4315], self[e31] * self[e25]])
                    * self.group3().zx().with_zw(1.0, 1.0)),
            // e1234
            2.0 * (self[scalar] * self[e1234]) - 2.0 * (self[e41] * self[e23]) - 2.0 * (self[e42] * self[e31]) - 2.0 * (self[e43] * self[e12]),
        )
    }
}
impl ConstraintViolation for VersorOddAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       26        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       17       29        0
    //  no simd       26       38        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[scalar] * self[e4235]) + (self[e23] * self[e45]) + (self[e12] * self[e4315])
                    - (self.group1().yzx()[0] * self.group2().zxy()[0])
                    - (self.group1().zxy()[0] * self.group2().yzx()[0]),
                (self[scalar] * self[e4315]) + (self[e23] * self[e4125]) + (self[e31] * self[e45])
                    - (self.group1().yzx()[1] * self.group2().zxy()[1])
                    - (self.group1().zxy()[1] * self.group2().yzx()[1]),
                2.0 * (self[scalar] * self[e4125]) + (self.group2().xyxw()[2] * self[e31])
                    - (self.group1().yzx()[2] * self.group2().zxy()[2])
                    - (self.group1().zxy()[2] * self.group2().yzx()[2]),
                -(self[e45] * self[e3215]) - 2.0 * (self[e15] * self[e23]) - 2.0 * (self[e25] * self[e31]) - 2.0 * (self[e35] * self[e12]),
            ]) + (Simd32x4::from(self[e45]) * Simd32x4::from([self.group1().xyx()[0], self.group1().xyx()[1], self.group1().yzz()[2], self.group2().yzzw()[3]]))
                + (Simd32x4::from([
                    self.group0().xx()[0],
                    self.group0().xx()[1],
                    self.group1().xyx()[2] * self[e4315],
                    self.group2().xyxw()[3] * self[scalar],
                ]) * self.group2().xy().with_zw(1.0, 1.0))
                + (Simd32x4::from([self.group1().yzz()[0], self.group1().yzz()[1], self.group1().wwww()[2] * self[e12], self[scalar] * self[e3215]])
                    * self.group2().zx().with_zw(1.0, 1.0)),
        )
    }
}
impl ConstraintViolation for VersorOddOrthogonalOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       24        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       13       29        0
    //  no simd       28       44        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e42] * self[e35]) + (self[e15] * self[e1234]) - (self[e43] * self[e25]),
                (self[e43] * self[e15]) + (self[e25] * self[e1234]) - (self[e42] * self[e3215]),
                self[e41] * self[e25],
                -(self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]),
            ]) + (Simd32x4::from([
                self.group0().yzz()[0],
                self.group0().yzz()[1],
                self.group0().yzz()[2] * self[e3215],
                self.group0().xyxw()[3] * self[e3215],
            ]) * self.group2().zx().with_zw(1.0, 1.0))
                + (self.group1().ww().with_zw(self[e25], self[scalar] * self[e3215]) * self.group0().xyx().with_w(1.0))
                - (Simd32x4::from([self.group0().zxy()[0], self.group0().zxy()[1], self.group0().zxy()[2], self.group1().wwwy()[3]]) * self.group2().yzxy())
                - (Simd32x4::from([self.group2().xyx()[0], self.group2().xyx()[1], self.group2().xyx()[2] * self[e42], self.group2().yzzx()[3] * self[e23]])
                    * self.group2().ww().with_zw(1.0, 1.0))
                - (Simd32x4::from([self[e3215], self[e35], self[e3215], self[e23] * self[e15]]) * self.group0().xxz().with_w(1.0)),
            // e1234
            2.0 * (self[scalar] * self[e1234]) - 2.0 * (self[e41] * self[e23]) - 2.0 * (self[e42] * self[e31]) - 2.0 * (self[e43] * self[e12]),
        )
    }
}
