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
// Total Implementations: 50
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         1       1       0
//  Average:         3       4       0
//  Maximum:        37      45       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         1       2       0
//  Average:         8       9       0
//  Maximum:       101      98       0
impl std::ops::Add<AntiScalar> for Flector {
    type Output = MultiVector;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other[e1234]]),
            // e1, e2, e3, e4
            self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            self.group1(),
        )
    }
}
impl std::ops::Add<DualNum> for Flector {
    type Output = MultiVector;
    fn add(self, other: DualNum) -> Self::Output {
        MultiVector::from_groups(
            // scalar, e1234
            other.group0(),
            // e1, e2, e3, e4
            self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            self.group1(),
        )
    }
}
impl std::ops::Add<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: Flector) -> Self::Output {
        Flector::from_groups(
            // e1, e2, e3, e4
            other.group0() + self.group0(),
            // e423, e431, e412, e321
            other.group1() + self.group1(),
        )
    }
}
impl std::ops::AddAssign<Flector> for Flector {
    fn add_assign(&mut self, other: Flector) {
        *self = Flector::from_groups(
            // e1, e2, e3, e4
            other.group0() + self.group0(),
            // e423, e431, e412, e321
            other.group1() + self.group1(),
        );
    }
}
impl std::ops::Add<Horizon> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            self.group0(),
            // e423, e431, e412, e321
            self.group1().xyz().with_w(self[e321] + other[e321]),
        )
    }
}
impl std::ops::AddAssign<Horizon> for Flector {
    fn add_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        *self = Flector::from_groups(
            // e1, e2, e3, e4
            self.group0(),
            // e423, e431, e412, e321
            self.group1().xyz().with_w(self[e321] + other[e321]),
        );
    }
}
impl std::ops::Add<Line> for Flector {
    type Output = MultiVector;
    fn add(self, other: Line) -> Self::Output {
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group0(),
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            other.group1(),
            // e423, e431, e412, e321
            self.group1(),
        )
    }
}
impl std::ops::Add<Motor> for Flector {
    type Output = MultiVector;
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[scalar], other[e1234]]),
            // e1, e2, e3, e4
            self.group0(),
            // e41, e42, e43
            other.group0().xyz(),
            // e23, e31, e12
            other.group1().xyz(),
            // e423, e431, e412, e321
            self.group1(),
        )
    }
}
impl std::ops::Add<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        MultiVector::from_groups(
            // scalar, e1234
            other.group0(),
            // e1, e2, e3, e4
            self.group0() + other.group1(),
            // e41, e42, e43
            other.group2(),
            // e23, e31, e12
            other.group3(),
            // e423, e431, e412, e321
            self.group1() + other.group4(),
        )
    }
}
impl std::ops::Add<Origin> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            self.group0().xyz().with_w(self[e4] + other[e4]),
            // e423, e431, e412, e321
            self.group1(),
        )
    }
}
impl std::ops::AddAssign<Origin> for Flector {
    fn add_assign(&mut self, other: Origin) {
        use crate::elements::*;
        *self = Flector::from_groups(
            // e1, e2, e3, e4
            self.group0().xyz().with_w(self[e4] + other[e4]),
            // e423, e431, e412, e321
            self.group1(),
        );
    }
}
impl std::ops::Add<Plane> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Plane) -> Self::Output {
        Flector::from_groups(/* e1, e2, e3, e4 */ self.group0(), /* e423, e431, e412, e321 */ self.group1() + other.group0())
    }
}
impl std::ops::AddAssign<Plane> for Flector {
    fn add_assign(&mut self, other: Plane) {
        *self = Flector::from_groups(/* e1, e2, e3, e4 */ self.group0(), /* e423, e431, e412, e321 */ self.group1() + other.group0());
    }
}
impl std::ops::Add<Point> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Point) -> Self::Output {
        Flector::from_groups(/* e1, e2, e3, e4 */ self.group0() + other.group0(), /* e423, e431, e412, e321 */ self.group1())
    }
}
impl std::ops::AddAssign<Point> for Flector {
    fn add_assign(&mut self, other: Point) {
        *self = Flector::from_groups(/* e1, e2, e3, e4 */ self.group0() + other.group0(), /* e423, e431, e412, e321 */ self.group1());
    }
}
impl std::ops::Add<Scalar> for Flector {
    type Output = MultiVector;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            self.group1(),
        )
    }
}
impl std::ops::BitXor<DualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<DualNum> for Flector {
    fn bitxor_assign(&mut self, other: DualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        8       10        0
    //  no simd       16       20        0
    fn bitxor(self, other: Flector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Horizon> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: Horizon) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Line> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd3        0        2        0
    //    simd4        3        1        0
    // Totals...
    // yes simd        4        5        0
    //  no simd       13       12        0
    fn bitxor(self, other: Line) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd3        0        2        0
    //    simd4        4        3        0
    // Totals...
    // yes simd        5        7        0
    //  no simd       17       20        0
    fn bitxor(self, other: Motor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Motor> for Flector {
    fn bitxor_assign(&mut self, other: Motor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd2        0        1        0
    //    simd3        3        5        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       15       21        0
    //  no simd       33       41        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Origin> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bitxor(self, other: Origin) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Plane> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: Plane) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Point> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        1        3        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        5        7        0
    //  no simd       13       16        0
    fn bitxor(self, other: Point) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Scalar> for Flector {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}

impl From<Horizon> for Flector {
    fn from(from_horizon: Horizon) -> Self {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(from_horizon[e321]),
        )
    }
}

impl From<Origin> for Flector {
    fn from(from_origin: Origin) -> Self {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_origin[e4]),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}

impl From<Plane> for Flector {
    fn from(from_plane: Plane) -> Self {
        Flector::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(0.0), /* e423, e431, e412, e321 */ from_plane.group0())
    }
}

impl From<Point> for Flector {
    fn from(from_point: Point) -> Self {
        Flector::from_groups(/* e1, e2, e3, e4 */ from_point.group0(), /* e423, e431, e412, e321 */ Simd32x4::from(0.0))
    }
}
impl std::ops::Mul<AntiScalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiScalar> for Flector {
    fn mul_assign(&mut self, other: AntiScalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        1        3        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        4       12        0
    fn mul(self, other: DualNum) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<DualNum> for Flector {
    fn mul_assign(&mut self, other: DualNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        0        4        0
    //    simd4       12        8        0
    // Totals...
    // yes simd       14       16        0
    //  no simd       50       48        0
    fn mul(self, other: Flector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Horizon> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn mul(self, other: Horizon) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Line> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        7        0
    //    simd4        9        2        0
    // Totals...
    // yes simd       13       17        0
    //  no simd       40       37        0
    fn mul(self, other: Line) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Line> for Flector {
    fn mul_assign(&mut self, other: Line) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        7        0
    //    simd3        0        6        0
    //    simd4       12        6        0
    // Totals...
    // yes simd       16       19        0
    //  no simd       52       49        0
    fn mul(self, other: Motor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Motor> for Flector {
    fn mul_assign(&mut self, other: Motor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       17        0
    //    simd2        4        8        0
    //    simd3       12       15        0
    //    simd4       12        5        0
    // Totals...
    // yes simd       37       45        0
    //  no simd      101       98        0
    fn mul(self, other: MultiVector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Origin> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn mul(self, other: Origin) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Plane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        0        2        0
    //    simd4        4        3        0
    // Totals...
    // yes simd        5        8        0
    //  no simd       17       21        0
    fn mul(self, other: Plane) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Point> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        0        4        0
    //    simd4        7        3        0
    // Totals...
    // yes simd        9       11        0
    //  no simd       30       28        0
    fn mul(self, other: Point) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn mul(self, other: Scalar) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Scalar> for Flector {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Neg for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn neg(self) -> Self::Output {
        Flector::from_groups(
            // e1, e2, e3, e4
            self.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412, e321
            self.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Not for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn not(self) -> Self::Output {
        self.right_dual()
    }
}
impl std::ops::Sub<AntiScalar> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other[e1234] * -1.0]),
            // e1, e2, e3, e4
            self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            self.group1(),
        )
    }
}
impl std::ops::Sub<DualNum> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn sub(self, other: DualNum) -> Self::Output {
        MultiVector::from_groups(
            // scalar, e1234
            other.group0() * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            self.group1(),
        )
    }
}
impl std::ops::Sub<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn sub(self, other: Flector) -> Self::Output {
        Flector::from_groups(
            // e1, e2, e3, e4
            self.group0() - other.group0(),
            // e423, e431, e412, e321
            self.group1() - other.group1(),
        )
    }
}
impl std::ops::SubAssign<Flector> for Flector {
    fn sub_assign(&mut self, other: Flector) {
        *self = Flector::from_groups(
            // e1, e2, e3, e4
            self.group0() - other.group0(),
            // e423, e431, e412, e321
            self.group1() - other.group1(),
        );
    }
}
impl std::ops::Sub<Horizon> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            self.group0(),
            // e423, e431, e412, e321
            self.group1().xyz().with_w(self[e321] - other[e321]),
        )
    }
}
impl std::ops::SubAssign<Horizon> for Flector {
    fn sub_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        *self = Flector::from_groups(
            // e1, e2, e3, e4
            self.group0(),
            // e423, e431, e412, e321
            self.group1().xyz().with_w(self[e321] - other[e321]),
        );
    }
}
impl std::ops::Sub<Line> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: Line) -> Self::Output {
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            self.group0(),
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            self.group1(),
        )
    }
}
impl std::ops::Sub<Motor> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        8        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[scalar] * -1.0, other[e1234] * -1.0]),
            // e1, e2, e3, e4
            self.group0(),
            // e41, e42, e43
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            self.group1(),
        )
    }
}
impl std::ops::Sub<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        8        8        0
    fn sub(self, other: MultiVector) -> Self::Output {
        MultiVector::from_groups(
            // scalar, e1234
            other.group0() * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            self.group0() - other.group1(),
            // e41, e42, e43
            other.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group3() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            self.group1() - other.group4(),
        )
    }
}
impl std::ops::Sub<Origin> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            self.group0().xyz().with_w(self[e4] - other[e4]),
            // e423, e431, e412, e321
            self.group1(),
        )
    }
}
impl std::ops::SubAssign<Origin> for Flector {
    fn sub_assign(&mut self, other: Origin) {
        use crate::elements::*;
        *self = Flector::from_groups(
            // e1, e2, e3, e4
            self.group0().xyz().with_w(self[e4] - other[e4]),
            // e423, e431, e412, e321
            self.group1(),
        );
    }
}
impl std::ops::Sub<Plane> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: Plane) -> Self::Output {
        Flector::from_groups(/* e1, e2, e3, e4 */ self.group0(), /* e423, e431, e412, e321 */ self.group1() - other.group0())
    }
}
impl std::ops::SubAssign<Plane> for Flector {
    fn sub_assign(&mut self, other: Plane) {
        *self = Flector::from_groups(/* e1, e2, e3, e4 */ self.group0(), /* e423, e431, e412, e321 */ self.group1() - other.group0());
    }
}
impl std::ops::Sub<Point> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: Point) -> Self::Output {
        Flector::from_groups(/* e1, e2, e3, e4 */ self.group0() - other.group0(), /* e423, e431, e412, e321 */ self.group1())
    }
}
impl std::ops::SubAssign<Point> for Flector {
    fn sub_assign(&mut self, other: Point) {
        *self = Flector::from_groups(/* e1, e2, e3, e4 */ self.group0() - other.group0(), /* e423, e431, e412, e321 */ self.group1());
    }
}
impl std::ops::Sub<Scalar> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[scalar] * -1.0, 0.0]),
            // e1, e2, e3, e4
            self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            self.group1(),
        )
    }
}

impl TryFrom<MultiVector> for Flector {
    type Error = String;
    fn try_from(multi_vector: MultiVector) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = multi_vector[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MultiVector do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(Flector::from_groups(
            // e1, e2, e3, e4
            multi_vector.group1(),
            // e423, e431, e412, e321
            multi_vector.group4(),
        ))
    }
}
