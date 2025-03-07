use crate::traits::GeometricProduct;
use crate::traits::RightDual;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 40
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         3       4       0
//  Maximum:        37      46       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       4       0
//  Average:         7       9       0
//  Maximum:        85      96       0
impl std::ops::Add<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0() + Simd32x3::from(0.0).with_w(other[e1234]),
            // e23, e31, e12, scalar
            self.group1(),
        );
    }
}
impl std::ops::AddAssign<AntiScalar> for Motor {
    fn add_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0() + Simd32x3::from(0.0).with_w(other[e1234]),
            // e23, e31, e12, scalar
            self.group1(),
        );
    }
}
impl std::ops::Add<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0().xyz().with_w(other[e1234] + self[e1234]),
            // e23, e31, e12, scalar
            self.group1().xyz().with_w(other[scalar] + self[scalar]),
        );
    }
}
impl std::ops::AddAssign<DualNum> for Motor {
    fn add_assign(&mut self, other: DualNum) {
        use crate::elements::*;
        *self = Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0().xyz().with_w(other[e1234] + self[e1234]),
            // e23, e31, e12, scalar
            self.group1().xyz().with_w(other[scalar] + self[scalar]),
        );
    }
}
impl std::ops::Add<Flector> for Motor {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[scalar], self[e1234]]),
            // e1, e2, e3, e4
            other.group0(),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e423, e431, e412, e321
            other.group1(),
        );
    }
}
impl std::ops::Add<Horizon> for Motor {
    type Output = MultiVector;
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[scalar], self[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[e321]),
        );
    }
}
impl std::ops::Add<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (other.group0() + self.group0().xyz()).with_w(self[e1234]),
            // e23, e31, e12, scalar
            (other.group1() + self.group1().xyz()).with_w(self[scalar]),
        );
    }
}
impl std::ops::AddAssign<Line> for Motor {
    fn add_assign(&mut self, other: Line) {
        use crate::elements::*;
        *self = Motor::from_groups(
            // e41, e42, e43, e1234
            (other.group0() + self.group0().xyz()).with_w(self[e1234]),
            // e23, e31, e12, scalar
            (other.group1() + self.group1().xyz()).with_w(self[scalar]),
        );
    }
}
impl std::ops::Add<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: Motor) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            other.group0() + self.group0(),
            // e23, e31, e12, scalar
            other.group1() + self.group1(),
        );
    }
}
impl std::ops::AddAssign<Motor> for Motor {
    fn add_assign(&mut self, other: Motor) {
        *self = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group0() + self.group0(),
            // e23, e31, e12, scalar
            other.group1() + self.group1(),
        );
    }
}
impl std::ops::Add<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[scalar], self[e1234]]) + other.group0(),
            // e1, e2, e3, e4
            other.group1(),
            // e41, e42, e43
            other.group2() + self.group0().xyz(),
            // e23, e31, e12
            other.group3() + self.group1().xyz(),
            // e423, e431, e412, e321
            other.group4(),
        );
    }
}
impl std::ops::Add<Origin> for Motor {
    type Output = MultiVector;
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[scalar], self[e1234]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Add<Plane> for Motor {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[scalar], self[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e423, e431, e412, e321
            other.group0(),
        );
    }
}
impl std::ops::Add<Point> for Motor {
    type Output = MultiVector;
    fn add(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[scalar], self[e1234]]),
            // e1, e2, e3, e4
            other.group0(),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Add<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0(),
            // e23, e31, e12, scalar
            self.group1() + Simd32x3::from(0.0).with_w(other[scalar]),
        );
    }
}
impl std::ops::AddAssign<Scalar> for Motor {
    fn add_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0(),
            // e23, e31, e12, scalar
            self.group1() + Simd32x3::from(0.0).with_w(other[scalar]),
        );
    }
}

impl From<AntiScalar> for Motor {
    fn from(from_anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(from_anti_scalar[e1234]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}

impl From<DualNum> for Motor {
    fn from(from_dual_num: DualNum) -> Self {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(from_dual_num[e1234]),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(from_dual_num[scalar]),
        );
    }
}

impl From<Line> for Motor {
    fn from(from_line: Line) -> Self {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            from_line.group0().with_w(0.0),
            // e23, e31, e12, scalar
            from_line.group1().with_w(0.0),
        );
    }
}

