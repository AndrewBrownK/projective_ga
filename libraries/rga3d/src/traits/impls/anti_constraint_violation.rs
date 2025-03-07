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
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DualNum {
    type Output = Scalar;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[scalar] * self[e1234] * 2.0);
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for Flector {
    type Output = Scalar;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Flector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        9        0
    //  no simd        7       12        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return Scalar::from_groups(
            // scalar
            (self[e1] * self[e423]) + (self[e2] * self[e431]) + (self[e3] * self[e412]) + (self[e4] * self[e321])
                - (anti_reverse_g0[0] * self[e423])
                - (anti_reverse_g0[1] * self[e431])
                - (anti_reverse_g0[2] * self[e412])
                - (anti_reverse_g0[3] * self[e321]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for Line {
    type Output = Scalar;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Line {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        return Scalar::from_groups(
            // scalar
            -(anti_reverse_g0[0] * self[e23])
                - (anti_reverse_g0[1] * self[e31])
                - (anti_reverse_g0[2] * self[e12])
                - (anti_reverse_g1[0] * self[e41])
                - (anti_reverse_g1[1] * self[e42])
                - (anti_reverse_g1[2] * self[e43]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for Motor {
    type Output = Scalar;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Motor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       10        0
    //  no simd        7       16        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Scalar::from_groups(
            // scalar
            (anti_reverse_g0[3] * self[scalar]) + (anti_reverse_g1[3] * self[e1234])
                - (anti_reverse_g0[0] * self[e23])
                - (anti_reverse_g0[1] * self[e31])
                - (anti_reverse_g0[2] * self[e12])
                - (anti_reverse_g1[0] * self[e41])
                - (anti_reverse_g1[1] * self[e42])
                - (anti_reverse_g1[2] * self[e43]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl std::ops::DivAssign<AntiConstraintViolationPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: AntiConstraintViolationPrefixOrPostfix) {
        *self = self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       26        0
    //    simd3        0        4        0
    //    simd4        8        7        0
    // Totals...
    // yes simd       29       37        0
    //  no simd       53       66        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let anti_reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                2.0 * (self[scalar] * self[e1234]) + (self[e1] * self[e423]) + (self[e2] * self[e431]) + (self[e3] * self[e412]) + (self[e4] * self[e321])
                    - (anti_reverse_g2[0] * self[e23])
                    - (anti_reverse_g2[1] * self[e31])
                    - (anti_reverse_g2[2] * self[e12])
                    - (anti_reverse_g3[0] * self[e41])
                    - (anti_reverse_g3[1] * self[e42])
                    - (anti_reverse_g3[2] * self[e43])
                    - (anti_reverse_g1[1] * self[e431])
                    - (anti_reverse_g1[2] * self[e412])
                    - (anti_reverse_g1[3] * self[e321])
                    - (anti_reverse_g1.xwzw()[0] * self[e423]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x4::from(self[e1234]) * self.group4())
                + (Simd32x4::from([self[e4], self[e412], self[e423], self[e321]]) * anti_reverse_g2.xxy().with_w(self[e1234]))
                + (Simd32x4::from([self[e431], self[e4], self[e4], self[e423]]) * anti_reverse_g2.zyz().with_w(anti_reverse_g3[0]))
                + (self.group0().yy().with_zw(self[e1234], self[scalar]) * self.group4().xyz().with_w(anti_reverse_g1[3]))
                + (anti_reverse_g1.ww().with_zw(self[e431], self[e431]) * self.group2().xyx().with_w(anti_reverse_g3[1]))
                + (self.group4().zx().with_zw(anti_reverse_g1[3], self[e412]) * self.group2().yzz().with_w(anti_reverse_g3[2]))
                + Simd32x3::from(0.0).with_w(
                    -(anti_reverse_g2[1] * self[e2])
                        - (anti_reverse_g2[2] * self[e3])
                        - (anti_reverse_g1[0] * self[e41])
                        - (anti_reverse_g1[1] * self[e42])
                        - (anti_reverse_g1[2] * self[e43])
                        - (self[e23] * self[e423])
                        - (self[e31] * self[e431])
                        - (self[e12] * self[e412]),
                )
                - (anti_reverse_g2.yzx() * self.group4().zxy()).with_w(self[scalar] * self[e4])
                - (self.group2().zxy() * self.group4().yzx()).with_w(anti_reverse_g2[0] * self[e1]),
        );
    }
}
