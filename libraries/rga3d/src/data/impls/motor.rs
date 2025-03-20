use crate::traits::GeometricProduct;
use crate::traits::RightDual;
use crate::traits::Wedge;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 51
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         2       3       0
//  Maximum:        31      36       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       4       0
//  Average:         8       9       0
//  Maximum:        88      96       0
impl std::ops::Add<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0().xyz().with_w(other[e1234] + self[e1234]),
            // e23, e31, e12, scalar
            self.group1(),
        )
    }
}
impl std::ops::AddAssign<AntiScalar> for Motor {
    fn add_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0().xyz().with_w(other[e1234] + self[e1234]),
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
        Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0().xyz().with_w(other[e1234] + self[e1234]),
            // e23, e31, e12, scalar
            self.group1().xyz().with_w(other[scalar] + self[scalar]),
        )
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
        MultiVector::from_groups(
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
        )
    }
}
impl std::ops::Add<Horizon> for Motor {
    type Output = MultiVector;
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
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
        )
    }
}
impl std::ops::Add<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: Line) -> Self::Output {
        Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0() + other.group0().with_w(0.0),
            // e23, e31, e12, scalar
            self.group1() + other.group1().with_w(0.0),
        )
    }
}
impl std::ops::AddAssign<Line> for Motor {
    fn add_assign(&mut self, other: Line) {
        *self = Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0() + other.group0().with_w(0.0),
            // e23, e31, e12, scalar
            self.group1() + other.group1().with_w(0.0),
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
        Motor::from_groups(
            // e41, e42, e43, e1234
            other.group0() + self.group0(),
            // e23, e31, e12, scalar
            other.group1() + self.group1(),
        )
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
        MultiVector::from_groups(
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
        )
    }
}
impl std::ops::Add<Origin> for Motor {
    type Output = MultiVector;
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
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
        )
    }
}
impl std::ops::Add<Plane> for Motor {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
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
        )
    }
}
impl std::ops::Add<Point> for Motor {
    type Output = MultiVector;
    fn add(self, other: Point) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
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
        )
    }
}
impl std::ops::Add<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0(),
            // e23, e31, e12, scalar
            self.group1().xyz().with_w(self[scalar] + other[scalar]),
        )
    }
}
impl std::ops::AddAssign<Scalar> for Motor {
    fn add_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0(),
            // e23, e31, e12, scalar
            self.group1().xyz().with_w(self[scalar] + other[scalar]),
        );
    }
}
impl std::ops::BitXor<AntiScalar> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: AntiScalar) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1        9        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<DualNum> for Motor {
    fn bitxor_assign(&mut self, other: DualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd3        0        2        0
    //    simd4        4        3        0
    // Totals...
    // yes simd        5        7        0
    //  no simd       17       20        0
    fn bitxor(self, other: Flector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Horizon> for Motor {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: Horizon) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn bitxor(self, other: Line) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Line> for Motor {
    fn bitxor_assign(&mut self, other: Line) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        8       11        0
    //  no simd       16       21        0
    fn bitxor(self, other: Motor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Motor> for Motor {
    fn bitxor_assign(&mut self, other: Motor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       11        0
    //    simd3        2        6        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       14       20        0
    //  no simd       30       41        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Origin> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn bitxor(self, other: Origin) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Plane> for Motor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: Plane) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd3        0        2        0
    //    simd4        3        2        0
    // Totals...
    // yes simd        4        6        0
    //  no simd       13       16        0
    fn bitxor(self, other: Point) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Scalar> for Motor {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}

impl From<AntiScalar> for Motor {
    fn from(from_anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(from_anti_scalar[e1234]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}

impl From<DualNum> for Motor {
    fn from(from_dual_num: DualNum) -> Self {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(from_dual_num[e1234]),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(from_dual_num[scalar]),
        )
    }
}

impl From<Line> for Motor {
    fn from(from_line: Line) -> Self {
        Motor::from_groups(
            // e41, e42, e43, e1234
            from_line.group0().with_w(0.0),
            // e23, e31, e12, scalar
            from_line.group1().with_w(0.0),
        )
    }
}

impl From<Scalar> for Motor {
    fn from(from_scalar: Scalar) -> Self {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(from_scalar[scalar]),
        )
    }
}
impl std::ops::Mul<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        self.geometric_product(other)
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
        self.geometric_product(other)
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
    //      f32        4        5        0
    //    simd3        0        5        0
    //    simd4       11        7        0
    // Totals...
    // yes simd       15       17        0
    //  no simd       48       48        0
    fn mul(self, other: Flector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Horizon> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn mul(self, other: Horizon) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd3        0        6        0
    //    simd4        9        3        0
    // Totals...
    // yes simd       13       15        0
    //  no simd       40       36        0
    fn mul(self, other: Line) -> Self::Output {
        self.geometric_product(other)
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
    //      f32        4        6        0
    //    simd3        0        6        0
    //    simd4       12        6        0
    // Totals...
    // yes simd       16       18        0
    //  no simd       52       48        0
    fn mul(self, other: Motor) -> Self::Output {
        self.geometric_product(other)
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
    //      f32        6        8        0
    //    simd2        4        4        0
    //    simd3       10       16        0
    //    simd4       11        8        0
    // Totals...
    // yes simd       31       36        0
    //  no simd       88       96        0
    fn mul(self, other: MultiVector) -> Self::Output {
        self.geometric_product(other)
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
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Plane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd2        0        1        0
    //    simd3        4        4        0
    // Totals...
    // yes simd        7       12        0
    //  no simd       15       21        0
    fn mul(self, other: Plane) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd3        0        3        0
    //    simd4        7        4        0
    // Totals...
    // yes simd        8       11        0
    //  no simd       29       29        0
    fn mul(self, other: Point) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn mul(self, other: Scalar) -> Self::Output {
        self.geometric_product(other)
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
        Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Not for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn not(self) -> Self::Output {
        self.right_dual()
    }
}
impl std::ops::Sub<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0().xyz().with_w(self[e1234] - other[e1234]),
            // e23, e31, e12, scalar
            self.group1(),
        )
    }
}
impl std::ops::SubAssign<AntiScalar> for Motor {
    fn sub_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0().xyz().with_w(self[e1234] - other[e1234]),
            // e23, e31, e12, scalar
            self.group1(),
        );
    }
}
impl std::ops::Sub<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0().xyz().with_w(self[e1234] - other[e1234]),
            // e23, e31, e12, scalar
            self.group1().xyz().with_w(self[scalar] - other[scalar]),
        )
    }
}
impl std::ops::SubAssign<DualNum> for Motor {
    fn sub_assign(&mut self, other: DualNum) {
        use crate::elements::*;
        *self = Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0().xyz().with_w(self[e1234] - other[e1234]),
            // e23, e31, e12, scalar
            self.group1().xyz().with_w(self[scalar] - other[scalar]),
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
        MultiVector::from_groups(
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
        )
    }
}
impl std::ops::Sub<Horizon> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[scalar], self[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
        )
    }
}
impl std::ops::Sub<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn sub(self, other: Line) -> Self::Output {
        Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0() - other.group0().with_w(0.0),
            // e23, e31, e12, scalar
            self.group1() - other.group1().with_w(0.0),
        )
    }
}
impl std::ops::SubAssign<Line> for Motor {
    fn sub_assign(&mut self, other: Line) {
        *self = Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0() - other.group0().with_w(0.0),
            // e23, e31, e12, scalar
            self.group1() - other.group1().with_w(0.0),
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
        Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0() - other.group0(),
            // e23, e31, e12, scalar
            self.group1() - other.group1(),
        )
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
        MultiVector::from_groups(
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
        )
    }
}
impl std::ops::Sub<Origin> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[scalar], self[e1234]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
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
        MultiVector::from_groups(
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
        )
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
        MultiVector::from_groups(
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
        )
    }
}
impl std::ops::Sub<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0(),
            // e23, e31, e12, scalar
            self.group1().xyz().with_w(self[scalar] - other[scalar]),
        )
    }
}
impl std::ops::SubAssign<Scalar> for Motor {
    fn sub_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0(),
            // e23, e31, e12, scalar
            self.group1().xyz().with_w(self[scalar] - other[scalar]),
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
        Ok(Motor::from_groups(
            // e41, e42, e43, e1234
            multi_vector.group2().with_w(multi_vector[e1234]),
            // e23, e31, e12, scalar
            multi_vector.group3().with_w(multi_vector[scalar]),
        ))
    }
}
