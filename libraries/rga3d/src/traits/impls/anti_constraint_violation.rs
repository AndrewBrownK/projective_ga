// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 5
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0     N/A
//   Median:         3       8       0     N/A
//  Average:         4      11       0     N/A
//  Maximum:        14      33       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0       0
//   Median:         3       8       0       0
//  Average:         5      15       0       0
//  Maximum:        21      51       0       0
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DualNum {
    type Output = Scalar;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * self[e1234] * 2.0)
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
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            2.0 * (self[e1] * self[e423]) + 2.0 * (self[e2] * self[e431]) + 2.0 * (self[e3] * self[e412]) + 2.0 * (self[e4] * self[e321]),
        )
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
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ 2.0 * (self[e41] * self[e23]) + 2.0 * (self[e42] * self[e31]) + 2.0 * (self[e43] * self[e12]))
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
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            2.0 * (self[e41] * self[e23]) + 2.0 * (self[e42] * self[e31]) + 2.0 * (self[e43] * self[e12]) + 2.0 * (self[e1234] * self[scalar]),
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
    //           add/sub      mul      div      pow
    //      f32       11       25        0        0
    //    simd3        2        6        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd       14       33        0      N/A
    //  no simd       21       51        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                2.0 * (self[e41] * self[e23])
                    + 2.0 * (self[e42] * self[e31])
                    + 2.0 * (self[e43] * self[e12])
                    + 2.0 * (self[e1] * self[e423])
                    + 2.0 * (self[e2] * self[e431])
                    + 2.0 * (self[e3] * self[e412])
                    + 2.0 * (self[e4] * self[e321])
                    + (self[scalar] * self[e1234]),
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
                + (Simd32x3::from(2.0) * (self.group2().yzx() * self.group4().zxy())
                    - Simd32x3::from(2.0) * (Simd32x3::from([self[e4], self[e412], self[e423]]) * self.group2().xxy())
                    - Simd32x3::from(2.0) * (Simd32x3::from([self[e431], self[e4], self[e4]]) * self.group2().zyz()))
                .with_w(
                    2.0 * (self[e41] * self[e1])
                        - 2.0 * (self[scalar] * self[e4])
                        - 2.0 * (self[e23] * self[e423])
                        - 2.0 * (self[e31] * self[e431])
                        - 2.0 * (self[e12] * self[e412]),
                ),
        )
    }
}