impl From<Scalar> for Motor {
    fn from(from_scalar: Scalar) -> Self {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(from_scalar[scalar]),
        );
    }
}
impl std::ops::Mul<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiScalar> for Motor {
    fn mul_assign(&mut self, other: AntiScalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        3        0
    // no simd        4       12        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DualNum> for Motor {
    fn mul_assign(&mut self, other: DualNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       18        0
    //    simd3        0        2        0
    //    simd4        8        6        0
    // Totals...
    // yes simd       20       26        0
    //  no simd       44       48        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn mul(self, other: Horizon) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       25        0
    //    simd3        0        1        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       28       36        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Line> for Motor {
    fn mul_assign(&mut self, other: Line) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       26        0
    //    simd3        0        2        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       22       32        0
    //  no simd       40       48        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Motor> for Motor {
    fn mul_assign(&mut self, other: Motor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       22        0
    //    simd2        4        4        0
    //    simd3       10       14        0
    //    simd4        8        6        0
    // Totals...
    // yes simd       37       46        0
    //  no simd       85       96        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn mul(self, other: Origin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        3        5        0
    // Totals...
    // yes simd        6       10        0
    //  no simd       12       20        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       16        0
    //    simd4        4        4        0
    // Totals...
    // yes simd        8       20        0
    //  no simd       20       32        0
    fn mul(self, other: Point) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for Motor {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Neg for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn neg(self) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            self.group1() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Not for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
            // e23, e31, e12, scalar
            self.group1(),
        );
    }
}
impl std::ops::SubAssign<AntiScalar> for Motor {
    fn sub_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
            // e23, e31, e12, scalar
            self.group1(),
        );
    }
}
impl std::ops::Sub<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        8        2        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
            // e23, e31, e12, scalar
            self.group1() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
        );
    }
}
impl std::ops::SubAssign<DualNum> for Motor {
    fn sub_assign(&mut self, other: DualNum) {
        use crate::elements::*;
        *self = Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0() + Simd32x3::from(0.0).with_w(other[e1234] * -1.0),
            // e23, e31, e12, scalar
            self.group1() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
        );
    }
}
impl std::ops::Sub<Flector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[scalar], self[e1234]]),
            // e1, e2, e3, e4
            other.group0() * Simd32x4::from(-1.0),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e423, e431, e412, e321
            other.group1() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<Horizon> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[scalar], self[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        );
    }
}
impl std::ops::Sub<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        8        6        0
    fn sub(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
        );
    }
}
impl std::ops::SubAssign<Line> for Motor {
    fn sub_assign(&mut self, other: Line) {
        use crate::elements::*;
        *self = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0]) + self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0]) + self.group1(),
        );
    }
}
impl std::ops::Sub<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn sub(self, other: Motor) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0() - other.group0(),
            // e23, e31, e12, scalar
            self.group1() - other.group1(),
        );
    }
}
impl std::ops::SubAssign<Motor> for Motor {
    fn sub_assign(&mut self, other: Motor) {
        *self = Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0() - other.group0(),
            // e23, e31, e12, scalar
            self.group1() - other.group1(),
        );
    }
}
impl std::ops::Sub<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        8        8        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[scalar], self[e1234]]) - other.group0(),
            // e1, e2, e3, e4
            other.group1() * Simd32x4::from(-1.0),
            // e41, e42, e43
            self.group0().xyz() - other.group2(),
            // e23, e31, e12
            self.group1().xyz() - other.group3(),
            // e423, e431, e412, e321
            other.group4() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<Origin> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[scalar], self[e1234]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Sub<Plane> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[scalar], self[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e423, e431, e412, e321
            other.group0() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<Point> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[scalar], self[e1234]]),
            // e1, e2, e3, e4
            other.group0() * Simd32x4::from(-1.0),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Sub<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0(),
            // e23, e31, e12, scalar
            self.group1() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
        );
    }
}
impl std::ops::SubAssign<Scalar> for Motor {
    fn sub_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0(),
            // e23, e31, e12, scalar
            self.group1() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
        );
    }
}

impl TryFrom<MultiVector> for Motor {
    type Error = String;
    fn try_from(multi_vector: MultiVector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = multi_vector[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[15];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MultiVector do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Motor::from_groups(
            // e41, e42, e43, e1234
            multi_vector.group2().with_w(multi_vector[e1234]),
            // e23, e31, e12, scalar
            multi_vector.group3().with_w(multi_vector[scalar]),
        ));
    }
}
