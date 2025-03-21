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
// Total Implementations: 114
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         2       2       0     N/A
//  Average:        12      15       0     N/A
//  Maximum:       193     211       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         5       6       0       0
//  Average:        38      38       0       0
//  Maximum:       525     512       0       0
impl std::ops::Add<AntiCircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        3        0        0      N/A
    // no simd       12        0        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() + other.group0().with_w(other[scalar]),
            // e23, e31, e12, e45
            other.group1() + self.group1(),
            // e15, e25, e35, e1234
            self.group2() + other.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        )
    }
}
impl std::ops::AddAssign<AntiCircleRotor> for VersorOdd {
    fn add_assign(&mut self, other: AntiCircleRotor) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() + other.group0().with_w(other[scalar]),
            // e23, e31, e12, e45
            other.group1() + self.group1(),
            // e15, e25, e35, e1234
            self.group2() + other.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]),
            // e5
            other[e5],
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2().xyz(),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::Add<AntiDualNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        0        0        0
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(other[scalar] + self[scalar]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(other[e3215] + self[e3215]),
        )
    }
}
impl std::ops::AddAssign<AntiDualNum> for VersorOdd {
    fn add_assign(&mut self, other: AntiDualNum) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(other[scalar] + self[scalar]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(other[e3215] + self[e3215]),
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz(),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::Add<AntiFlector> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            other.group1().xyz().with_w(0.0),
            // e5
            other[e5],
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz(),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::Add<AntiLine> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        2        0        0      N/A
    // no simd        8        0        0        0
    fn add(self, other: AntiLine) -> Self::Output {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + other.group0().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2() + other.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        )
    }
}
impl std::ops::AddAssign<AntiLine> for VersorOdd {
    fn add_assign(&mut self, other: AntiLine) {
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1() + other.group0().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2() + other.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<AntiMotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        0        0        0
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        4        0        0      N/A
    //  no simd       10        0        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(other[scalar] + self[scalar]),
            // e23, e31, e12, e45
            self.group1() + other.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2() + other.group1().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(other[e3215] + self[e3215]),
        )
    }
}
impl std::ops::AddAssign<AntiMotor> for VersorOdd {
    fn add_assign(&mut self, other: AntiMotor) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(other[scalar] + self[scalar]),
            // e23, e31, e12, e45
            self.group1() + other.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2() + other.group1().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(other[e3215] + self[e3215]),
        );
    }
}
impl std::ops::Add<AntiPlane> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            other.group0().xyz().with_w(0.0),
            // e5
            other[e5],
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::Add<AntiScalar> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::Add<Circle> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::Add<CircleRotor> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2().xyz(),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::Add<Dipole> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        3        0        0      N/A
    // no simd       12        0        0        0
    fn add(self, other: Dipole) -> Self::Output {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() + other.group0().with_w(0.0),
            // e23, e31, e12, e45
            other.group1() + self.group1(),
            // e15, e25, e35, e1234
            self.group2() + other.group2().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        )
    }
}
impl std::ops::AddAssign<Dipole> for VersorOdd {
    fn add_assign(&mut self, other: Dipole) {
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() + other.group0().with_w(0.0),
            // e23, e31, e12, e45
            other.group1() + self.group1(),
            // e15, e25, e35, e1234
            self.group2() + other.group2().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<DipoleInversion> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        4        0        0      N/A
    // no simd       16        0        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() + other.group0().with_w(0.0),
            // e23, e31, e12, e45
            other.group1() + self.group1(),
            // e15, e25, e35, e1234
            other.group2() + self.group2(),
            // e4235, e4315, e4125, e3215
            other.group3() + self.group3(),
        )
    }
}
impl std::ops::AddAssign<DipoleInversion> for VersorOdd {
    fn add_assign(&mut self, other: DipoleInversion) {
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() + other.group0().with_w(0.0),
            // e23, e31, e12, e45
            other.group1() + self.group1(),
            // e15, e25, e35, e1234
            other.group2() + self.group2(),
            // e4235, e4315, e4125, e3215
            other.group3() + self.group3(),
        );
    }
}
impl std::ops::Add<DualNum> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::Add<FlatPoint> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        5        0        0        0
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35, e1234
            self.group2() + other.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        )
    }
}
impl std::ops::AddAssign<FlatPoint> for VersorOdd {
    fn add_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35, e1234
            self.group2() + other.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<Flector> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd        9        0        0        0
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35, e1234
            self.group2() + other.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            other.group1() + self.group3(),
        )
    }
}
impl std::ops::AddAssign<Flector> for VersorOdd {
    fn add_assign(&mut self, other: Flector) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(other[e45] + self[e45]),
            // e15, e25, e35, e1234
            self.group2() + other.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            other.group1() + self.group3(),
        );
    }
}
impl std::ops::Add<Line> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group0().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::Add<Motor> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().xyz(),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::Add<MultiVector> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd2        1        0        0      N/A
    //    simd3        2        0        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        7        0        0      N/A
    //  no simd       21        0        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]) + other.group0(),
            // e1, e2, e3, e4
            other.group1(),
            // e5
            other[e5],
            // e15, e25, e35, e45
            other.group3() + Simd32x3::from(0.0).with_w(self[e45]) + self.group2().xyz().with_w(0.0),
            // e41, e42, e43
            other.group4() + self.group0().xyz(),
            // e23, e31, e12
            other.group5() + self.group1().xyz(),
            // e415, e425, e435, e321
            other.group6(),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            other.group8(),
            // e4235, e4315, e4125, e3215
            other.group9() + self.group3(),
            // e1234
            other[e1234] + self[e1234],
        )
    }
}
impl std::ops::Add<Plane> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn add(self, other: Plane) -> Self::Output {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            other.group0() + self.group3(),
        )
    }
}
impl std::ops::AddAssign<Plane> for VersorOdd {
    fn add_assign(&mut self, other: Plane) {
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            other.group0() + self.group3(),
        );
    }
}
impl std::ops::Add<RoundPoint> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            other[e5],
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::Add<Scalar> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        0        0        0
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(other[scalar] + self[scalar]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        )
    }
}
impl std::ops::AddAssign<Scalar> for VersorOdd {
    fn add_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(other[scalar] + self[scalar]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Add<Sphere> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        5        0        0        0
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234] + self[e1234]),
            // e4235, e4315, e4125, e3215
            other.group0() + self.group3(),
        )
    }
}
impl std::ops::AddAssign<Sphere> for VersorOdd {
    fn add_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(other[e1234] + self[e1234]),
            // e4235, e4315, e4125, e3215
            other.group0() + self.group3(),
        );
    }
}
impl std::ops::Add<VersorEven> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            other.group3(),
            // e5
            other[e5],
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            other.group2().xyz(),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::Add<VersorOdd> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        4        0        0      N/A
    // no simd       16        0        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() + self.group0(),
            // e23, e31, e12, e45
            other.group1() + self.group1(),
            // e15, e25, e35, e1234
            other.group2() + self.group2(),
            // e4235, e4315, e4125, e3215
            other.group3() + self.group3(),
        )
    }
}
impl std::ops::AddAssign<VersorOdd> for VersorOdd {
    fn add_assign(&mut self, other: VersorOdd) {
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() + self.group0(),
            // e23, e31, e12, e45
            other.group1() + self.group1(),
            // e15, e25, e35, e1234
            other.group2() + self.group2(),
            // e4235, e4315, e4125, e3215
            other.group3() + self.group3(),
        );
    }
}
impl std::ops::BitXor<AntiCircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       11        0        0
    //    simd3        1        7        0      N/A
    //    simd4       10        6        0      N/A
    // Totals...
    // yes simd       19       24        0      N/A
    //  no simd       51       56        0        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<AntiCircleRotor> for VersorOdd {
    fn bitxor_assign(&mut self, other: AntiCircleRotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for VersorOdd {
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
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiDualNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        1        6        0      N/A
    //  no simd        1       17        0        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<AntiDualNum> for VersorOdd {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for VersorOdd {
    type Output = CircleRotor;
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
impl std::ops::BitXor<AntiFlector> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       11        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        3        0      N/A
    //    simd4        4        4        0      N/A
    // Totals...
    // yes simd       15       20        0      N/A
    //  no simd       35       40        0        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiLine> for VersorOdd {
    type Output = DipoleInversion;
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
impl std::ops::BitXor<AntiMotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        1        4        0      N/A
    //    simd4        6        5        0      N/A
    // Totals...
    // yes simd       13       18        0      N/A
    //  no simd       33       41        0        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<AntiMotor> for VersorOdd {
    fn bitxor_assign(&mut self, other: AntiMotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        5        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd       10       16        0      N/A
    //  no simd       28       33        0        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<AntiScalar> for VersorOdd {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn bitxor(self, other: AntiScalar) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Circle> for VersorOdd {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       10        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        9       13        0      N/A
    //  no simd        9       20        0        0
    fn bitxor(self, other: Circle) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<CircleRotor> for VersorOdd {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       11        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       10       14        0      N/A
    //  no simd       10       21        0        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Dipole> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       11        0        0
    //    simd3        0        7        0      N/A
    //    simd4        6        2        0      N/A
    // Totals...
    // yes simd       14       20        0      N/A
    //  no simd       32       40        0        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<DipoleInversion> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       11        0        0
    //    simd3        0        6        0      N/A
    //    simd4        7        4        0      N/A
    // Totals...
    // yes simd       16       21        0      N/A
    //  no simd       37       45        0        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<DualNum> for VersorOdd {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        1        9        0        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<FlatPoint> for VersorOdd {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       13       16        0        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Flector> for VersorOdd {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       17       20        0        0
    fn bitxor(self, other: Flector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Line> for VersorOdd {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       12        0        0
    fn bitxor(self, other: Line) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Motor> for VersorOdd {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       16       21        0        0
    fn bitxor(self, other: Motor) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<MultiVector> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       26       36        0        0
    //    simd2        0        2        0      N/A
    //    simd3       10       16        0      N/A
    //    simd4       13        9        0      N/A
    // Totals...
    // yes simd       49       63        0      N/A
    //  no simd      108      124        0        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Plane> for VersorOdd {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn bitxor(self, other: Plane) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<RoundPoint> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        5        0      N/A
    //    simd4        6        4        0      N/A
    // Totals...
    // yes simd       12       18        0      N/A
    //  no simd       36       41        0        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<Scalar> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<Scalar> for VersorOdd {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Sphere> for VersorOdd {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn bitxor(self, other: Sphere) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<VersorEven> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       17        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        4        0      N/A
    //    simd4        8        7        0      N/A
    // Totals...
    // yes simd       25       30        0      N/A
    //  no simd       57       61        0        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXor<VersorOdd> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       11        0        0
    //    simd3        1        6        0      N/A
    //    simd4       11        8        0      N/A
    // Totals...
    // yes simd       20       25        0      N/A
    //  no simd       55       61        0        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        self.wedge(other)
    }
}
impl std::ops::BitXorAssign<VersorOdd> for VersorOdd {
    fn bitxor_assign(&mut self, other: VersorOdd) {
        *self = self.wedge(other);
    }
}

impl From<AntiCircleRotor> for VersorOdd {
    fn from(from_anti_circle_rotor: AntiCircleRotor) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            from_anti_circle_rotor.group0().with_w(from_anti_circle_rotor[scalar]),
            // e23, e31, e12, e45
            from_anti_circle_rotor.group1(),
            // e15, e25, e35, e1234
            from_anti_circle_rotor.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiDualNum> for VersorOdd {
    fn from(from_anti_dual_num: AntiDualNum) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_anti_dual_num[scalar]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_anti_dual_num[e3215]),
        )
    }
}

impl From<AntiLine> for VersorOdd {
    fn from(from_anti_line: AntiLine) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            from_anti_line.group0().with_w(0.0),
            // e15, e25, e35, e1234
            from_anti_line.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiMotor> for VersorOdd {
    fn from(from_anti_motor: AntiMotor) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_anti_motor[scalar]),
            // e23, e31, e12, e45
            from_anti_motor.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            from_anti_motor.group1().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_anti_motor[e3215]),
        )
    }
}

