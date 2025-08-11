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
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         4       1       0     N/A
//  Average:        27      32       0     N/A
//  Maximum:       478     502       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         9       1       0       0
//  Average:        56      63       0       0
//  Maximum:       946     976       0       0
impl std::ops::Add<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd2        1        0        0      N/A
    //    simd3        2        0        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        4        0        0      N/A
    //  no simd       12        0        0        0
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
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd3        2        0        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        5        0        0      N/A
    //  no simd       15        0        0        0
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
    //      add/sub      mul      div      pow
    // f32        2        0        0        0
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] + self[scalar], self[e12345]]),
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
            Simd32x2::from([other[scalar] + self[scalar], self[e12345]]),
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
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd3        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        4        0        0        0
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
    //           add/sub      mul      div      pow
    //      f32        2        0        0        0
    //    simd3        2        0        0      N/A
    // Totals...
    // yes simd        4        0        0      N/A
    //  no simd        8        0        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (other.group1().xyz() + self.group1().xyz()).with_w(self[e4]),
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
            (other.group1().xyz() + self.group1().xyz()).with_w(self[e4]),
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
    //           add/sub      mul      div      pow
    //    simd3        1        0        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        7        0        0        0
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
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd2        1        0        0      N/A
    //    simd3        2        0        0      N/A
    // Totals...
    // yes simd        4        0        0      N/A
    //  no simd        9        0        0        0
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
            (other.group1().xyz() + self.group3().xyz()).with_w(self[e45]),
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
            (other.group1().xyz() + self.group3().xyz()).with_w(self[e45]),
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
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd3        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        4        0        0        0
    fn add(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (other.group0().xyz() + self.group1().xyz()).with_w(self[e4]),
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
            (other.group0().xyz() + self.group1().xyz()).with_w(self[e4]),
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
    //          add/sub      mul      div      pow
    //   simd2        1        0        0      N/A
    // no simd        2        0        0        0
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
    //           add/sub      mul      div      pow
    //    simd3        2        0        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd       10        0        0        0
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
    //           add/sub      mul      div      pow
    //    simd2        1        0        0      N/A
    //    simd3        2        0        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        4        0        0      N/A
    //  no simd       12        0        0        0
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
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd3        3        0        0      N/A
    // Totals...
    // yes simd        4        0        0      N/A
    //  no simd       10        0        0        0
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
            (other.group2() + self.group3().xyz()).with_w(other[e45] + self[e45]),
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
            (other.group2() + self.group3().xyz()).with_w(other[e45] + self[e45]),
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
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd3        2        0        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        5        0        0      N/A
    //  no simd       15        0        0        0
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
    //      add/sub      mul      div      pow
    // f32        2        0        0        0
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345] + self[e12345]]),
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
            Simd32x2::from([self[scalar], other[e12345] + self[e12345]]),
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
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        2        0        0      N/A
    // no simd        8        0        0        0
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
    //           add/sub      mul      div      pow
    //    simd3        1        0        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        7        0        0        0
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
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd2        1        0        0      N/A
    //    simd3        2        0        0      N/A
    // Totals...
    // yes simd        4        0        0      N/A
    //  no simd        9        0        0        0
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
            (other.group0().xyz() + self.group6().xyz()).with_w(self[e321]),
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
            (other.group0().xyz() + self.group6().xyz()).with_w(self[e321]),
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
    //           add/sub      mul      div      pow
    //      f32        2        0        0        0
    //    simd2        1        0        0      N/A
    //    simd3        4        0        0      N/A
    //    simd4        4        0        0      N/A
    // Totals...
    // yes simd       11        0        0      N/A
    //  no simd       32        0        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
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
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        5        0        0        0
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
    //          add/sub      mul      div      pow
    //   simd2        1        0        0      N/A
    // no simd        2        0        0        0
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
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        5        0        0        0
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
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd2        1        0        0      N/A
    //    simd3        2        0        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        6        0        0      N/A
    //  no simd       17        0        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group3(),
            // e5
            other[e5] + self[e5],
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
            other[e5] + self[e5],
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
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd2        1        0        0      N/A
    //    simd3        2        0        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        6        0        0      N/A
    //  no simd       17        0        0        0
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
            other[e1234] + self[e1234],
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
            other[e1234] + self[e1234],
        );
    }
}
impl std::ops::BitXor<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       19       28        0        0
    //    simd3       16       21        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd       36       52        0      N/A
    //  no simd       71      103        0        0
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
    //           add/sub      mul      div      pow
    //      f32       30       41        0        0
    //    simd2        1        2        0      N/A
    //    simd3        9       12        0      N/A
    //    simd4        7        9        0      N/A
    // Totals...
    // yes simd       47       64        0      N/A
    //  no simd       87      117        0        0
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
    //           add/sub      mul      div      pow
    //      f32        2        7        0        0
    //    simd3        0        5        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        2       15        0      N/A
    //  no simd        2       34        0        0
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
    //           add/sub      mul      div      pow
    //      f32        6       10        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        6       12        0      N/A
    //  no simd        6       16        0        0
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
    //           add/sub      mul      div      pow
    //      f32       22       36        0        0
    //    simd2        0        1        0      N/A
    //    simd3        5       10        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd       30       51        0      N/A
    //  no simd       49       84        0        0
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
    //           add/sub      mul      div      pow
    //      f32        9       12        0        0
    //    simd3        4       10        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       13       23        0      N/A
    //  no simd       21       46        0        0
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
    //           add/sub      mul      div      pow
    //      f32       15       24        0        0
    //    simd3        6       12        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd       24       40        0      N/A
    //  no simd       45       76        0        0
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
    //           add/sub      mul      div      pow
    //      f32       11       23        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4        9        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd       18       36        0      N/A
    //  no simd       35       64        0        0
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
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn bitxor(self, other: AntiScalar) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       16        0        0
    //    simd2        1        2        0      N/A
    //    simd3        2        4        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       17       23        0      N/A
    //  no simd       25       36        0        0
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
    //           add/sub      mul      div      pow
    //      f32       14       17        0        0
    //    simd2        1        2        0      N/A
    //    simd3        2        4        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       18       24        0      N/A
    //  no simd       26       37        0        0
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
    //           add/sub      mul      div      pow
    //      f32       14       16        0        0
    //    simd3       11       17        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       25       34        0      N/A
    //  no simd       47       71        0        0
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
    //           add/sub      mul      div      pow
    //      f32       23       28        0        0
    //    simd3       10       15        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd       34       45        0      N/A
    //  no simd       57       81        0        0
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
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        1        8        0      N/A
    //  no simd        1       21        0        0
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
    //           add/sub      mul      div      pow
    //      f32        6       10        0        0
    //    simd3        3        5        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        9       16        0      N/A
    //  no simd       15       29        0        0
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
impl std::ops::BitXor<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        2        5        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        7       12        0      N/A
    //  no simd       11       25        0        0
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
    //      f32        8       11        0        0
    //    simd2        1        2        0      N/A
    //    simd3        3        5        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       14       20        0      N/A
    //  no simd       27       38        0        0
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
    //      f32       65       77        0        0
    //    simd2        3        4        0      N/A
    //    simd3       30       33        0      N/A
    //    simd4        9       11        0      N/A
    // Totals...
    // yes simd      107      125        0      N/A
    //  no simd      197      228        0        0
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
impl std::ops::BitXor<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       15       26        0        0
    //    simd2        1        2        0      N/A
    //    simd3        8       11        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd       25       42        0      N/A
    //  no simd       45       75        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        4        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0       11        0      N/A
    //  no simd        0       32        0        0
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
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd        4       10        0        0
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
    //           add/sub      mul      div      pow
    //      f32       33       44        0        0
    //    simd2        1        2        0      N/A
    //    simd3       11       14        0      N/A
    //    simd4        5        7        0      N/A
    // Totals...
    // yes simd       50       67        0      N/A
    //  no simd       88      118        0        0
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
    //           add/sub      mul      div      pow
    //      f32       29       42        0        0
    //    simd2        1        2        0      N/A
    //    simd3       14       17        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd       46       65        0      N/A
    //  no simd       81      113        0        0
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
            from_anti_circle_rotor.group2().xyz().with_w(from_anti_circle_rotor[e45]),
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
            from_anti_dipole_inversion.group3().xyz().with_w(from_anti_dipole_inversion[e4]),
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
            from_dipole.group2().with_w(from_dipole[e45]),
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
            from_dipole_inversion.group2().xyz().with_w(from_dipole_inversion[e45]),
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
            from_versor_odd.group2().xyz().with_w(from_versor_odd[e45]),
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
    //           add/sub      mul      div      pow
    //      f32       37       52        0        0
    //    simd2       17       22        0      N/A
    //    simd3       66       68        0      N/A
    //    simd4        5        5        0      N/A
    // Totals...
    // yes simd      125      147        0      N/A
    //  no simd      289      320        0        0
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
    //           add/sub      mul      div      pow
    //      f32      119      137        0        0
    //    simd2        9       15        0      N/A
    //    simd3       81       81        0      N/A
    //    simd4       15       15        0      N/A
    // Totals...
    // yes simd      224      248        0      N/A
    //  no simd      440      470        0        0
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
    //           add/sub      mul      div      pow
    //      f32        2        7        0        0
    //    simd2        1        2        0      N/A
    //    simd3        7       14        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd       11       25        0      N/A
    //  no simd       29       61        0        0
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
    //           add/sub      mul      div      pow
    //      f32       17       29        0        0
    //    simd2        2        4        0      N/A
    //    simd3       17       21        0      N/A
    //    simd4        7        6        0      N/A
    // Totals...
    // yes simd       43       60        0      N/A
    //  no simd      100      124        0        0
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
    //           add/sub      mul      div      pow
    //      f32       51       69        0        0
    //    simd2        6       11        0      N/A
    //    simd3       43       44        0      N/A
    //    simd4        5        5        0      N/A
    // Totals...
    // yes simd      105      129        0      N/A
    //  no simd      212      243        0        0
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
    //           add/sub      mul      div      pow
    //      f32       12       15        0        0
    //    simd2        5        6        0      N/A
    //    simd3       40       48        0      N/A
    // Totals...
    // yes simd       57       69        0      N/A
    //  no simd      142      171        0        0
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
    //           add/sub      mul      div      pow
    //      f32       26       37        0        0
    //    simd2       14       20        0      N/A
    //    simd3       39       41        0      N/A
    //    simd4       12       10        0      N/A
    // Totals...
    // yes simd       91      108        0      N/A
    //  no simd      219      240        0        0
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
    //           add/sub      mul      div      pow
    //      f32       16       28        0        0
    //    simd2        5        8        0      N/A
    //    simd3       12       14        0      N/A
    //    simd4        9        9        0      N/A
    // Totals...
    // yes simd       42       59        0      N/A
    //  no simd       98      122        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd2        0        2        0      N/A
    //    simd3        0        6        0      N/A
    //    simd4        0        6        0      N/A
    // Totals...
    // yes simd        0       19        0      N/A
    //  no simd        0       51        0        0
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
    //           add/sub      mul      div      pow
    //      f32       63       78        0        0
    //    simd2        3        5        0      N/A
    //    simd3       59       63        0      N/A
    //    simd4        9        9        0      N/A
    // Totals...
    // yes simd      134      155        0      N/A
    //  no simd      282      313        0        0
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
    //           add/sub      mul      div      pow
    //      f32       79       96        0        0
    //    simd2        5        9        0      N/A
    //    simd3       63       65        0      N/A
    //    simd4        9        9        0      N/A
    // Totals...
    // yes simd      156      179        0      N/A
    //  no simd      314      345        0        0
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
    //           add/sub      mul      div      pow
    //      f32       33       44        0        0
    //    simd2       14       18        0      N/A
    //    simd3       64       68        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd      113      131        0      N/A
    //  no simd      261      288        0        0
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
    //           add/sub      mul      div      pow
    //      f32       86      104        0        0
    //    simd2       22       28        0      N/A
    //    simd3       90       90        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd      201      225        0      N/A
    //  no simd      412      442        0        0
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
    //           add/sub      mul      div      pow
    //      f32        4       11        0        0
    //    simd3        5       13        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd       12       27        0      N/A
    //  no simd       31       62        0        0
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
    //           add/sub      mul      div      pow
    //      f32        7       18        0        0
    //    simd2        3        5        0      N/A
    //    simd3       19       24        0      N/A
    //    simd4        6        5        0      N/A
    // Totals...
    // yes simd       35       52        0      N/A
    //  no simd       94      120        0        0
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
    //           add/sub      mul      div      pow
    //      f32       48       65        0        0
    //    simd2        8       13        0      N/A
    //    simd3       42       43        0      N/A
    //    simd4        7        7        0      N/A
    // Totals...
    // yes simd      105      128        0      N/A
    //  no simd      218      248        0        0
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
    //           add/sub      mul      div      pow
    //      f32       27       33        0        0
    //    simd3       39       47        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       67       81        0      N/A
    //  no simd      148      178        0        0
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
    //      f32       49       65        0        0
    //    simd2        6       10        0      N/A
    //    simd3       41       43        0      N/A
    //    simd4        8        7        0      N/A
    // Totals...
    // yes simd      104      125        0      N/A
    //  no simd      216      242        0        0
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
    //      f32      233      251        0        0
    //    simd2       27       33        0      N/A
    //    simd3      213      213        0      N/A
    //    simd4        5        5        0      N/A
    // Totals...
    // yes simd      478      502        0      N/A
    //  no simd      946      976        0        0
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
    //           add/sub      mul      div      pow
    //      f32       17       30        0        0
    //    simd2        2        5        0      N/A
    //    simd3       19       23        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd       42       60        0      N/A
    //  no simd       94      117        0        0
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
    //           add/sub      mul      div      pow
    //      f32       20       36        0        0
    //    simd2        5        7        0      N/A
    //    simd3       23       27        0      N/A
    //    simd4        5        5        0      N/A
    // Totals...
    // yes simd       53       75        0      N/A
    //  no simd      119      151        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        4        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0       11        0      N/A
    //  no simd        0       32        0        0
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
    //           add/sub      mul      div      pow
    //      f32       24       40        0        0
    //    simd2        1        4        0      N/A
    //    simd3       25       29        0      N/A
    //    simd4        5        5        0      N/A
    // Totals...
    // yes simd       55       78        0      N/A
    //  no simd      121      155        0        0
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
    //           add/sub      mul      div      pow
    //      f32      148      172        0        0
    //    simd2       15       23        0      N/A
    //    simd3       72       68        0      N/A
    //    simd4       21       21        0      N/A
    // Totals...
    // yes simd      256      284        0      N/A
    //  no simd      478      506        0        0
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
    //           add/sub      mul      div      pow
    //      f32      123      147        0        0
    //    simd2       19       27        0      N/A
    //    simd3       87       83        0      N/A
    //    simd4        6        6        0      N/A
    // Totals...
    // yes simd      235      263        0      N/A
    //  no simd      446      474        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        4        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0       11        0      N/A
    //  no simd        0       32        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        4        0      N/A
    // Totals...
    // yes simd        0        8        0      N/A
    //  no simd        0       17        0        0
    fn not(self) -> Self::Output {
        self.right_dual()
    }
}
impl std::ops::Sub<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd2        1        0        0      N/A
    //    simd3        2        0        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       12        5        0        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() + -other.group2().xyz().with_w(other[e45] * -1.0),
            // e41, e42, e43
            self.group4() - other.group0(),
            // e23, e31, e12
            Simd32x3::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0]) + self.group5(),
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
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() + -other.group2().xyz().with_w(other[e45] * -1.0),
            // e41, e42, e43
            self.group4() - other.group0(),
            // e23, e31, e12
            Simd32x3::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0]) + self.group5(),
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
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd3        2        0        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        5        4        0      N/A
    //  no simd       15        4        0        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + -other.group3().xyz().with_w(other[e4] * -1.0),
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
            Simd32x3::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0]) + self.group8(),
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
            self.group1() + -other.group3().xyz().with_w(other[e4] * -1.0),
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
            Simd32x3::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0]) + self.group8(),
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
    //           add/sub      mul      div      pow
    //      f32        1        1        0        0
    //    simd2        1        0        0      N/A
    // Totals...
    // yes simd        2        1        0      N/A
    //  no simd        3        1        0        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
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
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
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
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        1        0        0      N/A
    // Totals...
    // yes simd        2        3        0      N/A
    //  no simd        4        3        0        0
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
            Simd32x3::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0]) + self.group8(),
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
            Simd32x3::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0]) + self.group8(),
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
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        1        0        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        4        3        0      N/A
    //  no simd        9        3        0        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + -other.group1().xyz().with_w(0.0),
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
            Simd32x3::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0]) + self.group8(),
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
            self.group1() + -other.group1().xyz().with_w(0.0),
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
            Simd32x3::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0]) + self.group8(),
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
    //           add/sub      mul      div      pow
    //    simd3        1        0        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        7        0        0        0
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
            self.group3() + -other.group1().with_w(0.0),
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
            self.group3() + -other.group1().with_w(0.0),
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
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd2        1        0        0      N/A
    //    simd3        1        0        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        4        4        0      N/A
    //  no simd       10        4        0        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() + -other.group1().xyz().with_w(0.0),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0]) + self.group5(),
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
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() + -other.group1().xyz().with_w(0.0),
            // e41, e42, e43
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0]) + self.group5(),
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
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        5        0        0        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + -other.group0().xyz().with_w(0.0),
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
            self.group1() + -other.group0().xyz().with_w(0.0),
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
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        1        0        0      N/A
    // Totals...
    // yes simd        1        1        0      N/A
    //  no simd        2        1        0        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
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
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
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
    //           add/sub      mul      div      pow
    //    simd3        2        0        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd       10        0        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd2        1        0        0      N/A
    //    simd3        2        0        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        4        4        0      N/A
    //  no simd       12        4        0        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
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
            Simd32x3::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0]) + self.group8(),
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
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
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
            Simd32x3::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0]) + self.group8(),
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
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        2        0        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        3        4        0      N/A
    //  no simd       10        4        0        0
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
            self.group3() + -other.group2().with_w(other[e45] * -1.0),
            // e41, e42, e43
            self.group4() - other.group0(),
            // e23, e31, e12
            Simd32x3::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0]) + self.group5(),
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
            self.group3() + -other.group2().with_w(other[e45] * -1.0),
            // e41, e42, e43
            self.group4() - other.group0(),
            // e23, e31, e12
            Simd32x3::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0]) + self.group5(),
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
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd3        2        0        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        5        4        0      N/A
    //  no simd       15        4        0        0
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
            self.group3() + -other.group2().xyz().with_w(other[e45] * -1.0),
            // e41, e42, e43
            self.group4() - other.group0(),
            // e23, e31, e12
            Simd32x3::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0]) + self.group5(),
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
            self.group3() + -other.group2().xyz().with_w(other[e45] * -1.0),
            // e41, e42, e43
            self.group4() - other.group0(),
            // e23, e31, e12
            Simd32x3::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0]) + self.group5(),
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
    //           add/sub      mul      div      pow
    //      f32        1        1        0        0
    //    simd2        1        0        0      N/A
    // Totals...
    // yes simd        2        1        0      N/A
    //  no simd        3        1        0        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
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
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
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
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        2        0        0      N/A
    // no simd        8        0        0        0
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
    //           add/sub      mul      div      pow
    //    simd3        1        0        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        7        0        0        0
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
            self.group6() + -other.group0().with_w(0.0),
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
            self.group6() + -other.group0().with_w(0.0),
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
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd2        1        0        0      N/A
    //    simd3        1        0        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        4        4        0      N/A
    //  no simd       10        4        0        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
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
            self.group6() + -other.group0().xyz().with_w(0.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0]) + self.group8(),
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
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
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
            self.group6() + -other.group0().xyz().with_w(0.0),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0]) + self.group8(),
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
    //           add/sub      mul      div      pow
    //      f32        2        0        0        0
    //    simd2        1        0        0      N/A
    //    simd3        4        0        0      N/A
    //    simd4        4        0        0      N/A
    // Totals...
    // yes simd       11        0        0      N/A
    //  no simd       32        0        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
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
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        5        0        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        1        0        0      N/A
    // Totals...
    // yes simd        1        1        0      N/A
    //  no simd        2        1        0        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
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
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
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
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        5        0        0        0
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
    //           add/sub      mul      div      pow
    //      f32        1        7        0        0
    //    simd2        1        0        0      N/A
    //    simd3        2        0        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        6        7        0      N/A
    //  no simd       17        7        0        0
    fn sub(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
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
            Simd32x3::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0]) + self.group7(),
            // e235, e315, e125
            Simd32x3::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0]) + self.group8(),
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
            Simd32x2::from([0.0, other[e12345] * -1.0]) + self.group0(),
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
            Simd32x3::from([other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0]) + self.group7(),
            // e235, e315, e125
            Simd32x3::from([other[e235] * -1.0, other[e315] * -1.0, other[e125] * -1.0]) + self.group8(),
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
    //           add/sub      mul      div      pow
    //      f32        1        8        0        0
    //    simd2        1        0        0      N/A
    //    simd3        2        0        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        6        8        0      N/A
    //  no simd       17        8        0        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() + -other.group2().xyz().with_w(other[e45] * -1.0),
            // e41, e42, e43
            Simd32x3::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0]) + self.group4(),
            // e23, e31, e12
            Simd32x3::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0]) + self.group5(),
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
            Simd32x2::from([other[scalar] * -1.0, 0.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() + -other.group2().xyz().with_w(other[e45] * -1.0),
            // e41, e42, e43
            Simd32x3::from([other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0]) + self.group4(),
            // e23, e31, e12
            Simd32x3::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0]) + self.group5(),
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
