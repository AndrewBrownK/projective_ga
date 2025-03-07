// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 17
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:        13      22       0
//  Average:        23      32       0
//  Maximum:       164     212       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:        25      40       0
//  Average:        44      56       0
//  Maximum:       332     372       0
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiCircleRotor {
    type Output = Sphere;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiCircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       23        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       25       41        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let reverse_g2 = self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse_g0[1] * self[e35]) + (reverse_g1[0] * self[e45]) + (reverse_g1[3] * self[e23]) + (reverse_g2[2] * self[e42]),
                (reverse_g0[2] * self[e15]) + (reverse_g1[1] * self[e45]) + (reverse_g1[3] * self[e31]) + (reverse_g2[0] * self[e43]),
                (reverse_g0[0] * self[e25]) + (reverse_g1[2] * self[e45]) + (reverse_g1[3] * self[e12]) + (reverse_g2[1] * self[e41]),
                -(reverse_g1[2] * self[e35]) - (reverse_g2[0] * self[e23]) - (reverse_g2[1] * self[e31]) - (reverse_g2[2] * self[e12]),
            ]) - (self.group2().yzxx() * reverse_g0.zxy().with_w(reverse_g1[0]))
                - (self.group0().zxy() * reverse_g2.yzx()).with_w(reverse_g1[1] * self[e25]),
            // e1234
            -(reverse_g0[0] * self[e23])
                - (reverse_g0[1] * self[e31])
                - (reverse_g0[2] * self[e12])
                - (reverse_g1[0] * self[e41])
                - (reverse_g1[1] * self[e42])
                - (reverse_g1[2] * self[e43]),
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
    //      f32       17       23        0
    //    simd3        0        2        0
    //    simd4       12       13        0
    // Totals...
    // yes simd       29       38        0
    //  no simd       65       81        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let reverse_g2 = self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(reverse_g2[3] * self[e235]) - (self[e425] * self[e3]),
                -(reverse_g2[3] * self[e315]) - (self[e435] * self[e1]),
                -(reverse_g2[3] * self[e125]) - (self[e415] * self[e2]),
                (self[e321] * self[e5]) + (self[e125] * self[e3]),
            ]) + (Simd32x4::from([reverse_g2[1], self[e5], self[e5], self[e125]]) * self.group0().zyz().with_w(reverse_g1[2]))
                + (Simd32x4::from([self[e5], reverse_g2[2], reverse_g2[0], self[e315]]) * self.group0().xxy().with_w(reverse_g1[1]))
                + (self.group1().xyzz() * reverse_g1.www().with_w(reverse_g2[2]))
                + (self.group2().yzxx() * reverse_g0.zxy().with_w(reverse_g1[0]))
                + (self.group2().wwwx() * reverse_g2.xyz().with_w(self[e1]))
                + (self.group3().yzxy() * self.group1().zxy().with_w(self[e315]))
                + (self.group1().ww().with_zw(self[e2], self[e415]) * reverse_g1.xyx().with_w(reverse_g2[0]))
                + (self.group3().zx().with_zw(self[e321], self[e425]) * reverse_g1.yzz().with_w(reverse_g2[1]))
                - (self.group3().yzxz() * reverse_g1.zxy().with_w(reverse_g2[2]))
                - (self.group2().zx().with_zw(self[e5], self[e1]) * reverse_g0.yzz().with_w(reverse_g2[0]))
                - (self.group3().ww().with_zw(self[e315], self[e5]) * reverse_g0.xyx().with_w(reverse_g1[3]))
                - (self.group0().yzx() * reverse_g2.zxy()).with_w(reverse_g2[1] * self[e2]),
            // e1234
            (reverse_g0[0] * self[e415])
                + (reverse_g0[0] * self[e1])
                + (reverse_g0[1] * self[e425])
                + (reverse_g0[1] * self[e2])
                + (reverse_g0[2] * self[e435])
                + (reverse_g0[2] * self[e3])
                + (reverse_g1[0] * self[e423])
                + (reverse_g1[1] * self[e431])
                + (reverse_g1[2] * self[e412])
                + (reverse_g1[3] * self[e4])
                - (reverse_g2[3] * self[e321])
                - (self[e423] * self[e1])
                - (self[e431] * self[e2])
                - (self[e412] * self[e3]),
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
    //      add/sub      mul      div
    // f32        0        2        0
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
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        9        0
    //  no simd        7       12        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            (self[e235] * self[e1]) + (self[e315] * self[e2]) + (self[e125] * self[e3]) + (self[e321] * self[e5])
                - (reverse_g0[0] * self[e1])
                - (reverse_g0[1] * self[e2])
                - (reverse_g0[2] * self[e3])
                - (reverse_g0[3] * self[e5]),
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
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            -(reverse_g0[0] * self[e15])
                - (reverse_g0[1] * self[e25])
                - (reverse_g0[2] * self[e35])
                - (reverse_g1[0] * self[e23])
                - (reverse_g1[1] * self[e31])
                - (reverse_g1[2] * self[e12]),
            0.0,
        ]))
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
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       10        0
    //  no simd        7       16        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            (reverse_g0[3] * self[e3215]) + (reverse_g1[3] * self[scalar])
                - (reverse_g0[0] * self[e15])
                - (reverse_g0[1] * self[e25])
                - (reverse_g0[2] * self[e35])
                - (reverse_g1[0] * self[e23])
                - (reverse_g1[1] * self[e31])
                - (reverse_g1[2] * self[e12]),
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
    //      f32        9       17        0
    //    simd3        0        5        0
    //    simd4        4        2        0
    // Totals...
    // yes simd       13       24        0
    //  no simd       25       40        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(reverse_g0[1] * self[e125]) - (reverse_g2[2] * self[e431]),
                -(reverse_g0[2] * self[e235]) - (reverse_g2[0] * self[e412]),
                -(reverse_g0[0] * self[e315]) - (reverse_g2[1] * self[e423]),
                (reverse_g1[1] * self[e315]) + (reverse_g1[2] * self[e125]),
            ]) + (self.group1().wwwz() * reverse_g1.xyz().with_w(reverse_g2[2]))
                + (reverse_g0.zxy() * self.group2().yzx()).with_w(reverse_g2[0] * self[e415])
                + (reverse_g2.yzx() * self.group0().zxy()).with_w(reverse_g2[1] * self[e425])
                + (self.group1().xyz() * reverse_g1.www()).with_w(reverse_g1[0] * self[e235]),
            // e1234
            (reverse_g0[0] * self[e415])
                + (reverse_g0[1] * self[e425])
                + (reverse_g0[2] * self[e435])
                + (reverse_g1[0] * self[e423])
                + (reverse_g1[1] * self[e431])
                + (reverse_g1[2] * self[e412]),
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
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        2        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       25       41        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let reverse_g2 = self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(reverse_g0[1] * self[e125]) - (reverse_g2[2] * self[e431]),
                -(reverse_g0[2] * self[e235]) - (reverse_g2[0] * self[e412]),
                -(reverse_g0[0] * self[e315]) - (reverse_g2[1] * self[e423]),
                (reverse_g2[1] * self[e425]) + (reverse_g2[2] * self[e435]),
            ]) + (reverse_g1.xyzz() * self.group1().www().with_w(self[e125]))
                + (self.group1().xyzx() * reverse_g1.www().with_w(reverse_g2[0]))
                + (self.group2().yzxx() * reverse_g0.zxy().with_w(reverse_g1[0]))
                + (self.group0().zxy() * reverse_g2.yzx()).with_w(reverse_g1[1] * self[e315]),
            // e1234
            (reverse_g0[0] * self[e415])
                + (reverse_g0[1] * self[e425])
                + (reverse_g0[2] * self[e435])
                + (reverse_g1[0] * self[e423])
                + (reverse_g1[1] * self[e431])
                + (reverse_g1[2] * self[e412]),
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
    //      f32       17       24        0
    //    simd3        0        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       25       40        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse_g0[1] * self[e35]) + (reverse_g2[2] * self[e42]) + (reverse_g1[0] * self[e45]) + (reverse_g1[3] * self[e23]),
                (reverse_g0[2] * self[e15]) + (reverse_g2[0] * self[e43]) + (reverse_g1[1] * self[e45]) + (reverse_g1[3] * self[e31]),
                (reverse_g0[0] * self[e25]) + (reverse_g2[1] * self[e41]) + (reverse_g1[2] * self[e45]) + (reverse_g1[3] * self[e12]),
                -(reverse_g2[2] * self[e12]) - (reverse_g1[0] * self[e15]) - (reverse_g1[1] * self[e25]) - (reverse_g1[2] * self[e35]),
            ]) - (reverse_g0.zxy() * self.group2().yzx()).with_w(reverse_g2[0] * self[e23])
                - (reverse_g2.yzx() * self.group0().zxy()).with_w(reverse_g2[1] * self[e31]),
            // e1234
            -(reverse_g0[0] * self[e23])
                - (reverse_g0[1] * self[e31])
                - (reverse_g0[2] * self[e12])
                - (reverse_g1[0] * self[e41])
                - (reverse_g1[1] * self[e42])
                - (reverse_g1[2] * self[e43]),
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
    //      f32       25       32        0
    //    simd3        0        3        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       35       45        0
    //  no simd       65       81        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let reverse_g2 = self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse_g1[1] * self[e4125]) + (reverse_g1[3] * self[e23]) + (reverse_g2[3] * self[e15]) + (self[e12] * self[e4315]),
                (reverse_g1[2] * self[e4235]) + (reverse_g1[3] * self[e31]) + (reverse_g2[3] * self[e25]) + (self[e23] * self[e4125]),
                (reverse_g1[2] * self[e45]) + (reverse_g1[3] * self[e12]) + (reverse_g2[3] * self[e35]) + (self[e31] * self[e4235]),
                -(reverse_g2[1] * self[e31]) - (reverse_g2[1] * self[e4315]) - (reverse_g2[2] * self[e12]) - (reverse_g2[2] * self[e4125]),
            ]) + (self.group1().ww().with_zw(self[e4315], self[e45]) * reverse_g1.xyx().with_w(self[e3215]))
                + (self.group2().zx().with_zw(self[e3215], self[e25]) * reverse_g0.yzz().with_w(self[e4315]))
                + (self.group3().ww().with_zw(self[e25], self[e15]) * reverse_g0.xyx().with_w(self[e4235]))
                + (self.group0().yzx() * reverse_g2.zxy()).with_w(self[e35] * self[e4125])
                - (Simd32x4::from([reverse_g2[1], self[e3215], self[e3215], self[e35]]) * self.group0().zyz().with_w(reverse_g1[2]))
                - (Simd32x4::from([self[e3215], reverse_g2[2], reverse_g2[0], self[e25]]) * self.group0().xxy().with_w(reverse_g1[1]))
                - (reverse_g1.zxyw() * self.group3().yzxw())
                - (reverse_g2.xyzx() * self.group2().www().with_w(self[e23]))
                - (self.group2().yzxx() * reverse_g0.zxy().with_w(reverse_g1[0]))
                - (self.group1().yzx() * self.group3().zxy()).with_w(reverse_g2[0] * self[e4235]),
            // e1234
            (reverse_g0[0] * self[e4235]) + (reverse_g0[1] * self[e4315]) + (reverse_g0[2] * self[e4125]) + (reverse_g1[3] * self[e1234])
                - (reverse_g0[0] * self[e23])
                - (reverse_g0[1] * self[e31])
                - (reverse_g0[2] * self[e12])
                - (reverse_g1[0] * self[e41])
                - (reverse_g1[1] * self[e42])
                - (reverse_g1[2] * self[e43])
                - (reverse_g2[3] * self[e45])
                - (self[e41] * self[e4235])
                - (self[e42] * self[e4315])
                - (self[e43] * self[e4125]),
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
    //      add/sub      mul      div
    // f32        0        2        0
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
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        9        0
    //  no simd        7       12        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            (self[e15] * self[e4235]) + (self[e25] * self[e4315]) + (self[e35] * self[e4125]) + (self[e45] * self[e3215])
                - (reverse_g0[0] * self[e4235])
                - (reverse_g0[1] * self[e4315])
                - (reverse_g0[2] * self[e4125])
                - (reverse_g0[3] * self[e3215]),
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
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            (reverse_g0[0] * self[e235])
                + (reverse_g0[1] * self[e315])
                + (reverse_g0[2] * self[e125])
                + (reverse_g1[0] * self[e415])
                + (reverse_g1[1] * self[e425])
                + (reverse_g1[2] * self[e435]),
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
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       10        0
    //  no simd        7       16        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            (reverse_g0[0] * self[e235])
                + (reverse_g0[1] * self[e315])
                + (reverse_g0[2] * self[e125])
                + (reverse_g1[0] * self[e415])
                + (reverse_g1[1] * self[e425])
                + (reverse_g1[2] * self[e435])
                - (reverse_g0[3] * self[e5])
                - (reverse_g1[3] * self[e12345]),
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
    //      f32      108      150        0
    //    simd3        0       26        0
    //    simd4       56       36        0
    // Totals...
    // yes simd      164      212        0
    //  no simd      332      372        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g3 = self.group3() * Simd32x4::from(-1.0);
        let reverse_g4 = self.group4() * Simd32x3::from(-1.0);
        let reverse_g5 = self.group5() * Simd32x3::from(-1.0);
        let reverse_g6 = self.group6() * Simd32x4::from(-1.0);
        let reverse_g7 = self.group7() * Simd32x3::from(-1.0);
        let reverse_g8 = self.group8() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                2.0 * (self[e5] * self[e1234])
                    + (self.group9().wxzw()[1] * self[e1])
                    + 2.0 * (self[scalar] * self[e12345])
                    + (self[e1] * self[e4235])
                    + 2.0 * (self[e2] * self[e4315])
                    + 2.0 * (self[e3] * self[e4125])
                    + 2.0 * (self[e4] * self[e3215])
                    - (reverse_g4[0] * self[e235])
                    - (reverse_g4[1] * self[e315])
                    - (reverse_g4[2] * self[e125])
                    - (reverse_g5[0] * self[e415])
                    - (reverse_g5[1] * self[e425])
                    - (reverse_g5[2] * self[e435])
                    - (reverse_g7[0] * self[e15])
                    - (reverse_g7[1] * self[e25])
                    - (reverse_g7[2] * self[e35])
                    - (reverse_g8[0] * self[e41])
                    - (reverse_g8[1] * self[e42])
                    - (reverse_g8[2] * self[e43])
                    - (reverse_g3[0] * self[e423])
                    - (reverse_g3[1] * self[e431])
                    - (reverse_g3[2] * self[e412])
                    - (reverse_g3[3] * self[e321])
                    - (reverse_g6[1] * self[e31])
                    - (reverse_g6[2] * self[e12])
                    - (reverse_g6[3] * self[e45])
                    - (reverse_g6.wxzw()[1] * self[e23]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (reverse_g3[0] * self[e4]) + (reverse_g3[1] * self[e412]) + (reverse_g6[2] * self[e4315]) + (self[e425] * self[e4125]),
                (reverse_g3[1] * self[e4]) + (reverse_g6[0] * self[e4125]) + (self[e435] * self[e4235]) + (self[e431] * self[e3215]),
                (reverse_g3[2] * self[e4]) + (reverse_g6[1] * self[e4235]) + (self[e415] * self[e4315]) + (self[e412] * self[e3215]),
                -(reverse_g6[3] * self[e1234]) - (self[e4] * self[e45]) - (self[e431] * self[e4315]) - (self[e412] * self[e4125]),
            ]) + (Simd32x4::from(self[scalar]) * self.group1())
                + (Simd32x4::from([reverse_g6[3], self[e3], self[e1], self[e3]]) * self.group5().xxy().with_w(self[e43]))
                + (Simd32x4::from([self[e2], reverse_g6[3], reverse_g6[3], self[e4]]) * self.group5().zyz().with_w(reverse_g3[3]))
                + (Simd32x4::from([self[e3215], reverse_g3[2], reverse_g3[0], self[e1234]]) * self.group7().xxy().with_w(self[e321]))
                + (Simd32x4::from([self[e1234], self[e1234], self[e42], self[e4125]]) * reverse_g8.xyx().with_w(reverse_g7[2]))
                + (self.group0().xx().with_zw(self[scalar], self[e12345]) * self.group1().xyz().with_w(self[e1234]))
                + (self.group4().zx().with_zw(self[e1234], self[e1]) * reverse_g8.yzz().with_w(self[e41]))
                + (self.group1().zx().with_zw(self[e321], self[e4235]) * reverse_g5.yzz().with_w(reverse_g7[0]))
                + (self.group6().ww().with_zw(self[e2], self[e1234]) * reverse_g5.xyx().with_w(self[e12345]))
                + (Simd32x3::from(self[e5]) * self.group4()).with_w(self[e2] * self[e42])
                + (reverse_g4.zxy() * self.group8().yzx()).with_w(self[scalar] * self[e4])
                + (reverse_g7.zxy() * self.group3().yzx()).with_w(reverse_g7[1] * self[e4315])
                - (Simd32x4::from([self[e5], self[e5], self[e315], self[e2]]) * reverse_g4.xyx().with_w(reverse_g4[1]))
                - (self.group1().yzxz() * reverse_g5.zxy().with_w(reverse_g4[2]))
                - (self.group0().yy().with_zw(self[e12345], reverse_g4[0]) * self.group9().xyz().with_w(self[e1]))
                - (self.group0().yy().with_zw(self[e12345], reverse_g4[0]) * self.group9().xyz().with_w(self[e415]))
                - (self.group8().zx().with_zw(self[e5], self[e425]) * reverse_g4.yzz().with_w(reverse_g4[1]))
                - (self.group3().zx().with_zw(self[e3215], self[e423]) * reverse_g7.yzz().with_w(reverse_g5[0]))
                - (self.group3().ww().with_zw(self[e4315], reverse_g6[1]) * reverse_g6.xyx().with_w(self[e42]))
                - (self.group9().zx().with_zw(self[e45], reverse_g6[2]) * reverse_g6.yzz().with_w(self[e43]))
                - (self.group9().ww().with_zw(self[e25], self[e435]) * reverse_g7.xyx().with_w(reverse_g4[2]))
                - (Simd32x3::from(self[e1234]) * self.group8()).with_w(reverse_g7[1] * self[e31])
                - (reverse_g8.zxy() * self.group4().yzx()).with_w(reverse_g5[1] * self[e431])
                - (self.group5().yzx() * self.group1().zxy()).with_w(reverse_g5[2] * self[e412])
                - (self.group7().yzx() * reverse_g3.zxy()).with_w(reverse_g7[0] * self[e23])
                - (self.group6().zxy() * self.group9().yzx()).with_w(self[e423] * self[e4235])
                - (self.group3().xyz() * self.group1().www()).with_w(reverse_g7[2] * self[e12])
                - (self.group6().xyz() * reverse_g3.www()).with_w(reverse_g6[0] * self[e41]),
            // e5
            (reverse_g3[0] * self[e1])
                + (reverse_g3[1] * self[e2])
                + (reverse_g3[2] * self[e3])
                + (reverse_g6[3] * self[e3215])
                + 2.0 * (self[scalar] * self[e5])
                + 2.0 * (self[e12345] * self[e3215])
                + (self[e5] * self[e45])
                + (self[e235] * self[e4235])
                + (self[e315] * self[e4315])
                + (self[e125] * self[e4125])
                - (reverse_g5[0] * self[e235])
                - (reverse_g5[1] * self[e315])
                - (reverse_g5[2] * self[e125])
                - (reverse_g8[0] * self[e23])
                - (reverse_g8[0] * self[e4235])
                - (reverse_g8[1] * self[e31])
                - (reverse_g8[1] * self[e4315])
                - (reverse_g8[2] * self[e12])
                - (reverse_g8[2] * self[e4125])
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
            Simd32x4::from([
                (reverse_g6[0] * self[e321]) + (reverse_g6[1] * self[e3]) + (reverse_g6[3] * self[e415]) + (self[e15] * self[e1234]),
                (reverse_g6[1] * self[e321]) + (reverse_g6[2] * self[e1]) + (reverse_g6[3] * self[e425]) + (self[e25] * self[e1234]),
                (reverse_g6[0] * self[e2]) + (reverse_g6[2] * self[e321]) + (reverse_g6[3] * self[e435]) + (self[e35] * self[e1234]),
                -(reverse_g3[1] * self[e4315]) - (reverse_g3[2] * self[e4125]) - (reverse_g3[3] * self[e3215]) - (reverse_g6[3] * self[e5]),
            ]) + (Simd32x4::from(self[scalar]) * self.group9())
                + (Simd32x4::from([reverse_g3[3], self[e4125], self[e4235], self[e25]]) * self.group5().xxy().with_w(self[e4315]))
                + (Simd32x4::from([self[e4315], reverse_g3[3], reverse_g3[3], self[e35]]) * self.group5().zyz().with_w(self[e4125]))
                + (self.group0().xx().with_zw(self[scalar], reverse_g8[0]) * self.group9().xyz().with_w(self[e415]))
                + (self.group0().yy().with_zw(self[e12345], reverse_g8[1]) * self.group1().xyz().with_w(self[e425]))
                + (self.group0().yy().with_zw(self[e12345], self[scalar]) * self.group1().xyz().with_w(self[e3215]))
                + (self.group7().zx().with_zw(self[e4], reverse_g6[2]) * reverse_g8.yzz().with_w(self[e125]))
                + (self.group1().ww().with_zw(self[e431], self[e3]) * reverse_g8.xyx().with_w(self[e125]))
                + (self.group3().zx().with_zw(self[e3215], self[e1]) * reverse_g4.yzz().with_w(self[e235]))
                + (self.group3().ww().with_zw(self[e4315], reverse_g6[0]) * reverse_g5.xyx().with_w(self[e235]))
                + (self.group9().zx().with_zw(self[e45], self[e2]) * reverse_g5.yzz().with_w(self[e315]))
                + (self.group9().ww().with_zw(self[e25], self[e435]) * reverse_g4.xyx().with_w(reverse_g8[2]))
                + (Simd32x3::from(self[e5]) * self.group7()).with_w(self[e45] * self[e3215])
                + (reverse_g7.zxy() * self.group8().yzx()).with_w(reverse_g6[1] * self[e315])
                + (self.group4().yzx() * reverse_g3.zxy()).with_w(self[e15] * self[e4235])
                + (self.group1().yzx() * self.group6().zxy()).with_w(self[e5] * self[e321])
                - (Simd32x4::from([reverse_g3[1], self[e3215], self[e3215], self[e2]]) * self.group4().zyz().with_w(reverse_g8[1]))
                - (Simd32x4::from([self[e5], self[e5], self[e315], self[e15]]) * reverse_g7.xyx().with_w(reverse_g5[0]))
                - (Simd32x4::from([self[e3215], reverse_g3[2], reverse_g3[0], self[e1]]) * self.group4().xxy().with_w(reverse_g8[0]))
                - (self.group8().zx().with_zw(self[e5], self[e25]) * reverse_g7.yzz().with_w(reverse_g5[1]))
                - (Simd32x3::from(self[e1234]) * reverse_g3.xyz()).with_w(reverse_g3[2] * self[e12])
                - (self.group8() * self.group1().www()).with_w(reverse_g3[0] * self[e23])
                - (reverse_g4.zxy() * self.group3().yzx()).with_w(self[e12345] * self[e5])
                - (reverse_g5.zxy() * self.group9().yzx()).with_w(self[e12345] * self[e5])
                - (reverse_g8.zxy() * self.group7().yzx()).with_w(reverse_g5[2] * self[e35])
                - (self.group5().yzx() * self.group9().zxy()).with_w(reverse_g8[2] * self[e3])
                - (reverse_g6.zxy() * self.group1().yzx()).with_w(reverse_g3[0] * self[e4235])
                - (self.group1().zxy() * self.group6().yzx()).with_w(reverse_g3[1] * self[e31]),
            // e1234
            (reverse_g4[0] * self[e4235])
                + (reverse_g4[1] * self[e4315])
                + (reverse_g4[2] * self[e4125])
                + (reverse_g7[0] * self[e1])
                + (reverse_g7[0] * self[e415])
                + (reverse_g7[1] * self[e2])
                + (reverse_g7[1] * self[e425])
                + (reverse_g7[2] * self[e3])
                + (reverse_g7[2] * self[e435])
                + (reverse_g3[3] * self[e1234])
                + (reverse_g6[0] * self[e423])
                + (reverse_g6[1] * self[e431])
                + (reverse_g6[2] * self[e412])
                + (reverse_g6[3] * self[e4])
                + 2.0 * (self[scalar] * self[e1234])
                - (reverse_g4[0] * self[e23])
                - (reverse_g4[1] * self[e31])
                - (reverse_g4[2] * self[e12])
                - (reverse_g5[0] * self[e41])
                - (reverse_g5[1] * self[e42])
                - (reverse_g5[2] * self[e43])
                - 2.0 * (self[e12345] * self[e4])
                - (self[e1] * self[e423])
                - (self[e2] * self[e431])
                - (self[e3] * self[e412])
                - (self[e4] * self[e321])
                - (self[e45] * self[e1234])
                - (self[e41] * self[e4235])
                - (self[e42] * self[e4315])
                - (self[e43] * self[e4125]),
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
    //      f32       15       20        0
    //    simd3        0        4        0
    //    simd4       15       15        0
    // Totals...
    // yes simd       30       39        0
    //  no simd       75       92        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let reverse_g2 = self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (reverse_g1.xyxz() * self.group1().ww().with_zw(self[e2], self[e125]))
                + (reverse_g2.xyxz() * self.group3().ww().with_zw(self[e431], self[e435]))
                + (reverse_g2.yzzw() * self.group0().zx().with_zw(self[e4], self[e321]))
                + (self.group1().xyzy() * reverse_g1.www().with_w(reverse_g2[1]))
                + (self.group2().yzxx() * reverse_g0.zxy().with_w(reverse_g1[0]))
                + (self.group3().xyxy() * self.group0().ww().with_zw(self[e425], self[e315]))
                + (self.group3().yzzz() * self.group1().zx().with_zw(self[e12345], self[e125]))
                + (self.group3().zx().with_zw(self[e321], self[e415]) * reverse_g1.yzz().with_w(reverse_g2[0]))
                + (self.group0().xyz() * reverse_g2.www()).with_w(self[e235] * self[e1])
                + (self.group3().xyz() * reverse_g0.www()).with_w(reverse_g1[1] * self[e315])
                - (reverse_g0.xyxw() * self.group2().wwyw())
                - (reverse_g2.zxyy() * self.group0().yzx().with_w(self[e2]))
                - (self.group2().zxww() * reverse_g0.yzz().with_w(reverse_g1[3]))
                - (self.group3().yzxx() * reverse_g1.zxy().with_w(reverse_g2[0]))
                - (self.group1().yzx() * self.group3().zxy()).with_w(reverse_g2[2] * self[e3])
                - (self.group2().xyz() * self.group3().www()).with_w(reverse_g2[3] * self[e12345]),
            // e1234
            (reverse_g0[0] * self[e415])
                + (reverse_g0[0] * self[e1])
                + (reverse_g0[1] * self[e425])
                + (reverse_g0[1] * self[e2])
                + (reverse_g0[2] * self[e435])
                + (reverse_g0[2] * self[e3])
                + (reverse_g1[0] * self[e423])
                + (reverse_g1[1] * self[e431])
                + (reverse_g1[2] * self[e412])
                + (reverse_g1[3] * self[e4])
                - (reverse_g0[3] * self[e4])
                - (self[e423] * self[e1])
                - (self[e431] * self[e2])
                - (self[e412] * self[e3])
                - (self[e12345] * self[e4])
                - (self[e321] * self[e4]),
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
    //      f32       27       34        0
    //    simd3        0        2        0
    //    simd4       12       13        0
    // Totals...
    // yes simd       39       49        0
    //  no simd       75       92        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let reverse_g2 = self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse_g2[2] * self[e42]) + (reverse_g2[3] * self[e15]) + (self[scalar] * self[e4235]) + (self[e12] * self[e4315]),
                (reverse_g2[0] * self[e43]) + (reverse_g2[3] * self[e25]) + (self[scalar] * self[e4315]) + (self[e23] * self[e4125]),
                (reverse_g2[1] * self[e41]) + (reverse_g2[3] * self[e35]) + (self[scalar] * self[e4125]) + (self[e31] * self[e4235]),
                -(reverse_g2[1] * self[e31]) - (reverse_g2[1] * self[e4315]) - (reverse_g2[2] * self[e12]) - (reverse_g2[2] * self[e4125]),
            ]) + (self.group1() * reverse_g1.www().with_w(self[e3215]))
                + (reverse_g0.xyxw() * self.group3().ww().with_zw(self[e25], self[e3215]))
                + (self.group1().ww().with_zw(self[e4315], self[e35]) * reverse_g1.xyx().with_w(self[e4125]))
                + (self.group2().zx().with_zw(self[e3215], self[e15]) * reverse_g0.yzz().with_w(self[e4235]))
                + (self.group3().zx().with_zw(self[e45], self[scalar]) * reverse_g1.yzz().with_w(self[e3215]))
                + (self.group3().xyz() * reverse_g0.www()).with_w(self[e25] * self[e4315])
                - (reverse_g1.zxyy() * self.group3().yzx().with_w(self[e25]))
                - (self.group1().yzxx() * self.group3().zxy().with_w(reverse_g2[0]))
                - (self.group2().yzxx() * reverse_g0.zxy().with_w(reverse_g1[0]))
                - (self.group0().zx().with_zw(self[e1234], self[e3215]) * reverse_g2.yzz().with_w(reverse_g1[3]))
                - (self.group2().ww().with_zw(self[e42], self[e35]) * reverse_g2.xyx().with_w(reverse_g1[2]))
                - (self.group0().xyz() * self.group3().www()).with_w(reverse_g2[0] * self[e4235]),
            // e1234
            (reverse_g0[0] * self[e4235])
                + (reverse_g0[1] * self[e4315])
                + (reverse_g0[2] * self[e4125])
                + (reverse_g0[3] * self[e1234])
                + (reverse_g1[3] * self[e1234])
                + (reverse_g2[3] * self[scalar])
                - (reverse_g0[0] * self[e23])
                - (reverse_g0[1] * self[e31])
                - (reverse_g0[2] * self[e12])
                - (reverse_g1[0] * self[e41])
                - (reverse_g1[1] * self[e42])
                - (reverse_g1[2] * self[e43])
                - (reverse_g2[3] * self[e45])
                - (self[e41] * self[e4235])
                - (self[e42] * self[e4315])
                - (self[e43] * self[e4125]),
        )
    }
}
