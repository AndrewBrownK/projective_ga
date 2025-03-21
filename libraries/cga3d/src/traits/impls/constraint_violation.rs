// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 17
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0     N/A
//   Median:         5      14       0     N/A
//  Average:        14      27       0     N/A
//  Maximum:       124     204       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0       0
//   Median:        14      31       0       0
//  Average:        33      45       0       0
//  Maximum:       289     313       0       0
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiCircleRotor {
    type Output = Sphere;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
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
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiDipoleInversion {
    type Output = Sphere;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
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
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([self[e3215] * self[scalar] * 2.0, 0.0]))
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiFlector {
    type Output = AntiDualNum;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiFlector {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            2.0 * (self[e235] * self[e1]) + 2.0 * (self[e315] * self[e2]) + 2.0 * (self[e125] * self[e3]) + 2.0 * (self[e321] * self[e5]),
            0.0,
        ]))
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
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(
            // e3215, scalar
            Simd32x2::from([2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35]), 0.0]),
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
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35]) + 2.0 * (self[scalar] * self[e3215]),
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
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for CircleRotor {
    type Output = Sphere;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
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
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for Dipole {
    type Output = Sphere;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
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
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for DipoleInversion {
    type Output = Sphere;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
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
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for DualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([self[e5] * self[e12345] * -2.0, 0.0]))
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for Flector {
    type Output = AntiDualNum;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Flector {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            2.0 * (self[e15] * self[e4235]) + 2.0 * (self[e25] * self[e4315]) + 2.0 * (self[e35] * self[e4125]) + 2.0 * (self[e45] * self[e3215]),
            0.0,
        ]))
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
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            -2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]),
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
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            -2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]) - 2.0 * (self[e12345] * self[e5]),
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
    //           add/sub      mul      div      pow
    //      f32       69      165        0        0
    //    simd2        0        2        0      N/A
    //    simd3        0        4        0      N/A
    //    simd4       55       33        0      N/A
    // Totals...
    // yes simd      124      204        0      N/A
    //  no simd      289      313        0        0
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
                    + (self[e15] * self[e423])
                    + (self[e25] * self[e431])
                    + (self[e35] * self[e412])
                    + 2.0 * (self[e41] * self[e235])
                    + 2.0 * (self[e42] * self[e315])
                    + 2.0 * (self[e43] * self[e125])
                    + (self[e23] * self[e415])
                    + (self[e31] * self[e425])
                    + (self[e12] * self[e435])
                    - (reverse_g3[0] * self[e423])
                    - (reverse_g3[1] * self[e431])
                    - (reverse_g3[2] * self[e412])
                    - (reverse_g3[3] * self[e321])
                    - (reverse_g6[0] * self[e23])
                    - (reverse_g6[1] * self[e31])
                    - (reverse_g6[2] * self[e12])
                    - (reverse_g6[3] * self[e45]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (Simd32x4::from(self[scalar]) * self.group1())
                + Simd32x4::from(2.0) * (Simd32x4::from([self[e5], self[e5], self[e5], self[e2] * self[e42]]) * self.group4().with_w(1.0))
                + Simd32x4::from(2.0) * (self.group1().yzxx() * self.group5().zxy().with_w(self[e41]))
                + Simd32x4::from([0.0, self[e43] * self[e235], reverse_g6[3] * self[e12], self[e42] * self[e425]])
                + Simd32x4::from([reverse_g6[2] * self[e4315], reverse_g3[2] * self[e423], reverse_g3[0] * self[e431], 0.0])
                + Simd32x4::from([reverse_g6[3] * self[e23], reverse_g6[0] * self[e4125], 0.0, self[e12345] * self[e1234]])
                + Simd32x4::from([self[e42] * self[e125], reverse_g6[3] * self[e31], reverse_g6[1] * self[e4235], 0.0])
                + Simd32x4::from([self[e423] * self[e3215], 0.0, self[e41] * self[e315], self[e43] * self[e435]])
                + (reverse_g3 * Simd32x4::from(self[e4]))
                + (Simd32x4::from(self[e412]) * Simd32x4::from([reverse_g3[1], self[e15], self[e3215], self[e12]]))
                + (Simd32x4::from([self[e35], self[e3215], self[e25], self[e3] * self[e43]]) * self.group7().yyx().with_w(1.0))
                + (self.group6().yzxx() * self.group9().zxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w(self[e23] * self[e423])
                + Simd32x3::from(0.0).with_w(2.0 * (self[e31] * self[e431]) + (self[e321] * self[e1234]))
                - Simd32x4::from([0.0, reverse_g6[2] * self[e4235], self[e2] * self[e23], reverse_g6[0] * self[e41]])
                - Simd32x4::from([0.0, self[e315] * self[e1234], self[e42] * self[e235], 0.0])
                - Simd32x4::from([reverse_g3[2] * self[e431], self[e1] * self[e12], 0.0, reverse_g6[1] * self[e42]])
                - Simd32x4::from([reverse_g3[3] * self[e415], 0.0, self[e15] * self[e431], reverse_g6[2] * self[e43]])
                - Simd32x4::from([self[e12345] * self[e4235], reverse_g3[0] * self[e412], reverse_g3[1] * self[e423], 0.0])
                - Simd32x4::from([self[e23] * self[e321], 0.0, 0.0, 0.0])
                - Simd32x4::from([self[e235] * self[e1234], reverse_g3[3] * self[e425], reverse_g3[3] * self[e435], 0.0])
                - (reverse_g6 * Simd32x3::from(self[e45]).with_w(self[e1234]))
                - (Simd32x4::from(self[e4]) * self.group3())
                - (Simd32x4::from(self[e4315]) * Simd32x4::from([self[e435], self[e12345], reverse_g6[0], self[e431]]))
                - (Simd32x4::from(self[e4125]) * Simd32x4::from([reverse_g6[1], self[e415], self[e12345], self[e412]]))
                - (Simd32x4::from([self[e3], self[e321], self[e321], self[e423] * self[e4235]]) * self.group5().yyz().with_w(1.0))
                - Simd32x2::from(0.0).with_zw(self[e125] * self[e1234], 0.0)
                - (self.group3().yz() * self.group7().zx()).with_z(0.0).with_w(0.0)
                - (self.group4().zx() * self.group8().yz()).with_z(self[e425] * self[e4235]).with_w(0.0),
            // e5
            2.0 * (self[scalar] * self[e5])
                + 2.0 * (self[e12345] * self[e3215])
                + (reverse_g3[0] * self[e1])
                + (reverse_g3[1] * self[e2])
                + (reverse_g3[2] * self[e3])
                + (reverse_g6[3] * self[e3215])
                + (self[e5] * self[e45])
                + 2.0 * (self[e23] * self[e235])
                + 2.0 * (self[e31] * self[e315])
                + 2.0 * (self[e12] * self[e125])
                + 2.0 * (self[e235] * self[e4235])
                + 2.0 * (self[e315] * self[e4315])
                + 2.0 * (self[e125] * self[e4125])
                - (reverse_g3[0] * self[e415])
                - (reverse_g3[1] * self[e425])
                - (reverse_g3[2] * self[e435])
                - (reverse_g3[3] * self[e5])
                - (reverse_g6[0] * self[e15])
                - (reverse_g6[1] * self[e25])
                - (reverse_g6[2] * self[e35])
                - (self[e1] * self[e15])
                - (self[e2] * self[e25])
                - (self[e3] * self[e35])
                - (self[e321] * self[e3215]),
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
                + Simd32x4::from([0.0, reverse_g6[1] * self[e321], reverse_g6[2] * self[e321], reverse_g6[0] * self[e235]])
                + Simd32x4::from([self[e12345] * self[e1], reverse_g3[0] * self[e43], reverse_g3[1] * self[e41], 0.0])
                + Simd32x4::from([self[e5] * self[e423], reverse_g3[3] * self[e31], reverse_g3[3] * self[e12], 0.0])
                + Simd32x4::from([
                    (reverse_g3[2] * self[e42]) + (reverse_g3[3] * self[e23]),
                    (self[e412] * self[e235]) * 2.0,
                    (self[e423] * self[e315]) * 2.0,
                    (self[e25] * self[e4315]) + (self[e35] * self[e12]) + (self[e35] * self[e4125]),
                ])
                + (Simd32x4::from(self[e2]) * Simd32x4::from([self[e435], self[e12345], reverse_g6[0], self[e315]]))
                + (Simd32x4::from(self[e3]) * Simd32x4::from([reverse_g6[1], self[e415], self[e12345], self[e125]]))
                + (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * self.group3().yzxx())
                + (Simd32x4::from([self[e321], self[e1], self[e1] * self[e425], self[e1] * self[e235]]) * reverse_g6.xz().with_zw(1.0, 1.0))
                + (Simd32x4::from([self[e125], self[e5], self[e5], reverse_g6[1] * self[e315]]) * self.group7().yyz().with_w(1.0))
                + (Simd32x4::from([self[e1234], self[e1234], self[e1234], self[e3215]]) * self.group3())
                + (self.group6() * Simd32x3::from(reverse_g6[3]).with_w(self[e5]))
                + (self.group9().yzxx() * self.group5().zxy().with_w(self[e15]))
                + Simd32x3::from(0.0).with_w(reverse_g6[2] * self[e125])
                + Simd32x3::from(0.0).with_w(self[e25] * self[e31])
                - (reverse_g3 * Simd32x4::from([self[e1234], self[e1234], self[e1234], self[e3215]]))
                - (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * reverse_g3.yzxx())
                - (reverse_g6.zxyw() * self.group1().yzx().with_w(self[e5]))
                - (self.group6().yzxx() * self.group1().zxy().with_w(self[e235]))
                - (self.group9().zxyy() * self.group5().yzx().with_w(reverse_g3[1]))
                - (self.group9().wwwx() * self.group4().with_w(reverse_g3[0]))
                - Simd32x3::from(0.0).with_w(reverse_g3[1] * self[e31])
                - Simd32x3::from(0.0).with_w(reverse_g3[2] * self[e12])
                - Simd32x3::from(0.0).with_w(self[e425] * self[e315])
                - (Simd32x3::from(self[e4]) * self.group8()).with_w(reverse_g3[2] * self[e4125])
                - (Simd32x3::from(self[e45]) * self.group5()).with_w(self[e12345] * self[e5])
                - (self.group4().yzx() * self.group3().zxy()).with_w(self[e435] * self[e125])
                - (self.group7().zxy() * self.group8().yzx()).with_w(0.0),
            // e1234
            2.0 * (self[scalar] * self[e1234])
                + (reverse_g3[3] * self[e1234])
                + (reverse_g6[0] * self[e423])
                + (reverse_g6[1] * self[e431])
                + (reverse_g6[2] * self[e412])
                + (reverse_g6[3] * self[e4])
                + 2.0 * (self[e41] * self[e23])
                + 2.0 * (self[e42] * self[e31])
                + 2.0 * (self[e43] * self[e12])
                - 2.0 * (self[e1] * self[e423])
                - 2.0 * (self[e2] * self[e431])
                - 2.0 * (self[e3] * self[e412])
                - (self[e4] * self[e321])
                - (self[e45] * self[e1234])
                - 2.0 * (self[e41] * self[e4235])
                - 2.0 * (self[e42] * self[e4315])
                - 2.0 * (self[e43] * self[e4125])
                - (self[e415] * self[e423])
                - (self[e425] * self[e431])
                - (self[e435] * self[e412])
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
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for VersorOdd {
    type Output = Sphere;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
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
