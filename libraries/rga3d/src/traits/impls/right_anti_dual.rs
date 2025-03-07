// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 9
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       0       0
//  Maximum:         0       2       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       2       0
//  Maximum:         0       7       0
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DualNum {
    type Output = AntiScalar;
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[scalar])
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Flector {
    type Output = Flector;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl std::ops::DivAssign<RightAntiDualPrefixOrPostfix> for Flector {
    fn div_assign(&mut self, _rhs: RightAntiDualPrefixOrPostfix) {
        *self = self.right_anti_dual()
    }
}
impl RightAntiDual for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412, e321
            self.group0().xyz().with_w(0.0),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Horizon {
    type Output = Origin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Horizon {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e321] * -1.0)
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Line {
    type Output = Line;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl std::ops::DivAssign<RightAntiDualPrefixOrPostfix> for Line {
    fn div_assign(&mut self, _rhs: RightAntiDualPrefixOrPostfix) {
        *self = self.right_anti_dual()
    }
}
impl RightAntiDual for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn right_anti_dual(self) -> Self::Output {
        Line::from_groups(/* e41, e42, e43 */ self.group1() * Simd32x3::from(-1.0), /* e23, e31, e12 */ Simd32x3::from(0.0))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Motor {
    type Output = Motor;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl std::ops::DivAssign<RightAntiDualPrefixOrPostfix> for Motor {
    fn div_assign(&mut self, _rhs: RightAntiDualPrefixOrPostfix) {
        *self = self.right_anti_dual()
    }
}
impl RightAntiDual for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        Motor::from_groups(
            // e41, e42, e43, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl std::ops::DivAssign<RightAntiDualPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: RightAntiDualPrefixOrPostfix) {
        *self = self.right_anti_dual()
    }
}
impl RightAntiDual for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, self[scalar]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e41, e42, e43
            self.group3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            self.group1().xyz().with_w(0.0),
        )
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Plane {
    type Output = Origin;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Plane {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e321] * -1.0)
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Point {
    type Output = Plane;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Point {
    type Output = Plane;
    fn right_anti_dual(self) -> Self::Output {
        Plane::from_groups(/* e423, e431, e412, e321 */ self.group0().xyz().with_w(0.0))
    }
}
impl std::ops::Div<RightAntiDualPrefixOrPostfix> for Scalar {
    type Output = AntiScalar;
    fn div(self, _rhs: RightAntiDualPrefixOrPostfix) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Scalar {
    type Output = AntiScalar;
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[scalar])
    }
}
