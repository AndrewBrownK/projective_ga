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
// Total Implementations: 126
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         4       1       0
//  Average:        25      34       0
//  Maximum:       392     509       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:        10       2       0
//  Average:        59      69       0
//  Maximum:       992    1083       0
impl std::ops::Add<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       12        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() + other.group2().xyz().with_w(other[e45]),
            // e41, e42, e43
            other.group0() + self.group4(),
            // e23, e31, e12
            self.group5() + other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::AddAssign<AntiCircleRotor> for MultiVector {
    fn add_assign(&mut self, other: AntiCircleRotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() + other.group2().xyz().with_w(other[e45]),
            // e41, e42, e43
            other.group0() + self.group4(),
            // e23, e31, e12
            self.group5() + other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       15        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group3().xyz().with_w(other[e4]),
            // e5
            other[e5] + self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            other.group1() + self.group6(),
            // e423, e431, e412
            other.group0() + self.group7(),
            // e235, e315, e125
            self.group8() + other.group2().xyz(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::AddAssign<AntiDipoleInversion> for MultiVector {
    fn add_assign(&mut self, other: AntiDipoleInversion) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group3().xyz().with_w(other[e4]),
            // e5
            other[e5] + self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            other.group1() + self.group6(),
            // e423, e431, e412
            other.group0() + self.group7(),
            // e235, e315, e125
            self.group8() + other.group2().xyz(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<AntiDualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd2        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        3        0        0
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9().xyz().with_w(other[e3215] + self[e3215]),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::AddAssign<AntiDualNum> for MultiVector {
    fn add_assign(&mut self, other: AntiDualNum) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9().xyz().with_w(other[e3215] + self[e3215]),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(other[e321] + self[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() + other.group0().xyz(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::AddAssign<AntiFlatPoint> for MultiVector {
    fn add_assign(&mut self, other: AntiFlatPoint) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(other[e321] + self[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() + other.group0().xyz(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<AntiFlector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        9        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group1().xyz().with_w(0.0),
            // e5
            other[e5] + self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(other[e321] + self[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() + other.group0().xyz(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::AddAssign<AntiFlector> for MultiVector {
    fn add_assign(&mut self, other: AntiFlector) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group1().xyz().with_w(0.0),
            // e5
            other[e5] + self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(other[e321] + self[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() + other.group0().xyz(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<AntiLine> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() + other.group1().with_w(0.0),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            other.group0() + self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::AddAssign<AntiLine> for MultiVector {
    fn add_assign(&mut self, other: AntiLine) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() + other.group1().with_w(0.0),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            other.group0() + self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd2        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       10        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() + other.group1().xyz().with_w(0.0),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5() + other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9().xyz().with_w(other[e3215] + self[e3215]),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::AddAssign<AntiMotor> for MultiVector {
    fn add_assign(&mut self, other: AntiMotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() + other.group1().xyz().with_w(0.0),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5() + other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9().xyz().with_w(other[e3215] + self[e3215]),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<AntiPlane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group0().xyz().with_w(0.0),
            // e5
            other[e5] + self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::AddAssign<AntiPlane> for MultiVector {
    fn add_assign(&mut self, other: AntiPlane) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group0().xyz().with_w(0.0),
            // e5
            other[e5] + self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        1        0        0
    // no simd        2        0        0
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::AddAssign<AntiScalar> for MultiVector {
    fn add_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn add(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            other.group1() + self.group6(),
            // e423, e431, e412
            other.group0() + self.group7(),
            // e235, e315, e125
            other.group2() + self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::AddAssign<Circle> for MultiVector {
    fn add_assign(&mut self, other: Circle) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            other.group1() + self.group6(),
            // e423, e431, e412
            other.group0() + self.group7(),
            // e235, e315, e125
            other.group2() + self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       12        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            other.group1() + self.group6(),
            // e423, e431, e412
            other.group0() + self.group7(),
            // e235, e315, e125
            self.group8() + other.group2().xyz(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::AddAssign<CircleRotor> for MultiVector {
    fn add_assign(&mut self, other: CircleRotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            other.group1() + self.group6(),
            // e423, e431, e412
            other.group0() + self.group7(),
            // e235, e315, e125
            self.group8() + other.group2().xyz(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() + other.group2().with_w(other[e45]),
            // e41, e42, e43
            other.group0() + self.group4(),
            // e23, e31, e12
            self.group5() + other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::AddAssign<Dipole> for MultiVector {
    fn add_assign(&mut self, other: Dipole) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() + other.group2().with_w(other[e45]),
            // e41, e42, e43
            other.group0() + self.group4(),
            // e23, e31, e12
            self.group5() + other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       15        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() + other.group2().xyz().with_w(other[e45]),
            // e41, e42, e43
            other.group0() + self.group4(),
            // e23, e31, e12
            self.group5() + other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            other.group3() + self.group9(),
            // e1234
            other[e1234] + self[e1234],
        )
    }
}
impl std::ops::AddAssign<DipoleInversion> for MultiVector {
    fn add_assign(&mut self, other: DipoleInversion) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() + other.group2().xyz().with_w(other[e45]),
            // e41, e42, e43
            other.group0() + self.group4(),
            // e23, e31, e12
            self.group5() + other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            other.group3() + self.group9(),
            // e1234
            other[e1234] + self[e1234],
        );
    }
}
impl std::ops::Add<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd2        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        3        0        0
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            other[e5] + self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::AddAssign<DualNum> for MultiVector {
    fn add_assign(&mut self, other: DualNum) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            other[e5] + self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group0() + self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::AddAssign<FlatPoint> for MultiVector {
    fn add_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group0() + self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group0() + self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            other.group1() + self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::AddAssign<Flector> for MultiVector {
    fn add_assign(&mut self, other: Flector) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            other.group0() + self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            other.group1() + self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + other.group0().with_w(0.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            other.group1() + self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::AddAssign<Line> for MultiVector {
    fn add_assign(&mut self, other: Line) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + other.group0().with_w(0.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            other.group1() + self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd2        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       10        0        0
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            other[e5] + self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + other.group0().xyz().with_w(0.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() + other.group1().xyz(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::AddAssign<Motor> for MultiVector {
    fn add_assign(&mut self, other: Motor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            other[e5] + self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + other.group0().xyz().with_w(0.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() + other.group1().xyz(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd2        1        0        0
    //    simd3        4        0        0
    //    simd4        4        0        0
    // Totals...
    // yes simd       11        0        0
    //  no simd       32        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            other.group0() + self.group0(),
            // e1, e2, e3, e4
            other.group1() + self.group1(),
            // e5
            other[e5] + self[e5],
            // e15, e25, e35, e45
            other.group3() + self.group3(),
            // e41, e42, e43
            other.group4() + self.group4(),
            // e23, e31, e12
            other.group5() + self.group5(),
            // e415, e425, e435, e321
            other.group6() + self.group6(),
            // e423, e431, e412
            other.group7() + self.group7(),
            // e235, e315, e125
            other.group8() + self.group8(),
            // e4235, e4315, e4125, e3215
            other.group9() + self.group9(),
            // e1234
            other[e1234] + self[e1234],
        )
    }
}
impl std::ops::AddAssign<MultiVector> for MultiVector {
    fn add_assign(&mut self, other: MultiVector) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            other.group0() + self.group0(),
            // e1, e2, e3, e4
            other.group1() + self.group1(),
            // e5
            other[e5] + self[e5],
            // e15, e25, e35, e45
            other.group3() + self.group3(),
            // e41, e42, e43
            other.group4() + self.group4(),
            // e23, e31, e12
            other.group5() + self.group5(),
            // e415, e425, e435, e321
            other.group6() + self.group6(),
            // e423, e431, e412
            other.group7() + self.group7(),
            // e235, e315, e125
            other.group8() + self.group8(),
            // e4235, e4315, e4125, e3215
            other.group9() + self.group9(),
            // e1234
            other[e1234] + self[e1234],
        );
    }
}
impl std::ops::Add<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9() + other.group0(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::AddAssign<Plane> for MultiVector {
    fn add_assign(&mut self, other: Plane) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9() + other.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group0(),
            // e5
            self[e5] + other[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::AddAssign<RoundPoint> for MultiVector {
    fn add_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group0(),
            // e5
            self[e5] + other[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        1        0        0
    // no simd        2        0        0
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::AddAssign<Scalar> for MultiVector {
    fn add_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9() + other.group0(),
            // e1234
            self[e1234] + other[e1234],
        )
    }
}
impl std::ops::AddAssign<Sphere> for MultiVector {
    fn add_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9() + other.group0(),
            // e1234
            self[e1234] + other[e1234],
        );
    }
}
impl std::ops::Add<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       17        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group3(),
            // e5
            self[e5] + other[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + other.group1(),
            // e423, e431, e412
            self.group7() + other.group0().xyz(),
            // e235, e315, e125
            self.group8() + other.group2().xyz(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::AddAssign<VersorEven> for MultiVector {
    fn add_assign(&mut self, other: VersorEven) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group3(),
            // e5
            self[e5] + other[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + other.group1(),
            // e423, e431, e412
            self.group7() + other.group0().xyz(),
            // e235, e315, e125
            self.group8() + other.group2().xyz(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Add<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       17        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() + other.group2().xyz().with_w(other[e45]),
            // e41, e42, e43
            self.group4() + other.group0().xyz(),
            // e23, e31, e12
            self.group5() + other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9() + other.group3(),
            // e1234
            self[e1234] + other[e1234],
        )
    }
}
impl std::ops::AddAssign<VersorOdd> for MultiVector {
    fn add_assign(&mut self, other: VersorOdd) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() + other.group2().xyz().with_w(other[e45]),
            // e41, e42, e43
            self.group4() + other.group0().xyz(),
            // e23, e31, e12
            self.group5() + other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9() + other.group3(),
            // e1234
            self[e1234] + other[e1234],
        );
    }
}
impl std::ops::BitXor<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       47        0
    //    simd3        8       14        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       46       67        0
    //  no simd       80      113        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<AntiCircleRotor> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiCircleRotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       35        0
    //    simd3        8       15        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       43       60        0
    //  no simd       89      120        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<AntiDipoleInversion> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiDipoleInversion) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        7        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       15        0
    //  no simd        2       35        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<AntiDualNum> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       17        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<AntiFlatPoint> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiFlatPoint) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        0
    //    simd3        6       15        0
    //    simd4        6        3        0
    // Totals...
    // yes simd       26       41        0
    //  no simd       56       80        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<AntiFlector> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiFlector) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd3        2        7        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       19       32        0
    //  no simd       26       49        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<AntiLine> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiLine) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       31        0
    //    simd3        6       13        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       26       47        0
    //  no simd       50       82        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<AntiMotor> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiMotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       22        0
    //    simd3        5       11        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       15       36        0
    //  no simd       34       67        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<AntiPlane> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiPlane) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiScalar> for MultiVector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: AntiScalar) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       22        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       18       27        0
    //  no simd       24       40        0
    fn bitxor(self, other: Circle) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Circle> for MultiVector {
    fn bitxor_assign(&mut self, other: Circle) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       23        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       25       41        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<CircleRotor> for MultiVector {
    fn bitxor_assign(&mut self, other: CircleRotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       41        0
    //    simd3        4        9        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       37       53        0
    //  no simd       54       80        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Dipole> for MultiVector {
    fn bitxor_assign(&mut self, other: Dipole) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       48        0
    //    simd3        4       10        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       44       61        0
    //  no simd       64       90        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<DipoleInversion> for MultiVector {
    fn bitxor_assign(&mut self, other: DipoleInversion) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        7        0
    //  no simd        1       17        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<DualNum> for MultiVector {
    fn bitxor_assign(&mut self, other: DualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd3        2        4        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       17       32        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<FlatPoint> for MultiVector {
    fn bitxor_assign(&mut self, other: FlatPoint) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       17        0
    //    simd3        2        5        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       15       24        0
    //  no simd       25       40        0
    fn bitxor(self, other: Flector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Flector> for MultiVector {
    fn bitxor_assign(&mut self, other: Flector) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       14        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       17        0
    //  no simd       13       24        0
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
    //           add/sub      mul      div
    //      f32       11       18        0
    //    simd3        2        5        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       15       25        0
    //  no simd       25       41        0
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
    //           add/sub      mul      div
    //      f32       71       94        0
    //    simd3       20       32        0
    //    simd4       20       14        0
    // Totals...
    // yes simd      111      140        0
    //  no simd      211      246        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<MultiVector> for MultiVector {
    fn bitxor_assign(&mut self, other: MultiVector) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bitxor(self, other: Plane) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Plane> for MultiVector {
    fn bitxor_assign(&mut self, other: Plane) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       27        0
    //    simd3        6       10        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       25       43        0
    //  no simd       49       81        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<RoundPoint> for MultiVector {
    fn bitxor_assign(&mut self, other: RoundPoint) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       32        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Scalar> for MultiVector {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        4       10        0
    fn bitxor(self, other: Sphere) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Sphere> for MultiVector {
    fn bitxor_assign(&mut self, other: Sphere) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       40        0
    //    simd3        8       17        0
    //    simd4       10        8        0
    // Totals...
    // yes simd       44       65        0
    //  no simd       90      123        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<VersorEven> for MultiVector {
    fn bitxor_assign(&mut self, other: VersorEven) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       52        0
    //    simd3        8       14        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       53       73        0
    //  no simd       90      122        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<VersorOdd> for MultiVector {
    fn bitxor_assign(&mut self, other: VersorOdd) {
        *self = self.wedge(other);
    }
}

impl From<AntiCircleRotor> for MultiVector {
    fn from(from_anti_circle_rotor: AntiCircleRotor) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_circle_rotor[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([from_anti_circle_rotor[e15], from_anti_circle_rotor[e25], from_anti_circle_rotor[e35], from_anti_circle_rotor[e45]]),
            // e41, e42, e43
            from_anti_circle_rotor.group0(),
            // e23, e31, e12
            from_anti_circle_rotor.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}

impl From<AntiDipoleInversion> for MultiVector {
    fn from(from_anti_dipole_inversion: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([
                from_anti_dipole_inversion[e1],
                from_anti_dipole_inversion[e2],
                from_anti_dipole_inversion[e3],
                from_anti_dipole_inversion[e4],
            ]),
            // e5
            from_anti_dipole_inversion[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_anti_dipole_inversion.group1(),
            // e423, e431, e412
            from_anti_dipole_inversion.group0(),
            // e235, e315, e125
            from_anti_dipole_inversion.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}

impl From<AntiDualNum> for MultiVector {
    fn from(from_anti_dual_num: AntiDualNum) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_dual_num[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_anti_dual_num[e3215]),
            // e1234
            0.0,
        )
    }
}

impl From<AntiFlatPoint> for MultiVector {
    fn from(from_anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flat_point[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_anti_flat_point.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}

impl From<AntiFlector> for MultiVector {
    fn from(from_anti_flector: AntiFlector) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_anti_flector.group1().xyz().with_w(0.0),
            // e5
            from_anti_flector[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flector[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_anti_flector.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}

impl From<AntiLine> for MultiVector {
    fn from(from_anti_line: AntiLine) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            from_anti_line.group1().with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_anti_line.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}

impl From<AntiMotor> for MultiVector {
    fn from(from_anti_motor: AntiMotor) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_motor[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            from_anti_motor.group1().xyz().with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_anti_motor.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_anti_motor[e3215]),
            // e1234
            0.0,
        )
    }
}

impl From<AntiPlane> for MultiVector {
    fn from(from_anti_plane: AntiPlane) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_anti_plane.group0().xyz().with_w(0.0),
            // e5
            from_anti_plane[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}

impl From<AntiScalar> for MultiVector {
    fn from(from_anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_anti_scalar[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}

impl From<Circle> for MultiVector {
    fn from(from_circle: Circle) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_circle.group1(),
            // e423, e431, e412
            from_circle.group0(),
            // e235, e315, e125
            from_circle.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}

impl From<CircleRotor> for MultiVector {
    fn from(from_circle_rotor: CircleRotor) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_circle_rotor[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_circle_rotor.group1(),
            // e423, e431, e412
            from_circle_rotor.group0(),
            // e235, e315, e125
            from_circle_rotor.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}

impl From<Dipole> for MultiVector {
    fn from(from_dipole: Dipole) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([from_dipole[e15], from_dipole[e25], from_dipole[e35], from_dipole[e45]]),
            // e41, e42, e43
            from_dipole.group0(),
            // e23, e31, e12
            from_dipole.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}

impl From<DipoleInversion> for MultiVector {
    fn from(from_dipole_inversion: DipoleInversion) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([from_dipole_inversion[e15], from_dipole_inversion[e25], from_dipole_inversion[e35], from_dipole_inversion[e45]]),
            // e41, e42, e43
            from_dipole_inversion.group0(),
            // e23, e31, e12
            from_dipole_inversion.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            from_dipole_inversion.group3(),
            // e1234
            from_dipole_inversion[e1234],
        )
    }
}

impl From<DualNum> for MultiVector {
    fn from(from_dual_num: DualNum) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_dual_num[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            from_dual_num[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}

impl From<FlatPoint> for MultiVector {
    fn from(from_flat_point: FlatPoint) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            from_flat_point.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}

impl From<Flector> for MultiVector {
    fn from(from_flector: Flector) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            from_flector.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            from_flector.group1(),
            // e1234
            0.0,
        )
    }
}

impl From<Line> for MultiVector {
    fn from(from_line: Line) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_line.group0().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_line.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}

impl From<Motor> for MultiVector {
    fn from(from_motor: Motor) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_motor[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            from_motor[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_motor.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_motor.group1().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}

impl From<Plane> for MultiVector {
    fn from(from_plane: Plane) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            from_plane.group0(),
            // e1234
            0.0,
        )
    }
}

impl From<RoundPoint> for MultiVector {
    fn from(from_round_point: RoundPoint) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_round_point.group0(),
            // e5
            from_round_point[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}

impl From<Scalar> for MultiVector {
    fn from(from_scalar: Scalar) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_scalar[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}

impl From<Sphere> for MultiVector {
    fn from(from_sphere: Sphere) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            from_sphere.group0(),
            // e1234
            from_sphere[e1234],
        )
    }
}

impl From<VersorEven> for MultiVector {
    fn from(from_versor_even: VersorEven) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_versor_even[e12345]]),
            // e1, e2, e3, e4
            from_versor_even.group3(),
            // e5
            from_versor_even[e5],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_versor_even.group1(),
            // e423, e431, e412
            from_versor_even.group0().xyz(),
            // e235, e315, e125
            from_versor_even.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}

impl From<VersorOdd> for MultiVector {
    fn from(from_versor_odd: VersorOdd) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_versor_odd[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([from_versor_odd[e15], from_versor_odd[e25], from_versor_odd[e35], from_versor_odd[e45]]),
            // e41, e42, e43
            from_versor_odd.group0().xyz(),
            // e23, e31, e12
            from_versor_odd.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            from_versor_odd.group3(),
            // e1234
            from_versor_odd[e1234],
        )
    }
}
impl std::ops::Mul<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64      101        0
    //    simd2       10       10        0
    //    simd3       40       49        0
    //    simd4       29       24        0
    // Totals...
    // yes simd      143      184        0
    //  no simd      320      364        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiCircleRotor> for MultiVector {
    fn mul_assign(&mut self, other: AntiCircleRotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       97        0
    //    simd2        6        6        0
    //    simd3       56       72        0
    //    simd4       56       44        0
    // Totals...
    // yes simd      162      219        0
    //  no simd      448      501        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiDipoleInversion> for MultiVector {
    fn mul_assign(&mut self, other: AntiDipoleInversion) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       23        0
    //    simd2        1        2        0
    //    simd3        4        9        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       14       38        0
    //  no simd       32       70        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiDualNum> for MultiVector {
    fn mul_assign(&mut self, other: AntiDualNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       34        0
    //    simd2        1        1        0
    //    simd3       12       17        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       42       63        0
    //  no simd      100      131        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiFlatPoint> for MultiVector {
    fn mul_assign(&mut self, other: AntiFlatPoint) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       68        0
    //    simd2        4        4        0
    //    simd3       24       31        0
    //    simd4       26       23        0
    // Totals...
    // yes simd       99      126        0
    //  no simd      229      261        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiFlector> for MultiVector {
    fn mul_assign(&mut self, other: AntiFlector) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       78        0
    //    simd2        5        6        0
    //    simd3       17       28        0
    //    simd4       13        6        0
    // Totals...
    // yes simd       83      118        0
    //  no simd      161      198        0
    fn mul(self, other: AntiLine) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiLine> for MultiVector {
    fn mul_assign(&mut self, other: AntiLine) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       54       84        0
    //    simd2        7        8        0
    //    simd3       24       33        0
    //    simd4       21       16        0
    // Totals...
    // yes simd      106      141        0
    //  no simd      224      263        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiMotor> for MultiVector {
    fn mul_assign(&mut self, other: AntiMotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       26        0
    //    simd2        3        3        0
    //    simd3       11       15        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       34       58        0
    //  no simd      101      133        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiPlane> for MultiVector {
    fn mul_assign(&mut self, other: AntiPlane) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd2        0        2        0
    //    simd3        0        6        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        0       19        0
    //  no simd        0       51        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiScalar> for MultiVector {
    fn mul_assign(&mut self, other: AntiScalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       77        0
    //    simd2        1        1        0
    //    simd3       36       49        0
    //    simd4       35       28        0
    // Totals...
    // yes simd      110      155        0
    //  no simd      288      338        0
    fn mul(self, other: Circle) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Circle> for MultiVector {
    fn mul_assign(&mut self, other: Circle) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       78        0
    //    simd2        3        3        0
    //    simd3       40       54        0
    //    simd4       40       31        0
    // Totals...
    // yes simd      117      166        0
    //  no simd      320      370        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<CircleRotor> for MultiVector {
    fn mul_assign(&mut self, other: CircleRotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       62       98        0
    //    simd2        9        9        0
    //    simd3       36       45        0
    //    simd4       25       20        0
    // Totals...
    // yes simd      132      172        0
    //  no simd      288      331        0
    fn mul(self, other: Dipole) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Dipole> for MultiVector {
    fn mul_assign(&mut self, other: Dipole) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       74      127        0
    //    simd2       11       11        0
    //    simd3       56       70        0
    //    simd4       46       36        0
    // Totals...
    // yes simd      187      244        0
    //  no simd      448      503        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<DipoleInversion> for MultiVector {
    fn mul_assign(&mut self, other: DipoleInversion) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       28        0
    //    simd3        4        9        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       15       40        0
    //  no simd       32       67        0
    fn mul(self, other: DualNum) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<DualNum> for MultiVector {
    fn mul_assign(&mut self, other: DualNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       40        0
    //    simd2        3        3        0
    //    simd3       12       17        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       46       68        0
    //  no simd       97      129        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<FlatPoint> for MultiVector {
    fn mul_assign(&mut self, other: FlatPoint) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       65        0
    //    simd2        4        4        0
    //    simd3       24       37        0
    //    simd4       27       19        0
    // Totals...
    // yes simd       96      125        0
    //  no simd      229      260        0
    fn mul(self, other: Flector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Flector> for MultiVector {
    fn mul_assign(&mut self, other: Flector) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       73        0
    //    simd3       17       25        0
    //    simd4       19       15        0
    // Totals...
    // yes simd       73      113        0
    //  no simd      164      208        0
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
    //           add/sub      mul      div
    //      f32       45       60        0
    //    simd2        4        4        0
    //    simd3       24       28        0
    //    simd4       26       26        0
    // Totals...
    // yes simd       99      118        0
    //  no simd      229      256        0
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
    //           add/sub      mul      div
    //      f32      140      253        0
    //    simd2       16       16        0
    //    simd3      124      162        0
    //    simd4      112       78        0
    // Totals...
    // yes simd      392      509        0
    //  no simd      992     1083        0
    fn mul(self, other: MultiVector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<MultiVector> for MultiVector {
    fn mul_assign(&mut self, other: MultiVector) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       50        0
    //    simd2        1        1        0
    //    simd3       11       15        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       49       74        0
    //  no simd       96      129        0
    fn mul(self, other: Plane) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Plane> for MultiVector {
    fn mul_assign(&mut self, other: Plane) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       46        0
    //    simd2        3        3        0
    //    simd3       16       22        0
    //    simd4       15       13        0
    // Totals...
    // yes simd       48       84        0
    //  no simd      128      170        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<RoundPoint> for MultiVector {
    fn mul_assign(&mut self, other: RoundPoint) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       32        0
    fn mul(self, other: Scalar) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Scalar> for MultiVector {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       45        0
    //    simd2        2        2        0
    //    simd3       16       24        0
    //    simd4       14       11        0
    // Totals...
    // yes simd       52       82        0
    //  no simd      128      165        0
    fn mul(self, other: Sphere) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Sphere> for MultiVector {
    fn mul_assign(&mut self, other: Sphere) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       99        0
    //    simd2        8        8        0
    //    simd3       60       84        0
    //    simd4       59       42        0
    // Totals...
    // yes simd      175      233        0
    //  no simd      480      535        0
    fn mul(self, other: VersorEven) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<VersorEven> for MultiVector {
    fn mul_assign(&mut self, other: VersorEven) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       76      122        0
    //    simd2       12       12        0
    //    simd3       60       73        0
    //    simd4       50       41        0
    // Totals...
    // yes simd      198      248        0
    //  no simd      480      529        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<VersorOdd> for MultiVector {
    fn mul_assign(&mut self, other: VersorOdd) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Neg for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       32        0
    fn neg(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0() * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            self.group1() * Simd32x4::from(-1.0),
            // e5
            self[e5] * -1.0,
            // e15, e25, e35, e45
            self.group3() * Simd32x4::from(-1.0),
            // e41, e42, e43
            self.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group6() * Simd32x4::from(-1.0),
            // e423, e431, e412
            self.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group8() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group9() * Simd32x4::from(-1.0),
            // e1234
            self[e1234] * -1.0,
        )
    }
}
impl std::ops::Not for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       17        0
    fn not(self) -> Self::Output {
        self.right_dual()
    }
}
impl std::ops::Sub<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        1        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        1        0
    //  no simd       12        2        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0() + (Simd32x2::from([other[scalar], 0.0]) * Simd32x2::from([-1.0, 0.0])),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() - other.group2().xyz().with_w(other[e45]),
            // e41, e42, e43
            self.group4() - other.group0(),
            // e23, e31, e12
            self.group5() - other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::SubAssign<AntiCircleRotor> for MultiVector {
    fn sub_assign(&mut self, other: AntiCircleRotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0() + (Simd32x2::from([other[scalar], 0.0]) * Simd32x2::from([-1.0, 0.0])),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() - other.group2().xyz().with_w(other[e45]),
            // e41, e42, e43
            self.group4() - other.group0(),
            // e23, e31, e12
            self.group5() - other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       15        0        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group3().xyz().with_w(other[e4]),
            // e5
            self[e5] - other[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group1(),
            // e423, e431, e412
            self.group7() - other.group0(),
            // e235, e315, e125
            self.group8() - other.group2().xyz(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::SubAssign<AntiDipoleInversion> for MultiVector {
    fn sub_assign(&mut self, other: AntiDipoleInversion) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group3().xyz().with_w(other[e4]),
            // e5
            self[e5] - other[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group1(),
            // e423, e431, e412
            self.group7() - other.group0(),
            // e235, e315, e125
            self.group8() - other.group2().xyz(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<AntiDualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd2        1        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        3        2        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0() + (Simd32x2::from([other[scalar], 0.0]) * Simd32x2::from([-1.0, 0.0])),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9().xyz().with_w(self[e3215] - other[e3215]),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::SubAssign<AntiDualNum> for MultiVector {
    fn sub_assign(&mut self, other: AntiDualNum) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0() + (Simd32x2::from([other[scalar], 0.0]) * Simd32x2::from([-1.0, 0.0])),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9().xyz().with_w(self[e3215] - other[e3215]),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(self[e321] - other[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group0().xyz(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::SubAssign<AntiFlatPoint> for MultiVector {
    fn sub_assign(&mut self, other: AntiFlatPoint) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(self[e321] - other[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group0().xyz(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<AntiFlector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        1        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        9        3        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            self[e5] - other[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(self[e321] - other[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group0().xyz(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::SubAssign<AntiFlector> for MultiVector {
    fn sub_assign(&mut self, other: AntiFlector) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            self[e5] - other[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6().xyz().with_w(self[e321] - other[e321]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group0().xyz(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<AntiLine> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        7        3        0
    fn sub(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() + (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5() - other.group0(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::SubAssign<AntiLine> for MultiVector {
    fn sub_assign(&mut self, other: AntiLine) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() + (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5() - other.group0(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd2        1        1        0
    //    simd3        1        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        2        0
    //  no simd       10        5        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0() + (Simd32x2::from([other[scalar], 0.0]) * Simd32x2::from([-1.0, 0.0])),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() + (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5() - other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9().xyz().with_w(self[e3215] - other[e3215]),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::SubAssign<AntiMotor> for MultiVector {
    fn sub_assign(&mut self, other: AntiMotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0() + (Simd32x2::from([other[scalar], 0.0]) * Simd32x2::from([-1.0, 0.0])),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() + (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5() - other.group0().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9().xyz().with_w(self[e3215] - other[e3215]),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<AntiPlane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        5        3        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            self[e5] - other[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::SubAssign<AntiPlane> for MultiVector {
    fn sub_assign(&mut self, other: AntiPlane) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            self[e5] - other[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        1        1        0
    // no simd        2        2        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0() + (Simd32x2::from([0.0, other[e12345]]) * Simd32x2::from([0.0, -1.0])),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::SubAssign<AntiScalar> for MultiVector {
    fn sub_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0() + (Simd32x2::from([0.0, other[e12345]]) * Simd32x2::from([0.0, -1.0])),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn sub(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group1(),
            // e423, e431, e412
            self.group7() - other.group0(),
            // e235, e315, e125
            self.group8() - other.group2(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::SubAssign<Circle> for MultiVector {
    fn sub_assign(&mut self, other: Circle) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group1(),
            // e423, e431, e412
            self.group7() - other.group0(),
            // e235, e315, e125
            self.group8() - other.group2(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        1        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        1        0
    //  no simd       12        2        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0() + (Simd32x2::from([0.0, other[e12345]]) * Simd32x2::from([0.0, -1.0])),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group1(),
            // e423, e431, e412
            self.group7() - other.group0(),
            // e235, e315, e125
            self.group8() - other.group2().xyz(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::SubAssign<CircleRotor> for MultiVector {
    fn sub_assign(&mut self, other: CircleRotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0() + (Simd32x2::from([0.0, other[e12345]]) * Simd32x2::from([0.0, -1.0])),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group1(),
            // e423, e431, e412
            self.group7() - other.group0(),
            // e235, e315, e125
            self.group8() - other.group2().xyz(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn sub(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() - other.group2().with_w(other[e45]),
            // e41, e42, e43
            self.group4() - other.group0(),
            // e23, e31, e12
            self.group5() - other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::SubAssign<Dipole> for MultiVector {
    fn sub_assign(&mut self, other: Dipole) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() - other.group2().with_w(other[e45]),
            // e41, e42, e43
            self.group4() - other.group0(),
            // e23, e31, e12
            self.group5() - other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       15        0        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() - other.group2().xyz().with_w(other[e45]),
            // e41, e42, e43
            self.group4() - other.group0(),
            // e23, e31, e12
            self.group5() - other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9() - other.group3(),
            // e1234
            self[e1234] - other[e1234],
        )
    }
}
impl std::ops::SubAssign<DipoleInversion> for MultiVector {
    fn sub_assign(&mut self, other: DipoleInversion) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() - other.group2().xyz().with_w(other[e45]),
            // e41, e42, e43
            self.group4() - other.group0(),
            // e23, e31, e12
            self.group5() - other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9() - other.group3(),
            // e1234
            self[e1234] - other[e1234],
        );
    }
}
impl std::ops::Sub<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd2        1        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        3        2        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0() + (Simd32x2::from([0.0, other[e12345]]) * Simd32x2::from([0.0, -1.0])),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5] - other[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::SubAssign<DualNum> for MultiVector {
    fn sub_assign(&mut self, other: DualNum) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0() + (Simd32x2::from([0.0, other[e12345]]) * Simd32x2::from([0.0, -1.0])),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5] - other[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() - other.group0(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::SubAssign<FlatPoint> for MultiVector {
    fn sub_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() - other.group0(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() - other.group0(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9() - other.group1(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::SubAssign<Flector> for MultiVector {
    fn sub_assign(&mut self, other: Flector) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() - other.group0(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9() - other.group1(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        7        3        0
    fn sub(self, other: Line) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group1(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::SubAssign<Line> for MultiVector {
    fn sub_assign(&mut self, other: Line) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group1(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd2        1        1        0
    //    simd3        1        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        2        0
    //  no simd       10        5        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0() + (Simd32x2::from([0.0, other[e12345]]) * Simd32x2::from([0.0, -1.0])),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5] - other[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group1().xyz(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::SubAssign<Motor> for MultiVector {
    fn sub_assign(&mut self, other: Motor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0() + (Simd32x2::from([0.0, other[e12345]]) * Simd32x2::from([0.0, -1.0])),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5] - other[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() + (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8() - other.group1().xyz(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd2        1        0        0
    //    simd3        4        0        0
    //    simd4        4        0        0
    // Totals...
    // yes simd       11        0        0
    //  no simd       32        0        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0() - other.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group1(),
            // e5
            self[e5] - other[e5],
            // e15, e25, e35, e45
            self.group3() - other.group3(),
            // e41, e42, e43
            self.group4() - other.group4(),
            // e23, e31, e12
            self.group5() - other.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group6(),
            // e423, e431, e412
            self.group7() - other.group7(),
            // e235, e315, e125
            self.group8() - other.group8(),
            // e4235, e4315, e4125, e3215
            self.group9() - other.group9(),
            // e1234
            self[e1234] - other[e1234],
        )
    }
}
impl std::ops::SubAssign<MultiVector> for MultiVector {
    fn sub_assign(&mut self, other: MultiVector) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0() - other.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group1(),
            // e5
            self[e5] - other[e5],
            // e15, e25, e35, e45
            self.group3() - other.group3(),
            // e41, e42, e43
            self.group4() - other.group4(),
            // e23, e31, e12
            self.group5() - other.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group6(),
            // e423, e431, e412
            self.group7() - other.group7(),
            // e235, e315, e125
            self.group8() - other.group8(),
            // e4235, e4315, e4125, e3215
            self.group9() - other.group9(),
            // e1234
            self[e1234] - other[e1234],
        );
    }
}
impl std::ops::Sub<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9() - other.group0(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::SubAssign<Plane> for MultiVector {
    fn sub_assign(&mut self, other: Plane) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9() - other.group0(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group0(),
            // e5
            self[e5] - other[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::SubAssign<RoundPoint> for MultiVector {
    fn sub_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group0(),
            // e5
            self[e5] - other[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        1        1        0
    // no simd        2        2        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0() + (Simd32x2::from([other[scalar], 0.0]) * Simd32x2::from([-1.0, 0.0])),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::SubAssign<Scalar> for MultiVector {
    fn sub_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0() + (Simd32x2::from([other[scalar], 0.0]) * Simd32x2::from([-1.0, 0.0])),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9() - other.group0(),
            // e1234
            self[e1234] - other[e1234],
        )
    }
}
impl std::ops::SubAssign<Sphere> for MultiVector {
    fn sub_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9() - other.group0(),
            // e1234
            self[e1234] - other[e1234],
        );
    }
}
impl std::ops::Sub<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd2        1        1        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        1        0
    //  no simd       17        2        0
    fn sub(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0() + (Simd32x2::from([0.0, other[e12345]]) * Simd32x2::from([0.0, -1.0])),
            // e1, e2, e3, e4
            self.group1() - other.group3(),
            // e5
            self[e5] - other[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group1(),
            // e423, e431, e412
            self.group7() - other.group0().xyz(),
            // e235, e315, e125
            self.group8() - other.group2().xyz(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::SubAssign<VersorEven> for MultiVector {
    fn sub_assign(&mut self, other: VersorEven) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0() + (Simd32x2::from([0.0, other[e12345]]) * Simd32x2::from([0.0, -1.0])),
            // e1, e2, e3, e4
            self.group1() - other.group3(),
            // e5
            self[e5] - other[e5],
            // e15, e25, e35, e45
            self.group3(),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6() - other.group1(),
            // e423, e431, e412
            self.group7() - other.group0().xyz(),
            // e235, e315, e125
            self.group8() - other.group2().xyz(),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
    }
}
impl std::ops::Sub<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd2        1        1        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        1        0
    //  no simd       17        2        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0() + (Simd32x2::from([other[scalar], 0.0]) * Simd32x2::from([-1.0, 0.0])),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() - other.group2().xyz().with_w(other[e45]),
            // e41, e42, e43
            self.group4() - other.group0().xyz(),
            // e23, e31, e12
            self.group5() - other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9() - other.group3(),
            // e1234
            self[e1234] - other[e1234],
        )
    }
}
impl std::ops::SubAssign<VersorOdd> for MultiVector {
    fn sub_assign(&mut self, other: VersorOdd) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0() + (Simd32x2::from([other[scalar], 0.0]) * Simd32x2::from([-1.0, 0.0])),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() - other.group2().xyz().with_w(other[e45]),
            // e41, e42, e43
            self.group4() - other.group0().xyz(),
            // e23, e31, e12
            self.group5() - other.group1().xyz(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e4235, e4315, e4125, e3215
            self.group9() - other.group3(),
            // e1234
            self[e1234] - other[e1234],
        );
    }
}
