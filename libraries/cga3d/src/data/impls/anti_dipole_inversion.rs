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
// Total Implementations: 112
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         3       3       0     N/A
//  Average:        11      14       0     N/A
//  Maximum:       179     205       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         8       6       0       0
//  Average:        36      36       0       0
//  Maximum:       481     482       0       0
impl std::ops::Add<AntiCircleRotor> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]),
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<AntiDipoleInversion> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        1        0        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        4        0        0      N/A
    //  no simd       15        0        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() + self.group0(),
            // e415, e425, e435, e321
            other.group1() + self.group1(),
            // e235, e315, e125, e4
            other.group2() + self.group2(),
            // e1, e2, e3, e5
            other.group3() + self.group3(),
        )
    }
}
impl std::ops::AddAssign<AntiDipoleInversion> for AntiDipoleInversion {
    fn add_assign(&mut self, other: AntiDipoleInversion) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() + self.group0(),
            // e415, e425, e435, e321
            other.group1() + self.group1(),
            // e235, e315, e125, e4
            other.group2() + self.group2(),
            // e1, e2, e3, e5
            other.group3() + self.group3(),
        );
    }
}
impl std::ops::Add<AntiDualNum> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(other[e3215]),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<AntiFlatPoint> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        5        0        0        0
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(self[e321] + other[e321]),
            // e235, e315, e125, e4
            self.group2() + other.group0().xyz().with_w(0.0),
            // e1, e2, e3, e5
            self.group3(),
        )
    }
}
impl std::ops::AddAssign<AntiFlatPoint> for AntiDipoleInversion {
    fn add_assign(&mut self, other: AntiFlatPoint) {
        use crate::elements::*;
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(self[e321] + other[e321]),
            // e235, e315, e125, e4
            self.group2() + other.group0().xyz().with_w(0.0),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Add<AntiFlector> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd        9        0        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(self[e321] + other[e321]),
            // e235, e315, e125, e4
            self.group2() + other.group0().xyz().with_w(0.0),
            // e1, e2, e3, e5
            self.group3() + other.group1(),
        )
    }
}
impl std::ops::AddAssign<AntiFlector> for AntiDipoleInversion {
    fn add_assign(&mut self, other: AntiFlector) {
        use crate::elements::*;
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(self[e321] + other[e321]),
            // e235, e315, e125, e4
            self.group2() + other.group0().xyz().with_w(0.0),
            // e1, e2, e3, e5
            self.group3() + other.group1(),
        );
    }
}
impl std::ops::Add<AntiLine> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group1().with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<AntiMotor> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group1().xyz().with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(other[e3215]),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<AntiPlane> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn add(self, other: AntiPlane) -> Self::Output {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3() + other.group0(),
        )
    }
}
impl std::ops::AddAssign<AntiPlane> for AntiDipoleInversion {
    fn add_assign(&mut self, other: AntiPlane) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3() + other.group0(),
        );
    }
}
impl std::ops::Add<AntiScalar> for AntiDipoleInversion {
    type Output = VersorEven;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().with_w(other[e12345]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        )
    }
}
impl std::ops::Add<Circle> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        1        0        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd       11        0        0        0
    fn add(self, other: Circle) -> Self::Output {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0() + other.group0(),
            // e415, e425, e435, e321
            self.group1() + other.group1(),
            // e235, e315, e125, e4
            self.group2() + other.group2().with_w(0.0),
            // e1, e2, e3, e5
            self.group3(),
        )
    }
}
impl std::ops::AddAssign<Circle> for AntiDipoleInversion {
    fn add_assign(&mut self, other: Circle) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0() + other.group0(),
            // e415, e425, e435, e321
            self.group1() + other.group1(),
            // e235, e315, e125, e4
            self.group2() + other.group2().with_w(0.0),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Add<CircleRotor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        2        0        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd       10        0        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0() + other.group0()).with_w(other[e12345]),
            // e415, e425, e435, e321
            self.group1() + other.group1(),
            // e235, e315, e125, e5
            (self.group2().xyz() + other.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        )
    }
}
impl std::ops::Add<Dipole> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]),
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<DipoleInversion> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]),
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            other.group3(),
            // e1234
            other[e1234],
        )
    }
}
impl std::ops::Add<DualNum> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        0        0        0
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().with_w(other[e12345]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e5] + other[e5]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        )
    }
}
impl std::ops::Add<FlatPoint> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<Flector> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            other.group1(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<Line> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        2        0        0      N/A
    // no simd        8        0        0        0
    fn add(self, other: Line) -> Self::Output {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1() + other.group0().with_w(0.0),
            // e235, e315, e125, e4
            self.group2() + other.group1().with_w(0.0),
            // e1, e2, e3, e5
            self.group3(),
        )
    }
}
impl std::ops::AddAssign<Line> for AntiDipoleInversion {
    fn add_assign(&mut self, other: Line) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1() + other.group0().with_w(0.0),
            // e235, e315, e125, e4
            self.group2() + other.group1().with_w(0.0),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Add<Motor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        2        0        0      N/A
    // no simd        8        0        0        0
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().with_w(other[e12345]),
            // e415, e425, e435, e321
            self.group1() + other.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            other.group1() + self.group2().xyz().with_w(self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        )
    }
}
impl std::ops::Add<MultiVector> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd3        2        0        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        5        0        0      N/A
    //  no simd       15        0        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            other.group0(),
            // e1, e2, e3, e4
            other.group1() + self.group3().xyz().with_w(self[e4]),
            // e5
            self[e5] + other[e5],
            // e15, e25, e35, e45
            other.group3(),
            // e41, e42, e43
            other.group4(),
            // e23, e31, e12
            other.group5(),
            // e415, e425, e435, e321
            self.group1() + other.group6(),
            // e423, e431, e412
            self.group0() + other.group7(),
            // e235, e315, e125
            other.group8() + self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            other.group9(),
            // e1234
            other[e1234],
        )
    }
}
impl std::ops::Add<Plane> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            other.group0(),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<RoundPoint> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd        9        0        0        0
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2().xyz().with_w(self[e4] + other[e4]),
            // e1, e2, e3, e5
            self.group3() + Simd32x3::from(0.0).with_w(other[e5]) + other.group0().xyz().with_w(0.0),
        )
    }
}
impl std::ops::AddAssign<RoundPoint> for AntiDipoleInversion {
    fn add_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2().xyz().with_w(self[e4] + other[e4]),
            // e1, e2, e3, e5
            self.group3() + Simd32x3::from(0.0).with_w(other[e5]) + other.group0().xyz().with_w(0.0),
        );
    }
}
impl std::ops::Add<Scalar> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Add<Sphere> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            other.group0(),
            // e1234
            other[e1234],
        )
    }
}
impl std::ops::Add<VersorEven> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        4        0        0      N/A
    // no simd       16        0        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() + self.group0().with_w(0.0),
            // e415, e425, e435, e321
            self.group1() + other.group1(),
            // e235, e315, e125, e5
            other.group2() + self.group2().xyz().with_w(self[e5]),
            // e1, e2, e3, e4
            other.group3() + self.group3().xyz().with_w(self[e4]),
        )
    }
}
impl std::ops::Add<VersorOdd> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from([other[e15], other[e25], other[e35], other[e45]]),
            // e41, e42, e43
            other.group0().xyz(),
            // e23, e31, e12
            other.group1().xyz(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            other.group3(),
            // e1234
            other[e1234],
        )
    }
}
impl std::ops::BitXor<AntiCircleRotor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       15        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        8        0      N/A
    //    simd4        8        3        0      N/A
    // Totals...
    // yes simd       22       28        0      N/A
    //  no simd       54       55        0        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        1        4        0      N/A
    //    simd4       11       10        0      N/A
    // Totals...
    // yes simd       18       22        0      N/A
    //  no simd       53       60        0        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiDualNum> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       16        0        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiFlatPoint> for AntiDipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3        8        0        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiFlector> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       11        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        4        0      N/A
    //    simd4        6        4        0      N/A
    // Totals...
    // yes simd       13       20        0      N/A
    //  no simd       35       41        0        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiLine> for AntiDipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        0        5        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd       18       24        0        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiMotor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       12        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        4        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       16       21        0      N/A
    //  no simd       36       40        0        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiPlane> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        7        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        4        0      N/A
    //    simd4        5        3        0      N/A
    // Totals...
    // yes simd        9       15        0      N/A
    //  no simd       28       33        0        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Circle> for AntiDipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       20       20        0        0
    fn bitxor(self, other: Circle) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<CircleRotor> for AntiDipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       20       20        0        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Dipole> for AntiDipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       12        0        0
    //    simd3        2        8        0      N/A
    //    simd4        6        1        0      N/A
    // Totals...
    // yes simd       17       21        0      N/A
    //  no simd       39       40        0        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<DipoleInversion> for AntiDipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       14        0        0
    //    simd3        2        5        0      N/A
    //    simd4        6        4        0      N/A
    // Totals...
    // yes simd       20       23        0      N/A
    //  no simd       42       45        0        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<DualNum> for AntiDipoleInversion {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<FlatPoint> for AntiDipoleInversion {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       13       16        0        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Flector> for AntiDipoleInversion {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       16       20        0        0
    fn bitxor(self, other: Flector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Line> for AntiDipoleInversion {
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
impl std::ops::BitXor<Motor> for AntiDipoleInversion {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       17       20        0        0
    fn bitxor(self, other: Motor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<MultiVector> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       25       33        0        0
    //    simd2        0        3        0      N/A
    //    simd3       10       14        0      N/A
    //    simd4       13       10        0      N/A
    // Totals...
    // yes simd       48       60        0      N/A
    //  no simd      107      121        0        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Plane> for AntiDipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn bitxor(self, other: Plane) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<RoundPoint> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        1        6        0      N/A
    //    simd4        7        4        0      N/A
    // Totals...
    // yes simd       11       16        0      N/A
    //  no simd       34       40        0        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Scalar> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       15        0        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Scalar> for AntiDipoleInversion {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Sphere> for AntiDipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        5        0        0
    fn bitxor(self, other: Sphere) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<VersorEven> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       11        0        0
    //    simd3        1        7        0      N/A
    //    simd4       11        7        0      N/A
    // Totals...
    // yes simd       19       25        0      N/A
    //  no simd       54       60        0        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<VersorOdd> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       17        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        5        0      N/A
    //    simd4        8        6        0      N/A
    // Totals...
    // yes simd       24       30        0      N/A
    //  no simd       56       60        0        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        self.wedge(other)
    }
}

impl From<AntiFlatPoint> for AntiDipoleInversion {
    fn from(from_anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flat_point[e321]),
            // e235, e315, e125, e4
            from_anti_flat_point.group0().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiFlector> for AntiDipoleInversion {
    fn from(from_anti_flector: AntiFlector) -> Self {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flector[e321]),
            // e235, e315, e125, e4
            from_anti_flector.group0().xyz().with_w(0.0),
            // e1, e2, e3, e5
            from_anti_flector.group1(),
        )
    }
}

impl From<AntiPlane> for AntiDipoleInversion {
    fn from(from_anti_plane: AntiPlane) -> Self {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            from_anti_plane.group0(),
        )
    }
}

