// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 4
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:         3       8       0
//  Average:         6      13       0
//  Maximum:        20      36       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:         3       8       0
//  Average:         7      14       0
//  Maximum:        23      42       0
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
        Scalar::from_groups(/* scalar */ (self[scalar] * self[e1234]) * 2.0)
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
    //      add/sub      mul      div
    // f32        2        6        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -2.0 * (self[e41] * self[e23]) - 2.0 * (self[e42] * self[e31]) - 2.0 * (self[e43] * self[e12]))
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
    //      add/sub      mul      div
    // f32        3        8        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            2.0 * (self[e1234] * self[scalar]) - 2.0 * (self[e41] * self[e23]) - 2.0 * (self[e42] * self[e31]) - 2.0 * (self[e43] * self[e12]),
        )
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
    //      f32       19       34        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       20       36        0
    //  no simd       23       42        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                2.0 * (self[scalar] * self[e1234]) + (self[e1] * self[e423])
                    - (self.group1().xwzw()[0] * self[e423])
                    - 2.0 * (self[e41] * self[e23])
                    - 2.0 * (self[e42] * self[e31])
                    - 2.0 * (self[e43] * self[e12]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(2.0) * (Simd32x4::from(self[e1234]) * self.group4())
                + Simd32x4::from([
                    2.0 * (self[e4] * self[e41]) + (self.group4().zxy()[0] * self[e42]) + (self[e43] * self[e431])
                        - (self.group4().yzx()[0] * self[e43])
                        - (self.group4().zxy()[0] * self[e42]),
                    2.0 * (self[e4] * self[e42]) + (self.group4().zxy()[1] * self[e43]) + (self[e41] * self[e412])
                        - (self.group4().yzx()[1] * self[e41])
                        - (self.group4().zxy()[1] * self[e43]),
                    2.0 * (self[e4] * self[e43]) + (self.group4().zxy()[2] * self[e41]) + (self[e42] * self[e423])
                        - (self.group4().yzx()[2] * self[e42])
                        - (self.group4().zxy()[2] * self[e41]),
                    -2.0 * (self[e1] * self[e41]) - 2.0 * (self[e2] * self[e42]) - 2.0 * (self[e3] * self[e43]),
                ]),
        )
    }
}
