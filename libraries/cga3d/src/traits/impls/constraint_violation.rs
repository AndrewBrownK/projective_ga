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
//   Median:         4      12       0     N/A
//  Average:        10      24       0     N/A
//  Maximum:        85     174       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0       0
//   Median:         8      24       0       0
//  Average:        19      41       0       0
//  Maximum:       148     295       0       0
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
    //      f32        2        6        0        0
    //    simd3        2        6        0      N/A
    // Totals...
    // yes simd        4       12        0      N/A
    //  no simd        8       24        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(2.0) * (self.group0().zxy() * self.group2().yzx())
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e45]) * self.group1().xyz())
                - Simd32x3::from(2.0) * (self.group0().yzx() * self.group2().zxy()))
            .with_w(0.0),
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
    //      f32        8       20        0        0
    //    simd3        3        8        0      N/A
    //    simd4        3        5        0      N/A
    // Totals...
    // yes simd       14       33        0      N/A
    //  no simd       29       64        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (self.group1().zxyw() * self.group3().yzxw())
                + (Simd32x3::from(2.0) * (self.group0().xyx() * Simd32x2::from(self[e5]).with_z(self[e315]))
                    + Simd32x3::from(2.0) * (self.group0().yzz() * self.group2().zx().with_z(self[e5]))
                    - Simd32x3::from(2.0) * (Simd32x3::from(self[e4]) * self.group2().xyz())
                    - Simd32x3::from(2.0) * (self.group0().zxy() * self.group2().yzx()))
                .with_w(2.0 * (self[e235] * self[e1]) + 2.0 * (self[e315] * self[e2]) - 2.0 * (self[e415] * self[e235]))
                - Simd32x4::from(2.0) * (self.group1().xyxy() * Simd32x2::from(self[e321]).with_zw(self[e2], self[e315]))
                - Simd32x4::from(2.0) * (self.group1().yzzz() * self.group3().zx().with_zw(self[e321], self[e125])),
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
    //      f32        2        6        0        0
    //    simd3        2        6        0      N/A
    // Totals...
    // yes simd        4       12        0      N/A
    //  no simd        8       24        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(2.0) * (self.group0().yzx() * self.group2().zxy())
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e321]) * self.group1().xyz())
                - Simd32x3::from(2.0) * (self.group0().zxy() * self.group2().yzx()))
            .with_w(0.0),
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
    //      f32        2        6        0        0
    //    simd3        2        6        0      N/A
    // Totals...
    // yes simd        4       12        0      N/A
    //  no simd        8       24        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(2.0) * (self.group0().yzx() * self.group2().zxy())
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e321]) * self.group1().xyz())
                - Simd32x3::from(2.0) * (self.group0().zxy() * self.group2().yzx()))
            .with_w(0.0),
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
    //      f32        2        6        0        0
    //    simd3        4        7        0      N/A
    // Totals...
    // yes simd        6       13        0      N/A
    //  no simd       14       27        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            ((reverse_g2.zxy() * self.group0().yzx()) + (self.group0().zxy() * self.group2().yzx())
                - (reverse_g2.yzx() * self.group0().zxy())
                - (self.group0().yzx() * self.group2().zxy())
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e45]) * self.group1().xyz()))
            .with_w(0.0),
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
    //      f32        7       19        0        0
    //    simd2        1        4        0      N/A
    //    simd3        3        6        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       13       31        0      N/A
    //  no simd       26       53        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (self.group2().xyzy() * Simd32x3::from(self[e1234]).with_w(self[e4315]))
                + (self.group3().yzxx() * self.group1().zxy().with_w(self[e15]))
                + (Simd32x3::from(2.0) * (self.group0().zxy() * self.group2().yzx())
                    + (Simd32x2::from(-2.0) * (Simd32x2::from(self[e45]) * self.group1().xy()) - Simd32x2::from(2.0) * (self.group1().yz() * self.group3().zx()))
                        .with_z(-2.0 * (self[e23] * self[e4315]) - 2.0 * (self[e12] * self[e45]))
                    - Simd32x3::from(2.0) * (self.group0().xyx() * Simd32x2::from(self[e3215]).with_z(self[e25]))
                    - Simd32x3::from(2.0) * (self.group0().yzz() * self.group2().zx().with_z(self[e3215])))
                .with_w(self[e45] * self[e3215]),
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
    //      f32       55      113        0        0
    //    simd2        2        8        0      N/A
    //    simd3       23       46        0      N/A
    //    simd4        5        7        0      N/A
    // Totals...
    // yes simd       85      174        0      N/A
    //  no simd      148      295        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                2.0 * (self[scalar] * self[e12345])
                    + 2.0 * (self[e41] * self[e235])
                    + 2.0 * (self[e42] * self[e315])
                    + 2.0 * (self[e43] * self[e125])
                    + 2.0 * (self[e23] * self[e415])
                    + 2.0 * (self[e31] * self[e425])
                    + 2.0 * (self[e12] * self[e435])
                    + 2.0 * (self[e423] * self[e15])
                    + 2.0 * (self[e431] * self[e25])
                    + 2.0 * (self[e412] * self[e35])
                    + 2.0 * (self[e1] * self[e4235])
                    + 2.0 * (self[e45] * self[e321])
                    + 2.0 * (self[e5] * self[e1234])
                    + (self[e2] * self[e4315])
                    + (self[e3] * self[e4125])
                    + (self[e4] * self[e3215]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (Simd32x4::from(self[scalar]) * self.group1())
                + (self.group1().yzxy() * self.group5().zxy().with_w(self[e42]))
                + (Simd32x3::from(2.0) * (Simd32x3::from(self[e45]) * self.group6().xyz())
                    + Simd32x3::from(2.0) * (self.group4().xyx() * Simd32x2::from(self[e5]).with_z(self[e315]))
                    + Simd32x3::from(2.0) * (self.group4().yzz() * self.group8().zx().with_z(self[e5]))
                    + Simd32x3::from(2.0) * (self.group7().xyx() * Simd32x2::from(self[e3215]).with_z(self[e25]))
                    + Simd32x3::from(2.0) * (self.group7().yzz() * self.group3().zx().with_z(self[e3215]))
                    + (Simd32x2::from(2.0) * (self.group6().yz() * self.group9().zx()) - Simd32x2::from(2.0) * (self.group6().zx() * self.group9().yz()))
                        .with_z(2.0 * (self[e415] * self[e4315]) - 2.0 * (self[e425] * self[e4235]))
                    - Simd32x3::from(2.0) * (Simd32x3::from(self[e12345]) * self.group9().xyz())
                    - Simd32x3::from(2.0) * (Simd32x3::from(self[e4]) * self.group3().xyz())
                    - Simd32x3::from(2.0) * (Simd32x3::from(self[e1234]) * self.group8())
                    - Simd32x3::from(2.0) * (self.group4().zxy() * self.group8().yzx())
                    - Simd32x3::from(2.0) * (self.group5().xyx() * Simd32x2::from(self[e321]).with_z(self[e2]))
                    - Simd32x3::from(2.0) * (self.group5().yzz() * self.group1().zx().with_z(self[e321]))
                    - Simd32x3::from(2.0) * (self.group7().zxy() * self.group3().yzx()))
                .with_w(2.0 * (self[e12345] * self[e1234]) + (self[e41] * self[e1]) + (self[e43] * self[e3]) + (self[e321] * self[e1234])),
            // e5
            2.0 * (self[scalar] * self[e5])
                + 2.0 * (self[e12345] * self[e3215])
                + 2.0 * (self[e23] * self[e235])
                + 2.0 * (self[e31] * self[e315])
                + 2.0 * (self[e12] * self[e125])
                + 2.0 * (self[e235] * self[e4235])
                + 2.0 * (self[e315] * self[e4315])
                + 2.0 * (self[e125] * self[e4125])
                + 2.0 * (self[e15] * self[e415])
                + 2.0 * (self[e25] * self[e425])
                + 2.0 * (self[e35] * self[e435])
                + 2.0 * (self[e45] * self[e5])
                - 2.0 * (self[e1] * self[e15])
                - 2.0 * (self[e2] * self[e25])
                - 2.0 * (self[e3] * self[e35])
                - 2.0 * (self[e321] * self[e3215]),
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
                + (self.group3() * Simd32x3::from(self[e1234]).with_w(self[e3215]))
                + (self.group1().xyzx() * Simd32x3::from(self[e12345]).with_w(self[e235]))
                + (Simd32x3::from(2.0) * (self.group4().zxy() * self.group3().yzx())
                    + Simd32x3::from(2.0) * (self.group5().zxy() * self.group9().yzx())
                    + Simd32x3::from(2.0) * (self.group7().xyx() * Simd32x2::from(self[e5]).with_z(self[e315]))
                    + Simd32x3::from(2.0) * (self.group7().yzz() * self.group8().zx().with_z(self[e5]))
                    + (Simd32x2::from(2.0) * (self.group1().yz() * self.group6().zx()) - Simd32x2::from(2.0) * (self.group1().zx() * self.group6().yz()))
                        .with_z(2.0 * (self[e1] * self[e425]) - 2.0 * (self[e2] * self[e415]))
                    - Simd32x3::from(2.0) * (Simd32x3::from(self[e4]) * self.group8())
                    - Simd32x3::from(2.0) * (Simd32x3::from(self[e321]) * self.group6().xyz())
                    - Simd32x3::from(2.0) * (self.group4().xyx() * Simd32x2::from(self[e3215]).with_z(self[e25]))
                    - Simd32x3::from(2.0) * (self.group4().yzz() * self.group3().zx().with_z(self[e3215]))
                    - Simd32x3::from(2.0) * (self.group5().xyx() * Simd32x2::from(self[e45]).with_z(self[e4315]))
                    - Simd32x3::from(2.0) * (self.group5().yzz() * self.group9().zx().with_z(self[e45]))
                    - Simd32x3::from(2.0) * (self.group7().zxy() * self.group8().yzx()))
                .with_w(
                    (self[e315] * self[e2]) + (self[e125] * self[e3]) + (self[e15] * self[e4235]) + (self[e25] * self[e4315]) + (self[e35] * self[e4125])
                        - 2.0 * (self[e125] * self[e435]),
                ),
            // e1234
            2.0 * (self[scalar] * self[e1234]) + 2.0 * (self[e41] * self[e23]) + 2.0 * (self[e42] * self[e31]) + 2.0 * (self[e43] * self[e12])
                - 2.0 * (self[e41] * self[e4235])
                - 2.0 * (self[e42] * self[e4315])
                - 2.0 * (self[e43] * self[e4125])
                - 2.0 * (self[e423] * self[e1])
                - 2.0 * (self[e423] * self[e415])
                - 2.0 * (self[e431] * self[e2])
                - 2.0 * (self[e431] * self[e425])
                - 2.0 * (self[e412] * self[e3])
                - 2.0 * (self[e412] * self[e435])
                - 2.0 * (self[e4] * self[e321])
                - 2.0 * (self[e45] * self[e1234])
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
    //      f32        8       22        0        0
    //    simd2        0        2        0      N/A
    //    simd3        1        2        0      N/A
    //    simd4        6       12        0      N/A
    // Totals...
    // yes simd       15       38        0      N/A
    //  no simd       35       80        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(2.0) * (self.group2().wwyw() * self.group0().xyx().with_w(self[e321]))
                + Simd32x4::from(2.0) * (self.group3().xyzy() * Simd32x3::from(self[e12345]).with_w(self[e315]))
                + Simd32x4::from(2.0) * (self.group3().yzxz() * self.group1().zxy().with_w(self[e125]))
                + (Simd32x2::from(2.0) * (self.group0().yz() * self.group2().zx()).with_z(self[e412] * self[e5] * 2.0)
                    - Simd32x3::from(2.0) * (Simd32x3::from(self[e4]) * self.group2().xyz()))
                .with_w(2.0 * (self[e235] * self[e1]) - 2.0 * (self[e435] * self[e125]))
                - Simd32x4::from(2.0) * (self.group0().zxyw() * self.group2().yzxw())
                - Simd32x4::from(2.0) * (self.group1().xyxx() * Simd32x2::from(self[e321]).with_zw(self[e2], self[e235]))
                - Simd32x4::from(2.0) * (self.group1().yzzy() * self.group3().zx().with_zw(self[e321], self[e315])),
            // e1234
            -2.0 * (self[e423] * self[e415])
                - 2.0 * (self[e423] * self[e1])
                - 2.0 * (self[e431] * self[e425])
                - 2.0 * (self[e431] * self[e2])
                - 2.0 * (self[e412] * self[e435])
                - 2.0 * (self[e412] * self[e3])
                - 2.0 * (self[e12345] * self[e4])
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
    //      f32       10       25        0        0
    //    simd2        3        8        0      N/A
    //    simd3        1        2        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd       17       39        0      N/A
    //  no simd       31       63        0        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(2.0) * (self.group0().zxyw() * self.group2().yzx().with_w(self[e3215]))
                + (self.group3() * Simd32x3::from(self[scalar]).with_w(self[e45]))
                + (self.group3().yzxy() * self.group1().zxy().with_w(self[e25]))
                + (Simd32x3::from(2.0) * (Simd32x3::from(self[e1234]) * self.group2().xyz())
                    + (Simd32x2::from(-2.0) * (Simd32x2::from(self[e45]) * self.group1().xy())
                        - Simd32x2::from(2.0) * (Simd32x2::from(self[e3215]) * self.group0().xy())
                        - Simd32x2::from(2.0) * (self.group0().yz() * self.group2().zx())
                        - Simd32x2::from(2.0) * (self.group1().yz() * self.group3().zx()))
                    .with_z(-2.0 * (self[e41] * self[e25]) - 2.0 * (self[e43] * self[e3215]) - 2.0 * (self[e23] * self[e4315]) - 2.0 * (self[e12] * self[e45])))
                .with_w(self[e15] * self[e4235]),
            // e1234
            2.0 * (self[e41] * self[e23]) + 2.0 * (self[e42] * self[e31]) + 2.0 * (self[e43] * self[e12]) + 2.0 * (self[scalar] * self[e1234])
                - 2.0 * (self[e41] * self[e4235])
                - 2.0 * (self[e42] * self[e4315])
                - 2.0 * (self[e43] * self[e4125])
                - 2.0 * (self[e45] * self[e1234]),
        )
    }
}