impl From<Circle> for AntiDipoleInversion {
    fn from(from_circle: Circle) -> Self {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            from_circle.group0(),
            // e415, e425, e435, e321
            from_circle.group1(),
            // e235, e315, e125, e4
            from_circle.group2().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<Line> for AntiDipoleInversion {
    fn from(from_line: Line) -> Self {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_line.group0().with_w(0.0),
            // e235, e315, e125, e4
            from_line.group1().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<RoundPoint> for AntiDipoleInversion {
    fn from(from_round_point: RoundPoint) -> Self {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(from_round_point[e4]),
            // e1, e2, e3, e5
            from_round_point.group0().xyz().with_w(from_round_point[e5]),
        )
    }
}
impl std::ops::Mul<AntiCircleRotor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       21        0        0
    //    simd3        0       20        0      N/A
    //    simd4       41       21        0      N/A
    // Totals...
    // yes simd       52       62        0      N/A
    //  no simd      175      165        0        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiDipoleInversion> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       16        0        0
    //    simd3        0       15        0      N/A
    //    simd4       56       41        0      N/A
    // Totals...
    // yes simd       66       72        0      N/A
    //  no simd      234      225        0        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiDualNum> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        1        4        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd       15       30        0        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiFlatPoint> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       10        0        0
    //    simd2        0        2        0      N/A
    //    simd3        6        6        0      N/A
    //    simd4        7        7        0      N/A
    // Totals...
    // yes simd       19       25        0      N/A
    //  no simd       52       60        0        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiFlector> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7        9        0        0
    //    simd3        0        9        0      N/A
    //    simd4       28       21        0      N/A
    // Totals...
    // yes simd       35       39        0      N/A
    //  no simd      119      120        0        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiLine> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       19        0        0
    //    simd3        0       16        0      N/A
    //    simd4       22        6        0      N/A
    // Totals...
    // yes simd       30       41        0      N/A
    //  no simd       96       91        0        0
    fn mul(self, other: AntiLine) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiMotor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       14        0        0
    //    simd3        0       13        0      N/A
    //    simd4       30       17        0      N/A
    // Totals...
    // yes simd       38       44        0      N/A
    //  no simd      128      121        0        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiPlane> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        3        0      N/A
    //    simd4       13       12        0      N/A
    // Totals...
    // yes simd       14       18        0      N/A
    //  no simd       53       60        0        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiScalar> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        5        0      N/A
    // Totals...
    // yes simd        0        8        0      N/A
    //  no simd        0       25        0        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Circle> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       20        0        0
    //    simd3        0       18        0      N/A
    //    simd4       36       19        0      N/A
    // Totals...
    // yes simd       45       57        0      N/A
    //  no simd      153      150        0        0
    fn mul(self, other: Circle) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<CircleRotor> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       17        0        0
    //    simd3        0       15        0      N/A
    //    simd4       41       26        0      N/A
    // Totals...
    // yes simd       51       58        0      N/A
    //  no simd      174      166        0        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Dipole> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       22        0        0
    //    simd3        0       20        0      N/A
    //    simd4       37       17        0      N/A
    // Totals...
    // yes simd       48       59        0      N/A
    //  no simd      159      150        0        0
    fn mul(self, other: Dipole) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<DipoleInversion> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       21        0        0
    //    simd3        0       20        0      N/A
    //    simd4       55       36        0      N/A
    // Totals...
    // yes simd       66       77        0      N/A
    //  no simd      231      225        0        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<DualNum> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       15        0        0
    //    simd3        2        5        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        8       21        0      N/A
    //  no simd       18       34        0        0
    fn mul(self, other: DualNum) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<FlatPoint> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd3        3        7        0      N/A
    //    simd4       10        8        0      N/A
    // Totals...
    // yes simd       16       23        0      N/A
    //  no simd       52       61        0        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Flector> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       14        0        0
    //    simd3        0       13        0      N/A
    //    simd4       30       17        0      N/A
    // Totals...
    // yes simd       35       44        0      N/A
    //  no simd      125      121        0        0
    fn mul(self, other: Flector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Line> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       21        0        0
    //    simd3        0       19        0      N/A
    //    simd4       21        3        0      N/A
    // Totals...
    // yes simd       29       43        0      N/A
    //  no simd       92       90        0        0
    fn mul(self, other: Line) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Motor> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       12        0        0
    //    simd3        0       11        0      N/A
    //    simd4       30       19        0      N/A
    // Totals...
    // yes simd       37       42        0      N/A
    //  no simd      127      121        0        0
    fn mul(self, other: Motor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<MultiVector> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       55       79        0        0
    //    simd2        6       13        0      N/A
    //    simd3       58       75        0      N/A
    //    simd4       60       38        0      N/A
    // Totals...
    // yes simd      179      205        0      N/A
    //  no simd      481      482        0        0
    fn mul(self, other: MultiVector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Plane> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd3        0        7        0      N/A
    //    simd4       15        8        0      N/A
    // Totals...
    // yes simd       18       22        0      N/A
    //  no simd       63       60        0        0
    fn mul(self, other: Plane) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<RoundPoint> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        8        0        0
    //    simd3        0        7        0      N/A
    //    simd4       18       12        0      N/A
    // Totals...
    // yes simd       19       27        0      N/A
    //  no simd       73       77        0        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Scalar> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       15        0        0
    fn mul(self, other: Scalar) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Scalar> for AntiDipoleInversion {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        0        7        0      N/A
    //    simd4       19       12        0      N/A
    // Totals...
    // yes simd       21       25        0      N/A
    //  no simd       78       75        0        0
    fn mul(self, other: Sphere) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<VersorEven> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       17        0        0
    //    simd3        0       17        0      N/A
    //    simd4       60       43        0      N/A
    // Totals...
    // yes simd       73       77        0      N/A
    //  no simd      253      240        0        0
    fn mul(self, other: VersorEven) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<VersorOdd> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       19        0        0
    //    simd3        0       18        0      N/A
    //    simd4       60       42        0      N/A
    // Totals...
    // yes simd       72       79        0      N/A
    //  no simd      252      241        0        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Neg for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       15        0        0
    fn neg(self) -> Self::Output {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            self.group3() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Not for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn not(self) -> Self::Output {
        self.right_dual()
    }
}
impl std::ops::Sub<AntiCircleRotor> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        3        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       11        0        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5],
            // e15, e25, e35, e45
            (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(other[e45] * -1.0),
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<AntiDipoleInversion> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        1        0        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        4        0        0      N/A
    //  no simd       15        0        0        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0() - other.group0(),
            // e415, e425, e435, e321
            self.group1() - other.group1(),
            // e235, e315, e125, e4
            self.group2() - other.group2(),
            // e1, e2, e3, e5
            self.group3() - other.group3(),
        )
    }
}
impl std::ops::SubAssign<AntiDipoleInversion> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: AntiDipoleInversion) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0() - other.group0(),
            // e415, e425, e435, e321
            self.group1() - other.group1(),
            // e235, e315, e125, e4
            self.group2() - other.group2(),
            // e1, e2, e3, e5
            self.group3() - other.group3(),
        );
    }
}
impl std::ops::Sub<AntiDualNum> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<AntiFlatPoint> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        5        0        0        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(self[e321] - other[e321]),
            // e235, e315, e125, e4
            self.group2() - other.group0().xyz().with_w(0.0),
            // e1, e2, e3, e5
            self.group3(),
        )
    }
}
impl std::ops::SubAssign<AntiFlatPoint> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: AntiFlatPoint) {
        use crate::elements::*;
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(self[e321] - other[e321]),
            // e235, e315, e125, e4
            self.group2() - other.group0().xyz().with_w(0.0),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiFlector> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd        9        0        0        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(self[e321] - other[e321]),
            // e235, e315, e125, e4
            self.group2() - other.group0().xyz().with_w(0.0),
            // e1, e2, e3, e5
            self.group3() - other.group1(),
        )
    }
}
impl std::ops::SubAssign<AntiFlector> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: AntiFlector) {
        use crate::elements::*;
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1().xyz().with_w(self[e321] - other[e321]),
            // e235, e315, e125, e4
            self.group2() - other.group0().xyz().with_w(0.0),
            // e1, e2, e3, e5
            self.group3() - other.group1(),
        );
    }
}
impl std::ops::Sub<AntiLine> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn sub(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5],
            // e15, e25, e35, e45
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<AntiMotor> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5],
            // e15, e25, e35, e45
            (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<AntiPlane> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3() - other.group0(),
        )
    }
}
impl std::ops::SubAssign<AntiPlane> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: AntiPlane) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3() - other.group0(),
        );
    }
}
impl std::ops::Sub<AntiScalar> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        )
    }
}
impl std::ops::Sub<Circle> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        1        0        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd       11        0        0        0
    fn sub(self, other: Circle) -> Self::Output {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0() - other.group0(),
            // e415, e425, e435, e321
            self.group1() - other.group1(),
            // e235, e315, e125, e4
            self.group2() - other.group2().with_w(0.0),
            // e1, e2, e3, e5
            self.group3(),
        )
    }
}
impl std::ops::SubAssign<Circle> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: Circle) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0() - other.group0(),
            // e415, e425, e435, e321
            self.group1() - other.group1(),
            // e235, e315, e125, e4
            self.group2() - other.group2().with_w(0.0),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Sub<CircleRotor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        2        0        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        3        1        0      N/A
    //  no simd       10        1        0        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0() - other.group0()).with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            self.group1() - other.group1(),
            // e235, e315, e125, e5
            (self.group2().xyz() - other.group2().xyz()).with_w(self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        )
    }
}
impl std::ops::Sub<Dipole> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        3        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn sub(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5],
            // e15, e25, e35, e45
            (other.group2() * Simd32x3::from(-1.0)).with_w(other[e45] * -1.0),
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<DipoleInversion> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        3        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       15        0        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5],
            // e15, e25, e35, e45
            (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(other[e45] * -1.0),
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        )
    }
}
impl std::ops::Sub<DualNum> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        1        0        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e5] - other[e5]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        )
    }
}
impl std::ops::Sub<FlatPoint> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from(-1.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<Flector> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from(-1.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from(-1.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<Line> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        2        0        0      N/A
    // no simd        8        0        0        0
    fn sub(self, other: Line) -> Self::Output {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1() - other.group0().with_w(0.0),
            // e235, e315, e125, e4
            self.group2() - other.group1().with_w(0.0),
            // e1, e2, e3, e5
            self.group3(),
        )
    }
}
impl std::ops::SubAssign<Line> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: Line) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1() - other.group0().with_w(0.0),
            // e235, e315, e125, e4
            self.group2() - other.group1().with_w(0.0),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Sub<Motor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        2        1        0      N/A
    //  no simd        8        1        0        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            self.group1() - other.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e5]) - other.group1(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
        )
    }
}
impl std::ops::Sub<MultiVector> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        1        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        5        6        0      N/A
    //  no simd       15       17        0        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            other.group0() * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            self.group3().xyz().with_w(self[e4]) - other.group1(),
            // e5
            self[e5] - other[e5],
            // e15, e25, e35, e45
            other.group3() * Simd32x4::from(-1.0),
            // e41, e42, e43
            other.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() - other.group6(),
            // e423, e431, e412
            self.group0() - other.group7(),
            // e235, e315, e125
            self.group2().xyz() - other.group8(),
            // e4235, e4315, e4125, e3215
            other.group9() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        )
    }
}
impl std::ops::Sub<Plane> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn sub(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            other.group0() * Simd32x4::from(-1.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<RoundPoint> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        1        0        0
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        3        1        0      N/A
    //  no simd        9        1        0        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2().xyz().with_w(self[e4] - other[e4]),
            // e1, e2, e3, e5
            self.group3() + Simd32x3::from(0.0).with_w(other[e5] * -1.0) - other.group0().xyz().with_w(0.0),
        )
    }
}
impl std::ops::SubAssign<RoundPoint> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2().xyz().with_w(self[e4] - other[e4]),
            // e1, e2, e3, e5
            self.group3() + Simd32x3::from(0.0).with_w(other[e5] * -1.0) - other.group0().xyz().with_w(0.0),
        );
    }
}
impl std::ops::Sub<Scalar> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Sub<Sphere> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            other.group0() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        )
    }
}
impl std::ops::Sub<VersorEven> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        1        0        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        4        1        0      N/A
    //  no simd       15        1        0        0
    fn sub(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0() - other.group0().xyz()).with_w(other[e12345] * -1.0),
            // e415, e425, e435, e321
            self.group1() - other.group1(),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e5]) - other.group2(),
            // e1, e2, e3, e4
            self.group3().xyz().with_w(self[e4]) - other.group3(),
        )
    }
}
impl std::ops::Sub<VersorOdd> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        3        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        7        0      N/A
    //  no simd        0       16        0        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e5
            self[e5],
            // e15, e25, e35, e45
            (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(other[e45] * -1.0),
            // e41, e42, e43
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from(-1.0),
            // e1234
            other[e1234] * -1.0,
        )
    }
}

