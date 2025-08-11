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
// Total Implementations: 56
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         1       0       0     N/A
//  Average:         6       8       0     N/A
//  Maximum:        87      99       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         2       0       0       0
//  Average:        12      15       0       0
//  Maximum:       172     186       0       0
impl std::ops::Add<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        1        0        0      N/A
    // no simd        2        0        0        0
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other[e1234]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        )
    }
}
impl std::ops::AddAssign<AntiScalar> for MultiVector {
    fn add_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other[e1234]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Add<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        1        0        0      N/A
    // no simd        2        0        0        0
    fn add(self, other: DualNum) -> Self::Output {
        MultiVector::from_groups(
            // scalar, e1234
            other.group0() + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        )
    }
}
impl std::ops::AddAssign<DualNum> for MultiVector {
    fn add_assign(&mut self, other: DualNum) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            other.group0() + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Add<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        2        0        0      N/A
    // no simd        8        0        0        0
    fn add(self, other: Flector) -> Self::Output {
        MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            other.group0() + self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            other.group1() + self.group4(),
        )
    }
}
impl std::ops::AddAssign<Flector> for MultiVector {
    fn add_assign(&mut self, other: Flector) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            other.group0() + self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            other.group1() + self.group4(),
        );
    }
}
impl std::ops::Add<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        0        0        0
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4().xyz().with_w(self[e321] + other[e321]),
        )
    }
}
impl std::ops::AddAssign<Horizon> for MultiVector {
    fn add_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4().xyz().with_w(self[e321] + other[e321]),
        );
    }
}
impl std::ops::Add<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        0        0      N/A
    // no simd        6        0        0        0
    fn add(self, other: Line) -> Self::Output {
        MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            other.group0() + self.group2(),
            // e23, e31, e12
            other.group1() + self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        )
    }
}
impl std::ops::AddAssign<Line> for MultiVector {
    fn add_assign(&mut self, other: Line) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            other.group0() + self.group2(),
            // e23, e31, e12
            other.group1() + self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Add<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd2        1        0        0      N/A
    //    simd3        2        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd        8        0        0        0
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[scalar], other[e1234]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2() + other.group0().xyz(),
            // e23, e31, e12
            self.group3() + other.group1().xyz(),
            // e423, e431, e412, e321
            self.group4(),
        )
    }
}
impl std::ops::AddAssign<Motor> for MultiVector {
    fn add_assign(&mut self, other: Motor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[scalar], other[e1234]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2() + other.group0().xyz(),
            // e23, e31, e12
            self.group3() + other.group1().xyz(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Add<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd2        1        0        0      N/A
    //    simd3        2        0        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        5        0        0      N/A
    //  no simd       16        0        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        MultiVector::from_groups(
            // scalar, e1234
            other.group0() + self.group0(),
            // e1, e2, e3, e4
            other.group1() + self.group1(),
            // e41, e42, e43
            other.group2() + self.group2(),
            // e23, e31, e12
            other.group3() + self.group3(),
            // e423, e431, e412, e321
            other.group4() + self.group4(),
        )
    }
}
impl std::ops::AddAssign<MultiVector> for MultiVector {
    fn add_assign(&mut self, other: MultiVector) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            other.group0() + self.group0(),
            // e1, e2, e3, e4
            other.group1() + self.group1(),
            // e41, e42, e43
            other.group2() + self.group2(),
            // e23, e31, e12
            other.group3() + self.group3(),
            // e423, e431, e412, e321
            other.group4() + self.group4(),
        );
    }
}
impl std::ops::Add<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        0        0        0
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(self[e4] + other[e4]),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        )
    }
}
impl std::ops::AddAssign<Origin> for MultiVector {
    fn add_assign(&mut self, other: Origin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(self[e4] + other[e4]),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Add<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn add(self, other: Plane) -> Self::Output {
        MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4() + other.group0(),
        )
    }
}
impl std::ops::AddAssign<Plane> for MultiVector {
    fn add_assign(&mut self, other: Plane) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4() + other.group0(),
        );
    }
}
impl std::ops::Add<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn add(self, other: Point) -> Self::Output {
        MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group0(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        )
    }
}
impl std::ops::AddAssign<Point> for MultiVector {
    fn add_assign(&mut self, other: Point) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group0(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Add<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        1        0        0      N/A
    // no simd        2        0        0        0
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        )
    }
}
impl std::ops::AddAssign<Scalar> for MultiVector {
    fn add_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::BitXor<AntiScalar> for MultiVector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn bitxor(self, other: AntiScalar) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        1        7        0      N/A
    //  no simd        1       17        0        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<DualNum> for MultiVector {
    fn bitxor_assign(&mut self, other: DualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       14        0        0
    //    simd3        3        5        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd       14       21        0      N/A
    //  no simd       23       37        0        0
    fn bitxor(self, other: Flector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Flector> for MultiVector {
    fn bitxor_assign(&mut self, other: Flector) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn bitxor(self, other: Horizon) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Horizon> for MultiVector {
    fn bitxor_assign(&mut self, other: Horizon) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        7       11        0      N/A
    //  no simd       11       21        0        0
    fn bitxor(self, other: Line) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Line> for MultiVector {
    fn bitxor_assign(&mut self, other: Line) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       12        0        0
    //    simd2        1        2        0      N/A
    //    simd3        4        6        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       13       21        0      N/A
    //  no simd       22       38        0        0
    fn bitxor(self, other: Motor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Motor> for MultiVector {
    fn bitxor_assign(&mut self, other: Motor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       18       23        0        0
    //    simd3       10       12        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd       31       39        0      N/A
    //  no simd       60       75        0        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<MultiVector> for MultiVector {
    fn bitxor_assign(&mut self, other: MultiVector) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       10        0        0
    fn bitxor(self, other: Origin) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Origin> for MultiVector {
    fn bitxor_assign(&mut self, other: Origin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
    fn bitxor(self, other: Plane) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Plane> for MultiVector {
    fn bitxor_assign(&mut self, other: Plane) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       10        0        0
    //    simd3        3        5        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        9       16        0      N/A
    //  no simd       15       29        0        0
    fn bitxor(self, other: Point) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Point> for MultiVector {
    fn bitxor_assign(&mut self, other: Point) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       16        0        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Scalar> for MultiVector {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}

impl From<AntiScalar> for MultiVector {
    fn from(from_anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, from_anti_scalar[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}

impl From<DualNum> for MultiVector {
    fn from(from_dual_num: DualNum) -> Self {
        MultiVector::from_groups(
            // scalar, e1234
            from_dual_num.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}

impl From<Flector> for MultiVector {
    fn from(from_flector: Flector) -> Self {
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_flector.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            from_flector.group1(),
        )
    }
}

impl From<Horizon> for MultiVector {
    fn from(from_horizon: Horizon) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(from_horizon[e321]),
        )
    }
}

impl From<Line> for MultiVector {
    fn from(from_line: Line) -> Self {
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            from_line.group0(),
            // e23, e31, e12
            from_line.group1(),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}

impl From<Motor> for MultiVector {
    fn from(from_motor: Motor) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([from_motor[scalar], from_motor[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            from_motor.group0().xyz(),
            // e23, e31, e12
            from_motor.group1().xyz(),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}

impl From<Origin> for MultiVector {
    fn from(from_origin: Origin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_origin[e4]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}

impl From<Plane> for MultiVector {
    fn from(from_plane: Plane) -> Self {
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            from_plane.group0(),
        )
    }
}

impl From<Point> for MultiVector {
    fn from(from_point: Point) -> Self {
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_point.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}

impl From<Scalar> for MultiVector {
    fn from(from_scalar: Scalar) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([from_scalar[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}
impl std::ops::Mul<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiScalar> for MultiVector {
    fn mul_assign(&mut self, other: AntiScalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        2        6        0      N/A
    // Totals...
    // yes simd        4       12        0      N/A
    //  no simd        8       24        0        0
    fn mul(self, other: DualNum) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<DualNum> for MultiVector {
    fn mul_assign(&mut self, other: DualNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       24       34        0        0
    //    simd2        4        6        0      N/A
    //    simd3       13       13        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       43       55        0      N/A
    //  no simd       79       93        0        0
    fn mul(self, other: Flector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Flector> for MultiVector {
    fn mul_assign(&mut self, other: Flector) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        2        0      N/A
    //    simd3        0        3        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        9        0      N/A
    //  no simd        0       26        0        0
    fn mul(self, other: Horizon) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Horizon> for MultiVector {
    fn mul_assign(&mut self, other: Horizon) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd2        3        3        0      N/A
    //    simd3       14       18        0      N/A
    // Totals...
    // yes simd       19       24        0      N/A
    //  no simd       50       63        0        0
    fn mul(self, other: Line) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Line> for MultiVector {
    fn mul_assign(&mut self, other: Line) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       18        0        0
    //    simd2        8       10        0      N/A
    //    simd3       13       15        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       34       45        0      N/A
    //  no simd       74       91        0        0
    fn mul(self, other: Motor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Motor> for MultiVector {
    fn mul_assign(&mut self, other: Motor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       43       53        0        0
    //    simd2        5        7        0      N/A
    //    simd3       37       37        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       87       99        0      N/A
    //  no simd      172      186        0        0
    fn mul(self, other: MultiVector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<MultiVector> for MultiVector {
    fn mul_assign(&mut self, other: MultiVector) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       10        0        0
    fn mul(self, other: Origin) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Origin> for MultiVector {
    fn mul_assign(&mut self, other: Origin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       18        0        0
    //    simd3        5        8        0      N/A
    // Totals...
    // yes simd       14       26        0      N/A
    //  no simd       24       42        0        0
    fn mul(self, other: Plane) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Plane> for MultiVector {
    fn mul_assign(&mut self, other: Plane) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       19        0        0
    //    simd3        6        8        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       19       29        0      N/A
    //  no simd       37       51        0        0
    fn mul(self, other: Point) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Point> for MultiVector {
    fn mul_assign(&mut self, other: Point) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       16        0        0
    fn mul(self, other: Scalar) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Scalar> for MultiVector {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Neg for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       16        0        0
    fn neg(self) -> Self::Output {
        MultiVector::from_groups(
            // scalar, e1234
            self.group0() * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            self.group1() * Simd32x4::from(-1.0),
            // e41, e42, e43
            self.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group3() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            self.group4() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Not for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        4        0        0
    fn not(self) -> Self::Output {
        self.right_dual()
    }
}
impl std::ops::Sub<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        1        0        0      N/A
    // Totals...
    // yes simd        1        1        0      N/A
    //  no simd        2        1        0        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other[e1234] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        )
    }
}
impl std::ops::SubAssign<AntiScalar> for MultiVector {
    fn sub_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other[e1234] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Sub<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        1        0        0      N/A
    // no simd        2        0        0        0
    fn sub(self, other: DualNum) -> Self::Output {
        MultiVector::from_groups(
            // scalar, e1234
            self.group0() - other.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        )
    }
}
impl std::ops::SubAssign<DualNum> for MultiVector {
    fn sub_assign(&mut self, other: DualNum) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0() - other.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Sub<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        2        0        0      N/A
    // no simd        8        0        0        0
    fn sub(self, other: Flector) -> Self::Output {
        MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group0(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4() - other.group1(),
        )
    }
}
impl std::ops::SubAssign<Flector> for MultiVector {
    fn sub_assign(&mut self, other: Flector) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group0(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4() - other.group1(),
        );
    }
}
impl std::ops::Sub<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        0        0        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4().xyz().with_w(self[e321] - other[e321]),
        )
    }
}
impl std::ops::SubAssign<Horizon> for MultiVector {
    fn sub_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4().xyz().with_w(self[e321] - other[e321]),
        );
    }
}
impl std::ops::Sub<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        0        0      N/A
    // no simd        6        0        0        0
    fn sub(self, other: Line) -> Self::Output {
        MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2() - other.group0(),
            // e23, e31, e12
            self.group3() - other.group1(),
            // e423, e431, e412, e321
            self.group4(),
        )
    }
}
impl std::ops::SubAssign<Line> for MultiVector {
    fn sub_assign(&mut self, other: Line) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2() - other.group0(),
            // e23, e31, e12
            self.group3() - other.group1(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Sub<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        8        0        0
    //    simd2        1        0        0      N/A
    //    simd3        2        0        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        8        8        0        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[scalar] * -1.0, other[e1234] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            Simd32x3::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0]) + self.group2(),
            // e23, e31, e12
            Simd32x3::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0]) + self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        )
    }
}
impl std::ops::SubAssign<Motor> for MultiVector {
    fn sub_assign(&mut self, other: Motor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[scalar] * -1.0, other[e1234] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            Simd32x3::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0]) + self.group2(),
            // e23, e31, e12
            Simd32x3::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0]) + self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Sub<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd2        1        0        0      N/A
    //    simd3        2        0        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        5        0        0      N/A
    //  no simd       16        0        0        0
    fn sub(self, other: MultiVector) -> Self::Output {
        MultiVector::from_groups(
            // scalar, e1234
            self.group0() - other.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group1(),
            // e41, e42, e43
            self.group2() - other.group2(),
            // e23, e31, e12
            self.group3() - other.group3(),
            // e423, e431, e412, e321
            self.group4() - other.group4(),
        )
    }
}
impl std::ops::SubAssign<MultiVector> for MultiVector {
    fn sub_assign(&mut self, other: MultiVector) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0() - other.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group1(),
            // e41, e42, e43
            self.group2() - other.group2(),
            // e23, e31, e12
            self.group3() - other.group3(),
            // e423, e431, e412, e321
            self.group4() - other.group4(),
        );
    }
}
impl std::ops::Sub<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        0        0        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(self[e4] - other[e4]),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        )
    }
}
impl std::ops::SubAssign<Origin> for MultiVector {
    fn sub_assign(&mut self, other: Origin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1().xyz().with_w(self[e4] - other[e4]),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Sub<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn sub(self, other: Plane) -> Self::Output {
        MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4() - other.group0(),
        )
    }
}
impl std::ops::SubAssign<Plane> for MultiVector {
    fn sub_assign(&mut self, other: Plane) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4() - other.group0(),
        );
    }
}
impl std::ops::Sub<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn sub(self, other: Point) -> Self::Output {
        MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group0(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        )
    }
}
impl std::ops::SubAssign<Point> for MultiVector {
    fn sub_assign(&mut self, other: Point) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group0(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Sub<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        1        0        0      N/A
    // Totals...
    // yes simd        1        1        0      N/A
    //  no simd        2        1        0        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        )
    }
}
impl std::ops::SubAssign<Scalar> for MultiVector {
    fn sub_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
