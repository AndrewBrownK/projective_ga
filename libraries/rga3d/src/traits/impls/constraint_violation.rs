// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 5
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:         7       9       0
//  Average:         9      13       0
//  Maximum:        29      37       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:         7      12       0
//  Average:        14      21       0
//  Maximum:        53      66       0
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[scalar] * self[e1234] * 2.0);
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for Flector {
    type Output = AntiScalar;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        9        0
    //  no simd        7       12        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        return AntiScalar::from_groups(
            // e1234
            (reverse_g1[0] * self[e1]) + (reverse_g1[1] * self[e2]) + (reverse_g1[2] * self[e3]) + (reverse_g1[3] * self[e4])
                - (self[e1] * self[e423])
                - (self[e2] * self[e431])
                - (self[e3] * self[e412])
                - (self[e4] * self[e321]),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for Line {
    type Output = AntiScalar;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Line {
    type Output = AntiScalar;
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
        return AntiScalar::from_groups(
            // e1234
            -(reverse_g0[0] * self[e23])
                - (reverse_g0[1] * self[e31])
                - (reverse_g0[2] * self[e12])
                - (reverse_g1[0] * self[e41])
                - (reverse_g1[1] * self[e42])
                - (reverse_g1[2] * self[e43]),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for Motor {
    type Output = AntiScalar;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Motor {
    type Output = AntiScalar;
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
        return AntiScalar::from_groups(
            // e1234
            (reverse_g0[3] * self[scalar]) + (reverse_g1[3] * self[e1234])
                - (reverse_g0[0] * self[e23])
                - (reverse_g0[1] * self[e31])
                - (reverse_g0[2] * self[e12])
                - (reverse_g1[0] * self[e41])
                - (reverse_g1[1] * self[e42])
                - (reverse_g1[2] * self[e43]),
        );
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
    //      f32       21       26        0
    //    simd3        0        4        0
    //    simd4        8        7        0
    // Totals...
    // yes simd       29       37        0
    //  no simd       53       66        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        let reverse_g4 = self.group4() * Simd32x4::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (reverse_g4[0] * self[e1]) + (reverse_g4[1] * self[e2]) + (reverse_g4[2] * self[e3]) + (reverse_g4[3] * self[e4]) + 2.0 * (self[scalar] * self[e1234])
                    - (reverse_g2[0] * self[e23])
                    - (reverse_g2[1] * self[e31])
                    - (reverse_g2[2] * self[e12])
                    - (reverse_g3[0] * self[e41])
                    - (reverse_g3[1] * self[e42])
                    - (reverse_g3[2] * self[e43])
                    - (self.group4().wxzw()[1] * self[e1])
                    - (self[e2] * self[e431])
                    - (self[e3] * self[e412])
                    - (self[e4] * self[e321]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self[scalar]) * self.group1())
                + (Simd32x4::from([reverse_g4[3], self[e3], self[e1], self[e2]]) * self.group3().xxy().with_w(self[e42]))
                + (Simd32x4::from([self[e2], reverse_g4[3], reverse_g4[3], self[e3]]) * self.group3().zyz().with_w(self[e43]))
                + (self.group0().xx().with_zw(self[scalar], self[e1234]) * self.group1().xyz().with_w(self[e321]))
                + (self.group1().zx().with_zw(self[e321], self[e1]) * reverse_g3.yzz().with_w(self[e41]))
                + (self.group4().ww().with_zw(self[e2], self[e4]) * reverse_g3.xyx().with_w(self[scalar]))
                + Simd32x3::from(0.0).with_w(
                    -(reverse_g2[1] * self[e2])
                        - (reverse_g2[2] * self[e3])
                        - (reverse_g3[0] * self[e423])
                        - (reverse_g3[1] * self[e431])
                        - (reverse_g3[2] * self[e412])
                        - (reverse_g4[0] * self[e23])
                        - (reverse_g4[1] * self[e31])
                        - (reverse_g4[2] * self[e12]),
                )
                - (reverse_g3.zxy() * self.group1().yzx()).with_w(reverse_g4[3] * self[e1234])
                - (self.group3().yzx() * self.group1().zxy()).with_w(reverse_g2[0] * self[e1]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}