impl TryFrom<CircleRotor> for AntiDipoleInversion {
    type Error = String;
    fn try_from(circle_rotor: CircleRotor) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotor do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            circle_rotor.group0(),
            // e415, e425, e435, e321
            circle_rotor.group1(),
            // e235, e315, e125, e4
            circle_rotor.group2().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<DualNum> for AntiDipoleInversion {
    type Error = String;
    fn try_from(dual_num: DualNum) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dual_num[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DualNum do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(dual_num[e5]),
        ))
    }
}

impl TryFrom<Motor> for AntiDipoleInversion {
    type Error = String;
    fn try_from(motor: Motor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Motor do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            motor.group0().xyz().with_w(0.0),
            // e235, e315, e125, e4
            motor.group1().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(motor[e5]),
        ))
    }
}

impl TryFrom<MultiVector> for AntiDipoleInversion {
    type Error = String;
    fn try_from(multi_vector: MultiVector) -> Result<Self, Self::Error> {
        use crate::elements::*;
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
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[15];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[16];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[27];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[28];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[29];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[30];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[31];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MultiVector do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            multi_vector.group7(),
            // e415, e425, e435, e321
            multi_vector.group6(),
            // e235, e315, e125, e4
            multi_vector.group8().with_w(multi_vector[e4]),
            // e1, e2, e3, e5
            multi_vector.group1().xyz().with_w(multi_vector[e5]),
        ))
    }
}

impl TryFrom<VersorEven> for AntiDipoleInversion {
    type Error = String;
    fn try_from(versor_even: VersorEven) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEven do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            versor_even.group0().xyz(),
            // e415, e425, e435, e321
            versor_even.group1(),
            // e235, e315, e125, e4
            versor_even.group2().xyz().with_w(versor_even[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([versor_even[e1], versor_even[e2], versor_even[e3], versor_even[e5]]),
        ))
    }
}
