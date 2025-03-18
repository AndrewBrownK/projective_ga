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
//  Average:         2       9       0
//  Maximum:         6      22       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:         3       8       0
//  Average:         3      11       0
//  Maximum:         9      28       0
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
        AntiScalar::from_groups(/* e1234 */ (self[scalar] * self[e1234]) * 2.0)
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
    //      add/sub      mul      div
    // f32        2        6        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ -2.0 * (self[e41] * self[e23]) - 2.0 * (self[e42] * self[e31]) - 2.0 * (self[e43] * self[e12]))
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
    //      add/sub      mul      div
    // f32        3        8        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e1234
            2.0 * (self[e1234] * self[scalar]) - 2.0 * (self[e41] * self[e23]) - 2.0 * (self[e42] * self[e31]) - 2.0 * (self[e43] * self[e12]),
        )
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
    //      f32        5       20        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        6       22        0
    //  no simd        9       28        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                2.0 * (self[scalar] * self[e1234]) - 2.0 * (self[e41] * self[e23]) - 2.0 * (self[e42] * self[e31]) - 2.0 * (self[e43] * self[e12]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (Simd32x4::from(self[scalar]) * self.group1())
                + Simd32x4::from([
                    self[e23] * self[e321] * 2.0,
                    self[e31] * self[e321] * 2.0,
                    self[e12] * self[e321] * 2.0,
                    -2.0 * (self[e23] * self[e423]) - 2.0 * (self[e31] * self[e431]) - 2.0 * (self[e12] * self[e412]),
                ]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}