impl From<Dipole> for VersorOdd {
    fn from(from_dipole: Dipole) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            from_dipole.group0().with_w(0.0),
            // e23, e31, e12, e45
            from_dipole.group1(),
            // e15, e25, e35, e1234
            from_dipole.group2().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<DipoleInversion> for VersorOdd {
    fn from(from_dipole_inversion: DipoleInversion) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            from_dipole_inversion.group0().with_w(0.0),
            // e23, e31, e12, e45
            from_dipole_inversion.group1(),
            // e15, e25, e35, e1234
            from_dipole_inversion.group2(),
            // e4235, e4315, e4125, e3215
            from_dipole_inversion.group3(),
        )
    }
}

impl From<FlatPoint> for VersorOdd {
    fn from(from_flat_point: FlatPoint) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_flat_point[e45]),
            // e15, e25, e35, e1234
            from_flat_point.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<Flector> for VersorOdd {
    fn from(from_flector: Flector) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_flector[e45]),
            // e15, e25, e35, e1234
            from_flector.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            from_flector.group1(),
        )
    }
}

impl From<Plane> for VersorOdd {
    fn from(from_plane: Plane) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            from_plane.group0(),
        )
    }
}

impl From<Scalar> for VersorOdd {
    fn from(from_scalar: Scalar) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_scalar[scalar]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<Sphere> for VersorOdd {
    fn from(from_sphere: Sphere) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_sphere[e1234]),
            // e4235, e4315, e4125, e3215
            from_sphere.group0(),
        )
    }
}
impl std::ops::Mul<AntiCircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       16       21        0        0
    //    simd3        0       21        0      N/A
    //    simd4       44       23        0      N/A
    // Totals...
    // yes simd       60       65        0      N/A
    //  no simd      192      176        0        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiCircleRotor> for VersorOdd {
    fn mul_assign(&mut self, other: AntiCircleRotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       21        0        0
    //    simd3        0       20        0      N/A
    //    simd4       60       40        0      N/A
    // Totals...
    // yes simd       72       81        0      N/A
    //  no simd      252      241        0        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiDualNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        7        0        0
    //    simd3        2        6        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        8       15        0      N/A
    //  no simd       24       33        0        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiDualNum> for VersorOdd {
    fn mul_assign(&mut self, other: AntiDualNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       15        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4        9        0      N/A
    //    simd4       11        6        0      N/A
    // Totals...
    // yes simd       21       31        0      N/A
    //  no simd       62       68        0        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiFlector> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       13        0        0
    //    simd3        0       13        0      N/A
    //    simd4       31       19        0      N/A
    // Totals...
    // yes simd       41       45        0      N/A
    //  no simd      134      128        0        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiLine> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       20        0        0
    //    simd3        0       19        0      N/A
    //    simd4       23        5        0      N/A
    // Totals...
    // yes simd       32       44        0      N/A
    //  no simd      101       97        0        0
    fn mul(self, other: AntiLine) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiLine> for VersorOdd {
    fn mul_assign(&mut self, other: AntiLine) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       16        0        0
    //    simd3        0       16        0      N/A
    //    simd4       32       16        0      N/A
    // Totals...
    // yes simd       44       48        0      N/A
    //  no simd      140      128        0        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<AntiMotor> for VersorOdd {
    fn mul_assign(&mut self, other: AntiMotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd3        0        6        0      N/A
    //    simd4       15       10        0      N/A
    // Totals...
    // yes simd       18       23        0      N/A
    //  no simd       63       65        0        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<AntiScalar> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        5        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       23        0        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Circle> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       21        0        0
    //    simd3        0       21        0      N/A
    //    simd4       40       19        0      N/A
    // Totals...
    // yes simd       51       61        0      N/A
    //  no simd      171      160        0        0
    fn mul(self, other: Circle) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<CircleRotor> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       21        0        0
    //    simd3        0       21        0      N/A
    //    simd4       44       23        0      N/A
    // Totals...
    // yes simd       57       65        0      N/A
    //  no simd      189      176        0        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Dipole> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       23        0        0
    //    simd3        0       22        0      N/A
    //    simd4       40       18        0      N/A
    // Totals...
    // yes simd       52       63        0      N/A
    //  no simd      172      161        0        0
    fn mul(self, other: Dipole) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Dipole> for VersorOdd {
    fn mul_assign(&mut self, other: Dipole) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       19       23        0        0
    //    simd3        0       23        0      N/A
    //    simd4       60       37        0      N/A
    // Totals...
    // yes simd       79       83        0      N/A
    //  no simd      259      240        0        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<DipoleInversion> for VersorOdd {
    fn mul_assign(&mut self, other: DipoleInversion) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        5        0        0
    //    simd3        1        4        0      N/A
    //    simd4        4        4        0      N/A
    // Totals...
    // yes simd        6       13        0      N/A
    //  no simd       20       33        0        0
    fn mul(self, other: DualNum) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<FlatPoint> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       14        0        0
    //    simd2        0        3        0      N/A
    //    simd3        7        8        0      N/A
    //    simd4        8        5        0      N/A
    // Totals...
    // yes simd       24       30        0      N/A
    //  no simd       62       64        0        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<FlatPoint> for VersorOdd {
    fn mul_assign(&mut self, other: FlatPoint) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       16        0        0
    //    simd3        0       15        0      N/A
    //    simd4       32       17        0      N/A
    // Totals...
    // yes simd       43       48        0      N/A
    //  no simd      139      129        0        0
    fn mul(self, other: Flector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Flector> for VersorOdd {
    fn mul_assign(&mut self, other: Flector) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       19        0        0
    //    simd3        0       16        0      N/A
    //    simd4       23        8        0      N/A
    // Totals...
    // yes simd       32       43        0      N/A
    //  no simd      101       99        0        0
    fn mul(self, other: Line) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Motor> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       14        0        0
    //    simd3        0       14        0      N/A
    //    simd4       31       18        0      N/A
    // Totals...
    // yes simd       42       46        0      N/A
    //  no simd      135      128        0        0
    fn mul(self, other: Motor) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<MultiVector> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       53       71        0        0
    //    simd2       12       24        0      N/A
    //    simd3       64       71        0      N/A
    //    simd4       64       45        0      N/A
    // Totals...
    // yes simd      193      211        0      N/A
    //  no simd      525      512        0        0
    fn mul(self, other: MultiVector) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Plane> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       10        0        0
    //    simd3        0        9        0      N/A
    //    simd4       16        7        0      N/A
    // Totals...
    // yes simd       21       26        0      N/A
    //  no simd       69       65        0        0
    fn mul(self, other: Plane) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Plane> for VersorOdd {
    fn mul_assign(&mut self, other: Plane) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        0        8        0      N/A
    //    simd4       20       12        0      N/A
    // Totals...
    // yes simd       24       28        0      N/A
    //  no simd       84       80        0        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<Scalar> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn mul(self, other: Scalar) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Scalar> for VersorOdd {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       10        0        0
    //    simd3        0        9        0      N/A
    //    simd4       20       11        0      N/A
    // Totals...
    // yes simd       24       30        0      N/A
    //  no simd       84       81        0        0
    fn mul(self, other: Sphere) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<Sphere> for VersorOdd {
    fn mul_assign(&mut self, other: Sphere) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       15       19        0        0
    //    simd3        0       19        0      N/A
    //    simd4       64       45        0      N/A
    // Totals...
    // yes simd       79       83        0      N/A
    //  no simd      271      256        0        0
    fn mul(self, other: VersorEven) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::Mul<VersorOdd> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       19       23        0        0
    //    simd3        0       23        0      N/A
    //    simd4       64       41        0      N/A
    // Totals...
    // yes simd       83       87        0      N/A
    //  no simd      275      256        0        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        self.geometric_product(other)
    }
}
impl std::ops::MulAssign<VersorOdd> for VersorOdd {
    fn mul_assign(&mut self, other: VersorOdd) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Neg for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn neg(self) -> Self::Output {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group3() * Simd32x4::from(-1.0),
        )
    }
}
impl std::ops::Not for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       14        0        0
    fn not(self) -> Self::Output {
        self.right_dual()
    }
}
impl std::ops::Sub<AntiCircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        4        0        0      N/A
    // Totals...
    // yes simd        4        1        0      N/A
    //  no simd       16        1        0        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0) - other.group0().with_w(0.0),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            self.group2() - other.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        )
    }
}
impl std::ops::SubAssign<AntiCircleRotor> for VersorOdd {
    fn sub_assign(&mut self, other: AntiCircleRotor) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() + Simd32x3::from(0.0).with_w(other[scalar] * -1.0) - other.group0().with_w(0.0),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            self.group2() - other.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        3        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       15        0        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e4] * -1.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::Sub<AntiDualNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        0        0        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(self[scalar] - other[scalar]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(self[e3215] - other[e3215]),
        )
    }
}
impl std::ops::SubAssign<AntiDualNum> for VersorOdd {
    fn sub_assign(&mut self, other: AntiDualNum) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(self[scalar] - other[scalar]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(self[e3215] - other[e3215]),
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        4        0        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::Sub<AntiFlector> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::Sub<AntiLine> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        2        0        0      N/A
    // no simd        8        0        0        0
    fn sub(self, other: AntiLine) -> Self::Output {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1() - other.group0().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2() - other.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        )
    }
}
impl std::ops::SubAssign<AntiLine> for VersorOdd {
    fn sub_assign(&mut self, other: AntiLine) {
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1() - other.group0().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2() - other.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiMotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        0        0        0
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        4        0        0      N/A
    //  no simd       10        0        0        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(self[scalar] - other[scalar]),
            // e23, e31, e12, e45
            self.group1() - other.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2() - other.group1().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(self[e3215] - other[e3215]),
        )
    }
}
impl std::ops::SubAssign<AntiMotor> for VersorOdd {
    fn sub_assign(&mut self, other: AntiMotor) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(self[scalar] - other[scalar]),
            // e23, e31, e12, e45
            self.group1() - other.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            self.group2() - other.group1().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(self[e3215] - other[e3215]),
        );
    }
}
impl std::ops::Sub<AntiPlane> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        4        0        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::Sub<AntiScalar> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345] * -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::Sub<Circle> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       10        0        0
    fn sub(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::Sub<CircleRotor> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       11        0        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345] * -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::Sub<Dipole> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        3        0        0      N/A
    // no simd       12        0        0        0
    fn sub(self, other: Dipole) -> Self::Output {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() - other.group0().with_w(0.0),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            self.group2() - other.group2().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        )
    }
}
impl std::ops::SubAssign<Dipole> for VersorOdd {
    fn sub_assign(&mut self, other: Dipole) {
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() - other.group0().with_w(0.0),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            self.group2() - other.group2().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<DipoleInversion> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        4        0        0      N/A
    // no simd       16        0        0        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() - other.group0().with_w(0.0),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            self.group2() - other.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group3(),
        )
    }
}
impl std::ops::SubAssign<DipoleInversion> for VersorOdd {
    fn sub_assign(&mut self, other: DipoleInversion) {
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() - other.group0().with_w(0.0),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            self.group2() - other.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group3(),
        );
    }
}
impl std::ops::Sub<DualNum> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345] * -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::Sub<FlatPoint> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        5        0        0        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e45] - other[e45]),
            // e15, e25, e35, e1234
            self.group2() - other.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        )
    }
}
impl std::ops::SubAssign<FlatPoint> for VersorOdd {
    fn sub_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e45] - other[e45]),
            // e15, e25, e35, e1234
            self.group2() - other.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<Flector> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        3        0        0      N/A
    //  no simd        9        0        0        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e45] - other[e45]),
            // e15, e25, e35, e1234
            self.group2() - other.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group1(),
        )
    }
}
impl std::ops::SubAssign<Flector> for VersorOdd {
    fn sub_assign(&mut self, other: Flector) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1().xyz().with_w(self[e45] - other[e45]),
            // e15, e25, e35, e1234
            self.group2() - other.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group1(),
        );
    }
}
impl std::ops::Sub<Line> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn sub(self, other: Line) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::Sub<Motor> for VersorOdd {
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
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345] * -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::Sub<MultiVector> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        2        0        0
    //    simd3        2        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        6        6        0      N/A
    //  no simd       16       16        0        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], other[e12345] * -1.0]),
            // e1, e2, e3, e4
            other.group1() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            self.group2().xyz().with_w(self[e45]) - other.group3(),
            // e41, e42, e43
            self.group0().xyz() - other.group4(),
            // e23, e31, e12
            self.group1().xyz() - other.group5(),
            // e415, e425, e435, e321
            other.group6() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group8() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group9(),
            // e1234
            self[e1234] - other[e1234],
        )
    }
}
impl std::ops::Sub<Plane> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        0        0      N/A
    // no simd        4        0        0        0
    fn sub(self, other: Plane) -> Self::Output {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group0(),
        )
    }
}
impl std::ops::SubAssign<Plane> for VersorOdd {
    fn sub_assign(&mut self, other: Plane) {
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group0(),
        );
    }
}
impl std::ops::Sub<RoundPoint> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            other.group0() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::Sub<Scalar> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        0        0        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(self[scalar] - other[scalar]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        )
    }
}
impl std::ops::SubAssign<Scalar> for VersorOdd {
    fn sub_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().xyz().with_w(self[scalar] - other[scalar]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl std::ops::Sub<Sphere> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        0        0        0
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        2        0        0      N/A
    //  no simd        5        0        0        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(self[e1234] - other[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group0(),
        )
    }
}
impl std::ops::SubAssign<Sphere> for VersorOdd {
    fn sub_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(self[e1234] - other[e1234]),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group0(),
        );
    }
}
impl std::ops::Sub<VersorEven> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       16        0        0
    fn sub(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345] * -1.0]),
            // e1, e2, e3, e4
            other.group3() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]),
            // e41, e42, e43
            self.group0().xyz(),
            // e23, e31, e12
            self.group1().xyz(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group3(),
            // e1234
            self[e1234],
        )
    }
}
impl std::ops::Sub<VersorOdd> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        4        0        0      N/A
    // no simd       16        0        0        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() - other.group0(),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            self.group2() - other.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group3(),
        )
    }
}
impl std::ops::SubAssign<VersorOdd> for VersorOdd {
    fn sub_assign(&mut self, other: VersorOdd) {
        *self = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() - other.group0(),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            self.group2() - other.group2(),
            // e4235, e4315, e4125, e3215
            self.group3() - other.group3(),
        );
    }
}

impl TryFrom<MultiVector> for VersorOdd {
    type Error = String;
    fn try_from(multi_vector: MultiVector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = multi_vector[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
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
        let el = multi_vector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[17];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[18];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[19];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[20];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[21];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[22];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[23];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[24];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[25];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[26];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MultiVector do not fit into VersorOdd { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOdd::from_groups(
            // e41, e42, e43, scalar
            multi_vector.group4().with_w(multi_vector[scalar]),
            // e23, e31, e12, e45
            multi_vector.group5().with_w(multi_vector[e45]),
            // e15, e25, e35, e1234
            multi_vector.group3().xyz().with_w(multi_vector[e1234]),
            // e4235, e4315, e4125, e3215
            multi_vector.group9(),
        ))
    }
}
