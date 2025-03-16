// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 15
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:         7      25       0
//  Average:        19      35       0
//  Maximum:       129     206       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       3       0
//   Median:        11      30       0
//  Average:        24      42       0
//  Maximum:       159     254       0
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiCircleRotor {
    type Output = Sphere;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
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
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiDipoleInversion {
    type Output = Sphere;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
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
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl std::ops::DivAssign<ConstraintViolationPrefixOrPostfix> for AntiDualNum {
    fn div_assign(&mut self, _rhs: ConstraintViolationPrefixOrPostfix) {
        *self = self.constraint_violation()
    }
}
impl ConstraintViolation for AntiDualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([self[e3215] * self[scalar], 0.0]) * Simd32x2::from([2.0, 0.0]))
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiLine {
    type Output = AntiDualNum;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiLine {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(
            // e3215, scalar
            Simd32x2::from([-2.0 * (self[e23] * self[e15]) - 2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]), 0.0]),
        )
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiMotor {
    type Output = AntiDualNum;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiMotor {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            2.0 * (self[scalar] * self[e3215]) - 2.0 * (self[e23] * self[e15]) - 2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]),
            0.0,
        ]))
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for Circle {
    type Output = Sphere;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
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
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for CircleRotor {
    type Output = Sphere;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
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
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for Dipole {
    type Output = Sphere;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
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
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for DipoleInversion {
    type Output = Sphere;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
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
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for DualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([self[e5] * self[e12345], 0.0]) * Simd32x2::from([-2.0, 0.0]))
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for Line {
    type Output = AntiDualNum;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Line {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            2.0 * (self[e415] * self[e235]) + 2.0 * (self[e425] * self[e315]) + 2.0 * (self[e435] * self[e125]),
            0.0,
        ]))
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for Motor {
    type Output = AntiDualNum;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Motor {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            2.0 * (self[e415] * self[e235]) + 2.0 * (self[e425] * self[e315]) + 2.0 * (self[e435] * self[e125]) - 2.0 * (self[e12345] * self[e5]),
            0.0,
        ]))
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl std::ops::DivAssign<ConstraintViolationPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: ConstraintViolationPrefixOrPostfix) {
        *self = self.constraint_violation()
    }
}
impl ConstraintViolation for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      119      188        0
    //    simd2        0        2        0
    //    simd3        0        2        0
    //    simd4       10       14        0
    // Totals...
    // yes simd      129      206        0
    //  no simd      159      254        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                2.0 * (self.group9().wxzw()[1] * self[e1])
                    + 2.0 * (self[scalar] * self[e12345])
                    + 2.0 * (self[e2] * self[e4315])
                    + 2.0 * (self[e3] * self[e4125])
                    + 2.0 * (self[e4] * self[e3215])
                    + 2.0 * (self[e5] * self[e1234])
                    - 2.0 * (self[e15] * self[e423])
                    - 2.0 * (self[e25] * self[e431])
                    - 2.0 * (self[e35] * self[e412])
                    - 2.0 * (self[e45] * self[e321])
                    - 2.0 * (self[e41] * self[e235])
                    - 2.0 * (self[e42] * self[e315])
                    - 2.0 * (self[e43] * self[e125])
                    - 2.0 * (self[e23] * self[e415])
                    - 2.0 * (self[e31] * self[e425])
                    - 2.0 * (self[e12] * self[e435]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (Simd32x4::from(self[scalar]) * self.group1())
                + Simd32x4::from(2.0) * (self.group4().zxy() * self.group8().yzx()).with_w(self[e12345] * self[e1234])
                + Simd32x4::from([
                    2.0 * (self[e23] * self[e321])
                        + (self.group3().xyx()[0] * self[e4])
                        + (self.group3().yzx()[0] * self[e412])
                        + (self.group3().yzz()[0] * self[e412])
                        + (self[e3] * self[e31])
                        + (self[e425] * self[e4125])
                        + (self[e435] * self[e4315])
                        - (self.group1().zxy()[0] * self[e31])
                        - (self.group3().zxy()[0] * self[e431])
                        - (self.group6().xyx()[0] * self[e45])
                        - (self.group6().yzz()[0] * self[e4125])
                        - (self.group6().zxy()[0] * self.group9().yzx()[0])
                        - (self[e4] * self[e15])
                        - (self[e35] * self[e431])
                        - (self[e45] * self[e415]),
                    2.0 * (self[e31] * self[e321])
                        + (self.group3().xyx()[1] * self[e4])
                        + (self.group3().yzx()[1] * self[e423])
                        + (self.group3().yzz()[1] * self[e423])
                        + (self[e1] * self[e12])
                        + (self[e415] * self[e4125])
                        + (self[e435] * self[e4235])
                        - (self.group1().zxy()[1] * self[e12])
                        - (self.group3().zxy()[1] * self[e412])
                        - (self.group6().xyx()[1] * self[e45])
                        - (self.group6().yzz()[1] * self[e4235])
                        - (self.group6().zxy()[1] * self.group9().yzx()[1])
                        - (self[e4] * self[e25])
                        - (self[e15] * self[e412])
                        - (self[e45] * self[e425]),
                    2.0 * (self[e12] * self[e321])
                        + (self.group3().xyx()[2] * self[e431])
                        + (self.group3().yzx()[2] * self[e431])
                        + (self.group3().yzz()[2] * self[e4])
                        + (self[e2] * self[e23])
                        + (self[e415] * self[e4315])
                        + (self[e425] * self[e4235])
                        - (self.group1().zxy()[2] * self[e23])
                        - (self.group3().zxy()[2] * self[e423])
                        - (self.group6().xyx()[2] * self[e4315])
                        - (self.group6().yzz()[2] * self[e45])
                        - (self.group6().zxy()[2] * self.group9().yzx()[2])
                        - (self.group1().wwww()[2] * self[e35])
                        - (self.group3().wwww()[2] * self[e435])
                        - (self[e25] * self[e423]),
                    -2.0 * (self[e43] * self[e435]) - 2.0 * (self[e23] * self[e423]) - 2.0 * (self[e31] * self[e431]) - 2.0 * (self[e12] * self[e412]),
                ])
                - Simd32x4::from(2.0) * (self.group0().yy().with_zw(self[e12345], self[e42] * self[e425]) * self.group9().xyz().with_w(1.0))
                - Simd32x4::from(2.0) * (self.group4().yzx() * self.group8().zxy()).with_w(self[e41] * self[e415]),
            // e5
            2.0 * (self[scalar] * self[e5]) + 2.0 * (self[e12345] * self[e3215])
                - 2.0 * (self[e15] * self[e415])
                - 2.0 * (self[e25] * self[e425])
                - 2.0 * (self[e35] * self[e435])
                - 2.0 * (self[e23] * self[e235])
                - 2.0 * (self[e31] * self[e315])
                - 2.0 * (self[e12] * self[e125]),
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(2.0) * (Simd32x4::from(self[scalar]) * self.group9())
                + Simd32x4::from(2.0) * (self.group0().yy().with_zw(self[e12345], self[e415] * self[e235]) * self.group1().xyz().with_w(1.0))
                + Simd32x4::from(2.0) * (self.group5().xy() * self.group3().ww()).with_zw(self[e431] * self[e235], self[e425] * self[e315])
                + Simd32x4::from([
                    2.0 * (self[e412] * self[e315])
                        + (self.group1().yzx()[0] * self.group6().zxy()[0])
                        + (self.group6().xyx()[0] * self[e321])
                        + (self.group6().yzz()[0] * self[e3])
                        + (self[e15] * self[e1234])
                        + (self[e35] * self[e42])
                        + (self[e12] * self[e4315])
                        + (self[e415] * self[e321])
                        - (self.group1().yzx()[0] * self.group6().zxy()[0])
                        - (self.group1().zxy()[0] * self.group6().yzx()[0])
                        - (self.group3().xyx()[0] * self[e1234])
                        - 2.0 * (self[e431] * self[e125]),
                    2.0 * (self[e423] * self[e125])
                        + (self.group1().yzx()[1] * self.group6().zxy()[1])
                        + (self.group6().xyx()[1] * self[e321])
                        + (self.group6().yzz()[1] * self[e1])
                        + (self[e15] * self[e43])
                        + (self[e25] * self[e1234])
                        + (self[e23] * self[e4125])
                        + (self[e425] * self[e321])
                        - (self.group1().yzx()[1] * self.group6().zxy()[1])
                        - (self.group1().zxy()[1] * self.group6().yzx()[1])
                        - (self.group3().xyx()[1] * self[e1234])
                        - 2.0 * (self[e412] * self[e235]),
                    (self.group3().zxy()[2] * self[e41])
                        + (self.group6().xyx()[2] * self[e2])
                        + (self.group6().yzz()[2] * self[e321])
                        + (self.group3().wwww()[2] * self[e12])
                        + (self.group6().wwww()[2] * self[e435])
                        + (self[e4] * self[e125])
                        + (self[e25] * self[e41])
                        + (self[e35] * self[e1234])
                        + (self[e45] * self[e12])
                        + (self[e43] * self[e3215])
                        + (self[e23] * self[e4315])
                        + (self[e31] * self[e4235])
                        - (self.group1().yzx()[2] * self.group6().zxy()[2])
                        - (self.group1().zxy()[2] * self.group6().yzx()[2])
                        - (self.group3().xyx()[2] * self[e42])
                        - (self.group9().yzx()[2] * self[e31])
                        - (self.group1().wwww()[2] * self[e125])
                        - (self.group3().yzzx()[2] * self[e1234])
                        - (self.group9().wwww()[2] * self[e43])
                        - (self[e15] * self[e42]),
                    2.0 * (self[e435] * self[e125])
                        - (self.group9().zxyz()[3] * self[e35])
                        - (self[e15] * self[e23])
                        - 2.0 * (self[e12345] * self[e5])
                        - 2.0 * (self[e35] * self[e12]),
                ])
                + (Simd32x4::from([self.group3().zxy()[0], self.group3().zxy()[1], self.group1().yzx()[2] * self.group6().zxy()[2], self[e35] * self[e4125]])
                    * self.group4().yz().with_zw(1.0, 1.0))
                - (Simd32x4::from([
                    self.group9().yzx()[0],
                    self.group9().yzx()[1],
                    self.group5().yzx()[2] * self.group9().zxyz()[2],
                    self.group3().yzzx()[3] * self[e23],
                ]) * self.group5().zx().with_zw(1.0, 1.0))
                - Simd32x4::from(2.0) * (self.group4().zx() * self.group3().yz()).with_zw(self[e423] * self[e315], self[e25] * self[e31]),
            // e1234
            2.0 * (self[scalar] * self[e1234]) + 2.0 * (self[e415] * self[e423]) + 2.0 * (self[e425] * self[e431]) + 2.0 * (self[e435] * self[e412])
                - 2.0 * (self[e41] * self[e23])
                - 2.0 * (self[e42] * self[e31])
                - 2.0 * (self[e43] * self[e12])
                - 2.0 * (self[e12345] * self[e4]),
        )
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for VersorEven {
    type Output = Sphere;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
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
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for VersorOdd {
    type Output = Sphere;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
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
