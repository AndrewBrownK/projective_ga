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
// Total Implementations: 48
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         0       1       0     N/A
//  Average:         1       3       0     N/A
//  Maximum:        20      26       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         0       4       0       0
//  Average:         5       7       0       0
//  Maximum:        58      57       0       0
impl std::ops::Add<AntiScalar> for Point {
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
            Simd32x4::from(0.0),
        )
    }
}
impl std::ops::Add<DualNum> for Point {
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
            Simd32x4::from(0.0),
        )
    }
}
impl std::ops::Add<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn add(self, other: Flector) -> Self::Output {
        Flector::from_groups(/* e1, e2, e3, e4 */ other.group0() + self.group0(), /* e423, e431, e412, e321 */ other.group1())
    }
}
impl std::ops::Add<Horizon> for Point {
    type Output = Flector;
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(/* e1, e2, e3, e4 */ self.group0(), /* e423, e431, e412, e321 */ Simd32x3::from(0.0).with_w(other[e321]))
    }
}
impl std::ops::Add<Line> for Point {
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
            Simd32x4::from(0.0),
        )
    }
}
impl std::ops::Add<Motor> for Point {
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
            Simd32x4::from(0.0),
        )
    }
}
impl std::ops::Add<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        MultiVector::from_groups(
            // scalar, e1234
            other.group0(),
            // e1, e2, e3, e4
            other.group1() + self.group0(),
            // e41, e42, e43
            other.group2(),
            // e23, e31, e12
            other.group3(),
            // e423, e431, e412, e321
            other.group4(),
        )
    }
}
impl std::ops::Add<Origin> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        0        0        0
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ self.group0().xyz().with_w(other[e4] + self[e4]))
    }
}
impl std::ops::AddAssign<Origin> for Point {
    fn add_assign(&mut self, other: Origin) {
        use crate::elements::*;
        *self = Point::from_groups(/* e1, e2, e3, e4 */ self.group0().xyz().with_w(other[e4] + self[e4]));
    }
}
impl std::ops::Add<Plane> for Point {
    type Output = Flector;
    fn add(self, other: Plane) -> Self::Output {
        Flector::from_groups(/* e1, e2, e3, e4 */ self.group0(), /* e423, e431, e412, e321 */ other.group0())
    }
}
impl std::ops::Add<Point> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn add(self, other: Point) -> Self::Output {
        Point::from_groups(/* e1, e2, e3, e4 */ other.group0() + self.group0())
    }
}
impl std::ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, other: Point) {
        *self = Point::from_groups(/* e1, e2, e3, e4 */ other.group0() + self.group0());
    }
}
impl std::ops::Add<Scalar> for Point {
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
            Simd32x4::from(0.0),
        )
    }
}
impl std::ops::BitXor<DualNum> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<DualNum> for Point {
    fn bitxor_assign(&mut self, other: DualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       13       16        0        0
    fn bitxor(self, other: Flector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Horizon> for Point {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn bitxor(self, other: Horizon) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Line> for Point {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn bitxor(self, other: Line) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       13       16        0        0
    fn bitxor(self, other: Motor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        5        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd       10       16        0      N/A
    //  no simd       25       33        0        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Origin> for Point {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        4        0        0
    fn bitxor(self, other: Origin) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Plane> for Point {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn bitxor(self, other: Plane) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Point> for Point {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        3        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        9       13        0        0
    fn bitxor(self, other: Point) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Scalar> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Scalar> for Point {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}

impl From<Origin> for Point {
    fn from(from_origin: Origin) -> Self {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(from_origin[e4]))
    }
}
impl std::ops::Mul<AntiScalar> for Point {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<DualNum> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        7        0        0
    fn mul(self, other: DualNum) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Flector> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        4        0      N/A
    //    simd4        7        3        0      N/A
    // Totals...
    // yes simd        9       11        0      N/A
    //  no simd       30       28        0        0
    fn mul(self, other: Flector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Horizon> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        5        0        0
    fn mul(self, other: Horizon) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Line> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        3        0      N/A
    //    simd4        5        2        0      N/A
    // Totals...
    // yes simd        7        9        0      N/A
    //  no simd       22       21        0        0
    fn mul(self, other: Line) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        4        0      N/A
    //    simd4        7        3        0      N/A
    // Totals...
    // yes simd        9       11        0      N/A
    //  no simd       30       28        0        0
    fn mul(self, other: Motor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        9        0        0
    //    simd2        3        6        0      N/A
    //    simd3        7        8        0      N/A
    //    simd4        7        3        0      N/A
    // Totals...
    // yes simd       20       26        0      N/A
    //  no simd       58       57        0        0
    fn mul(self, other: MultiVector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Origin> for Point {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        4        0        0
    fn mul(self, other: Origin) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Plane> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        2        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd       10       14        0        0
    fn mul(self, other: Plane) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Point> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       12       15        0        0
    fn mul(self, other: Point) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Scalar> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn mul(self, other: Scalar) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Scalar> for Point {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Neg for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn neg(self) -> Self::Output {
        Point::from_groups(/* e1, e2, e3, e4 */ self.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Not for Point {
    type Output = Plane;
    fn not(self) -> Self::Output {
        self.right_dual()
    }
}
impl std::ops::Sub<AntiScalar> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
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
            Simd32x4::from(0.0),
        )
    }
}
impl std::ops::Sub<DualNum> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        0        1        0      N/A
    // no simd        0        2        0        0
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
            Simd32x4::from(0.0),
        )
    }
}
impl std::ops::Sub<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        1        0      N/A
    // no simd        4        4        0        0
    fn sub(self, other: Flector) -> Self::Output {
        Flector::from_groups(
            // e1, e2, e3, e4
            self.group0() - other.group0(),
            // e423, e431, e412, e321
            other.group1() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Sub<Horizon> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            self.group0(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
        )
    }
}
impl std::ops::Sub<Line> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
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
            Simd32x4::from(0.0),
        )
    }
}
impl std::ops::Sub<Motor> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
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
            Simd32x4::from(0.0),
        )
    }
}
impl std::ops::Sub<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        4       12        0        0
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
            other.group4() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Sub<Origin> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        0        0        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ self.group0().xyz().with_w(self[e4] - other[e4]))
    }
}
impl std::ops::SubAssign<Origin> for Point {
    fn sub_assign(&mut self, other: Origin) {
        use crate::elements::*;
        *self = Point::from_groups(/* e1, e2, e3, e4 */ self.group0().xyz().with_w(self[e4] - other[e4]));
    }
}
impl std::ops::Sub<Plane> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn sub(self, other: Plane) -> Self::Output {
        Flector::from_groups(/* e1, e2, e3, e4 */ self.group0(), /* e423, e431, e412, e321 */ other.group0() * Simd32x4::from(-1.0))
    }
}
impl std::ops::Sub<Point> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn sub(self, other: Point) -> Self::Output {
        Point::from_groups(/* e1, e2, e3, e4 */ self.group0() - other.group0())
    }
}
impl std::ops::SubAssign<Point> for Point {
    fn sub_assign(&mut self, other: Point) {
        *self = Point::from_groups(/* e1, e2, e3, e4 */ self.group0() - other.group0());
    }
}
impl std::ops::Sub<Scalar> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
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
            Simd32x4::from(0.0),
        )
    }
}

impl TryFrom<Flector> for Point {
    type Error = String;
    fn try_from(flector: Flector) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = flector[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Flector do not fit into Point { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(Point::from_groups(/* e1, e2, e3, e4 */ flector.group0()))
    }
}

impl TryFrom<MultiVector> for Point {
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
            let mut error = "Elements from MultiVector do not fit into Point { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(Point::from_groups(/* e1, e2, e3, e4 */ multi_vector.group1()))
    }
}
